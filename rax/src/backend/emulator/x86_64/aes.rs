//! AES-NI helper functions for x86_64 CPU emulator.
//! Implements the core AES operations: SubBytes, ShiftRows, MixColumns and their inverses.

/// AES S-box lookup table
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// AES Inverse S-box lookup table
const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// Galois Field multiplication by 2 in GF(2^8)
#[inline]
fn gf_mul2(x: u8) -> u8 {
    if x & 0x80 != 0 {
        (x << 1) ^ 0x1b
    } else {
        x << 1
    }
}

/// Galois Field multiplication by 3 in GF(2^8)
#[inline]
fn gf_mul3(x: u8) -> u8 {
    gf_mul2(x) ^ x
}

/// Galois Field multiplication by 9 in GF(2^8)
#[inline]
fn gf_mul9(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ x
}

/// Galois Field multiplication by 11 in GF(2^8)
#[inline]
fn gf_mul11(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x)) ^ x) ^ x
}

/// Galois Field multiplication by 13 in GF(2^8)
#[inline]
fn gf_mul13(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x) ^ x)) ^ x
}

/// Galois Field multiplication by 14 in GF(2^8)
#[inline]
fn gf_mul14(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x) ^ x) ^ x)
}

/// Convert 128-bit state from two u64s to 16 bytes (column-major order)
#[inline]
fn state_to_bytes(lo: u64, hi: u64) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&lo.to_le_bytes());
    bytes[8..16].copy_from_slice(&hi.to_le_bytes());
    bytes
}

/// Convert 16 bytes back to two u64s
#[inline]
fn bytes_to_state(bytes: &[u8; 16]) -> (u64, u64) {
    let lo = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    let hi = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    (lo, hi)
}

/// Apply SubBytes transformation (S-box substitution)
pub fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

/// Apply InvSubBytes transformation (inverse S-box substitution)
pub fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = INV_SBOX[*byte as usize];
    }
}

/// Apply ShiftRows transformation
/// Row 0: no shift
/// Row 1: shift left by 1
/// Row 2: shift left by 2
/// Row 3: shift left by 3
pub fn shift_rows(state: &mut [u8; 16]) {
    // State is column-major: state[row + 4*col]
    // Row 1: shift left by 1
    let tmp = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = tmp;

    // Row 2: shift left by 2
    let tmp0 = state[2];
    let tmp1 = state[6];
    state[2] = state[10];
    state[6] = state[14];
    state[10] = tmp0;
    state[14] = tmp1;

    // Row 3: shift left by 3 (or right by 1)
    let tmp = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = state[3];
    state[3] = tmp;
}

/// Apply InvShiftRows transformation
pub fn inv_shift_rows(state: &mut [u8; 16]) {
    // Row 1: shift right by 1
    let tmp = state[13];
    state[13] = state[9];
    state[9] = state[5];
    state[5] = state[1];
    state[1] = tmp;

    // Row 2: shift right by 2
    let tmp0 = state[2];
    let tmp1 = state[6];
    state[2] = state[10];
    state[6] = state[14];
    state[10] = tmp0;
    state[14] = tmp1;

    // Row 3: shift right by 3 (or left by 1)
    let tmp = state[3];
    state[3] = state[7];
    state[7] = state[11];
    state[11] = state[15];
    state[15] = tmp;
}

/// Apply MixColumns transformation
pub fn mix_columns(state: &mut [u8; 16]) {
    for col in 0..4 {
        let i = col * 4;
        let s0 = state[i];
        let s1 = state[i + 1];
        let s2 = state[i + 2];
        let s3 = state[i + 3];

        state[i] = gf_mul2(s0) ^ gf_mul3(s1) ^ s2 ^ s3;
        state[i + 1] = s0 ^ gf_mul2(s1) ^ gf_mul3(s2) ^ s3;
        state[i + 2] = s0 ^ s1 ^ gf_mul2(s2) ^ gf_mul3(s3);
        state[i + 3] = gf_mul3(s0) ^ s1 ^ s2 ^ gf_mul2(s3);
    }
}

/// Apply InvMixColumns transformation
pub fn inv_mix_columns(state: &mut [u8; 16]) {
    for col in 0..4 {
        let i = col * 4;
        let s0 = state[i];
        let s1 = state[i + 1];
        let s2 = state[i + 2];
        let s3 = state[i + 3];

        state[i] = gf_mul14(s0) ^ gf_mul11(s1) ^ gf_mul13(s2) ^ gf_mul9(s3);
        state[i + 1] = gf_mul9(s0) ^ gf_mul14(s1) ^ gf_mul11(s2) ^ gf_mul13(s3);
        state[i + 2] = gf_mul13(s0) ^ gf_mul9(s1) ^ gf_mul14(s2) ^ gf_mul11(s3);
        state[i + 3] = gf_mul11(s0) ^ gf_mul13(s1) ^ gf_mul9(s2) ^ gf_mul14(s3);
    }
}

/// SubWord: Apply S-box to each byte of a 32-bit word
pub fn sub_word(word: u32) -> u32 {
    let b0 = SBOX[(word & 0xFF) as usize] as u32;
    let b1 = SBOX[((word >> 8) & 0xFF) as usize] as u32;
    let b2 = SBOX[((word >> 16) & 0xFF) as usize] as u32;
    let b3 = SBOX[((word >> 24) & 0xFF) as usize] as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

/// RotWord: cyclically permute the bytes of a 32-bit word [b0,b1,b2,b3] ->
/// [b1,b2,b3,b0], as defined by FIPS-197 / the Intel SDM AESKEYGENASSIST op.
///
/// Words in this module are stored little-endian (b0 is the least-significant
/// byte: `b0 | b1<<8 | b2<<16 | b3<<24`), so producing `b1 | b2<<8 | b3<<16 |
/// b0<<24` is a rotate-RIGHT by 8 bits, NOT a rotate-left. (A rotate-left would
/// implement the inverse permutation [b3,b0,b1,b2] and was the previous bug.)
pub fn rot_word(word: u32) -> u32 {
    word.rotate_right(8)
}

/// AESENC: Perform one round of AES encryption
/// STATE := ShiftRows(SubBytes(STATE))
/// STATE := MixColumns(STATE)
/// DEST := STATE XOR RoundKey
pub fn aesenc(state_lo: u64, state_hi: u64, key_lo: u64, key_hi: u64) -> (u64, u64) {
    let mut state = state_to_bytes(state_lo, state_hi);
    shift_rows(&mut state);
    sub_bytes(&mut state);
    mix_columns(&mut state);
    let (result_lo, result_hi) = bytes_to_state(&state);
    (result_lo ^ key_lo, result_hi ^ key_hi)
}

/// AESENCLAST: Perform last round of AES encryption (no MixColumns)
/// STATE := ShiftRows(SubBytes(STATE))
/// DEST := STATE XOR RoundKey
pub fn aesenclast(state_lo: u64, state_hi: u64, key_lo: u64, key_hi: u64) -> (u64, u64) {
    let mut state = state_to_bytes(state_lo, state_hi);
    shift_rows(&mut state);
    sub_bytes(&mut state);
    let (result_lo, result_hi) = bytes_to_state(&state);
    (result_lo ^ key_lo, result_hi ^ key_hi)
}

/// AESDEC: Perform one round of AES decryption (Equivalent Inverse Cipher)
/// STATE := InvShiftRows(InvSubBytes(STATE))
/// STATE := InvMixColumns(STATE)
/// DEST := STATE XOR RoundKey
pub fn aesdec(state_lo: u64, state_hi: u64, key_lo: u64, key_hi: u64) -> (u64, u64) {
    let mut state = state_to_bytes(state_lo, state_hi);
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    inv_mix_columns(&mut state);
    let (result_lo, result_hi) = bytes_to_state(&state);
    (result_lo ^ key_lo, result_hi ^ key_hi)
}

/// AESDECLAST: Perform last round of AES decryption (no InvMixColumns)
/// STATE := InvShiftRows(InvSubBytes(STATE))
/// DEST := STATE XOR RoundKey
pub fn aesdeclast(state_lo: u64, state_hi: u64, key_lo: u64, key_hi: u64) -> (u64, u64) {
    let mut state = state_to_bytes(state_lo, state_hi);
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    let (result_lo, result_hi) = bytes_to_state(&state);
    (result_lo ^ key_lo, result_hi ^ key_hi)
}

/// AESIMC: Perform InvMixColumns transformation
pub fn aesimc(src_lo: u64, src_hi: u64) -> (u64, u64) {
    let mut state = state_to_bytes(src_lo, src_hi);
    inv_mix_columns(&mut state);
    bytes_to_state(&state)
}

/// AESKEYGENASSIST: AES Key Generation Assist
/// X3[31:0] := SRC[127:96]; X2[31:0] := SRC[95:64]; X1[31:0] := SRC[63:32]; X0[31:0] := SRC[31:0]
/// DEST[31:0] := SubWord(X1)
/// DEST[63:32] := RotWord(SubWord(X1)) XOR RCON
/// DEST[95:64] := SubWord(X3)
/// DEST[127:96] := RotWord(SubWord(X3)) XOR RCON
pub fn aeskeygenassist(src_lo: u64, src_hi: u64, rcon: u8) -> (u64, u64) {
    let x1 = (src_lo >> 32) as u32;
    let x3 = (src_hi >> 32) as u32;
    let rcon32 = rcon as u32;

    let sw_x1 = sub_word(x1);
    let sw_x3 = sub_word(x3);

    let dest0 = sw_x1;
    let dest1 = rot_word(sw_x1) ^ rcon32;
    let dest2 = sw_x3;
    let dest3 = rot_word(sw_x3) ^ rcon32;

    let lo = (dest0 as u64) | ((dest1 as u64) << 32);
    let hi = (dest2 as u64) | ((dest3 as u64) << 32);
    (lo, hi)
}
