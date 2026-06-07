// The Canonical Autonomyx.
//
//   core    — the single, independent, complete working unit; the trust
//             boundary. Minimal and dependency-free by design: the smaller it
//             is, the more it can be trusted and the faster it runs.
//   runtime — userland built on the core. Not trusted; the core verifies it.
//
// Trust is not based on origin. The only trust is an explicit CORS contract,
// held by the provider and enforced at the edge. Expanding the core dilutes the
// trust and costs performance — so it stays small, and capability grows around
// it, never inside it.

export * as core from "./core/index.ts";
export * as runtime from "./runtime/index.ts";
