use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// SHA256MSG1 - Perform an Intermediate Calculation for the Next Four SHA256 Message Dwords
//
// The SHA256MSG1 instruction is one of two SHA256 message scheduling instructions.
// The instruction performs an intermediate calculation for the next four SHA256 message dwords.
//
// Operation:
// W4 := SRC2[31:0]
// W3 := SRC1[127:96]
// W2 := SRC1[95:64]
// W1 := SRC1[63:32]
// W0 := SRC1[31:0]
// DEST[127:96] := W3 + σ0(W4)
// DEST[95:64] := W2 + σ0(W3)
// DEST[63:32] := W1 + σ0(W2)
// DEST[31:0] := W0 + σ0(W1)
//
// Where σ0(x) = (x ROR 7) XOR (x ROR 18) XOR (x SHR 3)
//
// Opcode:
// NP 0F 38 CC /r    SHA256MSG1 xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_sha256msg1_xmm0_xmm1() {
    // SHA256MSG1 XMM0, XMM1
    let code = [
        0x0f, 0x38, 0xcc, 0xc1, // SHA256MSG1 XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm1_xmm0() {
    // SHA256MSG1 XMM1, XMM0
    let code = [
        0x0f, 0x38, 0xcc, 0xc8, // SHA256MSG1 XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm2_xmm3() {
    // SHA256MSG1 XMM2, XMM3
    let code = [
        0x0f, 0x38, 0xcc, 0xd3, // SHA256MSG1 XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm3_xmm2() {
    // SHA256MSG1 XMM3, XMM2
    let code = [
        0x0f, 0x38, 0xcc, 0xda, // SHA256MSG1 XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm4_xmm5() {
    // SHA256MSG1 XMM4, XMM5
    let code = [
        0x0f, 0x38, 0xcc, 0xe5, // SHA256MSG1 XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm5_xmm4() {
    // SHA256MSG1 XMM5, XMM4
    let code = [
        0x0f, 0x38, 0xcc, 0xec, // SHA256MSG1 XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm6_xmm7() {
    // SHA256MSG1 XMM6, XMM7
    let code = [
        0x0f, 0x38, 0xcc, 0xf7, // SHA256MSG1 XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm7_xmm6() {
    // SHA256MSG1 XMM7, XMM6
    let code = [
        0x0f, 0x38, 0xcc, 0xfe, // SHA256MSG1 XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm8_xmm9() {
    // SHA256MSG1 XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xc1, // SHA256MSG1 XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm9_xmm8() {
    // SHA256MSG1 XMM9, XMM8
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xc8, // SHA256MSG1 XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm10_xmm11() {
    // SHA256MSG1 XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xd3, // SHA256MSG1 XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm11_xmm10() {
    // SHA256MSG1 XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xda, // SHA256MSG1 XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm12_xmm13() {
    // SHA256MSG1 XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xe5, // SHA256MSG1 XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm13_xmm12() {
    // SHA256MSG1 XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xec, // SHA256MSG1 XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm14_xmm15() {
    // SHA256MSG1 XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xf7, // SHA256MSG1 XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_xmm15_xmm14() {
    // SHA256MSG1 XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x38, 0xcc, 0xfe, // SHA256MSG1 XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_sha256msg1_xmm0_mem() {
    // SHA256MSG1 XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
fn test_sha256msg1_xmm1_mem() {
    // SHA256MSG1 XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x08, // SHA256MSG1 XMM1, [RAX]
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
fn test_sha256msg1_xmm7_mem() {
    // SHA256MSG1 XMM7, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x38, // SHA256MSG1 XMM7, [RAX]
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
fn test_sha256msg1_xmm8_mem() {
    // SHA256MSG1 XMM8, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM8, [RAX]
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
fn test_sha256msg1_xmm15_mem() {
    // SHA256MSG1 XMM15, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xcc, 0x38, // SHA256MSG1 XMM15, [RAX]
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
fn test_sha256msg1_base_displacement() {
    // SHA256MSG1 XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x40, 0x20, // SHA256MSG1 XMM0, [RAX+0x20]
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
fn test_sha256msg1_with_rbx_base() {
    // SHA256MSG1 XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x03, // SHA256MSG1 XMM0, [RBX]
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
fn test_sha256msg1_with_rcx_base() {
    // SHA256MSG1 XMM1, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x09, // SHA256MSG1 XMM1, [RCX]
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
fn test_sha256msg1_with_rdx_base() {
    // SHA256MSG1 XMM2, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x12, // SHA256MSG1 XMM2, [RDX]
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
fn test_sha256msg1_with_rsi_base() {
    // SHA256MSG1 XMM3, [RSI]
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x1e, // SHA256MSG1 XMM3, [RSI]
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
fn test_sha256msg1_with_rdi_base() {
    // SHA256MSG1 XMM4, [RDI]
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x27, // SHA256MSG1 XMM4, [RDI]
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
fn test_sha256msg1_all_zeros() {
    // Test with all zero values
    let code = [
        0x0f, 0x38, 0xcc, 0xc1, // SHA256MSG1 XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_all_ones() {
    // Test with all ones pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
fn test_sha256msg1_sequential_values() {
    // Test with sequential dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
fn test_sha256msg1_alternating_pattern() {
    // Test with alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
fn test_sha256msg1_sha256_context_1() {
    // Test with typical SHA256 message schedule values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // SHA256 initial state values (H0-H3 in little-endian)
    let data = [
        0x67, 0xE6, 0x09, 0x6A, 0x85, 0xAE, 0x67, 0xBB, 0x72, 0xF3, 0x6E, 0x3C, 0x3A, 0xF5, 0x4F,
        0xA5,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_chain_operations() {
    // Test chaining multiple SHA256MSG1 operations
    let code = [
        0x0f, 0x38, 0xcc, 0xc1, // SHA256MSG1 XMM0, XMM1
        0x0f, 0x38, 0xcc, 0xca, // SHA256MSG1 XMM1, XMM2
        0x0f, 0x38, 0xcc, 0xd3, // SHA256MSG1 XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_same_register() {
    // SHA256MSG1 XMM0, XMM0 - source and destination are the same
    let code = [
        0x0f, 0x38, 0xcc, 0xc0, // SHA256MSG1 XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha256msg1_high_values() {
    // Test with high dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
fn test_sha256msg1_sparse_pattern() {
    // Test with sparse bit patterns
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xcc, 0x00, // SHA256MSG1 XMM0, [RAX]
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
