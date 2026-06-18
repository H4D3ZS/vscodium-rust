use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VDIVSS - Divide Scalar Single-Precision Floating-Point Values
// VDIVSD - Divide Scalar Double-Precision Floating-Point Values
//
// Opcodes:
// VEX.LIG.F3.0F.WIG 5E /r    VDIVSS xmm1, xmm2, xmm3/m32
// VEX.LIG.F2.0F.WIG 5E /r    VDIVSD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VDIVSS Tests
// ============================================================================

#[test]
fn test_vdivss_xmm0_xmm1_xmm2() {
    let code = [0xc5, 0xf2, 0x5e, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm1_xmm2_xmm3() {
    let code = [0xc5, 0xea, 0x5e, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm2_xmm3_xmm4() {
    let code = [0xc5, 0xe2, 0x5e, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm3_xmm4_xmm5() {
    let code = [0xc5, 0xda, 0x5e, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm4_xmm5_xmm6() {
    let code = [0xc5, 0xd2, 0x5e, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm5_xmm6_xmm7() {
    let code = [0xc5, 0xca, 0x5e, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm6_xmm7_xmm8() {
    let code = [0xc4, 0xc1, 0x42, 0x5e, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm7_xmm8_xmm9() {
    let code = [0xc4, 0xc1, 0x3a, 0x5e, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm8_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x32, 0x5e, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm9_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x2a, 0x5e, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm10_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x22, 0x5e, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm11_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x1a, 0x5e, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm12_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x12, 0x5e, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm13_xmm14_xmm15() {
    let code = [0xc4, 0x41, 0x0a, 0x5e, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm14_xmm15_xmm0() {
    let code = [0xc4, 0x61, 0x02, 0x5e, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm15_xmm0_xmm1() {
    let code = [0xc4, 0x61, 0x7a, 0x5e, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm0_xmm1_mem32() {
    let code = [0xc5, 0xf2, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0x00, 0x00, 0x80, 0x3f]; // 1.0f
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivss_xmm2_xmm3_mem32() {
    let code = [0xc5, 0xe2, 0x5e, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0x00, 0x00, 0x00, 0x40]; // 2.0f
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VDIVSD Tests
// ============================================================================

#[test]
fn test_vdivsd_xmm0_xmm1_xmm2() {
    let code = [0xc5, 0xf3, 0x5e, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm1_xmm2_xmm3() {
    let code = [0xc5, 0xeb, 0x5e, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm2_xmm3_xmm4() {
    let code = [0xc5, 0xe3, 0x5e, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm3_xmm4_xmm5() {
    let code = [0xc5, 0xdb, 0x5e, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm4_xmm5_xmm6() {
    let code = [0xc5, 0xd3, 0x5e, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm5_xmm6_xmm7() {
    let code = [0xc5, 0xcb, 0x5e, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm6_xmm7_xmm8() {
    let code = [0xc4, 0xc1, 0x43, 0x5e, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm7_xmm8_xmm9() {
    let code = [0xc4, 0xc1, 0x3b, 0x5e, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm8_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x33, 0x5e, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm9_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x2b, 0x5e, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm10_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x23, 0x5e, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm11_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x1b, 0x5e, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm12_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x13, 0x5e, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm13_xmm14_xmm15() {
    let code = [0xc4, 0x41, 0x0b, 0x5e, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm14_xmm15_xmm0() {
    let code = [0xc4, 0x61, 0x03, 0x5e, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm15_xmm0_xmm1() {
    let code = [0xc4, 0x61, 0x7b, 0x5e, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm0_xmm1_mem64() {
    let code = [0xc5, 0xf3, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f]; // 1.0
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm2_xmm3_mem64() {
    let code = [0xc5, 0xe3, 0x5e, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]; // 2.0
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vdivsd_xmm4_xmm5_mem64() {
    let code = [0xc5, 0xd3, 0x5e, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40]; // 3.0
    vm_memory
        .write(&test_data, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
