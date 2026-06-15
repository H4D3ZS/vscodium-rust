use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVDQA - Move Aligned Double Quadword (128-bit Integer)
//
// Moves 128 bits of packed integer data from source to destination.
// When the operand is a memory location, it must be aligned on a 16-byte boundary.
// Otherwise, a general-protection exception (#GP) is generated.
//
// Opcodes:
// 66 0F 6F /r    MOVDQA xmm1, xmm2/m128    - Move aligned packed integer from xmm2/mem to xmm1
// 66 0F 7F /r    MOVDQA xmm2/m128, xmm1    - Move aligned packed integer from xmm1 to xmm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movdqa_xmm0_to_xmm1() {
    // MOVDQA XMM1, XMM0
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm2_to_xmm3() {
    // MOVDQA XMM3, XMM2
    let code = [
        0x66, 0x0f, 0x6f, 0xda, // MOVDQA XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm4_to_xmm5() {
    // MOVDQA XMM5, XMM4
    let code = [
        0x66, 0x0f, 0x6f, 0xec, // MOVDQA XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm6_to_xmm7() {
    // MOVDQA XMM7, XMM6
    let code = [
        0x66, 0x0f, 0x6f, 0xfe, // MOVDQA XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm8_to_xmm9() {
    // MOVDQA XMM9, XMM8 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xc8, // MOVDQA XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm10_to_xmm11() {
    // MOVDQA XMM11, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xda, // MOVDQA XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm12_to_xmm13() {
    // MOVDQA XMM13, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xec, // MOVDQA XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm14_to_xmm15() {
    // MOVDQA XMM15, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xfe, // MOVDQA XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm0_to_xmm15() {
    // MOVDQA XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x6f, 0xf8, // MOVDQA XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm15_to_xmm0() {
    // MOVDQA XMM0, XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x6f, 0xc7, // MOVDQA XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_movdqa_mem_to_xmm0_aligned() {
    // MOVDQA XMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write 128-bit integer data
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
fn test_movdqa_mem_to_xmm1_aligned() {
    // MOVDQA XMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
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
fn test_movdqa_mem_to_xmm7_aligned() {
    // MOVDQA XMM7, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, // MOVDQA XMM7, [RAX]
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
fn test_movdqa_mem_to_xmm8_aligned() {
    // MOVDQA XMM8, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
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
fn test_movdqa_mem_to_xmm15_aligned() {
    // MOVDQA XMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x38, // MOVDQA XMM15, [RAX]
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
fn test_movdqa_xmm0_to_mem_aligned() {
    // MOVDQA [aligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM0
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
fn test_movdqa_xmm1_to_mem_aligned() {
    // MOVDQA [aligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x08, // MOVDQA [RAX], XMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm7_to_mem_aligned() {
    // MOVDQA [aligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x38, // MOVDQA [RAX], XMM7
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm8_to_mem_aligned() {
    // MOVDQA [aligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM8
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_xmm15_to_mem_aligned() {
    // MOVDQA [aligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x7f, 0x38, // MOVDQA [RAX], XMM15
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Integer Data Pattern Tests
// ============================================================================

#[test]
fn test_movdqa_all_zeros() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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
fn test_movdqa_all_ones() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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
fn test_movdqa_packed_bytes() {
    // Test with 16 distinct bytes
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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

#[test]
fn test_movdqa_packed_words() {
    // Test with 8 packed 16-bit words
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_packed_dwords() {
    // Test with 4 packed 32-bit dwords
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let dwords: [u32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    let mut data = Vec::new();
    for dword in &dwords {
        data.extend_from_slice(&dword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_packed_qwords() {
    // Test with 2 packed 64-bit qwords
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let qwords: [u64; 2] = [0x1111111111111111, 0x2222222222222222];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_alternating_pattern() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_movdqa_base_displacement() {
    // MOVDQA XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x40, 0x10, // MOVDQA XMM0, [RAX + 0x10]
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
fn test_movdqa_rip_relative() {
    // MOVDQA XMM0, [RIP + displacement]
    // Code starts at 0x1000, instruction is 9 bytes (8 + HLT)
    // After the MOVDQA (8 bytes), RIP = 0x1008
    // We need [RIP+disp] to be 16-byte aligned
    // To read from 0x1010: disp = 0x1010 - 0x1008 = 8
    let code = [
        0x66, 0x0f, 0x6f, 0x05, 0x08, 0x00, 0x00, 0x00, // MOVDQA XMM0, [RIP+8]
        0xf4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Write data at 0x1010 (16-byte aligned)
    mem.write_slice(&[0xAA; 16], GuestAddress(0x1010)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_with_rbx_base() {
    // MOVDQA XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x03, // MOVDQA XMM0, [RBX]
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
fn test_movdqa_with_rcx_base() {
    // MOVDQA XMM0, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x01, // MOVDQA XMM0, [RCX]
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
fn test_movdqa_with_rdx_base() {
    // MOVDQA XMM0, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x02, // MOVDQA XMM0, [RDX]
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
fn test_movdqa_roundtrip_reg_mem_reg() {
    // Test: XMM0 -> Memory -> XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM0
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_chain_move() {
    // Test: XMM0 -> XMM1 -> XMM2 -> XMM3
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0x66, 0x0f, 0x6f, 0xd1, // MOVDQA XMM2, XMM1
        0x66, 0x0f, 0x6f, 0xda, // MOVDQA XMM3, XMM2
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn test_movdqa_at_0x2000_aligned() {
    // Use DATA_ADDR (0x2000) to avoid overlapping with code at 0x1000
    const TEST_ADDR: u64 = 0x2000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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
fn test_movdqa_at_0x4000_aligned() {
    const TEST_ADDR: u64 = 0x4000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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
fn test_movdqa_multiple_xmm_operations() {
    // Test multiple MOVDQA operations in sequence
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0x66, 0x0f, 0x6f, 0xd0, // MOVDQA XMM2, XMM0
        0x66, 0x0f, 0x6f, 0xd8, // MOVDQA XMM3, XMM0
        0x66, 0x0f, 0x6f, 0xe0, // MOVDQA XMM4, XMM0
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movdqa_overwrite_destination() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
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

#[test]
fn test_movdqa_max_values() {
    // Test with maximum values for different integer types
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Two packed 64-bit integers: max i64 and max u64
    let qwords: [u64; 2] = [i64::MAX as u64, u64::MAX];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
