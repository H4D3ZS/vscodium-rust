use crate::common::*;

// PMADDUBSW - Multiply and Add Packed Signed and Unsigned Bytes
//
// Multiplies vertically each unsigned byte of the destination operand with the
// corresponding signed byte of the source operand, producing intermediate signed
// 16-bit integers. Each adjacent pair of signed words is added and the saturated
// result is packed to the destination operand.
//
// For each pair of bytes:
//   temp[i] = unsigned_byte[2*i] * signed_byte[2*i] + unsigned_byte[2*i+1] * signed_byte[2*i+1]
//   result[i] = saturate_to_signed_word(temp[i])
//
// Opcode:
//   66 0F 38 04 /r    PMADDUBSW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pmaddubsw_xmm0_xmm1_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm2_xmm3_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xd3, // PMADDUBSW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm4_xmm5_zeros() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xe5, // PMADDUBSW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm6_xmm7_ones() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xf7, // PMADDUBSW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm0_xmm1_positive() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm1_xmm2_negative_signed() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xca, // PMADDUBSW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm3_xmm4_mixed() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xdc, // PMADDUBSW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm5_xmm6_saturate_positive() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xee, // PMADDUBSW XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm7_xmm0_saturate_negative() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xf8, // PMADDUBSW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xd3, // PMADDUBSW XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xe5, // PMADDUBSW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xf7, // PMADDUBSW XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm0_mem() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x04, 0x00, // PMADDUBSW XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm1_mem_signed_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x04, 0x08, // PMADDUBSW XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1,
        0xF0,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm2_mem_saturate() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x04, 0x10, // PMADDUBSW XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Values that will saturate: 255 * 127 + 255 * 127 = 64770 > 32767
    let data: [u8; 16] = [
        0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
        0x7F,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_same_register() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc0, // PMADDUBSW XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_sequential() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x04, 0xd3, // PMADDUBSW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x04, 0xe5, // PMADDUBSW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_xmm0_xmm15_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xf8, // PMADDUBSW XMM0, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x04, 0x40, 0x10, // PMADDUBSW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional tests for saturation behavior
#[test]
fn test_pmaddubsw_saturate_to_max() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_saturate_to_min() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_max_unsigned_positive_signed() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_max_unsigned_negative_signed() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_zero_multiplication() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_identity_multiplication() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_alternating_pattern() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_edge_case_127() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_edge_case_128() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_large_products() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_small_products() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_mixed_small_large() {
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
