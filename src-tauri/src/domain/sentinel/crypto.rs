//! Crypto analysis + weak RSA key recovery — native Rust port of the
//! FlutterSentinel crypto-exploitation tooling. Includes a self-contained
//! bignum (u64 limbs) so RSA private-key recovery from a weakly-primed modulus
//! works without new dependencies.

use std::cmp::Ordering;

// ═══════════════════════════ minimal bignum ═══════════════════════════

type Big = Vec<u64>; // little-endian limbs, trimmed of leading zeros, 0 == empty

fn trim(a: &mut Big) {
    while a.len() > 1 && *a.last().unwrap() == 0 {
        a.pop();
    }
    if a.len() == 1 && a[0] == 0 {
        a.clear();
    }
}

fn from_u64(v: u64) -> Big {
    if v == 0 {
        return Big::new();
    }
    vec![v]
}

fn parse_hex(s: &str) -> Result<Big, String> {
    let s = s.trim().trim_start_matches("0x");
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("invalid hex number".to_string());
    }
    let mut limbs = Big::new();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut acc: u64 = 0;
    let mut shift: u32 = 0;
    for &b in bytes.iter().rev() {
        let v = match b {
            b'0'..=b'9' => (b - b'0') as u64,
            b'a'..=b'f' => (b - b'a' + 10) as u64,
            b'A'..=b'F' => (b - b'A' + 10) as u64,
            _ => unreachable!(),
        };
        acc |= v << shift;
        shift += 4;
        if shift == 64 {
            limbs.push(acc);
            acc = 0;
            shift = 0;
        }
    }
    if shift > 0 {
        limbs.push(acc);
    }
    let _ = n;
    trim(&mut limbs);
    Ok(limbs)
}

fn to_hex(a: &Big) -> String {
    if a.is_empty() {
        return "0".to_string();
    }
    let mut out = String::new();
    for &limb in a.iter().rev() {
        out.push_str(&format!("{limb:016x}"));
    }
    let t = out.trim_start_matches('0');
    if t.is_empty() {
        "0".to_string()
    } else {
        t.to_string()
    }
}

fn cmp(a: &Big, b: &Big) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for i in (0..a.len()).rev() {
        if a[i] != b[i] {
            return a[i].cmp(&b[i]);
        }
    }
    Ordering::Equal
}

#[cfg(test)]
fn add(a: &Big, b: &Big) -> Big {
    let mut out = vec![0u64; a.len().max(b.len())];
    let mut carry: u128 = 0;
    for i in 0..out.len() {
        let av = a.get(i).copied().unwrap_or(0) as u128;
        let bv = b.get(i).copied().unwrap_or(0) as u128;
        let cur = av + bv + carry;
        out[i] = (cur & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        carry = cur >> 64;
    }
    if carry > 0 {
        out.push(carry as u64);
    }
    trim(&mut out);
    out
}

fn sub(a: &Big, b: &Big) -> Big {
    // requires a >= b
    let mut out = a.clone();
    let mut borrow = 0u128;
    for i in 0..a.len() {
        let bv = b.get(i).copied().unwrap_or(0) as u128;
        let cur = (out[i] as u128).wrapping_sub(bv).wrapping_sub(borrow);
        let full = (a[i] as u128 + (1u128 << 64))
            .wrapping_sub(bv)
            .wrapping_sub(borrow);
        out[i] = (cur & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        borrow = if (full >> 64) == 0 { 1 } else { 0 };
    }
    trim(&mut out);
    out
}

fn mul(a: &Big, b: &Big) -> Big {
    if a.is_empty() || b.is_empty() {
        return Big::new();
    }
    let mut prod = vec![0u64; a.len() + b.len()];
    for i in 0..a.len() {
        if a[i] == 0 {
            continue;
        }
        let mut carry: u128 = 0;
        let mut j = 0;
        while j < b.len() {
            let cur = prod[i + j] as u128 + (a[i] as u128) * (b[j] as u128) + carry;
            prod[i + j] = (cur & 0xFFFF_FFFF_FFFF_FFFF) as u64;
            carry = cur >> 64;
            j += 1;
        }
        let mut k = i + b.len();
        while carry > 0 {
            let cur = prod[k] as u128 + carry;
            prod[k] = (cur & 0xFFFF_FFFF_FFFF_FFFF) as u64;
            carry = cur >> 64;
            k += 1;
        }
    }
    trim(&mut prod);
    prod
}

fn bit_len(a: &Big) -> usize {
    if a.is_empty() {
        return 0;
    }
    let top = *a.last().unwrap();
    (a.len() - 1) * 64 + (64 - top.leading_zeros() as usize)
}

fn bit_at(a: &Big, i: usize) -> bool {
    let limb = i / 64;
    let off = (i % 64) as u32;
    a.get(limb).copied().unwrap_or(0) & (1u64 << off) != 0
}

fn set_bit(a: &mut Big, i: usize) {
    let limb = i / 64;
    let off = (i % 64) as u32;
    if a.len() <= limb {
        a.resize(limb + 1, 0);
    }
    a[limb] |= 1u64 << off;
}

fn shl(a: &Big, bits: usize) -> Big {
    if a.is_empty() || bits == 0 {
        return a.clone();
    }
    let limb_shift = bits / 64;
    let bit_shift = (bits % 64) as u32;
    let mut out = vec![0u64; a.len() + limb_shift + 1];
    if bit_shift == 0 {
        for i in (0..a.len()).rev() {
            out[i + limb_shift] = a[i];
        }
    } else {
        for i in (0..a.len()).rev() {
            let v = a[i];
            out[i + limb_shift] |= v << bit_shift;
            if i + limb_shift + 1 < out.len() {
                out[i + limb_shift + 1] |= v >> (64 - bit_shift);
            }
        }
    }
    trim(&mut out);
    out
}

/// a mod b — b must be non-empty.
fn div_rem(a: &Big, b: &Big) -> (Big, Big) {
    if b.is_empty() {
        panic!("division by zero");
    }
    if a.is_empty() || cmp(a, b) == Ordering::Less {
        return (Big::new(), a.clone());
    }
    let n = bit_len(a);
    let mut q = vec![0u64; a.len()];
    let mut rem: Big = Big::new();
    for i in (0..n).rev() {
        rem = shl(&rem, 1);
        if bit_at(a, i) {
            if rem.is_empty() {
                rem = from_u64(1);
            } else {
                rem[0] |= 1;
            }
        }
        if cmp(&rem, b) != Ordering::Less {
            rem = sub(&rem, b);
            set_bit(&mut q, i);
        }
    }
    trim(&mut q);
    trim(&mut rem);
    (q, rem)
}

fn big_mod(a: &Big, b: &Big) -> Big {
    div_rem(a, b).1
}

fn mod_small(a: &Big, m: u64) -> u64 {
    if a.is_empty() {
        return 0;
    }
    let mut rem: u128 = 0;
    for &limb in a.iter().rev() {
        rem = ((rem << 64) | limb as u128) % m as u128;
    }
    rem as u64
}

fn mod_mul(a: &Big, b: &Big, m: &Big) -> Big {
    big_mod(&mul(a, b), m)
}

fn mod_sub(a: &Big, b: &Big, m: &Big) -> Big {
    // (a - b) mod m, a,b already < m
    match cmp(a, b) {
        Ordering::Less => sub(m, &sub(b, a)),
        _ => sub(a, b),
    }
}

/// Modular inverse of `a` mod `m` — EEA working entirely mod m (no negatives).
fn mod_inv(a: &Big, m: &Big) -> Option<Big> {
    if cmp(a, m) != Ordering::Less {
        let red = big_mod(a, m);
        return mod_inv(&red, m);
    }
    if a.is_empty() {
        return None;
    }
    let mut r0 = m.clone();
    let mut r1 = a.clone();
    let mut s0 = m.clone(); // ≡ 0 (mod m)
    let mut s1 = from_u64(1);
    loop {
        if r1.is_empty() {
            break;
        }
        let (q, rnew) = div_rem(&r0, &r1);
        let t = mod_sub(&s0, &mod_mul(&q, &s1, m), m);
        r0 = r1;
        r1 = rnew;
        s0 = s1;
        s1 = t;
    }
    if cmp(&r0, &from_u64(1)) != Ordering::Equal {
        return None;
    }
    Some(s0)
}

fn small_primes_up_to(limit: usize) -> Vec<u64> {
    let mut sieve = vec![true; limit.max(1) + 1];
    sieve[0] = false;
    if limit >= 1 {
        sieve[1] = false;
    }
    let mut i = 2;
    while i * i <= limit {
        if sieve[i] {
            let mut j = i * i;
            while j <= limit {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=limit)
        .filter(|&t| sieve[t])
        .map(|t| t as u64)
        .collect()
}

// ═══════════════════════════ RSA weak-key recovery ═══════════════════════════

/// Given an RSA modulus `n` and public exponent `e`, try to recover the private
/// key by trial-dividing the modulus by small primes. Returns p, q, and the
/// private exponent d when a factor is found. This is the "RSA vulnerable to
/// factoring when generated with a small prime" crypto exploit.
pub fn rsa_recover_private_key(n_hex: &str, e_hex: &str) -> Result<Value, String> {
    let n = parse_hex(n_hex)?;
    let e = parse_hex(e_hex).unwrap_or(from_u64(65537));
    if n.is_empty() {
        return Err("empty modulus".to_string());
    }
    let bitlen = bit_len(&n);
    let primes = small_primes_up_to(1_000_000);
    for &p in &primes {
        if n.len() == 1 && n[0] == p {
            continue; // n == p (trivial, still bad but no q)
        }
        if mod_small(&n, p) == 0 {
            let pb = from_u64(p);
            let (q, r) = div_rem(&n, &pb);
            if !r.is_empty() && cmp(&q, &from_u64(1)) != Ordering::Greater {
                continue;
            }
            // phi = (p-1)*(q-1)
            let p1 = from_u64(p - 1);
            let q1 = {
                let mut t = q.clone();
                // q - 1
                let one = from_u64(1);
                let diff = sub(&q, &one);
                trim(&mut t);
                diff
            };
            let phi = mul(&p1, &q1);
            let d = mod_inv(&e, &phi);
            let mut result = json!({
                "status": "recovered",
                "factor": p,
                "p": p,
                "q_hex": to_hex(&q),
                "modulus_bits": bitlen,
                "phi": to_hex(&phi),
            });
            if let Some(d) = d {
                result["d_hex"] = Value::String(to_hex(&d));
                result["recovered_private_key"] = Value::Bool(true);
            } else {
                result["d_hex"] = Value::Null;
                result["recovered_private_key"] = Value::Bool(false);
            }
            return Ok(result);
        }
    }
    Ok(json!({
        "status": "not_found",
        "note": "no small prime factor ≤ 1,000,000 (modulus likely uses large primes)",
        "modulus_bits": bitlen,
    }))
}

// ═══════════════════════════ weak-crypto text scan ═══════════════════════════

/// Scan a blob (config, source, strings dump) for weak-crypto usage. Mirrors the
/// FlutterSentinel `crypto_analyzer` heuristics.
pub fn analyze_crypto_text(text: &str) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::new();
    let checks: &[(&str, &str)] = &[
        ("MD5", r"(?i)\b(md5)\b"),
        ("SHA-1", r"(?i)\b(sha1|sha-1)\b"),
        ("RC4", r"(?i)\b(rc4)\b"),
        (
            "DES (single)",
            r#"(?i)\b\?des\b|\bDesCryptoServiceProvider\b|\bCipher\.getInstance\(\s*"DES""#,
        ),
        ("3DES", r"(?i)\b(3des|desede|tripledes)\b"),
        ("ECB mode", r"(?i)\b(ECB)\b"),
        (
            "Weak nonce (WEP-style)",
            r"(?i)\b(wep|nonce=0{8}|nonce.{0,8}00000000)\b",
        ),
        (
            "Static IV",
            r#"(?i)\b(static.?iv|iv\s*=\s*["']0{10,}["'])\b"#,
        ),
        (
            "Hardcoded salt",
            r#"(?i)\b(salt\s*=\s*["'][a-z0-9]{1,6}["'])\b"#,
        ),
        (
            "Mersenne Twister/random (predictable)",
            r"(?i)\b(deterministic.?random|predictable.?random|mt19937)\b",
        ),
        (
            "PEM private key",
            r"-----BEGIN (RSA |EC |DSA )?PRIVATE KEY-----",
        ),
        (
            "Hardcoded seed",
            r#"(?i)\b(seed\s*=\s*["'][a-z0-9_]{1,8}["'])\b"#,
        ),
    ];
    for (label, pat) in checks {
        if let Ok(re) = regex::Regex::new(pat) {
            let mut count = 0usize;
            for m in re.find_iter(text) {
                count += 1;
                if count <= 5 {
                    let start = m.start().saturating_sub(48);
                    let end = (m.end() + 48).min(text.len());
                    let ctx = text[start..end]
                        .replace('\n', " ")
                        .chars()
                        .take(120)
                        .collect::<String>();
                    let severity = if label.contains("private key")
                        || label.contains("ECB")
                        || label.contains("MD5")
                    {
                        "HIGH"
                    } else {
                        "MEDIUM"
                    };
                    out.push(json!({
                        "crypto": label,
                        "severity": severity,
                        "context": ctx,
                    }));
                }
            }
        }
    }
    out
}

use serde_json::{json, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        assert_eq!(to_hex(&parse_hex("abc123").unwrap()), "abc123");
        assert_eq!(
            to_hex(&parse_hex("ffffffffffffffffffffff").unwrap()),
            "ffffffffffffffffffffff"
        );
        assert_eq!(to_hex(&parse_hex("0").unwrap()), "0");
    }

    #[test]
    fn mul_sub_div() {
        let a = parse_hex("f".repeat(64).as_str()).unwrap();
        let b = parse_hex("3").unwrap();
        let prod = mul(&a, &b);
        assert_eq!(to_hex(&prod), to_hex(&add(&a, &add(&a, &a))));
        let (q, r) = div_rem(&prod, &a);
        assert_eq!(to_hex(&q), "3");
        assert!(r.is_empty());
    }

    #[test]
    fn mod_inv_simple() {
        let m = parse_hex("b").unwrap();
        let a = parse_hex("3").unwrap();
        let inv = mod_inv(&a, &m).unwrap();
        assert_eq!(to_hex(&inv), "4");
    }

    #[test]
    fn rsa_recovery_small() {
        // n = 91 = 7 * 13, e = 5 → d = 29
        let out = rsa_recover_private_key("5b", "5").unwrap();
        assert_eq!(out["status"], "recovered");
        assert_eq!(out["factor"], 7);
        assert_eq!(out["q_hex"], "d"); // 13
        assert_eq!(out["d_hex"], "1d"); // 29
    }
}
