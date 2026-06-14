use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VUCOMISD - Compare Unordered Scalar Double-Precision Floating-Point Values
//
// Opcodes: VEX.LIG.66.0F.WIG 2E /r

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vucomisd_xmm0_xmm1_xmm2() {
    let code = [0xc5, 0xf9, 0x2e, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm1_xmm2_xmm3() {
    let code = [0xc5, 0xf9, 0x2e, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm2_xmm3_xmm4() {
    let code = [0xc5, 0xf9, 0x2e, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm3_xmm4_xmm5() {
    let code = [0xc5, 0xf9, 0x2e, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm4_xmm5_xmm6() {
    let code = [0xc5, 0xf9, 0x2e, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm5_xmm6_xmm7() {
    let code = [0xc5, 0xf9, 0x2e, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm6_xmm7_xmm8() {
    let code = [0xc5, 0xf9, 0x2e, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vucomisd_xmm7_xmm8_xmm9() {
    let code = [0xc4, 0xc1, 0x79, 0x2e, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional tests for memory operands and YMM registers would go here
