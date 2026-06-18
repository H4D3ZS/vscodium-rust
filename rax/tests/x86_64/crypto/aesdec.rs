use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// AESDEC - Perform One Round of an AES Decryption Flow
//
// Performs a single round of AES decryption using the Equivalent Inverse Cipher.
// Uses InvShiftRows, InvSubBytes, InvMixColumns, and XOR with the round key.
// This instruction is used for all but the last decryption round.
// For the last round, use AESDECLAST (which omits InvMixColumns).
//
// Operation:
//   STATE := SRC1
//   RoundKey := SRC2
//   STATE := InvShiftRows(STATE)
//   STATE := InvSubBytes(STATE)
//   STATE := InvMixColumns(STATE)
//   DEST[127:0] := STATE XOR RoundKey
//
// Opcodes:
// 66 0F 38 DE /r              AESDEC xmm1, xmm2/m128          - Perform one round of AES decryption

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// AESDEC Tests - Register to Register Forms
// ============================================================================

#[test]
fn test_aesdec_xmm0_xmm1() {
    // AESDEC XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm1_xmm2() {
    // AESDEC XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xca, // AESDEC XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm2_xmm3() {
    // AESDEC XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xd3, // AESDEC XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm3_xmm4() {
    // AESDEC XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xdc, // AESDEC XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm4_xmm5() {
    // AESDEC XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xe5, // AESDEC XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm5_xmm6() {
    // AESDEC XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xee, // AESDEC XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm6_xmm7() {
    // AESDEC XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xf7, // AESDEC XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm7_xmm0() {
    // AESDEC XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xf8, // AESDEC XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - Extended Registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_aesdec_xmm8_xmm9() {
    // AESDEC XMM8, XMM9 (requires REX.R and REX.B)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm9_xmm10() {
    // AESDEC XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xca, // AESDEC XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm10_xmm11() {
    // AESDEC XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xd3, // AESDEC XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm11_xmm12() {
    // AESDEC XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xdc, // AESDEC XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm12_xmm13() {
    // AESDEC XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xe5, // AESDEC XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm13_xmm14() {
    // AESDEC XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xee, // AESDEC XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm14_xmm15() {
    // AESDEC XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xf7, // AESDEC XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm15_xmm8() {
    // AESDEC XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xf8, // AESDEC XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - Memory Operand Forms
// ============================================================================

#[test]
fn test_aesdec_xmm0_mem() {
    // AESDEC XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm1_mem() {
    // AESDEC XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm2_mem() {
    // AESDEC XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm3_mem() {
    // AESDEC XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm4_mem() {
    // AESDEC XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM4, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm5_mem() {
    // AESDEC XMM5, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM5, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm6_mem() {
    // AESDEC XMM6, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM6, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm7_mem() {
    // AESDEC XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm8_mem() {
    // AESDEC XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDEC XMM8, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm15_mem() {
    // AESDEC XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDEC XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - Mixed Register Combinations
// ============================================================================

#[test]
fn test_aesdec_xmm0_xmm7() {
    // AESDEC XMM0, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc7, // AESDEC XMM0, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm7_xmm7() {
    // AESDEC XMM7, XMM7 (same register)
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xff, // AESDEC XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm0_xmm15() {
    // AESDEC XMM0, XMM15 (low to high extended)
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0xde, 0xc7, // AESDEC XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm15_xmm0() {
    // AESDEC XMM15, XMM0 (high extended to low)
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0xf8, // AESDEC XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm8_xmm0() {
    // AESDEC XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0xc0, // AESDEC XMM8, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm0_xmm8() {
    // AESDEC XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0xde, 0xc0, // AESDEC XMM0, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - Sequential Operations
// ============================================================================

#[test]
fn test_aesdec_sequential_rounds() {
    // Multiple AESDEC operations simulating AES decryption rounds
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1 (round 1)
        0x66, 0x0f, 0x38, 0xde, 0xc2, // AESDEC XMM0, XMM2 (round 2)
        0x66, 0x0f, 0x38, 0xde, 0xc3, // AESDEC XMM0, XMM3 (round 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_alternating_registers() {
    // Alternating between different XMM registers
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xde, 0xca, // AESDEC XMM1, XMM2
        0x66, 0x0f, 0x38, 0xde, 0xd3, // AESDEC XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_parallel_operations() {
    // Operating on different register pairs
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc1, // AESDEC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xde, 0xd3, // AESDEC XMM2, XMM3
        0x66, 0x0f, 0x38, 0xde, 0xe5, // AESDEC XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - All 16 XMM Registers as Destination
// ============================================================================

#[test]
fn test_aesdec_all_destinations() {
    // Test each XMM register as destination
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0xc0, // AESDEC XMM0, XMM0
        0x66, 0x0f, 0x38, 0xde, 0xc9, // AESDEC XMM1, XMM1
        0x66, 0x0f, 0x38, 0xde, 0xd2, // AESDEC XMM2, XMM2
        0x66, 0x0f, 0x38, 0xde, 0xdb, // AESDEC XMM3, XMM3
        0x66, 0x0f, 0x38, 0xde, 0xe4, // AESDEC XMM4, XMM4
        0x66, 0x0f, 0x38, 0xde, 0xed, // AESDEC XMM5, XMM5
        0x66, 0x0f, 0x38, 0xde, 0xf6, // AESDEC XMM6, XMM6
        0x66, 0x0f, 0x38, 0xde, 0xff, // AESDEC XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_all_extended_destinations() {
    // Test each extended XMM register as destination
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xc0, // AESDEC XMM8, XMM8
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xc9, // AESDEC XMM9, XMM9
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xd2, // AESDEC XMM10, XMM10
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xdb, // AESDEC XMM11, XMM11
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xe4, // AESDEC XMM12, XMM12
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xed, // AESDEC XMM13, XMM13
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xf6, // AESDEC XMM14, XMM14
        0x66, 0x45, 0x0f, 0x38, 0xde, 0xff, // AESDEC XMM15, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Tests - Additional Coverage
// ============================================================================

#[test]
fn test_aesdec_xmm9_mem() {
    // AESDEC XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0x0c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDEC XMM9, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesdec_xmm10_mem() {
    // AESDEC XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xde, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESDEC XMM10, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESDEC Known-Answer Tests (Intel AES-NI reference vector)
// ============================================================================
//
// AESDEC = InvShiftRows; InvSubBytes; InvMixColumns; XOR round key
// (Equivalent Inverse Cipher round). Using the canonical Intel input:
//   state = 7b5b54657374566563746f725d53475d
//   rkey  = 48692853686179295b477565726f6e5d
//   AESDEC result = 138ac342faea2787b58eb95eb730392a

const AESDEC_STATE: u128 = 0x7b5b54657374566563746f725d53475d;
const AESDEC_RKEY: u128 = 0x48692853686179295b477565726f6e5d;
const AESDEC_RESULT: u128 = 0x138ac342faea2787b58eb95eb730392a;

#[test]
fn kat_aesdec_intel_vector() {
    // AESDEC XMM0, XMM1  (66 0F 38 DE C1)
    let code = [0x66, 0x0f, 0x38, 0xde, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, AESDEC_STATE);
    set_xmm(&mem, &mut vcpu, 1, AESDEC_RKEY);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        AESDEC_RESULT,
        "AESDEC produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        AESDEC_RESULT
    );
}

#[test]
fn kat_aesdec_memory_operand() {
    // AESDEC XMM0, [0x3000]: round key sourced from memory, same Intel vector.
    let code = [
        0x66, 0x0f, 0x38, 0xde, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // AESDEC XMM0,[0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, AESDEC_STATE);
    mem.write_slice(&AESDEC_RKEY.to_le_bytes(), GuestAddress(0x3000))
        .unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), AESDEC_RESULT);
}
