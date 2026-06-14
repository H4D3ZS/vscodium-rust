//! Tests for PSRAW / PSRAD with immediate count (MMX).
//!
//! Shift packed words/doublewords right *arithmetic* by an immediate: the sign bit is
//! replicated into the vacated high bits. A count >= element width fills each lane with
//! its sign bit (0x0000.. for non-negative, 0xFFFF.. for negative).
//! There is no PSRAQ in MMX.
//!
//! Opcodes (immediate form, group 12/13, reg field /4 -> ModRM 0xE0 for MM0):
//!   PSRAW mm, imm8 : 0F 71 /4 ib
//!   PSRAD mm, imm8 : 0F 72 /4 ib
//!
//! Known-answer value tests via MOVQ memory round-trip.

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

fn run_shift_imm(grp: u8, imm: u8, mm0: u64) -> u64 {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, grp, 0xe0, imm, // PSRAx MM0, imm8  (reg field /4)
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, mm0);
    run_until_hlt(&mut vcpu).unwrap();
    read_mem_at_u64(&mem, 0x2010)
}

// Words lane0..3: 0x7FFF(+32767), 0xFFFF(-1), 0x8000(-32768), 0x0001(+1).
const SEED_W: u64 = 0x00018000FFFF7FFF;
// Dwords lane0,1: 0xF0000010(negative), 0x12345678(positive).
const SEED_D: u64 = 0x12345678F0000010;

#[test]
fn test_psraw_by_4() {
    // Arithmetic >> 4 per word, sign-extended.
    let r = run_shift_imm(0x71, 4, SEED_W);
    assert_eq!(r, 0x0000F800FFFF07FF, "PSRAW by 4 (sign-extended)");
}

#[test]
fn test_psraw_by_16_sign_fill() {
    // Count >= 16 fills each word with its sign bit.
    let r = run_shift_imm(0x71, 16, SEED_W);
    assert_eq!(r, 0x0000FFFFFFFF0000, "PSRAW by 16 -> sign fill");
}

#[test]
fn test_psraw_by_0_identity() {
    let r = run_shift_imm(0x71, 0, SEED_W);
    assert_eq!(r, SEED_W, "PSRAW by 0 is identity");
}

#[test]
fn test_psrad_by_4() {
    // Arithmetic >> 4 per dword, sign-extended.
    let r = run_shift_imm(0x72, 4, SEED_D);
    assert_eq!(r, 0x01234567FF000001, "PSRAD by 4 (sign-extended)");
}

#[test]
fn test_psrad_by_32_sign_fill() {
    // Count >= 32 fills each dword with its sign bit.
    let r = run_shift_imm(0x72, 32, SEED_D);
    assert_eq!(r, 0x00000000FFFFFFFF, "PSRAD by 32 -> sign fill");
}
