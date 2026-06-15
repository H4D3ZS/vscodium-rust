use crate::common::*;

// PHADDW/PHADDD - Packed Horizontal Add
//
// Adds two adjacent 16-bit/32-bit signed integers horizontally from the source
// and destination operands and packs the signed results to the destination.
//
// PHADDW: horizontal add of adjacent words (8 operations)
// PHADDD: horizontal add of adjacent dwords (4 operations)
//
// For PHADDW with 128-bit operands:
//   DEST[15:0] = DEST[31:16] + DEST[15:0]
//   DEST[31:16] = DEST[63:48] + DEST[47:32]
//   DEST[47:32] = DEST[95:80] + DEST[79:64]
//   DEST[63:48] = DEST[127:112] + DEST[111:96]
//   DEST[79:64] = SRC[31:16] + SRC[15:0]
//   DEST[95:80] = SRC[63:48] + SRC[47:32]
//   DEST[111:96] = SRC[95:80] + SRC[79:64]
//   DEST[127:112] = SRC[127:112] + SRC[111:96]
//
// Opcodes:
//   66 0F 38 01 /r    PHADDW xmm1, xmm2/m128
//   66 0F 38 02 /r    PHADDD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PHADDW Tests (Packed Horizontal Add Words)
// ============================================================================

#[test]
fn test_phaddw_xmm0_xmm1_basic() {
    // PHADDW XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xc1, // PHADDW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm2_xmm3_basic() {
    // PHADDW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xd3, // PHADDW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm4_xmm5_zeros() {
    // PHADDW XMM4, XMM5 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xe5, // PHADDW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm6_xmm7_ones() {
    // PHADDW XMM6, XMM7 - all ones
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xf7, // PHADDW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm0_xmm1_positive() {
    // PHADDW XMM0, XMM1 - positive values
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xc1, // PHADDW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm1_xmm2_negative() {
    // PHADDW XMM1, XMM2 - negative values
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xca, // PHADDW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm3_xmm4_mixed() {
    // PHADDW XMM3, XMM4 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xdc, // PHADDW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm5_xmm6_overflow() {
    // PHADDW XMM5, XMM6 - test wraparound/overflow
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xee, // PHADDW XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm7_xmm0() {
    // PHADDW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xf8, // PHADDW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm8_xmm9() {
    // PHADDW XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x01, 0xc1, // PHADDW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm10_xmm11() {
    // PHADDW XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x01, 0xd3, // PHADDW XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm12_xmm13() {
    // PHADDW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x01, 0xe5, // PHADDW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm14_xmm15() {
    // PHADDW XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x01, 0xf7, // PHADDW XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm0_mem() {
    // PHADDW XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x01, 0x00, // PHADDW XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (8 words)
    let data: [u8; 16] = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm1_mem_negative() {
    // PHADDW XMM1, [mem] - negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x01, 0x08, // PHADDW XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm2_mem_wraparound() {
    // PHADDW XMM2, [mem] - test overflow/wraparound
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x01, 0x10, // PHADDW XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Values that will overflow
    let data: [u8; 16] = [
        0xFF, 0x7F, 0x01, 0x00, // 32767 + 1 = wraparound
        0x00, 0x80, 0xFF, 0xFF, // -32768 + -1 = wraparound
        0xFF, 0x7F, 0xFF, 0x7F, // 32767 + 32767 = wraparound
        0x00, 0x80, 0x00, 0x80, // -32768 + -32768 = wraparound
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PHADDD Tests (Packed Horizontal Add Dwords)
// ============================================================================

#[test]
fn test_phaddd_xmm0_xmm1_basic() {
    // PHADDD XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xc1, // PHADDD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm2_xmm3_basic() {
    // PHADDD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xd3, // PHADDD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm4_xmm5_zeros() {
    // PHADDD XMM4, XMM5 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xe5, // PHADDD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm6_xmm7_ones() {
    // PHADDD XMM6, XMM7 - all ones
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xf7, // PHADDD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm0_xmm1_positive() {
    // PHADDD XMM0, XMM1 - positive values
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xc1, // PHADDD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm1_xmm2_negative() {
    // PHADDD XMM1, XMM2 - negative values
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xca, // PHADDD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm3_xmm4_mixed() {
    // PHADDD XMM3, XMM4 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xdc, // PHADDD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm5_xmm6_overflow() {
    // PHADDD XMM5, XMM6 - test wraparound/overflow
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xee, // PHADDD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm7_xmm0() {
    // PHADDD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xf8, // PHADDD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm8_xmm9() {
    // PHADDD XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x02, 0xc1, // PHADDD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm10_xmm11() {
    // PHADDD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x02, 0xd3, // PHADDD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm12_xmm13() {
    // PHADDD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x02, 0xe5, // PHADDD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm14_xmm15() {
    // PHADDD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x02, 0xf7, // PHADDD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm0_mem() {
    // PHADDD XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x02, 0x00, // PHADDD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (4 dwords)
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm1_mem_negative() {
    // PHADDD XMM1, [mem] - negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x02, 0x08, // PHADDD XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFD, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm2_mem_wraparound() {
    // PHADDD XMM2, [mem] - test overflow/wraparound
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x02, 0x10, // PHADDD XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Values that will overflow
    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0x7F, 0x01, 0x00, 0x00, 0x00, // INT32_MAX + 1
        0x00, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0xFF, // INT32_MIN + -1
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional edge case tests
// ============================================================================

#[test]
fn test_phaddw_same_register() {
    // PHADDW XMM0, XMM0 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xc0, // PHADDW XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_same_register() {
    // PHADDD XMM1, XMM1 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xc9, // PHADDD XMM1, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_sequential() {
    // Multiple PHADDW operations in sequence
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xc1, // PHADDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x01, 0xd3, // PHADDW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x01, 0xe5, // PHADDW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_sequential() {
    // Multiple PHADDD operations in sequence
    let code = [
        0x66, 0x0f, 0x38, 0x02, 0xc1, // PHADDD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x02, 0xd3, // PHADDD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x02, 0xe5, // PHADDD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phadd_mixed_operations() {
    // Mix of PHADDW and PHADDD operations
    let code = [
        0x66, 0x0f, 0x38, 0x01, 0xc1, // PHADDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x02, 0xd3, // PHADDD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x01, 0xe5, // PHADDW XMM4, XMM5
        0x66, 0x0f, 0x38, 0x02, 0xf7, // PHADDD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_xmm0_xmm15_cross_range() {
    // PHADDW XMM0, XMM15 - test low and high register mix
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x01, 0xf8, // PHADDW XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_xmm15_xmm0_cross_range() {
    // PHADDD XMM15, XMM0 - test high and low register mix
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x02, 0xf8, // PHADDD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddw_mem_displacement() {
    // PHADDW XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x01, 0x40, 0x10, // PHADDW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x01, 0x00,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phaddd_mem_displacement() {
    // PHADDD XMM1, [RBX + disp]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x20).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x02, 0x4b, 0x20, // PHADDD XMM1, [RBX+0x20]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
