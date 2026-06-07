//! The wire codec: a minimal JSON parser and (de)serialization of messages and
//! contracts. The core already canonicalizes records for signing; here we parse
//! them back so a message can cross a real boundary and still verify.
//!
//! Faithful field round-trip is all that is required: the edge recomputes the
//! signed bytes from the struct, so a parsed message verifies iff every field
//! survived intact.

use std::collections::BTreeMap;

use canonical_core::{canonical_string, Caveat, CorsGrant, Json, Message};

#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}
impl std::error::Error for ParseError {}

// ---- serialization (struct -> wire string) ---------------------------------

fn caveat_to_json(c: &Caveat) -> Json {
    let mut m = BTreeMap::new();
    m.insert("type".to_string(), Json::Str(c.kind.clone()));
    if let Some(n) = c.not_after {
        m.insert("notAfter".to_string(), Json::Int(n));
    }
    if let Some(n) = c.not_before {
        m.insert("notBefore".to_string(), Json::Int(n));
    }
    Json::Obj(m)
}

fn grant_to_json(g: &CorsGrant) -> Json {
    let mut m = BTreeMap::new();
    m.insert("action".to_string(), Json::Str(g.action.clone()));
    if !g.caveats.is_empty() {
        m.insert(
            "caveats".to_string(),
            Json::Arr(g.caveats.iter().map(caveat_to_json).collect()),
        );
    }
    m.insert("issuedAt".to_string(), Json::Int(g.issued_at));
    m.insert("issuer".to_string(), Json::Str(g.issuer.clone()));
    if let Some(p) = &g.proof {
        m.insert("proof".to_string(), grant_to_json(p));
    }
    m.insert("resource".to_string(), Json::Str(g.resource.clone()));
    m.insert("sig".to_string(), Json::Str(g.sig.clone()));
    m.insert("subject".to_string(), Json::Str(g.subject.clone()));
    Json::Obj(m)
}

pub fn message_to_json(msg: &Message) -> Json {
    let mut m = BTreeMap::new();
    m.insert("action".to_string(), Json::Str(msg.action.clone()));
    m.insert("from".to_string(), Json::Str(msg.from.clone()));
    if let Some(g) = &msg.grant {
        m.insert("grant".to_string(), grant_to_json(g));
    }
    m.insert("issuedAt".to_string(), Json::Int(msg.issued_at));
    m.insert("nonce".to_string(), Json::Str(msg.nonce.clone()));
    m.insert("payload".to_string(), msg.payload.clone());
    m.insert("resource".to_string(), Json::Str(msg.resource.clone()));
    m.insert("sig".to_string(), Json::Str(msg.sig.clone()));
    m.insert("to".to_string(), Json::Str(msg.to.clone()));
    Json::Obj(m)
}

pub fn to_wire(msg: &Message) -> String {
    canonical_string(&message_to_json(msg))
}

// ---- deserialization (wire string -> struct) -------------------------------

fn obj(j: &Json) -> Result<&BTreeMap<String, Json>, ParseError> {
    match j {
        Json::Obj(m) => Ok(m),
        _ => Err(ParseError("expected object".into())),
    }
}

fn get_str(m: &BTreeMap<String, Json>, k: &str) -> Result<String, ParseError> {
    match m.get(k) {
        Some(Json::Str(s)) => Ok(s.clone()),
        _ => Err(ParseError(format!("missing string '{k}'"))),
    }
}

fn get_int(m: &BTreeMap<String, Json>, k: &str) -> Result<i64, ParseError> {
    match m.get(k) {
        Some(Json::Int(n)) => Ok(*n),
        _ => Err(ParseError(format!("missing integer '{k}'"))),
    }
}

fn opt_int(m: &BTreeMap<String, Json>, k: &str) -> Result<Option<i64>, ParseError> {
    match m.get(k) {
        None => Ok(None),
        Some(Json::Int(n)) => Ok(Some(*n)),
        Some(_) => Err(ParseError(format!("'{k}' must be an integer"))),
    }
}

fn caveat_from_json(j: &Json) -> Result<Caveat, ParseError> {
    let m = obj(j)?;
    Ok(Caveat {
        kind: get_str(m, "type")?,
        not_after: opt_int(m, "notAfter")?,
        not_before: opt_int(m, "notBefore")?,
    })
}

fn grant_from_json(j: &Json) -> Result<CorsGrant, ParseError> {
    let m = obj(j)?;
    let caveats = match m.get("caveats") {
        None => Vec::new(),
        Some(Json::Arr(a)) => a
            .iter()
            .map(caveat_from_json)
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(ParseError("'caveats' must be an array".into())),
    };
    let proof = match m.get("proof") {
        None => None,
        Some(p) => Some(Box::new(grant_from_json(p)?)),
    };
    Ok(CorsGrant {
        issuer: get_str(m, "issuer")?,
        subject: get_str(m, "subject")?,
        action: get_str(m, "action")?,
        resource: get_str(m, "resource")?,
        caveats,
        issued_at: get_int(m, "issuedAt")?,
        proof,
        sig: get_str(m, "sig")?,
    })
}

fn message_from_json(j: &Json) -> Result<Message, ParseError> {
    let m = obj(j)?;
    let grant = match m.get("grant") {
        None => None,
        Some(g) => Some(grant_from_json(g)?),
    };
    Ok(Message {
        from: get_str(m, "from")?,
        to: get_str(m, "to")?,
        action: get_str(m, "action")?,
        resource: get_str(m, "resource")?,
        payload: m.get("payload").cloned().unwrap_or(Json::Null),
        nonce: get_str(m, "nonce")?,
        issued_at: get_int(m, "issuedAt")?,
        grant,
        sig: get_str(m, "sig")?,
    })
}

pub fn from_wire(s: &str) -> Result<Message, ParseError> {
    message_from_json(&parse(s)?)
}

// ---- a tiny JSON parser (objects, arrays, strings, integers, bool, null) ----

pub(crate) fn parse(s: &str) -> Result<Json, ParseError> {
    let mut p = Parser {
        b: s.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return Err(ParseError("trailing characters".into()));
    }
    Ok(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn value(&mut self) -> Result<Json, ParseError> {
        self.ws();
        match self.b.get(self.i) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.literal("true", Json::Bool(true)),
            Some(b'f') => self.literal("false", Json::Bool(false)),
            Some(b'n') => self.literal("null", Json::Null),
            Some(c) if *c == b'-' || c.is_ascii_digit() => self.number(),
            _ => Err(ParseError("unexpected end or token".into())),
        }
    }

    fn literal(&mut self, word: &str, val: Json) -> Result<Json, ParseError> {
        if self.b[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(val)
        } else {
            Err(ParseError(format!("expected '{word}'")))
        }
    }

    fn number(&mut self) -> Result<Json, ParseError> {
        let start = self.i;
        if self.b.get(self.i) == Some(&b'-') {
            self.i += 1;
        }
        while self.i < self.b.len() && self.b[self.i].is_ascii_digit() {
            self.i += 1;
        }
        if self.i < self.b.len() && matches!(self.b[self.i], b'.' | b'e' | b'E') {
            return Err(ParseError("only integer numbers are supported".into()));
        }
        let text = std::str::from_utf8(&self.b[start..self.i])
            .map_err(|_| ParseError("bad number".into()))?;
        text.parse::<i64>()
            .map(Json::Int)
            .map_err(|_| ParseError(format!("bad integer '{text}'")))
    }

    fn string(&mut self) -> Result<String, ParseError> {
        self.i += 1; // opening quote
        let mut out = String::new();
        loop {
            let c = *self
                .b
                .get(self.i)
                .ok_or_else(|| ParseError("unterminated string".into()))?;
            self.i += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    let e = *self
                        .b
                        .get(self.i)
                        .ok_or_else(|| ParseError("bad escape".into()))?;
                    self.i += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0c}'),
                        b'u' => out.push(self.unicode_escape()?),
                        _ => return Err(ParseError("invalid escape".into())),
                    }
                }
                // a UTF-8 continuation/lead byte: copy bytes through to the char
                _ => {
                    let start = self.i - 1;
                    while self.i < self.b.len() && self.b[self.i] & 0xC0 == 0x80 {
                        self.i += 1;
                    }
                    out.push_str(
                        std::str::from_utf8(&self.b[start..self.i])
                            .map_err(|_| ParseError("bad utf-8".into()))?,
                    );
                }
            }
        }
    }

    fn unicode_escape(&mut self) -> Result<char, ParseError> {
        if self.i + 4 > self.b.len() {
            return Err(ParseError("short \\u escape".into()));
        }
        let hex = std::str::from_utf8(&self.b[self.i..self.i + 4])
            .map_err(|_| ParseError("bad \\u".into()))?;
        let code = u32::from_str_radix(hex, 16).map_err(|_| ParseError("bad \\u hex".into()))?;
        self.i += 4;
        char::from_u32(code).ok_or_else(|| ParseError("invalid code point".into()))
    }

    fn array(&mut self) -> Result<Json, ParseError> {
        self.i += 1; // [
        let mut items = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b']') {
            self.i += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            items.push(self.value()?);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Arr(items));
                }
                _ => return Err(ParseError("expected ',' or ']'".into())),
            }
        }
    }

    fn object(&mut self) -> Result<Json, ParseError> {
        self.i += 1; // {
        let mut map = BTreeMap::new();
        self.ws();
        if self.b.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Ok(Json::Obj(map));
        }
        loop {
            self.ws();
            if self.b.get(self.i) != Some(&b'"') {
                return Err(ParseError("expected object key".into()));
            }
            let key = self.string()?;
            self.ws();
            if self.b.get(self.i) != Some(&b':') {
                return Err(ParseError("expected ':'".into()));
            }
            self.i += 1;
            let val = self.value()?;
            map.insert(key, val);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Obj(map));
                }
                _ => return Err(ParseError("expected ',' or '}'".into())),
            }
        }
    }
}
