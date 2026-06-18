//! Tests for PMADDWD instruction (MMX).
//!
//! PMADDWD - Multiply and Add Packed Integers
//!
//! Multiplies the four signed 16-bit words of the source and destination pairwise,
//! then adds adjacent 32-bit products:
//!   DEST[31:0]  := a0*b0 + a1*b1
//!   DEST[63:32] := a2*b2 + a3*b3
//!
//! Opcode (MMX form): NP 0F F5 /r  (PMADDWD mm, mm/m64)
//!
//! Known-answer value tests via MOVQ memory round-trip.
//!
//! KNOWN BUG (tests #[ignore]d below): the emulator's `pmaddwd` handler
//! (src/backend/emulator/x86_64/insn/simd/sse.rs) only implements the SSE2 form
//! `66 0F F5 /r` and explicitly errors with "PMADDWD requires 66 prefix" on the
//! no-prefix MMX encoding. The MMX form (NP 0F F5 /r) is a valid, original-MMX
//! instruction and should compute over the 64-bit MM registers. Until the handler
//! supports the NP/MMX form these tests cannot run; they encode the correct
//! known-answer values so they will pass once the bug is fixed.

use crate::common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

fn run_pmaddwd(mm0: u64, mm1: u64) -> u64 {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xf5, 0xc1, // PMADDWD MM0, MM1
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
fn test_pmaddwd_basic() {
    // MM0 words [2,3,100,-50], MM1 words [5,7,4,6]
    //   dword0 = 2*5 + 3*7 = 31
    //   dword1 = 100*4 + (-50)*6 = 400 - 300 = 100
    let r = run_pmaddwd(0xFFCE006400030002, 0x0006000400070005);
    assert_eq!(r, 0x000000640000001F, "PMADDWD basic: [31, 100]");
}

#[test]
fn test_pmaddwd_zero() {
    let r = run_pmaddwd(0x0000000000000000, 0x1234567890ABCDEF);
    assert_eq!(r, 0x0000000000000000, "PMADDWD with zero -> 0");
}

#[test]
fn test_pmaddwd_extreme() {
    // -32768 * -32768 + -32768 * -32768 = 2 * 2^30 = 2^31 = 0x80000000.
    // (This is the only input that overflows i32 for PMADDWD; result wraps to i32::MIN.)
    // dword1: 1*1 + 1*1 = 2.
    let r = run_pmaddwd(0x0001000180008000, 0x0001000180008000);
    assert_eq!(r, 0x0000000280000000, "PMADDWD extreme: [0x80000000, 2]");
}

#[test]
fn test_pmaddwd_negative_products() {
    // MM0 [-1,-1,-1,-1], MM1 [1,1,1,1]
    //   dword0 = -1 + -1 = -2 = 0xFFFFFFFE
    //   dword1 = -1 + -1 = -2 = 0xFFFFFFFE
    let r = run_pmaddwd(0xFFFFFFFFFFFFFFFF, 0x0001000100010001);
    assert_eq!(
        r, 0xFFFFFFFEFFFFFFFE,
        "PMADDWD negative products -> [-2, -2]"
    );
}
