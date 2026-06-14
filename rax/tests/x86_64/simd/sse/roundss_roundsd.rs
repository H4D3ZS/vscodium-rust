use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// ROUNDSS - Round Scalar Single Precision Floating-Point Values
// ROUNDSD - Round Scalar Double Precision Floating-Point Values
//
// ROUNDSS rounds the low single-precision (32-bit) floating-point value, upper 3 elements preserved
// ROUNDSD rounds the low double-precision (64-bit) floating-point value, upper element preserved
//
// Opcodes:
// 66 0F 3A 0A /r ib    ROUNDSS xmm1, xmm2/m32, imm8 - Round scalar single from xmm2/m32 to xmm1 using imm8 mode
// 66 0F 3A 0B /r ib    ROUNDSD xmm1, xmm2/m64, imm8 - Round scalar double from xmm2/m64 to xmm1 using imm8 mode
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
// ROUNDSS Tests - Scalar Single Precision (float32)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundss_xmm0_xmm1_nearest() {
    // ROUNDSS XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x00, // ROUNDSS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm2_xmm3_nearest() {
    // ROUNDSS XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xd3, 0x00, // ROUNDSS XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm4_xmm5_nearest() {
    // ROUNDSS XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xe5, 0x00, // ROUNDSS XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm6_xmm7_nearest() {
    // ROUNDSS XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xf7, 0x00, // ROUNDSS XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundss_xmm0_xmm1_down() {
    // ROUNDSS XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x01, // ROUNDSS XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm2_xmm3_down() {
    // ROUNDSS XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xd3, 0x01, // ROUNDSS XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm4_xmm5_down() {
    // ROUNDSS XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xe5, 0x01, // ROUNDSS XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm6_xmm7_down() {
    // ROUNDSS XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xf7, 0x01, // ROUNDSS XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundss_xmm0_xmm1_up() {
    // ROUNDSS XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x02, // ROUNDSS XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm2_xmm3_up() {
    // ROUNDSS XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xd3, 0x02, // ROUNDSS XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm4_xmm5_up() {
    // ROUNDSS XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xe5, 0x02, // ROUNDSS XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm6_xmm7_up() {
    // ROUNDSS XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xf7, 0x02, // ROUNDSS XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundss_xmm0_xmm1_trunc() {
    // ROUNDSS XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x03, // ROUNDSS XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm2_xmm3_trunc() {
    // ROUNDSS XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xd3, 0x03, // ROUNDSS XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm4_xmm5_trunc() {
    // ROUNDSS XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xe5, 0x03, // ROUNDSS XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm6_xmm7_trunc() {
    // ROUNDSS XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xf7, 0x03, // ROUNDSS XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundss_xmm0_xmm1_nearest_suppress() {
    // ROUNDSS XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x08, // ROUNDSS XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm2_xmm3_down_suppress() {
    // ROUNDSS XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xd3, 0x09, // ROUNDSS XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm4_xmm5_up_suppress() {
    // ROUNDSS XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xe5, 0x0a, // ROUNDSS XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm6_xmm7_trunc_suppress() {
    // ROUNDSS XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xf7, 0x0b, // ROUNDSS XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundss_xmm8_xmm9_nearest() {
    // ROUNDSS XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0a, 0xc1, 0x00, // ROUNDSS XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm10_xmm11_down() {
    // ROUNDSS XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0a, 0xd3, 0x01, // ROUNDSS XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm12_xmm13_up() {
    // ROUNDSS XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0a, 0xe5, 0x02, // ROUNDSS XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm14_xmm15_trunc() {
    // ROUNDSS XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0a, 0xf7, 0x03, // ROUNDSS XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_roundss_xmm0_mem_nearest() {
    // ROUNDSS XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x00, // ROUNDSS XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm1_mem_down() {
    // ROUNDSS XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // ROUNDSS XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm7_mem_up() {
    // ROUNDSS XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // ROUNDSS XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundss_xmm15_mem_trunc() {
    // ROUNDSS XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x03, // ROUNDSS XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// ROUNDSD Tests - Scalar Double Precision (float64)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundsd_xmm0_xmm1_nearest() {
    // ROUNDSD XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x00, // ROUNDSD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm2_xmm3_nearest() {
    // ROUNDSD XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xd3, 0x00, // ROUNDSD XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm4_xmm5_nearest() {
    // ROUNDSD XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xe5, 0x00, // ROUNDSD XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm6_xmm7_nearest() {
    // ROUNDSD XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xf7, 0x00, // ROUNDSD XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundsd_xmm0_xmm1_down() {
    // ROUNDSD XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x01, // ROUNDSD XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm2_xmm3_down() {
    // ROUNDSD XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xd3, 0x01, // ROUNDSD XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm4_xmm5_down() {
    // ROUNDSD XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xe5, 0x01, // ROUNDSD XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm6_xmm7_down() {
    // ROUNDSD XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xf7, 0x01, // ROUNDSD XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundsd_xmm0_xmm1_up() {
    // ROUNDSD XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x02, // ROUNDSD XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm2_xmm3_up() {
    // ROUNDSD XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xd3, 0x02, // ROUNDSD XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm4_xmm5_up() {
    // ROUNDSD XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xe5, 0x02, // ROUNDSD XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm6_xmm7_up() {
    // ROUNDSD XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xf7, 0x02, // ROUNDSD XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundsd_xmm0_xmm1_trunc() {
    // ROUNDSD XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x03, // ROUNDSD XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm2_xmm3_trunc() {
    // ROUNDSD XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xd3, 0x03, // ROUNDSD XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm4_xmm5_trunc() {
    // ROUNDSD XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xe5, 0x03, // ROUNDSD XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm6_xmm7_trunc() {
    // ROUNDSD XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xf7, 0x03, // ROUNDSD XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundsd_xmm0_xmm1_nearest_suppress() {
    // ROUNDSD XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x08, // ROUNDSD XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm2_xmm3_down_suppress() {
    // ROUNDSD XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xd3, 0x09, // ROUNDSD XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm4_xmm5_up_suppress() {
    // ROUNDSD XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xe5, 0x0a, // ROUNDSD XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm6_xmm7_trunc_suppress() {
    // ROUNDSD XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xf7, 0x0b, // ROUNDSD XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundsd_xmm8_xmm9_nearest() {
    // ROUNDSD XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0b, 0xc1, 0x00, // ROUNDSD XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm10_xmm11_down() {
    // ROUNDSD XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0b, 0xd3, 0x01, // ROUNDSD XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm12_xmm13_up() {
    // ROUNDSD XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0b, 0xe5, 0x02, // ROUNDSD XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm14_xmm15_trunc() {
    // ROUNDSD XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x0b, 0xf7, 0x03, // ROUNDSD XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory operand tests
#[test]
fn test_roundsd_xmm0_mem_nearest() {
    // ROUNDSD XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x00, // ROUNDSD XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm1_mem_down() {
    // ROUNDSD XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // ROUNDSD XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm7_mem_up() {
    // ROUNDSD XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // ROUNDSD XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm15_mem_trunc() {
    // ROUNDSD XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x0b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x03, // ROUNDSD XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional mode combinations
#[test]
fn test_roundss_xmm1_xmm2_mode_0x04() {
    // ROUNDSS XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xca, 0x04, // ROUNDSS XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_xmm1_xmm2_mode_0x04() {
    // ROUNDSD XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xca, 0x04, // ROUNDSD XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Upper bits preservation tests (critical for scalar operations)
#[test]
fn test_roundss_upper_preservation() {
    // ROUNDSS XMM1, XMM2, 0x00 - upper 96 bits should be preserved from XMM1
    let code = [
        0x66, 0x0f, 0x3a, 0x0a, 0xca, 0x00, // ROUNDSS XMM1, XMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_roundsd_upper_preservation() {
    // ROUNDSD XMM1, XMM2, 0x00 - upper 64 bits should be preserved from XMM1
    let code = [
        0x66, 0x0f, 0x3a, 0x0b, 0xca, 0x00, // ROUNDSD XMM1, XMM2, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// ROUNDSS rounds only the low dword (upper 96 bits of DST preserved);
// ROUNDSD rounds only the low qword (upper 64 bits preserved).
// ============================================================================

#[test]
fn kat_roundss_ceil_low_preserves_upper() {
    // ROUNDSS XMM0, XMM1, 2 (66 0F 3A 0A C1 02): ceil of low single.
    // XMM1 low = 2.7 -> ceil 3.0 (0x40400000). Upper 96 bits of XMM0 preserved.
    let code = [0x66, 0x0f, 0x3a, 0x0a, 0xc1, 0x02, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xCAFEBABE_DEADBEEF_FACEFEED_11111111);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00000000_00000000_00000000_402ccccd);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xCAFEBABE_DEADBEEF_FACEFEED_40400000,
        "ROUNDSS got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_roundsd_floor_low_preserves_upper() {
    // ROUNDSD XMM0, XMM1, 1 (66 0F 3A 0B C1 01): floor of low double.
    // XMM1 low = 2.7 -> floor 2.0 (0x4000000000000000). Upper 64 bits preserved.
    let code = [0x66, 0x0f, 0x3a, 0x0b, 0xc1, 0x01, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xCAFEBABEDEADBEEF_400599999999999a);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000000_400599999999999a);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0xCAFEBABEDEADBEEF_4000000000000000,
        "ROUNDSD got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
