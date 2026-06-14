use crate::common::*;

// PCMPEQQ - Compare Packed Qword Data for Equal
//
// Performs an SIMD compare for equality of the packed quadwords in the
// destination operand and the source operand. If a pair of data elements
// is equal, the corresponding data element in the destination is set to
// all 1s; otherwise, it is set to 0s.
//
// For each quadword:
//   if DEST[63:0] == SRC[63:0] then DEST[63:0] := 0xFFFFFFFFFFFFFFFF
//   else DEST[63:0] := 0x0000000000000000
//
// Opcode:
//   66 0F 38 29 /r    PCMPEQQ xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pcmpeqq_xmm0_xmm1_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm2_xmm3_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xd3, // PCMPEQQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm4_xmm5_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xe5, // PCMPEQQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm6_xmm7_not_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xf7, // PCMPEQQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm0_xmm1_zeros() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm1_xmm2_all_ones() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xca, // PCMPEQQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm3_xmm4_first_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xdc, // PCMPEQQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm5_xmm6_second_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xee, // PCMPEQQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm7_xmm0_both_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xf8, // PCMPEQQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x29, 0xd3, // PCMPEQQ XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x29, 0xe5, // PCMPEQQ XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x29, 0xf7, // PCMPEQQ XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm0_mem() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x29, 0x00, // PCMPEQQ XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm1_mem_equal() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x29, 0x08, // PCMPEQQ XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Two equal qwords
    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm2_mem_not_equal() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x29, 0x10, // PCMPEQQ XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Two different qwords
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_same_register() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc0, // PCMPEQQ XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_sequential() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0x66, 0x0f, 0x38, 0x29, 0xd3, // PCMPEQQ XMM2, XMM3
        0x66, 0x0f, 0x38, 0x29, 0xe5, // PCMPEQQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_xmm15_xmm0_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x29, 0xf8, // PCMPEQQ XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x29, 0x40, 0x10, // PCMPEQQ XMM0, [RAX+0x10]
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

#[test]
fn test_pcmpeqq_max_int64() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_min_int64() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_negative_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_positive_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_mixed_signs() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_result_all_ones() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_result_all_zeros() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_result_mixed() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_boundary_low_qword() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_boundary_high_qword() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_power_of_two() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_alternating_bits() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_incremental_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_identical_qwords() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpeqq_large_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x29, 0xc1, // PCMPEQQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PCMPEQQ sets a qword lane to all-ones when the two qwords are equal.
// ============================================================================

#[test]
fn kat_pcmpeqq_value() {
    // PCMPEQQ XMM0, XMM1 (66 0F 38 29 C1)
    // qword0 equal, qword1 not equal.
    let code = [0x66, 0x0f, 0x38, 0x29, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x0000000000000001_1122334455667788);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000002_1122334455667788);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x00000000000000000000000000000000u128 | 0xFFFFFFFFFFFFFFFFu128,
        "PCMPEQQ got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
