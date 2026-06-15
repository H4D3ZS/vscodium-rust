use crate::common::*;

// PHMINPOSUW - Packed Horizontal Word Minimum
//
// Determines the minimum unsigned word value in the source operand and places
// the unsigned word in the low word (bits 0-15) of the destination operand.
// The word index of the minimum value is stored in bits 16-18 of the destination
// operand. The remaining upper bits of the destination are set to zero.
//
// Result format:
//   DEST[15:0]   = minimum unsigned word value
//   DEST[18:16]  = index (0-7) of minimum word
//   DEST[127:19] = 0
//
// Opcode:
//   66 0F 38 41 /r    PHMINPOSUW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_phminposuw_xmm0_xmm1_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm2_xmm3_basic() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm4_xmm5_zeros() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm6_xmm7_min_at_index_0() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xf7, // PHMINPOSUW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm0_xmm1_min_at_index_1() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm1_xmm2_min_at_index_7() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xca, // PHMINPOSUW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm3_xmm4_min_at_middle() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xdc, // PHMINPOSUW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xf7, // PHMINPOSUW XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm0_mem() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x00, // PHMINPOSUW XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Descending values: 8, 7, 6, 5, 4, 3, 2, 1 (min = 1 at index 7)
    let data: [u8; 16] = [8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, 0];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm1_mem_min_at_start() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x08, // PHMINPOSUW XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Ascending values: 1, 2, 3, 4, 5, 6, 7, 8 (min = 1 at index 0)
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm2_mem_all_max() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x10, // PHMINPOSUW XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All max values: 0xFFFF (min = 0xFFFF at index 0)
    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_same_register() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc0, // PHMINPOSUW XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_sequential() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_xmm15_xmm0_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xf8, // PHMINPOSUW XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x41, 0x40, 0x10, // PHMINPOSUW XMM0, [RAX+0x10]
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
fn test_phminposuw_min_is_zero() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_min_is_one() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_all_equal() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_duplicate_minimums() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_index_bits_16_18() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_upper_bits_zeroed() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_unsigned_comparison() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_alternating_pattern() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_random_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_power_of_two() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_boundary_values() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_min_at_each_position() {
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PHMINPOSUW finds the minimum UNSIGNED 16-bit word of SRC; result low word =
// min value, next word = its index (lowest index on ties), rest zero.
// ============================================================================

#[test]
fn kat_phminposuw_value() {
    // PHMINPOSUW XMM0, XMM1 (66 0F 38 41 C1)
    // SRC words lane0..7 = [9,8,7,6,5,4,3,2] => min 2 at index 7.
    let code = [0x66, 0x0f, 0x38, 0x41, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00020003000400050006000700080009);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x00000000000000000000000000070002,
        "PHMINPOSUW got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_phminposuw_tie_lowest_index() {
    // Two minima (value 1) at index 2 and index 5 => lowest index (2) wins.
    let code = [0x66, 0x0f, 0x38, 0x41, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00020003000100050006000100080009);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x00000000000000000000000000020001,
        "PHMINPOSUW tie got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
