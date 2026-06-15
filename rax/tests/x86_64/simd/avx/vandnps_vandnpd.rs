use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VANDNPS - Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values
// VANDNPD - Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise AND NOT on packed floating-point values.
// The operation is: dest = NOT(src1) AND src2
//
// Opcodes:
// VEX.128.NP 0F 55 /r    VANDNPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 55 /r    VANDNPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 55 /r    VANDNPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 55 /r    VANDNPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VANDNPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vandnps_xmm_basic() {
    // VANDNPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x55, 0xc2, // VANDNPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_same_dest_src1() {
    // VANDNPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x55, 0xca, // VANDNPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_all_regs() {
    // VANDNPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x55, 0xdd, // VANDNPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_high_regs() {
    // VANDNPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x55, 0xf2, // VANDNPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_dest() {
    // VANDNPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x55, 0xc2, // VANDNPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_src1() {
    // VANDNPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x55, 0xca, // VANDNPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_src2() {
    // VANDNPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x55, 0xca, // VANDNPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_all_extended() {
    // VANDNPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x55, 0xdd, // VANDNPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_r14_r15_r8() {
    // VANDNPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x55, 0xf0, // VANDNPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_mem() {
    // VANDNPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPS XMM1, XMM0, [rip+0x4000]
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
fn test_vandnps_xmm_mem_extended() {
    // VANDNPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x55, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDNPS XMM10, XMM11, [rip+0x4000]
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
fn test_vandnps_xmm_self() {
    // VANDNPS XMM0, XMM0, XMM0 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xf8, 0x55, 0xc0, // VANDNPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_xmm_different_operands() {
    // VANDNPS XMM5, XMM3, XMM7
    let code = [
        0xc5, 0xe0, 0x55, 0xef, // VANDNPS XMM5, XMM3, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VANDNPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vandnps_ymm_basic() {
    // VANDNPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x55, 0xc2, // VANDNPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_same_dest_src1() {
    // VANDNPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x55, 0xca, // VANDNPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_all_regs() {
    // VANDNPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x55, 0xdd, // VANDNPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_high_regs() {
    // VANDNPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x55, 0xf2, // VANDNPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_dest() {
    // VANDNPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x55, 0xc2, // VANDNPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_src1() {
    // VANDNPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x55, 0xca, // VANDNPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_src2() {
    // VANDNPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x55, 0xca, // VANDNPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_all_extended() {
    // VANDNPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x55, 0xdd, // VANDNPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_r14_r15_r8() {
    // VANDNPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x55, 0xf0, // VANDNPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_mem() {
    // VANDNPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPS YMM1, YMM0, [rip+0x4000]
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
fn test_vandnps_ymm_mem_extended() {
    // VANDNPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x55, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDNPS YMM10, YMM11, [rip+0x4000]
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
fn test_vandnps_ymm_self() {
    // VANDNPS YMM5, YMM5, YMM5 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xd4, 0x55, 0xed, // VANDNPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnps_ymm_different_operands() {
    // VANDNPS YMM4, YMM2, YMM6
    let code = [
        0xc5, 0xec, 0x55, 0xe6, // VANDNPS YMM4, YMM2, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VANDNPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vandnpd_xmm_basic() {
    // VANDNPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x55, 0xc2, // VANDNPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_same_dest_src1() {
    // VANDNPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x55, 0xca, // VANDNPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_all_regs() {
    // VANDNPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x55, 0xdd, // VANDNPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_high_regs() {
    // VANDNPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x55, 0xf2, // VANDNPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_dest() {
    // VANDNPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x55, 0xc2, // VANDNPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_src1() {
    // VANDNPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x55, 0xca, // VANDNPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_src2() {
    // VANDNPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x55, 0xca, // VANDNPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_all_extended() {
    // VANDNPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x55, 0xdd, // VANDNPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_r14_r15_r8() {
    // VANDNPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x55, 0xf0, // VANDNPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_mem() {
    // VANDNPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPD XMM1, XMM0, [rip+0x4000]
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
fn test_vandnpd_xmm_mem_extended() {
    // VANDNPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x55, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDNPD XMM10, XMM11, [rip+0x4000]
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
fn test_vandnpd_xmm_self() {
    // VANDNPD XMM2, XMM2, XMM2 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xe9, 0x55, 0xd2, // VANDNPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_xmm_different_operands() {
    // VANDNPD XMM7, XMM1, XMM4
    let code = [
        0xc5, 0xf1, 0x55, 0xfc, // VANDNPD XMM7, XMM1, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VANDNPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vandnpd_ymm_basic() {
    // VANDNPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x55, 0xc2, // VANDNPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_same_dest_src1() {
    // VANDNPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x55, 0xca, // VANDNPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_all_regs() {
    // VANDNPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x55, 0xdd, // VANDNPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_high_regs() {
    // VANDNPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x55, 0xf2, // VANDNPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_dest() {
    // VANDNPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x55, 0xc2, // VANDNPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_src1() {
    // VANDNPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x55, 0xca, // VANDNPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_src2() {
    // VANDNPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x55, 0xca, // VANDNPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_all_extended() {
    // VANDNPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x55, 0xdd, // VANDNPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_r14_r15_r8() {
    // VANDNPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x55, 0xf0, // VANDNPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_mem() {
    // VANDNPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPD YMM1, YMM0, [rip+0x4000]
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
fn test_vandnpd_ymm_mem_extended() {
    // VANDNPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x55, 0x15, 0x00, 0x40, 0x00,
        0x00, // VANDNPD YMM10, YMM11, [rip+0x4000]
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
fn test_vandnpd_ymm_self() {
    // VANDNPD YMM7, YMM7, YMM7 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xc5, 0x55, 0xff, // VANDNPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vandnpd_ymm_different_operands() {
    // VANDNPD YMM6, YMM3, YMM1
    let code = [
        0xc5, 0xe5, 0x55, 0xf1, // VANDNPD YMM6, YMM3, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VANDNPS/PD compute (NOT src1) AND src2.
// For "VANDN dst, src1, src2": src1 = vvvv, src2 = r/m.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;

fn kanv_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kanv_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kanv_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

#[test]
fn test_vandnps_xmm_value() {
    // VANDNPS XMM0, XMM1, XMM2 ; result = (~XMM1) & XMM2, upper 128 zeroed.
    let code = [0xc5, 0xf0, 0x55, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let s1: u128 = 0xF0F0_F0F0_AAAA_5555_FFFF_0000_0F0F_0F0F;
    let s2: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
    kanv_set(&mut vcpu, 1, s1, 0xDEAD);
    kanv_set(&mut vcpu, 2, s2, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    // (~s1) & all-ones == ~s1.
    assert_eq!(kanv_lo(&vcpu, 0), (!s1) & s2);
    assert_eq!(kanv_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vandnps_ymm_value() {
    // VANDNPS YMM0, YMM1, YMM2 ; both lanes = (~src1) & src2.
    let code = [0xc5, 0xf4, 0x55, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let s1_lo: u128 = 0xFFFF_FFFF_0000_0000_AAAA_AAAA_5555_5555;
    let s2_lo: u128 = 0x0F0F_0F0F_0F0F_0F0F_FFFF_FFFF_FFFF_FFFF;
    let s1_hi: u128 = 0x1234_5678_9ABC_DEF0_0F0F_0F0F_F0F0_F0F0;
    let s2_hi: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
    kanv_set(&mut vcpu, 1, s1_lo, s1_hi);
    kanv_set(&mut vcpu, 2, s2_lo, s2_hi);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kanv_lo(&vcpu, 0), (!s1_lo) & s2_lo);
    assert_eq!(kanv_hi(&vcpu, 0), (!s1_hi) & s2_hi);
}

#[test]
fn test_vandnpd_ymm_value() {
    // VANDNPD YMM0, YMM1, YMM2 ; both lanes = (~src1) & src2.
    let code = [0xc5, 0xf5, 0x55, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let s1_lo: u128 = 0xCCCC_CCCC_CCCC_CCCC_3333_3333_3333_3333;
    let s2_lo: u128 = 0xFFFF_0000_FFFF_0000_FFFF_FFFF_0000_0000;
    let s1_hi: u128 = 0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFF;
    let s2_hi: u128 = 0xDEAD_BEEF_CAFE_BABE_1122_3344_5566_7788;
    kanv_set(&mut vcpu, 1, s1_lo, s1_hi);
    kanv_set(&mut vcpu, 2, s2_lo, s2_hi);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kanv_lo(&vcpu, 0), (!s1_lo) & s2_lo);
    // ~0 & s2_hi == s2_hi ; ~all-ones & ... == 0 for low half.
    assert_eq!(kanv_hi(&vcpu, 0), (!s1_hi) & s2_hi);
}
