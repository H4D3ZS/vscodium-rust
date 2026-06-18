use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// AESDECLAST - Perform Last Round of an AES Decryption Flow
//
// Performs the last round of AES decryption using the Equivalent Inverse Cipher.
// Uses InvShiftRows, InvSubBytes, and XOR with the round key.
// Unlike AESDEC, this instruction does NOT perform InvMixColumns,
// as per AES specification for the final decryption round.
//
// Operation:
//   STATE := SRC1
//   RoundKey := SRC2
//   STATE := InvShiftRows(STATE)
//   STATE := InvSubBytes(STATE)
//   DEST[127:0] := STATE XOR RoundKey
//
// Opcodes:
// 66 0F 38 DF /r              AESDECLAST xmm1, xmm2/m128     - Perform last round of AES decryption

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// AESDECLAST Tests - Register to Register Forms
// ============================================================================

#[test]
fn test_aesdeclast_xmm0_xmm1() {
    // AESDECLAST XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xc1, // AESDECLAST XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm1_xmm2() {
    // AESDECLAST XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xca, // AESDECLAST XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm2_xmm3() {
    // AESDECLAST XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xd3, // AESDECLAST XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm3_xmm4() {
    // AESDECLAST XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xdc, // AESDECLAST XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm4_xmm5() {
    // AESDECLAST XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xe5, // AESDECLAST XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm5_xmm6() {
    // AESDECLAST XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xee, // AESDECLAST XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm6_xmm7() {
    // AESDECLAST XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xf7, // AESDECLAST XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm7_xmm0() {
    // AESDECLAST XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xf8, // AESDECLAST XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Tests - Extended Registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_aesdeclast_xmm8_xmm9() {
    // AESDECLAST XMM8, XMM9 (requires REX.R and REX.B)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xc1, // AESDECLAST XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm9_xmm10() {
    // AESDECLAST XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xca, // AESDECLAST XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm10_xmm11() {
    // AESDECLAST XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xd3, // AESDECLAST XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm11_xmm12() {
    // AESDECLAST XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xdc, // AESDECLAST XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm12_xmm13() {
    // AESDECLAST XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xe5, // AESDECLAST XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm13_xmm14() {
    // AESDECLAST XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xee, // AESDECLAST XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm14_xmm15() {
    // AESDECLAST XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xf7, // AESDECLAST XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm15_xmm8() {
    // AESDECLAST XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xf8, // AESDECLAST XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Tests - Memory Operand Forms
// ============================================================================

#[test]
fn test_aesdeclast_xmm0_mem() {
    // AESDECLAST XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm1_mem() {
    // AESDECLAST XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x0c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm2_mem() {
    // AESDECLAST XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm7_mem() {
    // AESDECLAST XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm8_mem() {
    // AESDECLAST XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM8, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm15_mem() {
    // AESDECLAST XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdf, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Tests - Mixed Register Combinations
// ============================================================================

#[test]
fn test_aesdeclast_xmm0_xmm7() {
    // AESDECLAST XMM0, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xc7, // AESDECLAST XMM0, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm7_xmm7() {
    // AESDECLAST XMM7, XMM7 (same register)
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xff, // AESDECLAST XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm0_xmm15() {
    // AESDECLAST XMM0, XMM15 (low to high extended)
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0xdf, 0xc7, // AESDECLAST XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm15_xmm0() {
    // AESDECLAST XMM15, XMM0 (high extended to low)
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdf, 0xf8, // AESDECLAST XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Tests - After Regular Rounds
// ============================================================================

#[test]
fn test_aesdeclast_after_aesdec() {
    // Typical AES flow: AESDEC followed by AESDECLAST
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xde, 0xc2, // AESDEC XMM0, XMM2
        0x66, 0x0f, 0x38, 0xdf, 0xc3, // AESDECLAST XMM0, XMM3 (final round)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_aes128_flow() {
    // Simulating AES-128 decryption (10 rounds: 9 regular + 1 final)
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1 (round 1)
        0x66, 0x0f, 0x38, 0xde, 0xc2, // AESDEC XMM0, XMM2 (round 2)
        0x66, 0x0f, 0x38, 0xde, 0xc3, // AESDEC XMM0, XMM3 (round 3)
        0x66, 0x0f, 0x38, 0xdf, 0xc4, // AESDECLAST XMM0, XMM4 (round 10)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_multiple_blocks() {
    // Processing multiple AES blocks
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xc1, // AESDECLAST XMM0, XMM1 (block 1)
        0x66, 0x0f, 0x38, 0xdf, 0xd3, // AESDECLAST XMM2, XMM3 (block 2)
        0x66, 0x0f, 0x38, 0xdf, 0xe5, // AESDECLAST XMM4, XMM5 (block 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Tests - All Registers Coverage
// ============================================================================

#[test]
fn test_aesdeclast_all_low_regs() {
    // Test all low XMM registers (XMM0-XMM7)
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0xc0, // AESDECLAST XMM0, XMM0
        0x66, 0x0f, 0x38, 0xdf, 0xc9, // AESDECLAST XMM1, XMM1
        0x66, 0x0f, 0x38, 0xdf, 0xd2, // AESDECLAST XMM2, XMM2
        0x66, 0x0f, 0x38, 0xdf, 0xdb, // AESDECLAST XMM3, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_all_high_regs() {
    // Test all high XMM registers (XMM8-XMM15)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xc0, // AESDECLAST XMM8, XMM8
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xc9, // AESDECLAST XMM9, XMM9
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xd2, // AESDECLAST XMM10, XMM10
        0x66, 0x45, 0x0f, 0x38, 0xdf, 0xff, // AESDECLAST XMM15, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm3_mem() {
    // AESDECLAST XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x1c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm4_mem() {
    // AESDECLAST XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdf, 0x24, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM4, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm9_mem() {
    // AESDECLAST XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdf, 0x0c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM9, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdeclast_xmm10_mem() {
    // AESDECLAST XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdf, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDECLAST XMM10, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDECLAST Known-Answer Tests (Intel AES-NI reference vector)
// ============================================================================
//
// AESDECLAST = InvShiftRows; InvSubBytes; XOR round key (no InvMixColumns).
// Canonical Intel input:
//   state = 7b5b54657374566563746f725d53475d
//   rkey  = 48692853686179295b477565726f6e5d
//   AESDECLAST result = c5a391ef6b317f95d410637b72a593d0

const AESDECLAST_STATE: u128 = 0x7b5b54657374566563746f725d53475d;
const AESDECLAST_RKEY: u128 = 0x48692853686179295b477565726f6e5d;
const AESDECLAST_RESULT: u128 = 0xc5a391ef6b317f95d410637b72a593d0;

#[test]
fn kat_aesdeclast_intel_vector() {
    // AESDECLAST XMM0, XMM1  (66 0F 38 DF C1)
    let code = [0x66, 0x0f, 0x38, 0xdf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, AESDECLAST_STATE);
    set_xmm(&mem, &mut vcpu, 1, AESDECLAST_RKEY);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        AESDECLAST_RESULT,
        "AESDECLAST produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        AESDECLAST_RESULT
    );
}

#[test]
fn kat_aesdeclast_zero_state_zero_key() {
    // InvSubBytes(0)=0x52 everywhere; InvShiftRows is a no-op on a uniform
    // state; no InvMixColumns; XOR zero key => all-0x52.
    let code = [0x66, 0x0f, 0x38, 0xdf, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0);
    set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x5252525252525252_5252525252525252u128);
}
