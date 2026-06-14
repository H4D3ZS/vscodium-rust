use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REPNE/REPNZ CMPS - Repeat Compare String While Not Equal/Not Zero
//
// Opcodes:
//   F2 A6        - REPNE CMPSB (repeat compare byte while ZF=0)
//   F2 66 A7     - REPNE CMPSW (repeat compare word while ZF=0)
//   F2 A7        - REPNE CMPSD (repeat compare doubleword while ZF=0)
//   F2 REX.W A7  - REPNE CMPSQ (repeat compare quadword while ZF=0)
//
// Operation:
// WHILE RCX != 0 DO
//   temp := [RSI] - [RDI]
//   Set flags based on temp
//   IF DF = 0 THEN RSI += size; RDI += size
//   ELSE RSI -= size; RDI -= size
//   RCX -= 1
//   IF ZF = 1 THEN exit loop (found match)
// END
//
// Terminates when RCX=0 OR when bytes match (ZF=1)
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REPNE CMPSB - Repeat Compare Byte While Not Equal
// ============================================================================

#[test]
fn test_repne_cmpsb_all_different() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // All bytes different
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x10 + i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 0x20 + i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "All bytes different, scans all");
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags), "ZF clear (last comparison unequal)");
}

#[test]
fn test_repne_cmpsb_match_at_start() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte matches
    write_mem_at_u8(&mem, 0x3000, 0x42);
    write_mem_at_u8(&mem, 0x4000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Stopped after 1 iteration");
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
    assert!(zf_set(regs.rflags), "ZF set (match found)");
}

#[test]
fn test_repne_cmpsb_match_in_middle() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Different for first 4, match at position 4
    for i in 0..4 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x10 + i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 0x20 + i as u8);
    }
    write_mem_at_u8(&mem, 0x3004, 0x99);
    write_mem_at_u8(&mem, 0x4004, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 3, "Stopped after 5 iterations");
    assert_eq!(regs.rsi, 0x3005);
    assert_eq!(regs.rdi, 0x4005);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_cmpsb_single_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
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
fn test_repne_cmpsb_single_mismatch() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
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
fn test_repne_cmpsb_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x07, 0x30, 0x00, 0x00, // MOV RSI, 0x3007
        0x48, 0xc7, 0xc7, 0x07, 0x40, 0x00, 0x00, // MOV RDI, 0x4007
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfd, // STD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // All different
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x10 + i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 0x20 + i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFF);
    assert_eq!(regs.rdi, 0x3FFF);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_scan_for_char() {
    // Like strchr: scan for matching character
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000 (target)
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Setup: ABCDEFGHIJKLMNOP vs 'K' (position 10)
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x3000 + i, b'A' + i as u8);
    }
    // Target is 'K'
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, b'K');
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should find 'K' at position 10 (11th byte)
    assert_eq!(regs.rcx, 5, "Stopped after finding 'K'");
    assert_eq!(regs.rsi, 0x300B, "Points past match");
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_no_match_found() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // All As vs all Bs (no match)
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x3000 + i, b'A');
        write_mem_at_u8(&mem, 0x4000 + i, b'B');
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "No match, scanned all");
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_sets_flags() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // 0x80 - 0x40 = 0x40 (positive, no CF)
    write_mem_at_u8(&mem, 0x3000, 0x80);
    write_mem_at_u8(&mem, 0x4000, 0x40);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!zf_set(regs.rflags));
    assert!(!cf_set(regs.rflags), "No borrow");
}

#[test]
fn test_repne_cmpsb_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pattern: 0, 1, 2, ..., 254, 255 vs all 128
    for i in 0..256 {
        write_mem_at_u8(&mem, 0x3000 + i, i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 128);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Finds match at position 128
    assert_eq!(regs.rcx, 127, "Stopped at byte 128");
    assert_eq!(regs.rsi, 0x3081);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_match_at_end() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..15 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x10 + i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 0x20 + i as u8);
    }
    // Last byte matches
    write_mem_at_u8(&mem, 0x300F, 0x99);
    write_mem_at_u8(&mem, 0x400F, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// REPNE CMPSW - Repeat Compare Word While Not Equal
// ============================================================================

#[test]
fn test_repne_cmpsw_all_different() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x1000 + i as u16);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x2000 + i as u16);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsw_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3000, 0x1111);
    write_mem_at_u16(&mem, 0x4000, 0x2222);
    write_mem_at_u16(&mem, 0x3002, 0x3333);
    write_mem_at_u16(&mem, 0x4002, 0x3333); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rsi, 0x3004);
    assert_eq!(regs.rdi, 0x4004);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_cmpsw_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x06, 0x30, 0x00, 0x00, // MOV RSI, 0x3006
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x1000 + i as u16);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x2000 + i as u16);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFE);
    assert_eq!(regs.rdi, 0x3FFE);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsw_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..128 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, i as u16);
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x8000);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "No match");
    assert!(!zf_set(regs.rflags));
}

// ============================================================================
// REPNE CMPSD - Repeat Compare Doubleword While Not Equal
// ============================================================================

#[test]
fn test_repne_cmpsd_all_different() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0x10000000 + i as u32);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x20000000 + i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3010);
    assert_eq!(regs.rdi, 0x4010);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsd_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3000, 0x11111111);
    write_mem_at_u32(&mem, 0x4000, 0x22222222);
    write_mem_at_u32(&mem, 0x3004, 0x33333333);
    write_mem_at_u32(&mem, 0x4004, 0x33333333); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rsi, 0x3008);
    assert_eq!(regs.rdi, 0x4008);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_cmpsd_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x0C, 0x30, 0x00, 0x00, // MOV RSI, 0x300C
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0x10000000 + i as u32);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x20000000 + i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FFC);
    assert_eq!(regs.rdi, 0x3FFC);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsd_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, i as u32);
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x80000000);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert!(!zf_set(regs.rflags));
}

// ============================================================================
// REPNE CMPSQ - Repeat Compare Quadword While Not Equal
// ============================================================================

#[test]
fn test_repne_cmpsq_all_different() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0x1000000000000000 + i as u64);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x2000000000000000 + i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3020);
    assert_eq!(regs.rdi, 0x4020);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsq_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x3000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x4000, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x3008, 0x3333333333333333);
    write_mem_at_u64(&mem, 0x4008, 0x3333333333333333); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rsi, 0x3010);
    assert_eq!(regs.rdi, 0x4010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x3000);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_cmpsq_backward() {
    let code = [
        0x48, 0xc7, 0xc6, 0x18, 0x30, 0x00, 0x00, // MOV RSI, 0x3018
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xfd, // STD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0x1000000000000000 + i as u64);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x2000000000000000 + i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, 0x2FF8);
    assert_eq!(regs.rdi, 0x3FF8);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsq_large() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, i as u64);
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x8000000000000000);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert!(!zf_set(regs.rflags));
}

// ============================================================================
// Edge cases and mixed tests
// ============================================================================

#[test]
fn test_repne_cmps_stops_on_first_match() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte matches
    write_mem_at_u8(&mem, 0x3000, 0x42);
    write_mem_at_u8(&mem, 0x4000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 99);
    assert_eq!(regs.rsi, 0x3001);
    assert_eq!(regs.rdi, 0x4001);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_search_pattern() {
    // Search for first occurrence of a pattern
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000 (haystack)
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000 (needle)
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Haystack: mostly As, with 'X' at position 15
    for i in 0..32 {
        write_mem_at_u8(&mem, 0x3000 + i, if i == 15 { b'X' } else { b'A' });
    }
    // Needle: all Xs
    for i in 0..32 {
        write_mem_at_u8(&mem, 0x4000 + i, b'X');
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 16, "Found at position 15");
    assert_eq!(regs.rsi, 0x3010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmps_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x3000 + i, 0x10 + i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, 0x20 + i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rbx, 0x99);
}

#[test]
fn test_repne_cmpsq_find_sentinel() {
    // Find a sentinel value in array
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000 (sentinel)
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Array with 0 sentinel at position 8
    for i in 0..16 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, if i == 8 { 0 } else { i as u64 + 1 });
    }
    // Looking for 0
    for i in 0..16 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Found at position 8");
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_cmpsb_count_100() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..100 {
        write_mem_at_u8(&mem, 0x3000 + i, (i % 256) as u8);
        write_mem_at_u8(&mem, 0x4000 + i, (i % 256) as u8 + 1);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "RCX should reach zero");
}

#[test]
fn test_repne_cmpsw_pattern_search() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x1234);
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 5 { 0x1234 } else { 0x5678 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 4, "Should stop at matching word");
}

#[test]
fn test_repne_cmpsd_boundary_check() {
    let code = [
        0x48, 0xc7, 0xc6, 0xfc, 0x2f, 0x00, 0x00, // MOV RSI, 0x2FFC
        0x48, 0xc7, 0xc7, 0xfc, 0x3f, 0x00, 0x00, // MOV RDI, 0x3FFC
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x2FFC, 0x11111111);
    write_mem_at_u32(&mem, 0x3000, 0x22222222);
    write_mem_at_u32(&mem, 0x3004, 0x33333333);
    write_mem_at_u32(&mem, 0x3FFC, 0x44444444);
    write_mem_at_u32(&mem, 0x4000, 0x22222222);
    write_mem_at_u32(&mem, 0x4004, 0x55555555);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rcx < 3, "Should find match");
}

#[test]
fn test_repne_cmpsq_incremental_values() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, i as u64);
        write_mem_at_u64(
            &mem,
            0x4000 + i * 8,
            if i == 3 { 3 } else { i as u64 + 100 },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 4, "Should stop at matching qword");
}

#[test]
fn test_repne_cmpsb_alternating_pattern() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..16 {
        write_mem_at_u8(&mem, 0x3000 + i, 0xAA);
        write_mem_at_u8(&mem, 0x4000 + i, if i % 2 == 0 { 0x55 } else { 0xAA });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rcx < 16, "Should find match at odd position");
}

#[test]
fn test_repne_cmpsw_single_word() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x3000, 0x1234);
    write_mem_at_u16(&mem, 0x4000, 0x5678);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "Should complete comparison");
}

#[test]
fn test_repne_cmpsd_powers_of_two() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let powers = [1u32, 2, 4, 8, 16, 32, 64, 128];
    for i in 0..8u64 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, powers[i as usize]);
        write_mem_at_u32(&mem, 0x4000 + i * 4, if i == 4 { 16 } else { 0 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 3, "Should find matching power of 2");
}

#[test]
fn test_repne_cmpsq_null_terminated_array() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, i as u64 + 1000);
        write_mem_at_u64(&mem, 0x4000 + i * 8, if i == 7 { 1007 } else { 0 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 2, "Should find match at position 7");
}

#[test]
fn test_repne_cmpsb_sequential_bytes() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..20 {
        write_mem_at_u8(&mem, 0x3000 + i, i as u8);
        write_mem_at_u8(&mem, 0x4000 + i, if i == 10 { 10 } else { 0xFF });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 9, "Should find match at byte 10");
}

#[test]
fn test_repne_cmpsw_high_bit_pattern() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0c, 0x00, 0x00, 0x00, // MOV RCX, 12
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..12 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0x8000 | (i as u16));
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 6 { 0x8006 } else { 0x0000 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 5, "Should find matching high-bit word");
}

#[test]
fn test_repne_cmpsd_backward_scan() {
    let code = [
        0x48, 0xc7, 0xc6, 0x18, 0x30, 0x00, 0x00, // MOV RSI, 0x3018
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x07, 0x00, 0x00, 0x00, // MOV RCX, 7
        0xfd, // STD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..7 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xAAAAAAAA);
        write_mem_at_u32(
            &mem,
            0x4000 + i * 4,
            if i == 2 { 0xAAAAAAAA } else { 0xBBBBBBBB },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rcx < 7, "Should find match during backward scan");
}

#[test]
fn test_repne_cmpsq_sparse_matches() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0f, 0x00, 0x00, 0x00, // MOV RCX, 15
        0xfc, // CLD
        0xf2, 0x48, 0xa7, // REPNE CMPSQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..15 {
        write_mem_at_u64(&mem, 0x3000 + i * 8, 0x123456789ABCDEF0 + i as u64);
        write_mem_at_u64(
            &mem,
            0x4000 + i * 8,
            if i == 9 { 0x123456789ABCDEF9 } else { 0 },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 5, "Should find first match");
}

#[test]
fn test_repne_cmpsb_hex_digits() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xa6, // REPNE CMPSB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let hex = b"0123456789ABCDEF";
    for i in 0..16u64 {
        write_mem_at_u8(&mem, 0x3000 + i, hex[i as usize]);
        write_mem_at_u8(&mem, 0x4000 + i, if i == 12 { b'C' } else { b'Z' });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 3, "Should find 'C' at position 12");
}

#[test]
fn test_repne_cmpsw_unicode_bom() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x66, 0xa7, // REPNE CMPSW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u16(&mem, 0x3000 + i * 2, 0xFEFF);
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 0 { 0xFEFF } else { 0x0000 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 9, "Should find BOM at start");
}

#[test]
fn test_repne_cmpsd_signed_negative() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x30, 0x00, 0x00, // MOV RSI, 0x3000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xa7, // REPNE CMPSD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..8 {
        write_mem_at_u32(&mem, 0x3000 + i * 4, 0xFFFFFFFF);
        write_mem_at_u32(
            &mem,
            0x4000 + i * 4,
            if i == 5 { 0xFFFFFFFF } else { 0x7FFFFFFF },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 2, "Should find matching negative value");
}
