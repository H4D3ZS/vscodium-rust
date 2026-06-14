use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VPMOVMSKB - Move Byte Mask to Integer

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpmovmskb_xmm2_xmm0_xmm1() {
    let code = [0xc5, 0xf9, 0xd7, 0xd0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm3_xmm1_xmm2() {
    let code = [0xc5, 0xf9, 0xd7, 0xd9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm4_xmm2_xmm3() {
    let code = [0xc5, 0xf9, 0xd7, 0xe2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm5_xmm3_xmm4() {
    let code = [0xc5, 0xf9, 0xd7, 0xeb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm6_xmm4_xmm5() {
    let code = [0xc5, 0xf9, 0xd7, 0xf4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm7_xmm5_xmm6() {
    let code = [0xc5, 0xf9, 0xd7, 0xfd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm8_xmm6_xmm7() {
    let code = [0xc5, 0x79, 0xd7, 0xc6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm9_xmm7_xmm8() {
    let code = [0xc5, 0x79, 0xd7, 0xcf, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm10_xmm8_xmm9() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xd0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm11_xmm9_xmm10() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xd9, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm12_xmm10_xmm11() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xe2, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm13_xmm11_xmm12() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xeb, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm14_xmm12_xmm13() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xf4, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm15_xmm13_xmm14() {
    let code = [0xc4, 0x41, 0x79, 0xd7, 0xfd, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm0_xmm14_xmm15() {
    let code = [0xc4, 0xc1, 0x79, 0xd7, 0xc6, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_xmm1_xmm15_xmm0() {
    let code = [0xc4, 0xc1, 0x79, 0xd7, 0xcf, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm0_ymm1_ymm2() {
    let code = [0xc5, 0xfd, 0xd7, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm1_ymm2_ymm3() {
    let code = [0xc5, 0xfd, 0xd7, 0xca, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm2_ymm3_ymm4() {
    let code = [0xc5, 0xfd, 0xd7, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm3_ymm4_ymm5() {
    let code = [0xc5, 0xfd, 0xd7, 0xdc, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm4_ymm5_ymm6() {
    let code = [0xc5, 0xfd, 0xd7, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm5_ymm6_ymm7() {
    let code = [0xc5, 0xfd, 0xd7, 0xee, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm6_ymm7_ymm0() {
    let code = [0xc5, 0xfd, 0xd7, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
#[test]
fn test_vpmovmskb_ymm7_ymm0_ymm1() {
    let code = [0xc5, 0xfd, 0xd7, 0xf8, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VPMOVMSKB gathers the MSB of every byte into a GPR.
//   XMM source -> 16 bits ; YMM source -> 32 bits.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::VCpu;

fn kpmb_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}

// Build a 128-bit value whose byte i has MSB set iff bit i of `mask` is set.
fn bytes_from_mask16(mask: u16) -> u128 {
    let mut out = [0u8; 16];
    for i in 0..16 {
        if (mask >> i) & 1 == 1 {
            out[i] = 0x80;
        } else {
            out[i] = 0x01;
        }
    }
    u128::from_le_bytes(out)
}

#[test]
fn test_vpmovmskb_xmm_value() {
    // VPMOVMSKB EAX, XMM1 ; 16-byte mask.
    let code = [0xc5, 0xf9, 0xd7, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let mask: u16 = 0b1010_1100_0011_0101;
    kpmb_set(&mut vcpu, 1, bytes_from_mask16(mask), 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, mask as u64);
    // Upper bits of the 32-bit result must be clear for an XMM source.
    assert_eq!(regs.rax & 0xFFFF_0000, 0);
}

#[test]
fn test_vpmovmskb_ymm_value() {
    // VPMOVMSKB EAX, YMM1 ; 32-byte mask (low 16 bits = low lane, high 16 = high lane).
    let code = [0xc5, 0xfd, 0xd7, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let lo_mask: u16 = 0b0000_1111_1111_0000;
    let hi_mask: u16 = 0b1100_0011_0101_1010;
    kpmb_set(
        &mut vcpu,
        1,
        bytes_from_mask16(lo_mask),
        bytes_from_mask16(hi_mask),
    );
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let expected = (lo_mask as u64) | ((hi_mask as u64) << 16);
    assert_eq!(regs.rax & 0xFFFF_FFFF, expected);
}

#[test]
fn test_vpmovmskb_all_set_all_clear() {
    // All bytes negative -> 0xFFFFFFFF ; all positive -> 0.
    let code = [0xc5, 0xfd, 0xd7, 0xc1, 0xf4]; // VPMOVMSKB EAX, YMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    kpmb_set(&mut vcpu, 1, u128::MAX, u128::MAX);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0xFFFF_FFFF);

    let code2 = [0xc5, 0xfd, 0xd7, 0xc1, 0xf4];
    let (mut vcpu2, _) = setup_vm(&code2, None);
    kpmb_set(&mut vcpu2, 1, 0, 0);
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();
    assert_eq!(regs2.rax & 0xFFFF_FFFF, 0);
}
