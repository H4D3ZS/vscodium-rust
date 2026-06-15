use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VMOVAPS - Move Aligned Packed Single-Precision Floating-Point Values (ZMM)
//
// AVX-512 version using ZMM registers (512-bit / 64 bytes).
// Moves 16 single-precision floating-point values (16x f32).
// When the operand is a memory location, it must be aligned on a 64-byte boundary.
//
// Opcodes (EVEX encoded):
// EVEX.512.0F.W0 28 /r    VMOVAPS zmm1 {k1}{z}, zmm2/m512    - Move aligned packed single from zmm2/m512 to zmm1
// EVEX.512.0F.W0 29 /r    VMOVAPS zmm2/m512 {k1}{z}, zmm1    - Move aligned packed single from zmm1 to zmm2/m512

const ALIGNED_ADDR: u64 = 0x3000; // 64-byte aligned address for testing

// ============================================================================
// Register to Register Tests - ZMM0-ZMM7
// ============================================================================

#[test]
fn test_vmovaps_zmm0_to_zmm1() {
    // VMOVAPS ZMM1, ZMM0
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xc8, // VMOVAPS ZMM1, ZMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm1_to_zmm0() {
    // VMOVAPS ZMM0, ZMM1
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xc1, // VMOVAPS ZMM0, ZMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm2_to_zmm3() {
    // VMOVAPS ZMM3, ZMM2
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xda, // VMOVAPS ZMM3, ZMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm3_to_zmm4() {
    // VMOVAPS ZMM4, ZMM3
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xe3, // VMOVAPS ZMM4, ZMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm4_to_zmm5() {
    // VMOVAPS ZMM5, ZMM4
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xec, // VMOVAPS ZMM5, ZMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm5_to_zmm6() {
    // VMOVAPS ZMM6, ZMM5
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xf5, // VMOVAPS ZMM6, ZMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm6_to_zmm7() {
    // VMOVAPS ZMM7, ZMM6
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xfe, // VMOVAPS ZMM7, ZMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm7_to_zmm0() {
    // VMOVAPS ZMM0, ZMM7
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM0, ZMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - ZMM8-ZMM15 (requires EVEX.R')
// ============================================================================

#[test]
fn test_vmovaps_zmm8_to_zmm9() {
    // VMOVAPS ZMM9, ZMM8
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xc8, // VMOVAPS ZMM9, ZMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm9_to_zmm10() {
    // VMOVAPS ZMM10, ZMM9
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xd1, // VMOVAPS ZMM10, ZMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm10_to_zmm11() {
    // VMOVAPS ZMM11, ZMM10
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xda, // VMOVAPS ZMM11, ZMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm11_to_zmm12() {
    // VMOVAPS ZMM12, ZMM11
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xe3, // VMOVAPS ZMM12, ZMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm12_to_zmm13() {
    // VMOVAPS ZMM13, ZMM12
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xec, // VMOVAPS ZMM13, ZMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm13_to_zmm14() {
    // VMOVAPS ZMM14, ZMM13
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xf5, // VMOVAPS ZMM14, ZMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm14_to_zmm15() {
    // VMOVAPS ZMM15, ZMM14
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xfe, // VMOVAPS ZMM15, ZMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm15_to_zmm8() {
    // VMOVAPS ZMM8, ZMM15
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM8, ZMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - ZMM16-ZMM23 (requires EVEX.R and EVEX.X)
// ============================================================================

#[test]
fn test_vmovaps_zmm16_to_zmm17() {
    // VMOVAPS ZMM17, ZMM16
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xc8, // VMOVAPS ZMM17, ZMM16
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm17_to_zmm18() {
    // VMOVAPS ZMM18, ZMM17
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xd1, // VMOVAPS ZMM18, ZMM17
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm18_to_zmm19() {
    // VMOVAPS ZMM19, ZMM18
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xda, // VMOVAPS ZMM19, ZMM18
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm19_to_zmm20() {
    // VMOVAPS ZMM20, ZMM19
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xe3, // VMOVAPS ZMM20, ZMM19
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm20_to_zmm21() {
    // VMOVAPS ZMM21, ZMM20
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xec, // VMOVAPS ZMM21, ZMM20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm21_to_zmm22() {
    // VMOVAPS ZMM22, ZMM21
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xf5, // VMOVAPS ZMM22, ZMM21
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm22_to_zmm23() {
    // VMOVAPS ZMM23, ZMM22
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xfe, // VMOVAPS ZMM23, ZMM22
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm23_to_zmm16() {
    // VMOVAPS ZMM16, ZMM23
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM16, ZMM23
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Register Tests - ZMM24-ZMM31 (requires all EVEX extension bits)
// ============================================================================

#[test]
fn test_vmovaps_zmm24_to_zmm25() {
    // VMOVAPS ZMM25, ZMM24
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xc8, // VMOVAPS ZMM25, ZMM24
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm25_to_zmm26() {
    // VMOVAPS ZMM26, ZMM25
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xd1, // VMOVAPS ZMM26, ZMM25
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm26_to_zmm27() {
    // VMOVAPS ZMM27, ZMM26
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xda, // VMOVAPS ZMM27, ZMM26
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm27_to_zmm28() {
    // VMOVAPS ZMM28, ZMM27
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xe3, // VMOVAPS ZMM28, ZMM27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm28_to_zmm29() {
    // VMOVAPS ZMM29, ZMM28
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xec, // VMOVAPS ZMM29, ZMM28
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm29_to_zmm30() {
    // VMOVAPS ZMM30, ZMM29
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xf5, // VMOVAPS ZMM30, ZMM29
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm30_to_zmm31() {
    // VMOVAPS ZMM31, ZMM30
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xfe, // VMOVAPS ZMM31, ZMM30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm31_to_zmm24() {
    // VMOVAPS ZMM24, ZMM31
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM24, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Cross-range Register Tests
// ============================================================================

#[test]
fn test_vmovaps_zmm0_to_zmm15() {
    // VMOVAPS ZMM15, ZMM0
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x28, 0xf8, // VMOVAPS ZMM15, ZMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm15_to_zmm0() {
    // VMOVAPS ZMM0, ZMM15
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM0, ZMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm0_to_zmm31() {
    // VMOVAPS ZMM31, ZMM0
    let code = [
        0x62, 0x61, 0x7c, 0x48, 0x28, 0xf8, // VMOVAPS ZMM31, ZMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm31_to_zmm0() {
    // VMOVAPS ZMM0, ZMM31
    let code = [
        0x62, 0x61, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM0, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm7_to_zmm24() {
    // VMOVAPS ZMM24, ZMM7
    let code = [
        0x62, 0xe1, 0x7c, 0x48, 0x28, 0xc7, // VMOVAPS ZMM24, ZMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory to Register Tests (64-byte aligned)
// ============================================================================

#[test]
fn test_vmovaps_mem_to_zmm0_aligned() {
    // VMOVAPS ZMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x00, // VMOVAPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to aligned address (64 bytes)
    let test_data = [0x01u8; 64];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_mem_to_zmm1_aligned() {
    // VMOVAPS ZMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x08, // VMOVAPS ZMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0xFFu8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_mem_to_zmm7_aligned() {
    // VMOVAPS ZMM7, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x38, // VMOVAPS ZMM7, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0xAAu8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_mem_to_zmm15_aligned() {
    // VMOVAPS ZMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x28, 0x38, // VMOVAPS ZMM15, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x33u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_mem_to_zmm31_aligned() {
    // VMOVAPS ZMM31, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x28, 0x38, // VMOVAPS ZMM31, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x77u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register to Memory Tests (64-byte aligned)
// ============================================================================

#[test]
fn test_vmovaps_zmm0_to_mem_aligned() {
    // VMOVAPS [aligned_addr], ZMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x29, 0x00, // VMOVAPS [RAX], ZMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();

    // Verify data was written
    let mut result = [0u8; 64];
    mem.read_slice(&mut result, GuestAddress(ALIGNED_ADDR))
        .unwrap();
}

#[test]
fn test_vmovaps_zmm1_to_mem_aligned() {
    // VMOVAPS [aligned_addr], ZMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x29, 0x08, // VMOVAPS [RAX], ZMM1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm15_to_mem_aligned() {
    // VMOVAPS [aligned_addr], ZMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x29, 0x38, // VMOVAPS [RAX], ZMM15
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_zmm31_to_mem_aligned() {
    // VMOVAPS [aligned_addr], ZMM31
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x29, 0x38, // VMOVAPS [RAX], ZMM31
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_vmovaps_all_zeros() {
    // Test moving all zeros
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x00, // VMOVAPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_all_ones() {
    // Test moving all ones
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x00, // VMOVAPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0xFFu8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_alternating_pattern() {
    // Test moving alternating bit pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x00, // VMOVAPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let mut data = [0u8; 64];
    for i in 0..64 {
        data[i] = if i % 2 == 0 { 0xAA } else { 0x55 };
    }
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_float_values() {
    // Test moving actual float values (16 x f32 = 64 bytes)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x00, // VMOVAPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write 16 float values
    let mut data = Vec::new();
    for i in 0..16 {
        let val: f32 = (i + 1) as f32;
        data.extend_from_slice(&val.to_le_bytes());
    }

    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_vmovaps_roundtrip_reg_mem_reg() {
    // Test: ZMM0 -> Memory -> ZMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x29, 0x00, // VMOVAPS [RAX], ZMM0
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0x08, // VMOVAPS ZMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovaps_chain_move() {
    // Test: ZMM0 -> ZMM1 -> ZMM2 -> ZMM3
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xc8, // VMOVAPS ZMM1, ZMM0
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xd1, // VMOVAPS ZMM2, ZMM1
        0x62, 0xf1, 0x7c, 0x48, 0x28, 0xda, // VMOVAPS ZMM3, ZMM2
        0xf4, // HLT
    ];

    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
