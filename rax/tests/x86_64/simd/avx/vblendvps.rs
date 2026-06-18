use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VBLENDVPS - Variable Blend Packed Single-Precision Floating-Point Values
//
// Conditionally copies each dword of the source operand (second operand) to the
// destination operand (first operand) depending on mask bits defined in the
// mask operand (third operand).
//
// Opcodes:
// VEX.128.66.0F3A.W0 4A /r /is4       VBLENDVPS xmm1, xmm2, xmm3/m128, xmm4
// VEX.256.66.0F3A.W0 4A /r /is4       VBLENDVPS ymm1, ymm2, ymm3/m256, ymm4

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VBLENDVPS Tests (VEX.128)
// ============================================================================

#[test]
fn test_vblendvps_xmm0_xmm1_xmm2_xmm3() {
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm1_xmm2_xmm3_xmm4() {
    let code = [
        0xc4, 0xe3, 0x69, 0x4a, 0xcb, 0x40, // VBLENDVPS XMM1, XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm2_xmm3_xmm4_xmm5() {
    let code = [
        0xc4, 0xe3, 0x61, 0x4a, 0xd4, 0x50, // VBLENDVPS XMM2, XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm3_xmm4_xmm5_xmm6() {
    let code = [
        0xc4, 0xe3, 0x59, 0x4a, 0xdd, 0x60, // VBLENDVPS XMM3, XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm4_xmm5_xmm6_xmm7() {
    let code = [
        0xc4, 0xe3, 0x51, 0x4a, 0xe6, 0x70, // VBLENDVPS XMM4, XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm7_xmm0_xmm1_xmm2() {
    let code = [
        0xc4, 0xe3, 0x79, 0x4a, 0xf9, 0x20, // VBLENDVPS XMM7, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm8_xmm9_xmm10_xmm11() {
    let code = [
        0xc4, 0x43, 0x31, 0x4a, 0xc2, 0xb0, // VBLENDVPS XMM8, XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm15_xmm0_xmm1_xmm2() {
    let code = [
        0xc4, 0x63, 0x79, 0x4a, 0xf9, 0x20, // VBLENDVPS XMM15, XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_xmm0_xmm1_mem_xmm3() {
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x30, // VBLENDVPS XMM0, XMM1, [0x3000], XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VBLENDVPS Tests (VEX.256)
// ============================================================================

#[test]
fn test_vblendvps_ymm0_ymm1_ymm2_ymm3() {
    let code = [
        0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x30, // VBLENDVPS YMM0, YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_ymm1_ymm2_ymm3_ymm4() {
    let code = [
        0xc4, 0xe3, 0x6d, 0x4a, 0xcb, 0x40, // VBLENDVPS YMM1, YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_ymm2_ymm3_ymm4_ymm5() {
    let code = [
        0xc4, 0xe3, 0x65, 0x4a, 0xd4, 0x50, // VBLENDVPS YMM2, YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_ymm3_ymm4_ymm5_ymm6() {
    let code = [
        0xc4, 0xe3, 0x5d, 0x4a, 0xdd, 0x60, // VBLENDVPS YMM3, YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_ymm7_ymm0_ymm1_ymm2() {
    let code = [
        0xc4, 0xe3, 0x7d, 0x4a, 0xf9, 0x20, // VBLENDVPS YMM7, YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Different mask patterns
// ============================================================================

#[test]
fn test_vblendvps_all_from_first() {
    // Mask selects all from first operand (mask all zeros)
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2,
        0x00, // VBLENDVPS XMM0, XMM1, XMM2, XMM0 (assuming XMM0 is zeros)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_all_from_second() {
    // Mask selects all from second operand (mask all ones)
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_alternating() {
    // Mask with alternating pattern
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x40, // VBLENDVPS XMM0, XMM1, XMM2, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_ymm_different_lanes() {
    // YMM test with different mask per lane
    let code = [
        0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x50, // VBLENDVPS YMM0, YMM1, YMM2, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_high_registers() {
    // Test with high numbered registers
    let code = [
        0xc4, 0x43, 0x31, 0x4a, 0xc2, 0xb0, // VBLENDVPS XMM8, XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vblendvps_sequential_ops() {
    // Sequential blend operations
    let code = [
        0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, // VBLENDVPS XMM0, XMM1, XMM2, XMM3
        0xc4, 0xe3, 0x59, 0x4a, 0xdd, 0x60, // VBLENDVPS XMM3, XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VBLENDVPS selects per 32-bit lane from src2 when
// the mask lane's sign bit (MSB) is set, otherwise from src1.
//   VBLENDVPS dst, src1, src2, mask : src1=vvvv, src2=r/m, mask=is4.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::VCpu;

fn kbv_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}
fn kbv_lo(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.xmm[idx][0] as u128) | ((r.xmm[idx][1] as u128) << 64)
}
fn kbv_hi(vcpu: &X86_64Vcpu, idx: usize) -> u128 {
    let r = vcpu.get_regs().unwrap();
    (r.ymm_high[idx][0] as u128) | ((r.ymm_high[idx][1] as u128) << 64)
}

fn blendv_ps(s1: u128, s2: u128, mask: u128) -> u128 {
    let mut out = 0u128;
    for i in 0..4 {
        let m = (mask >> (i * 32 + 31)) & 1;
        let v = if m == 1 {
            (s2 >> (i * 32)) & 0xFFFF_FFFF
        } else {
            (s1 >> (i * 32)) & 0xFFFF_FFFF
        };
        out |= v << (i * 32);
    }
    out
}

const BV_S1_LO: u128 = 0x1111_1111_2222_2222_3333_3333_4444_4444;
const BV_S2_LO: u128 = 0xAAAA_AAAA_BBBB_BBBB_CCCC_CCCC_DDDD_DDDD;
const BV_S1_HI: u128 = 0x5555_5555_6666_6666_7777_7777_8888_8888;
const BV_S2_HI: u128 = 0xEEEE_EEEE_FFFF_FFFF_0F0F_0F0F_1010_1010;
// Mask: lanes with sign set pick src2. Choose alternating per lane.
const BV_MASK_LO: u128 = 0x8000_0000_0000_0000_8000_0000_0000_0000; // lanes 1,3 -> src2
const BV_MASK_HI: u128 = 0x0000_0000_8000_0000_0000_0000_8000_0000; // lanes 0,2 -> src2

#[test]
fn test_vblendvps_xmm_value() {
    // VBLENDVPS XMM0, XMM1, XMM2, XMM3 ; upper 128 zeroed.
    let code = [0xc4, 0xe3, 0x71, 0x4a, 0xc2, 0x30, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbv_set(&mut vcpu, 1, BV_S1_LO, 0xDEAD);
    kbv_set(&mut vcpu, 2, BV_S2_LO, 0xBEEF);
    kbv_set(&mut vcpu, 3, BV_MASK_LO, 0xCAFE);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbv_lo(&vcpu, 0), blendv_ps(BV_S1_LO, BV_S2_LO, BV_MASK_LO));
    assert_eq!(kbv_hi(&vcpu, 0), 0, "VEX.128 must zero upper 128 bits");
}

#[test]
fn test_vblendvps_ymm_value() {
    // VBLENDVPS YMM0, YMM1, YMM2, YMM3 ; both lanes blended per-element.
    let code = [0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x30, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    kbv_set(&mut vcpu, 1, BV_S1_LO, BV_S1_HI);
    kbv_set(&mut vcpu, 2, BV_S2_LO, BV_S2_HI);
    kbv_set(&mut vcpu, 3, BV_MASK_LO, BV_MASK_HI);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbv_lo(&vcpu, 0), blendv_ps(BV_S1_LO, BV_S2_LO, BV_MASK_LO));
    assert_eq!(kbv_hi(&vcpu, 0), blendv_ps(BV_S1_HI, BV_S2_HI, BV_MASK_HI));
}

#[test]
fn test_vblendvps_all_from_one_source() {
    // All-zero mask -> all src1 ; all-sign mask -> all src2.
    let code = [0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x30, 0xf4]; // YMM0,YMM1,YMM2,YMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    kbv_set(&mut vcpu, 1, BV_S1_LO, BV_S1_HI);
    kbv_set(&mut vcpu, 2, BV_S2_LO, BV_S2_HI);
    kbv_set(&mut vcpu, 3, 0, 0); // all mask MSBs clear -> src1
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(kbv_lo(&vcpu, 0), BV_S1_LO);
    assert_eq!(kbv_hi(&vcpu, 0), BV_S1_HI);

    let code2 = [0xc4, 0xe3, 0x75, 0x4a, 0xc2, 0x30, 0xf4];
    let (mut vcpu2, _) = setup_vm(&code2, None);
    kbv_set(&mut vcpu2, 1, BV_S1_LO, BV_S1_HI);
    kbv_set(&mut vcpu2, 2, BV_S2_LO, BV_S2_HI);
    kbv_set(&mut vcpu2, 3, u128::MAX, u128::MAX); // all MSBs set -> src2
    run_until_hlt(&mut vcpu2).unwrap();
    assert_eq!(kbv_lo(&vcpu2, 0), BV_S2_LO);
    assert_eq!(kbv_hi(&vcpu2, 0), BV_S2_HI);
}
