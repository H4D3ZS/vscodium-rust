use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REPE/REPZ CMPS - Repeat Compare String While Equal/Zero
//
// Opcodes:
//   F3 A6        - REPE CMPSB (repeat compare byte while ZF=1)
//   F3 66 A7     - REPE CMPSW (repeat compare word while ZF=1)
//   F3 A7        - REPE CMPSD (repeat compare doubleword while ZF=1)
//   F3 REX.W A7  - REPE CMPSQ (repeat compare quadword while ZF=1)
//
// Operation:
// WHILE RCX != 0 DO
//   temp := [RSI] - [RDI]
//   Set flags based on temp
//   IF DF = 0 THEN RSI += size; RDI += size
//   ELSE RSI -= size; RDI -= size
//   RCX -= 1
//   IF ZF = 0 THEN exit loop (found mismatch)
// END
//
// Terminates when RCX=0 OR when bytes don't match (ZF=0)
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REPE CMPSB - Repeat Compare Byte While Equal
// ============================================================================

#[test]
fn test_repe_cmpsb_all_equal() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup identical data
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x42);
        write_mem_at_u8(&mem, 0x4000 + i, 0x42);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "All bytes equal, should scan all");
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(
        zf_set(regs.rflags),
        "ZF should be set (last comparison equal)"
    );
}

#[test]
fn test_repe_cmpsb_mismatch_at_start() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte differs
    write_mem_at_u8(&mem, 0x3000, 0x42);
    write_mem_at_u8(&mem, 0x4000, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Should stop after 1 iteration");
    assert_eq!(regs.rsi, 0x3001, "Incremented once");
    assert_eq!(regs.rdi, 0x4001, "Incremented once");
    assert!(!zf_set(regs.rflags), "ZF should be clear (mismatch)");
}

#[test]
fn test_repe_cmpsb_mismatch_in_middle() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Match first 4, differ at position 4
    for i in 0..4 {
        write_mem_at_u8(&mem, 0x3000 + i, 0xAA);
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }
    write_mem_at_u8(&mem, 0x3004, 0x42);
    write_mem_at_u8(&mem, 0x4004, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 3, "Should stop after 5 iterations");
    assert_eq!(regs.rsi, 0x3005);
    assert_eq!(regs.rdi, 0x4005);
    assert!(!zf_set(regs.rflags), "ZF clear");
}

#[test]
fn test_repe_cmpsb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_cmpsb_single_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u8(&mem, 0x3000, 0x42);
    write_mem_at_u8(&mem, 0x4000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_single_mismatch() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u8(&mem, 0x3000, 0x42);
    write_mem_at_u8(&mem, 0x4000, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x07, 0x30, 0x00, 0x00, // MOV RSI, 0x3007
        0x48, 0xc7, 0xc7, 0x07, 0x40, 0x00, 0x00, // MOV RDI, 0x4007
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfd, // STD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup identical backward data
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x55);
        write_mem_at_u8(&mem, 0x4000 + i, 0x55);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFF);
    assert_eq!(regs.rdi, 0x3FFF);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_string_compare() {
    // Compare "HELLO" vs "HELLO"
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let s1 = b"HELLO";
    let s2 = b"HELLO";
    for i in 0..5 {
        write_mem_at_u8(&mem, 0x3000 + i, s1[i as usize]);
        write_mem_at_u8(&mem, 0x4000 + i, s2[i as usize]);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "Strings match");
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_string_prefix_match() {
    // Compare "HELLO" vs "HELP!"
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let s1 = b"HELLO";
    let s2 = b"HELP!";
    for i in 0..5 {
        write_mem_at_u8(&mem, 0x3000 + i, s1[i as usize]);
        write_mem_at_u8(&mem, 0x4000 + i, s2[i as usize]);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // "HEL" matches (3 iterations), then mismatch 'L' vs 'P' (4th iteration)
    // Intel REPE: decrement RCX AFTER each comparison (including mismatch)
    // So RCX goes: 5 → 4 → 3 → 2 → 1 (exit after 4th comparison with ZF=0)
    assert_eq!(regs.rcx, 1, "Stopped after comparing position 3");
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_sets_flags() {
    // Verify comparison sets SF, CF, OF correctly
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // 0x30 - 0x50 = -0x20 (sets SF, CF)
    write_mem_at_u8(&mem, 0x3000, 0x30);
    write_mem_at_u8(&mem, 0x4000, 0x50);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!zf_set(regs.rflags), "ZF clear (not equal)");
    assert!(sf_set(regs.rflags), "SF set (negative result)");
    assert!(cf_set(regs.rflags), "CF set (borrow)");
}

#[test]
fn test_repe_cmpsb_large_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..256 {
        let val = (i & 0xFF) as u8;
        write_mem_at_u8(&mem, 0x3000 + i, val);
        write_mem_at_u8(&mem, 0x4000 + i, val);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_mismatch_at_end() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..15 {
        write_mem_at_u8(&mem, 0x3000 + i, 0xAA);
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }
    // Last byte differs
    write_mem_at_u8(&mem, 0x300F, 0x42);
    write_mem_at_u8(&mem, 0x400F, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "Processed all, stopped on last");
    assert!(!zf_set(regs.rflags));
}

// ============================================================================
// REPE CMPSW - Repeat Compare Word While Equal
// ============================================================================

#[test]
fn test_repe_cmpsw_all_equal() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x1234);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x1234);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_mismatch() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3000, 0x1111);
    write_mem_at_u16(&mem, 0x4000, 0x1111);
    write_mem_at_u16(&mem, 0x3002, 0x2222);
    write_mem_at_u16(&mem, 0x4002, 0x3333); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2, "Stopped after 2 words");
    assert_eq!(regs.rsi, 0x3004);
    assert_eq!(regs.rdi, 0x4004);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_cmpsw_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x06, 0x30, 0x00, 0x00, // MOV RSI, 0x3006
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0xABCD);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0xABCD);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFE);
    assert_eq!(regs.rdi, 0x3FFE);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..128 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, i as u16);
        write_mem_at_u16(&mem, 0x4000 + i * 2, i as u16);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// REPE CMPSD - Repeat Compare Doubleword While Equal
// ============================================================================

#[test]
fn test_repe_cmpsd_all_equal() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0x12345678);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x12345678);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3010);
    assert_eq!(regs.rdi, 0x4010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsd_mismatch() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0x11111111);
    write_mem_at_u32(&mem, 0x4000, 0x11111111);
    write_mem_at_u32(&mem, 0x3004, 0x22222222);
    write_mem_at_u32(&mem, 0x4004, 0x33333333); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_cmpsd_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x0C, 0x30, 0x00, 0x00, // MOV RSI, 0x300C
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xDEADBEEF);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xDEADBEEF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFC);
    assert_eq!(regs.rdi, 0x3FFC);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsd_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xAAAA0000 | i as u32);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xAAAA0000 | i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// REPE CMPSQ - Repeat Compare Quadword While Equal
// ============================================================================

#[test]
fn test_repe_cmpsq_all_equal() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0x1234567890ABCDEF);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x1234567890ABCDEF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3020);
    assert_eq!(regs.rdi, 0x4020);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsq_mismatch() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x4000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x3008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x4008, 0x3333333333333333); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rsi, 0x3010);
    assert_eq!(regs.rdi, 0x4010);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_cmpsq_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x18, 0x30, 0x00, 0x00, // MOV RSI, 0x3018
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0xCAFEBABEDEADBEEF);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xCAFEBABEDEADBEEF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FF8);
    assert_eq!(regs.rdi, 0x3FF8);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsq_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0xBBBB000000000000 | i as u64);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xBBBB000000000000 | i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3100);
    assert_eq!(regs.rdi, 0x4100);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// Edge cases and mixed tests
// ============================================================================

#[test]
fn test_repe_cmps_stops_on_first_mismatch() {
    // Verify early termination
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte different
    write_mem_at_u8(&mem, 0x3000, 0xFF);
    write_mem_at_u8(&mem, 0x4000, 0x00);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 99, "Should stop after 1 iteration");
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
}

#[test]
fn test_repe_cmpsb_strcmp_usage() {
    // Typical strcmp-like usage
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RCX, max
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let s1 = b"TEST";
    let s2 = b"TEXT";
    for i in 0..4 {
        write_mem_at_u8(&mem, 0x3000 + i, s1[i as usize]);
        write_mem_at_u8(&mem, 0x4000 + i, s2[i as usize]);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // "TE" matches, stops at 'S' vs 'X'
    assert_eq!(regs.rsi, 0x3003);
    assert_eq!(regs.rdi, 0x4003);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmps_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x3000 + i, 0xAA);
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rbx, 0x99);
}

#[test]
fn test_repe_cmps_memcmp_pattern() {
    // Like memcmp: compare fixed-length buffers
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x50, 0x00, 0x00, // MOV RSI, 0x5000
        0x48, 0xc7, 0xc7, 0x00, 0x60, 0x00, 0x00, // MOV RDI, 0x6000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x5000 + i * 4, i as u32);
        write_mem_at_u32(&mem, 0x6000 + i * 4, i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "All equal");
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsq_pointer_comparison() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

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
        write_mem_at_u64(&mem, 0x4000 + i as u64 * 8, ptr);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_exact_match_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..5 {
        write_mem_at_u8(&mem, 0x3000 + i, i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, i as u8);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_partial_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..3 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x1111);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x1111);
    }
    write_mem_at_u16(&mem, 0x3006, 0x2222);
    write_mem_at_u16(&mem, 0x4006, 0x3333);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 1);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsd_all_zeros() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..8 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsq_single_element() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x3000, 0x123456789ABCDEF0);
    write_mem_at_u64(&mem, 0x4000, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsb_max_byte_values() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u8(&mem, 0x3000, 0x00);
    write_mem_at_u8(&mem, 0x4000, 0x00);
    write_mem_at_u8(&mem, 0x3001, 0xFF);
    write_mem_at_u8(&mem, 0x4001, 0xFF);
    write_mem_at_u8(&mem, 0x3002, 0x7F);
    write_mem_at_u8(&mem, 0x4002, 0x7F);
    write_mem_at_u8(&mem, 0x3003, 0x80);
    write_mem_at_u8(&mem, 0x4003, 0x80);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_incremental() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, i as u16);
        write_mem_at_u16(&mem, 0x4000 + i * 2, i as u16);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsd_repeating_pattern() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xABABABAB);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xABABABAB);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsq_alternating() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..8 {
        let val = if i % 2 == 0 {
            0x1111111111111111
        } else {
            0x2222222222222222
        };
        write_mem_at_u64(&mem, 0x3000 + i * 8, val);
        write_mem_at_u64(&mem, 0x4000 + i * 8, val);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsb_boundary_values() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u8(&mem, 0x3000, 0);
    write_mem_at_u8(&mem, 0x4000, 0);
    write_mem_at_u8(&mem, 0x3001, 0x80);
    write_mem_at_u8(&mem, 0x4001, 0x80);
    write_mem_at_u8(&mem, 0x3002, 0xFF);
    write_mem_at_u8(&mem, 0x4002, 0xFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_cmpsw_count_255() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, 0x5000
        0x48, 0xc7, 0xc1, 0xFF, 0x00, 0x00, 0x00, // MOV RCX, 255
        0xfc, // CLD
        0xf3, 0x66, 0xa7, // REPE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..255 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0xBEEF);
        write_mem_at_u16(&mem, 0x5000 + i * 2, 0xBEEF);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsd_powers_of_two() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf3, 0xa7, // REPE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..8 {
        let val = 1u32 << i;
        write_mem_at_u32(&mem, 0x3000 + i * 4, val);
        write_mem_at_u32(&mem, 0x4000 + i * 4, val);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsq_fib_sequence() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0xfc, // CLD
        0xf3, 0x48, 0xa7, // REPE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let fib = [1u64, 1, 2, 3, 5, 8];
    for (i, &val) in fib.iter().enumerate() {
        write_mem_at_u64(&mem, 0x3000 + i as u64 * 8, val);
        write_mem_at_u64(&mem, 0x4000 + i as u64 * 8, val);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_cmpsb_ascii_text() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0A, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf3, 0xa6, // REPE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let text = b"0123456789";
    for (i, &byte) in text.iter().enumerate() {
        write_mem_at_u8(&mem, 0x3000 + i as u64, byte);
        write_mem_at_u8(&mem, 0x4000 + i as u64, byte);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}
