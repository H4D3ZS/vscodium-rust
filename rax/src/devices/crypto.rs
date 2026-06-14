//! Self-contained AES and SHA-1 primitives for the S5L8900 hardware crypto
//! engine. These are deliberately simple, table-driven reference
//! implementations (correctness over speed) used by `S5lAes` to decrypt
//! 8900/IMG2-wrapped boot images (LLB, iBoot, the kernelcache, ...) and to
//! compute their SHA-1 digests, mirroring the on-die AES block and the SHA
//! engine of the S5L8900.

// ---------------------------------------------------------------------------
// AES (FIPS-197) — encryption is unused here; the engine only ever decrypts.
// ---------------------------------------------------------------------------

#[rustfmt::skip]
const SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

fn inv_sbox() -> [u8; 256] {
    let mut inv = [0u8; 256];
    for (i, &s) in SBOX.iter().enumerate() {
        inv[s as usize] = i as u8;
    }
    inv
}

/// Multiply by `x` in GF(2^8) (the AES field, reduction polynomial 0x11b).
fn xtime(a: u8) -> u8 {
    let hi = a & 0x80;
    let mut r = a << 1;
    if hi != 0 {
        r ^= 0x1b;
    }
    r
}

/// Multiply two GF(2^8) elements.
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

/// Expanded AES key schedule (round keys), supporting 128/192/256-bit keys.
pub struct AesKey {
    /// Round keys, each 16 bytes; `rounds + 1` of them.
    round_keys: Vec<[u8; 16]>,
    rounds: usize,
}

impl AesKey {
    /// Build the key schedule for a 16-, 24-, or 32-byte key.
    pub fn new(key: &[u8]) -> Option<AesKey> {
        let nk = match key.len() {
            16 => 4,
            24 => 6,
            32 => 8,
            _ => return None,
        };
        let rounds = nk + 6;
        let total_words = 4 * (rounds + 1);
        let mut w = vec![0u32; total_words];
        for i in 0..nk {
            w[i] = u32::from_be_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
        }
        let mut rcon: u8 = 1;
        for i in nk..total_words {
            let mut tmp = w[i - 1];
            if i % nk == 0 {
                tmp = sub_word(tmp.rotate_left(8)) ^ ((rcon as u32) << 24);
                rcon = xtime(rcon);
            } else if nk > 6 && i % nk == 4 {
                tmp = sub_word(tmp);
            }
            w[i] = w[i - nk] ^ tmp;
        }
        let mut round_keys = Vec::with_capacity(rounds + 1);
        for r in 0..=rounds {
            let mut rk = [0u8; 16];
            for c in 0..4 {
                rk[4 * c..4 * c + 4].copy_from_slice(&w[4 * r + c].to_be_bytes());
            }
            round_keys.push(rk);
        }
        Some(AesKey { round_keys, rounds })
    }

    /// Decrypt a single 16-byte block in place (equivalent inverse cipher,
    /// FIPS-197 §5.3 straightforward form).
    pub fn decrypt_block(&self, block: &mut [u8; 16]) {
        let inv = inv_sbox();
        add_round_key(block, &self.round_keys[self.rounds]);
        for round in (1..self.rounds).rev() {
            inv_shift_rows(block);
            for b in block.iter_mut() {
                *b = inv[*b as usize];
            }
            add_round_key(block, &self.round_keys[round]);
            inv_mix_columns(block);
        }
        inv_shift_rows(block);
        for b in block.iter_mut() {
            *b = inv[*b as usize];
        }
        add_round_key(block, &self.round_keys[0]);
    }
}

fn sub_word(w: u32) -> u32 {
    let b = w.to_be_bytes();
    u32::from_be_bytes([
        SBOX[b[0] as usize],
        SBOX[b[1] as usize],
        SBOX[b[2] as usize],
        SBOX[b[3] as usize],
    ])
}

fn add_round_key(state: &mut [u8; 16], rk: &[u8; 16]) {
    for i in 0..16 {
        state[i] ^= rk[i];
    }
}

/// InvShiftRows on a column-major state (state[col*4 + row]).
fn inv_shift_rows(s: &mut [u8; 16]) {
    // Row 1: shift right by 1
    let t = [s[1], s[5], s[9], s[13]];
    s[1] = t[3];
    s[5] = t[0];
    s[9] = t[1];
    s[13] = t[2];
    // Row 2: shift right by 2
    let t = [s[2], s[6], s[10], s[14]];
    s[2] = t[2];
    s[6] = t[3];
    s[10] = t[0];
    s[14] = t[1];
    // Row 3: shift right by 3
    let t = [s[3], s[7], s[11], s[15]];
    s[3] = t[1];
    s[7] = t[2];
    s[11] = t[3];
    s[15] = t[0];
}

fn inv_mix_columns(s: &mut [u8; 16]) {
    for c in 0..4 {
        let i = c * 4;
        let a0 = s[i];
        let a1 = s[i + 1];
        let a2 = s[i + 2];
        let a3 = s[i + 3];
        s[i] = gmul(a0, 14) ^ gmul(a1, 11) ^ gmul(a2, 13) ^ gmul(a3, 9);
        s[i + 1] = gmul(a0, 9) ^ gmul(a1, 14) ^ gmul(a2, 11) ^ gmul(a3, 13);
        s[i + 2] = gmul(a0, 13) ^ gmul(a1, 9) ^ gmul(a2, 14) ^ gmul(a3, 11);
        s[i + 3] = gmul(a0, 11) ^ gmul(a1, 13) ^ gmul(a2, 9) ^ gmul(a3, 14);
    }
}

/// AES-CBC decrypt `data` (length must be a multiple of 16) in place, using
/// the supplied key schedule and 16-byte IV. Matches the S5L8900 AES engine's
/// `AES_cbc_encrypt(..., AES_DECRYPT)` path.
pub fn aes_cbc_decrypt(key: &AesKey, iv: &[u8; 16], data: &mut [u8]) {
    let mut prev = *iv;
    let mut off = 0;
    while off + 16 <= data.len() {
        let mut block = [0u8; 16];
        block.copy_from_slice(&data[off..off + 16]);
        let cipher = block;
        key.decrypt_block(&mut block);
        for i in 0..16 {
            block[i] ^= prev[i];
        }
        data[off..off + 16].copy_from_slice(&block);
        prev = cipher;
        off += 16;
    }
}

// ---------------------------------------------------------------------------
// SHA-1 (FIPS-180) — used for image digest verification.
// ---------------------------------------------------------------------------

/// Compute the SHA-1 digest of `data`.
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bit_len = (data.len() as u64).wrapping_mul(8);

    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for (i, &wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let tmp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = tmp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut out = [0u8; 20];
    for (i, hv) in h.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&hv.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    #[test]
    fn aes128_decrypt_block_fips197() {
        // FIPS-197 Appendix C.1 vector.
        let key = hex("000102030405060708090a0b0c0d0e0f");
        let ct = hex("69c4e0d86a7b0430d8cdb78070b4c55a");
        let pt = hex("00112233445566778899aabbccddeeff");
        let k = AesKey::new(&key).unwrap();
        let mut block = [0u8; 16];
        block.copy_from_slice(&ct);
        k.decrypt_block(&mut block);
        assert_eq!(&block[..], &pt[..]);
    }

    #[test]
    fn aes256_decrypt_block_fips197() {
        // FIPS-197 Appendix C.3 vector.
        let key = hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
        let ct = hex("8ea2b7ca516745bfeafc49904b496089");
        let pt = hex("00112233445566778899aabbccddeeff");
        let k = AesKey::new(&key).unwrap();
        let mut block = [0u8; 16];
        block.copy_from_slice(&ct);
        k.decrypt_block(&mut block);
        assert_eq!(&block[..], &pt[..]);
    }

    #[test]
    fn aes128_cbc_decrypt_nist() {
        // NIST SP800-38A F.2.2 CBC-AES128.Decrypt vector (first two blocks).
        let key = hex("2b7e151628aed2a6abf7158809cf4f3c");
        let iv = hex("000102030405060708090a0b0c0d0e0f");
        let ct = hex("7649abac8119b246cee98e9b12e9197d5086cb9b507219ee95db113a917678b2");
        let pt = hex("6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e51");
        let k = AesKey::new(&key).unwrap();
        let mut iv16 = [0u8; 16];
        iv16.copy_from_slice(&iv);
        let mut data = ct.clone();
        aes_cbc_decrypt(&k, &iv16, &mut data);
        assert_eq!(data, pt);
    }

    #[test]
    fn sha1_known_vectors() {
        assert_eq!(
            sha1(b"abc"),
            hex("a9993e364706816aba3e25717850c26c9cd0d89d")[..]
        );
        assert_eq!(sha1(b"")[..], hex("da39a3ee5e6b4b0d3255bfef95601890afd80709")[..]);
        assert_eq!(
            sha1(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")[..],
            hex("84983e441c3bd26ebaae4aa1f95129e5e54670f1")[..]
        );
    }
}
