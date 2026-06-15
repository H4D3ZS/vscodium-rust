use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// SHA1MSG2 - Perform a Final Calculation for the Next Four SHA1 Message Dwords
//
// The SHA1MSG2 instruction is one of two SHA1 message scheduling instructions.
// The instruction performs the final calculation to derive the next four SHA1 message dwords.
//
// Operation:
// W13 := SRC2[95:64]
// W14 := SRC2[63:32]
// W15 := SRC2[31:0]
// W16 := (SRC1[127:96] XOR W13) ROL 1
// W17 := (SRC1[95:64] XOR W14) ROL 1
// W18 := (SRC1[63:32] XOR W15) ROL 1
// W19 := (SRC1[31:0] XOR W16) ROL 1
// DEST[127:96] := W16
// DEST[95:64] := W17
// DEST[63:32] := W18
// DEST[31:0] := W19
//
// Opcode:
// NP 0F 38 CA /r    SHA1MSG2 xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_sha1msg2_xmm0_xmm1() {
    // SHA1MSG2 XMM0, XMM1
    let code = [
        0x0f, 0x38, 0xca, 0xc1, // SHA1MSG2 XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm1_xmm0() {
    // SHA1MSG2 XMM1, XMM0
    let code = [
        0x0f, 0x38, 0xca, 0xc8, // SHA1MSG2 XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm2_xmm3() {
    // SHA1MSG2 XMM2, XMM3
    let code = [
        0x0f, 0x38, 0xca, 0xd3, // SHA1MSG2 XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm3_xmm2() {
    // SHA1MSG2 XMM3, XMM2
    let code = [
        0x0f, 0x38, 0xca, 0xda, // SHA1MSG2 XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm4_xmm5() {
    // SHA1MSG2 XMM4, XMM5
    let code = [
        0x0f, 0x38, 0xca, 0xe5, // SHA1MSG2 XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm5_xmm4() {
    // SHA1MSG2 XMM5, XMM4
    let code = [
        0x0f, 0x38, 0xca, 0xec, // SHA1MSG2 XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm6_xmm7() {
    // SHA1MSG2 XMM6, XMM7
    let code = [
        0x0f, 0x38, 0xca, 0xf7, // SHA1MSG2 XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm7_xmm6() {
    // SHA1MSG2 XMM7, XMM6
    let code = [
        0x0f, 0x38, 0xca, 0xfe, // SHA1MSG2 XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm8_xmm9() {
    // SHA1MSG2 XMM8, XMM9 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xc1, // SHA1MSG2 XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm9_xmm8() {
    // SHA1MSG2 XMM9, XMM8
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xc8, // SHA1MSG2 XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm10_xmm11() {
    // SHA1MSG2 XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xd3, // SHA1MSG2 XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm11_xmm10() {
    // SHA1MSG2 XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xda, // SHA1MSG2 XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm12_xmm13() {
    // SHA1MSG2 XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xe5, // SHA1MSG2 XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm13_xmm12() {
    // SHA1MSG2 XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xec, // SHA1MSG2 XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm14_xmm15() {
    // SHA1MSG2 XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xf7, // SHA1MSG2 XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_xmm15_xmm14() {
    // SHA1MSG2 XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x38, 0xca, 0xfe, // SHA1MSG2 XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_sha1msg2_xmm0_mem() {
    // SHA1MSG2 XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
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
fn test_sha1msg2_xmm1_mem() {
    // SHA1MSG2 XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x08, // SHA1MSG2 XMM1, [RAX]
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
fn test_sha1msg2_xmm7_mem() {
    // SHA1MSG2 XMM7, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x38, // SHA1MSG2 XMM7, [RAX]
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
fn test_sha1msg2_xmm8_mem() {
    // SHA1MSG2 XMM8, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM8, [RAX]
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
fn test_sha1msg2_xmm15_mem() {
    // SHA1MSG2 XMM15, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x38, 0xca, 0x38, // SHA1MSG2 XMM15, [RAX]
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
fn test_sha1msg2_base_displacement() {
    // SHA1MSG2 XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x40, 0x20, // SHA1MSG2 XMM0, [RAX+0x20]
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
fn test_sha1msg2_with_rbx_base() {
    // SHA1MSG2 XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x03, // SHA1MSG2 XMM0, [RBX]
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
fn test_sha1msg2_with_rcx_base() {
    // SHA1MSG2 XMM1, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x09, // SHA1MSG2 XMM1, [RCX]
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
fn test_sha1msg2_with_rdx_base() {
    // SHA1MSG2 XMM2, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x12, // SHA1MSG2 XMM2, [RDX]
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
fn test_sha1msg2_with_rsi_base() {
    // SHA1MSG2 XMM3, [RSI]
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x1e, // SHA1MSG2 XMM3, [RSI]
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
fn test_sha1msg2_with_rdi_base() {
    // SHA1MSG2 XMM4, [RDI]
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x27, // SHA1MSG2 XMM4, [RDI]
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
fn test_sha1msg2_all_zeros() {
    // Test with all zero values
    let code = [
        0x0f, 0x38, 0xca, 0xc1, // SHA1MSG2 XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_all_ones() {
    // Test with all ones pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
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
fn test_sha1msg2_sequential_values() {
    // Test with sequential dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Write sequential dwords
    let data = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_alternating_pattern() {
    // Test with alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
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
// Rotation Tests - SHA1MSG2 includes ROL 1 operation
// ============================================================================

#[test]
fn test_sha1msg2_rotation_high_bit() {
    // Test rotation with high bit set (should rotate to bit 0)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // High bit set in each dword
    let data = [
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_rotation_low_bit() {
    // Test rotation with only low bit set
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SHA1 Context Tests
// ============================================================================

#[test]
fn test_sha1msg2_sha1_context_1() {
    // Test with typical SHA1 message schedule values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Typical SHA1 message dwords
    let data = [
        0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89, 0x98, 0xBA, 0xDC, 0xFE, 0x10, 0x32, 0x54,
        0x76,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_chain_operations() {
    // Test chaining multiple SHA1MSG2 operations
    let code = [
        0x0f, 0x38, 0xca, 0xc1, // SHA1MSG2 XMM0, XMM1
        0x0f, 0x38, 0xca, 0xca, // SHA1MSG2 XMM1, XMM2
        0x0f, 0x38, 0xca, 0xd3, // SHA1MSG2 XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_same_register() {
    // SHA1MSG2 XMM0, XMM0 - source and destination are the same
    let code = [
        0x0f, 0x38, 0xca, 0xc0, // SHA1MSG2 XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_msg1_msg2_sequence() {
    // Test SHA1MSG1 followed by SHA1MSG2 (typical usage pattern)
    let code = [
        0x0f, 0x38, 0xc9, 0xc1, // SHA1MSG1 XMM0, XMM1
        0x0f, 0x38, 0xca, 0xc2, // SHA1MSG2 XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_high_values() {
    // Test with high dword values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0x7F, // Max positive int32
        0xFF, 0xFF, 0xFF, 0xFF, // Max uint32
        0x00, 0x00, 0x00, 0x80, // Min negative int32
        0x00, 0x00, 0x00, 0x00, // Zero
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sha1msg2_sparse_pattern() {
    // Test with sparse bit patterns
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x38, 0xca, 0x00, // SHA1MSG2 XMM0, [RAX]
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
