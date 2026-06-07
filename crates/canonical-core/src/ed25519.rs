//! Ed25519 (RFC 8032) signatures, implemented in-tree with no dependencies.
//!
//! This is a faithful port of the TweetNaCl reference implementation (Bernstein
//! et al., public domain) into safe Rust. We build the crypto rather than pull a
//! crate so the trusted core has no supply chain. Correctness is checked by
//! round-trip tests here, and interoperability with an independent
//! implementation (the TypeScript core / OpenSSL) is verified out of band.
//!
//! Field elements are 16 little-endian limbs of radix 2^16, exactly as in
//! TweetNaCl; the arithmetic stays comfortably within i64.

use crate::sha512::sha512;

type Gf = [i64; 16];

const GF0: Gf = [0; 16];
const GF1: Gf = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const D: Gf = [
    0x78a3, 0x1359, 0x4dca, 0x75eb, 0xd8ab, 0x4141, 0x0a4d, 0x0070, 0xe898, 0x7779, 0x4079, 0x8cc7,
    0xfe73, 0x2b6f, 0x6cee, 0x5203,
];
const D2: Gf = [
    0xf159, 0x26b2, 0x9b94, 0xebd6, 0xb156, 0x8283, 0x149a, 0x00e0, 0xd130, 0xeef3, 0x80f2, 0x198e,
    0xfce7, 0x56df, 0xd9dc, 0x2406,
];
const X: Gf = [
    0xd51a, 0x8f25, 0x2d60, 0xc956, 0xa7b2, 0x9525, 0xc760, 0x692c, 0xdc5c, 0xfdd6, 0xe231, 0xc0a4,
    0x53fe, 0xcd6e, 0x36d3, 0x2169,
];
const Y: Gf = [
    0x6658, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666,
    0x6666, 0x6666, 0x6666, 0x6666,
];
const I: Gf = [
    0xa0b0, 0x4a0e, 0x1b27, 0xc4ee, 0xe478, 0xad2f, 0x1806, 0x2f43, 0xd7a7, 0x3dfb, 0x0099, 0x2b4d,
    0xdf0b, 0x4fc1, 0x2480, 0x2b83,
];

const L: [i64; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
];

fn car25519(o: &mut Gf) {
    for i in 0..16 {
        o[i] += 1 << 16;
        let c = o[i] >> 16;
        if i < 15 {
            o[i + 1] += c - 1;
        } else {
            // top limb folds back into limb 0 as (c-1) + 37*(c-1) = 38*(c-1)
            o[0] += 38 * (c - 1);
        }
        o[i] -= c << 16;
    }
}

fn sel25519(p: &mut Gf, q: &mut Gf, b: i64) {
    let c = !(b - 1);
    for i in 0..16 {
        let t = c & (p[i] ^ q[i]);
        p[i] ^= t;
        q[i] ^= t;
    }
}

fn pack25519(n: &Gf) -> [u8; 32] {
    let mut t = *n;
    car25519(&mut t);
    car25519(&mut t);
    car25519(&mut t);
    for _ in 0..2 {
        let mut m: Gf = GF0;
        m[0] = t[0] - 0xffed;
        for i in 1..15 {
            m[i] = t[i] - 0xffff - ((m[i - 1] >> 16) & 1);
            m[i - 1] &= 0xffff;
        }
        m[15] = t[15] - 0x7fff - ((m[14] >> 16) & 1);
        let b = (m[15] >> 16) & 1;
        m[14] &= 0xffff;
        sel25519(&mut t, &mut m, 1 - b);
    }
    let mut o = [0u8; 32];
    for i in 0..16 {
        o[2 * i] = (t[i] & 0xff) as u8;
        o[2 * i + 1] = (t[i] >> 8) as u8;
    }
    o
}

fn unpack25519(n: &[u8; 32]) -> Gf {
    let mut o: Gf = GF0;
    for i in 0..16 {
        o[i] = (n[2 * i] as i64) + ((n[2 * i + 1] as i64) << 8);
    }
    o[15] &= 0x7fff;
    o
}

fn add_fe(a: &Gf, b: &Gf) -> Gf {
    let mut o = GF0;
    for i in 0..16 {
        o[i] = a[i] + b[i];
    }
    o
}

fn sub_fe(a: &Gf, b: &Gf) -> Gf {
    let mut o = GF0;
    for i in 0..16 {
        o[i] = a[i] - b[i];
    }
    o
}

fn mul_fe(a: &Gf, b: &Gf) -> Gf {
    let mut t = [0i64; 31];
    for i in 0..16 {
        for j in 0..16 {
            t[i + j] += a[i] * b[j];
        }
    }
    for i in 0..15 {
        t[i] += 38 * t[i + 16];
    }
    let mut o: Gf = GF0;
    o[..16].copy_from_slice(&t[..16]);
    car25519(&mut o);
    car25519(&mut o);
    o
}

fn sq_fe(a: &Gf) -> Gf {
    mul_fe(a, a)
}

fn inv25519(i: &Gf) -> Gf {
    let mut c = *i;
    for a in (0..=253).rev() {
        c = sq_fe(&c);
        if a != 2 && a != 4 {
            c = mul_fe(&c, i);
        }
    }
    c
}

fn pow2523(i: &Gf) -> Gf {
    let mut c = *i;
    for a in (0..=250).rev() {
        c = sq_fe(&c);
        if a != 1 {
            c = mul_fe(&c, i);
        }
    }
    c
}

fn eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut d = 0u8;
    for i in 0..32 {
        d |= a[i] ^ b[i];
    }
    d == 0
}

fn neq25519(a: &Gf, b: &Gf) -> bool {
    !eq32(&pack25519(a), &pack25519(b))
}

fn par25519(a: &Gf) -> u8 {
    pack25519(a)[0] & 1
}

// A point is [X, Y, Z, T] in extended twisted Edwards coordinates.
fn point_add(p: &mut [Gf; 4], q: &[Gf; 4]) {
    let a = mul_fe(&sub_fe(&p[1], &p[0]), &sub_fe(&q[1], &q[0]));
    let b = mul_fe(&add_fe(&p[0], &p[1]), &add_fe(&q[0], &q[1]));
    let mut c = mul_fe(&p[3], &q[3]);
    c = mul_fe(&c, &D2);
    let mut d = mul_fe(&p[2], &q[2]);
    d = add_fe(&d, &d);
    let e = sub_fe(&b, &a);
    let f = sub_fe(&d, &c);
    let g = add_fe(&d, &c);
    let h = add_fe(&b, &a);
    p[0] = mul_fe(&e, &f);
    p[1] = mul_fe(&h, &g);
    p[2] = mul_fe(&g, &f);
    p[3] = mul_fe(&e, &h);
}

fn cswap(p: &mut [Gf; 4], q: &mut [Gf; 4], b: u8) {
    for i in 0..4 {
        sel25519(&mut p[i], &mut q[i], b as i64);
    }
}

fn pack_point(p: &[Gf; 4]) -> [u8; 32] {
    let zi = inv25519(&p[2]);
    let tx = mul_fe(&p[0], &zi);
    let ty = mul_fe(&p[1], &zi);
    let mut r = pack25519(&ty);
    r[31] ^= par25519(&tx) << 7;
    r
}

fn scalarmult(q: &[Gf; 4], s: &[u8; 32]) -> [Gf; 4] {
    let mut p: [Gf; 4] = [GF0, GF1, GF1, GF0];
    let mut q = *q;
    for i in (0..=255).rev() {
        let b = (s[i >> 3] >> (i & 7)) & 1;
        cswap(&mut p, &mut q, b);
        point_add(&mut q, &p);
        let p_copy = p;
        point_add(&mut p, &p_copy);
        cswap(&mut p, &mut q, b);
    }
    p
}

fn scalarbase(s: &[u8; 32]) -> [Gf; 4] {
    let q: [Gf; 4] = [X, Y, GF1, mul_fe(&X, &Y)];
    scalarmult(&q, s)
}

fn modl(r: &mut [u8; 32], x: &mut [i64; 64]) {
    for i in (32..64).rev() {
        let mut carry = 0i64;
        let mut j = i - 32;
        while j < i - 12 {
            x[j] += carry - 16 * x[i] * L[j - (i - 32)];
            carry = (x[j] + 128) >> 8;
            x[j] -= carry << 8;
            j += 1;
        }
        x[j] += carry;
        x[i] = 0;
    }
    let mut carry = 0i64;
    for j in 0..32 {
        x[j] += carry - (x[31] >> 4) * L[j];
        carry = x[j] >> 8;
        x[j] &= 255;
    }
    for j in 0..32 {
        x[j] -= carry * L[j];
    }
    for i in 0..32 {
        x[i + 1] += x[i] >> 8;
        r[i] = (x[i] & 255) as u8;
    }
}

fn reduce(r: &mut [u8; 64]) {
    let mut x = [0i64; 64];
    for i in 0..64 {
        x[i] = r[i] as i64;
    }
    let mut out = [0u8; 32];
    modl(&mut out, &mut x);
    r[..32].copy_from_slice(&out);
    for b in r[32..].iter_mut() {
        *b = 0;
    }
}

fn unpackneg(p: &[u8; 32]) -> Option<[Gf; 4]> {
    let mut r: [Gf; 4] = [GF0, unpack25519(p), GF1, GF0];
    let mut num = sq_fe(&r[1]);
    let mut den = mul_fe(&num, &D);
    num = sub_fe(&num, &r[2]);
    den = add_fe(&r[2], &den);
    let den2 = sq_fe(&den);
    let den4 = sq_fe(&den2);
    let den6 = mul_fe(&den4, &den2);
    let mut t = mul_fe(&den6, &num);
    t = mul_fe(&t, &den);
    t = pow2523(&t);
    t = mul_fe(&t, &num);
    t = mul_fe(&t, &den);
    t = mul_fe(&t, &den);
    r[0] = mul_fe(&t, &den);

    let mut chk = sq_fe(&r[0]);
    chk = mul_fe(&chk, &den);
    if neq25519(&chk, &num) {
        r[0] = mul_fe(&r[0], &I);
    }
    chk = sq_fe(&r[0]);
    chk = mul_fe(&chk, &den);
    if neq25519(&chk, &num) {
        return None;
    }
    if par25519(&r[0]) == (p[31] >> 7) {
        r[0] = sub_fe(&GF0, &r[0]);
    }
    r[3] = mul_fe(&r[0], &r[1]);
    Some(r)
}

/// Derive the 32-byte public key from a 32-byte seed.
pub fn public_key(seed: &[u8; 32]) -> [u8; 32] {
    let mut d = sha512(seed);
    d[0] &= 248;
    d[31] &= 127;
    d[31] |= 64;
    let mut scalar = [0u8; 32];
    scalar.copy_from_slice(&d[..32]);
    pack_point(&scalarbase(&scalar))
}

/// Sign `msg` with a 32-byte seed, returning a 64-byte detached signature.
pub fn sign(seed: &[u8; 32], msg: &[u8]) -> [u8; 64] {
    let pk = public_key(seed);
    let mut d = sha512(seed);
    d[0] &= 248;
    d[31] &= 127;
    d[31] |= 64;

    // r = H(prefix || msg), reduced.
    let mut to_hash = Vec::with_capacity(32 + msg.len());
    to_hash.extend_from_slice(&d[32..64]);
    to_hash.extend_from_slice(msg);
    let mut r = sha512(&to_hash);
    let mut r64 = [0u8; 64];
    r64.copy_from_slice(&r);
    reduce(&mut r64);
    let mut r_scalar = [0u8; 32];
    r_scalar.copy_from_slice(&r64[..32]);

    // R = r·B
    let rr = pack_point(&scalarbase(&r_scalar));

    // h = H(R || A || msg), reduced.
    let mut hmsg = Vec::with_capacity(64 + msg.len());
    hmsg.extend_from_slice(&rr);
    hmsg.extend_from_slice(&pk);
    hmsg.extend_from_slice(msg);
    let h = sha512(&hmsg);
    let mut h64 = [0u8; 64];
    h64.copy_from_slice(&h);
    reduce(&mut h64);

    // S = r + h·a  (mod L)
    let mut x = [0i64; 64];
    for i in 0..32 {
        x[i] = r64[i] as i64;
    }
    for i in 0..32 {
        for j in 0..32 {
            x[i + j] += (h64[i] as i64) * (d[j] as i64);
        }
    }
    let mut s = [0u8; 32];
    modl(&mut s, &mut x);

    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&rr);
    sig[32..].copy_from_slice(&s);
    // scrub the secret-derived hash material
    for b in r.iter_mut() {
        *b = 0;
    }
    sig
}

/// Verify a 64-byte detached signature of `msg` under public key `pk`.
pub fn verify(pk: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    // S must be a canonical scalar in [0, L); reject otherwise (malleability).
    let mut s = [0u8; 32];
    s.copy_from_slice(&sig[32..]);
    if !scalar_in_range(&s) {
        return false;
    }
    let neg_a = match unpackneg(pk) {
        Some(q) => q,
        None => return false,
    };

    let mut r = [0u8; 32];
    r.copy_from_slice(&sig[..32]);

    let mut hmsg = Vec::with_capacity(64 + msg.len());
    hmsg.extend_from_slice(&r);
    hmsg.extend_from_slice(pk);
    hmsg.extend_from_slice(msg);
    let h = sha512(&hmsg);
    let mut h64 = [0u8; 64];
    h64.copy_from_slice(&h);
    reduce(&mut h64);
    let mut h_scalar = [0u8; 32];
    h_scalar.copy_from_slice(&h64[..32]);

    // check  S·B == R + h·A   <=>   R == S·B + h·(-A)
    let mut p = scalarmult(&neg_a, &h_scalar);
    let q = scalarbase(&s);
    point_add(&mut p, &q);
    let t = pack_point(&p);
    eq32(&t, &r)
}

// Is the little-endian 32-byte scalar strictly less than the group order L?
fn scalar_in_range(s: &[u8; 32]) -> bool {
    const L_LE: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];
    for i in (0..32).rev() {
        if s[i] < L_LE[i] {
            return true;
        }
        if s[i] > L_LE[i] {
            return false;
        }
    }
    false // equal to L is out of range
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(n: u8) -> [u8; 32] {
        [n; 32]
    }

    #[test]
    fn sign_then_verify() {
        let sk = seed(7);
        let pk = public_key(&sk);
        let msg = b"the only trust is an explicit contract";
        let sig = sign(&sk, msg);
        assert!(verify(&pk, msg, &sig));
    }

    #[test]
    fn rejects_tampered_message() {
        let sk = seed(9);
        let pk = public_key(&sk);
        let sig = sign(&sk, b"resource: tool/echo");
        assert!(!verify(&pk, b"resource: tool/secret", &sig));
    }

    #[test]
    fn rejects_wrong_key() {
        let pk_other = public_key(&seed(2));
        let sig = sign(&seed(1), b"x");
        assert!(!verify(&pk_other, b"x", &sig));
    }

    #[test]
    fn rejects_tampered_signature() {
        let sk = seed(3);
        let pk = public_key(&sk);
        let mut sig = sign(&sk, b"x");
        sig[0] ^= 1;
        assert!(!verify(&pk, b"x", &sig));
    }

    #[test]
    fn public_key_is_deterministic() {
        assert_eq!(public_key(&seed(5)), public_key(&seed(5)));
        assert_ne!(public_key(&seed(5)), public_key(&seed(6)));
    }

    #[test]
    fn empty_message_round_trips() {
        let sk = seed(42);
        let pk = public_key(&sk);
        let sig = sign(&sk, b"");
        assert!(verify(&pk, b"", &sig));
    }
}
