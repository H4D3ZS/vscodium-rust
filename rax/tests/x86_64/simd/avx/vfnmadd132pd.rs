use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VFNMADD132PD - Fused Negative Multiply-Add Packed Double-Precision (132)
//
// FMA (Fused Multiply-Add) instructions perform a*b+c in a single operation
// with only one rounding, providing better performance and precision.

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vfnmadd132pd_xmm0_xmm1_xmm2() {
    let code = [0xc4, 0xe2, 0xf1, 0x9c, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm1_xmm2_xmm3() {
    let code = [0xc4, 0xe2, 0xe9, 0x9c, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm2_xmm3_xmm4() {
    let code = [0xc4, 0xe2, 0xe1, 0x9c, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm3_xmm4_xmm5() {
    let code = [0xc4, 0xe2, 0xd9, 0x9c, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm4_xmm5_xmm6() {
    let code = [0xc4, 0xe2, 0xd1, 0x9c, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm5_xmm6_xmm7() {
    let code = [0xc4, 0xe2, 0xc9, 0x9c, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm6_xmm7_xmm8() {
    let code = [0xc4, 0xc2, 0xc1, 0x9c, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm7_xmm8_xmm9() {
    let code = [0xc4, 0xc2, 0xb9, 0x9c, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm8_xmm9_xmm10() {
    let code = [0xc4, 0x42, 0xb1, 0x9c, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm9_xmm10_xmm11() {
    let code = [0xc4, 0x42, 0xa9, 0x9c, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm10_xmm11_xmm12() {
    let code = [0xc4, 0x42, 0xa1, 0x9c, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm11_xmm12_xmm13() {
    let code = [0xc4, 0x42, 0x99, 0x9c, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm0_ymm1_ymm2() {
    let code = [0xc4, 0xe2, 0xf5, 0x9c, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm1_ymm2_ymm3() {
    let code = [0xc4, 0xe2, 0xed, 0x9c, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm2_ymm3_ymm4() {
    let code = [0xc4, 0xe2, 0xe5, 0x9c, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm3_ymm4_ymm5() {
    let code = [0xc4, 0xe2, 0xdd, 0x9c, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm4_ymm5_ymm6() {
    let code = [0xc4, 0xe2, 0xd5, 0x9c, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm5_ymm6_ymm7() {
    let code = [0xc4, 0xe2, 0xcd, 0x9c, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm6_ymm7_ymm0() {
    let code = [0xc4, 0xe2, 0xc5, 0x9c, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm7_ymm0_ymm1() {
    let code = [0xc4, 0xe2, 0xfd, 0x9c, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm0_xmm1_mem() {
    let code = [0xc4, 0xe2, 0xf1, 0x9c, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm1_xmm2_mem() {
    let code = [0xc4, 0xe2, 0xe9, 0x9c, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm2_xmm3_mem() {
    let code = [0xc4, 0xe2, 0xe1, 0x9c, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm3_xmm4_mem() {
    let code = [0xc4, 0xe2, 0xd9, 0x9c, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm4_xmm5_mem() {
    let code = [0xc4, 0xe2, 0xd1, 0x9c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_xmm5_xmm6_mem() {
    let code = [0xc4, 0xe2, 0xc9, 0x9c, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm0_ymm1_mem256() {
    let code = [0xc4, 0xe2, 0xf5, 0x9c, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm1_ymm2_mem256() {
    let code = [0xc4, 0xe2, 0xed, 0x9c, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm2_ymm3_mem256() {
    let code = [0xc4, 0xe2, 0xe5, 0x9c, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfnmadd132pd_ymm3_ymm4_mem256() {
    let code = [0xc4, 0xe2, 0xdd, 0x9c, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
