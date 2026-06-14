use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPADDSW - Add Packed Signed Word Integers with Saturation

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpaddsw_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xf9, 0xed, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf1, 0xed, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xe9, 0xed, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xe1, 0xed, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xd9, 0xed, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xd1, 0xed, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0x49, 0xed, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm9_xmm7_xmm8() {
    let code = [0xc5, 0x39, 0xed, 0xcf, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x41, 0x39, 0xed, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x31, 0xed, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x29, 0xed, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x21, 0xed, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x19, 0xed, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x11, 0xed, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm0_xmm14_xmm15() {
    let code = [0xc4, 0xc1, 0x09, 0xed, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm1_xmm15_xmm0() {
    let code = [0xc5, 0x81, 0xed, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm0_xmm1_mem() {
    let code = [0xc5, 0xf1, 0xed, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm1_xmm2_mem() {
    let code = [0xc5, 0xe9, 0xed, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm2_xmm3_mem() {
    let code = [0xc5, 0xe1, 0xed, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm3_xmm4_mem() {
    let code = [0xc5, 0xd9, 0xed, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm4_xmm5_mem() {
    let code = [0xc5, 0xd1, 0xed, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm5_xmm6_mem() {
    let code = [0xc5, 0xc9, 0xed, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm6_xmm7_mem() {
    let code = [0xc5, 0xc1, 0xed, 0x35, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_xmm7_xmm8_mem() {
    let code = [0xc5, 0xb9, 0xed, 0x3d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm0_ymm1_ymm2() {
    let code = [0xc5, 0xf5, 0xed, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm1_ymm2_ymm3() {
    let code = [0xc5, 0xed, 0xed, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm2_ymm3_ymm4() {
    let code = [0xc5, 0xe5, 0xed, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm3_ymm4_ymm5() {
    let code = [0xc5, 0xdd, 0xed, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm4_ymm5_ymm6() {
    let code = [0xc5, 0xd5, 0xed, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm5_ymm6_ymm7() {
    let code = [0xc5, 0xcd, 0xed, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm6_ymm7_ymm0() {
    let code = [0xc5, 0xc5, 0xed, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm7_ymm0_ymm1() {
    let code = [0xc5, 0xfd, 0xed, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm0_ymm1_mem256() {
    let code = [0xc5, 0xf5, 0xed, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm1_ymm2_mem256() {
    let code = [0xc5, 0xed, 0xed, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm2_ymm3_mem256() {
    let code = [0xc5, 0xe5, 0xed, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm3_ymm4_mem256() {
    let code = [0xc5, 0xdd, 0xed, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm4_ymm5_mem256() {
    let code = [0xc5, 0xd5, 0xed, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpaddsw_ymm5_ymm6_mem256() {
    let code = [0xc5, 0xcd, 0xed, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
