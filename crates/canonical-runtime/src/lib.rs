//! # canonical-runtime
//!
//! Userland built on `canonical-core`. NOT part of the trusted computing base —
//! the core verifies everything here. This is where capability grows so the core
//! does not have to: working groups, a wire codec, and a network-edge transport.
//!
//! Std only; zero third-party dependencies, same as the core.

mod net;
mod wire;
mod working_group;

pub use net::{call, serve_once, WireOutcome};
pub use wire::{from_wire, to_wire, ParseError};
pub use working_group::{Share, WorkingGroup};
