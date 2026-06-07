//! Identity: an Ed25519 keypair. An identity IS an origin — a namespace and a
//! public key. It confers no trust by itself (see cors.rs).

use crate::codec::{b64url, b64url_decode, canonical_string, Json};
use crate::ed25519;
use crate::sys;

/// A public identity — its id is base64url(raw 32-byte Ed25519 public key).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub id: String,
}

/// A signing identity. The 32-byte seed never leaves this struct's owner.
#[derive(Clone)]
pub struct Signer {
    seed: [u8; 32],
    pub id: String,
}

impl Signer {
    pub fn generate() -> std::io::Result<Signer> {
        let mut seed = [0u8; 32];
        sys::fill_random(&mut seed)?;
        Ok(Signer::from_seed(seed))
    }

    pub fn from_seed(seed: [u8; 32]) -> Signer {
        let pk = ed25519::public_key(&seed);
        Signer {
            seed,
            id: b64url(&pk),
        }
    }

    pub fn identity(&self) -> Identity {
        Identity {
            id: self.id.clone(),
        }
    }

    pub fn sign(&self, data: &[u8]) -> String {
        b64url(&ed25519::sign(&self.seed, data))
    }

    /// Sign the canonical bytes of a record body (the body excludes `sig`).
    pub fn sign_record(&self, body: &Json) -> String {
        self.sign(canonical_string(body).as_bytes())
    }
}

pub fn verify_signature(id: &str, data: &[u8], sig: &str) -> bool {
    let pk: [u8; 32] = match b64url_decode(id) {
        Some(v) if v.len() == 32 => v.try_into().unwrap(),
        _ => return false,
    };
    let s: [u8; 64] = match b64url_decode(sig) {
        Some(v) if v.len() == 64 => v.try_into().unwrap(),
        _ => return false,
    };
    ed25519::verify(&pk, data, &s)
}

pub fn verify_record(id: &str, body: &Json, sig: &str) -> bool {
    verify_signature(id, canonical_string(body).as_bytes(), sig)
}
