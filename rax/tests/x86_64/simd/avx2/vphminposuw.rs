use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPHMINPOSUW - Horizontal Minimum of Packed Unsigned Words

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vphminposuw_xmm2_xmm0_xmm1() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xd0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm3_xmm1_xmm2() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xd9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm4_xmm2_xmm3() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xe2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm5_xmm3_xmm4() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xeb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm6_xmm4_xmm5() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xf4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm7_xmm5_xmm6() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xfd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm8_xmm6_xmm7() {
    let code = [0xc4, 0x62, 0x79, 0x41, 0xc6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm9_xmm7_xmm8() {
    let code = [0xc4, 0x62, 0x79, 0x41, 0xcf, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xd0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xd9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xe2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xeb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xf4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x42, 0x79, 0x41, 0xfd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm0_xmm14_xmm15() {
    let code = [0xc4, 0xc2, 0x79, 0x41, 0xc6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm1_xmm15_xmm0() {
    let code = [0xc4, 0xc2, 0x79, 0x41, 0xcf, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm0_xmm1_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x05, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm1_xmm2_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x0d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm2_xmm3_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x15, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm3_xmm4_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm4_xmm5_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x25, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm5_xmm6_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x2d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm6_xmm7_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x35, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vphminposuw_xmm7_xmm8_mem() {
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x3d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
