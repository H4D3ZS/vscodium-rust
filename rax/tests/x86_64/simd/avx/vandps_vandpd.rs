use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VANDPS - Bitwise Logical AND of Packed Single Precision Floating-Point Values
// VANDPD - Bitwise Logical AND of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise AND on packed floating-point values.
//
// Opcodes:
// VEX.128.NP 0F 54 /r    VANDPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 54 /r    VANDPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 54 /r    VANDPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 54 /r    VANDPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VANDPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vandps_xmm_basic() {
    // VANDPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x54, 0xc2, // VANDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_same_dest_src1() {
    // VANDPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x54, 0xca, // VANDPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_all_regs() {
    // VANDPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x54, 0xdd, // VANDPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_high_regs() {
    // VANDPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x54, 0xf2, // VANDPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_extended_dest() {
    // VANDPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x54, 0xc2, // VANDPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_extended_src1() {
    // VANDPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x54, 0xca, // VANDPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_extended_src2() {
    // VANDPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x54, 0xca, // VANDPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_all_extended() {
    // VANDPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x54, 0xdd, // VANDPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_r14_r15_r8() {
    // VANDPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x54, 0xf0, // VANDPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_mem() {
    // VANDPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0xaa,
        0xaa,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_xmm_mem_extended() {
    // VANDPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x54, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDPS XMM10, XMM11, [rip+0x4000]
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

// ============================================================================
// VANDPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vandps_ymm_basic() {
    // VANDPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x54, 0xc2, // VANDPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_same_dest_src1() {
    // VANDPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x54, 0xca, // VANDPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_all_regs() {
    // VANDPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x54, 0xdd, // VANDPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_high_regs() {
    // VANDPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x54, 0xf2, // VANDPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_extended_dest() {
    // VANDPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x54, 0xc2, // VANDPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_extended_src1() {
    // VANDPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x54, 0xca, // VANDPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_extended_src2() {
    // VANDPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x54, 0xca, // VANDPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_all_extended() {
    // VANDPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x54, 0xdd, // VANDPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_r14_r15_r8() {
    // VANDPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x54, 0xf0, // VANDPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_mem() {
    // VANDPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0xaa,
        0xaa, 0x55, 0x55, 0x55, 0x55, 0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45,
        0x67, 0x89,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandps_ymm_mem_extended() {
    // VANDPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x54, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDPS YMM10, YMM11, [rip+0x4000]
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

// ============================================================================
// VANDPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vandpd_xmm_basic() {
    // VANDPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x54, 0xc2, // VANDPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_same_dest_src1() {
    // VANDPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x54, 0xca, // VANDPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_all_regs() {
    // VANDPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x54, 0xdd, // VANDPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_high_regs() {
    // VANDPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x54, 0xf2, // VANDPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_dest() {
    // VANDPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x54, 0xc2, // VANDPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_src1() {
    // VANDPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x54, 0xca, // VANDPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_src2() {
    // VANDPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x54, 0xca, // VANDPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_all_extended() {
    // VANDPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x54, 0xdd, // VANDPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_r14_r15_r8() {
    // VANDPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x54, 0xf0, // VANDPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_mem() {
    // VANDPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_xmm_mem_extended() {
    // VANDPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x54, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDPD XMM10, XMM11, [rip+0x4000]
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

// ============================================================================
// VANDPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vandpd_ymm_basic() {
    // VANDPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x54, 0xc2, // VANDPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_same_dest_src1() {
    // VANDPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x54, 0xca, // VANDPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_all_regs() {
    // VANDPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x54, 0xdd, // VANDPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_high_regs() {
    // VANDPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x54, 0xf2, // VANDPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_dest() {
    // VANDPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x54, 0xc2, // VANDPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_src1() {
    // VANDPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x54, 0xca, // VANDPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_src2() {
    // VANDPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x54, 0xca, // VANDPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_all_extended() {
    // VANDPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x54, 0xdd, // VANDPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_r14_r15_r8() {
    // VANDPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x54, 0xf0, // VANDPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_mem() {
    // VANDPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, 0x55,
    ];
    mem.write_slice(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandpd_ymm_mem_extended() {
    // VANDPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x54, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDPD YMM10, YMM11, [rip+0x4000]
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

// ============================================================================
// Known-answer VALUE tests (bitwise AND of exact bit patterns).
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kav_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kav_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kav_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

#[test]
fn test_vandps_xmm_value() {
    // VANDPS XMM0, XMM1, XMM2 : 128-bit AND, upper 128 must be zeroed.
    let code = [0xc5, 0xf0, 0x54, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kav_set(
        &mut vcpu,
        1,
        0xF0F0_F0F0_FFFF_0000_AAAA_5555_FFFF_FFFF,
        0xDEAD,
    );
    kav_set(
        &mut vcpu,
        2,
        0x0FF0_0FF0_0F0F_F0F0_FFFF_0000_5555_AAAA,
        0xBEEF,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kav_lo(&vcpu, 0),
        0xF0F0_F0F0_FFFF_0000_AAAA_5555_FFFF_FFFF & 0x0FF0_0FF0_0F0F_F0F0_FFFF_0000_5555_AAAA
    );
    assert_eq!(kav_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vandps_ymm_value() {
    // VANDPS YMM0, YMM1, YMM2 : both 128-bit lanes ANDed independently.
    let code = [0xc5, 0xf4, 0x54, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kav_set(
        &mut vcpu,
        1,
        0xFFFF_FFFF_0000_0000_FF00_FF00_AAAA_AAAA,
        0x1234_5678_9ABC_DEF0_0F0F_0F0F_F0F0_F0F0,
    );
    kav_set(
        &mut vcpu,
        2,
        0x0F0F_0F0F_FFFF_FFFF_0F0F_0F0F_FFFF_0000,
        0xFFFF_0000_FFFF_0000_FFFF_FFFF_0000_0000,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kav_lo(&vcpu, 0),
        0xFFFF_FFFF_0000_0000_FF00_FF00_AAAA_AAAA & 0x0F0F_0F0F_FFFF_FFFF_0F0F_0F0F_FFFF_0000
    );
    assert_eq!(
        kav_hi(&vcpu, 0),
        0x1234_5678_9ABC_DEF0_0F0F_0F0F_F0F0_F0F0 & 0xFFFF_0000_FFFF_0000_FFFF_FFFF_0000_0000
    );
}

#[test]
fn test_vandpd_xmm_value() {
    // VANDPD XMM0, XMM1, XMM2 : identical bitwise behavior, upper 128 zeroed.
    let code = [0xc5, 0xf1, 0x54, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kav_set(
        &mut vcpu,
        1,
        0xCCCC_CCCC_CCCC_CCCC_3333_3333_3333_3333,
        0x99,
    );
    kav_set(
        &mut vcpu,
        2,
        0xCC33_CC33_CC33_CC33_3333_FFFF_0000_3333,
        0x77,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kav_lo(&vcpu, 0),
        0xCCCC_CCCC_CCCC_CCCC_3333_3333_3333_3333 & 0xCC33_CC33_CC33_CC33_3333_FFFF_0000_3333
    );
    assert_eq!(kav_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vandpd_ymm_value() {
    // VANDPD YMM0, YMM1, YMM2 : both lanes ANDed.
    let code = [0xc5, 0xf5, 0x54, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kav_set(
        &mut vcpu,
        1,
        0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0000,
        0xAAAA_AAAA_AAAA_AAAA_5555_5555_5555_5555,
    );
    kav_set(
        &mut vcpu,
        2,
        0x0F0F_F0F0_0F0F_F0F0_FFFF_FFFF_FFFF_FFFF,
        0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000,
    );
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        kav_lo(&vcpu, 0),
        0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0000 & 0x0F0F_F0F0_0F0F_F0F0_FFFF_FFFF_FFFF_FFFF
    );
    assert_eq!(
        kav_hi(&vcpu, 0),
        0xAAAA_AAAA_AAAA_AAAA_5555_5555_5555_5555 & 0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000
    );
}
