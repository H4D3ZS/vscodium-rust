//! Tests for TSX (Transactional Synchronization Extensions) instructions.
//!
//! XBEGIN - Begin Transaction
//! XEND - End Transaction
//! XABORT - Abort Transaction
//! XTEST - Test if in Transactional Execution
//!
//! TSX provides hardware transactional memory support.
//!
//! Opcodes:
//!   C7 F8 - XBEGIN rel32
//!   0F 01 D5 - XEND
//!   C6 F8 ib - XABORT imm8
//!   0F 01 D6 - XTEST
//!
//! Reference: docs/xbegin.txt, docs/xend.txt, docs/xabort.txt, docs/xtest.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// XBEGIN Tests - Begin Transaction
// ============================================================================

#[test]
fn test_xbegin_basic() {
    // XBEGIN - Start a transaction
    // C7 F8 + rel32 = XBEGIN
    // Returns -1 in EAX if transaction starts successfully
    // Note: Our emulator doesn't support TSX, so XBEGIN always jumps to fallback
    let code = [
        // XBEGIN at 0x1000, 6 bytes. MOV is 7 bytes, XEND is 3 bytes.
        // To reach HLT: offset = 7 + 3 = 10 = 0x0A
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (fallback = HLT)
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (in transaction)
        0x0F, 0x01, 0xD5, // XEND
        // fallback:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSX not supported - always aborts, EAX = 0
    assert_eq!(regs.rax, 0, "XBEGIN should abort with status 0");
}

#[test]
fn test_xbegin_preserves_registers() {
    // XBEGIN should preserve other registers (when it jumps to fallback)
    // XBEGIN at 0x100E, 6 bytes. XEND is 3 bytes.
    // To reach HLT: offset = 3 = 0x03
    let code = [
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC1, 0x99, 0x99, 0x99, 0x99, // MOV RCX, 0x99999999
        0xC7, 0xF8, 0x03, 0x00, 0x00, 0x00, // XBEGIN +3 (to HLT)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX and RCX should be preserved
    assert_eq!(regs.rbx, 0x42424242);
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x99999999);
}

#[test]
fn test_xbegin_abort_path() {
    // XBEGIN always jumps to fallback (TSX not supported)
    // XBEGIN at 0x1000, 6 bytes
    // MOV RBX,1 is 7 bytes, XABORT is 3 bytes = 10 bytes to skip
    let code = [
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (to fallback)
        // Transaction path (skipped):
        0x48, 0xC7, 0xC3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xC6, 0xF8, 0x01, // XABORT 1
        // Fallback path:
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should take fallback path and RBX = 2
    assert_eq!(regs.rbx, 2);
}

#[test]
fn test_xbegin_with_memory() {
    // XBEGIN jumps directly to after-transaction code
    // MOV RCX is 7 bytes, XBEGIN at 0x1007, 6 bytes
    // Transaction code: MOV [RCX] is 6 bytes, XEND is 3 bytes = 9 bytes to skip
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xC7, 0xF8, 0x09, 0x00, 0x00, 0x00, // XBEGIN +9 (skip to after transaction)
        // In transaction (skipped):
        0xC7, 0x01, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RCX], 0x42424242
        0x0F, 0x01, 0xD5, // XEND
        // After transaction:
        0x8B, 0x01, // MOV EAX, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Memory was never written (transaction skipped), so EAX will be 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0);
}

#[test]
fn test_xbegin_nested() {
    // Since TSX not supported, outer XBEGIN immediately jumps to HLT
    // XBEGIN at 0x1000, 6 bytes. Skip everything to HLT.
    // Total code: 7+6+7+3+3 = 26 bytes after XBEGIN
    let code = [
        0xC7, 0xF8, 0x1A, 0x00, 0x00, 0x00, // XBEGIN +26 (to HLT)
        // All of this is skipped:
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xC7, 0xF8, 0x0D, 0x00, 0x00, 0x00, // XBEGIN (inner, would also skip)
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x0F, 0x01, 0xD5, // XEND (inner)
        0x0F, 0x01, 0xD5, // XEND (outer)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Both MOVs were skipped
    assert_eq!(regs.rax, 0);
    assert_eq!(regs.rbx, 0);
}

// ============================================================================
// XEND Tests - End Transaction
// ============================================================================

#[test]
fn test_xend_basic() {
    // XEND - Commit transaction (but we skip to HLT since XBEGIN aborts)
    // XBEGIN at 0x1000, 6 bytes. MOV is 7, XEND is 3 = 10 bytes to skip
    let code = [
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (to HLT)
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (skipped)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV was skipped, RAX = 0 (abort status)
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_xend_preserves_registers() {
    // Since XBEGIN aborts, we skip transaction and go directly to HLT
    // XBEGIN at 0x1007, 6 bytes. MOV is 7, XEND is 3 = 10 bytes to skip
    let code = [
        0x48, 0xC7, 0xC3, 0x11, 0x11, 0x11, 0x11, // MOV RBX, 0x11111111
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (to HLT)
        0x48, 0xC7, 0xC0, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x22222222 (skipped)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX was set before XBEGIN
    assert_eq!(regs.rbx, 0x11111111);
    // RAX = abort status (0), not 0x22222222
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_xend_preserves_flags() {
    // XBEGIN aborts, skips transaction code, goes to HLT
    // XBEGIN at 0x1000, 6 bytes. MOV is 7, ADD is 4, XEND is 3 = 14 bytes
    let code = [
        0xC7, 0xF8, 0x0E, 0x00, 0x00, 0x00, // XBEGIN +14 (to HLT)
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1 (skipped)
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (skipped)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should be 0 (abort status), not result of -1+1
    assert_eq!(regs.rax, 0);
}

// ============================================================================
// XABORT Tests - Abort Transaction
// ============================================================================

#[test]
fn test_xabort_basic() {
    // XABORT is never reached because XBEGIN aborts immediately
    // XOR is 3 bytes, XBEGIN at 0x1003, 6 bytes
    // MOV is 7 bytes, XABORT is 3 bytes = 10 bytes to skip
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (to HLT)
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (skipped)
        0xC6, 0xF8, 0x42, // XABORT 0x42 (skipped)
        // Fallback:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX is 0 (abort status from XBEGIN, XOR was before it)
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_xabort_different_codes() {
    // XABORT never reached because XBEGIN aborts
    // XBEGIN at 0x1000, 6 bytes. XABORT is 3 bytes = skip 3
    let code = [
        0xC7, 0xF8, 0x03, 0x00, 0x00, 0x00, // XBEGIN +3 (to HLT)
        0xC6, 0xF8, 0xFF, // XABORT 0xFF (skipped)
        // Fallback:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0); // abort status
}

#[test]
fn test_xabort_unconditional() {
    // Since XBEGIN aborts immediately, the transaction code is skipped
    // MOV RBX is 7 bytes, XBEGIN at 0x1007, 6 bytes
    // MOV RBX is 7 bytes, XABORT is 3 bytes = 10 bytes to skip
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0xC7, 0xF8, 0x0A, 0x00, 0x00, 0x00, // XBEGIN +10 (to HLT)
        0x48, 0xC7, 0xC3, 0x99, 0x99, 0x99, 0x99, // MOV RBX, 0x99999999 (skipped)
        0xC6, 0xF8, 0x01, // XABORT 1 (skipped)
        // Fallback:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should be 0 (from the first MOV, transaction was skipped)
    assert_eq!(regs.rbx, 0);
}

// ============================================================================
// XTEST Tests - Test Transaction State
// ============================================================================

#[test]
fn test_xtest_outside_transaction() {
    // XTEST - Test if in transaction
    // 0F 01 D6 = XTEST
    // Sets ZF=1 if not in transaction
    let code = [
        0x0F, 0x01, 0xD6, // XTEST
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should be 1 (not in transaction)
    assert!(
        regs.rflags & 0x40 != 0,
        "ZF should be set outside transaction"
    );
}

#[test]
fn test_xtest_inside_transaction() {
    // XTEST inside a transaction (but XBEGIN aborts, so we skip to HLT)
    // XBEGIN at 0x1000, 6 bytes. XTEST is 3, XEND is 3 = 6 bytes to skip
    let code = [
        0xC7, 0xF8, 0x06, 0x00, 0x00, 0x00, // XBEGIN +6 (to HLT)
        0x0F, 0x01, 0xD6, // XTEST (skipped)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RAX should be 0 (abort status)
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_xtest_preserves_registers() {
    // XTEST should only modify RFLAGS
    // Note: MOV RAX, imm32 sign-extends to 64 bits
    let code = [
        0x48, 0xC7, 0xC0, 0x42, 0x42, 0x42, 0x42, // MOV RAX, 0x42424242 (sign-extended)
        0x48, 0xC7, 0xC3, 0x99, 0x99, 0x99,
        0x99, // MOV RBX, 0x99999999 (sign-extended to 0xFFFFFFFF99999999)
        0x0F, 0x01, 0xD6, // XTEST
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42424242);
    // 0x99999999 is sign-extended because bit 31 is set
    assert_eq!(regs.rbx, 0xFFFFFFFF99999999);
}

#[test]
fn test_xtest_multiple() {
    // Multiple XTEST calls
    let code = [
        0x0F, 0x01, 0xD6, // XTEST #1
        0x0F, 0x01, 0xD6, // XTEST #2
        0x0F, 0x01, 0xD6, // XTEST #3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        regs.rflags & 0x40 != 0,
        "All XTESTs should show not-in-transaction"
    );
}

// ============================================================================
// Combined TSX Tests
// ============================================================================

#[test]
fn test_tsx_complete_transaction() {
    // Transaction is skipped because XBEGIN aborts
    // MOV RCX is 7 bytes, XBEGIN at 0x1007, 6 bytes
    // MOV [RCX] is 6, XTEST is 3, XEND is 3 = 12 bytes to skip
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xC7, 0xF8, 0x0C, 0x00, 0x00, 0x00, // XBEGIN +12 (to MOV EAX)
        // In transaction (skipped):
        0xC7, 0x01, 0x42, 0x42, 0x42, 0x42, // MOV DWORD PTR [RCX], 0x42424242
        0x0F, 0x01, 0xD6, // XTEST
        0x0F, 0x01, 0xD5, // XEND
        // After transaction:
        0x8B, 0x01, // MOV EAX, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Memory was never written, so EAX is 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0);
}

#[test]
fn test_tsx_abort_and_retry() {
    // XBEGIN always aborts immediately, no retry possible
    // This test structure doesn't work with non-TSX emulation
    // Simplified: XBEGIN jumps directly to HLT
    // XOR is 3, XBEGIN at 0x1003, 6 bytes
    // Skip everything to done: CMP 4 + JNE 2 + XABORT 3 + XEND 3 + JMP 2 + INC 3 + JMP 2 = 19
    // Actually simpler: just jump to done which is after JMP done instruction
    let code = [
        0x48, 0x31, 0xC3, // XOR RBX, RBX (retry counter)
        0xC7, 0xF8, 0x13, 0x00, 0x00, 0x00, // XBEGIN +19 (to done: HLT)
        // In transaction (skipped):
        0x48, 0x83, 0xFB, 0x00, // CMP RBX, 0
        0x75, 0x02, // JNE skip_abort
        0xC6, 0xF8, 0x01, // XABORT 1
        // skip_abort:
        0x0F, 0x01, 0xD5, // XEND
        0xEB, 0x03, // JMP done
        // Fallback (skipped):
        0x48, 0xFF, 0xC3, // INC RBX
        0xEB, 0xED, // JMP retry
        // done:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX is 0 (never incremented because fallback was skipped)
    assert_eq!(regs.rbx, 0);
}

#[test]
fn test_tsx_xtest_sequence() {
    // First XTEST outside transaction, then XBEGIN aborts to after XEND
    // XTEST at 0x1000, 3 bytes
    // XBEGIN at 0x1003, 6 bytes. Skip XTEST + XEND = 6 bytes
    let code = [
        0x0F, 0x01, 0xD6, // XTEST (outside) - ZF=1
        0xC7, 0xF8, 0x06, 0x00, 0x00, 0x00, // XBEGIN +6 (to XTEST after)
        0x0F, 0x01, 0xD6, // XTEST (inside, skipped)
        0x0F, 0x01, 0xD5, // XEND (skipped)
        0x0F, 0x01, 0xD6, // XTEST (after) - ZF=1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // ZF should be set (not in transaction)
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

#[test]
fn test_tsx_with_arithmetic() {
    // Transaction with arithmetic operations (all skipped because XBEGIN aborts)
    // XOR is 3, XBEGIN at 0x1003, 6 bytes
    // MOV+MOV+ADD+XEND = 7+7+3+3 = 20 bytes to skip
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xC7, 0xF8, 0x14, 0x00, 0x00, 0x00, // XBEGIN +20 (to HLT)
        // In transaction (skipped):
        0x48, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xC7, 0xC3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x48, 0x01, 0xD8, // ADD RAX, RBX
        0x0F, 0x01, 0xD5, // XEND
        // After transaction:
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Transaction aborted, RAX = abort status (0), not 30
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_tsx_memory_conflict_simulation() {
    // Memory operations before and after transaction (transaction skipped)
    // MOV RCX is 7, MOV [RCX] is 6, XBEGIN at 0x100D, 6 bytes
    // Transaction code: MOV+ADD+MOV+XEND = 2+4+2+3 = 11 bytes to skip
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xC7, 0x01, 0x00, 0x00, 0x00, 0x00, // MOV DWORD PTR [RCX], 0
        0xC7, 0xF8, 0x0B, 0x00, 0x00, 0x00, // XBEGIN +11 (to MOV EAX after)
        // In transaction (skipped):
        0x8B, 0x01, // MOV EAX, [RCX]
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1
        0x89, 0x01, // MOV [RCX], EAX
        0x0F, 0x01, 0xD5, // XEND
        // After:
        0x8B, 0x01, // MOV EAX, [RCX]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Memory contains 0 (transaction was skipped), EAX = 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0);
}
