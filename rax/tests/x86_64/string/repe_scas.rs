use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// REPE/REPZ SCAS - Repeat Scan String While Equal/Zero
//
// Opcodes:
//   F3 AE        - REPE SCASB (repeat scan byte while ZF=1)
//   F3 66 AF     - REPE SCASW (repeat scan word while ZF=1)
//   F3 AF        - REPE SCASD (repeat scan doubleword while ZF=1)
//   F3 REX.W AF  - REPE SCASQ (repeat scan quadword while ZF=1)
//
// Operation:
// WHILE RCX != 0 DO
//   temp := AL/AX/EAX/RAX - [RDI]
//   Set flags based on temp
//   IF DF = 0 THEN RDI += size
//   ELSE RDI -= size
//   RCX -= 1
//   IF ZF = 0 THEN exit loop (found non-matching value)
// END
//
// Terminates when RCX=0 OR when [RDI] != AL/AX/EAX/RAX (ZF=0)
// Scans for first value that does NOT match accumulator
//
// Based on: /Users/int/dev/rax/docs/rep:repe:repz:repne:repnz.txt

// ============================================================================
// REPE SCASB - Repeat Scan Byte While Equal
// ============================================================================

#[test]
fn test_repe_scasb_all_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Fill with 0x42
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x42);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "All match, scan all");
    assert_eq!(regs.rdi, 0x4008);
    assert!(zf_set(regs.rflags), "ZF set (last matched)");
}

#[test]
fn test_repe_scasb_mismatch_at_start() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte different
    write_mem_at_u8(&mem, 0x4000, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Stopped after 1");
    assert_eq!(regs.rdi, 0x4001);
    assert!(!zf_set(regs.rflags), "ZF clear (mismatch)");
}

#[test]
fn test_repe_scasb_find_non_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Fill with 0xAA, except position 10
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, if i == 10 { 0x55 } else { 0xAA });
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 5, "Found mismatch at position 10");
    assert_eq!(regs.rdi, 0x400B);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasb_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb0, 0x42, // MOV AL, 0x42
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_scasb_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x07, 0x40, 0x00, 0x00, // MOV RDI, 0x4007
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb0, 0x55, // MOV AL, 0x55
        0xfd, // STD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..8 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x55);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFF);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_scasb_strlen_like() {
    // Find null terminator (non-zero scan)
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RCX, max
        0xb0, 0x00, // MOV AL, 0 (looking for non-zero)
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // String "HELLO" with null at position 5
    let s = b"HELLO\0";
    for (i, &byte) in s.iter().enumerate() {
        write_mem_at_u8(&mem, 0x4000 + i as u64, byte);
    }
    // Fill rest with data
    for i in 6..16 {
        write_mem_at_u8(&mem, 0x4000 + i, b'X');
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Note: REPE with AL=0 finds first NON-zero
    // Position 0 is 'H' (non-zero), stops immediately
    assert_eq!(regs.rdi, 0x4001);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasb_scan_zeros() {
    // Scan past zeros
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0x00, // MOV AL, 0
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Zeros at 0-7, non-zero at 8
    for i in 0..8 {
        write_mem_at_u8(&mem, 0x4000 + i, 0);
    }
    write_mem_at_u8(&mem, 0x4008, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7, "Stopped at first non-zero");
    assert_eq!(regs.rdi, 0x4009);
}

#[test]
fn test_repe_scasb_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 256
        0xb0, 0x77, // MOV AL, 0x77
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..256 {
        write_mem_at_u8(&mem, 0x4000 + i, 0x77);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

#[test]
fn test_repe_scasb_sets_flags() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xb0, 0x30, // MOV AL, 0x30
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
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

// ============================================================================
// REPE SCASW - Repeat Scan Word While Equal
// ============================================================================

#[test]
fn test_repe_scasw_all_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x1234);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4008);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_scasw_mismatch() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0x11, 0x11, // MOV AX, 0x1111
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, 0x4000, 0x1111);
    write_mem_at_u16(&mem, 0x4002, 0x2222); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4004);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasw_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x66, 0xb8, 0xFF, 0xFF, // MOV AX, 0xFFFF
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_scasw_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x06, 0x40, 0x00, 0x00, // MOV RDI, 0x4006
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x66, 0xb8, 0xCD, 0xAB, // MOV AX, 0xABCD
        0xfd, // STD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0xABCD);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFE);
}

#[test]
fn test_repe_scasw_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 128
        0x66, 0xb8, 0x88, 0x77, // MOV AX, 0x7788
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..128 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x7788);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

// ============================================================================
// REPE SCASD - Repeat Scan Doubleword While Equal
// ============================================================================

#[test]
fn test_repe_scasd_all_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x12345678);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4010);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_scasd_mismatch() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u32(&mem, 0x4000, 0x11111111);
    write_mem_at_u32(&mem, 0x4004, 0x22222222); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4008);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasd_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_scasd_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x0C, 0x40, 0x00, 0x00, // MOV RDI, 0x400C
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EAX, 0xDEADBEEF
        0xfd, // STD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xDEADBEEF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFC);
}

#[test]
fn test_repe_scasd_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x40, 0x00, 0x00, 0x00, // MOV RCX, 64
        0xb8, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EAX, 0xDDCCBBAA
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..64 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xDDCCBBAA);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

// ============================================================================
// REPE SCASQ - Repeat Scan Quadword While Equal
// ============================================================================

#[test]
fn test_repe_scasq_all_match() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x1234567890ABCDEF);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4020);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_scasq_mismatch() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x1111...
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u64(&mem, 0x4000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x4008, 0x2222222222222222); // Mismatch

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 2);
    assert_eq!(regs.rdi, 0x4010);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasq_zero_count() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4000);
}

#[test]
fn test_repe_scasq_backward() {
    let code = [
        0x48, 0xc7, 0xc7, 0x18, 0x40, 0x00, 0x00, // MOV RDI, 0x4018
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xBE, 0xBA, 0xFE, 0xCA, 0xEF, 0xBE, 0xAD,
        0xDE, // MOV RAX, 0xDEADBEEFCAFEBABE
        0xfd, // STD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xDEADBEEFCAFEBABE);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FF8);
}

#[test]
fn test_repe_scasq_large() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0x48, 0xb8, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x2222...
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..32 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0x2222222222222222);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4100);
}

// ============================================================================
// Edge cases and mixed tests
// ============================================================================

#[test]
fn test_repe_scas_stops_on_mismatch() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // First byte different
    write_mem_at_u8(&mem, 0x4000, 0x55);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 99);
    assert_eq!(regs.rdi, 0x4001);
}

#[test]
fn test_repe_scas_preserves_other_regs() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0xc7, 0xc6, 0x99, 0x00, 0x00, 0x00, // MOV RSI, 0x99
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rsi, 0x99);
}

#[test]
fn test_repe_scasq_pointer_scan() {
    // Scan for non-null pointer
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x31, 0xc0, // XOR RAX, RAX (null)
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Nulls for first 8, non-null at position 8
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0);
    }
    write_mem_at_u64(&mem, 0x4040, 0x1000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 7);
    assert_eq!(regs.rdi, 0x4048);
}

#[test]
fn test_repe_scasb_count_100() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xb0, 0xCC, // MOV AL, 0xCC
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..100 {
        write_mem_at_u8(&mem, 0x4000 + i, 0xCC);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x4064);
}

#[test]
fn test_repe_scasw_pattern_search() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0x66, 0xb8, 0xFF, 0xFF, // MOV AX, 0xFFFF
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..20 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0xFFFF);
    }
    write_mem_at_u16(&mem, 0x4028, 0x0000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 11);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasd_boundary_check() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x80000000);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasq_incremental_values() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x0A, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x31, 0xc0, // XOR RAX, RAX (0)
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..5 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0);
    }
    write_mem_at_u64(&mem, 0x4028, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // After 5 matching 0s, the 6th value (1) doesn't match
    // RCX is decremented after each iteration, so: 10 - 6 = 4
    assert_eq!(regs.rcx, 4);
}

#[test]
fn test_repe_scasb_alternating_pattern() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, 0xAA, // MOV AL, 0xAA
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in (0..16).step_by(2) {
        write_mem_at_u8(&mem, 0x4000 + i, 0xAA);
        write_mem_at_u8(&mem, 0x4000 + i + 1, 0x55);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // First byte matches (0xAA), second (0x55) doesn't
    // RCX decremented after each: 16 - 2 = 14
    assert_eq!(regs.rcx, 14);
}

#[test]
fn test_repe_scasw_single_word() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0xb8, 0x00, 0x80, // MOV AX, 0x8000
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u16(&mem, 0x4000, 0x8000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert!(zf_set(regs.rflags));
}

#[test]
fn test_repe_scasd_powers_of_two_scan() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb8, 0x00, 0x00, 0x00, 0x01, // MOV EAX, 0x01000000
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..10 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x01000000);
    }
    write_mem_at_u32(&mem, 0x4028, 0x02000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 10 values match, 11th doesn't. RCX: 16 - 11 = 5
    assert_eq!(regs.rcx, 5);
}

#[test]
fn test_repe_scasq_null_terminated_array() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xFFFFFFFFFFFFFFFF);
    }
    write_mem_at_u64(&mem, 0x4080, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 16 values match (-1), 17th (0) doesn't. RCX: 32 - 17 = 15
    assert_eq!(regs.rcx, 15);
}

#[test]
fn test_repe_scasb_sequential_bytes() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, 0x30, // MOV AL, '0'
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..10 {
        write_mem_at_u8(&mem, 0x4000 + i, b'0');
    }
    write_mem_at_u8(&mem, 0x400A, b'1');
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 21);
    assert!(!zf_set(regs.rflags));
}

#[test]
fn test_repe_scasw_high_bit_pattern() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x66, 0xb8, 0x00, 0x80, // MOV AX, 0x8000
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0x8000);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasd_backward_scan() {
    let code = [
        0x48, 0xc7, 0xc7, 0x1C, 0x40, 0x00, 0x00, // MOV RDI, 0x401C
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xb8, 0x99, 0x99, 0x99, 0x99, // MOV EAX, 0x99999999
        0xfd, // STD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..8 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0x99999999);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, 0x3FFC);
}

#[test]
fn test_repe_scasq_sparse_nulls() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x4000, 0);
    write_mem_at_u64(&mem, 0x4008, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // First value matches (0), second (1) doesn't. RCX: 16 - 2 = 14
    assert_eq!(regs.rcx, 14);
}

#[test]
fn test_repe_scasb_hex_digits() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0xb0, b'F', // MOV AL, 'F'
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..16 {
        write_mem_at_u8(&mem, 0x4000 + i, b'F');
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasw_unicode_bom() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x66, 0xb8, 0xFF, 0xFE, // MOV AX, 0xFEFF (UTF-16 LE BOM)
        0xfc, // CLD
        0xf3, 0x66, 0xaf, // REPE SCASW
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..8 {
        write_mem_at_u16(&mem, 0x4000 + i * 2, 0xFEFF);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasd_signed_negative() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, -1
        0xfc, // CLD
        0xf3, 0xaf, // REPE SCASD
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..4 {
        write_mem_at_u32(&mem, 0x4000 + i * 4, 0xFFFFFFFF);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasq_magic_numbers() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xb8, 0xEF, 0xBE, 0xAD, 0xDE, 0x0D, 0xF0, 0xAD,
        0xBA, // MOV RAX, 0xBAADF00DDEADBEEF
        0xfc, // CLD
        0xf3, 0x48, 0xaf, // REPE SCASQ
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..4 {
        write_mem_at_u64(&mem, 0x4000 + i * 8, 0xBAADF00DDEADBEEF);
    }
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_repe_scasb_space_padding() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x40, 0x00, 0x00, // MOV RDI, 0x4000
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 32
        0xb0, b' ', // MOV AL, space
        0xfc, // CLD
        0xf3, 0xae, // REPE SCASB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    for i in 0..20 {
        write_mem_at_u8(&mem, 0x4000 + i, b' ');
    }
    write_mem_at_u8(&mem, 0x4014, b'A');
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 11);
}
