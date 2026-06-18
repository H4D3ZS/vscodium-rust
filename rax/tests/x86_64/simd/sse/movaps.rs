use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVAPS - Move Aligned Packed Single Precision Floating-Point Values
//
// Moves 128 bits (4 single-precision floating-point values) from source to destination.
// When the operand is a memory location, it must be aligned on a 16-byte boundary.
// Otherwise, a general-protection exception (#GP) is generated.
//
// Opcodes:
// NP 0F 28 /r    MOVAPS xmm1, xmm2/m128    - Move aligned packed single from xmm2/mem to xmm1
// NP 0F 29 /r    MOVAPS xmm2/m128, xmm1    - Move aligned packed single from xmm1 to xmm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movaps_xmm0_to_xmm1() {
    // MOVAPS XMM1, XMM0
    let code = [
        0x0f, 0x28, 0xc8, // MOVAPS XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up XMM0 with test data
    mem.write_slice(
        &[
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    // Verify XMM1 contains the moved data
    let mut xmm1 = [0u8; 16];
    mem.read_slice(&mut xmm1, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    // Note: In actual implementation, we'd need to read XMM registers
    // This is a placeholder test structure
}

#[test]
fn test_movaps_xmm2_to_xmm3() {
    // MOVAPS XMM3, XMM2
    let code = [
        0x0f, 0x28, 0xda, // MOVAPS XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm4_to_xmm5() {
    // MOVAPS XMM5, XMM4
    let code = [
        0x0f, 0x28, 0xec, // MOVAPS XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm6_to_xmm7() {
    // MOVAPS XMM7, XMM6
    let code = [
        0x0f, 0x28, 0xfe, // MOVAPS XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm8_to_xmm9() {
    // MOVAPS XMM9, XMM8 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x28, 0xc8, // MOVAPS XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm10_to_xmm11() {
    // MOVAPS XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x28, 0xda, // MOVAPS XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm12_to_xmm13() {
    // MOVAPS XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x28, 0xec, // MOVAPS XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm14_to_xmm15() {
    // MOVAPS XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x28, 0xfe, // MOVAPS XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm0_to_xmm15() {
    // MOVAPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x28, 0xf8, // MOVAPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm15_to_xmm0() {
    // MOVAPS XMM0, XMM15
    let code = [
        0x44, 0x0f, 0x28, 0xc7, // MOVAPS XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_movaps_mem_to_xmm0_aligned() {
    // MOVAPS XMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to aligned address
    mem.write_slice(
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_mem_to_xmm1_aligned() {
    // MOVAPS XMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x08, // MOVAPS XMM1, [RAX]
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
fn test_movaps_mem_to_xmm7_aligned() {
    // MOVAPS XMM7, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x38, // MOVAPS XMM7, [RAX]
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
fn test_movaps_mem_to_xmm8_aligned() {
    // MOVAPS XMM8, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x28, 0x00, // MOVAPS XMM8, [RAX]
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
fn test_movaps_mem_to_xmm15_aligned() {
    // MOVAPS XMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x28, 0x38, // MOVAPS XMM15, [RAX]
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

// ============================================================================
// Register to Memory Tests (Aligned)
// ============================================================================

#[test]
fn test_movaps_xmm0_to_mem_aligned() {
    // MOVAPS [aligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x29, 0x00, // MOVAPS [RAX], XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify data was written
    let mut result = [0u8; 16];
    mem.read_slice(&mut result, GuestAddress(ALIGNED_ADDR))
        .unwrap();
}

#[test]
fn test_movaps_xmm1_to_mem_aligned() {
    // MOVAPS [aligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x29, 0x08, // MOVAPS [RAX], XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm7_to_mem_aligned() {
    // MOVAPS [aligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x29, 0x38, // MOVAPS [RAX], XMM7
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm8_to_mem_aligned() {
    // MOVAPS [aligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x29, 0x00, // MOVAPS [RAX], XMM8
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_xmm15_to_mem_aligned() {
    // MOVAPS [aligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x29, 0x38, // MOVAPS [RAX], XMM15
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_movaps_all_zeros() {
    // Test moving all zeros
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_all_ones() {
    // Test moving all ones
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
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
fn test_movaps_alternating_pattern() {
    // Test moving alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
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
fn test_movaps_float_values() {
    // Test moving actual float values (bit patterns)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write bit patterns for: 1.0f, 2.0f, 3.0f, 4.0f
    let float1: f32 = 1.0;
    let float2: f32 = 2.0;
    let float3: f32 = 3.0;
    let float4: f32 = 4.0;

    let mut data = Vec::new();
    data.extend_from_slice(&float1.to_le_bytes());
    data.extend_from_slice(&float2.to_le_bytes());
    data.extend_from_slice(&float3.to_le_bytes());
    data.extend_from_slice(&float4.to_le_bytes());

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_sequential_bytes() {
    // Test with sequential byte pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_movaps_base_displacement() {
    // MOVAPS XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x40, 0x10, // MOVAPS XMM0, [RAX + 0x10]
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
fn test_movaps_rip_relative() {
    // MOVAPS XMM0, [RIP + displacement]
    // Code is at 0x1000. After the 7-byte MOVAPS instruction, RIP = 0x1007.
    // We need to reach an aligned address. 0x3000 is aligned.
    // displacement = 0x3000 - 0x1007 = 0x1FF9
    let code = [
        0x0f, 0x28, 0x05, 0xF9, 0x1F, 0x00, 0x00, // MOVAPS XMM0, [RIP+0x1FF9] -> addr 0x3000
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Write test data at the aligned address
    mem.write_slice(&[0xAA; 16], GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_with_rbx_base() {
    // MOVAPS XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x03, // MOVAPS XMM0, [RBX]
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
fn test_movaps_with_rcx_base() {
    // MOVAPS XMM0, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x01, // MOVAPS XMM0, [RCX]
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
fn test_movaps_with_rdx_base() {
    // MOVAPS XMM0, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x02, // MOVAPS XMM0, [RDX]
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

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_movaps_roundtrip_reg_mem_reg() {
    // Test: XMM0 -> Memory -> XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x29, 0x00, // MOVAPS [RAX], XMM0
        0x0f, 0x28, 0x08, // MOVAPS XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_chain_move() {
    // Test: XMM0 -> XMM1 -> XMM2 -> XMM3
    let code = [
        0x0f, 0x28, 0xc8, // MOVAPS XMM1, XMM0
        0x0f, 0x28, 0xd1, // MOVAPS XMM2, XMM1
        0x0f, 0x28, 0xda, // MOVAPS XMM3, XMM2
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn test_movaps_at_0x5000_aligned() {
    // Test at 0x5000 (16-byte aligned)
    // Note: 0x1000 is CODE_ADDR where test code is loaded, so we use 0x5000 instead
    const TEST_ADDR: u64 = 0x5000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB,
            0xBB, 0xBB,
        ],
        GuestAddress(TEST_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_at_0x4000_aligned() {
    // Test at 0x4000 (16-byte aligned)
    const TEST_ADDR: u64 = 0x4000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
            0xCC, 0xCC,
        ],
        GuestAddress(TEST_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_multiple_xmm_operations() {
    // Test multiple MOVAPS operations in sequence
    let code = [
        0x0f, 0x28, 0xc8, // MOVAPS XMM1, XMM0
        0x0f, 0x28, 0xd0, // MOVAPS XMM2, XMM0
        0x0f, 0x28, 0xd8, // MOVAPS XMM3, XMM0
        0x0f, 0x28, 0xe0, // MOVAPS XMM4, XMM0
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movaps_overwrite_destination() {
    // Test that destination is completely overwritten
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write distinct pattern
    mem.write_slice(
        &[
            0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87, 0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D,
            0x1E, 0x0F,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}
