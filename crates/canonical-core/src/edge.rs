//! The edge: CORS is enforced here, and nowhere else. Default-deny. The edge is
//! the whole of a provider's authority and the entire trust boundary.
//!
//! Verification is local and self-contained: a message carries its own proof
//! (the contract), so the edge needs no external lookup and no round-trip.

use std::collections::HashSet;

use crate::cors::{authorizes, verify_grant};
use crate::identity::verify_record;
use crate::message::Message;
use crate::resource::resource_owner;
use crate::sys;

#[derive(Debug, Clone)]
pub struct Decision {
    pub ok: bool,
    pub reason: Option<String>,
}

impl Decision {
    fn allow() -> Decision {
        Decision {
            ok: true,
            reason: None,
        }
    }
    fn deny(reason: impl Into<String>) -> Decision {
        Decision {
            ok: false,
            reason: Some(reason.into()),
        }
    }
}

pub struct Edge {
    owner_id: String,
    window_ms: i64,
    seen: HashSet<String>,
}

impl Edge {
    pub fn new(owner_id: String) -> Edge {
        Edge::with_window(owner_id, 60_000)
    }

    pub fn with_window(owner_id: String, window_ms: i64) -> Edge {
        Edge {
            owner_id,
            window_ms,
            seen: HashSet::new(),
        }
    }

    /// The complete trust decision for one request — pure CORS, at the boundary.
    pub fn admit(&mut self, msg: &Message) -> Decision {
        let now = sys::now_ms();
        // 1. integrity
        if !verify_record(&msg.from, &msg.signing_json(), &msg.sig) {
            return Decision::deny("forged or tampered message");
        }
        // 2. addressed to us, for a resource we own
        if msg.to != self.owner_id {
            return Decision::deny("message not addressed to this edge");
        }
        if resource_owner(&msg.resource) != self.owner_id {
            return Decision::deny("resource not owned by this edge");
        }
        // 3. freshness
        if (now - msg.issued_at).abs() > self.window_ms {
            return Decision::deny("message outside freshness window");
        }
        // 4. replay
        if self.seen.contains(&msg.nonce) {
            return Decision::deny("replayed nonce");
        }
        // 5. CORS — the only trust. No contract, no access. Origin grants nothing.
        let grant = match &msg.grant {
            Some(g) => g,
            None => {
                return Decision::deny(format!(
                    "no CORS contract for {} {}",
                    msg.action, msg.resource
                ))
            }
        };
        if grant.subject != msg.from {
            return Decision::deny("contract not held by the sender");
        }
        if !authorizes(grant, &msg.action, &msg.resource) {
            return Decision::deny("contract does not authorize this action/resource");
        }
        let gc = verify_grant(grant, &self.owner_id, now);
        if !gc.ok {
            return Decision::deny(format!(
                "invalid contract: {}",
                gc.reason.unwrap_or_default()
            ));
        }
        self.seen.insert(msg.nonce.clone());
        Decision::allow()
    }
}
