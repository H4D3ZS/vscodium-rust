//! Tests for the FCLEX and FNCLEX instructions.
//!
//! FCLEX - Clear FPU Exception Flags (with exception check)
//! FNCLEX - Clear FPU Exception Flags (without exception check)
//!
//! FCLEX and FNCLEX clear the floating-point exception flags (PE, UE, OE, ZE, DE, and IE),
//! the exception summary status flag (ES), the stack fault flag (SF), and the busy flag (B)
//! in the FPU status word. FCLEX checks for pending exceptions before clearing, while FNCLEX does not.
//!
//! Opcodes:
//! - FCLEX: 9B DB E2
//! - FNCLEX: DB E2
//!
//! Status Word Bits Cleared:
//! - Bit 0: IE (Invalid Operation)
//! - Bit 1: DE (Denormalized Operand)
//! - Bit 2: ZE (Zero Divide)
//! - Bit 3: OE (Overflow)
//! - Bit 4: UE (Underflow)
//! - Bit 5: PE (Precision)
//! - Bit 6: SF (Stack Fault)
//! - Bit 7: ES (Exception Summary Status)
//! - Bit 15: B (Busy)
//!
//! Condition codes C0, C1, C2, C3 are undefined after FCLEX/FNCLEX.
//!
//! References: /Users/int/dev/rax/docs/fclex:fnclex.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// Status word bit definitions
const IE_BIT: u16 = 0x0001; // Invalid Operation
const DE_BIT: u16 = 0x0002; // Denormalized Operand
const ZE_BIT: u16 = 0x0004; // Zero Divide
const OE_BIT: u16 = 0x0008; // Overflow
const UE_BIT: u16 = 0x0010; // Underflow
const PE_BIT: u16 = 0x0020; // Precision
const SF_BIT: u16 = 0x0040; // Stack Fault
const ES_BIT: u16 = 0x0080; // Exception Summary Status
const B_BIT: u16 = 0x8000; // Busy

const EXCEPTION_MASK: u16 = IE_BIT | DE_BIT | ZE_BIT | OE_BIT | UE_BIT | PE_BIT;

// Helper function to write u16 to memory
fn write_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: u16) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read u16 from memory
fn read_u16(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u16 {
    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u16::from_le_bytes(buf)
}

// Helper function to write f64 to memory
fn write_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: f64) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// ============================================================================
// FNCLEX - Clear Exception Flags without Wait
// ============================================================================

#[test]
fn test_fnclex_basic() {
    // Basic FNCLEX operation
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    // Exception flags should be cleared
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "Exception flags should be cleared"
    );
}

#[test]
fn test_fnclex_clears_exception_flags() {
    // FNCLEX should clear exception flags
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "FNCLEX should clear all exception flags"
    );
}

#[test]
fn test_fnclex_multiple_times() {
    // Multiple FNCLEX operations
    let code = [
        0xDB, 0xE2, // FNCLEX (1st time)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDB, 0xE2, // FNCLEX (2nd time)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x02, 0x30, 0x00, 0x00, // MOV word [0x3002], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status1 = read_u16(&mem, 0x3000);
    let status2 = read_u16(&mem, 0x3002);
    assert_eq!(
        status1 & EXCEPTION_MASK,
        0,
        "First FNCLEX should clear exceptions"
    );
    assert_eq!(
        status2 & EXCEPTION_MASK,
        0,
        "Second FNCLEX should clear exceptions"
    );
}

#[test]
fn test_fnclex_clears_es_bit() {
    // FNCLEX should clear the ES (Exception Summary Status) bit
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & ES_BIT, 0, "ES bit should be cleared");
}

#[test]
fn test_fnclex_clears_sf_bit() {
    // FNCLEX should clear the SF (Stack Fault) bit
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & SF_BIT, 0, "SF bit should be cleared");
}

// ============================================================================
// FCLEX - Clear Exception Flags with Wait
// ============================================================================

#[test]
fn test_fclex_basic() {
    // Basic FCLEX operation with FWAIT prefix
    let code = [
        0x9B, 0xDB, 0xE2, // FCLEX (with FWAIT)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "FCLEX should clear exception flags"
    );
}

#[test]
fn test_fclex_clears_exception_flags() {
    // FCLEX should clear exception flags
    let code = [
        0x9B, 0xDB, 0xE2, // FCLEX (with FWAIT)
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "FCLEX should clear all exception flags"
    );
}

// ============================================================================
// FCLEX vs FNCLEX Equivalence
// ============================================================================

#[test]
fn test_fclex_vs_fnclex() {
    // FCLEX and FNCLEX should have same effect in normal operation
    let code1 = [
        0x9B, 0xDB, 0xE2, // FCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let code2 = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    run_until_hlt(&mut vcpu1).unwrap();
    let status1 = read_u16(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    run_until_hlt(&mut vcpu2).unwrap();
    let status2 = read_u16(&mem2, 0x3000);

    assert_eq!(status1, status2, "FCLEX and FNCLEX should give same result");
}

// ============================================================================
// Exception Flag Preservation after FNCLEX
// ============================================================================

#[test]
fn test_fnclex_then_fnstsw() {
    // Verify exception flags stay cleared
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDF, 0xE0, // FNSTSW AX (2nd time)
        0x66, 0x89, 0x04, 0x25, 0x02, 0x30, 0x00, 0x00, // MOV word [0x3002], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status1 = read_u16(&mem, 0x3000);
    let status2 = read_u16(&mem, 0x3002);
    assert_eq!(
        status1 & EXCEPTION_MASK,
        0,
        "First FNSTSW should show cleared exceptions"
    );
    assert_eq!(
        status2 & EXCEPTION_MASK,
        0,
        "Second FNSTSW should show cleared exceptions"
    );
}

// ============================================================================
// FNCLEX with FPU Operations
// ============================================================================

#[test]
fn test_fnclex_before_operations() {
    // FNCLEX before FPU operations
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "Exceptions should still be cleared after FLD"
    );
}

#[test]
fn test_fnclex_after_operations() {
    // FNCLEX after FPU operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "FNCLEX should clear exceptions after arithmetic"
    );
}

// ============================================================================
// Specific Exception Flags
// ============================================================================

#[test]
fn test_fnclex_individual_exception_bits() {
    // Test that FNCLEX clears each exception bit
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(status & IE_BIT, 0, "IE bit should be cleared");
    assert_eq!(status & DE_BIT, 0, "DE bit should be cleared");
    assert_eq!(status & ZE_BIT, 0, "ZE bit should be cleared");
    assert_eq!(status & OE_BIT, 0, "OE bit should be cleared");
    assert_eq!(status & UE_BIT, 0, "UE bit should be cleared");
    assert_eq!(status & PE_BIT, 0, "PE bit should be cleared");
}

// ============================================================================
// Sequential FNCLEX Operations
// ============================================================================

#[test]
fn test_sequential_fnclex() {
    // Multiple sequential FNCLEX operations
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDB, 0xE2, // FNCLEX
        0xDB, 0xE2, // FNCLEX
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "Multiple FNCLEX should clear all exceptions"
    );
}

// ============================================================================
// FNCLEX with Comparison Operations
// ============================================================================

#[test]
fn test_fnclex_after_comparison() {
    // FNCLEX after comparison (condition codes should not be cleared by FNCLEX)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0xDB, 0xE2, // FNCLEX (clears exception flags, not condition codes)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "Exception flags should be cleared"
    );
    // Condition codes may or may not be preserved (C0, C1, C2, C3 are undefined after FNCLEX)
}

// ============================================================================
// FNCLEX Integration Tests
// ============================================================================

#[test]
fn test_fnclex_complete_flow() {
    // Complete flow with FNCLEX
    let code = [
        0xDB, 0xE2, // FNCLEX (clear any initial exceptions)
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000] (before clear)
        0xDB, 0xE2, // FNCLEX
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTSW [0x3002] (after clear)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let status_before = read_u16(&mem, 0x3000);
    let status_after = read_u16(&mem, 0x3002);
    assert_eq!(
        status_after & EXCEPTION_MASK,
        0,
        "Exceptions should be cleared after FNCLEX"
    );
}

#[test]
fn test_fclex_multiple_operations() {
    // FCLEX with multiple operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0x9B, 0xDB, 0xE2, // FCLEX
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDE, 0xC1, // FADDP
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    assert_eq!(
        status & EXCEPTION_MASK,
        0,
        "Exceptions should remain cleared"
    );
}

// ============================================================================
// Status Word State Verification
// ============================================================================

#[test]
fn test_fnclex_status_word_clean_state() {
    // Verify clean state after FNCLEX
    let code = [
        0xDB, 0xE2, // FNCLEX
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let status = read_u16(&mem, 0x3000);
    // All exception bits should be 0
    assert_eq!(status & IE_BIT, 0);
    assert_eq!(status & DE_BIT, 0);
    assert_eq!(status & ZE_BIT, 0);
    assert_eq!(status & OE_BIT, 0);
    assert_eq!(status & UE_BIT, 0);
    assert_eq!(status & PE_BIT, 0);
    assert_eq!(status & ES_BIT, 0);
    assert_eq!(status & SF_BIT, 0);
}

#[test]
fn test_fnclex_preserves_other_bits() {
    // FNCLEX should only clear exception flags, not affect other FPU state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000] (before)
        0xDB, 0xE2, // FNCLEX
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTSW [0x3002] (after)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    let status_before = read_u16(&mem, 0x3000);
    let status_after = read_u16(&mem, 0x3002);

    // TOP should be preserved (bits 11-13)
    let top_before = (status_before >> 11) & 0x7;
    let top_after = (status_after >> 11) & 0x7;
    assert_eq!(top_before, top_after, "TOP should be preserved by FNCLEX");
}
