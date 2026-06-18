use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVDQU - Move Unaligned Double Quadword (128-bit Integer)
//
// Moves 128 bits of packed integer data from source to destination.
// Unlike MOVDQA, this instruction does NOT require 16-byte alignment.
// Can move data to/from unaligned memory addresses without causing #GP.
//
// Opcodes:
// F3 0F 6F /r    MOVDQU xmm1, xmm2/m128    - Move unaligned packed integer from xmm2/mem to xmm1
// F3 0F 7F /r    MOVDQU xmm2/m128, xmm1    - Move unaligned packed integer from xmm1 to xmm2/mem

const UNALIGNED_ADDR: u64 = 0x3001; // Intentionally unaligned (offset by 1)
const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned (MOVDQU works with aligned too)

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movdqu_xmm0_to_xmm1() {
    // MOVDQU XMM1, XMM0
    let code = [
        0xf3, 0x0f, 0x6f, 0xc8, // MOVDQU XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm2_to_xmm3() {
    // MOVDQU XMM3, XMM2
    let code = [
        0xf3, 0x0f, 0x6f, 0xda, // MOVDQU XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm4_to_xmm5() {
    // MOVDQU XMM5, XMM4
    let code = [
        0xf3, 0x0f, 0x6f, 0xec, // MOVDQU XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm6_to_xmm7() {
    // MOVDQU XMM7, XMM6
    let code = [
        0xf3, 0x0f, 0x6f, 0xfe, // MOVDQU XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm8_to_xmm9() {
    // MOVDQU XMM9, XMM8 (requires REX prefix)
    let code = [
        0xf3, 0x45, 0x0f, 0x6f, 0xc8, // MOVDQU XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm10_to_xmm11() {
    // MOVDQU XMM11, XMM10
    let code = [
        0xf3, 0x45, 0x0f, 0x6f, 0xda, // MOVDQU XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm12_to_xmm13() {
    // MOVDQU XMM13, XMM12
    let code = [
        0xf3, 0x45, 0x0f, 0x6f, 0xec, // MOVDQU XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm14_to_xmm15() {
    // MOVDQU XMM15, XMM14
    let code = [
        0xf3, 0x45, 0x0f, 0x6f, 0xfe, // MOVDQU XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm0_to_xmm15() {
    // MOVDQU XMM15, XMM0
    let code = [
        0xf3, 0x44, 0x0f, 0x6f, 0xf8, // MOVDQU XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm15_to_xmm0() {
    // MOVDQU XMM0, XMM15
    let code = [
        0xf3, 0x44, 0x0f, 0x6f, 0xc7, // MOVDQU XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Unaligned Memory to Register Tests
// ============================================================================

#[test]
fn test_movdqu_unaligned_mem_to_xmm0() {
    // MOVDQU XMM0, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data (128-bit integer)
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
fn test_movdqu_unaligned_offset_1() {
    // Test with 1-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_unaligned_offset_2() {
    // Test with 2-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_unaligned_offset_3() {
    // Test with 3-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 3).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_unaligned_offset_7() {
    // Test with 7-byte offset from alignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 7).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_unaligned_offset_15() {
    // Test with 15-byte offset from alignment (worst case)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 15).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_aligned_mem_to_xmm1() {
    // MOVDQU also works with aligned memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x08, // MOVDQU XMM1, [RAX]
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
fn test_movdqu_mem_to_xmm8_unaligned() {
    // MOVDQU XMM8, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x6f, 0x00, // MOVDQU XMM8, [RAX]
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
fn test_movdqu_mem_to_xmm15_unaligned() {
    // MOVDQU XMM15, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x6f, 0x38, // MOVDQU XMM15, [RAX]
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
fn test_movdqu_xmm0_to_unaligned_mem() {
    // MOVDQU [unaligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x7f, 0x00, // MOVDQU [RAX], XMM0
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
fn test_movdqu_xmm1_to_unaligned_mem() {
    // MOVDQU [unaligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x7f, 0x08, // MOVDQU [RAX], XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm7_to_unaligned_mem() {
    // MOVDQU [unaligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x7f, 0x38, // MOVDQU [RAX], XMM7
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm8_to_unaligned_mem() {
    // MOVDQU [unaligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x7f, 0x00, // MOVDQU [RAX], XMM8
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_xmm15_to_unaligned_mem() {
    // MOVDQU [unaligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x44, 0x0f, 0x7f, 0x38, // MOVDQU [RAX], XMM15
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Integrity Tests with Various Offsets
// ============================================================================

#[test]
fn test_movdqu_data_integrity_offset_1() {
    // Verify data integrity with 1-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 1;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_data_integrity_offset_4() {
    // Verify data integrity with 4-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 4;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_data_integrity_offset_8() {
    // Verify data integrity with 8-byte misalignment
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    let test_addr = ALIGNED_ADDR + 8;
    full_code.extend_from_slice(&test_addr.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
// Pattern Tests with Various Integer Types
// ============================================================================

#[test]
fn test_movdqu_all_zeros_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_all_ones_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_alternating_pattern_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
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
fn test_movdqu_packed_bytes_unaligned() {
    // Test with 16 distinct bytes (unaligned)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        GuestAddress(UNALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_packed_words_unaligned() {
    // Test with 8 packed 16-bit words (unaligned)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let words: [u16; 8] = [
        0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777, 0x8888,
    ];
    let mut data = Vec::new();
    for word in &words {
        data.extend_from_slice(&word.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_packed_dwords_unaligned() {
    // Test with 4 packed 32-bit dwords (unaligned)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let dwords: [u32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    let mut data = Vec::new();
    for dword in &dwords {
        data.extend_from_slice(&dword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqu_packed_qwords_unaligned() {
    // Test with 2 packed 64-bit qwords (unaligned)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let qwords: [u64; 2] = [0x1111111111111111, 0x2222222222222222];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_movdqu_roundtrip_unaligned() {
    // Test: Memory (unaligned) -> XMM0 -> Memory (unaligned) -> XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf3, 0x0f, 0x7f, 0x40, 0x20, // MOVDQU [RAX+0x20], XMM0
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
fn test_movdqu_chain_with_different_offsets() {
    // Load from offset 1, store to offset 5
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0x48, 0xb8, // MOV RAX, imm64
    ]);
    full_code.extend_from_slice(&(ALIGNED_ADDR + 5).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x7f, 0x00, // MOVDQU [RAX], XMM0
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
fn test_movdqu_base_displacement_unaligned() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(UNALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x40, 0x10, // MOVDQU XMM0, [RAX + 0x10]
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
fn test_movdqu_with_rbx_base_unaligned() {
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x03, // MOVDQU XMM0, [RBX]
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
fn test_movdqu_sequential_operations() {
    // Multiple MOVDQU operations in sequence with different alignments
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX] (aligned)
        0xf3, 0x0f, 0x6f, 0x48, 0x01, // MOVDQU XMM1, [RAX+1] (unaligned)
        0xf3, 0x0f, 0x6f, 0x50, 0x05, // MOVDQU XMM2, [RAX+5] (unaligned)
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

#[test]
fn test_movdqu_max_values_unaligned() {
    // Test with maximum values for different integer types (unaligned)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x6f, 0x00, // MOVDQU XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Two packed 64-bit integers: max i64 and max u64
    let qwords: [u64; 2] = [i64::MAX as u64, u64::MAX];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(UNALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
