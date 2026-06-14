use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPMULDQ - Multiply Packed Signed Dword Integers

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpmuldq_xmm2_xmm0_xmm1() {
    let code = [0xc4, 0xe2, 0x79, 0x28, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm3_xmm1_xmm2() {
    let code = [0xc4, 0xe2, 0x71, 0x28, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm4_xmm2_xmm3() {
    let code = [0xc4, 0xe2, 0x69, 0x28, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm5_xmm3_xmm4() {
    let code = [0xc4, 0xe2, 0x61, 0x28, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm6_xmm4_xmm5() {
    let code = [0xc4, 0xe2, 0x59, 0x28, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm7_xmm5_xmm6() {
    let code = [0xc4, 0xe2, 0x51, 0x28, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm8_xmm6_xmm7() {
    let code = [0xc4, 0x62, 0x49, 0x28, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm9_xmm7_xmm8() {
    let code = [0xc4, 0x42, 0x41, 0x28, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x42, 0x39, 0x28, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x42, 0x31, 0x28, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x42, 0x29, 0x28, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x42, 0x21, 0x28, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x42, 0x19, 0x28, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x42, 0x11, 0x28, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm0_xmm14_xmm15() {
    let code = [0xc4, 0xc2, 0x09, 0x28, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm1_xmm15_xmm0() {
    let code = [0xc4, 0xe2, 0x01, 0x28, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm0_xmm1_mem() {
    let code = [0xc4, 0xe2, 0x71, 0x28, 0x05, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm1_xmm2_mem() {
    let code = [0xc4, 0xe2, 0x69, 0x28, 0x0d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm2_xmm3_mem() {
    let code = [0xc4, 0xe2, 0x61, 0x28, 0x15, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm3_xmm4_mem() {
    let code = [0xc4, 0xe2, 0x59, 0x28, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm4_xmm5_mem() {
    let code = [0xc4, 0xe2, 0x51, 0x28, 0x25, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm5_xmm6_mem() {
    let code = [0xc4, 0xe2, 0x49, 0x28, 0x2d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm6_xmm7_mem() {
    let code = [0xc4, 0xe2, 0x41, 0x28, 0x35, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_xmm7_xmm8_mem() {
    let code = [0xc4, 0xe2, 0x39, 0x28, 0x3d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm0_ymm1_ymm2() {
    let code = [0xc4, 0xe2, 0x75, 0x28, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm1_ymm2_ymm3() {
    let code = [0xc4, 0xe2, 0x6d, 0x28, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm2_ymm3_ymm4() {
    let code = [0xc4, 0xe2, 0x65, 0x28, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm3_ymm4_ymm5() {
    let code = [0xc4, 0xe2, 0x5d, 0x28, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm4_ymm5_ymm6() {
    let code = [0xc4, 0xe2, 0x55, 0x28, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm5_ymm6_ymm7() {
    let code = [0xc4, 0xe2, 0x4d, 0x28, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm6_ymm7_ymm0() {
    let code = [0xc4, 0xe2, 0x45, 0x28, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm7_ymm0_ymm1() {
    let code = [0xc4, 0xe2, 0x7d, 0x28, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm0_ymm1_mem256() {
    let code = [0xc4, 0xe2, 0x75, 0x28, 0x05, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm1_ymm2_mem256() {
    let code = [0xc4, 0xe2, 0x6d, 0x28, 0x0d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm2_ymm3_mem256() {
    let code = [0xc4, 0xe2, 0x65, 0x28, 0x15, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm3_ymm4_mem256() {
    let code = [0xc4, 0xe2, 0x5d, 0x28, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm4_ymm5_mem256() {
    let code = [0xc4, 0xe2, 0x55, 0x28, 0x25, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmuldq_ymm5_ymm6_mem256() {
    let code = [0xc4, 0xe2, 0x4d, 0x28, 0x2d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
