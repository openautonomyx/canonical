//! Working groups: a set of cores wired together by mutual CORS contracts over
//! shared resources. Forming a group expands no core — it only issues contracts
//! that each core's edge independently verifies. Capability grows here, so the
//! core does not have to.

use std::collections::BTreeMap;

use canonical_core::{compose, now_ms, ComposeParams, Core, CorsGrant, Json, Outcome};

/// One share: `owner` grants `to` the right to `action` on `owner:path`.
pub struct Share<'a> {
    pub owner: &'a Core,
    pub path: &'a str,
    pub action: &'a str,
    pub to: &'a str,
}

pub struct WorkingGroup {
    contracts: BTreeMap<String, CorsGrant>,
}

impl WorkingGroup {
    /// Wire a set of cores together by issuing the shares between them.
    pub fn form(shares: &[Share]) -> WorkingGroup {
        let mut wg = WorkingGroup {
            contracts: BTreeMap::new(),
        };
        for s in shares {
            let contract = s.owner.share(s.to, s.path, s.action, vec![]);
            wg.contracts
                .insert(key(s.to, &s.owner.resource(s.path), s.action), contract);
        }
        wg
    }

    pub fn contract_for(&self, holder: &str, resource: &str, action: &str) -> Option<&CorsGrant> {
        self.contracts.get(&key(holder, resource, action))
    }

    /// A member invokes another member's resource using the group's contract.
    pub fn invoke(
        &self,
        holder: &Core,
        target: &mut Core,
        path: &str,
        action: &str,
        payload: Json,
    ) -> std::io::Result<Outcome> {
        let resource = target.resource(path);
        let grant = self.contract_for(holder.id(), &resource, action).cloned();
        let msg = compose(
            holder.signer(),
            ComposeParams {
                to: target.id(),
                action,
                resource: &resource,
                payload,
                grant,
            },
            now_ms(),
        )?;
        Ok(target.handle(&msg))
    }
}

fn key(holder: &str, resource: &str, action: &str) -> String {
    format!("{holder}|{resource}|{action}")
}
