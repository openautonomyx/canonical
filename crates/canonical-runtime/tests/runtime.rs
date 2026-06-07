//! Working groups, the wire round-trip, and the network edge.

use std::net::TcpListener;
use std::thread;

use canonical_core::{compose, now_ms, ComposeParams, Core, Json};
use canonical_runtime::{call, from_wire, serve_once, to_wire, Share, WorkingGroup};

fn echo(p: Json) -> Json {
    let mut m = std::collections::BTreeMap::new();
    m.insert("echo".to_string(), p);
    Json::Obj(m)
}

#[test]
fn working_group_wires_cores_and_collaborates() {
    let a = Core::create().unwrap();
    let mut b = Core::create().unwrap();
    b.expose("tool/echo", echo);
    let wg = WorkingGroup::form(&[Share {
        owner: &b,
        path: "tool/echo",
        action: "invoke",
        to: a.id(),
    }]);
    let out = wg
        .invoke(&a, &mut b, "tool/echo", "invoke", Json::str("hello"))
        .unwrap();
    assert!(out.decision.ok);
}

#[test]
fn working_group_denies_a_non_member() {
    let a = Core::create().unwrap();
    let mut b = Core::create().unwrap();
    let c = Core::create().unwrap();
    b.expose("tool/echo", echo);
    let wg = WorkingGroup::form(&[Share {
        owner: &b,
        path: "tool/echo",
        action: "invoke",
        to: a.id(),
    }]);
    // c was never granted anything
    let out = wg
        .invoke(&c, &mut b, "tool/echo", "invoke", Json::str("sneak"))
        .unwrap();
    assert!(!out.decision.ok);
}

#[test]
fn wire_round_trip_preserves_the_signature() {
    let alice = Core::create().unwrap();
    let mut bob = Core::create().unwrap();
    bob.expose("tool/echo", echo);
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let resource = bob.resource("tool/echo");
    let msg = compose(
        alice.signer(),
        ComposeParams {
            to: bob.id(),
            action: "invoke",
            resource: &resource,
            payload: Json::str("hi"),
            grant: Some(grant),
        },
        now_ms(),
    )
    .unwrap();

    // serialize -> parse -> the reconstructed message still verifies and runs
    let wire = to_wire(&msg);
    let parsed = from_wire(&wire).unwrap();
    let out = bob.handle(&parsed);
    assert!(out.decision.ok, "{:?}", out.decision.reason);
}

#[test]
fn message_crosses_a_real_tcp_boundary() {
    let alice = Core::create().unwrap();
    let mut bob = Core::create().unwrap();
    bob.expose("tool/echo", echo);
    let resource = bob.resource("tool/echo");

    // grant for the allowed call; capture ids before moving bob into the server
    let grant = bob.share(alice.id(), "tool/echo", "invoke", vec![]);
    let bob_id = bob.id().to_string();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    // server: handle two connections (a denied call, then an allowed one)
    let server = thread::spawn(move || {
        serve_once(&mut bob, &listener).unwrap();
        serve_once(&mut bob, &listener).unwrap();
    });

    // 1. no contract -> the remote edge denies
    let m1 = compose(
        alice.signer(),
        ComposeParams {
            to: &bob_id,
            action: "invoke",
            resource: &resource,
            payload: Json::str("hi"),
            grant: None,
        },
        now_ms(),
    )
    .unwrap();
    let r1 = call(addr, &m1).unwrap();
    assert!(!r1.ok);

    // 2. with the contract -> allowed, and the handler result comes back
    let m2 = compose(
        alice.signer(),
        ComposeParams {
            to: &bob_id,
            action: "invoke",
            resource: &resource,
            payload: Json::str("hi"),
            grant: Some(grant),
        },
        now_ms(),
    )
    .unwrap();
    let r2 = call(addr, &m2).unwrap();
    assert!(r2.ok);
    assert!(matches!(r2.result, Some(Json::Obj(_))));

    server.join().unwrap();
}
