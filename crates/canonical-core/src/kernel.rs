//! The Canonical Core — the single, independent, complete working unit.
//!
//! One identity. One edge. A handful of resources. One operation: handle a
//! message — decide at the edge (CORS), and only if admitted, execute. This is
//! the whole trusted computing base; it needs no other party to make a trust
//! decision, and it has zero third-party dependencies. Trust is inversely
//! proportional to its size, so build capability around it, never into it.

use std::collections::HashMap;

use crate::codec::Json;
use crate::cors::{issue_grant, Caveat, CorsGrant, IssueParams};
use crate::edge::{Decision, Edge};
use crate::identity::Signer;
use crate::message::Message;
use crate::resource::{make_resource, resource_path};
use crate::sys;

pub type Handler = Box<dyn Fn(Json) -> Json + Send + Sync>;

pub struct Outcome {
    pub decision: Decision,
    pub result: Option<Json>,
}

pub struct Core {
    signer: Signer,
    edge: Edge,
    resources: HashMap<String, Handler>,
}

impl Core {
    /// Genesis: a new, self-sufficient unit with its own origin.
    pub fn create() -> std::io::Result<Core> {
        let signer = Signer::generate()?;
        let edge = Edge::new(signer.id.clone());
        Ok(Core {
            signer,
            edge,
            resources: HashMap::new(),
        })
    }

    pub fn from_seed(seed: [u8; 32]) -> Core {
        let signer = Signer::from_seed(seed);
        let edge = Edge::new(signer.id.clone());
        Core {
            signer,
            edge,
            resources: HashMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.signer.id
    }

    pub fn signer(&self) -> &Signer {
        &self.signer
    }

    /// Publish a resource this unit owns. Owning is not sharing — see `share`.
    pub fn expose<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Json) -> Json + Send + Sync + 'static,
    {
        self.resources.insert(path.to_string(), Box::new(handler));
        self
    }

    /// The full address ("<thisId>:<path>") others target to reach a resource.
    pub fn resource(&self, path: &str) -> String {
        make_resource(&self.signer.id, path)
    }

    /// Explicitly share one of our resources — originate a CORS contract.
    pub fn share(
        &self,
        subject: &str,
        path: &str,
        action: &str,
        caveats: Vec<Caveat>,
    ) -> CorsGrant {
        let resource = self.resource(path);
        issue_grant(
            &self.signer,
            IssueParams {
                subject,
                action,
                resource: &resource,
                caveats,
                proof: None,
            },
            sys::now_ms(),
        )
    }

    /// Receive a message: decide at the edge, then execute only if admitted.
    pub fn handle(&mut self, msg: &Message) -> Outcome {
        let decision = self.edge.admit(msg);
        if !decision.ok {
            return Outcome {
                decision,
                result: None,
            };
        }
        let result = self
            .resources
            .get(resource_path(&msg.resource))
            .map(|h| h(msg.payload.clone()));
        Outcome { decision, result }
    }
}
