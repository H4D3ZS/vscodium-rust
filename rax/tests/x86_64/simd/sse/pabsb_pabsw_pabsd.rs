use crate::common::*;

// PABSB/PABSW/PABSD - Packed Absolute Value
//
// Computes the absolute value of each data element of the source operand
// and stores the UNSIGNED results in the destination operand.
//
// PABSB: operates on signed bytes (16 elements)
// PABSW: operates on signed 16-bit words (8 elements)
// PABSD: operates on signed 32-bit integers (4 elements)
//
// Opcodes:
//   66 0F 38 1C /r    PABSB xmm1, xmm2/m128
//   66 0F 38 1D /r    PABSW xmm1, xmm2/m128
//   66 0F 38 1E /r    PABSD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PABSB Tests (Packed Absolute Value Bytes)
// ============================================================================

#[test]
fn test_pabsb_xmm0_xmm1_basic() {
    // PABSB XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xc1, // PABSB XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm2_xmm3_basic() {
    // PABSB XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xd3, // PABSB XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm4_xmm5_positive() {
    // PABSB XMM4, XMM5 - all positive values
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xe5, // PABSB XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm6_xmm7_negative() {
    // PABSB XMM6, XMM7 - all negative values
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xf7, // PABSB XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm0_xmm1_zeros() {
    // PABSB XMM0, XMM1 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xc1, // PABSB XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm1_xmm2_int8_min() {
    // PABSB XMM1, XMM2 - INT8_MIN (-128)
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xca, // PABSB XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm3_xmm4_mixed() {
    // PABSB XMM3, XMM4 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xdc, // PABSB XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm5_xmm6_alternating() {
    // PABSB XMM5, XMM6 - alternating positive/negative
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xee, // PABSB XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm7_xmm0() {
    // PABSB XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xf8, // PABSB XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm8_xmm9() {
    // PABSB XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1c, 0xc1, // PABSB XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm10_xmm11() {
    // PABSB XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1c, 0xd3, // PABSB XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm0_mem() {
    // PABSB XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1c, 0x00, // PABSB XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory
    let data: [u8; 16] = [
        0x01, 0xFF, 0x7F, 0x80, 0x00, 0xFE, 0x02, 0x81, 0x10, 0xF0, 0x20, 0xE0, 0x30, 0xD0, 0x40,
        0xC0,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_xmm1_mem_negative() {
    // PABSB XMM1, [mem] - all negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1c, 0x08, // PABSB XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let data: [u8; 16] = [
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80,
    ]; // All INT8_MIN
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PABSW Tests (Packed Absolute Value Words)
// ============================================================================

#[test]
fn test_pabsw_xmm0_xmm1_basic() {
    // PABSW XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xc1, // PABSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm2_xmm3_basic() {
    // PABSW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xd3, // PABSW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm4_xmm5_positive() {
    // PABSW XMM4, XMM5 - all positive values
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xe5, // PABSW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm6_xmm7_negative() {
    // PABSW XMM6, XMM7 - all negative values
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xf7, // PABSW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm0_xmm1_zeros() {
    // PABSW XMM0, XMM1 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xc1, // PABSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm1_xmm2_int16_min() {
    // PABSW XMM1, XMM2 - INT16_MIN (-32768)
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xca, // PABSW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm3_xmm4_mixed() {
    // PABSW XMM3, XMM4 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xdc, // PABSW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm5_xmm6_alternating() {
    // PABSW XMM5, XMM6 - alternating positive/negative
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xee, // PABSW XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm7_xmm0() {
    // PABSW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xf8, // PABSW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm8_xmm9() {
    // PABSW XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1d, 0xc1, // PABSW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm12_xmm13() {
    // PABSW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1d, 0xe5, // PABSW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm0_mem() {
    // PABSW XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1d, 0x00, // PABSW XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (8 words)
    let data: [u8; 16] = [
        0x01, 0x00, 0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x80, 0x34, 0x12, 0xCC, 0xED, 0x00, 0x00, 0x01,
        0x80,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_xmm1_mem_negative() {
    // PABSW XMM1, [mem] - all negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1d, 0x08, // PABSW XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let data: [u8; 16] = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ]; // All INT16_MIN
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PABSD Tests (Packed Absolute Value Dwords)
// ============================================================================

#[test]
fn test_pabsd_xmm0_xmm1_basic() {
    // PABSD XMM0, XMM1 - basic register to register
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xc1, // PABSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm2_xmm3_basic() {
    // PABSD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xd3, // PABSD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm4_xmm5_positive() {
    // PABSD XMM4, XMM5 - all positive values
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xe5, // PABSD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm6_xmm7_negative() {
    // PABSD XMM6, XMM7 - all negative values
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xf7, // PABSD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm0_xmm1_zeros() {
    // PABSD XMM0, XMM1 - all zeros
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xc1, // PABSD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm1_xmm2_int32_min() {
    // PABSD XMM1, XMM2 - INT32_MIN (-2147483648)
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xca, // PABSD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm3_xmm4_mixed() {
    // PABSD XMM3, XMM4 - mixed positive and negative
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xdc, // PABSD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm5_xmm6_alternating() {
    // PABSD XMM5, XMM6 - alternating positive/negative
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xee, // PABSD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm7_xmm0() {
    // PABSD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xf8, // PABSD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm8_xmm9() {
    // PABSD XMM8, XMM9 - high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1e, 0xc1, // PABSD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm14_xmm15() {
    // PABSD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x1e, 0xf7, // PABSD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm0_mem() {
    // PABSD XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1e, 0x00, // PABSD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (4 dwords)
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, // 1
        0xFF, 0xFF, 0xFF, 0xFF, // -1
        0xFF, 0xFF, 0xFF, 0x7F, // INT32_MAX
        0x00, 0x00, 0x00, 0x80, // INT32_MIN
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_xmm1_mem_negative() {
    // PABSD XMM1, [mem] - all negative values
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x1e, 0x08, // PABSD XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        0x80,
    ]; // All INT32_MIN
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional edge case tests
// ============================================================================

#[test]
fn test_pabsb_same_register() {
    // PABSB XMM0, XMM0 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xc0, // PABSB XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_same_register() {
    // PABSW XMM1, XMM1 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xc9, // PABSW XMM1, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_same_register() {
    // PABSD XMM2, XMM2 - source and dest are same
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xd2, // PABSD XMM2, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsb_sequential() {
    // Multiple PABSB operations in sequence
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xc1, // PABSB XMM0, XMM1
        0x66, 0x0f, 0x38, 0x1c, 0xd3, // PABSB XMM2, XMM3
        0x66, 0x0f, 0x38, 0x1c, 0xe5, // PABSB XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsw_sequential() {
    // Multiple PABSW operations in sequence
    let code = [
        0x66, 0x0f, 0x38, 0x1d, 0xc1, // PABSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x1d, 0xd3, // PABSW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x1d, 0xe5, // PABSW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabsd_sequential() {
    // Multiple PABSD operations in sequence
    let code = [
        0x66, 0x0f, 0x38, 0x1e, 0xc1, // PABSD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x1e, 0xd3, // PABSD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x1e, 0xe5, // PABSD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pabs_mixed_operations() {
    // Mix of PABSB, PABSW, PABSD operations
    let code = [
        0x66, 0x0f, 0x38, 0x1c, 0xc1, // PABSB XMM0, XMM1
        0x66, 0x0f, 0x38, 0x1d, 0xd3, // PABSW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x1e, 0xe5, // PABSD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
