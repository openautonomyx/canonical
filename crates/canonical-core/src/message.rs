//! The message: the only way to reach a resource. Signed by the sender and,
//! when the resource is not its own, carrying the CORS contract that authorizes
//! the call.

use std::collections::BTreeMap;

use crate::codec::Json;
use crate::cors::CorsGrant;
use crate::identity::Signer;
use crate::sys;

#[derive(Clone, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub action: String,
    pub resource: String,
    pub payload: Json,
    pub nonce: String,
    pub issued_at: i64,
    pub grant: Option<CorsGrant>,
    pub sig: String,
}

impl Message {
    pub fn signing_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("action".to_string(), Json::str(self.action.clone()));
        m.insert("from".to_string(), Json::str(self.from.clone()));
        if let Some(g) = &self.grant {
            // embed the full contract (with its signature) so the edge is self-contained
            let mut gj = g.signing_json();
            if let Json::Obj(map) = &mut gj {
                map.insert("sig".to_string(), Json::str(g.sig.clone()));
            }
            m.insert("grant".to_string(), gj);
        }
        m.insert("issuedAt".to_string(), Json::Int(self.issued_at));
        m.insert("nonce".to_string(), Json::str(self.nonce.clone()));
        m.insert("payload".to_string(), self.payload.clone());
        m.insert("resource".to_string(), Json::str(self.resource.clone()));
        m.insert("to".to_string(), Json::str(self.to.clone()));
        Json::Obj(m)
    }
}

pub struct ComposeParams<'a> {
    pub to: &'a str,
    pub action: &'a str,
    pub resource: &'a str,
    pub payload: Json,
    pub grant: Option<CorsGrant>,
}

pub fn compose(from: &Signer, params: ComposeParams, now: i64) -> std::io::Result<Message> {
    let mut nonce = [0u8; 16];
    sys::fill_random(&mut nonce)?;
    let mut m = Message {
        from: from.id.clone(),
        to: params.to.to_string(),
        action: params.action.to_string(),
        resource: params.resource.to_string(),
        payload: params.payload,
        nonce: crate::codec::b64url(&nonce),
        issued_at: now,
        grant: params.grant,
        sig: String::new(),
    };
    m.sig = from.sign_record(&m.signing_json());
    Ok(m)
}
