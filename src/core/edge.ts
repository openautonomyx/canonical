// Canonical Core — the edge.
//
// CORS is enforced here, and nowhere else. The edge is the whole of a
// provider's authority and the entire trust boundary. It is default-deny.
//
// Verification is local and self-contained: a message carries its own proof
// (the contract), so the edge needs no external lookup, no central authority,
// and no network round-trip. The edge is native — the trust decision happens
// at the boundary itself.

import { verifyRecord } from "./identity.ts";
import { authorizes, verifyGrant } from "./cors.ts";
import { resourceOwner } from "./resource.ts";
import type { Message } from "./message.ts";

export interface Decision {
  readonly ok: boolean;
  readonly reason?: string;
}

const ALLOW: Decision = { ok: true };
function deny(reason: string): Decision {
  return { ok: false, reason };
}

export interface EdgeOptions {
  readonly ownerId: string; // the provider this edge guards
  readonly now?: () => number;
  readonly freshnessWindowMs?: number;
}

export class Edge {
  readonly ownerId: string;
  private readonly now: () => number;
  private readonly windowMs: number;
  private readonly seen = new Set<string>();

  constructor(opts: EdgeOptions) {
    this.ownerId = opts.ownerId;
    this.now = opts.now ?? (() => Date.now());
    this.windowMs = opts.freshnessWindowMs ?? 60_000;
  }

  /** The complete trust decision for one request — pure CORS, at the boundary. */
  async admit(msg: Message): Promise<Decision> {
    const now = this.now();
    // 1. Integrity: the sender really sent exactly this, untampered.
    if (!(await verifyRecord(msg.from, msg))) return deny("forged or tampered message");
    // 2. Addressed to us, for a resource we own.
    if (msg.to !== this.ownerId) return deny("message not addressed to this edge");
    if (resourceOwner(msg.resource) !== this.ownerId) return deny("resource not owned by this edge");
    // 3. Freshness.
    if (Math.abs(now - msg.issuedAt) > this.windowMs) return deny("message outside freshness window");
    // 4. Replay.
    if (this.seen.has(msg.nonce)) return deny("replayed nonce");
    // 5. CORS — the only trust. No contract, no access. Origin grants nothing.
    if (!msg.grant) return deny(`no CORS contract for ${msg.action} ${msg.resource}`);
    if (msg.grant.subject !== msg.from) return deny("contract not held by the sender");
    if (!authorizes(msg.grant, msg.action, msg.resource)) return deny("contract does not authorize this action/resource");
    const g = await verifyGrant(msg.grant, this.ownerId, now);
    if (!g.ok) return deny("invalid contract: " + g.reason);
    // Consume the nonce only once the request is fully admitted.
    this.seen.add(msg.nonce);
    return ALLOW;
  }
}
