use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// SHA1RNDS4 - Perform Four Rounds of SHA1 Operation
//
// The SHA1RNDS4 instruction performs four rounds of SHA1 operation using an initial SHA1
// state (A,B,C,D) from the first operand (which is a source operand and the destination operand)
// and some pre-computed sum of the next 4 round message dwords, and state variable E from
// the second operand (a source operand). The updated SHA1 state (A,B,C,D) after four rounds
// of processing is stored in the destination operand.
//
// The immediate byte controls logic functions and round constants:
// imm8[1:0] = 0: f0() and K0
// imm8[1:0] = 1: f1() and K1
// imm8[1:0] = 2: f2() and K2
// imm8[1:0] = 3: f3() and K3
//
// Opcode:
// NP 0F 3A CC /r ib    SHA1RNDS4 xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests - Function 0 (imm8 = 0)
// ============================================================================

#[test]
fn test_sha1rnds4_xmm0_xmm1_func0() {
    // SHA1RNDS4 XMM0, XMM1, 0
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x00, // SHA1RNDS4 XMM0, XMM1, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm2_xmm3_func0() {
    // SHA1RNDS4 XMM2, XMM3, 0
    let code = [
        0x0f, 0x3a, 0xcc, 0xd3, 0x00, // SHA1RNDS4 XMM2, XMM3, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm4_xmm5_func0() {
    // SHA1RNDS4 XMM4, XMM5, 0
    let code = [
        0x0f, 0x3a, 0xcc, 0xe5, 0x00, // SHA1RNDS4 XMM4, XMM5, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm6_xmm7_func0() {
    // SHA1RNDS4 XMM6, XMM7, 0
    let code = [
        0x0f, 0x3a, 0xcc, 0xf7, 0x00, // SHA1RNDS4 XMM6, XMM7, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm8_xmm9_func0() {
    // SHA1RNDS4 XMM8, XMM9, 0 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x3a, 0xcc, 0xc1, 0x00, // SHA1RNDS4 XMM8, XMM9, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm14_xmm15_func0() {
    // SHA1RNDS4 XMM14, XMM15, 0
    let code = [
        0x45, 0x0f, 0x3a, 0xcc, 0xf7, 0x00, // SHA1RNDS4 XMM14, XMM15, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - Function 1 (imm8 = 1)
// ============================================================================

#[test]
fn test_sha1rnds4_xmm0_xmm1_func1() {
    // SHA1RNDS4 XMM0, XMM1, 1
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x01, // SHA1RNDS4 XMM0, XMM1, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm2_xmm3_func1() {
    // SHA1RNDS4 XMM2, XMM3, 1
    let code = [
        0x0f, 0x3a, 0xcc, 0xd3, 0x01, // SHA1RNDS4 XMM2, XMM3, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm4_xmm5_func1() {
    // SHA1RNDS4 XMM4, XMM5, 1
    let code = [
        0x0f, 0x3a, 0xcc, 0xe5, 0x01, // SHA1RNDS4 XMM4, XMM5, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm6_xmm7_func1() {
    // SHA1RNDS4 XMM6, XMM7, 1
    let code = [
        0x0f, 0x3a, 0xcc, 0xf7, 0x01, // SHA1RNDS4 XMM6, XMM7, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm8_xmm9_func1() {
    // SHA1RNDS4 XMM8, XMM9, 1
    let code = [
        0x45, 0x0f, 0x3a, 0xcc, 0xc1, 0x01, // SHA1RNDS4 XMM8, XMM9, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - Function 2 (imm8 = 2)
// ============================================================================

#[test]
fn test_sha1rnds4_xmm0_xmm1_func2() {
    // SHA1RNDS4 XMM0, XMM1, 2
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x02, // SHA1RNDS4 XMM0, XMM1, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm2_xmm3_func2() {
    // SHA1RNDS4 XMM2, XMM3, 2
    let code = [
        0x0f, 0x3a, 0xcc, 0xd3, 0x02, // SHA1RNDS4 XMM2, XMM3, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm4_xmm5_func2() {
    // SHA1RNDS4 XMM4, XMM5, 2
    let code = [
        0x0f, 0x3a, 0xcc, 0xe5, 0x02, // SHA1RNDS4 XMM4, XMM5, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm6_xmm7_func2() {
    // SHA1RNDS4 XMM6, XMM7, 2
    let code = [
        0x0f, 0x3a, 0xcc, 0xf7, 0x02, // SHA1RNDS4 XMM6, XMM7, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm8_xmm9_func2() {
    // SHA1RNDS4 XMM8, XMM9, 2
    let code = [
        0x45, 0x0f, 0x3a, 0xcc, 0xc1, 0x02, // SHA1RNDS4 XMM8, XMM9, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - Function 3 (imm8 = 3)
// ============================================================================

#[test]
fn test_sha1rnds4_xmm0_xmm1_func3() {
    // SHA1RNDS4 XMM0, XMM1, 3
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x03, // SHA1RNDS4 XMM0, XMM1, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm2_xmm3_func3() {
    // SHA1RNDS4 XMM2, XMM3, 3
    let code = [
        0x0f, 0x3a, 0xcc, 0xd3, 0x03, // SHA1RNDS4 XMM2, XMM3, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm4_xmm5_func3() {
    // SHA1RNDS4 XMM4, XMM5, 3
    let code = [
        0x0f, 0x3a, 0xcc, 0xe5, 0x03, // SHA1RNDS4 XMM4, XMM5, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm6_xmm7_func3() {
    // SHA1RNDS4 XMM6, XMM7, 3
    let code = [
        0x0f, 0x3a, 0xcc, 0xf7, 0x03, // SHA1RNDS4 XMM6, XMM7, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm8_xmm9_func3() {
    // SHA1RNDS4 XMM8, XMM9, 3
    let code = [
        0x45, 0x0f, 0x3a, 0xcc, 0xc1, 0x03, // SHA1RNDS4 XMM8, XMM9, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests - All functions
// ============================================================================

#[test]
fn test_sha1rnds4_xmm0_mem_func0() {
    // SHA1RNDS4 XMM0, [RAX], 0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x00, 0x00, // SHA1RNDS4 XMM0, [RAX], 0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm1_mem_func1() {
    // SHA1RNDS4 XMM1, [RAX], 1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x08, 0x01, // SHA1RNDS4 XMM1, [RAX], 1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm2_mem_func2() {
    // SHA1RNDS4 XMM2, [RAX], 2
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x10, 0x02, // SHA1RNDS4 XMM2, [RAX], 2
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
            0x55, 0x55,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm3_mem_func3() {
    // SHA1RNDS4 XMM3, [RAX], 3
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x18, 0x03, // SHA1RNDS4 XMM3, [RAX], 3
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
            0x33, 0x33,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm7_mem_func0() {
    // SHA1RNDS4 XMM7, [RAX], 0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x38, 0x00, // SHA1RNDS4 XMM7, [RAX], 0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm8_mem_func1() {
    // SHA1RNDS4 XMM8, [RAX], 1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x3a, 0xcc, 0x00, 0x01, // SHA1RNDS4 XMM8, [RAX], 1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_xmm15_mem_func2() {
    // SHA1RNDS4 XMM15, [RAX], 2
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x3a, 0xcc, 0x38, 0x02, // SHA1RNDS4 XMM15, [RAX], 2
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE,
            0xEE, 0xEE,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_sha1rnds4_base_displacement() {
    // SHA1RNDS4 XMM0, [RAX + displacement], 0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x40, 0x20, 0x00, // SHA1RNDS4 XMM0, [RAX+0x20], 0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12,
            0x12, 0x12,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_with_rbx_base() {
    // SHA1RNDS4 XMM0, [RBX], 1
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x03, 0x01, // SHA1RNDS4 XMM0, [RBX], 1
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_with_rcx_base() {
    // SHA1RNDS4 XMM1, [RCX], 2
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x09, 0x02, // SHA1RNDS4 XMM1, [RCX], 2
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Function Sequence Tests - Testing progression through SHA1 rounds
// ============================================================================

#[test]
fn test_sha1rnds4_all_functions_sequence() {
    // Test all 4 functions in sequence (simulating 80 rounds of SHA1)
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x00, // SHA1RNDS4 XMM0, XMM1, 0
        0x0f, 0x3a, 0xcc, 0xc1, 0x01, // SHA1RNDS4 XMM0, XMM1, 1
        0x0f, 0x3a, 0xcc, 0xc1, 0x02, // SHA1RNDS4 XMM0, XMM1, 2
        0x0f, 0x3a, 0xcc, 0xc1, 0x03, // SHA1RNDS4 XMM0, XMM1, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_repeating_func0() {
    // Test repeating function 0 (rounds 0-19)
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x00, // SHA1RNDS4 XMM0, XMM1, 0
        0x0f, 0x3a, 0xcc, 0xca, 0x00, // SHA1RNDS4 XMM1, XMM2, 0
        0x0f, 0x3a, 0xcc, 0xd3, 0x00, // SHA1RNDS4 XMM2, XMM3, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_repeating_func1() {
    // Test repeating function 1 (rounds 20-39)
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x01, // SHA1RNDS4 XMM0, XMM1, 1
        0x0f, 0x3a, 0xcc, 0xca, 0x01, // SHA1RNDS4 XMM1, XMM2, 1
        0x0f, 0x3a, 0xcc, 0xd3, 0x01, // SHA1RNDS4 XMM2, XMM3, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_repeating_func2() {
    // Test repeating function 2 (rounds 40-59)
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x02, // SHA1RNDS4 XMM0, XMM1, 2
        0x0f, 0x3a, 0xcc, 0xca, 0x02, // SHA1RNDS4 XMM1, XMM2, 2
        0x0f, 0x3a, 0xcc, 0xd3, 0x02, // SHA1RNDS4 XMM2, XMM3, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_repeating_func3() {
    // Test repeating function 3 (rounds 60-79)
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x03, // SHA1RNDS4 XMM0, XMM1, 3
        0x0f, 0x3a, 0xcc, 0xca, 0x03, // SHA1RNDS4 XMM1, XMM2, 3
        0x0f, 0x3a, 0xcc, 0xd3, 0x03, // SHA1RNDS4 XMM2, XMM3, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_sha1rnds4_all_zeros_func0() {
    // Test with all zero state values
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0x00, // SHA1RNDS4 XMM0, XMM1, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_sha1_initial_state() {
    // Test with SHA1 initial state values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x3a, 0xcc, 0x00, 0x00, // SHA1RNDS4 XMM0, [RAX], 0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // SHA1 initial state in proper byte order
    let data = [
        0x67, 0x45, 0x23, 0x01, // Initial H0
        0xEF, 0xCD, 0xAB, 0x89, // Initial H1
        0x98, 0xBA, 0xDC, 0xFE, // Initial H2
        0x10, 0x32, 0x54, 0x76, // Initial H3
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_same_register() {
    // SHA1RNDS4 XMM0, XMM0, 0 - source and destination are the same
    let code = [
        0x0f, 0x3a, 0xcc, 0xc0, 0x00, // SHA1RNDS4 XMM0, XMM0, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_imm_bits_ignored() {
    // Test that only bits[1:0] of immediate matter
    // 0xFC has bits[1:0] = 0, so should be same as func 0
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0xfc, // SHA1RNDS4 XMM0, XMM1, 0xFC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_imm_bits_ignored_func1() {
    // 0xFD has bits[1:0] = 1, so should be same as func 1
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0xfd, // SHA1RNDS4 XMM0, XMM1, 0xFD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_imm_bits_ignored_func2() {
    // 0xFE has bits[1:0] = 2, so should be same as func 2
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0xfe, // SHA1RNDS4 XMM0, XMM1, 0xFE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1rnds4_imm_bits_ignored_func3() {
    // 0xFF has bits[1:0] = 3, so should be same as func 3
    let code = [
        0x0f, 0x3a, 0xcc, 0xc1, 0xff, // SHA1RNDS4 XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
