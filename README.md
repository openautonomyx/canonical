# The Canonical Autonomyx

The reference agent core for AutonomyX — and the trust model underneath it.

**Zero dependencies. Zero trust. Native at the edge.**

```
core    — the single, independent, complete working unit (the trust boundary)
runtime — userland built on the core (working groups, collaboration)
```

---

## The one idea

> **Trust is not based on origin. The only trust is an explicit CORS contract,
> held by the provider and enforced at the edge.**

Everything else in this repository follows from that sentence.

---

## Vocabulary

These are the words the system is built from. They were chosen precisely.

| Term | Meaning here |
| --- | --- |
| **Canonical** | The one authoritative, normalized, reference form. Not *a* version — *the* version that everything else reduces to. |
| **Core** | The single, independent, complete working unit. Like a CPU core: it executes and decides on its own. It is the **trusted computing base** — and nothing more. |
| **Edge** | The boundary of a core, and the *only* place a trust decision is made. Native: the decision happens at the boundary itself, with no central authority and no round-trip. |
| **CORS** | Cross/same **O**rigin **R**esource **S**haring. The one trust primitive: a signed contract by which a provider explicitly shares a resource. **Only CORS is trusted.** |
| **Contract** | What a CORS grant *is*. Only the **provider** (the resource owner) holds the authority to originate one; consumers only ever hold delegated sub-contracts. |
| **Origin / Identity** | A namespace and a public key. It confers **no trust by itself**. |
| **Working group** | A set of cores wired together by mutual contracts over shared resources. The unit of collaboration — formed *around* cores, never *inside* them. |

---

## Two security models, fused

Classic web **same-origin** trusts whatever shares your origin. **Zero-trust**
trusts nothing by location and verifies everything on its own merits. They look
opposite — so we keep only the half that survives scrutiny:

- Being from the same origin grants **nothing** (against same-origin trust).
- A request is honored **only** when it carries a valid, provider-signed
  contract, re-verified on every call at the edge (zero-trust).

The result is plain CORS turned into a trust primitive: the provider decides
exactly who may touch exactly what, the decision is a signed object that travels
with the request, and the edge — not a central server — enforces it.

---

## Why the core is tiny (and stays that way)

> **The moment you expand the core, you dilute the trust.**

The core is the trusted computing base. Every line inside it is a line every
consumer is forced to trust, and a cost paid on every single request. So:

- **Built, not composed.** The core has **zero third-party runtime
  dependencies** — native Ed25519 via Web Crypto, native encoding. There is no
  SBOM to audit and no supply chain to compromise.
- **Small, and shrinking.** Capability is added in `runtime/` (userland), which
  the core verifies. It is never added to the core. A test
  (`tests/core-minimal.test.ts`) fails the build if a core file imports
  userland or any third-party package.
- **Independent and complete.** A core can make a full trust decision alone: the
  contract travels with the request, so there is no lookup, no authority to
  phone home to, nothing else to trust.

This is a deliberate, full-stack design decision: own the whole stack, depend on
nothing, and the trust boundary stays auditable in a single sitting.

---

## How a request is decided

Every request passes the edge's default-deny gate, in order:

1. **Integrity** — the sender really signed exactly this message.
2. **Addressing** — it is for this core, for a resource this core owns.
3. **Freshness** — it is within the time window.
4. **Replay** — its nonce has not been seen.
5. **CORS** — it carries a contract that (a) names the sender, (b) authorizes
   this action on this resource, and (c) chains, by valid signatures and
   monotone attenuation, all the way back to the provider.

No grant, no access — even from the same origin.

```
 sender ──signed message (+ contract) ──►  ┌──────── target core ────────┐
                                           │  edge.admit()  ◄── the only  │
                                           │   1 integrity     trust      │
                                           │   2 addressing    decision   │
                                           │   3 freshness                │
                                           │   4 replay                   │
                                           │   5 CORS contract            │
                                           │        │ allow               │
                                           │        ▼                     │
                                           │     handler(payload)         │
                                           └──────────────────────────────┘
```

---

## Quickstart

Requires **Node ≥ 22.6** (runs TypeScript natively; no build step needed to run).

```bash
npm install        # only dev tools (typescript, @types/node); zero runtime deps
npm run demo       # a guided tour of the whole stack
npm test           # the full suite, on Node's built-in test runner
npm run typecheck  # tsc, no emit
npm run build      # emit dist/ (.js + .d.ts) for consumers
```

### In code

```ts
import { Core, compose, attenuate } from "./src/core/index.ts";

const alice = await Core.create();
const bob = await Core.create();

// Bob owns a resource. Owning is not sharing.
bob.expose("tool/echo", (payload) => ({ echo: payload }));
const echo = bob.resource("tool/echo");

// Same process, but origin grants nothing — this is DENIED.
await bob.handle(await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi" }));

// Bob issues a CORS contract to Alice — now it is ALLOWED.
const contract = await bob.share(alice.id, "tool/echo", "invoke");
const out = await bob.handle(
  await compose(alice.identity, { to: bob.id, action: "invoke", resource: echo, payload: "hi", grant: contract }),
);
// out.decision.ok === true, out.result === { echo: "hi" }

// Alice can delegate a NARROWER sub-contract to Carol; she can never widen it.
const sub = await attenuate(alice.identity, contract, { subject: (await Core.create()).id });
```

---

## Layout

```
src/
  core/            # the trusted computing base — zero deps, never grows
    codec.ts       #   deterministic encoding (canonical bytes, base64url)
    identity.ts    #   Ed25519 identities (origins) over native Web Crypto
    resource.ts    #   resource addressing "<ownerId>:<path>" + subsumption
    cors.ts        #   the contract: issue, attenuate, verify (the only trust)
    message.ts     #   the only way to reach a resource
    edge.ts        #   the boundary; where CORS is enforced; default-deny
    core.ts        #   the single independent complete working unit
  runtime/         # userland, built on the core; the core verifies it
    working-group.ts
examples/demo.ts   # runnable tour
tests/             # Node's built-in runner; no test framework dependency
```

---

## Invariants

- The core has **zero** third-party runtime dependencies.
- The core never imports from `runtime/`.
- Origin is identity only; it never grants access.
- Every resource access is default-deny and requires a provider-rooted contract.
- Delegation may only narrow a contract, never widen it.
- The edge enforces; nothing trusts the network or location.

## Roadmap

- Threshold / rotating provider keys (no single long-lived root).
- Transports (the contract is transport-agnostic; messages are just bytes).
- Revocation and short-lived contracts as first-class caveats.
- A WASM build of the core for non-JS edges.
