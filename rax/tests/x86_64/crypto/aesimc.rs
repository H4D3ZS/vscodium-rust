use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// AESIMC - Perform the AES InvMixColumn Transformation
//
// Performs the InvMixColumns transformation on a 128-bit round key.
// This instruction should be applied to expanded AES round keys
// (except for the first and last round keys) to prepare them for
// decryption using the "Equivalent Inverse Cipher" (defined in FIPS 197).
//
// The InvMixColumns transformation is a key step in preparing
// encryption round keys for use in decryption.
//
// Operation:
//   DEST[127:0] := InvMixColumns(SRC)
//
// Opcodes:
// 66 0F 38 DB /r              AESIMC xmm1, xmm2/m128          - Perform InvMixColumn transformation

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// AESIMC Tests - Register to Register Forms
// ============================================================================

#[test]
fn test_aesimc_xmm0_xmm1() {
    // AESIMC XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm1_xmm2() {
    // AESIMC XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xca, // AESIMC XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm2_xmm3() {
    // AESIMC XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xd3, // AESIMC XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm3_xmm4() {
    // AESIMC XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xdc, // AESIMC XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm4_xmm5() {
    // AESIMC XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xe5, // AESIMC XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm5_xmm6() {
    // AESIMC XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xee, // AESIMC XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm6_xmm7() {
    // AESIMC XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xf7, // AESIMC XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm7_xmm0() {
    // AESIMC XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xf8, // AESIMC XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESIMC Tests - Extended Registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_aesimc_xmm8_xmm9() {
    // AESIMC XMM8, XMM9 (requires REX.R and REX.B)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm9_xmm10() {
    // AESIMC XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xca, // AESIMC XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm10_xmm11() {
    // AESIMC XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xd3, // AESIMC XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm11_xmm12() {
    // AESIMC XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xdc, // AESIMC XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm12_xmm13() {
    // AESIMC XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xe5, // AESIMC XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm13_xmm14() {
    // AESIMC XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xee, // AESIMC XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm14_xmm15() {
    // AESIMC XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xf7, // AESIMC XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm15_xmm8() {
    // AESIMC XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xf8, // AESIMC XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESIMC Tests - Memory Operand Forms
// ============================================================================

#[test]
fn test_aesimc_xmm0_mem() {
    // AESIMC XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESIMC XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm1_mem() {
    // AESIMC XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESIMC XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm7_mem() {
    // AESIMC XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESIMC XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm8_mem() {
    // AESIMC XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdb, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESIMC XMM8, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm15_mem() {
    // AESIMC XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdb, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESIMC XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESIMC Tests - Same Register (Destination = Source)
// ============================================================================

#[test]
fn test_aesimc_xmm0_xmm0() {
    // AESIMC XMM0, XMM0 (in-place transformation)
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm7_xmm7() {
    // AESIMC XMM7, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xff, // AESIMC XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm8_xmm8() {
    // AESIMC XMM8, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM8, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm15_xmm15() {
    // AESIMC XMM15, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdb, 0xff, // AESIMC XMM15, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESIMC Tests - Key Schedule Preparation
// ============================================================================

#[test]
fn test_aesimc_key_schedule_prep() {
    // Preparing multiple round keys for decryption
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc1, // AESIMC XMM0, XMM1 (prepare round key 1)
        0x66, 0x0f, 0x38, 0xdb, 0xca, // AESIMC XMM1, XMM2 (prepare round key 2)
        0x66, 0x0f, 0x38, 0xdb, 0xd3, // AESIMIC XMM2, XMM3 (prepare round key 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_sequential_transforms() {
    // Sequential InvMixColumns transformations
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM0
        0x66, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM0 (double transform)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_mixed_combinations() {
    // Mixed register combinations
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0xc7, // AESIMC XMM0, XMM7
        0x66, 0x41, 0x0f, 0x38, 0xdb, 0xc0, // AESIMC XMM0, XMM8
        0x66, 0x44, 0x0f, 0x38, 0xdb, 0xf8, // AESIMC XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm2_mem() {
    // AESIMC XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // AESIMC XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesimc_xmm3_mem() {
    // AESIMC XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdb, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESIMC XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESIMC Known-Answer Tests
// ============================================================================
//
// AESIMC performs InvMixColumns(SRC) with no key XOR. Verified against the
// FIPS-197 / Intel AES-NI InvMixColumns transform on the canonical input:
//   src = 7b5b54657374566563746f725d53475d
//   AESIMC result = 627a6f6644b109c82b18330a81c3b3e5

const AESIMC_SRC: u128 = 0x7b5b54657374566563746f725d53475d;
const AESIMC_RESULT: u128 = 0x627a6f6644b109c82b18330a81c3b3e5;

#[test]
fn kat_aesimc_intel_vector() {
    // AESIMC XMM0, XMM1  (66 0F 38 DB C1): dst = xmm0, src = xmm1
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, AESIMC_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        AESIMC_RESULT,
        "AESIMC produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        AESIMC_RESULT
    );
}

#[test]
fn kat_aesimc_zero_is_zero() {
    // InvMixColumns is GF(2^8)-linear, so InvMixColumns(0) == 0.
    let code = [0x66, 0x0f, 0x38, 0xdb, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0);
}
