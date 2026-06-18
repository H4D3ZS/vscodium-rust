use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VFMADD231PS - Fused Multiply-Add Packed Single-Precision (231)
//
// FMA (Fused Multiply-Add) instructions perform a*b+c in a single operation
// with only one rounding, providing better performance and precision.

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vfmadd231ps_xmm0_xmm1_xmm2() {
    let code = [0xc4, 0xe2, 0x71, 0xb8, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm1_xmm2_xmm3() {
    let code = [0xc4, 0xe2, 0x69, 0xb8, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm2_xmm3_xmm4() {
    let code = [0xc4, 0xe2, 0x61, 0xb8, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm3_xmm4_xmm5() {
    let code = [0xc4, 0xe2, 0x59, 0xb8, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm4_xmm5_xmm6() {
    let code = [0xc4, 0xe2, 0x51, 0xb8, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm5_xmm6_xmm7() {
    let code = [0xc4, 0xe2, 0x49, 0xb8, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm6_xmm7_xmm8() {
    let code = [0xc4, 0xc2, 0x41, 0xb8, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm7_xmm8_xmm9() {
    let code = [0xc4, 0xc2, 0x39, 0xb8, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm8_xmm9_xmm10() {
    let code = [0xc4, 0x42, 0x31, 0xb8, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm9_xmm10_xmm11() {
    let code = [0xc4, 0x42, 0x29, 0xb8, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm10_xmm11_xmm12() {
    let code = [0xc4, 0x42, 0x21, 0xb8, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm11_xmm12_xmm13() {
    let code = [0xc4, 0x42, 0x19, 0xb8, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm0_ymm1_ymm2() {
    let code = [0xc4, 0xe2, 0x75, 0xb8, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm1_ymm2_ymm3() {
    let code = [0xc4, 0xe2, 0x6d, 0xb8, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm2_ymm3_ymm4() {
    let code = [0xc4, 0xe2, 0x65, 0xb8, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm3_ymm4_ymm5() {
    let code = [0xc4, 0xe2, 0x5d, 0xb8, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm4_ymm5_ymm6() {
    let code = [0xc4, 0xe2, 0x55, 0xb8, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm5_ymm6_ymm7() {
    let code = [0xc4, 0xe2, 0x4d, 0xb8, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm6_ymm7_ymm0() {
    let code = [0xc4, 0xe2, 0x45, 0xb8, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm7_ymm0_ymm1() {
    let code = [0xc4, 0xe2, 0x7d, 0xb8, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm0_xmm1_mem() {
    let code = [0xc4, 0xe2, 0x71, 0xb8, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm1_xmm2_mem() {
    let code = [0xc4, 0xe2, 0x69, 0xb8, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm2_xmm3_mem() {
    let code = [0xc4, 0xe2, 0x61, 0xb8, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm3_xmm4_mem() {
    let code = [0xc4, 0xe2, 0x59, 0xb8, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm4_xmm5_mem() {
    let code = [0xc4, 0xe2, 0x51, 0xb8, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_xmm5_xmm6_mem() {
    let code = [0xc4, 0xe2, 0x49, 0xb8, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm0_ymm1_mem256() {
    let code = [0xc4, 0xe2, 0x75, 0xb8, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm1_ymm2_mem256() {
    let code = [0xc4, 0xe2, 0x6d, 0xb8, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm2_ymm3_mem256() {
    let code = [0xc4, 0xe2, 0x65, 0xb8, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vfmadd231ps_ymm3_ymm4_mem256() {
    let code = [0xc4, 0xe2, 0x5d, 0xb8, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
