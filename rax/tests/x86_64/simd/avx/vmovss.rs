use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VMOVSS - Move Scalar Single-Precision Floating-Point

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovss_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xfa, 0x10, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf2, 0x10, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xea, 0x10, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xe2, 0x10, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xda, 0x10, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xd2, 0x10, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0x4a, 0x10, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm9_xmm7_xmm8() {
    let code = [0xc4, 0x41, 0x42, 0x10, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x41, 0x3a, 0x10, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x32, 0x10, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x2a, 0x10, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x22, 0x10, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x1a, 0x10, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x12, 0x10, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm0_xmm14_xmm15() {
    let code = [0xc5, 0x0a, 0x11, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovss_xmm1_xmm15_xmm0() {
    let code = [0xc5, 0x82, 0x10, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
