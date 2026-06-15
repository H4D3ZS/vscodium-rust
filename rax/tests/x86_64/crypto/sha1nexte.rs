use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// SHA1NEXTE - Calculate SHA1 State Variable E After Four Rounds
//
// The SHA1NEXTE calculates the SHA1 state variable E after four rounds of operation from
// the current SHA1 state variable A in the destination operand. The calculated value of
// the SHA1 state variable E is added to the source operand, which contains the scheduled dwords.
//
// Operation:
// TMP := (SRC1[127:96] ROL 30)
// DEST[127:96] := SRC2[127:96] + TMP
// DEST[95:64] := SRC2[95:64]
// DEST[63:32] := SRC2[63:32]
// DEST[31:0] := SRC2[31:0]
//
// Opcode:
// NP 0F 38 C8 /r    SHA1NEXTE xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_sha1nexte_xmm0_xmm1() {
    // SHA1NEXTE XMM0, XMM1
    let code = [
        0x0f, 0x38, 0xc8, 0xc1, // SHA1NEXTE XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm1_xmm0() {
    // SHA1NEXTE XMM1, XMM0
    let code = [
        0x0f, 0x38, 0xc8, 0xc8, // SHA1NEXTE XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm2_xmm3() {
    // SHA1NEXTE XMM2, XMM3
    let code = [
        0x0f, 0x38, 0xc8, 0xd3, // SHA1NEXTE XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm3_xmm2() {
    // SHA1NEXTE XMM3, XMM2
    let code = [
        0x0f, 0x38, 0xc8, 0xda, // SHA1NEXTE XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm4_xmm5() {
    // SHA1NEXTE XMM4, XMM5
    let code = [
        0x0f, 0x38, 0xc8, 0xe5, // SHA1NEXTE XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm5_xmm4() {
    // SHA1NEXTE XMM5, XMM4
    let code = [
        0x0f, 0x38, 0xc8, 0xec, // SHA1NEXTE XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm6_xmm7() {
    // SHA1NEXTE XMM6, XMM7
    let code = [
        0x0f, 0x38, 0xc8, 0xf7, // SHA1NEXTE XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm7_xmm6() {
    // SHA1NEXTE XMM7, XMM6
    let code = [
        0x0f, 0x38, 0xc8, 0xfe, // SHA1NEXTE XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm8_xmm9() {
    // SHA1NEXTE XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xc1, // SHA1NEXTE XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm9_xmm8() {
    // SHA1NEXTE XMM9, XMM8
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xc8, // SHA1NEXTE XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm10_xmm11() {
    // SHA1NEXTE XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xd3, // SHA1NEXTE XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm11_xmm10() {
    // SHA1NEXTE XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xda, // SHA1NEXTE XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm12_xmm13() {
    // SHA1NEXTE XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xe5, // SHA1NEXTE XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm13_xmm12() {
    // SHA1NEXTE XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xec, // SHA1NEXTE XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm14_xmm15() {
    // SHA1NEXTE XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xf7, // SHA1NEXTE XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_xmm15_xmm14() {
    // SHA1NEXTE XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x38, 0xc8, 0xfe, // SHA1NEXTE XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_sha1nexte_xmm0_mem() {
    // SHA1NEXTE XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
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
fn test_sha1nexte_xmm1_mem() {
    // SHA1NEXTE XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x08, // SHA1NEXTE XMM1, [RAX]
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
fn test_sha1nexte_xmm7_mem() {
    // SHA1NEXTE XMM7, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x38, // SHA1NEXTE XMM7, [RAX]
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
fn test_sha1nexte_xmm8_mem() {
    // SHA1NEXTE XMM8, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM8, [RAX]
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
fn test_sha1nexte_xmm15_mem() {
    // SHA1NEXTE XMM15, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xc8, 0x38, // SHA1NEXTE XMM15, [RAX]
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
fn test_sha1nexte_base_displacement() {
    // SHA1NEXTE XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x40, 0x20, // SHA1NEXTE XMM0, [RAX+0x20]
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
fn test_sha1nexte_with_rbx_base() {
    // SHA1NEXTE XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x03, // SHA1NEXTE XMM0, [RBX]
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
fn test_sha1nexte_with_rcx_base() {
    // SHA1NEXTE XMM1, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x09, // SHA1NEXTE XMM1, [RCX]
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
fn test_sha1nexte_with_rdx_base() {
    // SHA1NEXTE XMM2, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x12, // SHA1NEXTE XMM2, [RDX]
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
fn test_sha1nexte_with_rsi_base() {
    // SHA1NEXTE XMM3, [RSI]
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x1e, // SHA1NEXTE XMM3, [RSI]
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
fn test_sha1nexte_with_rdi_base() {
    // SHA1NEXTE XMM4, [RDI]
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x27, // SHA1NEXTE XMM4, [RDI]
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
fn test_sha1nexte_all_zeros() {
    // Test with all zero values
    let code = [
        0x0f, 0x38, 0xc8, 0xc1, // SHA1NEXTE XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_all_ones() {
    // Test with all ones pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
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
fn test_sha1nexte_sequential_values() {
    // Test with sequential dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
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

// ============================================================================
// ROL 30 Operation Tests - SHA1NEXTE includes ROL 30 on top dword
// ============================================================================

#[test]
fn test_sha1nexte_rotation_high_bits() {
    // Test rotation with high bits set
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Top dword has high bits that will rotate
    let data = [
        0x00, 0x00, 0x00, 0xC0, // Top bits set
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_rotation_low_bits() {
    // Test rotation with low bits set
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x03, 0x00, 0x00, 0x00, // Low bits that will rotate
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SHA1 State Variable Tests
// ============================================================================

#[test]
fn test_sha1nexte_sha1_state_a() {
    // Test with typical SHA1 state variable A value
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // SHA1 initial state values
    let data = [
        0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89, 0x98, 0xBA, 0xDC, 0xFE, 0x10, 0x32, 0x54,
        0x76,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_chain_operations() {
    // Test chaining multiple SHA1NEXTE operations
    let code = [
        0x0f, 0x38, 0xc8, 0xc1, // SHA1NEXTE XMM0, XMM1
        0x0f, 0x38, 0xc8, 0xca, // SHA1NEXTE XMM1, XMM2
        0x0f, 0x38, 0xc8, 0xd3, // SHA1NEXTE XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_same_register() {
    // SHA1NEXTE XMM0, XMM0 - source and destination are the same
    let code = [
        0x0f, 0x38, 0xc8, 0xc0, // SHA1NEXTE XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1nexte_alternating_pattern() {
    // Test with alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
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

#[test]
fn test_sha1nexte_high_values() {
    // Test with high dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xc8, 0x00, // SHA1NEXTE XMM0, [RAX]
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
