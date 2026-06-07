import { test } from "node:test";
import assert from "node:assert/strict";
import { Core } from "../src/core/index.ts";
import { WorkingGroup } from "../src/runtime/index.ts";

test("wires cores by contracts and collaborates", async () => {
  const a = await Core.create();
  const b = await Core.create();
  b.expose("tool/echo", (p) => ({ echo: p }));
  const wg = await WorkingGroup.form([{ owner: b, path: "tool/echo", action: "invoke", to: a }]);
  assert.equal(wg.members.length, 2);
  const out = await wg.invoke(a, b, "tool/echo", "invoke", "hello");
  assert.equal(out.decision.ok, true);
  assert.deepEqual(out.result, { echo: "hello" });
});

test("a member outside the shared contract is still denied", async () => {
  const a = await Core.create();
  const b = await Core.create();
  const c = await Core.create();
  b.expose("tool/echo", (p) => ({ echo: p }));
  const wg = await WorkingGroup.form([{ owner: b, path: "tool/echo", action: "invoke", to: a }]);
  // c was never granted anything; invoke carries no contract for c.
  const out = await wg.invoke(c, b, "tool/echo", "invoke", "sneak");
  assert.equal(out.decision.ok, false);
});
