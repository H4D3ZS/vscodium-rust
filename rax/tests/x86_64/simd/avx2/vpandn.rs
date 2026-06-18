use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPANDN - Bitwise Logical AND NOT

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpandn_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xf9, 0xdf, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf1, 0xdf, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xe9, 0xdf, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xe1, 0xdf, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xd9, 0xdf, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xd1, 0xdf, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0x49, 0xdf, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm9_xmm7_xmm8() {
    let code = [0xc4, 0x41, 0x41, 0xdf, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x41, 0x39, 0xdf, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x31, 0xdf, 0xda, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x29, 0xdf, 0xe3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x21, 0xdf, 0xec, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x19, 0xdf, 0xf5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x11, 0xdf, 0xfe, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm0_xmm14_xmm15() {
    let code = [0xc4, 0xc1, 0x09, 0xdf, 0xc7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm1_xmm15_xmm0() {
    let code = [0xc5, 0x81, 0xdf, 0xc8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm0_xmm1_mem() {
    let code = [0xc5, 0xf1, 0xdf, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm1_xmm2_mem() {
    let code = [0xc5, 0xe9, 0xdf, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm2_xmm3_mem() {
    let code = [0xc5, 0xe1, 0xdf, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm3_xmm4_mem() {
    let code = [0xc5, 0xd9, 0xdf, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm4_xmm5_mem() {
    let code = [0xc5, 0xd1, 0xdf, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm5_xmm6_mem() {
    let code = [0xc5, 0xc9, 0xdf, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm6_xmm7_mem() {
    let code = [0xc5, 0xc1, 0xdf, 0x35, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_xmm7_xmm8_mem() {
    let code = [0xc5, 0xb9, 0xdf, 0x3d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 16];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm0_ymm1_ymm2() {
    let code = [0xc5, 0xf5, 0xdf, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm1_ymm2_ymm3() {
    let code = [0xc5, 0xed, 0xdf, 0xcb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm2_ymm3_ymm4() {
    let code = [0xc5, 0xe5, 0xdf, 0xd4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm3_ymm4_ymm5() {
    let code = [0xc5, 0xdd, 0xdf, 0xdd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm4_ymm5_ymm6() {
    let code = [0xc5, 0xd5, 0xdf, 0xe6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm5_ymm6_ymm7() {
    let code = [0xc5, 0xcd, 0xdf, 0xef, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm6_ymm7_ymm0() {
    let code = [0xc5, 0xc5, 0xdf, 0xf0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm7_ymm0_ymm1() {
    let code = [0xc5, 0xfd, 0xdf, 0xf9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm0_ymm1_mem256() {
    let code = [0xc5, 0xf5, 0xdf, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm1_ymm2_mem256() {
    let code = [0xc5, 0xed, 0xdf, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm2_ymm3_mem256() {
    let code = [0xc5, 0xe5, 0xdf, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm3_ymm4_mem256() {
    let code = [0xc5, 0xdd, 0xdf, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm4_ymm5_mem256() {
    let code = [0xc5, 0xd5, 0xdf, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpandn_ymm5_ymm6_mem256() {
    let code = [0xc5, 0xcd, 0xdf, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    let (mut vcpu, vm_memory) = setup_vm(&code, None);
    let test_data = [0u8; 32];
    vm_memory.write(&test_data, GuestAddress(0x3000)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VPANDN computes (NOT src1) AND src2.
// "VPANDN dst, src1, src2": src1 = vvvv, src2 = r/m.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::VCpu;

fn kpan_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kpan_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kpan_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

#[test]
fn test_vpandn_xmm_value() {
    // VPANDN XMM2, XMM0, XMM1 ; XMM2 = (~XMM0) & XMM1, upper 128 zeroed.
    let code = [0xc5, 0xf9, 0xdf, 0xd1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let s1: u128 = 0xF0F0_F0F0_AAAA_5555_FFFF_0000_0F0F_0F0F;
    let s2: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
    kpan_set(&mut vcpu, 0, s1, 0xDEAD);
    kpan_set(&mut vcpu, 1, s2, 0xBEEF);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kpan_lo(&vcpu, 2), (!s1) & s2);
    assert_eq!(kpan_hi(&vcpu, 2), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vpandn_ymm_value() {
    // VPANDN YMM0, YMM1, YMM2 ; both lanes = (~YMM1) & YMM2.
    let code = [0xc5, 0xf5, 0xdf, 0xc2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let s1_lo: u128 = 0xFFFF_FFFF_0000_0000_AAAA_AAAA_5555_5555;
    let s2_lo: u128 = 0x0F0F_0F0F_0F0F_0F0F_FFFF_FFFF_FFFF_FFFF;
    let s1_hi: u128 = 0x1234_5678_9ABC_DEF0_0F0F_0F0F_F0F0_F0F0;
    let s2_hi: u128 = 0xDEAD_BEEF_CAFE_BABE_1122_3344_5566_7788;
    kpan_set(&mut vcpu, 1, s1_lo, s1_hi);
    kpan_set(&mut vcpu, 2, s2_lo, s2_hi);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kpan_lo(&vcpu, 0), (!s1_lo) & s2_lo);
    assert_eq!(kpan_hi(&vcpu, 0), (!s1_hi) & s2_hi);
}
