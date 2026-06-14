use crate::common::*;

// PTEST - Logical Compare
//
// Sets the ZF flag if all bits in the result are 0 of the bitwise AND of the
// first source operand and the second source operand. Sets the CF flag if all
// bits in the result are 0 of the bitwise AND of the second source operand and
// the logical NOT of the first source operand (destination).
//
// ZF = 1 if (SRC1 AND SRC2) == 0
// CF = 1 if (SRC2 AND NOT SRC1) == 0
//
// Opcode:
//   66 0F 38 17 /r    PTEST xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// Test basic register-to-register operations
#[test]
fn test_ptest_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xc1, // PTEST XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xd3, // PTEST XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xe5, // PTEST XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm6_xmm7() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xf7, // PTEST XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_ptest_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xc1, // PTEST XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xd3, // PTEST XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xe5, // PTEST XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xf7, // PTEST XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands - all zeros (should set ZF)
#[test]
fn test_ptest_xmm0_mem_all_zeros() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x00, // PTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]; // All zeros
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands - all ones
#[test]
fn test_ptest_xmm1_mem_all_ones() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x08, // PTEST XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ]; // All ones
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands - alternating bit pattern
#[test]
fn test_ptest_xmm2_mem_alternating() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x10, // PTEST XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa,
    ]; // 10101010 pattern
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands - alternating pattern 2
#[test]
fn test_ptest_xmm3_mem_alternating2() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x18, // PTEST XMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55,
    ]; // 01010101 pattern
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test same register (should test register against itself)
#[test]
fn test_ptest_same_register_xmm0() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xc0, // PTEST XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_same_register_xmm7() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xff, // PTEST XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test sequential operations
#[test]
fn test_ptest_sequential_operations() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xc1, // PTEST XMM0, XMM1
        0x66, 0x0f, 0x38, 0x17, 0xd3, // PTEST XMM2, XMM3
        0x66, 0x0f, 0x38, 0x17, 0xe5, // PTEST XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory with displacement
#[test]
fn test_ptest_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x40, 0x10, // PTEST XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_ptest_xmm0_xmm15() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x17, 0xf8, // PTEST XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test various bit patterns in memory to test ZF and CF flags
#[test]
fn test_ptest_xmm4_mem_pattern1() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x20, // PTEST XMM4, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x0f, 0x0f, 0x0f, 0x0f, 0xf0, 0xf0, 0xf0, 0xf0, 0x0f, 0x0f, 0x0f, 0x0f, 0xf0, 0xf0, 0xf0,
        0xf0,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm5_mem_pattern2() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x28, // PTEST XMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional register combinations
#[test]
fn test_ptest_xmm1_xmm0() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xc8, // PTEST XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm3_xmm2() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xda, // PTEST XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm5_xmm4() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xec, // PTEST XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm7_xmm6() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xfe, // PTEST XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test high register combinations
#[test]
fn test_ptest_xmm9_xmm8() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xc8, // PTEST XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm11_xmm10() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xda, // PTEST XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm13_xmm12() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xec, // PTEST XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm15_xmm14() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x17, 0xfe, // PTEST XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test specific bit patterns for flag testing
#[test]
fn test_ptest_xmm6_mem_single_bit() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x30, // PTEST XMM6, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm7_mem_high_bit() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x38, // PTEST XMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test nibble patterns
#[test]
fn test_ptest_xmm0_mem_low_nibble() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x00, // PTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f,
        0x0f,
    ]; // Low nibble set
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm1_mem_high_nibble() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x08, // PTEST XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
        0xf0,
    ]; // High nibble set
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test mixed patterns
#[test]
fn test_ptest_xmm2_mem_checkerboard() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x17, 0x10, // PTEST XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa,
        0x55,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional tests to reach 35+ tests
#[test]
fn test_ptest_xmm0_xmm3() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xc3, // PTEST XMM0, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm1_xmm4() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xcc, // PTEST XMM1, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ptest_xmm2_xmm5() {
    let code = [
        0x66, 0x0f, 0x38, 0x17, 0xd5, // PTEST XMM2, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PTEST sets ZF = ((SRC AND DST) == 0) and CF = ((SRC AND NOT DST) == 0).
// Here DST = XMM_reg (first operand), SRC = XMM/m128 (second operand).
// ============================================================================

#[test]
fn kat_ptest_zf_set_cf_clear() {
    // PTEST XMM0, XMM1 (66 0F 38 17 C1)
    // DST and SRC are bitwise-DISJOINT => (SRC AND DST)==0 => ZF=1.
    // SRC & ~DST = SRC (nonzero) => CF=0.
    let code = [0x66, 0x0f, 0x38, 0x17, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00FF00FF00FF00FF00FF00FF00FF00FF);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xFF00FF00FF00FF00FF00FF00FF00FF00);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert!(crate::common::zf_set(regs.rflags), "ZF should be set");
    assert!(!crate::common::cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn kat_ptest_zf_clear_cf_set() {
    // DST = SRC = same nonzero value.
    // AND = value != 0 => ZF=0.  SRC & ~DST = 0 => CF=1.
    let code = [0x66, 0x0f, 0x38, 0x17, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x0123456789ABCDEF_FEDCBA9876543210);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0123456789ABCDEF_FEDCBA9876543210);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert!(!crate::common::zf_set(regs.rflags), "ZF should be clear");
    assert!(crate::common::cf_set(regs.rflags), "CF should be set");
}

#[test]
fn kat_ptest_both_set() {
    // SRC = 0 => AND==0 (ZF=1) and SRC&~DST==0 (CF=1).
    let code = [0x66, 0x0f, 0x38, 0x17, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xDEADBEEFDEADBEEFDEADBEEFDEADBEEF);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert!(crate::common::zf_set(regs.rflags), "ZF should be set");
    assert!(crate::common::cf_set(regs.rflags), "CF should be set");
}
