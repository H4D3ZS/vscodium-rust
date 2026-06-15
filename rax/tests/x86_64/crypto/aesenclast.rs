use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// AESENCLAST - Perform Last Round of an AES Encryption Flow
//
// Performs the last round of AES encryption using ShiftRows, SubBytes,
// and XOR with the round key. Unlike AESENC, this instruction does NOT
// perform MixColumns, as per AES specification for the final round.
//
// Operation:
//   STATE := SRC1
//   RoundKey := SRC2
//   STATE := ShiftRows(STATE)
//   STATE := SubBytes(STATE)
//   DEST[127:0] := STATE XOR RoundKey
//
// Opcodes:
// 66 0F 38 DD /r              AESENCLAST xmm1, xmm2/m128      - Perform last round of AES encryption

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// AESENCLAST Tests - Register to Register Forms
// ============================================================================

#[test]
fn test_aesenclast_xmm0_xmm1() {
    // AESENCLAST XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xc1, // AESENCLAST XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm1_xmm2() {
    // AESENCLAST XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xca, // AESENCLAST XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm2_xmm3() {
    // AESENCLAST XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xd3, // AESENCLAST XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm3_xmm4() {
    // AESENCLAST XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xdc, // AESENCLAST XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm4_xmm5() {
    // AESENCLAST XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xe5, // AESENCLAST XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm5_xmm6() {
    // AESENCLAST XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xee, // AESENCLAST XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm6_xmm7() {
    // AESENCLAST XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xf7, // AESENCLAST XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm7_xmm0() {
    // AESENCLAST XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xf8, // AESENCLAST XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Tests - Extended Registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_aesenclast_xmm8_xmm9() {
    // AESENCLAST XMM8, XMM9 (requires REX.R and REX.B)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xc1, // AESENCLAST XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm9_xmm10() {
    // AESENCLAST XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xca, // AESENCLAST XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm10_xmm11() {
    // AESENCLAST XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xd3, // AESENCLAST XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm11_xmm12() {
    // AESENCLAST XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xdc, // AESENCLAST XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm12_xmm13() {
    // AESENCLAST XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xe5, // AESENCLAST XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm13_xmm14() {
    // AESENCLAST XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xee, // AESENCLAST XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm14_xmm15() {
    // AESENCLAST XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xf7, // AESENCLAST XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm15_xmm8() {
    // AESENCLAST XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xf8, // AESENCLAST XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Tests - Memory Operand Forms
// ============================================================================

#[test]
fn test_aesenclast_xmm0_mem() {
    // AESENCLAST XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm1_mem() {
    // AESENCLAST XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x0c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm2_mem() {
    // AESENCLAST XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm7_mem() {
    // AESENCLAST XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm8_mem() {
    // AESENCLAST XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0x04, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM8, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm15_mem() {
    // AESENCLAST XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Tests - Mixed Register Combinations
// ============================================================================

#[test]
fn test_aesenclast_xmm0_xmm7() {
    // AESENCLAST XMM0, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xc7, // AESENCLAST XMM0, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm7_xmm7() {
    // AESENCLAST XMM7, XMM7 (same register)
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xff, // AESENCLAST XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm0_xmm15() {
    // AESENCLAST XMM0, XMM15 (low to high extended)
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0xdd, 0xc7, // AESENCLAST XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm15_xmm0() {
    // AESENCLAST XMM15, XMM0 (high extended to low)
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0xf8, // AESENCLAST XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Tests - After Regular Rounds
// ============================================================================

#[test]
fn test_aesenclast_after_aesenc() {
    // Typical AES flow: AESENC followed by AESENCLAST
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xc1, // AESENC XMM0, XMM1
        0x66, 0x0f, 0x38, 0xdc, 0xc2, // AESENC XMM0, XMM2
        0x66, 0x0f, 0x38, 0xdd, 0xc3, // AESENCLAST XMM0, XMM3 (final round)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_aes128_flow() {
    // Simulating AES-128 (10 rounds: 9 regular + 1 final)
    let code = [
        0x66, 0x0f, 0x38, 0xdc, 0xc1, // AESENC XMM0, XMM1 (round 1)
        0x66, 0x0f, 0x38, 0xdc, 0xc2, // AESENC XMM0, XMM2 (round 2)
        0x66, 0x0f, 0x38, 0xdc, 0xc3, // AESENC XMM0, XMM3 (round 3)
        0x66, 0x0f, 0x38, 0xdd, 0xc4, // AESENCLAST XMM0, XMM4 (round 10)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_multiple_blocks() {
    // Processing multiple AES blocks
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xc1, // AESENCLAST XMM0, XMM1 (block 1)
        0x66, 0x0f, 0x38, 0xdd, 0xd3, // AESENCLAST XMM2, XMM3 (block 2)
        0x66, 0x0f, 0x38, 0xdd, 0xe5, // AESENCLAST XMM4, XMM5 (block 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Tests - All Registers Coverage
// ============================================================================

#[test]
fn test_aesenclast_all_low_regs() {
    // Test all low XMM registers (XMM0-XMM7)
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0xc0, // AESENCLAST XMM0, XMM0
        0x66, 0x0f, 0x38, 0xdd, 0xc9, // AESENCLAST XMM1, XMM1
        0x66, 0x0f, 0x38, 0xdd, 0xd2, // AESENCLAST XMM2, XMM2
        0x66, 0x0f, 0x38, 0xdd, 0xdb, // AESENCLAST XMM3, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_all_high_regs() {
    // Test all high XMM registers (XMM8-XMM15)
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xc0, // AESENCLAST XMM8, XMM8
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xc9, // AESENCLAST XMM9, XMM9
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xd2, // AESENCLAST XMM10, XMM10
        0x66, 0x45, 0x0f, 0x38, 0xdd, 0xff, // AESENCLAST XMM15, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm3_mem() {
    // AESENCLAST XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x1c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm4_mem() {
    // AESENCLAST XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0x38, 0xdd, 0x24, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM4, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm9_mem() {
    // AESENCLAST XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0x0c, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM9, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aesenclast_xmm10_mem() {
    // AESENCLAST XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xdd, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // AESENCLAST XMM10, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESENCLAST Known-Answer Tests (Intel AES-NI reference vector)
// ============================================================================
//
// AESENCLAST = ShiftRows; SubBytes; XOR round key (no MixColumns).
// Same canonical Intel input as AESENC:
//   state = 7b5b54657374566563746f725d53475d
//   rkey  = 48692853686179295b477565726f6e5d
//   AESENCLAST result = c7fb881e938c5964177ec42553fdc611

const AESENCLAST_STATE: u128 = 0x7b5b54657374566563746f725d53475d;
const AESENCLAST_RKEY: u128 = 0x48692853686179295b477565726f6e5d;
const AESENCLAST_RESULT: u128 = 0xc7fb881e938c5964177ec42553fdc611;

#[test]
fn kat_aesenclast_intel_vector() {
    // AESENCLAST XMM0, XMM1  (66 0F 38 DD C1)
    let code = [0x66, 0x0f, 0x38, 0xdd, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, AESENCLAST_STATE);
    set_xmm(&mem, &mut vcpu, 1, AESENCLAST_RKEY);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        AESENCLAST_RESULT,
        "AESENCLAST produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        AESENCLAST_RESULT
    );
}

#[test]
fn kat_aesenclast_zero_state_zero_key() {
    // SubBytes(0)=0x63 everywhere; ShiftRows is a no-op on a uniform state;
    // no MixColumns; XOR zero key => all-0x63.
    let code = [0x66, 0x0f, 0x38, 0xdd, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0);
    set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x6363636363636363_6363636363636363u128);
}
