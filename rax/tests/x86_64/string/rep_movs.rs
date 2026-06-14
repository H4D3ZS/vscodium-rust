use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REP MOVS/MOVSB/MOVSW/MOVSD/MOVSQ - Repeat Move Data from String to String
//
// Opcodes:
//   F3 A4        - REP MOVSB (repeat move byte)
//   F3 66 A5     - REP MOVSW (repeat move word)
//   F3 A5        - REP MOVSD (repeat move doubleword)
//   F3 REX.W A5  - REP MOVSQ (repeat move quadword)
//
// Operation:
// WHILE RCX != 0 DO
//   [RDI] := [RSI]
//   IF DF = 0 THEN RSI += size; RDI += size
//   ELSE RSI -= size; RDI -= size
//   RCX -= 1
// END
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REP MOVSB - Repeat Move Byte
// ============================================================================

#[test]
fn test_rep_movsb_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000 (source)
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000 (dest)
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD (clear direction flag)
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup source data: "HELLO"
    write_mem_at_u8(&mem, 0x3000, b'H');
    write_mem_at_u8(&mem, 0x3001, b'E');
    write_mem_at_u8(&mem, 0x3002, b'L');
    write_mem_at_u8(&mem, 0x3003, b'L');
    write_mem_at_u8(&mem, 0x3004, b'O');

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify data copied
    assert_eq!(read_mem_at_u8(&mem, 0x4000), b'H');
    assert_eq!(read_mem_at_u8(&mem, 0x4001), b'E');
    assert_eq!(read_mem_at_u8(&mem, 0x4002), b'L');
    assert_eq!(read_mem_at_u8(&mem, 0x4003), b'L');
    assert_eq!(read_mem_at_u8(&mem, 0x4004), b'O');

    // Verify registers updated
    assert_eq!(regs.rcx, 0, "RCX should be 0 after repeat");
    assert_eq!(regs.rsi, 0x3005, "RSI should advance by 5");
    assert_eq!(regs.rdi, 0x4005, "RDI should advance by 5");
}

#[test]
fn test_rep_movsb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB (should not execute)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Registers should not change
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_movsb_one_byte() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u8(&mem, 0x3000, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0xAA);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
}

#[test]
fn test_rep_movsb_backward() {
    // REP MOVSB with STD (decrement mode)
    let code = [
        0x48, 0xc7, 0xc6, 0x04, 0x30, 0x00, 0x00, // MOV RSI, 0x3004 (end)
        0x48, 0xc7, 0xc7, 0x04, 0x40, 0x00, 0x00, // MOV RDI, 0x4004
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfd, // STD (set direction flag)
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup source at 0x3000-0x3004: "HELLO"
    write_mem_at_u8(&mem, 0x3000, b'H');
    write_mem_at_u8(&mem, 0x3001, b'E');
    write_mem_at_u8(&mem, 0x3002, b'L');
    write_mem_at_u8(&mem, 0x3003, b'L');
    write_mem_at_u8(&mem, 0x3004, b'O');

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify data copied backward
    assert_eq!(read_mem_at_u8(&mem, 0x4000), b'H');
    assert_eq!(read_mem_at_u8(&mem, 0x4001), b'E');
    assert_eq!(read_mem_at_u8(&mem, 0x4002), b'L');
    assert_eq!(read_mem_at_u8(&mem, 0x4003), b'L');
    assert_eq!(read_mem_at_u8(&mem, 0x4004), b'O');

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFF, "RSI should decrement by 5");
    assert_eq!(regs.rdi, 0x3FFF, "RDI should decrement by 5");
}

#[test]
fn test_rep_movsb_large_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Fill source with pattern
    for i in 0..256 {
        write_mem_at_u8(&mem, 0x3000 + i, (i & 0xFF) as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify pattern copied
    for i in 0..256 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), (i & 0xFF) as u8);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_movsb_preserves_source() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0xDEADBEEF);

    run_until_hlt(&mut vcpu).unwrap();

    // Source should be unchanged
    assert_eq!(read_mem_at_u32(&mem, 0x3000), 0xDEADBEEF);
    // Dest should have copy
    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0xDEADBEEF);
}

#[test]
fn test_rep_movsb_overlapping_forward() {
    // Copy within same buffer, forward
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x02, 0x30, 0x00, 0x00, // MOV RDI, 0x3002 (overlap)
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup: ABCDEF
    write_mem_at_u8(&mem, 0x3000, b'A');
    write_mem_at_u8(&mem, 0x3001, b'B');
    write_mem_at_u8(&mem, 0x3002, b'C');
    write_mem_at_u8(&mem, 0x3003, b'D');
    write_mem_at_u8(&mem, 0x3004, b'E');
    write_mem_at_u8(&mem, 0x3005, b'F');

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result: ABABAB (forward copy propagates)
    assert_eq!(read_mem_at_u8(&mem, 0x3000), b'A');
    assert_eq!(read_mem_at_u8(&mem, 0x3001), b'B');
    assert_eq!(read_mem_at_u8(&mem, 0x3002), b'A');
    assert_eq!(read_mem_at_u8(&mem, 0x3003), b'B');
    assert_eq!(read_mem_at_u8(&mem, 0x3004), b'A');
    assert_eq!(read_mem_at_u8(&mem, 0x3005), b'B');

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsb_all_zeros() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Source is already zeros (default memory)
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all zeros copied
    for i in 0..16 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0);
    }

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsb_all_ones() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Fill source with 0xFF
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x3000 + i, 0xFF);
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0xFF);
    }
}

// ============================================================================
// REP MOVSW - Repeat Move Word
// ============================================================================

#[test]
fn test_rep_movsw_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup 4 words
    write_mem_at_u16(&mem, 0x3000, 0x1111);
    write_mem_at_u16(&mem, 0x3002, 0x2222);
    write_mem_at_u16(&mem, 0x3004, 0x3333);
    write_mem_at_u16(&mem, 0x3006, 0x4444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0x1111);
    assert_eq!(read_mem_at_u16(&mem, 0x4002), 0x2222);
    assert_eq!(read_mem_at_u16(&mem, 0x4004), 0x3333);
    assert_eq!(read_mem_at_u16(&mem, 0x4006), 0x4444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3008, "RSI advances by 8 (4 words)");
    assert_eq!(regs.rdi, 0x4008, "RDI advances by 8 (4 words)");
}

#[test]
fn test_rep_movsw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_movsw_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x06, 0x30, 0x00, 0x00, // MOV RSI, 0x3006
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3000, 0x1111);
    write_mem_at_u16(&mem, 0x3002, 0x2222);
    write_mem_at_u16(&mem, 0x3004, 0x3333);
    write_mem_at_u16(&mem, 0x3006, 0x4444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0x1111);
    assert_eq!(read_mem_at_u16(&mem, 0x4002), 0x2222);
    assert_eq!(read_mem_at_u16(&mem, 0x4004), 0x3333);
    assert_eq!(read_mem_at_u16(&mem, 0x4006), 0x4444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFE, "RSI decrements by 8");
    assert_eq!(regs.rdi, 0x3FFE, "RDI decrements by 8");
}

#[test]
fn test_rep_movsw_single() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3000, 0xABCD);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0xABCD);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3002);
    assert_eq!(regs.rdi, 0x4002);
}

#[test]
fn test_rep_movsw_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..128 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, i as u16);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..128 {
        assert_eq!(read_mem_at_u16(&mem, 0x4000 + i * 2), i as u16);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
}

// ============================================================================
// REP MOVSD - Repeat Move Doubleword
// ============================================================================

#[test]
fn test_rep_movsd_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0x11111111);
    write_mem_at_u32(&mem, 0x3004, 0x22222222);
    write_mem_at_u32(&mem, 0x3008, 0x33333333);
    write_mem_at_u32(&mem, 0x300C, 0x44444444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0x11111111);
    assert_eq!(read_mem_at_u32(&mem, 0x4004), 0x22222222);
    assert_eq!(read_mem_at_u32(&mem, 0x4008), 0x33333333);
    assert_eq!(read_mem_at_u32(&mem, 0x400C), 0x44444444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3010);
    assert_eq!(regs.rdi, 0x4010);
}

#[test]
fn test_rep_movsd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_movsd_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x0C, 0x30, 0x00, 0x00, // MOV RSI, 0x300C
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0x11111111);
    write_mem_at_u32(&mem, 0x3004, 0x22222222);
    write_mem_at_u32(&mem, 0x3008, 0x33333333);
    write_mem_at_u32(&mem, 0x300C, 0x44444444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0x11111111);
    assert_eq!(read_mem_at_u32(&mem, 0x4004), 0x22222222);
    assert_eq!(read_mem_at_u32(&mem, 0x4008), 0x33333333);
    assert_eq!(read_mem_at_u32(&mem, 0x400C), 0x44444444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFC);
    assert_eq!(regs.rdi, 0x3FFC);
}

#[test]
fn test_rep_movsd_single() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0xDEADBEEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0xDEADBEEF);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3004);
    assert_eq!(regs.rdi, 0x4004);
}

#[test]
fn test_rep_movsd_array_copy() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0A, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup array: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    for i in 0..10 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..10 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), i as u32);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3028);
    assert_eq!(regs.rdi, 0x4028);
}

#[test]
fn test_rep_movsd_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xAA000000 | i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), 0xAA000000 | i as u32);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
}

// ============================================================================
// REP MOVSQ - Repeat Move Quadword
// ============================================================================

#[test]
fn test_rep_movsq_basic() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x3008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x3010, 0x3333333333333333);
    write_mem_at_u64(&mem, 0x3018, 0x4444444444444444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, 0x4008), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&mem, 0x4010), 0x3333333333333333);
    assert_eq!(read_mem_at_u64(&mem, 0x4018), 0x4444444444444444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3020);
    assert_eq!(regs.rdi, 0x4020);
}

#[test]
fn test_rep_movsq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_rep_movsq_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x18, 0x30, 0x00, 0x00, // MOV RSI, 0x3018
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x3008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x3010, 0x3333333333333333);
    write_mem_at_u64(&mem, 0x3018, 0x4444444444444444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, 0x4008), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&mem, 0x4010), 0x3333333333333333);
    assert_eq!(read_mem_at_u64(&mem, 0x4018), 0x4444444444444444);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FF8);
    assert_eq!(regs.rdi, 0x3FF8);
}

#[test]
fn test_rep_movsq_single() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0xDEADBEEFCAFEBABE);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0xDEADBEEFCAFEBABE);
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
}

#[test]
fn test_rep_movsq_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0xBB00000000000000 | i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..32 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x4000 + i * 8),
            0xBB00000000000000 | i as u64
        );
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_rep_movsq_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Other registers preserved
    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rbx, 0x99);
}

// ============================================================================
// Mixed tests and edge cases
// ============================================================================

#[test]
fn test_rep_movs_direction_flag_affects_movement() {
    // Test that DF flag properly controls direction
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfd, // STD (direction = decrement)
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u8(&mem, 0x3000, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0xAA);
    assert_eq!(regs.rsi, 0x2FFF, "DF=1 should decrement");
    assert_eq!(regs.rdi, 0x3FFF, "DF=1 should decrement");
}

#[test]
fn test_rep_movsb_rcx_counts_iterations() {
    // Verify RCX counts down properly
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0F, 0x00, 0x00, 0x00, // MOV RCX, 15
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "RCX must be 0 after 15 iterations");
    assert_eq!(regs.rsi, 0x300F, "RSI advanced by 15");
    assert_eq!(regs.rdi, 0x400F, "RDI advanced by 15");
}

#[test]
fn test_rep_movs_different_sizes_use_different_increments() {
    // Compare byte vs dword increments
    let code1 = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB (4 bytes)
        0xf4, // HLT
    ];
    let (mut vcpu1, _) = setup_vm(&code1, None);
    let regs1 = run_until_hlt(&mut vcpu1).unwrap();

    let code2 = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD (4 dwords = 16 bytes)
        0xf4, // HLT
    ];
    let (mut vcpu2, _) = setup_vm(&code2, None);
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(regs1.rsi, 0x3004, "MOVSB advances by count");
    assert_eq!(regs2.rsi, 0x3010, "MOVSD advances by count * 4");
}

#[test]
fn test_rep_movsb_memory_boundary() {
    // Test copying across page boundary - source spans 0x5FF0-0x600F
    // Use addresses that don't overlap with code at 0x1000
    let code = [
        0x48, 0xc7, 0xc6, 0xF0, 0x5F, 0x00, 0x00, // MOV RSI, 0x5FF0
        0x48, 0xc7, 0xc7, 0x00, 0x70, 0x00, 0x00, // MOV RDI, 0x7000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Fill source spanning 0x6000 boundary (0x5FF0 to 0x600F)
    for i in 0..32 {
        write_mem_at_u8(&mem, 0x5FF0 + i, i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all data copied correctly
    for i in 0..32 {
        assert_eq!(read_mem_at_u8(&mem, 0x7000 + i), i as u8);
    }

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsw_unaligned() {
    // MOVSW with unaligned addresses
    let code = [
        0x48, 0xc7, 0xc6, 0x01, 0x30, 0x00, 0x00, // MOV RSI, 0x3001 (unaligned)
        0x48, 0xc7, 0xc7, 0x03, 0x40, 0x00, 0x00, // MOV RDI, 0x4003 (unaligned)
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3001, 0xABCD);
    write_mem_at_u16(&mem, 0x3003, 0x1234);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x4003), 0xABCD);
    assert_eq!(read_mem_at_u16(&mem, 0x4005), 0x1234);

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3005);
    assert_eq!(regs.rdi, 0x4007);
}

#[test]
fn test_rep_movsd_memcpy_pattern() {
    // Typical memcpy-style usage
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, src
        0x48, 0xc7, 0xc7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, dst
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64 (256 bytes)
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Create pattern
    for i in 0..256 {
        write_mem_at_u8(&mem, 0x3000 + i, (i ^ 0xAA) as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify exact copy
    for i in 0..256 {
        assert_eq!(
            read_mem_at_u8(&mem, 0x5000 + i),
            (i ^ 0xAA) as u8,
            "Mismatch at offset {}",
            i
        );
    }

    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsq_pointer_array_copy() {
    // Copy array of pointers (8 bytes each)
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8 pointers
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup pointer array
    let pointers: [u64; 8] = [
        0x0000000000001000,
        0x0000000000002000,
        0x0000000000003000,
        0x0000000000004000,
        0x0000000000005000,
        0x0000000000006000,
        0x0000000000007000,
        0x0000000000008000,
    ];

    for (i, &ptr) in pointers.iter().enumerate() {
        write_mem_at_u64(&mem, 0x3000 + i as u64 * 8, ptr);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all pointers copied
    for (i, &ptr) in pointers.iter().enumerate() {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i as u64 * 8), ptr);
    }

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3040);
    assert_eq!(regs.rdi, 0x4040);
}

#[test]
fn test_rep_movsb_alternating_pattern() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Alternating 0x55, 0xAA pattern
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x3000 + i, if i % 2 == 0 { 0x55 } else { 0xAA });
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..16 {
        assert_eq!(
            read_mem_at_u8(&mem, 0x4000 + i),
            if i % 2 == 0 { 0x55 } else { 0xAA }
        );
    }
}

#[test]
fn test_rep_movsd_sequential_numbers() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..20 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, (i * 100) as u32);
    }

    run_until_hlt(&mut vcpu).unwrap();

    for i in 0..20 {
        assert_eq!(read_mem_at_u32(&mem, 0x4000 + i * 4), (i * 100) as u32);
    }
}

#[test]
fn test_rep_movsq_max_values() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, 0x3008, 0x0000000000000000);
    write_mem_at_u64(&mem, 0x3010, 0x8000000000000000);
    write_mem_at_u64(&mem, 0x3018, 0x7FFFFFFFFFFFFFFF);

    run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(read_mem_at_u64(&mem, 0x4008), 0x0000000000000000);
    assert_eq!(read_mem_at_u64(&mem, 0x4010), 0x8000000000000000);
    assert_eq!(read_mem_at_u64(&mem, 0x4018), 0x7FFFFFFFFFFFFFFF);
}

#[test]
fn test_rep_movsb_size_1() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u8(&mem, 0x3000, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0x42);
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsw_size_2() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xfc, // CLD
        0xf3, 0x66, 0xa5, // REP MOVSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u16(&mem, 0x3000, 0xABCD);
    write_mem_at_u16(&mem, 0x3002, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u16(&mem, 0x4000), 0xABCD);
    assert_eq!(read_mem_at_u16(&mem, 0x4002), 0x1234);
    assert_eq!(regs.rdi, 0x4004);
}

#[test]
fn test_rep_movsd_size_4() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u32(&mem, 0x3000, 0xCAFEBABE);
    write_mem_at_u32(&mem, 0x3004, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u32(&mem, 0x4000), 0xCAFEBABE);
    assert_eq!(read_mem_at_u32(&mem, 0x4004), 0xDEADBEEF);
    assert_eq!(regs.rdi, 0x4008);
}

#[test]
fn test_rep_movsq_size_8() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x3000, 0x0102030405060708);
    write_mem_at_u64(&mem, 0x3008, 0x1112131415161718);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u64(&mem, 0x4000), 0x0102030405060708);
    assert_eq!(read_mem_at_u64(&mem, 0x4008), 0x1112131415161718);
    assert_eq!(regs.rdi, 0x4010);
}

#[test]
fn test_rep_movsb_rcx_255() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0xFF, 0x00, 0x00, 0x00, // MOV RCX, 255
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x30FF);
    assert_eq!(regs.rdi, 0x40FF);
}

#[test]
fn test_rep_movsd_odd_alignment() {
    let code = [
        0x48, 0xc7, 0xc6, 0x01, 0x30, 0x00, 0x00, // MOV RSI, 0x3001 (odd)
        0x48, 0xc7, 0xc7, 0x03, 0x40, 0x00, 0x00, // MOV RDI, 0x4003 (odd)
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u32(&mem, 0x3001, 0x11223344);
    write_mem_at_u32(&mem, 0x3005, 0x55667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_at_u32(&mem, 0x4003), 0x11223344);
    assert_eq!(read_mem_at_u32(&mem, 0x4007), 0x55667788);
    assert_eq!(regs.rdi, 0x400B);
}

#[test]
fn test_rep_movsb_page_crossing() {
    // Test copying across page boundary - source spans 0x4FFE-0x5001
    // Use addresses that don't overlap with code at 0x1000
    let code = [
        0x48, 0xc7, 0xc6, 0xFE, 0x4F, 0x00, 0x00, // MOV RSI, 0x4FFE
        0x48, 0xc7, 0xc7, 0x00, 0x60, 0x00, 0x00, // MOV RDI, 0x6000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (crosses page)
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..4 {
        write_mem_at_u8(&mem, 0x4FFE + i, 0x10 + i as u8);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    for i in 0..4 {
        assert_eq!(read_mem_at_u8(&mem, 0x6000 + i), 0x10 + i as u8);
    }
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsq_backward_odd_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x28, 0x30, 0x00, 0x00, // MOV RSI, 0x3028
        0x48, 0xc7, 0xc7, 0x28, 0x40, 0x00, 0x00, // MOV RDI, 0x4028
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5 (odd)
        0xfd, // STD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..5 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, i as u64);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    for i in 0..5 {
        assert_eq!(read_mem_at_u64(&mem, 0x4000 + i * 8), i as u64);
    }
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rep_movsb_fills_gaps() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Sparse pattern
    for i in (0..16).step_by(2) {
        write_mem_at_u8(&mem, 0x3000 + i, 0xAA);
    }
    run_until_hlt(&mut vcpu).unwrap();
    for i in (0..16).step_by(2) {
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i), 0xAA);
        assert_eq!(read_mem_at_u8(&mem, 0x4000 + i + 1), 0);
    }
}

// ============================================================================
// Bulk page-wise fast path regression tests
//
// These exercise the O(pages) fast path for forward REP MOVS, ensuring the
// page-spanning chunking produces byte-exact results and exact end registers.
// ============================================================================

#[test]
fn test_rep_movsd_dest_crosses_page_boundary() {
    // MOVSD where the destination starts near a page boundary so the run spans
    // two destination pages and one element straddles the boundary. The fast
    // path must split into page-bounded chunks and hand the straddling element
    // to the slow path, copying every dword correctly.
    // Dest: 0x4FF8 .. 0x501F (10 dwords = 40 bytes), crossing 0x5000.
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0xF8, 0x4F, 0x00, 0x00, // MOV RDI, 0x4FF8
        0x48, 0xc7, 0xc1, 0x0A, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf3, 0xa5, // REP MOVSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10u64 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xC0DE0000 | i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..10u64 {
        assert_eq!(
            read_mem_at_u32(&mem, 0x4FF8 + i * 4),
            0xC0DE0000 | i as u32,
            "dword {} mismatch across page boundary",
            i
        );
    }
    // 10 dwords = 40 bytes consumed.
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rsi, 0x3000 + 40, "RSI end value");
    assert_eq!(regs.rdi, 0x4FF8 + 40, "RDI end value");
}

#[test]
fn test_rep_movsq_source_crosses_page_boundary() {
    // MOVSQ where the SOURCE spans a page boundary and an element straddles it.
    // Source: 0x5FF0 .. 0x6027 (7 qwords = 56 bytes), crossing 0x6000.
    let code = [
        0x48, 0xc7, 0xc6, 0xF0, 0x5F, 0x00, 0x00, // MOV RSI, 0x5FF0
        0x48, 0xc7, 0xc7, 0x00, 0x80, 0x00, 0x00, // MOV RDI, 0x8000
        0x48, 0xc7, 0xc1, 0x07, 0x00, 0x00, 0x00, // MOV RCX, 7
        0xfc, // CLD
        0xf3, 0x48, 0xa5, // REP MOVSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..7u64 {
        write_mem_at_u64(&mem, 0x5FF0 + i * 8, 0xABCD000000000000 | i);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..7u64 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x8000 + i * 8),
            0xABCD000000000000 | i,
            "qword {} mismatch across page boundary",
            i
        );
    }
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rsi, 0x5FF0 + 56, "RSI end value");
    assert_eq!(regs.rdi, 0x8000 + 56, "RDI end value");
}

#[test]
fn test_rep_movsb_multi_page_exact_end_regs() {
    // A large byte copy spanning multiple pages, verifying exact end registers.
    // 0x2050 .. 0x2050 + 0x2400 spans 3 source pages.
    let code = [
        0x48, 0xc7, 0xc6, 0x50, 0x20, 0x00, 0x00, // MOV RSI, 0x2050
        0x48, 0xc7, 0xc7, 0x60, 0x90, 0x00, 0x00, // MOV RDI, 0x9060
        0x48, 0xc7, 0xc1, 0x00, 0x24, 0x00, 0x00, // MOV RCX, 0x2400
        0xfc, // CLD
        0xf3, 0xa4, // REP MOVSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..0x2400u64 {
        write_mem_at_u8(&mem, 0x2050 + i, (i.wrapping_mul(31) & 0xFF) as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..0x2400u64 {
        assert_eq!(
            read_mem_at_u8(&mem, 0x9060 + i),
            (i.wrapping_mul(31) & 0xFF) as u8,
            "byte {} mismatch",
            i
        );
    }
    assert_eq!(regs.rcx, 0, "RCX must be exactly 0");
    assert_eq!(regs.rsi, 0x2050 + 0x2400, "RSI end value");
    assert_eq!(regs.rdi, 0x9060 + 0x2400, "RDI end value");
}
