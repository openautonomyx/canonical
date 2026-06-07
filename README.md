# The Canonical Autonomyx

The reference agent **core** for AutonomyX — and the trust model underneath it.

**Zero dependencies. Zero trust. Native at the edge. Built, not composed.**

```
crates/canonical-core   Rust — the single, independent, complete working unit (the trusted core)
crates/edge-cli         Rust — a std-only command line at the edge
ts/                     TypeScript — the runtime layer, sharing the same primitive
```

The Rust core and the TypeScript core are **one trust domain**: a contract signed
in one is verified, byte-for-byte, in the other (proven below).

---

## The one idea

> **Trust is not based on origin. The only trust is an explicit CORS contract,
> held by the provider and enforced at the edge.**

Everything in this repository follows from that sentence.

---

## Vocabulary

| Term | Meaning here |
| --- | --- |
| **Canonical** | The one authoritative, normalized, reference form — *the* version everything else reduces to. |
| **Core** | The single, independent, complete working unit. Like a CPU/kernel core: it executes and decides on its own. It is the **trusted computing base** — and nothing more. |
| **Edge** | The boundary of a core, and the *only* place a trust decision is made. Native: decided at the boundary, no central authority, no round-trip. |
| **CORS** | (cross/same) **O**rigin **R**esource **S**haring — the one trust primitive: a signed contract by which a provider explicitly shares a resource. **Only CORS is trusted.** |
| **Contract** | What a CORS grant *is*. Only the **provider** (the resource owner) can originate one; consumers hold only delegated, monotonically narrower sub-contracts. |
| **Origin / Identity** | A namespace and a public key. It confers **no trust by itself**. |
| **Working group** | A set of cores wired together by mutual contracts — collaboration formed *around* cores, never *inside* them. |

---

## Why the core is tiny — and stays that way

> **The moment you expand the core, you dilute the trust.** (And you pay for it
> on every request.)

The core is the trusted computing base. So:

- **Built, not composed.** `canonical-core` has **zero third-party dependencies**
  — Ed25519 and SHA-512 are implemented in-tree. There is no SBOM to audit and
  no supply chain to compromise. CI fails if a dependency ever appears.
- **Small, and shrinking.** Capability is added *around* the core (working
  groups, runtime), which the core verifies. It is never added *into* it.
- **Independent and complete.** A core makes a full trust decision alone: the
  contract travels with the request — no lookup, no authority to phone home to.

This is the deliberate, full-stack design decision: own the whole stack, depend
on nothing, keep the trust boundary auditable in a single sitting.

---

## How a request is decided

Every request passes the edge's default-deny gate, in order:

1. **Integrity** — the sender really signed exactly this message.
2. **Addressing** — it is for this core, for a resource this core owns.
3. **Freshness** — within the time window.
4. **Replay** — its nonce has not been seen.
5. **CORS** — it carries a contract that names the sender, authorizes this action
   on this resource, and chains, by valid signatures and monotone attenuation,
   back to the provider.

No grant, no access — even from the same origin.

---

## Polyglot, one trust domain

Both cores implement the *same* primitive: Ed25519 over canonical-JSON records.
They are verified to interoperate:

- A signature produced by the Rust core is accepted by Node's independent Web
  Crypto (OpenSSL) — so the Rust crypto is RFC-8032 correct.
- A full CORS **contract** signed in Rust verifies in the TypeScript core,
  because both produce byte-identical canonical bytes:

  ```
  {"action":"invoke","issuedAt":1700000000000,"issuer":"…","resource":"…:tool/echo","subject":"…"}
  ```

---

## Quickstart

### Rust core (`crates/`)

```bash
cargo test --workspace      # crypto vectors + the full trust model
cargo run -p edge-cli -- demo     # a guided allow/deny tour
cargo run -p edge-cli -- keygen   # generate an identity (seed + id)
```

```
$ edge demo
1. invoke, no contract            -> DENY   (no CORS contract for invoke …:tool/echo)
2. invoke with Bob's contract     -> ALLOW  {"echo":"hi"}
3. carol via attenuated contract  -> ALLOW  {"echo":"via carol"}
4. carol tries to widen it        -> DENY   (invalid contract: delegation widens the resource)
5. replayed message               -> DENY   (replayed nonce)
```

### TypeScript runtime (`ts/`)

Requires **Node ≥ 22.6** (runs TypeScript natively; zero runtime dependencies).

```bash
cd ts
npm install        # dev tools only (typescript, @types/node)
npm run demo       # the same tour, plus working groups
npm test           # the full suite, on Node's built-in test runner
```

See [`ts/README.md`](ts/README.md) for the TypeScript API.

---

## Invariants

- The core has **zero** third-party dependencies (enforced in CI).
- Origin is identity only; it never grants access.
- Every resource access is default-deny and requires a provider-rooted contract.
- Delegation may only narrow a contract, never widen it.
- The edge enforces; nothing trusts the network or location.
- Rust and TypeScript cores produce identical signed bytes — one trust domain.

## Roadmap

- Threshold / rotating provider keys (no single long-lived root).
- Transports (the contract is transport-agnostic; messages are just bytes).
- Revocation and short-lived contracts as first-class caveats.
- `no_std` / WASM build of the Rust core for non-JS edges.
