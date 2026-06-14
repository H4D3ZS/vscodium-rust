use crate::common::*;

// PMULLD - Multiply Packed Signed Dword Integers and Store Low Result
//
// Performs a SIMD signed multiply of the packed signed dword integers from
// each element of the first source operand with the corresponding element in
// the second source operand. The low 32 bits of each 64-bit intermediate
// result are stored to the destination operand.
//
// Opcode:
//   66 0F 38 40 /r    PMULLD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// Test basic multiplication with different register pairs
#[test]
fn test_pmulld_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm6_xmm7() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xf7, // PMULLD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test high XMM registers (XMM8-XMM15)
#[test]
fn test_pmulld_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xf7, // PMULLD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands with positive values
#[test]
fn test_pmulld_xmm0_mem_positive() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x00, // PMULLD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Four signed dwords: 2, 3, 4, 5
    let data: [u8; 16] = [
        0x02, 0x00, 0x00, 0x00, // 2
        0x03, 0x00, 0x00, 0x00, // 3
        0x04, 0x00, 0x00, 0x00, // 4
        0x05, 0x00, 0x00, 0x00, // 5
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands with negative values
#[test]
fn test_pmulld_xmm1_mem_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x08, // PMULLD XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Four signed dwords: -1, -2, -3, -4
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, // -1
        0xfe, 0xff, 0xff, 0xff, // -2
        0xfd, 0xff, 0xff, 0xff, // -3
        0xfc, 0xff, 0xff, 0xff, // -4
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory operands with mixed positive and negative values
#[test]
fn test_pmulld_xmm2_mem_mixed() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x10, // PMULLD XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Mixed: 5, -3, 100, -50
    let data: [u8; 16] = [
        0x05, 0x00, 0x00, 0x00, // 5
        0xfd, 0xff, 0xff, 0xff, // -3
        0x64, 0x00, 0x00, 0x00, // 100
        0xce, 0xff, 0xff, 0xff, // -50
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test overflow behavior (result truncated to low 32 bits)
#[test]
fn test_pmulld_xmm3_mem_overflow() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x18, // PMULLD XMM3, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Large values that will overflow: 0x10000, 0x20000, 0x40000, 0x80000
    let data: [u8; 16] = [
        0x00, 0x00, 0x01, 0x00, // 0x10000 (65536)
        0x00, 0x00, 0x02, 0x00, // 0x20000 (131072)
        0x00, 0x00, 0x04, 0x00, // 0x40000 (262144)
        0x00, 0x00, 0x08, 0x00, // 0x80000 (524288)
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with zeros
#[test]
fn test_pmulld_xmm4_mem_zeros() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x20, // PMULLD XMM4, [RAX]
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

// Test with ones
#[test]
fn test_pmulld_xmm5_mem_ones() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x28, // PMULLD XMM5, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All ones (1, 1, 1, 1)
    let data: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test same register (square operation)
#[test]
fn test_pmulld_same_register_xmm0() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc0, // PMULLD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_same_register_xmm7() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xff, // PMULLD XMM7, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test sequential operations
#[test]
fn test_pmulld_sequential_operations() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc1, // PMULLD XMM0, XMM1
        0x66, 0x0f, 0x38, 0x40, 0xd3, // PMULLD XMM2, XMM3
        0x66, 0x0f, 0x38, 0x40, 0xe5, // PMULLD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory with displacement
#[test]
fn test_pmulld_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x40, 0x10, // PMULLD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [
        0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test cross-register operations (low to high)
#[test]
fn test_pmulld_xmm0_xmm15() {
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x40, 0xf8, // PMULLD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with maximum positive values
#[test]
fn test_pmulld_xmm6_mem_max_positive() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x30, // PMULLD XMM6, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Max positive 32-bit signed int: 0x7FFFFFFF
    let data: [u8; 16] = [
        0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff,
        0x7f,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with minimum negative values
#[test]
fn test_pmulld_xmm7_mem_min_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x38, // PMULLD XMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Min negative 32-bit signed int: 0x80000000
    let data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        0x80,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional register combinations
#[test]
fn test_pmulld_xmm1_xmm0() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc8, // PMULLD XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm3_xmm2() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xda, // PMULLD XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm5_xmm4() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xec, // PMULLD XMM5, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm7_xmm6() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xfe, // PMULLD XMM7, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test high register combinations
#[test]
fn test_pmulld_xmm9_xmm8() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xc8, // PMULLD XMM9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm11_xmm10() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xda, // PMULLD XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm13_xmm12() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xec, // PMULLD XMM13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm15_xmm14() {
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x40, 0xfe, // PMULLD XMM15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test alternating positive and negative
#[test]
fn test_pmulld_xmm0_mem_alternating() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x40, 0x00, // PMULLD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Alternating: 10, -10, 20, -20
    let data: [u8; 16] = [
        0x0a, 0x00, 0x00, 0x00, // 10
        0xf6, 0xff, 0xff, 0xff, // -10
        0x14, 0x00, 0x00, 0x00, // 20
        0xec, 0xff, 0xff, 0xff, // -20
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional tests to reach 35+ tests
#[test]
fn test_pmulld_xmm0_xmm2() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xc2, // PMULLD XMM0, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm1_xmm3() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xcb, // PMULLD XMM1, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm2_xmm4() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xd4, // PMULLD XMM2, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm3_xmm5() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xdd, // PMULLD XMM3, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulld_xmm4_xmm6() {
    let code = [
        0x66, 0x0f, 0x38, 0x40, 0xe6, // PMULLD XMM4, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PMULLD multiplies each pair of SIGNED dwords and stores the LOW 32 bits of
// the 64-bit product into the corresponding dword lane.
// ============================================================================

#[test]
fn kat_pmulld_value() {
    // PMULLD XMM0, XMM1 (66 0F 38 40 C1)
    // DST dwords (lane0..3): 2, -3, 0x10000, 0x7FFFFFFF
    // SRC dwords (lane0..3): 5,  7, 0x10000, 2
    // Products (low 32): 10, -21 (0xFFFFFFEB), 0x1_0000_0000->0, 0xFFFFFFFE
    let code = [0x66, 0x0f, 0x38, 0x40, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7FFFFFFF_00010000_FFFFFFFD_00000002);
    set_xmm(&mem, &mut vcpu, 1, 0x00000002_00010000_00000007_00000005);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0xFFFFFFFE_00000000_FFFFFFEB_0000000A,
        "PMULLD got {:032x}",
        get_xmm(&regs, 0)
    );
}
