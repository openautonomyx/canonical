# @autonomyx/canonical (TypeScript runtime)

The TypeScript layer of [The Canonical Autonomyx](../README.md). It mirrors the
Rust `canonical-core` primitive exactly — same Ed25519 identities, same
canonical-JSON records, same CORS contracts — and adds the runtime layer
(working groups). The two cores interoperate byte-for-byte.

**Zero runtime dependencies.** Native Ed25519 via Web Crypto, native encoding.
Runs on Node / Deno / Workers / the browser.

```
src/core/      identity · resource · cors (contract) · message · edge · Core   (the TCB)
src/runtime/   working-group                                                   (userland)
examples/      demo.ts — a runnable tour
tests/         Node's built-in runner; no test-framework dependency
```

## Quickstart

Requires **Node ≥ 22.6** (runs TypeScript natively; no build step to run).

```bash
npm install        # dev tools only (typescript, @types/node)
npm run demo       # a guided tour of the whole stack
npm test           # the full suite, on Node's built-in test runner
npm run typecheck  # tsc, no emit
npm run build      # emit dist/ (.js + .d.ts) for consumers
```

## In code

```ts
import { Core, compose, attenuate } from "./src/core/index.ts";

const alice = await Core.create();
const bob = await Core.create();

bob.expose("tool/echo", (payload) => ({ echo: payload }));
const echo = bob.resource("tool/echo");

// Same process, but origin grants nothing — DENIED.
await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi" }));

// Bob issues a CORS contract to Alice — now ALLOWED.
const contract = await bob.share(alice.id, "tool/echo", "invoke");
const out = await bob.handle(
  await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant: contract }),
);
// out.decision.ok === true, out.result === { echo: "hi" }

// Alice delegates a NARROWER sub-contract to Carol; she can never widen it.
const sub = await attenuate(alice.identity, contract, { subject: (await Core.create()).id });
```

## Invariants

- Zero third-party runtime dependencies.
- The core (`src/core/`) never imports from `src/runtime/` — enforced by
  `tests/core-minimal.test.ts`.
- Origin is identity only; it never grants access.
- Every resource access is default-deny and requires a provider-rooted contract.
- Delegation may only narrow a contract, never widen it.
