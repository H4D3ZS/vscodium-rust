use crate::common::*;

// UNPCKLPD - Unpack and Interleave Low Packed Double Precision Floating-Point Values
//
// Performs an interleaved unpack of the low double precision floating-point values
// from the first source operand and the second source operand.
//
// Operation:
// DEST[63:0]   := SRC1[63:0]
// DEST[127:64] := SRC2[63:0]
//
// Opcode: 66 0F 14 /r    UNPCKLPD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_unpcklpd_xmm0_xmm1() {
    // UNPCKLPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x14, 0xc1, // UNPCKLPD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm1_xmm2() {
    // UNPCKLPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x14, 0xca, // UNPCKLPD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm2_xmm3() {
    // UNPCKLPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x14, 0xd3, // UNPCKLPD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm3_xmm4() {
    // UNPCKLPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x14, 0xdc, // UNPCKLPD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm4_xmm5() {
    // UNPCKLPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x14, 0xe5, // UNPCKLPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm5_xmm6() {
    // UNPCKLPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x14, 0xee, // UNPCKLPD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm6_xmm7() {
    // UNPCKLPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x14, 0xf7, // UNPCKLPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm7_xmm0() {
    // UNPCKLPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x14, 0xf8, // UNPCKLPD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_unpcklpd_xmm8_xmm9() {
    // UNPCKLPD XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x14, 0xc1, // UNPCKLPD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm10_xmm11() {
    // UNPCKLPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x14, 0xd3, // UNPCKLPD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm12_xmm13() {
    // UNPCKLPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x14, 0xe5, // UNPCKLPD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm14_xmm15() {
    // UNPCKLPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x14, 0xf7, // UNPCKLPD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm0_xmm8() {
    // UNPCKLPD XMM0, XMM8
    let code = [
        0x66, 0x44, 0x0f, 0x14, 0xc0, // UNPCKLPD XMM0, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm15_xmm0() {
    // UNPCKLPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x14, 0xf8, // UNPCKLPD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm8_xmm0() {
    // UNPCKLPD XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x14, 0xc0, // UNPCKLPD XMM8, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm0_xmm15() {
    // UNPCKLPD XMM0, XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x14, 0xc7, // UNPCKLPD XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_unpcklpd_xmm0_mem() {
    // UNPCKLPD XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x00, // UNPCKLPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (2 doubles)
    let double_data: [f64; 2] = [1.0, 2.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm1_mem() {
    // UNPCKLPD XMM1, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x08, // UNPCKLPD XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let double_data: [f64; 2] = [3.0, 4.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm2_mem() {
    // UNPCKLPD XMM2, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x10, // UNPCKLPD XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm7_mem() {
    // UNPCKLPD XMM7, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x38, // UNPCKLPD XMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    mem.write_slice(
        &[
            0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
            0x55, 0x55,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing mode tests
// ============================================================================

#[test]
fn test_unpcklpd_xmm0_mem_displacement() {
    // UNPCKLPD XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x40, 0x10, // UNPCKLPD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm1_mem_rbx() {
    // UNPCKLPD XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x0b, // UNPCKLPD XMM1, [RBX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm2_mem_rcx() {
    // UNPCKLPD XMM2, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x11, // UNPCKLPD XMM2, [RCX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
            0x99, 0x99,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_xmm3_mem_rdx() {
    // UNPCKLPD XMM3, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x1a, // UNPCKLPD XMM3, [RDX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB,
            0xBB, 0xBB,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data pattern tests
// ============================================================================

#[test]
fn test_unpcklpd_all_zeros() {
    // Test with all zero data
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x00, // UNPCKLPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_all_ones() {
    // Test with all ones data
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x00, // UNPCKLPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_alternating_pattern() {
    // Test with alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x00, // UNPCKLPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
            0xAA, 0x55,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_sequential_bytes() {
    // Test with sequential byte pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x14, 0x00, // UNPCKLPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Sequential operations tests
// ============================================================================

#[test]
fn test_unpcklpd_sequential_operations() {
    // Multiple UNPCKLPD operations in sequence
    let code = [
        0x66, 0x0f, 0x14, 0xc1, // UNPCKLPD XMM0, XMM1
        0x66, 0x0f, 0x14, 0xd3, // UNPCKLPD XMM2, XMM3
        0x66, 0x0f, 0x14, 0xe5, // UNPCKLPD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_same_register() {
    // UNPCKLPD XMM0, XMM0 (unpack with itself)
    let code = [
        0x66, 0x0f, 0x14, 0xc0, // UNPCKLPD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_chain_operations() {
    // Chain of UNPCKLPD operations
    let code = [
        0x66, 0x0f, 0x14, 0xc1, // UNPCKLPD XMM0, XMM1
        0x66, 0x0f, 0x14, 0xc2, // UNPCKLPD XMM0, XMM2
        0x66, 0x0f, 0x14, 0xc3, // UNPCKLPD XMM0, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_all_register_pairs() {
    // Test various register pair combinations
    let code = [
        0x66, 0x0f, 0x14, 0xc1, // UNPCKLPD XMM0, XMM1
        0x66, 0x0f, 0x14, 0xd3, // UNPCKLPD XMM2, XMM3
        0x66, 0x0f, 0x14, 0xe5, // UNPCKLPD XMM4, XMM5
        0x66, 0x0f, 0x14, 0xf7, // UNPCKLPD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_unpcklpd_reverse_pairs() {
    // Test reverse register pair combinations
    let code = [
        0x66, 0x0f, 0x14, 0xc8, // UNPCKLPD XMM1, XMM0
        0x66, 0x0f, 0x14, 0xda, // UNPCKLPD XMM3, XMM2
        0x66, 0x0f, 0x14, 0xec, // UNPCKLPD XMM5, XMM4
        0x66, 0x0f, 0x14, 0xfe, // UNPCKLPD XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
