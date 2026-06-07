//! CORS: the one and only trust primitive.
//!
//! Trust is not based on origin. A resource is reachable only when its provider
//! has issued an explicit, signed sharing contract. A grant IS a contract: only
//! the provider can originate one, and every valid grant chains, by monotone
//! attenuation, back to that provider.

use std::collections::BTreeMap;

use crate::codec::Json;
use crate::identity::{verify_record, Signer};
use crate::resource::{action_subsumes, resource_subsumes};

#[derive(Clone, Debug)]
pub struct Caveat {
    pub kind: String,
    pub not_after: Option<i64>,
    pub not_before: Option<i64>,
}

impl Caveat {
    /// A short-lived contract: invalid after `not_after` (ms since epoch).
    pub fn expiry(not_after: i64) -> Caveat {
        Caveat {
            kind: "expiry".into(),
            not_after: Some(not_after),
            not_before: None,
        }
    }

    /// A contract that only becomes valid at `not_before`.
    pub fn not_before(not_before: i64) -> Caveat {
        Caveat {
            kind: "notBefore".into(),
            not_after: None,
            not_before: Some(not_before),
        }
    }

    pub fn unknown(kind: impl Into<String>) -> Caveat {
        Caveat {
            kind: kind.into(),
            not_after: None,
            not_before: None,
        }
    }

    fn json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("type".to_string(), Json::str(self.kind.clone()));
        if let Some(n) = self.not_after {
            m.insert("notAfter".to_string(), Json::Int(n));
        }
        if let Some(n) = self.not_before {
            m.insert("notBefore".to_string(), Json::Int(n));
        }
        Json::Obj(m)
    }
}

#[derive(Clone, Debug)]
pub struct CorsGrant {
    pub issuer: String,
    pub subject: String,
    pub action: String,
    pub resource: String,
    pub caveats: Vec<Caveat>,
    pub issued_at: i64,
    pub proof: Option<Box<CorsGrant>>,
    pub sig: String,
}

impl CorsGrant {
    /// The signed body (everything but `sig`). An embedded proof carries its own
    /// signature, so the contract is self-contained and verifiable at the edge.
    pub fn signing_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("action".to_string(), Json::str(self.action.clone()));
        if !self.caveats.is_empty() {
            m.insert(
                "caveats".to_string(),
                Json::Arr(self.caveats.iter().map(Caveat::json).collect()),
            );
        }
        m.insert("issuedAt".to_string(), Json::Int(self.issued_at));
        m.insert("issuer".to_string(), Json::str(self.issuer.clone()));
        if let Some(p) = &self.proof {
            m.insert("proof".to_string(), p.full_json());
        }
        m.insert("resource".to_string(), Json::str(self.resource.clone()));
        m.insert("subject".to_string(), Json::str(self.subject.clone()));
        Json::Obj(m)
    }

    fn full_json(&self) -> Json {
        let mut j = self.signing_json();
        if let Json::Obj(m) = &mut j {
            m.insert("sig".to_string(), Json::str(self.sig.clone()));
        }
        j
    }
}

/// Does this contract, on its face, permit `action` on `resource`?
pub fn authorizes(grant: &CorsGrant, action: &str, resource: &str) -> bool {
    action_subsumes(&grant.action, action) && resource_subsumes(&grant.resource, resource)
}

pub struct IssueParams<'a> {
    pub subject: &'a str,
    pub action: &'a str,
    pub resource: &'a str,
    pub caveats: Vec<Caveat>,
    pub proof: Option<CorsGrant>,
}

/// The provider originates a contract for one of its own resources.
pub fn issue_grant(owner: &Signer, params: IssueParams, now: i64) -> CorsGrant {
    let mut g = CorsGrant {
        issuer: owner.id.clone(),
        subject: params.subject.to_string(),
        action: params.action.to_string(),
        resource: params.resource.to_string(),
        caveats: params.caveats,
        issued_at: now,
        proof: params.proof.map(Box::new),
        sig: String::new(),
    };
    g.sig = owner.sign_record(&g.signing_json());
    g
}

/// A holder delegates part of a contract it holds to another subject.
pub fn attenuate(
    holder: &Signer,
    parent: &CorsGrant,
    subject: &str,
    action: Option<&str>,
    resource: Option<&str>,
    caveats: Vec<Caveat>,
    now: i64,
) -> CorsGrant {
    issue_grant(
        holder,
        IssueParams {
            subject,
            action: action.unwrap_or(&parent.action),
            resource: resource.unwrap_or(&parent.resource),
            caveats,
            proof: Some(parent.clone()),
        },
        now,
    )
}

#[derive(Debug)]
pub struct GrantCheck {
    pub ok: bool,
    pub reason: Option<String>,
}

fn deny(reason: &str) -> GrantCheck {
    GrantCheck {
        ok: false,
        reason: Some(reason.to_string()),
    }
}
fn ok() -> GrantCheck {
    GrantCheck {
        ok: true,
        reason: None,
    }
}

/// Verify the contract chains, by valid signatures and monotone attenuation, all
/// the way back to the resource provider. Only the provider can root a contract,
/// and no link may widen what its parent granted.
pub fn verify_grant(grant: &CorsGrant, provider_id: &str, now: i64) -> GrantCheck {
    verify_grant_depth(grant, provider_id, now, 0)
}

fn verify_grant_depth(grant: &CorsGrant, provider_id: &str, now: i64, depth: u32) -> GrantCheck {
    if depth > 32 {
        return deny("contract chain too deep");
    }
    if !verify_record(&grant.issuer, &grant.signing_json(), &grant.sig) {
        return deny("invalid contract signature");
    }
    for c in &grant.caveats {
        match c.kind.as_str() {
            "expiry" => match c.not_after {
                Some(n) if now <= n => {}
                _ => return deny("contract expired"),
            },
            "notBefore" => match c.not_before {
                Some(n) if now >= n => {}
                _ => return deny("contract not yet valid"),
            },
            // fail closed: an edge that does not understand a caveat must not honour it
            other => return deny(&format!("unknown caveat '{other}'")),
        }
    }
    if grant.issuer == provider_id {
        return ok();
    }
    let proof = match &grant.proof {
        Some(p) => p,
        None => return deny("contract not rooted at the provider"),
    };
    if proof.subject != grant.issuer {
        return deny("delegation not held by its issuer");
    }
    if !action_subsumes(&proof.action, &grant.action) {
        return deny("delegation widens the action");
    }
    if !resource_subsumes(&proof.resource, &grant.resource) {
        return deny("delegation widens the resource");
    }
    verify_grant_depth(proof, provider_id, now, depth + 1)
}
