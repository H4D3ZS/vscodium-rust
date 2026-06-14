use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VMOVHPD - Move High Packed Double-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovhpd_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xf9, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf1, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xe9, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xe1, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xd9, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xd1, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0x49, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm9_xmm7_xmm8() {
    let code = [0xc5, 0x41, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm10_xmm8_xmm9() {
    let code = [0xc5, 0x39, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm11_xmm9_xmm10() {
    let code = [0xc5, 0x31, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm12_xmm10_xmm11() {
    let code = [0xc5, 0x29, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm13_xmm11_xmm12() {
    let code = [0xc5, 0x21, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm14_xmm12_xmm13() {
    let code = [0xc5, 0x19, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm15_xmm13_xmm14() {
    let code = [0xc5, 0x11, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm0_xmm14_xmm15() {
    let code = [0xc5, 0x89, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm1_xmm15_xmm0() {
    let code = [0xc5, 0x81, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm0_xmm1_mem() {
    let code = [0xc5, 0xf1, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm1_xmm2_mem() {
    let code = [0xc5, 0xe9, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm2_xmm3_mem() {
    let code = [0xc5, 0xe1, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm3_xmm4_mem() {
    let code = [0xc5, 0xd9, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm4_xmm5_mem() {
    let code = [0xc5, 0xd1, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm5_xmm6_mem() {
    let code = [0xc5, 0xc9, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm6_xmm7_mem() {
    let code = [0xc5, 0xc1, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovhpd_xmm7_xmm8_mem() {
    let code = [0xc5, 0xb9, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
