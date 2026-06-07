//! The network edge: carry a message across a real TCP boundary. The transport
//! is dumb on purpose — it moves bytes. All trust is decided by the receiving
//! core's edge, exactly as for an in-process call. Std only.
//!
//! Wire protocol: one JSON message per line (request), one JSON outcome per line
//! (response).

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use canonical_core::{canonical_string, Core, Json, Message, Outcome};

use crate::wire::{from_wire, parse, to_wire, ParseError};

/// The result of a remote call: the edge's decision plus any handler result.
#[derive(Debug)]
pub struct WireOutcome {
    pub ok: bool,
    pub reason: Option<String>,
    pub result: Option<Json>,
}

fn outcome_to_json(out: &Outcome) -> Json {
    let mut m = BTreeMap::new();
    m.insert("ok".to_string(), Json::Bool(out.decision.ok));
    if let Some(r) = &out.decision.reason {
        m.insert("reason".to_string(), Json::Str(r.clone()));
    }
    if let Some(r) = &out.result {
        m.insert("result".to_string(), r.clone());
    }
    Json::Obj(m)
}

fn parse_outcome(j: &Json) -> Result<WireOutcome, ParseError> {
    let m = match j {
        Json::Obj(m) => m,
        _ => return Err(ParseError("outcome must be an object".into())),
    };
    let ok = matches!(m.get("ok"), Some(Json::Bool(true)));
    let reason = match m.get("reason") {
        Some(Json::Str(s)) => Some(s.clone()),
        _ => None,
    };
    let result = m.get("result").cloned();
    Ok(WireOutcome { ok, reason, result })
}

/// Serve exactly one request on an already-bound listener: read a message,
/// decide at the edge and execute if admitted, write the outcome back.
pub fn serve_once(core: &mut Core, listener: &TcpListener) -> std::io::Result<()> {
    let (stream, _) = listener.accept()?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let response = match from_wire(line.trim()) {
        Ok(msg) => outcome_to_json(&core.handle(&msg)),
        Err(e) => {
            let mut m = BTreeMap::new();
            m.insert("ok".to_string(), Json::Bool(false));
            m.insert(
                "reason".to_string(),
                Json::Str(format!("unreadable message: {}", e.0)),
            );
            Json::Obj(m)
        }
    };

    let mut writer = stream;
    writeln!(writer, "{}", canonical_string(&response))?;
    Ok(())
}

/// Send a message to a remote edge and read its decision.
pub fn call<A: ToSocketAddrs>(addr: A, msg: &Message) -> std::io::Result<WireOutcome> {
    let stream = TcpStream::connect(addr)?;
    let mut writer = stream.try_clone()?;
    writeln!(writer, "{}", to_wire(msg))?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let json = parse(line.trim())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.0))?;
    parse_outcome(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.0))
}
