import { test } from "node:test";
import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

// The core is the trusted computing base. It must stay the single independent
// unit: it may never depend on userland (runtime), or on any third-party
// package. These tests fail the build if the core starts to grow outward.

const coreDir = fileURLToPath(new URL("../src/core/", import.meta.url));

const importRe = /(?:from|import)\s+["']([^"']+)["']/g;

function coreFiles(): string[] {
  return readdirSync(coreDir).filter((f) => f.endsWith(".ts"));
}

function importsOf(file: string): string[] {
  const src = readFileSync(coreDir + file, "utf8");
  return [...src.matchAll(importRe)].map((m) => m[1]!);
}

test("the core never imports from userland (runtime)", () => {
  for (const f of coreFiles()) {
    for (const spec of importsOf(f)) {
      assert.ok(!spec.includes("runtime"), `${f} must not depend on runtime (imports '${spec}')`);
    }
  }
});

test("the core imports nothing but other core files and node: builtins", () => {
  for (const f of coreFiles()) {
    for (const spec of importsOf(f)) {
      const isRelativeCore = spec.startsWith("./") && spec.endsWith(".ts");
      const isNodeBuiltin = spec.startsWith("node:");
      assert.ok(isRelativeCore || isNodeBuiltin, `${f}: forbidden import '${spec}' (no third-party deps in the core)`);
    }
  }
});
