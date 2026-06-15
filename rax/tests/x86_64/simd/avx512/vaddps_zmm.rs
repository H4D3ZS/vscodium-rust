use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VADDPS - Add Packed Single-Precision Floating-Point Values (ZMM)
//
// AVX-512 version using ZMM registers (512-bit / 64 bytes).
// Adds 16 packed single-precision floating-point values (16x f32).
//
// Opcodes (EVEX encoded):
// EVEX.NDS.512.0F.W0 58 /r    VADDPS zmm1 {k1}{z}, zmm2, zmm3/m512/m32bcst
//   - Add packed single from zmm3/m512 to zmm2 and store result in zmm1

const ALIGNED_ADDR: u64 = 0x3000; // 64-byte aligned address for testing

// ============================================================================
// Register-Register-Register Tests - ZMM0-ZMM7
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm1_zmm2() {
    // VADDPS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xf1, 0x74, 0x48, 0x58, 0xc2, // VADDPS ZMM0, ZMM1, ZMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm1_zmm2_zmm3() {
    // VADDPS ZMM1, ZMM2, ZMM3
    let code = [
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM1, ZMM2, ZMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm2_zmm3_zmm4() {
    // VADDPS ZMM2, ZMM3, ZMM4
    let code = [
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xd4, // VADDPS ZMM2, ZMM3, ZMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm3_zmm4_zmm5() {
    // VADDPS ZMM3, ZMM4, ZMM5
    let code = [
        0x62, 0xf1, 0x5c, 0x48, 0x58, 0xdd, // VADDPS ZMM3, ZMM4, ZMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm4_zmm5_zmm6() {
    // VADDPS ZMM4, ZMM5, ZMM6
    let code = [
        0x62, 0xf1, 0x54, 0x48, 0x58, 0xe6, // VADDPS ZMM4, ZMM5, ZMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm5_zmm6_zmm7() {
    // VADDPS ZMM5, ZMM6, ZMM7
    let code = [
        0x62, 0xf1, 0x4c, 0x48, 0x58, 0xef, // VADDPS ZMM5, ZMM6, ZMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm6_zmm7_zmm0() {
    // VADDPS ZMM6, ZMM7, ZMM0
    let code = [
        0x62, 0xf1, 0x44, 0x48, 0x58, 0xf0, // VADDPS ZMM6, ZMM7, ZMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm0_zmm1() {
    // VADDPS ZMM7, ZMM0, ZMM1
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xf9, // VADDPS ZMM7, ZMM0, ZMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM8-ZMM15
// ============================================================================

#[test]
fn test_vaddps_zmm8_zmm9_zmm10() {
    // VADDPS ZMM8, ZMM9, ZMM10
    let code = [
        0x62, 0x51, 0x34, 0x48, 0x58, 0xc2, // VADDPS ZMM8, ZMM9, ZMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm9_zmm10_zmm11() {
    // VADDPS ZMM9, ZMM10, ZMM11
    let code = [
        0x62, 0x51, 0x2c, 0x48, 0x58, 0xcb, // VADDPS ZMM9, ZMM10, ZMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm10_zmm11_zmm12() {
    // VADDPS ZMM10, ZMM11, ZMM12
    let code = [
        0x62, 0x51, 0x24, 0x48, 0x58, 0xd4, // VADDPS ZMM10, ZMM11, ZMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm11_zmm12_zmm13() {
    // VADDPS ZMM11, ZMM12, ZMM13
    let code = [
        0x62, 0x51, 0x1c, 0x48, 0x58, 0xdd, // VADDPS ZMM11, ZMM12, ZMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm12_zmm13_zmm14() {
    // VADDPS ZMM12, ZMM13, ZMM14
    let code = [
        0x62, 0x51, 0x14, 0x48, 0x58, 0xe6, // VADDPS ZMM12, ZMM13, ZMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm13_zmm14_zmm15() {
    // VADDPS ZMM13, ZMM14, ZMM15
    let code = [
        0x62, 0x51, 0x0c, 0x48, 0x58, 0xef, // VADDPS ZMM13, ZMM14, ZMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm14_zmm15_zmm8() {
    // VADDPS ZMM14, ZMM15, ZMM8
    let code = [
        0x62, 0x51, 0x04, 0x48, 0x58, 0xf0, // VADDPS ZMM14, ZMM15, ZMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm8_zmm9() {
    // VADDPS ZMM15, ZMM8, ZMM9
    let code = [
        0x62, 0x51, 0x3c, 0x48, 0x58, 0xf9, // VADDPS ZMM15, ZMM8, ZMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM16-ZMM23
// ============================================================================

#[test]
fn test_vaddps_zmm16_zmm17_zmm18() {
    // VADDPS ZMM16, ZMM17, ZMM18
    let code = [
        0x62, 0xd1, 0x74, 0x48, 0x58, 0xc2, // VADDPS ZMM16, ZMM17, ZMM18
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm17_zmm18_zmm19() {
    // VADDPS ZMM17, ZMM18, ZMM19
    let code = [
        0x62, 0xd1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM17, ZMM18, ZMM19
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm20_zmm21_zmm22() {
    // VADDPS ZMM20, ZMM21, ZMM22
    let code = [
        0x62, 0xd1, 0x54, 0x48, 0x58, 0xe6, // VADDPS ZMM20, ZMM21, ZMM22
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm21_zmm22_zmm23() {
    // VADDPS ZMM21, ZMM22, ZMM23
    let code = [
        0x62, 0xd1, 0x4c, 0x48, 0x58, 0xef, // VADDPS ZMM21, ZMM22, ZMM23
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM24-ZMM31
// ============================================================================

#[test]
fn test_vaddps_zmm24_zmm25_zmm26() {
    // VADDPS ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x91, 0x34, 0x48, 0x58, 0xc2, // VADDPS ZMM24, ZMM25, ZMM26
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm25_zmm26_zmm27() {
    // VADDPS ZMM25, ZMM26, ZMM27
    let code = [
        0x62, 0x91, 0x2c, 0x48, 0x58, 0xcb, // VADDPS ZMM25, ZMM26, ZMM27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm28_zmm29_zmm30() {
    // VADDPS ZMM28, ZMM29, ZMM30
    let code = [
        0x62, 0x91, 0x14, 0x48, 0x58, 0xe6, // VADDPS ZMM28, ZMM29, ZMM30
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm29_zmm30_zmm31() {
    // VADDPS ZMM29, ZMM30, ZMM31
    let code = [
        0x62, 0x91, 0x0c, 0x48, 0x58, 0xef, // VADDPS ZMM29, ZMM30, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm30_zmm31_zmm24() {
    // VADDPS ZMM30, ZMM31, ZMM24
    let code = [
        0x62, 0x91, 0x04, 0x48, 0x58, 0xf0, // VADDPS ZMM30, ZMM31, ZMM24
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm24_zmm25() {
    // VADDPS ZMM31, ZMM24, ZMM25
    let code = [
        0x62, 0x91, 0x5c, 0x48, 0x58, 0xf9, // VADDPS ZMM31, ZMM24, ZMM25
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Cross-range Register Tests
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm15_zmm31() {
    // VADDPS ZMM0, ZMM15, ZMM31
    let code = [
        0x62, 0x71, 0x04, 0x48, 0x58, 0xc7, // VADDPS ZMM0, ZMM15, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm0_zmm15() {
    // VADDPS ZMM31, ZMM0, ZMM15
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x58, 0xff, // VADDPS ZMM31, ZMM0, ZMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm16_zmm24() {
    // VADDPS ZMM7, ZMM16, ZMM24
    let code = [
        0x62, 0xb1, 0x7c, 0x48, 0x58, 0xf8, // VADDPS ZMM7, ZMM16, ZMM24
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm16_zmm31() {
    // VADDPS ZMM15, ZMM16, ZMM31
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x58, 0xff, // VADDPS ZMM15, ZMM16, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Register-Register-Memory Tests
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm1_mem() {
    // VADDPS ZMM0, ZMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x74, 0x48, 0x58, 0x00, // VADDPS ZMM0, ZMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm6_mem() {
    // VADDPS ZMM7, ZMM6, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x4c, 0x48, 0x58, 0x38, // VADDPS ZMM7, ZMM6, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm14_mem() {
    // VADDPS ZMM15, ZMM14, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x0c, 0x48, 0x58, 0x38, // VADDPS ZMM15, ZMM14, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm30_mem() {
    // VADDPS ZMM31, ZMM30, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x0c, 0x48, 0x58, 0x38, // VADDPS ZMM31, ZMM30, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Self-Addition Tests (dest = src1 = src2)
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm0_zmm0() {
    // VADDPS ZMM0, ZMM0, ZMM0 (doubles the value)
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xc0, // VADDPS ZMM0, ZMM0, ZMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm15_zmm15() {
    // VADDPS ZMM15, ZMM15, ZMM15
    let code = [
        0x62, 0x71, 0x04, 0x48, 0x58, 0xff, // VADDPS ZMM15, ZMM15, ZMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm31_zmm31() {
    // VADDPS ZMM31, ZMM31, ZMM31
    let code = [
        0x62, 0x61, 0x04, 0x48, 0x58, 0xff, // VADDPS ZMM31, ZMM31, ZMM31
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Commutative Property Tests
// ============================================================================

#[test]
fn test_vaddps_commutative_zmm1_zmm2_zmm3() {
    // VADDPS ZMM1, ZMM2, ZMM3
    let code = [
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM1, ZMM2, ZMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_commutative_zmm1_zmm3_zmm2() {
    // VADDPS ZMM1, ZMM3, ZMM2 (should give same result as above)
    let code = [
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xca, // VADDPS ZMM1, ZMM3, ZMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Chain Addition Tests
// ============================================================================

#[test]
fn test_vaddps_chain_3_ops() {
    // Chain 3 additions: ZMM3 = (ZMM0 + ZMM1) + ZMM2
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xd9, // VADDPS ZMM3, ZMM0, ZMM1
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xda, // VADDPS ZMM3, ZMM3, ZMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_chain_4_ops() {
    // Chain 4 additions using different registers
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xe1, // VADDPS ZMM4, ZMM0, ZMM1
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xeb, // VADDPS ZMM5, ZMM2, ZMM3
        0x62, 0xf1, 0x5c, 0x48, 0x58, 0xf5, // VADDPS ZMM6, ZMM4, ZMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// All Registers Accumulation Test
// ============================================================================

#[test]
fn test_vaddps_accumulate_all_ranges() {
    // Add values from different register ranges
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xc1, // VADDPS ZMM0, ZMM0, ZMM1
        0x62, 0x51, 0x3c, 0x48, 0x58, 0xc1, // VADDPS ZMM8, ZMM8, ZMM9
        0x62, 0xd1, 0x7c, 0x48, 0x58, 0xc1, // VADDPS ZMM16, ZMM16, ZMM17
        0x62, 0x91, 0x5c, 0x48, 0x58, 0xc1, // VADDPS ZMM24, ZMM24, ZMM25
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory Addressing Mode Tests
// ============================================================================

#[test]
fn test_vaddps_mem_base_displacement() {
    // VADDPS ZMM0, ZMM1, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x40).to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x74, 0x48, 0x58, 0x40, 0x01, // VADDPS ZMM0, ZMM1, [RAX + 0x40]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_mem_with_rbx_base() {
    // VADDPS ZMM2, ZMM3, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x64, 0x48, 0x58, 0x13, // VADDPS ZMM2, ZMM3, [RBX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_mem_with_rcx_base() {
    // VADDPS ZMM4, ZMM5, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x54, 0x48, 0x58, 0x21, // VADDPS ZMM4, ZMM5, [RCX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vaddps_mem_with_rdx_base() {
    // VADDPS ZMM6, ZMM7, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x44, 0x48, 0x58, 0x32, // VADDPS ZMM6, ZMM7, [RDX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(&[0x00u8; 64], GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Regression tests for EVEX FP-arith correctness (vvvv inversion, PS/PD split,
// k-mask merge/zero handling). Values are set/read via VMOVUPS memory
// round-trips, the same pattern as the SSE move tests.
//
// Memory layout used by these tests:
//   0x3000 -> SRC1  (loaded into zmm1)
//   0x3100 -> SRC2  (loaded into zmm2)
//   0x3200 -> RESULT (zmm0 stored here, read back for assertions)
//   0x3300 -> POISON (loaded into zmm14 = the *wrong*, non-inverted vvvv reg)
// ============================================================================

const REG_SRC1: u64 = 0x3000;
const REG_SRC2: u64 = 0x3100;
const REG_RESULT: u64 = 0x3200;
const REG_POISON: u64 = 0x3300;

/// Emit `mov r64, imm` for rax/rbx/rcx/rsi (sign-extended imm32 form, 7 bytes).
fn mov_addrs(out: &mut Vec<u8>) {
    // mov rax, REG_SRC1
    out.extend_from_slice(&[0x48, 0xc7, 0xc0]);
    out.extend_from_slice(&(REG_SRC1 as u32).to_le_bytes());
    // mov rbx, REG_SRC2
    out.extend_from_slice(&[0x48, 0xc7, 0xc3]);
    out.extend_from_slice(&(REG_SRC2 as u32).to_le_bytes());
    // mov rcx, REG_RESULT
    out.extend_from_slice(&[0x48, 0xc7, 0xc1]);
    out.extend_from_slice(&(REG_RESULT as u32).to_le_bytes());
    // mov rsi, REG_POISON
    out.extend_from_slice(&[0x48, 0xc7, 0xc6]);
    out.extend_from_slice(&(REG_POISON as u32).to_le_bytes());
}

fn write_f32_lane(buf: &mut [u8; 64], i: usize, v: f32) {
    buf[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
}

fn read_f32_lane(buf: &[u8; 64], i: usize) -> f32 {
    f32::from_le_bytes([buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]])
}

fn write_f64_lane(buf: &mut [u8; 64], i: usize, v: f64) {
    buf[i * 8..i * 8 + 8].copy_from_slice(&v.to_le_bytes());
}

fn read_f64_lane(buf: &[u8; 64], i: usize) -> f64 {
    f64::from_le_bytes([
        buf[i * 8],
        buf[i * 8 + 1],
        buf[i * 8 + 2],
        buf[i * 8 + 3],
        buf[i * 8 + 4],
        buf[i * 8 + 5],
        buf[i * 8 + 6],
        buf[i * 8 + 7],
    ])
}

// Bug (a): EVEX.vvvv must be inverted to select src1. For `VADDPS zmm0,zmm1,zmm2`,
// the raw (non-inverted) vvvv field is 0b1110 = 14, so a missing inversion would
// read zmm14. We poison zmm14 with a value that would make the result obviously
// wrong, and assert the result equals zmm1 + zmm2.
#[test]
fn test_vaddps_uses_inverted_vvvv_src1() {
    let mut code = Vec::new();
    mov_addrs(&mut code);
    code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x08, // VMOVUPS ZMM1, [RAX]  (src1)
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x13, // VMOVUPS ZMM2, [RBX]  (src2)
        0x62, 0x71, 0x7c, 0x48, 0x10, 0x36, // VMOVUPS ZMM14, [RSI] (poison)
        0x62, 0xf1, 0x74, 0x48, 0x58, 0xc2, // VADDPS ZMM0, ZMM1, ZMM2
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x01, // VMOVUPS [RCX], ZMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&code, None);

    let mut src1 = [0u8; 64];
    let mut src2 = [0u8; 64];
    let mut poison = [0u8; 64];
    for i in 0..16 {
        write_f32_lane(&mut src1, i, (i as f32) + 1.0); // 1..16
        write_f32_lane(&mut src2, i, 100.0); // 100
        write_f32_lane(&mut poison, i, -9999.0); // would corrupt result if used
    }
    mem.write_slice(&src1, GuestAddress(REG_SRC1)).unwrap();
    mem.write_slice(&src2, GuestAddress(REG_SRC2)).unwrap();
    mem.write_slice(&poison, GuestAddress(REG_POISON)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut result = [0u8; 64];
    mem.read_slice(&mut result, GuestAddress(REG_RESULT))
        .unwrap();
    for i in 0..16 {
        let expected = (i as f32) + 1.0 + 100.0; // src1 + src2, NOT poison
        assert_eq!(
            read_f32_lane(&result, i),
            expected,
            "lane {} should use zmm1 (src1) not zmm14",
            i
        );
    }
}

// Bug (b): the 0x58/59/5C/5E opcodes must compute PD (f64) when the operand type
// is double (66 prefix / EVEX.pp=1 with W=1), not 16 packed f32. VADDPD adds 8
// f64 lanes; if mis-dispatched as PS it would garble the high dword of each lane.
#[test]
fn test_vaddpd_computes_f64_not_16xf32() {
    let mut code = Vec::new();
    mov_addrs(&mut code);
    code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x08, // VMOVUPS ZMM1, [RAX]  (src1)
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x13, // VMOVUPS ZMM2, [RBX]  (src2)
        0x62, 0xf1, 0xf5, 0x48, 0x58, 0xc2, // VADDPD ZMM0, ZMM1, ZMM2
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x01, // VMOVUPS [RCX], ZMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&code, None);

    let mut src1 = [0u8; 64];
    let mut src2 = [0u8; 64];
    // Use values whose f64 sum differs from any f32-reinterpreted result. A large
    // exponent ensures the f64 mantissa spans both dwords of the lane.
    for i in 0..8 {
        write_f64_lane(&mut src1, i, 1.0e300 + (i as f64));
        write_f64_lane(&mut src2, i, 2.5e300);
    }
    mem.write_slice(&src1, GuestAddress(REG_SRC1)).unwrap();
    mem.write_slice(&src2, GuestAddress(REG_SRC2)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut result = [0u8; 64];
    mem.read_slice(&mut result, GuestAddress(REG_RESULT))
        .unwrap();
    for i in 0..8 {
        let expected = (1.0e300 + (i as f64)) + 2.5e300;
        assert_eq!(read_f64_lane(&result, i), expected, "f64 lane {}", i);
    }
}

// Bug (c) merge-masking: with k1=0x5 (lanes 0 and 2 active), zeroing NOT set, the
// inactive lanes must keep the prior destination value. We pre-load zmm0 (dest)
// from the poison buffer so inactive lanes can be distinguished from zero.
#[test]
fn test_vaddps_merge_masking() {
    let mut code = Vec::new();
    mov_addrs(&mut code);
    code.extend_from_slice(&[
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5    (k-mask bits 0 and 2)
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x06, // VMOVUPS ZMM0, [RSI] (dest preload)
    ]);
    // mov_addrs set RAX=SRC1, but the 32-bit MOV EAX above clobbered RAX
    // (high bits cleared). Reload RAX before loading src1.
    code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
    code.extend_from_slice(&(REG_SRC1 as u32).to_le_bytes());
    code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x08, // VMOVUPS ZMM1, [RAX] (src1, reloaded)
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x13, // VMOVUPS ZMM2, [RBX] (src2)
        0x62, 0xf1, 0x74, 0x49, 0x58, 0xc2, // VADDPS ZMM0 {K1}, ZMM1, ZMM2 (merge)
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x01, // VMOVUPS [RCX], ZMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&code, None);

    let mut src1 = [0u8; 64];
    let mut src2 = [0u8; 64];
    let mut dest = [0u8; 64];
    for i in 0..16 {
        write_f32_lane(&mut src1, i, 10.0);
        write_f32_lane(&mut src2, i, 20.0);
        write_f32_lane(&mut dest, i, 7.0); // prior dest value for inactive lanes
    }
    mem.write_slice(&src1, GuestAddress(REG_SRC1)).unwrap();
    mem.write_slice(&src2, GuestAddress(REG_SRC2)).unwrap();
    mem.write_slice(&dest, GuestAddress(REG_POISON)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut result = [0u8; 64];
    mem.read_slice(&mut result, GuestAddress(REG_RESULT))
        .unwrap();
    for i in 0..16 {
        let active = (5u64 >> i) & 1 != 0; // lanes 0 and 2
        let expected = if active { 30.0 } else { 7.0 };
        assert_eq!(
            read_f32_lane(&result, i),
            expected,
            "lane {} merge-mask (active={})",
            i,
            active
        );
    }
}

// Bug (c) zeroing-masking: same as above but with {z}; inactive lanes must be 0.
#[test]
fn test_vaddps_zeroing_masking() {
    let mut code = Vec::new();
    mov_addrs(&mut code);
    code.extend_from_slice(&[
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX
        0x62, 0xf1, 0x7c, 0x48, 0x10,
        0x06, // VMOVUPS ZMM0, [RSI] (dest preload, must be zeroed)
    ]);
    // Reload RAX (clobbered by the 32-bit MOV EAX above).
    code.extend_from_slice(&[0x48, 0xc7, 0xc0]);
    code.extend_from_slice(&(REG_SRC1 as u32).to_le_bytes());
    code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x08, // VMOVUPS ZMM1, [RAX] (src1)
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x13, // VMOVUPS ZMM2, [RBX] (src2)
        0x62, 0xf1, 0x74, 0xc9, 0x58, 0xc2, // VADDPS ZMM0 {K1}{z}, ZMM1, ZMM2
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x01, // VMOVUPS [RCX], ZMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&code, None);

    let mut src1 = [0u8; 64];
    let mut src2 = [0u8; 64];
    let mut dest = [0u8; 64];
    for i in 0..16 {
        write_f32_lane(&mut src1, i, 10.0);
        write_f32_lane(&mut src2, i, 20.0);
        write_f32_lane(&mut dest, i, 7.0); // would survive if zeroing were broken
    }
    mem.write_slice(&src1, GuestAddress(REG_SRC1)).unwrap();
    mem.write_slice(&src2, GuestAddress(REG_SRC2)).unwrap();
    mem.write_slice(&dest, GuestAddress(REG_POISON)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();

    let mut result = [0u8; 64];
    mem.read_slice(&mut result, GuestAddress(REG_RESULT))
        .unwrap();
    for i in 0..16 {
        let active = (5u64 >> i) & 1 != 0;
        let expected = if active { 30.0 } else { 0.0 };
        assert_eq!(
            read_f32_lane(&result, i),
            expected,
            "lane {} zero-mask (active={})",
            i,
            active
        );
    }
}
