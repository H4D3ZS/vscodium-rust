use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// ROUNDPS - Round Packed Single Precision Floating-Point Values
// ROUNDPD - Round Packed Double Precision Floating-Point Values
//
// ROUNDPS rounds 4 packed single-precision (32-bit) floating-point values
// ROUNDPD rounds 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// 66 0F 3A 08 /r ib    ROUNDPS xmm1, xmm2/m128, imm8 - Round packed single from xmm2/m128 to xmm1 using imm8 mode
// 66 0F 3A 09 /r ib    ROUNDPD xmm1, xmm2/m128, imm8 - Round packed double from xmm2/m128 to xmm1 using imm8 mode
//
// Rounding modes (imm8 bits[1:0]):
//   00b - Round to nearest (even)
//   01b - Round down (toward -infinity)
//   10b - Round up (toward +infinity)
//   11b - Round toward zero (truncate)
// Bit 2: 0 = use imm8[1:0], 1 = use MXCSR.RC
// Bit 3: 0 = raise precision exception, 1 = suppress precision exception

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// ROUNDPS Tests - Packed Single Precision (4x float32)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundps_xmm0_xmm1_nearest() {
    // ROUNDPS XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x00, // ROUNDPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_nearest() {
    // ROUNDPS XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x00, // ROUNDPS XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_nearest() {
    // ROUNDPS XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x00, // ROUNDPS XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_nearest() {
    // ROUNDPS XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x00, // ROUNDPS XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundps_xmm0_xmm1_down() {
    // ROUNDPS XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x01, // ROUNDPS XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_down() {
    // ROUNDPS XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x01, // ROUNDPS XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_down() {
    // ROUNDPS XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x01, // ROUNDPS XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_down() {
    // ROUNDPS XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x01, // ROUNDPS XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundps_xmm0_xmm1_up() {
    // ROUNDPS XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x02, // ROUNDPS XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_up() {
    // ROUNDPS XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x02, // ROUNDPS XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_up() {
    // ROUNDPS XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x02, // ROUNDPS XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_up() {
    // ROUNDPS XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x02, // ROUNDPS XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundps_xmm0_xmm1_trunc() {
    // ROUNDPS XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x03, // ROUNDPS XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_trunc() {
    // ROUNDPS XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x03, // ROUNDPS XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_trunc() {
    // ROUNDPS XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x03, // ROUNDPS XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_trunc() {
    // ROUNDPS XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x03, // ROUNDPS XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundps_xmm0_xmm1_nearest_suppress() {
    // ROUNDPS XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x08, // ROUNDPS XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_down_suppress() {
    // ROUNDPS XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x09, // ROUNDPS XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_up_suppress() {
    // ROUNDPS XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x0a, // ROUNDPS XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_trunc_suppress() {
    // ROUNDPS XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x0b, // ROUNDPS XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundps_xmm8_xmm9_nearest() {
    // ROUNDPS XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xc1, 0x00, // ROUNDPS XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm10_xmm11_down() {
    // ROUNDPS XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xd3, 0x01, // ROUNDPS XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm12_xmm13_up() {
    // ROUNDPS XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xe5, 0x02, // ROUNDPS XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm14_xmm15_trunc() {
    // ROUNDPS XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xf7, 0x03, // ROUNDPS XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_roundps_xmm0_mem_nearest() {
    // ROUNDPS XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x00, // ROUNDPS XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm1_mem_down() {
    // ROUNDPS XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // ROUNDPS XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm7_mem_up() {
    // ROUNDPS XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // ROUNDPS XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundps_xmm15_mem_trunc() {
    // ROUNDPS XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x08, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x03, // ROUNDPS XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// ROUNDPD Tests - Packed Double Precision (2x float64)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundpd_xmm0_xmm1_nearest() {
    // ROUNDPD XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x00, // ROUNDPD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_nearest() {
    // ROUNDPD XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x00, // ROUNDPD XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_nearest() {
    // ROUNDPD XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x00, // ROUNDPD XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_nearest() {
    // ROUNDPD XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x00, // ROUNDPD XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundpd_xmm0_xmm1_down() {
    // ROUNDPD XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x01, // ROUNDPD XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_down() {
    // ROUNDPD XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x01, // ROUNDPD XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_down() {
    // ROUNDPD XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x01, // ROUNDPD XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_down() {
    // ROUNDPD XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x01, // ROUNDPD XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundpd_xmm0_xmm1_up() {
    // ROUNDPD XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x02, // ROUNDPD XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_up() {
    // ROUNDPD XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x02, // ROUNDPD XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_up() {
    // ROUNDPD XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x02, // ROUNDPD XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_up() {
    // ROUNDPD XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x02, // ROUNDPD XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundpd_xmm0_xmm1_trunc() {
    // ROUNDPD XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x03, // ROUNDPD XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_trunc() {
    // ROUNDPD XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x03, // ROUNDPD XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_trunc() {
    // ROUNDPD XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x03, // ROUNDPD XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_trunc() {
    // ROUNDPD XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x03, // ROUNDPD XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundpd_xmm0_xmm1_nearest_suppress() {
    // ROUNDPD XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x08, // ROUNDPD XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_down_suppress() {
    // ROUNDPD XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x09, // ROUNDPD XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_up_suppress() {
    // ROUNDPD XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x0a, // ROUNDPD XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_trunc_suppress() {
    // ROUNDPD XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x0b, // ROUNDPD XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundpd_xmm8_xmm9_nearest() {
    // ROUNDPD XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xc1, 0x00, // ROUNDPD XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm10_xmm11_down() {
    // ROUNDPD XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xd3, 0x01, // ROUNDPD XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm12_xmm13_up() {
    // ROUNDPD XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xe5, 0x02, // ROUNDPD XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm14_xmm15_trunc() {
    // ROUNDPD XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xf7, 0x03, // ROUNDPD XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_roundpd_xmm0_mem_nearest() {
    // ROUNDPD XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x00, // ROUNDPD XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm1_mem_down() {
    // ROUNDPD XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // ROUNDPD XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm7_mem_up() {
    // ROUNDPD XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // ROUNDPD XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm15_mem_trunc() {
    // ROUNDPD XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x09, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x03, // ROUNDPD XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional mode combinations
#[test]
fn test_roundps_xmm1_xmm2_mode_0x04() {
    // ROUNDPS XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xca, 0x04, // ROUNDPS XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundpd_xmm1_xmm2_mode_0x04() {
    // ROUNDPD XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xca, 0x04, // ROUNDPD XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// ROUNDPS/ROUNDPD round each lane. imm8[1:0]: 0=nearest,1=floor,2=ceil,
// 3=truncate. Inputs avoid x.5 so the round-nearest tie rule is irrelevant.
// ============================================================================

#[test]
fn kat_roundps_floor() {
    // ROUNDPS XMM0, XMM1, 1 (66 0F 3A 08 C1 01): floor.
    // input lanes [2.7, -2.3, 4.5, -2.7] -> [2.0, -3.0, 4.0, -3.0]
    let code = [0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x01, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xc02ccccd40900000c0133333402ccccd);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xc040000040800000c040000040000000,
        "ROUNDPS floor got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_roundps_ceil() {
    // ROUNDPS XMM0, XMM1, 2 (66 0F 3A 08 C1 02): ceil.
    let code = [0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x02, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xc02ccccd40900000c0133333402ccccd);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xc000000040a00000c000000040400000,
        "ROUNDPS ceil got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_roundps_trunc() {
    // ROUNDPS XMM0, XMM1, 3 (66 0F 3A 08 C1 03): truncate toward zero.
    let code = [0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x03, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xc02ccccd40900000c0133333402ccccd);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xc000000040800000c000000040000000,
        "ROUNDPS trunc got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_roundpd_floor() {
    // ROUNDPD XMM0, XMM1, 1 (66 0F 3A 09 C1 01): floor of [2.7, -2.3].
    let code = [0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x01, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xc002666666666666400599999999999a);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xc0080000000000004000000000000000,
        "ROUNDPD floor got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_roundpd_ceil() {
    // ROUNDPD XMM0, XMM1, 2 (66 0F 3A 09 C1 02): ceil of [2.7, -2.3].
    let code = [0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x02, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0xc002666666666666400599999999999a);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xc0000000000000004008000000000000,
        "ROUNDPD ceil got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
