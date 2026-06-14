use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVUPS - Move Unaligned Packed Single Precision Floating-Point Values
//
// Moves 128 bits (4 single-precision floating-point values) from source to destination.
// Unlike MOVAPS, this instruction does NOT require 16-byte alignment.
// Can move data to/from unaligned memory addresses without causing #GP.
//
// Opcodes:
// NP 0F 10 /r    MOVUPS xmm1, xmm2/m128    - Move unaligned packed single from xmm2/mem to xmm1
// NP 0F 11 /r    MOVUPS xmm2/m128, xmm1    - Move unaligned packed single from xmm1 to xmm2/mem

const UNALIGNED_ADDR: u64 = 0x3001; // Intentionally unaligned (offset by 1)
const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned (MOVUPS works with aligned too)

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movups_xmm0_to_xmm1() {
    // MOVUPS XMM1, XMM0
    let code = [
        0x0f, 0x10, 0xc8, // MOVUPS XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm2_to_xmm3() {
    // MOVUPS XMM3, XMM2
    let code = [
        0x0f, 0x10, 0xda, // MOVUPS XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm4_to_xmm5() {
    // MOVUPS XMM5, XMM4
    let code = [
        0x0f, 0x10, 0xec, // MOVUPS XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm6_to_xmm7() {
    // MOVUPS XMM7, XMM6
    let code = [
        0x0f, 0x10, 0xfe, // MOVUPS XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm8_to_xmm9() {
    // MOVUPS XMM9, XMM8 (requires REX prefix)
    let code = [
        0x45, 0x0f, 0x10, 0xc8, // MOVUPS XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm10_to_xmm11() {
    // MOVUPS XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x10, 0xda, // MOVUPS XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm12_to_xmm13() {
    // MOVUPS XMM13, XMM12
    let code = [
        0x45, 0x0f, 0x10, 0xec, // MOVUPS XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm14_to_xmm15() {
    // MOVUPS XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x10, 0xfe, // MOVUPS XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm0_to_xmm15() {
    // MOVUPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x10, 0xf8, // MOVUPS XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm15_to_xmm0() {
    // MOVUPS XMM0, XMM15
    let code = [
        0x44, 0x0f, 0x10, 0xc7, // MOVUPS XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Unaligned Memory to Register Tests
// ============================================================================

#[test]
fn test_movups_unaligned_mem_to_xmm0() {
    // MOVUPS XMM0, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to unaligned address
    mem.write_slice(
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_unaligned_offset_1() {
    // Test with 1-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(ALIGNED_ADDR + 1),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_unaligned_offset_2() {
    // Test with 2-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        GuestAddress(ALIGNED_ADDR + 2),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_unaligned_offset_3() {
    // Test with 3-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 3).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
            0x55, 0x55,
        ],
        GuestAddress(ALIGNED_ADDR + 3),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_unaligned_offset_7() {
    // Test with 7-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 7).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
            0x33, 0x33,
        ],
        GuestAddress(ALIGNED_ADDR + 7),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_unaligned_offset_15() {
    // Test with 15-byte offset from alignment (worst case)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 15).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        GuestAddress(ALIGNED_ADDR + 15),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_aligned_mem_to_xmm1() {
    // MOVUPS also works with aligned memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x08, // MOVUPS XMM1, [RAX]
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

#[test]
fn test_movups_mem_to_xmm8_unaligned() {
    // MOVUPS XMM8, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x10, 0x00, // MOVUPS XMM8, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
            0xCC, 0xCC,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_mem_to_xmm15_unaligned() {
    // MOVUPS XMM15, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x10, 0x38, // MOVUPS XMM15, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD, 0xDD,
            0xDD, 0xDD,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Unaligned Memory Tests
// ============================================================================

#[test]
fn test_movups_xmm0_to_unaligned_mem() {
    // MOVUPS [unaligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();

    // Verify data was written
    let mut result = [0u8; 16];
    mem.read_slice(&mut result, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
}

#[test]
fn test_movups_xmm1_to_unaligned_mem() {
    // MOVUPS [unaligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x08, // MOVUPS [RAX], XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm7_to_unaligned_mem() {
    // MOVUPS [unaligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x38, // MOVUPS [RAX], XMM7
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm8_to_unaligned_mem() {
    // MOVUPS [unaligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM8
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_xmm15_to_unaligned_mem() {
    // MOVUPS [unaligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x11, 0x38, // MOVUPS [RAX], XMM15
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Integrity Tests with Various Offsets
// ============================================================================

#[test]
fn test_movups_data_integrity_offset_1() {
    // Verify data integrity with 1-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 1;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let test_data = [
        0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF,
    ];
    mem.write_slice(&test_data, GuestAddress(test_addr))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_data_integrity_offset_4() {
    // Verify data integrity with 4-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 4;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let test_data = [
        0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87, 0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D, 0x1E,
        0x0F,
    ];
    mem.write_slice(&test_data, GuestAddress(test_addr))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_data_integrity_offset_8() {
    // Verify data integrity with 8-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 8;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let test_data = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00,
    ];
    mem.write_slice(&test_data, GuestAddress(test_addr))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_movups_all_zeros_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_all_ones_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_alternating_pattern_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
            0xAA, 0x55,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_float_values_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write bit patterns for: 1.5f, 2.5f, 3.5f, 4.5f
    let float1: f32 = 1.5;
    let float2: f32 = 2.5;
    let float3: f32 = 3.5;
    let float4: f32 = 4.5;

    let mut data = Vec::new();
    data.extend_from_slice(&float1.to_le_bytes());
    data.extend_from_slice(&float2.to_le_bytes());
    data.extend_from_slice(&float3.to_le_bytes());
    data.extend_from_slice(&float4.to_le_bytes());

    mem.write_slice(&data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_movups_roundtrip_unaligned() {
    // Test: Memory (unaligned) -> XMM0 -> Memory (unaligned) -> XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0x0f, 0x11, 0x40, 0x20, // MOVUPS [RAX+0x20], XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let test_data = [
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88,
    ];
    mem.write_slice(&test_data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_chain_with_different_offsets() {
    // Load from offset 1, store to offset 5
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX]
        0x48, 0xb8, // MOV RAX, imm64
    ]);
    full_code.extend_from_slice(&(ALIGNED_ADDR + 5).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x11, 0x00, // MOVUPS [RAX], XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE, 0xEE,
            0xEE, 0xEE,
        ],
        GuestAddress(ALIGNED_ADDR + 1),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing Mode Tests with Unaligned Memory
// ============================================================================

#[test]
fn test_movups_base_displacement_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(UNALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x40, 0x10, // MOVUPS XMM0, [RAX + 0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
            0x99, 0x99,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_with_rbx_base_unaligned() {
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x03, // MOVUPS XMM0, [RBX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movups_sequential_operations() {
    // Multiple MOVUPS operations in sequence with different alignments
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x10, 0x00, // MOVUPS XMM0, [RAX] (aligned)
        0x0f, 0x10, 0x48, 0x01, // MOVUPS XMM1, [RAX+1] (unaligned)
        0x0f, 0x10, 0x50, 0x05, // MOVUPS XMM2, [RAX+5] (unaligned)
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
            0x11, 0x11, 0x11, 0x11,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
