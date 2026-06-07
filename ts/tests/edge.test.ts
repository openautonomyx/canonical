import { test } from "node:test";
import assert from "node:assert/strict";
import { Core, compose } from "../src/core/index.ts";

async function setup() {
  const alice = await Core.create();
  const bob = await Core.create();
  bob.expose("tool/echo", (p) => ({ echo: p }));
  return { alice, bob, echo: bob.resource("tool/echo") };
}

test("denies without a contract — origin grants nothing", async () => {
  const { alice, bob, echo } = await setup();
  const out = await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: 1 }));
  assert.equal(out.decision.ok, false);
});

test("allows with the provider's contract, and runs the handler", async () => {
  const { alice, bob, echo } = await setup();
  const grant = await bob.share(alice.id, "tool/echo", "invoke");
  const out = await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant }));
  assert.equal(out.decision.ok, true);
  assert.deepEqual(out.result, { echo: "hi" });
});

test("rejects a tampered message", async () => {
  const { alice, bob, echo } = await setup();
  const grant = await bob.share(alice.id, "tool/echo", "invoke");
  const msg = await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant });
  const out = await bob.handle({ ...msg, payload: "tampered" });
  assert.equal(out.decision.ok, false);
});

test("rejects a replayed message", async () => {
  const { alice, bob, echo } = await setup();
  const grant = await bob.share(alice.id, "tool/echo", "invoke");
  const msg = await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant });
  assert.equal((await bob.handle(msg)).decision.ok, true);
  assert.equal((await bob.handle(msg)).decision.ok, false);
});

test("rejects a contract minted for someone else", async () => {
  const { alice, bob, echo } = await setup();
  const mallory = await Core.create();
  const grant = await bob.share(mallory.id, "tool/echo", "invoke"); // not for alice
  const out = await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant }));
  assert.equal(out.decision.ok, false);
});

test("rejects a message aimed at the wrong edge", async () => {
  const { alice, bob, echo } = await setup();
  const other = await Core.create();
  const grant = await bob.share(alice.id, "tool/echo", "invoke");
  // Addressed to bob's resource but delivered to a different core's edge.
  const out = await other.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant }));
  assert.equal(out.decision.ok, false);
});
