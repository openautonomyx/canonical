// Canonical Core — message.
//
// The only way to reach a resource. Signed by the sender and, when the resource
// is not the sender's own, carrying the CORS contract that authorizes the call.

import { randomNonce } from "./codec.ts";
import { signRecord, type Signer } from "./identity.ts";
import type { CorsGrant } from "./cors.ts";

export interface Message {
  readonly from: string;
  readonly to: string; // target provider id
  readonly action: string;
  readonly resource: string; // "<ownerId>:<path>"
  readonly payload: unknown;
  readonly nonce: string;
  readonly issuedAt: number;
  readonly grant?: CorsGrant;
  readonly sig: string;
}

export interface ComposeParams {
  to: string;
  action: string;
  resource: string;
  payload?: unknown;
  grant?: CorsGrant;
}

export async function compose(from: Signer, params: ComposeParams, now: number = Date.now()): Promise<Message> {
  const body = {
    from: from.id,
    to: params.to,
    action: params.action,
    resource: params.resource,
    payload: params.payload,
    nonce: randomNonce(),
    issuedAt: now,
    grant: params.grant,
  };
  return signRecord(from, body);
}
