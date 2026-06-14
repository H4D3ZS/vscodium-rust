use rax::cpu::Registers;

use crate::common::{VCpu, get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::backend::emulator::x86_64::X86_64Vcpu;

// CVTPI2PS — Convert Packed Dword Integers to Packed Single Precision Floating-Point Values
// CVTPS2PI — Convert Packed Single Precision Floating-Point Values to Packed Dword Integers
//
// Known-answer value tests. MMX registers (mm0-7) live in Registers.mm; we seed and
// read them via small file-local helpers that round-trip the whole Registers struct.
//
// Encodings:
//   CVTPI2PS XMM0, MM0 : NP 0F 2A C0  (MM0 two i32 -> XMM0[63:0] two f32, high qword kept)
//   CVTPS2PI MM0, XMM0 : NP 0F 2D C0  (XMM0[63:0] two f32 -> MM0 two i32)

/// Seed an MMX register (mm0-7) with an exact 64-bit value.
fn set_mm(vcpu: &mut X86_64Vcpu, index: usize, value: u64) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.mm[index] = value;
    vcpu.set_regs(&regs).unwrap();
}

/// Read an MMX register (mm0-7) from final register state.
fn get_mm(regs: &Registers, index: usize) -> u64 {
    regs.mm[index]
}

/// Pack two i32 lanes into a 64-bit MMX value (lane0 = low dword).
fn pack_i32x2(lo: i32, hi: i32) -> u64 {
    (lo as u32 as u64) | ((hi as u32 as u64) << 32)
}

/// Pack two f32 lanes into the low 64 bits of an XMM register (lane0 = low dword).
fn pack_f32x2(lo: f32, hi: f32) -> u128 {
    (lo.to_bits() as u128) | ((hi.to_bits() as u128) << 32)
}

// ============================================================================
// CVTPI2PS — integer -> single precision
// ============================================================================

#[test]
fn test_cvtpi2ps_basic() {
    // MM0 = [1, 2] -> XMM0[63:0] = [1.0f, 2.0f]
    let code = [0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PS XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(1, 2));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let lo64 = get_xmm(&regs, 0) as u64;
    assert_eq!(
        lo64,
        pack_f32x2(1.0, 2.0) as u64,
        "CVTPI2PS [1,2] -> [1.0f,2.0f]"
    );
}

#[test]
fn test_cvtpi2ps_zero() {
    let code = [0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PS XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(0, 0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let lo64 = get_xmm(&regs, 0) as u64;
    assert_eq!(lo64, 0, "CVTPI2PS [0,0] -> [0.0f,0.0f]");
}

#[test]
fn test_cvtpi2ps_negative() {
    // MM0 = [-1, -100] -> [-1.0f, -100.0f]
    let code = [0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PS XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(-1, -100));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let lo64 = get_xmm(&regs, 0) as u64;
    assert_eq!(lo64, pack_f32x2(-1.0, -100.0) as u64, "CVTPI2PS [-1,-100]");
}

#[test]
fn test_cvtpi2ps_mixed() {
    // MM0 = [42, -7] -> [42.0f, -7.0f]
    let code = [0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PS XMM0, MM0
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(42, -7));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let lo64 = get_xmm(&regs, 0) as u64;
    assert_eq!(lo64, pack_f32x2(42.0, -7.0) as u64, "CVTPI2PS [42,-7]");
}

#[test]
fn test_cvtpi2ps_high_quadword_unchanged() {
    // High 64 bits of XMM0 must be preserved (only low qword is written).
    let code = [0x0f, 0x2a, 0xc0, 0xf4]; // CVTPI2PS XMM0, MM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    let high: u128 = 0xDEAD_BEEF_CAFE_BABE;
    set_xmm(&mem, &mut vcpu, 0, high << 64);
    set_mm(&mut vcpu, 0, pack_i32x2(1, 2));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let val = get_xmm(&regs, 0);
    assert_eq!(val >> 64, high, "CVTPI2PS must preserve high quadword");
    assert_eq!(
        val as u64,
        pack_f32x2(1.0, 2.0) as u64,
        "CVTPI2PS low qword"
    );
}

// ============================================================================
// CVTPS2PI — single precision -> integer (round to nearest even by default)
// ============================================================================

#[test]
fn test_cvtps2pi_basic() {
    // XMM0[63:0] = [5.0f, 10.0f] -> MM0 = [5, 10]
    let code = [0x0f, 0x2d, 0xc0, 0xf4]; // CVTPS2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f32x2(5.0, 10.0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(5, 10),
        "CVTPS2PI [5.0,10.0] -> [5,10]"
    );
}

#[test]
fn test_cvtps2pi_negative() {
    let code = [0x0f, 0x2d, 0xc0, 0xf4]; // CVTPS2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f32x2(-3.0, -8.0));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(-3, -8),
        "CVTPS2PI [-3.0,-8.0] -> [-3,-8]"
    );
}

#[test]
fn test_cvtps2pi_round_half_to_even() {
    // 2.5 -> 2, 3.5 -> 4 (round to nearest even).
    let code = [0x0f, 0x2d, 0xc0, 0xf4]; // CVTPS2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    set_xmm(&mem, &mut vcpu, 0, pack_f32x2(2.5, 3.5));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(2, 4),
        "CVTPS2PI [2.5,3.5] round-even -> [2,4]"
    );
}

#[test]
fn test_cvtps2pi_uses_low_quadword_only() {
    // High 64 bits of XMM source are ignored.
    let code = [0x0f, 0x2d, 0xc0, 0xf4]; // CVTPS2PI MM0, XMM0
    let (mut vcpu, mem) = setup_vm(&code, Some(Registers::default()));
    let val: u128 = (0xFFFF_FFFF_FFFF_FFFFu128 << 64) | (pack_f32x2(7.0, 9.0) as u64 as u128);
    set_xmm(&mem, &mut vcpu, 0, val);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 0),
        pack_i32x2(7, 9),
        "CVTPS2PI ignores high qword"
    );
}

#[test]
fn test_cvtpi2ps_then_cvtps2pi_roundtrip() {
    // [3, -4] -> floats -> back to [3, -4].
    let code = [
        0x0f, 0x2a, 0xc0, // CVTPI2PS XMM0, MM0
        0x0f, 0x2d, 0xc8, // CVTPS2PI MM1, XMM0
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, Some(Registers::default()));
    set_mm(&mut vcpu, 0, pack_i32x2(3, -4));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_mm(&regs, 1),
        pack_i32x2(3, -4),
        "CVTPI2PS/CVTPS2PI roundtrip [3,-4]"
    );
}
