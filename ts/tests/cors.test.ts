import { test } from "node:test";
import assert from "node:assert/strict";
import { generateSigner, issueGrant, attenuate, verifyGrant, authorizes, makeResource } from "../src/core/index.ts";

test("a provider-issued contract verifies to the provider", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const r = makeResource(owner.id, "tool/echo");
  const g = await issueGrant(owner, { subject: sub.id, action: "invoke", resource: r });
  assert.equal((await verifyGrant(g, owner.id, Date.now())).ok, true);
  assert.equal(authorizes(g, "invoke", r), true);
});

test("a contract does not authorize a different resource", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(owner, { subject: sub.id, action: "invoke", resource: makeResource(owner.id, "tool/echo") });
  assert.equal(authorizes(g, "invoke", makeResource(owner.id, "tool/secret")), false);
});

test("attenuation narrows and still verifies to the provider", async () => {
  const owner = await generateSigner();
  const alice = await generateSigner();
  const carol = await generateSigner();
  const r = makeResource(owner.id, "tool/echo");
  const g = await issueGrant(owner, { subject: alice.id, action: "*", resource: makeResource(owner.id, "tool/*") });
  const sub = await attenuate(alice, g, { subject: carol.id, action: "invoke", resource: r });
  assert.equal((await verifyGrant(sub, owner.id, Date.now())).ok, true);
});

test("attenuation that widens is rejected", async () => {
  const owner = await generateSigner();
  const alice = await generateSigner();
  const carol = await generateSigner();
  const g = await issueGrant(owner, { subject: alice.id, action: "invoke", resource: makeResource(owner.id, "tool/echo") });
  const wide = await attenuate(alice, g, { subject: carol.id, resource: makeResource(owner.id, "tool/*") });
  assert.equal((await verifyGrant(wide, owner.id, Date.now())).ok, false);
});

test("a contract not rooted at the provider is rejected (origin grants nothing)", async () => {
  const owner = await generateSigner();
  const stranger = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(stranger, { subject: sub.id, action: "invoke", resource: makeResource(owner.id, "tool/echo") });
  assert.equal((await verifyGrant(g, owner.id, Date.now())).ok, false);
});

test("expiry caveat is enforced (fail closed)", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(owner, {
    subject: sub.id,
    action: "invoke",
    resource: makeResource(owner.id, "x"),
    caveats: [{ type: "expiry", notAfter: 1000 }],
  });
  assert.equal((await verifyGrant(g, owner.id, 2000)).ok, false);
  assert.equal((await verifyGrant(g, owner.id, 500)).ok, true);
});

test("an unknown caveat fails closed", async () => {
  const owner = await generateSigner();
  const sub = await generateSigner();
  const g = await issueGrant(owner, {
    subject: sub.id,
    action: "invoke",
    resource: makeResource(owner.id, "x"),
    caveats: [{ type: "from-the-future" }],
  });
  assert.equal((await verifyGrant(g, owner.id, Date.now())).ok, false);
});
