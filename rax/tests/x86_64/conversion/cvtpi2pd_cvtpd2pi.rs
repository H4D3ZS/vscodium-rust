use rax::cpu::Registers;

use crate::common::{VCpu, get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::backend::emulator::x86_64::X86_64Vcpu;

// CVTPI2PD — Convert Packed Dword Integers to Packed Double Precision Floating-Point Values
// CVTPD2PI — Convert Packed Double Precision Floating-Point Values to Packed Dword Integers
//
// Known-answer value tests. Double precision exactly represents all i32, so these
// conversions are lossless in both directions for the values used here.
//
// Encodings:
//   CVTPI2PD XMM0, MM0 : 66 0F 2A C0  (MM0 two i32 -> XMM0 two f64, full 128 bits)
//   CVTPD2PI MM0, XMM0 : 66 0F 2D C0  (XMM0 two f64 -> MM0 two i32)

fn set_mm(vcpu: &mut X86_64Vcpu, index: usize, value: u64) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.mm[index] = value;
    vcpu.set_regs(&regs).unwrap();
}

fn get_mm(regs: &Registers, index: usize) -> u64 {
    regs.mm[index]
}

fn pack_i32x2(lo: i32, hi: i32) -> u64 {
    (lo as u32 as u64) | ((hi as u32 as u64) << 32)
}

/// Pack two f64 lanes into a 128-bit XMM value (lane0 = low qword).
fn pack_f64x2(lo: f64, hi: f64) -> u128 {
    (lo.to_bits() as u128) | ((hi.to_bits() as u128) << 64)
}

// ============================================================================
// CVTPI2PD — integer -> double precision
// ============================================================================

#[test]
fn test_cvtpi2pd_basic() {
    // MM0 = [1, 10] -> XMM0 = [1.0, 10.0]
    let code = [0x66, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PD XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(1, 10));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        pack_f64x2(1.0, 10.0),
        "CVTPI2PD [1,10] -> [1.0,10.0]"
    );
}

#[test]
fn test_cvtpi2pd_zero() {
    let code = [0x66, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PD XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(0, 0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        pack_f64x2(0.0, 0.0),
        "CVTPI2PD [0,0] -> [0.0,0.0]"
    );
}

#[test]
fn test_cvtpi2pd_negative() {
    let code = [0x66, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PD XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(-1, -100));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        pack_f64x2(-1.0, -100.0),
        "CVTPI2PD [-1,-100]"
    );
}

#[test]
fn test_cvtpi2pd_max_positive_i32() {
    // 2147483647 is exactly representable in f64.
    let code = [0x66, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PD XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(i32::MAX, 0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        pack_f64x2(2_147_483_647.0, 0.0),
        "CVTPI2PD i32::MAX -> 2147483647.0"
    );
}

#[test]
fn test_cvtpi2pd_min_negative_i32() {
    // -2147483648 is exactly representable in f64.
    let code = [0x66, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PD XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(i32::MIN, 0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        pack_f64x2(-2_147_483_648.0, 0.0),
        "CVTPI2PD i32::MIN -> -2147483648.0"
    );
}

// ============================================================================
// CVTPD2PI — double precision -> integer (round to nearest even by default)
// ============================================================================

#[test]
fn test_cvtpd2pi_basic() {
    // XMM0 = [5.0, 10.0] -> MM0 = [5, 10]
    let code = [0x66, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTPD2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f64x2(5.0, 10.0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(5, 10),
        "CVTPD2PI [5.0,10.0] -> [5,10]"
    );
}

#[test]
fn test_cvtpd2pi_negative() {
    let code = [0x66, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTPD2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f64x2(-12.0, -34.0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(-12, -34),
        "CVTPD2PI [-12.0,-34.0]"
    );
}

#[test]
fn test_cvtpd2pi_round_half_to_even() {
    // 0.5 -> 0, 1.5 -> 2 (round to nearest even).
    let code = [0x66, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTPD2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f64x2(0.5, 1.5));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(0, 2),
        "CVTPD2PI [0.5,1.5] round-even -> [0,2]"
    );
}

#[test]
fn test_cvtpi2pd_then_cvtpd2pi_roundtrip() {
    // [7, -9] -> doubles -> back to [7, -9].
    let code = [
        0x66, 0x0f, 0x2a, 0xc0, // CVTPI2PD XMM0, MM0
        0x66, 0x0f, 0x2d, 0xc8, // CVTPD2PI MM1, XMM0
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(7, -9));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 1),
        pack_i32x2(7, -9),
        "CVTPI2PD/CVTPD2PI roundtrip [7,-9]"
    );
}
