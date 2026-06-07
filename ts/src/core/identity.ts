// Canonical Core — identity.
//
// Ed25519 over native Web Crypto. No dependencies, runs in Node / Deno / Workers
// / the browser. An identity IS an origin: a namespace and a public key. It
// confers NO trust by itself — trust is not based on origin (see ./cors.ts).

import { b64url, fromB64url, canonicalBytes, type Bytes } from "./codec.ts";

const ALG = { name: "Ed25519" } as const;

export interface Identity {
  /** base64url(raw 32-byte Ed25519 public key) — the origin. */
  readonly id: string;
}

export interface Signer extends Identity {
  sign(data: Bytes): Promise<string>;
}

/** Mint a fresh identity. The private key never leaves the returned closure. */
export async function generateSigner(): Promise<Signer> {
  const kp = (await crypto.subtle.generateKey(ALG, true, ["sign", "verify"])) as CryptoKeyPair;
  const raw = new Uint8Array(await crypto.subtle.exportKey("raw", kp.publicKey));
  const id = b64url(raw);
  return {
    id,
    async sign(data: Bytes): Promise<string> {
      const sig = await crypto.subtle.sign(ALG, kp.privateKey, data);
      return b64url(new Uint8Array(sig));
    },
  };
}

const pubCache = new Map<string, CryptoKey>();

async function importPublic(id: string): Promise<CryptoKey> {
  const cached = pubCache.get(id);
  if (cached) return cached;
  const key = await crypto.subtle.importKey("raw", fromB64url(id), ALG, true, ["verify"]);
  pubCache.set(id, key);
  return key;
}

export async function verifySignature(signerId: string, data: Bytes, sig: string): Promise<boolean> {
  try {
    const key = await importPublic(signerId);
    return await crypto.subtle.verify(ALG, key, fromB64url(sig), data);
  } catch {
    return false;
  }
}

// A signed record carries its signature in `sig`, over the canonical bytes of
// the rest of the object. This is how every primitive in the core is signed.
export async function signRecord<T extends object>(signer: Signer, body: T): Promise<T & { sig: string }> {
  const sig = await signer.sign(canonicalBytes(body));
  return { ...body, sig };
}

export async function verifyRecord<T extends { sig: string }>(signerId: string, record: T): Promise<boolean> {
  const { sig, ...body } = record as { sig: string } & Record<string, unknown>;
  return verifySignature(signerId, canonicalBytes(body), sig);
}
