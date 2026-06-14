use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VMOVNTPS - Store Non-Temporal Packed Single-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovntps_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xf8, 0x2b, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf8, 0x2b, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xf8, 0x2b, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xf8, 0x2b, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xf8, 0x2b, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xf8, 0x2b, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0xf8, 0x2b, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm9_xmm7_xmm8() {
    let code = [0xc5, 0xf8, 0x2b, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm10_xmm8_xmm9() {
    let code = [0xc5, 0x78, 0x2b, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm11_xmm9_xmm10() {
    let code = [0xc5, 0x78, 0x2b, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm12_xmm10_xmm11() {
    let code = [0xc5, 0x78, 0x2b, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm13_xmm11_xmm12() {
    let code = [0xc5, 0x78, 0x2b, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm14_xmm12_xmm13() {
    let code = [0xc5, 0x78, 0x2b, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm15_xmm13_xmm14() {
    let code = [0xc5, 0x78, 0x2b, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm0_xmm14_xmm15() {
    let code = [0xc5, 0x78, 0x2b, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm1_xmm15_xmm0() {
    let code = [0xc5, 0x78, 0x2b, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm0_xmm1_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm1_xmm2_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm2_xmm3_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm3_xmm4_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm4_xmm5_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm5_xmm6_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm6_xmm7_mem() {
    let code = [0xc5, 0xf8, 0x2b, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_xmm7_xmm8_mem() {
    let code = [0xc5, 0x78, 0x2b, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm0_ymm1_ymm2() {
    let code = [0xc5, 0xfc, 0x2b, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm1_ymm2_ymm3() {
    let code = [0xc5, 0xfc, 0x2b, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm2_ymm3_ymm4() {
    let code = [0xc5, 0xfc, 0x2b, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm3_ymm4_ymm5() {
    let code = [0xc5, 0xfc, 0x2b, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm4_ymm5_ymm6() {
    let code = [0xc5, 0xfc, 0x2b, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm5_ymm6_ymm7() {
    let code = [0xc5, 0xfc, 0x2b, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm6_ymm7_ymm0() {
    let code = [0xc5, 0xfc, 0x2b, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm7_ymm0_ymm1() {
    let code = [0xc5, 0xfc, 0x2b, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm0_ymm1_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm1_ymm2_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm2_ymm3_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm3_ymm4_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm4_ymm5_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vmovntps_ymm5_ymm6_mem256() {
    let code = [0xc5, 0xfc, 0x2b, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
