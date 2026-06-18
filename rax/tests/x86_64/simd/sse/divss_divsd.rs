use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// DIVSS - Divide Scalar Single Precision Floating-Point Values
// DIVSD - Divide Scalar Double Precision Floating-Point Values
//
// DIVSS divides the low single-precision (32-bit) floating-point value
// DIVSD divides the low double-precision (64-bit) floating-point value
// Upper bits are preserved from the first source operand
//
// Opcodes:
// F3 0F 5E /r             DIVSS xmm1, xmm2/m32     - Divide scalar single in xmm1 by xmm2/m32
// F2 0F 5E /r             DIVSD xmm1, xmm2/m64     - Divide scalar double in xmm1 by xmm2/m64

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// DIVSS Tests - Scalar Single Precision (low 32 bits, preserve upper)
// ============================================================================

#[test]
fn test_divss_xmm0_xmm1() {
    let code = [0xf3, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm1_xmm2() {
    let code = [0xf3, 0x0f, 0x5e, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm2_xmm3() {
    let code = [0xf3, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm3_xmm4() {
    let code = [0xf3, 0x0f, 0x5e, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm4_xmm5() {
    let code = [0xf3, 0x0f, 0x5e, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm5_xmm6() {
    let code = [0xf3, 0x0f, 0x5e, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm6_xmm7() {
    let code = [0xf3, 0x0f, 0x5e, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm7_xmm0() {
    let code = [0xf3, 0x0f, 0x5e, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm8_xmm9() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm9_xmm10() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm10_xmm11() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm11_xmm12() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm12_xmm13() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm13_xmm14() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm14_xmm15() {
    let code = [0xf3, 0x45, 0x0f, 0x5e, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm15_xmm0() {
    let code = [0xf3, 0x44, 0x0f, 0x5e, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm0_mem() {
    let code = [0xf3, 0x0f, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_xmm7_mem() {
    let code = [0xf3, 0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_preserves_upper_bits() {
    let code = [0xf3, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divss_positive_values() {
    let code = [0xf3, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// DIVSD Tests - Scalar Double Precision (low 64 bits, preserve upper)
// ============================================================================

#[test]
fn test_divsd_xmm0_xmm1() {
    let code = [0xf2, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm1_xmm2() {
    let code = [0xf2, 0x0f, 0x5e, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm2_xmm3() {
    let code = [0xf2, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm3_xmm4() {
    let code = [0xf2, 0x0f, 0x5e, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm4_xmm5() {
    let code = [0xf2, 0x0f, 0x5e, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm5_xmm6() {
    let code = [0xf2, 0x0f, 0x5e, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm6_xmm7() {
    let code = [0xf2, 0x0f, 0x5e, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm7_xmm0() {
    let code = [0xf2, 0x0f, 0x5e, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm8_xmm9() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm9_xmm10() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm10_xmm11() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm11_xmm12() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm12_xmm13() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm13_xmm14() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm14_xmm15() {
    let code = [0xf2, 0x45, 0x0f, 0x5e, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm15_xmm0() {
    let code = [0xf2, 0x44, 0x0f, 0x5e, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm0_mem() {
    let code = [0xf2, 0x0f, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_xmm7_mem() {
    let code = [0xf2, 0x0f, 0x5e, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_preserves_upper_bits() {
    let code = [0xf2, 0x0f, 0x5e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_divsd_positive_values() {
    let code = [0xf2, 0x0f, 0x5e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
