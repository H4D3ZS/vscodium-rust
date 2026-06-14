use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// SHA256RNDS2 - Perform Two Rounds of SHA256 Operation
//
// The SHA256RNDS2 instruction performs 2 rounds of SHA256 operation using an initial SHA256
// state (C,D,G,H) from the first operand, an initial SHA256 state (A,B,E,F) from the second
// operand, and a pre-computed sum of the next 2 round message dwords and the corresponding
// round constants from the implicit operand XMM0. Note that only the two lower dwords of XMM0
// are used by the instruction.
//
// The updated SHA256 state (A,B,E,F) is written to the first operand, and the second operand
// can be used as the updated state (C,D,G,H) in later rounds.
//
// Opcode:
// NP 0F 38 CB /r    SHA256RNDS2 xmm1, xmm2/m128, <XMM0>

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests (XMM0 is implicit source)
// ============================================================================

#[test]
fn test_sha256rnds2_xmm1_xmm2() {
    // SHA256RNDS2 XMM1, XMM2 (XMM0 implicit)
    let code = [
        0x0f, 0x38, 0xcb, 0xca, // SHA256RNDS2 XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm2_xmm3() {
    // SHA256RNDS2 XMM2, XMM3
    let code = [
        0x0f, 0x38, 0xcb, 0xd3, // SHA256RNDS2 XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm3_xmm4() {
    // SHA256RNDS2 XMM3, XMM4
    let code = [
        0x0f, 0x38, 0xcb, 0xdc, // SHA256RNDS2 XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm4_xmm5() {
    // SHA256RNDS2 XMM4, XMM5
    let code = [
        0x0f, 0x38, 0xcb, 0xe5, // SHA256RNDS2 XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm5_xmm6() {
    // SHA256RNDS2 XMM5, XMM6
    let code = [
        0x0f, 0x38, 0xcb, 0xee, // SHA256RNDS2 XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm6_xmm7() {
    // SHA256RNDS2 XMM6, XMM7
    let code = [
        0x0f, 0x38, 0xcb, 0xf7, // SHA256RNDS2 XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm7_xmm8() {
    // SHA256RNDS2 XMM7, XMM8
    let code = [
        0x41, 0x0f, 0x38, 0xcb, 0xf8, // SHA256RNDS2 XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm8_xmm9() {
    // SHA256RNDS2 XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xc1, // SHA256RNDS2 XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm9_xmm10() {
    // SHA256RNDS2 XMM9, XMM10
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xca, // SHA256RNDS2 XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm10_xmm11() {
    // SHA256RNDS2 XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xd3, // SHA256RNDS2 XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm11_xmm12() {
    // SHA256RNDS2 XMM11, XMM12
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xdc, // SHA256RNDS2 XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm12_xmm13() {
    // SHA256RNDS2 XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xe5, // SHA256RNDS2 XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm13_xmm14() {
    // SHA256RNDS2 XMM13, XMM14
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xee, // SHA256RNDS2 XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm14_xmm15() {
    // SHA256RNDS2 XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xf7, // SHA256RNDS2 XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_xmm15_xmm14() {
    // SHA256RNDS2 XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x38, 0xcb, 0xfe, // SHA256RNDS2 XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_sha256rnds2_xmm1_mem() {
    // SHA256RNDS2 XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
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
fn test_sha256rnds2_xmm2_mem() {
    // SHA256RNDS2 XMM2, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x10, // SHA256RNDS2 XMM2, [RAX]
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
fn test_sha256rnds2_xmm7_mem() {
    // SHA256RNDS2 XMM7, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x38, // SHA256RNDS2 XMM7, [RAX]
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
fn test_sha256rnds2_xmm8_mem() {
    // SHA256RNDS2 XMM8, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xcb, 0x00, // SHA256RNDS2 XMM8, [RAX]
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
fn test_sha256rnds2_xmm15_mem() {
    // SHA256RNDS2 XMM15, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xcb, 0x38, // SHA256RNDS2 XMM15, [RAX]
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
fn test_sha256rnds2_base_displacement() {
    // SHA256RNDS2 XMM1, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x48, 0x20, // SHA256RNDS2 XMM1, [RAX+0x20]
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
fn test_sha256rnds2_with_rbx_base() {
    // SHA256RNDS2 XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x0b, // SHA256RNDS2 XMM1, [RBX]
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
fn test_sha256rnds2_with_rcx_base() {
    // SHA256RNDS2 XMM2, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x11, // SHA256RNDS2 XMM2, [RCX]
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
fn test_sha256rnds2_with_rdx_base() {
    // SHA256RNDS2 XMM3, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x1a, // SHA256RNDS2 XMM3, [RDX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
            0x99, 0x99,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_with_rsi_base() {
    // SHA256RNDS2 XMM4, [RSI]
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x26, // SHA256RNDS2 XMM4, [RSI]
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
fn test_sha256rnds2_with_rdi_base() {
    // SHA256RNDS2 XMM5, [RDI]
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x2f, // SHA256RNDS2 XMM5, [RDI]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB,
            0xBB, 0xBB,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_sha256rnds2_all_zeros() {
    // Test with all zero values (XMM0, operands)
    let code = [
        0x0f, 0x38, 0xcb, 0xca, // SHA256RNDS2 XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_all_ones() {
    // Test with all ones pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
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
fn test_sha256rnds2_sequential_values() {
    // Test with sequential dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_alternating_pattern() {
    // Test with alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
            0xAA, 0x55,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SHA256 Context Tests
// ============================================================================

#[test]
fn test_sha256rnds2_sha256_initial_state() {
    // Test with SHA256 initial state values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // SHA256 initial H values
    let data = [
        0x67, 0xE6, 0x09, 0x6A, // H0
        0x85, 0xAE, 0x67, 0xBB, // H1
        0x72, 0xF3, 0x6E, 0x3C, // H2
        0x3A, 0xF5, 0x4F, 0xA5, // H3
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_chain_operations() {
    // Test chaining multiple SHA256RNDS2 operations
    let code = [
        0x0f, 0x38, 0xcb, 0xca, // SHA256RNDS2 XMM1, XMM2
        0x0f, 0x38, 0xcb, 0xd3, // SHA256RNDS2 XMM2, XMM3
        0x0f, 0x38, 0xcb, 0xdc, // SHA256RNDS2 XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_same_register() {
    // SHA256RNDS2 XMM1, XMM1 - source and destination are the same
    let code = [
        0x0f, 0x38, 0xcb, 0xc9, // SHA256RNDS2 XMM1, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_high_values() {
    // Test with high dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_sparse_pattern() {
    // Test with sparse bit patterns
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcb, 0x08, // SHA256RNDS2 XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256rnds2_with_xmm0_constant() {
    // Test with XMM0 holding round constants (typical usage)
    let code = [
        0x0f, 0x38, 0xcb, 0xca, // SHA256RNDS2 XMM1, XMM2 (uses XMM0[63:0])
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
