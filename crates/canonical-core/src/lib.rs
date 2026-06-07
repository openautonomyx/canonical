//! # canonical-core
//!
//! The single, independent, complete working unit — the trusted computing base
//! of the Canonical Autonomyx. Zero third-party dependencies: built, not
//! composed. Every line here is a line you must trust, so there are few of them.
//!
//! Trust is not based on origin. The only trust is an explicit CORS contract,
//! held by the provider and enforced at the edge.

mod codec;
mod cors;
mod ed25519;
mod edge;
mod identity;
mod kernel;
mod message;
mod resource;
mod sha512;
mod sys;

pub use codec::{b64url, b64url_decode, canonical_string, Json};
pub use cors::{
    attenuate, authorizes, issue_grant, verify_grant, Caveat, CorsGrant, GrantCheck, IssueParams,
};
pub use edge::{Decision, Edge};
pub use identity::{verify_record, verify_signature, Identity, Signer};
pub use kernel::{Core, Handler, Outcome};
pub use message::{compose, ComposeParams, Message};
pub use resource::{
    action_subsumes, make_resource, path_subsumes, resource_owner, resource_path, resource_subsumes,
};
pub use sys::now_ms;
