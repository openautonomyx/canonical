// Runtime (userland — NOT part of the trusted core).
//
// Built on the core; the core verifies everything it does. A working group is a
// set of cores wired together by mutual CORS contracts over shared resources.
// Forming a group expands no core — it only issues contracts that each core's
// edge independently verifies. This is where capability grows, so that the core
// does not have to.

import { compose, type Core, type CorsGrant, type Outcome } from "../core/index.ts";

export interface Share {
  /** the provider that owns the resource */
  owner: Core;
  /** the resource path on the owner */
  path: string;
  /** the action being shared, e.g. "invoke" */
  action: string;
  /** the consumer the resource is shared with */
  to: Core;
}

export class WorkingGroup {
  private readonly memberSet = new Set<Core>();
  private readonly contracts = new Map<string, CorsGrant>();

  get members(): Core[] {
    return [...this.memberSet];
  }

  /** Wire a set of cores together by issuing the shares between them. */
  static async form(shares: Share[]): Promise<WorkingGroup> {
    const wg = new WorkingGroup();
    for (const s of shares) {
      wg.memberSet.add(s.owner);
      wg.memberSet.add(s.to);
      const contract = await s.owner.share(s.to.id, s.path, s.action);
      wg.contracts.set(key(s.to.id, s.owner.resource(s.path), s.action), contract);
    }
    return wg;
  }

  contractFor(holder: string, resource: string, action: string): CorsGrant | undefined {
    return this.contracts.get(key(holder, resource, action));
  }

  /** A member invokes another member's resource using the group's contract. */
  async invoke(holder: Core, target: Core, path: string, action: string, payload?: unknown): Promise<Outcome> {
    const resource = target.resource(path);
    const grant = this.contractFor(holder.id, resource, action);
    const msg = await compose(holder.identity, { to: target.id, action, resource, payload, grant });
    return target.handle(msg);
  }
}

function key(holder: string, resource: string, action: string): string {
  return `${holder}|${resource}|${action}`;
}
