// Canonical Autonomyx — a runnable tour of the whole stack.
//
//   node examples/demo.ts
//
// Two independent cores in one process. Same machine, same genesis — and yet
// origin grants nothing. Only an explicit CORS contract, enforced at the edge,
// opens a door.

import { Core, compose, attenuate } from "../src/core/index.ts";
import { WorkingGroup } from "../src/runtime/index.ts";

const log = (s = "") => console.log(s);

function render(out: { decision: { ok: boolean; reason?: string }; result?: unknown }): string {
  return out.decision.ok ? `ALLOW  ${JSON.stringify(out.result)}` : `DENY   (${out.decision.reason})`;
}

async function main(): Promise<void> {
  log("Canonical Autonomyx");
  log("===================");

  const alice = await Core.create();
  const bob = await Core.create();

  // Bob owns a resource and exposes a handler. Owning is not sharing.
  bob.expose("tool/echo", (payload) => ({ echo: payload }));
  const echo = bob.resource("tool/echo");

  // 1. No contract -> denied. Trust is not based on origin.
  let out = await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi" }));
  log(`\n1. invoke, no contract            -> ${render(out)}`);

  // 2. Bob explicitly shares (CORS) with Alice -> allowed, through the edge.
  const contract = await bob.share(alice.id, "tool/echo", "invoke");
  out = await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant: contract }));
  log(`2. invoke with Bob's contract     -> ${render(out)}`);

  // 3. Attenuated delegation: Alice hands Carol a narrower sub-contract.
  const carol = await Core.create();
  const sub = await attenuate(alice.identity, contract, { subject: carol.id });
  out = await bob.handle(await compose(carol.identity, { to: bob.id, action: "invoke", resource: echo, payload: "via carol", grant: sub }));
  log(`3. carol via attenuated contract  -> ${render(out)}`);

  // 4. Delegation cannot widen what it was given.
  const widened = await attenuate(alice.identity, contract, { subject: carol.id, resource: bob.resource("tool/*") });
  out = await bob.handle(await compose(carol.identity, { to: bob.id, action: "invoke", resource: bob.resource("tool/secret"), payload: "x", grant: widened }));
  log(`4. carol tries to widen it        -> ${render(out)}`);

  // 5. Replay is rejected.
  const replayable = await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "again", grant: contract });
  await bob.handle(replayable);
  out = await bob.handle(replayable);
  log(`5. replayed message               -> ${render(out)}`);

  // 6. A working group: wire cores by mutual contracts, then collaborate.
  const wg = await WorkingGroup.form([{ owner: bob, path: "tool/echo", action: "invoke", to: alice }]);
  out = await wg.invoke(alice, bob, "tool/echo", "invoke", "group call");
  log(`6. alice -> bob inside a group    -> ${render(out)}`);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
