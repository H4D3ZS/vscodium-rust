//! Tests for PSRLW / PSRLD / PSRLQ with immediate count (MMX).
//!
//! Shift packed words/doublewords/quadword right *logical* by an immediate. Zeros are
//! shifted in from the left. A count >= element width produces all zeros.
//!
//! Opcodes (immediate form, group 12/13/14, reg field /2 -> ModRM 0xD0 for MM0):
//!   PSRLW mm, imm8 : 0F 71 /2 ib
//!   PSRLD mm, imm8 : 0F 72 /2 ib
//!   PSRLQ mm, imm8 : 0F 73 /2 ib
//!
//! Known-answer value tests via MOVQ memory round-trip.

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

fn run_shift_imm(grp: u8, imm: u8, mm0: u64) -> u64 {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, grp, 0xd0, imm, // PSRLx MM0, imm8  (reg field /2)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, mm0);
    run_until_hlt(&mut vcpu).unwrap();
    read_mem_at_u64(&mem, 0x2010)
}

const SEED: u64 = 0x00FFF00012340001;

#[test]
fn test_psrlw_by_4() {
    // Each 16-bit word >> 4 logical.
    let r = run_shift_imm(0x71, 4, SEED);
    assert_eq!(r, 0x000F0F0001230000, "PSRLW by 4");
}

#[test]
fn test_psrlw_by_16_zeroes() {
    let r = run_shift_imm(0x71, 16, SEED);
    assert_eq!(r, 0x0000000000000000, "PSRLW by 16 -> all zero");
}

#[test]
fn test_psrlw_by_0_identity() {
    let r = run_shift_imm(0x71, 0, SEED);
    assert_eq!(r, SEED, "PSRLW by 0 is identity");
}

#[test]
fn test_psrld_by_4() {
    // Each 32-bit dword >> 4 logical.
    let r = run_shift_imm(0x72, 4, SEED);
    assert_eq!(r, 0x000FFF0001234000, "PSRLD by 4");
}

#[test]
fn test_psrld_by_32_zeroes() {
    let r = run_shift_imm(0x72, 32, SEED);
    assert_eq!(r, 0x0000000000000000, "PSRLD by 32 -> all zero");
}

#[test]
fn test_psrlq_by_4() {
    // Whole 64-bit value >> 4 logical.
    let r = run_shift_imm(0x73, 4, SEED);
    assert_eq!(r, 0x000FFF0001234000, "PSRLQ by 4");
}

#[test]
fn test_psrlq_by_64_zeroes() {
    let r = run_shift_imm(0x73, 64, SEED);
    assert_eq!(r, 0x0000000000000000, "PSRLQ by 64 -> all zero");
}
