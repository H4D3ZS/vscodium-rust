//! Tests for PCMPGTB / PCMPGTW / PCMPGTD instructions (MMX).
//!
//! Compare packed signed integers for greater-than. Each result lane is all-ones
//! (0xFF.. / 0xFFFF.. / 0xFFFFFFFF) when dest lane > src lane (signed), else all-zero.
//!
//! Opcodes: PCMPGTB 0F 64 /r, PCMPGTW 0F 65 /r, PCMPGTD 0F 66 /r
//!
//! Known-answer value tests: MMX registers are seeded from memory with MOVQ and the
//! exact 64-bit result is read back from memory and asserted.

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

/// Run `0F <op> C1` (MMX MM0, MM1) with the two operands seeded from memory.
fn run_cmp(op: u8, mm0: u64, mm1: u64) -> u64 {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, op, 0xc1, // PCMPGTx MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mm_via_mem(&mem, 0x2000, mm0);
    write_mm_via_mem(&mem, 0x2008, mm1);
    run_until_hlt(&mut vcpu).unwrap();
    read_mem_at_u64(&mem, 0x2010)
}

#[test]
fn test_pcmpgtb_mixed() {
    // Signed byte compare. Equal lanes -> 0, dest>src -> 0xFF, dest<src -> 0.
    let r = run_cmp(0x64, 0x64810AFF007F8005, 0x648009FE01807F05);
    assert_eq!(r, 0x00FFFFFF00FF0000, "PCMPGTB mixed signed bytes");
}

#[test]
fn test_pcmpgtb_all_equal() {
    let r = run_cmp(0x64, 0x1122334455667788, 0x1122334455667788);
    assert_eq!(r, 0x0000000000000000, "PCMPGTB equal -> all zero");
}

#[test]
fn test_pcmpgtb_all_greater() {
    // Every dest byte (0x7F = +127) greater than every src byte (0x80 = -128).
    let r = run_cmp(0x64, 0x7F7F7F7F7F7F7F7F, 0x8080808080808080);
    assert_eq!(r, 0xFFFFFFFFFFFFFFFF, "PCMPGTB +127 > -128 -> all ones");
}

#[test]
fn test_pcmpgtw_mixed() {
    // Lanes: 100>50 (T), -32768>32767 (F), 32767>-32768 (T), 5>5 (F).
    let r = run_cmp(0x65, 0x00057FFF80000064, 0x000580007FFF0032);
    assert_eq!(r, 0x0000FFFF0000FFFF, "PCMPGTW mixed signed words");
}

#[test]
fn test_pcmpgtd_mixed() {
    // Lane0: 5 > -1 (T); Lane1: INT_MIN > INT_MAX (F).
    let r = run_cmp(0x66, 0x8000000000000005, 0x7FFFFFFFFFFFFFFF);
    assert_eq!(r, 0x00000000FFFFFFFF, "PCMPGTD 5 > -1, INT_MIN < INT_MAX");
}

#[test]
fn test_pcmpgtd_equal() {
    let r = run_cmp(0x66, 0x0000002A0000002A, 0x0000002A0000002A);
    assert_eq!(r, 0x0000000000000000, "PCMPGTD equal -> all zero");
}
