// Canonical Core — codec.
//
// Deterministic encoding so that signatures are stable and verifiable anywhere.
// Every signature in the core is taken over `canonicalBytes(...)`. Part of the
// trusted core: keep it tiny and dependency-free.

/** Bytes backed by a (non-shared) ArrayBuffer — what Web Crypto accepts. */
export type Bytes = Uint8Array<ArrayBuffer>;

export function b64url(bytes: Uint8Array): string {
  let bin = "";
  for (const b of bytes) bin += String.fromCharCode(b);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

export function fromB64url(s: string): Bytes {
  let t = s.replace(/-/g, "+").replace(/_/g, "/");
  const pad = t.length % 4;
  if (pad) t += "=".repeat(4 - pad);
  const bin = atob(t);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

const enc = new TextEncoder();

// Deterministic JSON: keys sorted, `undefined` dropped, only finite numbers.
// Two equal values always produce identical bytes, on any engine.
export function canonicalize(value: unknown): string {
  if (value === null) return "null";
  const t = typeof value;
  if (t === "number") {
    if (!Number.isFinite(value as number)) throw new Error("cannot canonicalize a non-finite number");
    return JSON.stringify(value);
  }
  if (t === "string" || t === "boolean") return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(canonicalize).join(",") + "]";
  if (t === "object") {
    const o = value as Record<string, unknown>;
    const keys = Object.keys(o).filter((k) => o[k] !== undefined).sort();
    return "{" + keys.map((k) => JSON.stringify(k) + ":" + canonicalize(o[k])).join(",") + "}";
  }
  throw new Error("cannot canonicalize value of type " + t);
}

export function canonicalBytes(value: unknown): Bytes {
  return enc.encode(canonicalize(value));
}

export function randomNonce(bytes = 16): string {
  return b64url(crypto.getRandomValues(new Uint8Array(bytes)));
}
