use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VHSUBPS - Packed Single Precision Floating-Point Horizontal Subtract
// VHSUBPD - Packed Double Precision Floating-Point Horizontal Subtract
//
// These instructions perform horizontal subtraction on packed floating-point values.
// For VHSUBPS: subtracts adjacent pairs of single-precision values
// For VHSUBPD: subtracts adjacent pairs of double-precision values
//
// Operation:
// VHSUBPS xmm1, xmm2, xmm3:
//   xmm1[31:0]   = xmm2[31:0]   - xmm2[63:32]
//   xmm1[63:32]  = xmm2[95:64]  - xmm2[127:96]
//   xmm1[95:64]  = xmm3[31:0]   - xmm3[63:32]
//   xmm1[127:96] = xmm3[95:64]  - xmm3[127:96]
//
// VHSUBPD xmm1, xmm2, xmm3:
//   xmm1[63:0]   = xmm2[63:0]   - xmm2[127:64]
//   xmm1[127:64] = xmm3[63:0]   - xmm3[127:64]
//
// Opcodes:
// VEX.128.F2 0F 7D /r    VHSUBPS xmm1, xmm2, xmm3/m128
// VEX.256.F2 0F 7D /r    VHSUBPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 7D /r    VHSUBPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 7D /r    VHSUBPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VHSUBPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vhsubps_xmm_basic() {
    // VHSUBPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7d, 0xc2, // VHSUBPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_same_dest_src1() {
    // VHSUBPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7d, 0xca, // VHSUBPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_all_regs() {
    // VHSUBPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x7d, 0xdd, // VHSUBPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_high_regs() {
    // VHSUBPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc3, 0x7d, 0xf2, // VHSUBPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_extended_dest() {
    // VHSUBPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x73, 0x7d, 0xc2, // VHSUBPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_extended_src1() {
    // VHSUBPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x33, 0x7d, 0xca, // VHSUBPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_extended_src2() {
    // VHSUBPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x6b, 0x7d, 0xca, // VHSUBPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_all_extended() {
    // VHSUBPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x1b, 0x7d, 0xdd, // VHSUBPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_r14_r15_r8() {
    // VHSUBPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x03, 0x7d, 0xf0, // VHSUBPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_mem() {
    // VHSUBPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xfb, 0x7d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHSUBPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_mem_extended() {
    // VHSUBPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x23, 0x7d, 0x15, 0x00, 0x40, 0x00,
        0x00, // VHSUBPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0f
        0x00, 0x00, 0xc0, 0x40, // 6.0f
        0x00, 0x00, 0xe0, 0x40, // 7.0f
        0x00, 0x00, 0x00, 0x41, // 8.0f
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_self() {
    // VHSUBPS XMM0, XMM0, XMM0 (should produce zeros)
    let code = [
        0xc5, 0xfb, 0x7d, 0xc0, // VHSUBPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_xmm_different_operands() {
    // VHSUBPS XMM5, XMM3, XMM7
    let code = [
        0xc5, 0xe3, 0x7d, 0xef, // VHSUBPS XMM5, XMM3, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VHSUBPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vhsubps_ymm_basic() {
    // VHSUBPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7d, 0xc2, // VHSUBPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_same_dest_src1() {
    // VHSUBPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7d, 0xca, // VHSUBPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_all_regs() {
    // VHSUBPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdf, 0x7d, 0xdd, // VHSUBPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_high_regs() {
    // VHSUBPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc7, 0x7d, 0xf2, // VHSUBPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_extended_dest() {
    // VHSUBPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x77, 0x7d, 0xc2, // VHSUBPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_extended_src1() {
    // VHSUBPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x37, 0x7d, 0xca, // VHSUBPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_extended_src2() {
    // VHSUBPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6f, 0x7d, 0xca, // VHSUBPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_all_extended() {
    // VHSUBPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1f, 0x7d, 0xdd, // VHSUBPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_r14_r15_r8() {
    // VHSUBPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x07, 0x7d, 0xf0, // VHSUBPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_mem() {
    // VHSUBPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xff, 0x7d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHSUBPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
        0x00, 0x00, 0xa0, 0x40, // 5.0f
        0x00, 0x00, 0xc0, 0x40, // 6.0f
        0x00, 0x00, 0xe0, 0x40, // 7.0f
        0x00, 0x00, 0x00, 0x41, // 8.0f
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_mem_extended() {
    // VHSUBPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x27, 0x7d, 0x15, 0x00, 0x40, 0x00,
        0x00, // VHSUBPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_self() {
    // VHSUBPS YMM5, YMM5, YMM5 (should produce zeros)
    let code = [
        0xc5, 0xd7, 0x7d, 0xed, // VHSUBPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubps_ymm_different_operands() {
    // VHSUBPS YMM4, YMM2, YMM6
    let code = [
        0xc5, 0xef, 0x7d, 0xe6, // VHSUBPS YMM4, YMM2, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VHSUBPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vhsubpd_xmm_basic() {
    // VHSUBPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7d, 0xc2, // VHSUBPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_same_dest_src1() {
    // VHSUBPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7d, 0xca, // VHSUBPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_all_regs() {
    // VHSUBPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x7d, 0xdd, // VHSUBPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_high_regs() {
    // VHSUBPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc3, 0x7d, 0xf2, // VHSUBPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_extended_dest() {
    // VHSUBPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x73, 0x7d, 0xc2, // VHSUBPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_extended_src1() {
    // VHSUBPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x33, 0x7d, 0xca, // VHSUBPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_extended_src2() {
    // VHSUBPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x6b, 0x7d, 0xca, // VHSUBPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_all_extended() {
    // VHSUBPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x1b, 0x7d, 0xdd, // VHSUBPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_r14_r15_r8() {
    // VHSUBPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x03, 0x7d, 0xf0, // VHSUBPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_mem() {
    // VHSUBPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xfb, 0x7d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHSUBPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_mem_extended() {
    // VHSUBPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x23, 0x7d, 0x15, 0x00, 0x40, 0x00,
        0x00, // VHSUBPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_self() {
    // VHSUBPD XMM2, XMM2, XMM2 (should produce zeros)
    let code = [
        0xc5, 0xeb, 0x7d, 0xd2, // VHSUBPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_xmm_different_operands() {
    // VHSUBPD XMM7, XMM1, XMM4
    let code = [
        0xc5, 0xf3, 0x7d, 0xfc, // VHSUBPD XMM7, XMM1, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VHSUBPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vhsubpd_ymm_basic() {
    // VHSUBPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7d, 0xc2, // VHSUBPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_same_dest_src1() {
    // VHSUBPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7d, 0xca, // VHSUBPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_all_regs() {
    // VHSUBPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdf, 0x7d, 0xdd, // VHSUBPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_high_regs() {
    // VHSUBPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc7, 0x7d, 0xf2, // VHSUBPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_extended_dest() {
    // VHSUBPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x77, 0x7d, 0xc2, // VHSUBPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_extended_src1() {
    // VHSUBPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x37, 0x7d, 0xca, // VHSUBPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_extended_src2() {
    // VHSUBPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6f, 0x7d, 0xca, // VHSUBPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_all_extended() {
    // VHSUBPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1f, 0x7d, 0xdd, // VHSUBPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_r14_r15_r8() {
    // VHSUBPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x07, 0x7d, 0xf0, // VHSUBPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_mem() {
    // VHSUBPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xff, 0x7d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHSUBPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_mem_extended() {
    // VHSUBPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x27, 0x7d, 0x15, 0x00, 0x40, 0x00,
        0x00, // VHSUBPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_self() {
    // VHSUBPD YMM7, YMM7, YMM7 (should produce zeros)
    let code = [
        0xc5, 0xc7, 0x7d, 0xff, // VHSUBPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vhsubpd_ymm_different_operands() {
    // VHSUBPD YMM6, YMM3, YMM1
    let code = [
        0xc5, 0xe7, 0x7d, 0xf1, // VHSUBPD YMM6, YMM3, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
