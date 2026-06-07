import { test } from "node:test";
import assert from "node:assert/strict";
import { generateSigner, signRecord, verifyRecord, verifySignature, canonicalBytes } from "../src/core/index.ts";

test("signs and verifies raw data", async () => {
  const s = await generateSigner();
  const data = new TextEncoder().encode("x");
  const sig = await s.sign(data);
  assert.equal(await verifySignature(s.id, data, sig), true);
});

test("rejects tampered data", async () => {
  const s = await generateSigner();
  const sig = await s.sign(canonicalBytes({ a: 1 }));
  assert.equal(await verifySignature(s.id, canonicalBytes({ a: 2 }), sig), false);
});

test("records: verify true, tamper false, key order irrelevant", async () => {
  const s = await generateSigner();
  const rec = await signRecord(s, { hello: "world", n: 3 });
  assert.equal(await verifyRecord(s.id, rec), true);
  // canonicalization sorts keys, so a reordered-but-equal record still verifies
  assert.equal(await verifyRecord(s.id, { n: 3, hello: "world", sig: rec.sig }), true);
  assert.equal(await verifyRecord(s.id, { ...rec, hello: "mars" }), false);
});

test("a different signer cannot verify", async () => {
  const a = await generateSigner();
  const b = await generateSigner();
  const rec = await signRecord(a, { x: 1 });
  assert.equal(await verifyRecord(b.id, rec), false);
});
