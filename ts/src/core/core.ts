// The Canonical Core — the single, independent, complete working unit.
//
// One identity. One edge. A handful of resources. One operation: handle a
// message — decide at the edge (CORS), and, only if admitted, execute.
//
// This is the whole trusted computing base. It needs no other party to make a
// trust decision, and it has zero third-party dependencies — it is built, not
// composed, so there is no supply chain to trust and nothing to dilute it.
//
// Trust is inversely proportional to the size of this unit. Every line added
// here is a line every consumer is forced to trust, and a cost paid on every
// request. So it stays small. Build new capability *around* the core (see
// ../runtime), never *into* it.

import { generateSigner, type Signer } from "./identity.ts";
import { Edge, type Decision } from "./edge.ts";
import { issueGrant, type Caveat, type CorsGrant } from "./cors.ts";
import { makeResource, resourcePath } from "./resource.ts";
import type { Message } from "./message.ts";

export type Handler = (payload: unknown, msg: Message) => unknown | Promise<unknown>;

export interface CoreOptions {
  now?: () => number;
  freshnessWindowMs?: number;
}

export interface ShareOptions {
  caveats?: readonly Caveat[];
  now?: number;
}

export interface Outcome {
  readonly decision: Decision;
  readonly result?: unknown;
}

export class Core {
  readonly identity: Signer;
  private readonly edge: Edge;
  private readonly resources = new Map<string, Handler>();

  private constructor(identity: Signer, opts: CoreOptions) {
    this.identity = identity;
    this.edge = new Edge({ ownerId: identity.id, now: opts.now, freshnessWindowMs: opts.freshnessWindowMs });
  }

  /** Genesis: a new, self-sufficient unit with its own origin. */
  static async create(opts: CoreOptions = {}): Promise<Core> {
    return new Core(await generateSigner(), opts);
  }

  get id(): string {
    return this.identity.id;
  }

  /** Publish a resource this unit owns. Owning is not sharing — see `share`. */
  expose(path: string, handler: Handler): this {
    this.resources.set(path, handler);
    return this;
  }

  /** The full address ("<thisId>:<path>") others target to reach a resource. */
  resource(path: string): string {
    return makeResource(this.identity.id, path);
  }

  /** Explicitly share one of our resources — originate a CORS contract. */
  async share(subject: string, path: string, action: string, opts: ShareOptions = {}): Promise<CorsGrant> {
    return issueGrant(
      this.identity,
      { subject, action, resource: this.resource(path), caveats: opts.caveats },
      opts.now,
    );
  }

  /** Receive a message: decide at the edge, then execute only if admitted. */
  async handle(msg: Message): Promise<Outcome> {
    const decision = await this.edge.admit(msg);
    if (!decision.ok) return { decision };
    const handler = this.resources.get(resourcePath(msg.resource));
    if (!handler) return { decision, result: undefined };
    const result = await handler(msg.payload, msg);
    return { decision, result };
  }
}
