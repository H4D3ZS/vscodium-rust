use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VORPS - Bitwise Logical OR of Packed Single Precision Floating-Point Values
// VORPD - Bitwise Logical OR of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise OR on packed floating-point values.
//
// Opcodes:
// VEX.128.NP 0F 56 /r    VORPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 56 /r    VORPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 56 /r    VORPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 56 /r    VORPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VORPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vorps_xmm_basic() {
    // VORPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x56, 0xc2, // VORPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_same_dest_src1() {
    // VORPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x56, 0xca, // VORPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_all_regs() {
    // VORPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x56, 0xdd, // VORPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_high_regs() {
    // VORPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x56, 0xf2, // VORPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_extended_dest() {
    // VORPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x56, 0xc2, // VORPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_extended_src1() {
    // VORPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x56, 0xca, // VORPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_extended_src2() {
    // VORPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x56, 0xca, // VORPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_all_extended() {
    // VORPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x56, 0xdd, // VORPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_r14_r15_r8() {
    // VORPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x56, 0xf0, // VORPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_mem() {
    // VORPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xaa, 0x55, 0xaa, 0x55, 0x33, 0xcc, 0x33,
        0xcc,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_mem_extended() {
    // VORPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x56, 0x15, 0x00, 0x40, 0x00,
        0x00, // VORPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_xmm_self() {
    // VORPS XMM0, XMM0, XMM0 (self OR should produce same value)
    let code = [
        0xc5, 0xf8, 0x56, 0xc0, // VORPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VORPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vorps_ymm_basic() {
    // VORPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x56, 0xc2, // VORPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_same_dest_src1() {
    // VORPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x56, 0xca, // VORPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_all_regs() {
    // VORPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x56, 0xdd, // VORPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_high_regs() {
    // VORPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x56, 0xf2, // VORPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_extended_dest() {
    // VORPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x56, 0xc2, // VORPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_extended_src1() {
    // VORPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x56, 0xca, // VORPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_extended_src2() {
    // VORPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x56, 0xca, // VORPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_all_extended() {
    // VORPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x56, 0xdd, // VORPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_r14_r15_r8() {
    // VORPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x56, 0xf0, // VORPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_mem() {
    // VORPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xaa, 0x55, 0xaa, 0x55, 0x33, 0xcc, 0x33,
        0xcc, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff, 0x00,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_mem_extended() {
    // VORPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x56, 0x15, 0x00, 0x40, 0x00,
        0x00, // VORPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, 0x55,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorps_ymm_self() {
    // VORPS YMM5, YMM5, YMM5 (self OR should produce same value)
    let code = [
        0xc5, 0xd4, 0x56, 0xed, // VORPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VORPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vorpd_xmm_basic() {
    // VORPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x56, 0xc2, // VORPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_same_dest_src1() {
    // VORPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x56, 0xca, // VORPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_all_regs() {
    // VORPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x56, 0xdd, // VORPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_high_regs() {
    // VORPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x56, 0xf2, // VORPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_dest() {
    // VORPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x56, 0xc2, // VORPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_src1() {
    // VORPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x56, 0xca, // VORPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_src2() {
    // VORPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x56, 0xca, // VORPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_all_extended() {
    // VORPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x56, 0xdd, // VORPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_r14_r15_r8() {
    // VORPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x56, 0xf0, // VORPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_mem() {
    // VORPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
        0xff,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_mem_extended() {
    // VORPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x56, 0x15, 0x00, 0x40, 0x00,
        0x00, // VORPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_xmm_self() {
    // VORPD XMM2, XMM2, XMM2 (self OR should produce same value)
    let code = [
        0xc5, 0xe9, 0x56, 0xd2, // VORPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VORPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vorpd_ymm_basic() {
    // VORPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x56, 0xc2, // VORPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_same_dest_src1() {
    // VORPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x56, 0xca, // VORPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_all_regs() {
    // VORPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x56, 0xdd, // VORPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_high_regs() {
    // VORPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x56, 0xf2, // VORPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_dest() {
    // VORPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x56, 0xc2, // VORPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_src1() {
    // VORPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x56, 0xca, // VORPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_src2() {
    // VORPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x56, 0xca, // VORPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_all_extended() {
    // VORPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x56, 0xdd, // VORPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_r14_r15_r8() {
    // VORPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x56, 0xf0, // VORPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_mem() {
    // VORPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
        0xff, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa,
        0x55, 0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_mem_extended() {
    // VORPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x56, 0x15, 0x00, 0x40, 0x00,
        0x00, // VORPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vorpd_ymm_self() {
    // VORPD YMM7, YMM7, YMM7 (self OR should produce same value)
    let code = [
        0xc5, 0xc5, 0x56, 0xff, // VORPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests (bitwise OR of exact bit patterns).
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kov_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kov_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kov_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

#[test]
fn test_vorps_xmm_value() {
    // VORPS XMM0, XMM1, XMM2 ; 128-bit OR, upper 128 zeroed.
    let code = [0xc5, 0xf0, 0x56, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kov_set(
        &mut vcpu,
        1,
        0xF0F0_0000_AAAA_5555_0000_FFFF_1234_5678,
        0xDEAD,
    );
    kov_set(
        &mut vcpu,
        2,
        0x0F0F_FFFF_5555_AAAA_FFFF_0000_8765_4321,
        0xBEEF,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kov_lo(&vcpu, 0),
        0xF0F0_0000_AAAA_5555_0000_FFFF_1234_5678 | 0x0F0F_FFFF_5555_AAAA_FFFF_0000_8765_4321
    );
    assert_eq!(kov_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vorps_ymm_value() {
    // VORPS YMM0, YMM1, YMM2 ; both lanes ORed.
    let code = [0xc5, 0xf4, 0x56, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kov_set(
        &mut vcpu,
        1,
        0x0000_0000_FFFF_FFFF_AAAA_0000_0000_5555,
        0x1111_2222_3333_4444_0F0F_0F0F_F0F0_F0F0,
    );
    kov_set(
        &mut vcpu,
        2,
        0xFFFF_0000_0000_FFFF_0000_AAAA_5555_0000,
        0x0000_FFFF_0000_FFFF_F0F0_F0F0_0F0F_0F0F,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kov_lo(&vcpu, 0),
        0x0000_0000_FFFF_FFFF_AAAA_0000_0000_5555 | 0xFFFF_0000_0000_FFFF_0000_AAAA_5555_0000
    );
    assert_eq!(
        kov_hi(&vcpu, 0),
        0x1111_2222_3333_4444_0F0F_0F0F_F0F0_F0F0 | 0x0000_FFFF_0000_FFFF_F0F0_F0F0_0F0F_0F0F
    );
}

#[test]
fn test_vorpd_ymm_value() {
    // VORPD YMM0, YMM1, YMM2 ; both lanes ORed.
    let code = [0xc5, 0xf5, 0x56, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kov_set(
        &mut vcpu,
        1,
        0xFF00_FF00_FF00_FF00_0000_0000_0000_0000,
        0x5555_5555_5555_5555_AAAA_AAAA_AAAA_AAAA,
    );
    kov_set(
        &mut vcpu,
        2,
        0x00FF_00FF_00FF_00FF_DEAD_BEEF_CAFE_BABE,
        0xAAAA_AAAA_AAAA_AAAA_5555_5555_5555_5555,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kov_lo(&vcpu, 0),
        0xFF00_FF00_FF00_FF00_0000_0000_0000_0000 | 0x00FF_00FF_00FF_00FF_DEAD_BEEF_CAFE_BABE
    );
    // 0x5555... | 0xAAAA... == all ones in both halves.
    assert_eq!(kov_hi(&vcpu, 0), u128::MAX);
}
