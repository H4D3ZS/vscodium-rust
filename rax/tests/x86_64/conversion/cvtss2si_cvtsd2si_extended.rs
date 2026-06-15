use rax::cpu::Registers;

use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};

// CVTSS2SI — Convert Scalar Single Precision Floating-Point Value to Signed Integer
// CVTSD2SI — Convert Scalar Double Precision Floating-Point Value to Signed Integer
// CVTTSS2SI / CVTTSD2SI — same, but truncating (round toward zero)
//
// These are known-answer value tests: XMM0 is seeded with an exact IEEE-754 bit
// pattern via set_xmm, the conversion instruction is executed, and the resulting
// general-purpose register value is asserted exactly.
//
// Encodings used:
//   CVTSS2SI  EAX, XMM0   : F3 0F 2D C0
//   CVTSS2SI  RAX, XMM0   : F3 REX.W 0F 2D C0
//   CVTSD2SI  EAX, XMM0   : F2 0F 2D C0
//   CVTSD2SI  RAX, XMM0   : F2 REX.W 0F 2D C0
//   CVTTSS2SI EAX, XMM0   : F3 0F 2C C0
//   CVTTSS2SI RAX, XMM0   : F3 REX.W 0F 2C C0
//   CVTTSD2SI EAX, XMM0   : F2 0F 2C C0
//   CVTTSD2SI RAX, XMM0   : F2 REX.W 0F 2C C0

// IEEE-754 single-precision bit patterns (low 32 bits of XMM).
const F32_0: u128 = 0x0000_0000; // 0.0f
const F32_1: u128 = 0x3F80_0000; // 1.0f
const F32_NEG1: u128 = 0xBF80_0000; // -1.0f
const F32_2_5: u128 = 0x4020_0000; // 2.5f
const F32_NEG2_5: u128 = 0xC020_0000; // -2.5f
const F32_42: u128 = 0x4228_0000; // 42.0f
const F32_NEG100: u128 = 0xC2C8_0000; // -100.0f

// IEEE-754 double-precision bit patterns (low 64 bits of XMM).
const F64_0: u128 = 0x0000_0000_0000_0000; // 0.0
const F64_1: u128 = 0x3FF0_0000_0000_0000; // 1.0
const F64_NEG1: u128 = 0xBFF0_0000_0000_0000; // -1.0
const F64_2_5: u128 = 0x4004_0000_0000_0000; // 2.5
const F64_NEG2_5: u128 = 0xC004_0000_0000_0000; // -2.5
const F64_42: u128 = 0x4045_0000_0000_0000; // 42.0
const F64_NEG100: u128 = 0xC059_0000_0000_0000; // -100.0
const F64_BIG: u128 = 0x4170_0000_0000_0000; // 16777216.0 (2^24)

/// Run a CVT instruction with XMM0 seeded to `xmm0_bits` and return final regs.
fn run_cvt(code: &[u8], xmm0_bits: u128) -> Registers {
    let (mut vcpu, mem) = setup_vm(code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, xmm0_bits);
    run_until_hlt(&mut vcpu).unwrap()
}

// ============================================================================
// CVTSS2SI — round per default MXCSR (round to nearest even)
// ============================================================================

#[test]
fn test_cvtss2si_32bit_zero() {
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_0);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0, "CVTSS2SI 0.0f -> 0");
}

#[test]
fn test_cvtss2si_64bit_zero() {
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI RAX, XMM0
    let regs = run_cvt(&code, F32_0);
    assert_eq!(regs.rax, 0, "CVTSS2SI(64) 0.0f -> 0");
}

#[test]
fn test_cvtsd2si_32bit_zero() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_0);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0, "CVTSD2SI 0.0 -> 0");
}

#[test]
fn test_cvtsd2si_64bit_zero() {
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI RAX, XMM0
    let regs = run_cvt(&code, F64_0);
    assert_eq!(regs.rax, 0, "CVTSD2SI(64) 0.0 -> 0");
}

#[test]
fn test_cvtss2si_positive_one() {
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 1, "CVTSS2SI 1.0f -> 1");
}

#[test]
fn test_cvtss2si_negative_one() {
    // EAX result is the 32-bit two's-complement of -1 = 0xFFFFFFFF.
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_NEG1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0xFFFF_FFFF, "CVTSS2SI -1.0f -> -1");
}

#[test]
fn test_cvtss2si_64bit_negative_one() {
    // RAX result is full 64-bit -1 = 0xFFFFFFFFFFFFFFFF (sign extended).
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI RAX, XMM0
    let regs = run_cvt(&code, F32_NEG1);
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF, "CVTSS2SI(64) -1.0f -> -1");
}

#[test]
fn test_cvtss2si_42() {
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_42);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 42, "CVTSS2SI 42.0f -> 42");
}

#[test]
fn test_cvtss2si_negative_100() {
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI RAX, XMM0
    let regs = run_cvt(&code, F32_NEG100);
    assert_eq!(regs.rax as i64, -100, "CVTSS2SI -100.0f -> -100");
}

#[test]
fn test_cvtss2si_round_half_to_even_2_5() {
    // 2.5 rounds to nearest-even -> 2.
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_2_5);
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        2,
        "CVTSS2SI 2.5f round-to-even -> 2"
    );
}

#[test]
fn test_cvtss2si_round_half_to_even_neg2_5() {
    // -2.5 rounds to nearest-even -> -2.
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_NEG2_5);
    assert_eq!(regs.rax as i32, -2, "CVTSS2SI -2.5f round-to-even -> -2");
}

// ============================================================================
// CVTSD2SI — round per default MXCSR
// ============================================================================

#[test]
fn test_cvtsd2si_positive_one() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 1, "CVTSD2SI 1.0 -> 1");
}

#[test]
fn test_cvtsd2si_negative_one() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_NEG1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0xFFFF_FFFF, "CVTSD2SI -1.0 -> -1");
}

#[test]
fn test_cvtsd2si_64bit_negative_one() {
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI RAX, XMM0
    let regs = run_cvt(&code, F64_NEG1);
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF, "CVTSD2SI(64) -1.0 -> -1");
}

#[test]
fn test_cvtsd2si_42() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_42);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 42, "CVTSD2SI 42.0 -> 42");
}

#[test]
fn test_cvtsd2si_negative_100() {
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI RAX, XMM0
    let regs = run_cvt(&code, F64_NEG100);
    assert_eq!(regs.rax as i64, -100, "CVTSD2SI -100.0 -> -100");
}

#[test]
fn test_cvtsd2si_round_half_to_even_2_5() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_2_5);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 2, "CVTSD2SI 2.5 round-to-even -> 2");
}

#[test]
fn test_cvtsd2si_round_half_to_even_neg2_5() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_NEG2_5);
    assert_eq!(regs.rax as i32, -2, "CVTSD2SI -2.5 round-to-even -> -2");
}

#[test]
fn test_cvtsd2si_to_i64_large() {
    // 16777216.0 = 2^24, exactly representable; fits i32 too but exercises 64-bit path.
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI RAX, XMM0
    let regs = run_cvt(&code, F64_BIG);
    assert_eq!(regs.rax, 16_777_216, "CVTSD2SI(64) 2^24 -> 16777216");
}

// ============================================================================
// CVTTSS2SI / CVTTSD2SI — truncation (round toward zero)
// ============================================================================

#[test]
fn test_cvttss2si_truncate_2_5() {
    // 2.5 truncates toward zero -> 2.
    let code = [0xf3, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_2_5);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 2, "CVTTSS2SI 2.5f trunc -> 2");
}

#[test]
fn test_cvttss2si_truncate_neg2_5() {
    // -2.5 truncates toward zero -> -2.
    let code = [0xf3, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_NEG2_5);
    assert_eq!(regs.rax as i32, -2, "CVTTSS2SI -2.5f trunc -> -2");
}

#[test]
fn test_cvttss2si_one() {
    let code = [0xf3, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSS2SI EAX, XMM0
    let regs = run_cvt(&code, F32_1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 1, "CVTTSS2SI 1.0f -> 1");
}

#[test]
fn test_cvttss2si_64bit_neg2_5() {
    let code = [0xf3, 0x48, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSS2SI RAX, XMM0
    let regs = run_cvt(&code, F32_NEG2_5);
    assert_eq!(regs.rax as i64, -2, "CVTTSS2SI(64) -2.5f trunc -> -2");
}

#[test]
fn test_cvttsd2si_truncate_2_5() {
    let code = [0xf2, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_2_5);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 2, "CVTTSD2SI 2.5 trunc -> 2");
}

#[test]
fn test_cvttsd2si_truncate_neg2_5() {
    let code = [0xf2, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_NEG2_5);
    assert_eq!(regs.rax as i32, -2, "CVTTSD2SI -2.5 trunc -> -2");
}

#[test]
fn test_cvttsd2si_one() {
    let code = [0xf2, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSD2SI EAX, XMM0
    let regs = run_cvt(&code, F64_1);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 1, "CVTTSD2SI 1.0 -> 1");
}

#[test]
fn test_cvttsd2si_64bit_42() {
    let code = [0xf2, 0x48, 0x0f, 0x2c, 0xc0, 0xf4]; // CVTTSD2SI RAX, XMM0
    let regs = run_cvt(&code, F64_42);
    assert_eq!(regs.rax, 42, "CVTTSD2SI(64) 42.0 trunc -> 42");
}

// ============================================================================
// Source operand must remain unchanged by the conversion.
// ============================================================================

#[test]
fn test_cvtss2si_source_unchanged() {
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    // High lane bits set to verify they are preserved.
    let seed: u128 = (0xDEAD_BEEF_CAFE_BABEu128 << 64) | F32_42;
    let regs = run_cvt(&code, seed);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 42, "CVTSS2SI 42.0f -> 42");
    assert_eq!(
        get_xmm(&regs, 0),
        seed,
        "CVTSS2SI must not modify XMM source"
    );
}

#[test]
fn test_cvtsd2si_source_unchanged() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let seed: u128 = (0x1122_3344_5566_7788u128 << 64) | F64_42;
    let regs = run_cvt(&code, seed);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 42, "CVTSD2SI 42.0 -> 42");
    assert_eq!(
        get_xmm(&regs, 0),
        seed,
        "CVTSD2SI must not modify XMM source"
    );
}
