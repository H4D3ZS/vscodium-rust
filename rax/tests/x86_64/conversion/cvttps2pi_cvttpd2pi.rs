use rax::cpu::Registers;

use crate::common::{run_until_hlt, set_xmm, setup_vm};

// CVTTPS2PI — Convert With Truncation Packed Single Precision FP to Packed Dword Integers
// CVTTPD2PI — Convert With Truncation Packed Double Precision FP to Packed Dword Integers
//
// These always truncate toward zero (ignore MXCSR rounding control). Known-answer
// value tests assert the exact 64-bit MMX result.
//
// Encodings:
//   CVTTPS2PI MM0, XMM0 : NP 0F 2C C0  (low qword of XMM0 = two f32 -> MM0 two i32)
//   CVTTPD2PI MM0, XMM0 : 66 0F 2C C0  (XMM0 two f64 -> MM0 two i32)

fn get_mm(regs: &Registers, index: usize) -> u64 {
    regs.mm[index]
}

fn pack_i32x2(lo: i32, hi: i32) -> u64 {
    (lo as u32 as u64) | ((hi as u32 as u64) << 32)
}

fn pack_f32x2(lo: f32, hi: f32) -> u128 {
    (lo.to_bits() as u128) | ((hi.to_bits() as u128) << 32)
}

fn pack_f64x2(lo: f64, hi: f64) -> u128 {
    (lo.to_bits() as u128) | ((hi.to_bits() as u128) << 64)
}

fn run_ps(seed_lo64: u128) -> Registers {
    let code = [0x0f, 0x2c, 0xc0, 0xf4]; // CVTTPS2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, seed_lo64);
    run_until_hlt(&mut vcpu).unwrap()
}

fn run_pd(seed: u128) -> Registers {
    let code = [0x66, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTPD2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, seed);
    run_until_hlt(&mut vcpu).unwrap()
}

// ============================================================================
// CVTTPS2PI — truncation
// ============================================================================

#[test]
fn test_cvttps2pi_zero() {
    let regs = run_ps(pack_f32x2(0.0, 0.0));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(0, 0),
        "CVTTPS2PI [0,0] -> [0,0]"
    );
}

#[test]
fn test_cvttps2pi_ones() {
    let regs = run_ps(pack_f32x2(1.0, -1.0));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(1, -1),
        "CVTTPS2PI [1.0,-1.0] -> [1,-1]"
    );
}

#[test]
fn test_cvttps2pi_truncation_positive() {
    // 1.9 -> 1, 2.9 -> 2 (truncate, not round).
    let regs = run_ps(pack_f32x2(1.9, 2.9));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(1, 2),
        "CVTTPS2PI [1.9,2.9] trunc -> [1,2]"
    );
}

#[test]
fn test_cvttps2pi_truncation_negative() {
    // -1.9 -> -1, -2.9 -> -2 (truncate toward zero).
    let regs = run_ps(pack_f32x2(-1.9, -2.9));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(-1, -2),
        "CVTTPS2PI [-1.9,-2.9] trunc -> [-1,-2]"
    );
}

#[test]
fn test_cvttps2pi_half_truncates_down() {
    // 2.5 truncates to 2 (CVTPS2PI would round to even = 2 here, but 0.5 -> 0 differs).
    let regs = run_ps(pack_f32x2(2.5, 0.5));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(2, 0),
        "CVTTPS2PI [2.5,0.5] trunc -> [2,0]"
    );
}

#[test]
fn test_cvttps2pi_small_fractions() {
    // 0.1 -> 0, -0.1 -> 0.
    let regs = run_ps(pack_f32x2(0.1, -0.1));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(0, 0),
        "CVTTPS2PI [0.1,-0.1] trunc -> [0,0]"
    );
}

#[test]
fn test_cvttps2pi_uses_low_quadword_only() {
    // High 64 bits of XMM source ignored.
    let seed: u128 = (0xFFFF_FFFF_FFFF_FFFFu128 << 64) | (pack_f32x2(6.7, -8.3) as u64 as u128);
    let regs = run_ps(seed);
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(6, -8),
        "CVTTPS2PI ignores high qword"
    );
}

// ============================================================================
// CVTTPD2PI — truncation
// ============================================================================

#[test]
fn test_cvttpd2pi_zero() {
    let regs = run_pd(pack_f64x2(0.0, 0.0));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(0, 0),
        "CVTTPD2PI [0,0] -> [0,0]"
    );
}

#[test]
fn test_cvttpd2pi_ones() {
    let regs = run_pd(pack_f64x2(1.0, -1.0));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(1, -1),
        "CVTTPD2PI [1.0,-1.0] -> [1,-1]"
    );
}

#[test]
fn test_cvttpd2pi_truncation_positive() {
    // 1.5 -> 1, 3.5 -> 3 (truncate).
    let regs = run_pd(pack_f64x2(1.5, 3.5));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(1, 3),
        "CVTTPD2PI [1.5,3.5] trunc -> [1,3]"
    );
}

#[test]
fn test_cvttpd2pi_truncation_negative() {
    // -1.5 -> -1, -3.5 -> -3 (truncate toward zero).
    let regs = run_pd(pack_f64x2(-1.5, -3.5));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(-1, -3),
        "CVTTPD2PI [-1.5,-3.5] trunc -> [-1,-3]"
    );
}

#[test]
fn test_cvttpd2pi_small_fractions() {
    // 0.9 -> 0, -0.9 -> 0.
    let regs = run_pd(pack_f64x2(0.9, -0.9));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(0, 0),
        "CVTTPD2PI [0.9,-0.9] trunc -> [0,0]"
    );
}

#[test]
fn test_cvttpd2pi_larger_values() {
    let regs = run_pd(pack_f64x2(123.99, -456.99));
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(123, -456),
        "CVTTPD2PI [123.99,-456.99] -> [123,-456]"
    );
}

/// Demonstrates the CVTT (truncate) vs CVT (round-to-nearest-even) difference:
/// 2.9 truncates to 2 with CVTTPS2PI but rounds to 3 with CVTPS2PI.
#[test]
fn test_cvttps2pi_vs_cvtps2pi_difference() {
    let code_t = [0x0f, 0x2c, 0xc0, 0xf4]; // CVTTPS2PI MM0, XMM0 (truncate)
    let (mut vcpu, mem) = setup_vm(&code_t, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f32x2(2.9, 2.9));
    let regs_t = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs_t, 0),
        pack_i32x2(2, 2),
        "CVTTPS2PI 2.9 -> 2 (truncate)"
    );

    let code_r = [0x0f, 0x2d, 0xc0, 0xf4]; // CVTPS2PI MM0, XMM0 (round)
    let (mut vcpu, mem) = setup_vm(&code_r, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f32x2(2.9, 2.9));
    let regs_r = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs_r, 0),
        pack_i32x2(3, 3),
        "CVTPS2PI 2.9 -> 3 (round)"
    );
}
