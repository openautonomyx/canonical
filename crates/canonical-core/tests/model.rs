//! The trust model, end to end, through the public API of canonical-core.

use canonical_core::*;

fn echo_core() -> (Core, Core, String) {
    let alice = Core::create().unwrap();
    let mut bob = Core::create().unwrap();
    bob.expose("tool/echo", |payload| {
        let mut m = std::collections::BTreeMap::new();
        m.insert("echo".to_string(), payload);
        Json::Obj(m)
    });
    let echo = bob.resource("tool/echo");
    (alice, bob, echo)
}

fn invoke(
    from: &Signer,
    to: &str,
    resource: &str,
    payload: Json,
    grant: Option<CorsGrant>,
) -> Message {
    compose(
        from,
        ComposeParams {
            to,
            action: "invoke",
            resource,
            payload,
            grant,
        },
        now_ms(),
    )
    .unwrap()
}

#[test]
fn denies_without_a_contract_origin_grants_nothing() {
    let (alice, mut bob, echo) = echo_core();
    let msg = invoke(alice.signer(), bob.id(), &echo, Json::Int(1), None);
    let out = bob.handle(&msg);
    assert!(!out.decision.ok);
}

#[test]
fn allows_with_provider_contract_and_runs_handler() {
    let (alice, mut bob, echo) = echo_core();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let msg = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(grant),
    );
    let out = bob.handle(&msg);
    assert!(out.decision.ok, "{:?}", out.decision.reason);
    match out.result {
        Some(Json::Obj(m)) => assert!(matches!(m.get("echo"), Some(Json::Str(s)) if s == "hi")),
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn attenuated_delegation_narrows_and_is_accepted() {
    let (alice, mut bob, echo) = echo_core();
    let carol = Core::create().unwrap();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let sub = attenuate(
        alice.signer(),
        &grant,
        carol.id(),
        None,
        None,
        vec![],
        now_ms(),
    );
    let msg = invoke(
        carol.signer(),
        bob.id(),
        &echo,
        Json::str("via carol"),
        Some(sub),
    );
    assert!(bob.handle(&msg).decision.ok);
}

#[test]
fn delegation_cannot_widen() {
    let (alice, mut bob, _echo) = echo_core();
    let carol = Core::create().unwrap();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let wide = bob.resource("tool/*");
    let widened = attenuate(
        alice.signer(),
        &grant,
        carol.id(),
        None,
        Some(&wide),
        vec![],
        now_ms(),
    );
    let secret = bob.resource("tool/secret");
    let msg = invoke(
        carol.signer(),
        bob.id(),
        &secret,
        Json::str("x"),
        Some(widened),
    );
    assert!(!bob.handle(&msg).decision.ok);
}

#[test]
fn replay_is_rejected() {
    let (alice, mut bob, echo) = echo_core();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let msg = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(grant),
    );
    assert!(bob.handle(&msg).decision.ok);
    assert!(!bob.handle(&msg).decision.ok);
}

#[test]
fn tampered_message_is_rejected() {
    let (alice, mut bob, echo) = echo_core();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let mut msg = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(grant),
    );
    msg.payload = Json::str("tampered");
    assert!(!bob.handle(&msg).decision.ok);
}

#[test]
fn contract_minted_for_someone_else_is_rejected() {
    let (alice, mut bob, echo) = echo_core();
    let mallory = Core::create().unwrap();
    let grant = bob.share(mallory.id(), "tool/echo", "invoke", vec![]); // not for alice
    let msg = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(grant),
    );
    assert!(!bob.handle(&msg).decision.ok);
}

#[test]
fn message_aimed_at_the_wrong_edge_is_rejected() {
    let (alice, bob, echo) = echo_core();
    let mut other = Core::create().unwrap();
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let msg = invoke(
        alice.signer(),
        bob.id(),
        &echo,
        Json::str("hi"),
        Some(grant),
    );
    assert!(!other.handle(&msg).decision.ok);
}

#[test]
fn expiry_caveat_is_enforced_fail_closed() {
    let owner = Signer::generate().unwrap();
    let sub = Signer::generate().unwrap();
    let resource = make_resource(&owner.id, "x");
    let grant = issue_grant(
        &owner,
        IssueParams {
            subject: &sub.id,
            action: "invoke",
            resource: &resource,
            caveats: vec![Caveat::expiry(1000)],
            proof: None,
        },
        500,
    );
    assert!(verify_grant(&grant, &owner.id, 500).ok);
    assert!(!verify_grant(&grant, &owner.id, 2000).ok);
}

#[test]
fn unknown_caveat_fails_closed() {
    let owner = Signer::generate().unwrap();
    let sub = Signer::generate().unwrap();
    let resource = make_resource(&owner.id, "x");
    let grant = issue_grant(
        &owner,
        IssueParams {
            subject: &sub.id,
            action: "invoke",
            resource: &resource,
            caveats: vec![Caveat::unknown("from-the-future")],
            proof: None,
        },
        0,
    );
    assert!(!verify_grant(&grant, &owner.id, 0).ok);
}

#[test]
fn contract_not_rooted_at_provider_is_rejected() {
    let owner = Signer::generate().unwrap();
    let stranger = Signer::generate().unwrap();
    let sub = Signer::generate().unwrap();
    let resource = make_resource(&owner.id, "tool/echo");
    // stranger signs a grant for a resource they do not own
    let grant = issue_grant(
        &stranger,
        IssueParams {
            subject: &sub.id,
            action: "invoke",
            resource: &resource,
            caveats: vec![],
            proof: None,
        },
        now_ms(),
    );
    assert!(!verify_grant(&grant, &owner.id, now_ms()).ok);
}
