//! SHA-NI helper functions for x86_64 CPU emulator.
//! Implements SHA-1 and SHA-256 cryptographic operations.

// =============================================================================
// SHA-1 Helper Functions
// =============================================================================

/// Rotate left for 32-bit value
#[inline]
fn rol32(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

/// SHA-1 function f0 (rounds 0-19): Ch(B, C, D) = (B AND C) XOR (NOT B AND D)
#[inline]
fn sha1_f0(b: u32, c: u32, d: u32) -> u32 {
    (b & c) ^ ((!b) & d)
}

/// SHA-1 function f1 (rounds 20-39): Parity(B, C, D) = B XOR C XOR D
#[inline]
fn sha1_f1(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

/// SHA-1 function f2 (rounds 40-59): Maj(B, C, D) = (B AND C) XOR (B AND D) XOR (C AND D)
#[inline]
fn sha1_f2(b: u32, c: u32, d: u32) -> u32 {
    (b & c) ^ (b & d) ^ (c & d)
}

/// SHA-1 function f3 (rounds 60-79): Parity(B, C, D) = B XOR C XOR D
#[inline]
fn sha1_f3(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

/// SHA-1 round constants
const SHA1_K0: u32 = 0x5A827999; // rounds 0-19
const SHA1_K1: u32 = 0x6ED9EBA1; // rounds 20-39
const SHA1_K2: u32 = 0x8F1BBCDC; // rounds 40-59
const SHA1_K3: u32 = 0xCA62C1D6; // rounds 60-79

// =============================================================================
// SHA-256 Helper Functions
// =============================================================================

/// SHA-256 σ0 function: ROTR7(x) XOR ROTR18(x) XOR SHR3(x)
#[inline]
fn sha256_sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

/// SHA-256 σ1 function: ROTR17(x) XOR ROTR19(x) XOR SHR10(x)
#[inline]
fn sha256_sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// SHA-256 Σ0 function: ROTR2(x) XOR ROTR13(x) XOR ROTR22(x)
#[inline]
fn sha256_big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// SHA-256 Σ1 function: ROTR6(x) XOR ROTR11(x) XOR ROTR25(x)
#[inline]
fn sha256_big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/// SHA-256 Ch function: (E AND F) XOR (NOT E AND G)
#[inline]
fn sha256_ch(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ ((!e) & g)
}

/// SHA-256 Maj function: (A AND B) XOR (A AND C) XOR (B AND C)
#[inline]
fn sha256_maj(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

// =============================================================================
// SHA-1 Instruction Implementations
// =============================================================================

/// SHA1MSG1: Perform an intermediate calculation for the next four SHA1 message dwords
///
/// Operation:
/// W0 := SRC1[127:96]; W1 := SRC1[95:64]; W2 := SRC1[63:32]; W3 := SRC1[31:0]
/// W4 := SRC2[127:96]; W5 := SRC2[95:64]
/// DEST[127:96] := W2 XOR W0
/// DEST[95:64] := W3 XOR W1
/// DEST[63:32] := W4 XOR W2
/// DEST[31:0] := W5 XOR W3
pub fn sha1msg1(src1_lo: u64, src1_hi: u64, src2_lo: u64, src2_hi: u64) -> (u64, u64) {
    // Extract W0-W5 from the XMM registers (big-endian dword order in the spec)
    let w0 = (src1_hi >> 32) as u32;
    let w1 = src1_hi as u32;
    let w2 = (src1_lo >> 32) as u32;
    let w3 = src1_lo as u32;
    let w4 = (src2_hi >> 32) as u32;
    let w5 = src2_hi as u32;

    // Compute results
    let r0 = w2 ^ w0; // DEST[127:96]
    let r1 = w3 ^ w1; // DEST[95:64]
    let r2 = w4 ^ w2; // DEST[63:32]
    let r3 = w5 ^ w3; // DEST[31:0]

    // Pack results back to XMM format
    let dest_lo = ((r2 as u64) << 32) | (r3 as u64);
    let dest_hi = ((r0 as u64) << 32) | (r1 as u64);

    (dest_lo, dest_hi)
}

/// SHA1MSG2: Perform a final calculation for the next four SHA1 message dwords
///
/// Operation:
/// W13 := SRC2[95:64]; W14 := SRC2[63:32]; W15 := SRC2[31:0]
/// W16 := (SRC1[127:96] XOR W13) ROL 1
/// W17 := (SRC1[95:64] XOR W14) ROL 1
/// W18 := (SRC1[63:32] XOR W15) ROL 1
/// W19 := (SRC1[31:0] XOR W16) ROL 1
/// DEST[127:96] := W16; DEST[95:64] := W17; DEST[63:32] := W18; DEST[31:0] := W19
pub fn sha1msg2(src1_lo: u64, src1_hi: u64, src2_lo: u64, _src2_hi: u64) -> (u64, u64) {
    // Extract words from SRC2 (W13, W14, W15)
    let w13 = src2_lo as u32; // SRC2[95:64] - but in little-endian storage
    let w14 = (src2_lo >> 32) as u32; // SRC2[63:32]
    let w15 = _src2_hi as u32; // SRC2[31:0] - Wait, let me reconsider

    // Actually, the spec notation uses [127:96] as the high dword
    // In little-endian XMM storage: xmm[0] = bits 63:0, xmm[1] = bits 127:64
    // So [127:96] is (xmm[1] >> 32), [95:64] is xmm[1] & 0xFFFFFFFF
    // [63:32] is (xmm[0] >> 32), [31:0] is xmm[0] & 0xFFFFFFFF

    let w13 = _src2_hi as u32; // SRC2[95:64]
    let w14 = (src2_lo >> 32) as u32; // SRC2[63:32]
    let w15 = src2_lo as u32; // SRC2[31:0]

    // Extract words from SRC1
    let s0 = (src1_hi >> 32) as u32; // SRC1[127:96]
    let s1 = src1_hi as u32; // SRC1[95:64]
    let s2 = (src1_lo >> 32) as u32; // SRC1[63:32]
    let s3 = src1_lo as u32; // SRC1[31:0]

    // Compute W16-W19
    let w16 = rol32(s0 ^ w13, 1);
    let w17 = rol32(s1 ^ w14, 1);
    let w18 = rol32(s2 ^ w15, 1);
    let w19 = rol32(s3 ^ w16, 1);

    // Pack results
    let dest_lo = ((w18 as u64) << 32) | (w19 as u64);
    let dest_hi = ((w16 as u64) << 32) | (w17 as u64);

    (dest_lo, dest_hi)
}

/// SHA1NEXTE: Calculate SHA1 state variable E after four rounds
///
/// Operation:
/// TMP := (SRC1[127:96] ROL 30)
/// DEST[127:96] := SRC2[127:96] + TMP
/// DEST[95:64] := SRC2[95:64]
/// DEST[63:32] := SRC2[63:32]
/// DEST[31:0] := SRC2[31:0]
pub fn sha1nexte(src1_lo: u64, src1_hi: u64, src2_lo: u64, src2_hi: u64) -> (u64, u64) {
    // Extract SRC1[127:96]
    let a = (src1_hi >> 32) as u32;

    // Rotate left by 30
    let tmp = rol32(a, 30);

    // Extract SRC2[127:96]
    let src2_high_dword = (src2_hi >> 32) as u32;

    // Add with wrapping
    let result_high_dword = src2_high_dword.wrapping_add(tmp);

    // Reconstruct dest_hi with the new high dword
    let dest_hi = ((result_high_dword as u64) << 32) | (src2_hi & 0xFFFFFFFF);
    let dest_lo = src2_lo;

    (dest_lo, dest_hi)
}

/// SHA1RNDS4: Perform four rounds of SHA1 operation
///
/// The function f() and constant K depend on imm8[1:0]:
/// 0: f0 (Ch), K0
/// 1: f1 (Parity), K1
/// 2: f2 (Maj), K2
/// 3: f3 (Parity), K3
pub fn sha1rnds4(src1_lo: u64, src1_hi: u64, src2_lo: u64, src2_hi: u64, imm8: u8) -> (u64, u64) {
    // Select function and constant based on imm8[1:0]
    let func_select = imm8 & 0x03;
    let k = match func_select {
        0 => SHA1_K0,
        1 => SHA1_K1,
        2 => SHA1_K2,
        3 => SHA1_K3,
        _ => unreachable!(),
    };

    // Extract initial state A, B, C, D from SRC1
    let mut a = (src1_hi >> 32) as u32; // SRC1[127:96]
    let mut b = src1_hi as u32; // SRC1[95:64]
    let mut c = (src1_lo >> 32) as u32; // SRC1[63:32]
    let mut d = src1_lo as u32; // SRC1[31:0]

    // Extract W0E, W1, W2, W3 from SRC2
    let w0e = (src2_hi >> 32) as u32; // SRC2[127:96] - includes E for first round
    let w1 = src2_hi as u32; // SRC2[95:64]
    let w2 = (src2_lo >> 32) as u32; // SRC2[63:32]
    let w3 = src2_lo as u32; // SRC2[31:0]

    let w = [w0e, w1, w2, w3];

    // Perform 4 rounds
    for i in 0..4 {
        let f_result = match func_select {
            0 => sha1_f0(b, c, d),
            1 => sha1_f1(b, c, d),
            2 => sha1_f2(b, c, d),
            3 => sha1_f3(b, c, d),
            _ => unreachable!(),
        };

        // For round 0, w[0] already includes E
        // For rounds 1-3, we need to use d as the previous E
        let e = if i == 0 { 0 } else { d };
        let wi_plus_e = if i == 0 { w[i] } else { w[i].wrapping_add(e) };

        let new_a = f_result
            .wrapping_add(rol32(a, 5))
            .wrapping_add(wi_plus_e)
            .wrapping_add(k);
        let new_b = a;
        let new_c = rol32(b, 30);
        let new_d = c;

        a = new_a;
        b = new_b;
        c = new_c;
        d = new_d;
    }

    // Pack results
    let dest_lo = ((c as u64) << 32) | (d as u64);
    let dest_hi = ((a as u64) << 32) | (b as u64);

    (dest_lo, dest_hi)
}

// =============================================================================
// SHA-256 Instruction Implementations
// =============================================================================

/// SHA256MSG1: Perform an intermediate calculation for the next four SHA256 message dwords
///
/// Operation:
/// W4 := SRC2[31:0]
/// W3 := SRC1[127:96]; W2 := SRC1[95:64]; W1 := SRC1[63:32]; W0 := SRC1[31:0]
/// DEST[127:96] := W3 + σ0(W4)
/// DEST[95:64] := W2 + σ0(W3)
/// DEST[63:32] := W1 + σ0(W2)
/// DEST[31:0] := W0 + σ0(W1)
pub fn sha256msg1(src1_lo: u64, src1_hi: u64, src2_lo: u64, _src2_hi: u64) -> (u64, u64) {
    // Extract W0-W3 from SRC1 and W4 from SRC2
    let w0 = src1_lo as u32; // SRC1[31:0]
    let w1 = (src1_lo >> 32) as u32; // SRC1[63:32]
    let w2 = src1_hi as u32; // SRC1[95:64]
    let w3 = (src1_hi >> 32) as u32; // SRC1[127:96]
    let w4 = src2_lo as u32; // SRC2[31:0]

    // Compute results
    let r0 = w0.wrapping_add(sha256_sigma0(w1)); // DEST[31:0]
    let r1 = w1.wrapping_add(sha256_sigma0(w2)); // DEST[63:32]
    let r2 = w2.wrapping_add(sha256_sigma0(w3)); // DEST[95:64]
    let r3 = w3.wrapping_add(sha256_sigma0(w4)); // DEST[127:96]

    // Pack results
    let dest_lo = (r0 as u64) | ((r1 as u64) << 32);
    let dest_hi = (r2 as u64) | ((r3 as u64) << 32);

    (dest_lo, dest_hi)
}

/// SHA256MSG2: Perform a final calculation for the next four SHA256 message dwords
///
/// Operation:
/// W14 := SRC2[95:64]; W15 := SRC2[127:96]
/// W16 := SRC1[31:0] + σ1(W14)
/// W17 := SRC1[63:32] + σ1(W15)
/// W18 := SRC1[95:64] + σ1(W16)
/// W19 := SRC1[127:96] + σ1(W17)
/// DEST[127:96] := W19; DEST[95:64] := W18; DEST[63:32] := W17; DEST[31:0] := W16
pub fn sha256msg2(src1_lo: u64, src1_hi: u64, _src2_lo: u64, src2_hi: u64) -> (u64, u64) {
    // Extract W14, W15 from SRC2
    let w14 = src2_hi as u32; // SRC2[95:64]
    let w15 = (src2_hi >> 32) as u32; // SRC2[127:96]

    // Extract intermediate values from SRC1
    let s0 = src1_lo as u32; // SRC1[31:0]
    let s1 = (src1_lo >> 32) as u32; // SRC1[63:32]
    let s2 = src1_hi as u32; // SRC1[95:64]
    let s3 = (src1_hi >> 32) as u32; // SRC1[127:96]

    // Compute W16-W19
    let w16 = s0.wrapping_add(sha256_sigma1(w14));
    let w17 = s1.wrapping_add(sha256_sigma1(w15));
    let w18 = s2.wrapping_add(sha256_sigma1(w16));
    let w19 = s3.wrapping_add(sha256_sigma1(w17));

    // Pack results
    let dest_lo = (w16 as u64) | ((w17 as u64) << 32);
    let dest_hi = (w18 as u64) | ((w19 as u64) << 32);

    (dest_lo, dest_hi)
}

/// SHA256RNDS2: Perform two rounds of SHA256 operation
///
/// Uses XMM0 implicitly for WK0 and WK1 (round constants + message schedule)
///
/// Operation (2 rounds):
/// A_0 := SRC2[127:96]; B_0 := SRC2[95:64]; E_0 := SRC2[63:32]; F_0 := SRC2[31:0]
/// C_0 := SRC1[127:96]; D_0 := SRC1[95:64]; G_0 := SRC1[63:32]; H_0 := SRC1[31:0]
/// WK0 := XMM0[31:0]; WK1 := XMM0[63:32]
///
/// For i = 0 to 1:
///   A_(i+1) := Ch(E_i,F_i,G_i) + Σ1(E_i) + WKi + H_i + Maj(A_i,B_i,C_i) + Σ0(A_i)
///   B_(i+1) := A_i
///   C_(i+1) := B_i
///   D_(i+1) := C_i
///   E_(i+1) := Ch(E_i,F_i,G_i) + Σ1(E_i) + WKi + H_i + D_i
///   F_(i+1) := E_i
///   G_(i+1) := F_i
///   H_(i+1) := G_i
///
/// DEST[127:96] := A_2; DEST[95:64] := B_2; DEST[63:32] := E_2; DEST[31:0] := F_2
pub fn sha256rnds2(
    src1_lo: u64,
    src1_hi: u64,
    src2_lo: u64,
    src2_hi: u64,
    xmm0_lo: u64,
) -> (u64, u64) {
    // Extract initial state from SRC2 (A, B, E, F)
    let mut a = (src2_hi >> 32) as u32; // SRC2[127:96]
    let mut b = src2_hi as u32; // SRC2[95:64]
    let mut e = (src2_lo >> 32) as u32; // SRC2[63:32]
    let mut f = src2_lo as u32; // SRC2[31:0]

    // Extract initial state from SRC1 (C, D, G, H)
    let mut c = (src1_hi >> 32) as u32; // SRC1[127:96]
    let mut d = src1_hi as u32; // SRC1[95:64]
    let mut g = (src1_lo >> 32) as u32; // SRC1[63:32]
    let mut h = src1_lo as u32; // SRC1[31:0]

    // Extract WK0, WK1 from XMM0
    let wk0 = xmm0_lo as u32; // XMM0[31:0]
    let wk1 = (xmm0_lo >> 32) as u32; // XMM0[63:32]

    let wk = [wk0, wk1];

    // Perform 2 rounds
    for i in 0..2 {
        let ch = sha256_ch(e, f, g);
        let maj = sha256_maj(a, b, c);
        let sigma0 = sha256_big_sigma0(a);
        let sigma1 = sha256_big_sigma1(e);

        let t1 = h.wrapping_add(sigma1).wrapping_add(ch).wrapping_add(wk[i]);
        let t2 = sigma0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    // Pack results: DEST = (A, B, E, F)
    let dest_lo = ((e as u64) << 32) | (f as u64);
    let dest_hi = ((a as u64) << 32) | (b as u64);

    (dest_lo, dest_hi)
}
