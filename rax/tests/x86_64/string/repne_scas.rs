use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REPNE/REPNZ SCAS - Repeat Scan String While Not Equal/Not Zero
//
// Opcodes:
//   F2 AE        - REPNE SCASB (repeat scan byte while ZF=0)
//   F2 66 AF     - REPNE SCASW (repeat scan word while ZF=0)
//   F2 AF        - REPNE SCASD (repeat scan doubleword while ZF=0)
//   F2 REX.W AF  - REPNE SCASQ (repeat scan quadword while ZF=0)
//
// Operation:
// WHILE RCX != 0 DO
//   temp := AL/AX/EAX/RAX - [RDI]
//   Set flags based on temp
//   IF DF = 0 THEN RDI += size
//   ELSE RDI -= size
//   RCX -= 1
//   IF ZF = 1 THEN exit loop (found matching value)
// END
//
// Terminates when RCX=0 OR when [RDI] == AL/AX/EAX/RAX (ZF=1)
// Scans for first value that DOES match accumulator
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REPNE SCASB - Repeat Scan Byte While Not Equal
// ============================================================================

#[test]
fn test_repne_scasb_all_different() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // No 0x42
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x99);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "No match, scan all");
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags), "ZF clear (no match)");
}

#[test]
fn test_repne_scasb_match_at_start() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte matches
    write_mem_at_u8(&mem, 0x4000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Stopped after 1");
    assert_eq!(regs.rdi, 0x4001);
    assert!(zf_set(regs.rflags), "ZF set (match)");
}

#[test]
fn test_repne_scasb_find_char() {
    // Classic strchr: find character in string
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, b'X', // MOV AL, 'X'
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // "ABCDEFGHIJKLMNOP" with 'X' at position 10
    for i in 0..10 {
        write_mem_at_u8(&mem, 0x4000 + i, b'A' + i as u8);
    }
    write_mem_at_u8(&mem, 0x400A, b'X'); // Position 10

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 5, "Found at position 10");
    assert_eq!(regs.rdi, 0x400B);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_scasb_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x07, 0x40, 0x00, 0x00, // MOV RDI, 0x4007
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x55, // MOV AL, 0x55
        0xfd, // STD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // All 0xAA except no 0x55
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFF);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_strlen() {
    // Classic strlen: scan for null terminator
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RCX, max
        0xb0, 0x00, // MOV AL, 0 (null)
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // "HELLO\0"
    let s = b"HELLO\0";
    for (i, &byte) in s.iter().enumerate() {
        write_mem_at_u8(&mem, 0x4000 + i as u64, byte);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Found null at position 5
    assert_eq!(regs.rdi, 0x4006, "Points past null");
    assert!(zf_set(regs.rflags));
    // String length = RDI - original - 1 = 0x4006 - 0x4000 - 1 = 5
}

#[test]
fn test_repne_scasb_no_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, b'Z', // MOV AL, 'Z'
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // All As (no Z)
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, b'A');
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "No match");
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_sets_flags() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xb0, 0x30, // MOV AL, 0x30
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // 0x30 - 0x50 = -0x20 (SF, CF)
    write_mem_at_u8(&mem, 0x4000, 0x50);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!zf_set(regs.rflags));
    assert!(sf_set(regs.rflags));
    assert!(cf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xb0, 0x77, // MOV AL, 0x77
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pattern: 0-255, so 0x77 is at position 119
    for i in 0..256 {
        write_mem_at_u8(&mem, 0x4000 + i, i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Found at 0x77 = 119
    assert_eq!(regs.rcx, 136, "Stopped at 0x77");
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_match_at_end() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0x99, // MOV AL, 0x99
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..15 {
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }
    // Last byte matches
    write_mem_at_u8(&mem, 0x400F, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_memchr_pattern() {
    // Like memchr: find byte in buffer
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x50, 0x00, 0x00, // MOV RDI, 0x5000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xb0, 0x00, // MOV AL, 0 (null)
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Non-zero data with null at position 32
    for i in 0..64 {
        write_mem_at_u8(&mem, 0x5000 + i, if i == 32 { 0 } else { 0xFF });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 31);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// REPNE SCASW - Repeat Scan Word While Not Equal
// ============================================================================

#[test]
fn test_repne_scasw_all_different() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x9999);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_scasw_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x4000, 0x9999);
    write_mem_at_u16(&mem, 0x4002, 0x1234); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4004);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x66, 0xb8, 0xFF, 0xFF, // MOV AX, 0xFFFF
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_scasw_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0xCD, 0xAB, // MOV AX, 0xABCD
        0xfd, // STD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x1111);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFE);
}

#[test]
fn test_repne_scasw_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0x66, 0xb8, 0x00, 0x80, // MOV AX, 0x8000
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..128 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, i as u16);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // No 0x8000 in range 0-127
    assert_eq!(regs.rcx, 0);
    assert!(!zf_set(regs.rflags));
}

// ============================================================================
// REPNE SCASD - Repeat Scan Doubleword While Not Equal
// ============================================================================

#[test]
fn test_repne_scasd_all_different() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x99999999);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4010);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_scasd_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x4000, 0x99999999);
    write_mem_at_u32(&mem, 0x4004, 0x12345678); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4008);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_scasd_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EAX, 0xDEADBEEF
        0xfd, // STD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x11111111);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFC);
}

#[test]
fn test_repne_scasd_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xb8, 0x20, 0x00, 0x00, 0x00, // MOV EAX, 32
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, i as u32);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Found 32 at position 32
    assert_eq!(regs.rcx, 31);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// REPNE SCASQ - Repeat Scan Quadword While Not Equal
// ============================================================================

#[test]
fn test_repne_scasq_all_different() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x9999999999999999);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4020);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repne_scasq_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x4000, 0x9999999999999999);
    write_mem_at_u64(&mem, 0x4008, 0x1234567890ABCDEF); // Match

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repne_scasq_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xBE, 0xBA, 0xFE, 0xCA, 0xEF, 0xBE, 0xAD,
        0xDE, // MOV RAX, 0xDEADBEEFCAFEBABE
        0xfd, // STD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x1111111111111111);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FF8);
}

#[test]
fn test_repne_scasq_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0x48, 0xb8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 16
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Found 16 at position 16
    assert_eq!(regs.rcx, 15);
    assert!(zf_set(regs.rflags));
}

// ============================================================================
// Edge cases and mixed tests
// ============================================================================

#[test]
fn test_repne_scas_stops_on_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte matches
    write_mem_at_u8(&mem, 0x4000, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 99);
    assert_eq!(regs.rdi, 0x4001);
}

#[test]
fn test_repne_scas_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0xc7, 0xc6, 0x99, 0x00, 0x00, 0x00, // MOV RSI, 0x99
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x55);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rsi, 0x99);
}

#[test]
fn test_repne_scasb_find_newline() {
    // Find newline in text
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, b'\n', // MOV AL, '\n'
        0xfc,  // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Text with newline at position 15
    for i in 0..15 {
        write_mem_at_u8(&mem, 0x4000 + i, b'A' + (i % 26) as u8);
    }
    write_mem_at_u8(&mem, 0x400F, b'\n');

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 16);
    assert_eq!(regs.rdi, 0x4010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasq_find_null_pointer() {
    // Find null pointer in array
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x31, 0xc0, // XOR RAX, RAX (null)
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Non-null pointers, null at position 8
    for i in 0..16 {
        write_mem_at_u64(
            &mem,
            0x4000 + i * 8,
            if i == 8 { 0 } else { 0x1000 + i as u64 },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_utf8_like_scan() {
    // Scan for specific byte in sequence
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, 0xC0, // MOV AL, 0xC0 (UTF-8 marker)
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // ASCII then UTF-8 marker
    for i in 0..20 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x40 + i as u8);
    }
    write_mem_at_u8(&mem, 0x4014, 0xC0);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 11);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repne_scasb_count_100() {
    let code = [
        0xb0, 0x42, // MOV AL, 0x42
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..100 {
        write_mem_at_u8(&mem, 0x4000 + i, if i == 50 { 0x42 } else { 0xFF });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 49, "Should stop at matching byte");
}

#[test]
fn test_repne_scasw_pattern_search() {
    let code = [
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 7 { 0x1234 } else { 0xABCD });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 2, "Should find pattern at position 7");
}

#[test]
fn test_repne_scasd_boundary_check() {
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0x48, 0xc7, 0xc7, 0xfc, 0x3f, 0x00, 0x00, // MOV RDI, 0x3FFC
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x3FFC, 0x22222222);
    write_mem_at_u32(&mem, 0x4000, 0x11111111);
    write_mem_at_u32(&mem, 0x4004, 0x33333333);
    write_mem_at_u32(&mem, 0x4008, 0x44444444);
    write_mem_at_u32(&mem, 0x400C, 0x55555555);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 3, "Should find match at boundary");
}

#[test]
fn test_repne_scasq_incremental_values() {
    let code = [
        0x48, 0xb8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 4, "Should find value 5 at position 5");
}

#[test]
fn test_repne_scasb_alternating_pattern() {
    let code = [
        0xb0, 0x55, // MOV AL, 0x55
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, if i % 2 == 0 { 0xAA } else { 0x55 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 14, "Should find 0x55 at odd position");
}

#[test]
fn test_repne_scasw_single_word() {
    let code = [
        0x66, 0xb8, 0x78, 0x56, // MOV AX, 0x5678
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x4000, 0x5678);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "Should find match on single comparison");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_repne_scasd_powers_of_two() {
    let code = [
        0xb8, 0x40, 0x00, 0x00, 0x00, // MOV EAX, 64
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let powers = [1u32, 2, 4, 8, 16, 32, 64, 128];
    for i in 0..8u64 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, powers[i as usize]);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 1, "Should find 64 at position 6");
}

#[test]
fn test_repne_scasq_null_sentinel() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..20 {
        write_mem_at_u64(
            &mem,
            0x4000 + i * 8,
            if i == 12 { 0 } else { i as u64 + 1000 },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 7, "Should find null at position 12");
}

#[test]
fn test_repne_scasb_sequential_bytes() {
    let code = [
        0xb0, 0x0f, // MOV AL, 15
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u8(&mem, 0x4000 + i, i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 16, "Should find byte 15 at position 15");
}

#[test]
fn test_repne_scasw_high_bit_pattern() {
    let code = [
        0x66, 0xb8, 0x00, 0x80, // MOV AX, 0x8000
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0c, 0x00, 0x00, 0x00, // MOV RCX, 12
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..12 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 8 { 0x8000 } else { i as u16 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 3, "Should find high-bit word");
}

#[test]
fn test_repne_scasd_backward_scan() {
    let code = [
        0xb8, 0xaa, 0xaa, 0xaa, 0xaa, // MOV EAX, 0xAAAAAAAA
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x07, 0x00, 0x00, 0x00, // MOV RCX, 7
        0xfd, // STD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..7 {
        write_mem_at_u32(
            &mem,
            0x4000 + i * 4,
            if i == 3 { 0xAAAAAAAA } else { 0xBBBBBBBB },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rcx < 7, "Should find match during backward scan");
}

#[test]
fn test_repne_scasq_sparse_values() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23,
        0x01, // MOV RAX, 0x0123456789ABCDEF
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0x48, 0xaf, // REPNE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..16 {
        write_mem_at_u64(
            &mem,
            0x4000 + i * 8,
            if i == 11 {
                0x0123456789ABCDEF
            } else {
                i as u64
            },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 4, "Should find magic value");
}

#[test]
fn test_repne_scasb_hex_digits() {
    let code = [
        0xb0, 0x46, // MOV AL, 'F'
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xfc, // CLD
        0xf2, 0xae, // REPNE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let hex = b"0123456789ABCDEF";
    for i in 0..16u64 {
        write_mem_at_u8(&mem, 0x4000 + i, hex[i as usize]);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "Should find 'F' at end");
}

#[test]
fn test_repne_scasw_unicode_bom() {
    let code = [
        0x66, 0xb8, 0xff, 0xfe, // MOV AX, 0xFEFF
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0x66, 0xaf, // REPNE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, if i == 4 { 0xFEFF } else { 0x0041 });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 5, "Should find BOM marker");
}

#[test]
fn test_repne_scasd_signed_negative() {
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0xfc, // CLD
        0xf2, 0xaf, // REPNE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..10 {
        write_mem_at_u32(
            &mem,
            0x4000 + i * 4,
            if i == 6 { 0xFFFFFFFF } else { i as u32 },
        );
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 3, "Should find -1 value");
}
