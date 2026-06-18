use crate::common::{TestCase, get_xmm, run_until_hlt, set_xmm, setup_vm};

// Galois Field (GF2P8) Instructions for AES-GCM

// GF2P8AFFINEINVQB - Galois Field Affine Inverse with Quadratic Basis

#[test]
fn test_gf2p8affineinvqb_xmm_xmm_imm8() {
    TestCase::from("66 0f 3a cf c1 00").check();
}

#[test]
fn test_gf2p8affineinvqb_xmm_m128_imm8() {
    TestCase::from("66 0f 3a cf 00 00").check();
}

#[test]
fn test_gf2p8affineinvqb_xmm1_xmm2_00() {
    TestCase::from("66 0f 3a cf c1 00").check();
}

#[test]
fn test_gf2p8affineinvqb_xmm1_xmm2_ff() {
    TestCase::from("66 0f 3a cf c1 ff").check();
}

#[test]
fn test_gf2p8affineinvqb_xmm1_mem_42() {
    TestCase::from("66 0f 3a cf 00 42").check();
}

#[test]
fn test_vgf2p8affineinvqb_xmm_xmm_xmm_imm8() {
    TestCase::from("c4 e3 f9 cf c1 00").check();
}

#[test]
fn test_vgf2p8affineinvqb_ymm_ymm_ymm_imm8() {
    TestCase::from("c4 e3 fd cf c1 00").check();
}

#[test]
fn test_vgf2p8affineinvqb_xmm_xmm_m128_imm8() {
    TestCase::from("c4 e3 f9 cf 00 00").check();
}

// GF2P8AFFINEQB - Galois Field Affine with Quadratic Basis

#[test]
fn test_gf2p8affineqb_xmm_xmm_imm8() {
    TestCase::from("66 0f 3a ce c1 00").check();
}

#[test]
fn test_gf2p8affineqb_xmm_m128_imm8() {
    TestCase::from("66 0f 3a ce 00 00").check();
}

#[test]
fn test_gf2p8affineqb_xmm1_xmm2_00() {
    TestCase::from("66 0f 3a ce c1 00").check();
}

#[test]
fn test_gf2p8affineqb_xmm1_xmm2_ff() {
    TestCase::from("66 0f 3a ce c1 ff").check();
}

#[test]
fn test_gf2p8affineqb_xmm1_mem_42() {
    TestCase::from("66 0f 3a ce 00 42").check();
}

#[test]
fn test_vgf2p8affineqb_xmm_xmm_xmm_imm8() {
    TestCase::from("c4 e3 f9 ce c1 00").check();
}

#[test]
fn test_vgf2p8affineqb_ymm_ymm_ymm_imm8() {
    TestCase::from("c4 e3 fd ce c1 00").check();
}

#[test]
fn test_vgf2p8affineqb_xmm_xmm_m128_imm8() {
    TestCase::from("c4 e3 f9 ce 00 00").check();
}

// GF2P8MULB - Galois Field Multiply Bytes

#[test]
fn test_gf2p8mulb_xmm_xmm() {
    TestCase::from("66 0f 38 cf c1").check();
}

#[test]
fn test_gf2p8mulb_xmm_m128() {
    TestCase::from("66 0f 38 cf 00").check();
}

#[test]
fn test_gf2p8mulb_xmm1_xmm2() {
    TestCase::from("66 0f 38 cf c1").check();
}

#[test]
fn test_gf2p8mulb_xmm3_xmm4() {
    TestCase::from("66 0f 38 cf dc").check();
}

#[test]
fn test_gf2p8mulb_xmm1_mem() {
    TestCase::from("66 0f 38 cf 00").check();
}

#[test]
fn test_vgf2p8mulb_xmm_xmm_xmm() {
    TestCase::from("c4 e2 71 cf c1").check();
}

#[test]
fn test_vgf2p8mulb_ymm_ymm_ymm() {
    TestCase::from("c4 e2 75 cf c1").check();
}

#[test]
fn test_vgf2p8mulb_xmm_xmm_m128() {
    TestCase::from("c4 e2 71 cf 00").check();
}

// LOADIWKEY - Load internal wrapping key

#[test]
fn test_loadiwkey_xmm1_xmm2() {
    TestCase::from("f3 0f 38 dc c8").check();
}

#[test]
fn test_loadiwkey_xmm3_xmm4() {
    TestCase::from("f3 0f 38 dc dc").check();
}

#[test]
fn test_loadiwkey_xmm5_xmm6() {
    TestCase::from("f3 0f 38 dc ee").check();
}

// ENCODEKEY128 - Encode 128-bit key

#[test]
fn test_encodekey128_r32_r32() {
    TestCase::from("f3 0f 38 fa c1").check();
}

#[test]
fn test_encodekey128_eax_ecx() {
    TestCase::from("f3 0f 38 fa c1").check();
}

#[test]
fn test_encodekey128_edx_ebx() {
    TestCase::from("f3 0f 38 fa d3").check();
}

// ENCODEKEY256 - Encode 256-bit key

#[test]
fn test_encodekey256_r32_r32() {
    TestCase::from("f3 0f 38 fb c1").check();
}

#[test]
fn test_encodekey256_eax_ecx() {
    TestCase::from("f3 0f 38 fb c1").check();
}

#[test]
fn test_encodekey256_edx_ebx() {
    TestCase::from("f3 0f 38 fb d3").check();
}

// ============================================================================
// GFNI Known-Answer Tests (Intel SDM GF2P8MULB / GF2P8AFFINEQB)
// ============================================================================
//
// NOTE: GFNI is NOT implemented by the emulator (the GF2P8* legacy-map opcodes
// fall through to `_ => Err(...)` in escape_38/escape_3a), so these
// known-answer tests are `#[ignore]`d. They encode the correct Intel SDM
// expected results and will validate the implementation once GFNI is added.

#[test]
fn kat_gf2p8mulb_fips_57x83() {
    // FIPS-197 §4.2 worked example: 0x57 * 0x83 = 0xC1 in GF(2^8).
    let code = [0x66, 0x0f, 0x38, 0xcf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x57);
    set_xmm(&mem, &mut vcpu, 1, 0x83);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0) & 0xff, 0xc1);
}

#[test]
fn kat_gf2p8mulb_by_two() {
    // Multiply each lane by 0x02 (xtime): byte i = 2*i reduced by 0x11B.
    // input  bytes 00..0f  -> u128 LE 0f0e..0100
    // output bytes 00,02,04,..,1e -> u128 LE 1e1c..0200
    let code = [0x66, 0x0f, 0x38, 0xcf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x0f0e0d0c0b0a09080706050403020100);
    set_xmm(&mem, &mut vcpu, 1, 0x02020202020202020202020202020202);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x1e1c1a18161412100e0c0a0806040200);
}

#[test]
fn kat_gf2p8affineqb_identity() {
    // GFNI identity matrix per qword (qword 0x0102040810204080) + imm8=0 is the
    // identity transform: output == input. (0x8040201008040201 would bit-REVERSE
    // each byte, not preserve it.)
    let code = [0x66, 0x0f, 0x3a, 0xce, 0xc1, 0x00, 0xf4];
    let x: u128 = 0x0011223344556677_8899aabbccddeeff;
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, x);
    set_xmm(&mem, &mut vcpu, 1, 0x0102040810204080_0102040810204080);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), x);
}
