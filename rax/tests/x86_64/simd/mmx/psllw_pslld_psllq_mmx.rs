//! Tests for PSLLW / PSLLD / PSLLQ with immediate count (MMX).
//!
//! Shift packed words/doublewords/quadword left logical by an immediate. Bits shifted
//! out are discarded, zeros shifted in. A count >= element width produces all zeros.
//!
//! Opcodes (immediate form, group 12/13/14):
//!   PSLLW mm, imm8 : 0F 71 /6 ib  (ModRM 0xF0 for MM0)
//!   PSLLD mm, imm8 : 0F 72 /6 ib  (ModRM 0xF0 for MM0)
//!   PSLLQ mm, imm8 : 0F 73 /6 ib  (ModRM 0xF0 for MM0)
//!
//! Known-answer value tests via MOVQ memory round-trip.

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

/// Run a shift-immediate `0F <grp> F0 <imm>` on MM0 seeded with `mm0`.
fn run_shift_imm(grp: u8, imm: u8, mm0: u64) -> u64 {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, grp, 0xf0, imm, // PSLLx MM0, imm8  (reg field /6)
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
fn test_psllw_by_4() {
    // Each 16-bit word << 4 (mod 16 bits).
    let r = run_shift_imm(0x71, 4, SEED);
    assert_eq!(r, 0x0FF0000023400010, "PSLLW by 4");
}

#[test]
fn test_psllw_by_16_zeroes() {
    // Count >= 16 zeroes every word.
    let r = run_shift_imm(0x71, 16, SEED);
    assert_eq!(r, 0x0000000000000000, "PSLLW by 16 -> all zero");
}

#[test]
fn test_psllw_by_0_identity() {
    let r = run_shift_imm(0x71, 0, SEED);
    assert_eq!(r, SEED, "PSLLW by 0 is identity");
}

#[test]
fn test_pslld_by_4() {
    // Each 32-bit dword << 4 (mod 32 bits).
    let r = run_shift_imm(0x72, 4, SEED);
    assert_eq!(r, 0x0FFF000023400010, "PSLLD by 4");
}

#[test]
fn test_pslld_by_32_zeroes() {
    let r = run_shift_imm(0x72, 32, SEED);
    assert_eq!(r, 0x0000000000000000, "PSLLD by 32 -> all zero");
}

#[test]
fn test_psllq_by_4() {
    // Whole 64-bit value << 4.
    let r = run_shift_imm(0x73, 4, SEED);
    assert_eq!(r, 0x0FFF000123400010, "PSLLQ by 4");
}

#[test]
fn test_psllq_by_64_zeroes() {
    let r = run_shift_imm(0x73, 64, SEED);
    assert_eq!(r, 0x0000000000000000, "PSLLQ by 64 -> all zero");
}
