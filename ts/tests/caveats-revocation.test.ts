import { test } from "node:test";
import assert from "node:assert/strict";
import { Core, compose, attenuate, issueGrant, verifyGrant, expiry, notBefore, makeResource, generateSigner } from "../src/core/index.ts";

async function echoSetup() {
  const alice = await Core.create();
  const bob = await Core.create();
  bob.expose("tool/echo", (p) => ({ echo: p }));
  return { alice, bob, echo: bob.resource("tool/echo") };
}

test("expiry caveat is enforced (fail closed)", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(owner, { subject: sub.id, action: "invoke", resource: makeResource(owner.id, "x"), caveats: [expiry(1000)] }, 500);
  assert.equal((await verifyGrant(g, owner.id, 500)).ok, true);
  assert.equal((await verifyGrant(g, owner.id, 2000)).ok, false);
});

test("notBefore caveat is enforced", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(owner, { subject: sub.id, action: "invoke", resource: makeResource(owner.id, "x"), caveats: [notBefore(1000)] }, 0);
  assert.equal((await verifyGrant(g, owner.id, 500)).ok, false); // not yet valid
  assert.equal((await verifyGrant(g, owner.id, 1500)).ok, true); // now valid
});

test("revoking a contract denies further use", async () => {
  const { alice, bob, echo } = await echoSetup();
  const grant = await bob.share(alice.id, "tool/echo", "invoke");
  const first = await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant });
  assert.equal((await bob.handle(first)).decision.ok, true);

  bob.revoke(grant);
  const second = await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant });
  const out = await bob.handle(second);
  assert.equal(out.decision.ok, false);
  assert.equal(out.decision.reason, "contract revoked");
});

test("revoking a parent invalidates its delegations", async () => {
  const { alice, bob, echo } = await echoSetup();
  const carol = await Core.create();
  const parent = await bob.share(alice.id, "tool/echo", "invoke");
  const sub = await attenuate(alice.identity, parent, { subject: carol.id });
  bob.revoke(parent);
  const msg = await compose(carol.identity, { to: bob.id, action: "invoke", resource: echo, payload: "via carol", grant: sub });
  assert.equal((await bob.handle(msg)).decision.ok, false);
});
