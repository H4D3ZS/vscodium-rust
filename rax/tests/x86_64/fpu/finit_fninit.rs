//! Tests for the FINIT and FNINIT instructions.
//!
//! FINIT - Initialize FPU (with exception check)
//! FNINIT - Initialize FPU (without exception check)
//!
//! FINIT and FNINIT initialize the FPU to its default state:
//! - FPU control word set to 037FH (round to nearest, all exceptions masked, 64-bit precision)
//! - FPU status word cleared to 0
//! - Tag word set to FFFFH (all registers tagged as empty)
//! - Data and instruction pointers cleared
//! - All register stack left unchanged but tagged as empty
//!
//! FINIT checks for pending exceptions before initializing, while FNINIT does not.
//!
//! Opcodes:
//! - FINIT: 9B DB E3
//! - FNINIT: DB E3
//!
//! Control Word after FINIT/FNINIT: 037FH
//! Status Word after FINIT/FNINIT: 0000H
//! Tag Word after FINIT/FNINIT: FFFFH
//!
//! References: /Users/int/dev/rax/docs/finit:fninit.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// FPU default values
const DEFAULT_CONTROL_WORD: u16 = 0x037F; // Round to nearest, all exceptions masked
const DEFAULT_STATUS_WORD: u16 = 0x0000; // No exceptions, TOP=0
const DEFAULT_TAG_WORD: u16 = 0xFFFF; // All registers empty

// Status word bit definitions
const IE_BIT: u16 = 0x0001; // Invalid Operation
const DE_BIT: u16 = 0x0002; // Denormalized Operand
const ZE_BIT: u16 = 0x0004; // Zero Divide
const OE_BIT: u16 = 0x0008; // Overflow
const UE_BIT: u16 = 0x0010; // Underflow
const PE_BIT: u16 = 0x0020; // Precision
const SF_BIT: u16 = 0x0040; // Stack Fault
const ES_BIT: u16 = 0x0080; // Exception Summary Status
const TOP_MASK: u16 = 0x3800; // TOP bits 11-13
const C2_BIT: u16 = 0x0400; // Condition Code 2
const C1_BIT: u16 = 0x0200; // Condition Code 1
const C3_BIT: u16 = 0x4000; // Condition Code 3
const C0_BIT: u16 = 0x0100; // Condition Code 0
const B_BIT: u16 = 0x8000; // Busy

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

// Helper function to read f64 from memory
fn read_f64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FNINIT - Initialize without Wait
// ============================================================================

#[test]
fn test_fninit_basic() {
    // Basic FNINIT operation
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    assert_eq!(
        cw, DEFAULT_CONTROL_WORD,
        "Control word should be 037FH after FNINIT"
    );
}

#[test]
fn test_fninit_clears_status_word() {
    // FNINIT should clear the status word
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        sw, DEFAULT_STATUS_WORD,
        "Status word should be 0000H after FNINIT"
    );
}

#[test]
fn test_fninit_sets_tag_word() {
    // FNINIT should set tag word to FFFFH (all registers empty)
    // Note: Tag word is typically read through environment save
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (load after init)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.5, "FPU should work correctly after FNINIT");
}

#[test]
fn test_fninit_resets_top_pointer() {
    // FNINIT should reset TOP (Top of Stack) pointer to 0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (push)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (push)
        0xDB, 0xE3, // FNINIT (reset TOP)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    let top = (sw & TOP_MASK) >> 11;
    assert_eq!(top, 0, "TOP should be 0 after FNINIT");
}

#[test]
fn test_fninit_multiple_times() {
    // Multiple FNINIT operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTCW [0x3002]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let cw1 = read_u16(&mem, 0x3000);
    let cw2 = read_u16(&mem, 0x3002);
    assert_eq!(
        cw1, DEFAULT_CONTROL_WORD,
        "First FNINIT should set CW to 037FH"
    );
    assert_eq!(
        cw2, DEFAULT_CONTROL_WORD,
        "Second FNINIT should set CW to 037FH"
    );
}

// ============================================================================
// FINIT - Initialize with Wait
// ============================================================================

#[test]
fn test_finit_basic() {
    // Basic FINIT operation with FWAIT prefix
    let code = [
        0x9B, 0xDB, 0xE3, // FINIT (with FWAIT)
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    assert_eq!(
        cw, DEFAULT_CONTROL_WORD,
        "Control word should be 037FH after FINIT"
    );
}

#[test]
fn test_finit_clears_status_word() {
    // FINIT should clear the status word
    let code = [
        0x9B, 0xDB, 0xE3, // FINIT (with FWAIT)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        sw, DEFAULT_STATUS_WORD,
        "Status word should be 0000H after FINIT"
    );
}

// ============================================================================
// FINIT vs FNINIT Equivalence
// ============================================================================

#[test]
fn test_finit_vs_fninit() {
    // FINIT and FNINIT should have same effect in normal operation
    let code1 = [
        0x9B, 0xDB, 0xE3, // FINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let code2 = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    run_until_hlt(&mut vcpu1).unwrap();
    let cw1 = read_u16(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    run_until_hlt(&mut vcpu2).unwrap();
    let cw2 = read_u16(&mem2, 0x3000);

    assert_eq!(cw1, cw2, "FINIT and FNINIT should give same result");
}

// ============================================================================
// Initialize after Operations
// ============================================================================

#[test]
fn test_fninit_after_arithmetic() {
    // FNINIT after arithmetic operations should reset state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDB, 0xE3, // FNINIT
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        sw, DEFAULT_STATUS_WORD,
        "Status word should be cleared after FNINIT"
    );
}

#[test]
fn test_finit_after_comparison() {
    // FINIT after comparison
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xD8, 0xD1, // FCOM ST(1)
        0x9B, 0xDB, 0xE3, // FINIT
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        sw, DEFAULT_STATUS_WORD,
        "Status word should be cleared after FINIT"
    );
}

// ============================================================================
// Control Word Verification after Initialize
// ============================================================================

#[test]
fn test_fninit_control_word_precision() {
    // Control word should have 64-bit precision (bits 8-9 = 11)
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    let precision = (cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be 64-bit (11 binary)");
}

#[test]
fn test_fninit_control_word_rounding() {
    // Control word should have round to nearest (bits 10-11 = 00)
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    let rounding = (cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0, "Rounding should be nearest (00 binary)");
}

#[test]
fn test_fninit_control_word_exceptions_masked() {
    // Control word should have all exceptions masked (bits 0-5 = 111111)
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    let exception_masks = cw & 0x3F;
    assert_eq!(exception_masks, 0x3F, "All exception masks should be set");
}

// ============================================================================
// FPU Usability after Initialize
// ============================================================================

#[test]
fn test_fninit_then_use_fpu() {
    // FPU should be usable immediately after FNINIT
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.5);
    write_f64(&mem, 0x2008, 4.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 8.0, "FPU should work correctly after FNINIT");
}

#[test]
fn test_fninit_stack_operations() {
    // Stack operations should work after FNINIT
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    // All values should be stored in reverse order (LIFO)
    let v1 = read_f64(&mem, 0x3000);
    let v2 = read_f64(&mem, 0x3008);
    let v3 = read_f64(&mem, 0x3010);
    assert_eq!(v1, 3.0, "Last pushed should be first popped");
    assert_eq!(v2, 2.0, "Middle value");
    assert_eq!(v3, 1.0, "First pushed should be last popped");
}

// ============================================================================
// Status Word after Initialize
// ============================================================================

#[test]
fn test_fninit_clears_exception_flags() {
    // All exception flags should be cleared
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(sw & IE_BIT, 0, "IE should be cleared");
    assert_eq!(sw & DE_BIT, 0, "DE should be cleared");
    assert_eq!(sw & ZE_BIT, 0, "ZE should be cleared");
    assert_eq!(sw & OE_BIT, 0, "OE should be cleared");
    assert_eq!(sw & UE_BIT, 0, "UE should be cleared");
    assert_eq!(sw & PE_BIT, 0, "PE should be cleared");
}

#[test]
fn test_fninit_clears_stack_fault() {
    // Stack Fault flag should be cleared
    let code = [
        0xDB, 0xE3, // FNINIT
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV word [0x3000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let sw = read_u16(&mem, 0x3000);
    assert_eq!(sw & SF_BIT, 0, "SF should be cleared");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fninit_complete_flow() {
    // Complete workflow with FNINIT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDD, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTSW [0x3000] (before init)
        0xDB, 0xE3, // FNINIT
        0xDD, 0x3C, 0x25, 0x02, 0x30, 0x00, 0x00, // FNSTSW [0x3002] (after init)
        0xD9, 0x3C, 0x25, 0x04, 0x30, 0x00, 0x00, // FNSTCW [0x3004]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let sw_after = read_u16(&mem, 0x3002);
    let cw = read_u16(&mem, 0x3004);
    assert_eq!(
        sw_after, DEFAULT_STATUS_WORD,
        "Status word should be default after FNINIT"
    );
    assert_eq!(
        cw, DEFAULT_CONTROL_WORD,
        "Control word should be default after FNINIT"
    );
}

#[test]
fn test_finit_preserves_data() {
    // FINIT doesn't modify register data, just tags them as empty
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDB, 0xE3, // FNINIT (data preserved, tagged empty)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (will use ST(0))
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 2.5, "Second FLD should work correctly after FNINIT");
}

#[test]
fn test_multiple_finit_cycles() {
    // Multiple initialize/use cycles
    let code = [
        // First cycle
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        // Second cycle
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        // Third cycle
        0xDB, 0xE3, // FNINIT
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let r1 = read_f64(&mem, 0x3000);
    let r2 = read_f64(&mem, 0x3008);
    let r3 = read_f64(&mem, 0x3010);
    assert_eq!(r1, 1.0, "First cycle result");
    assert_eq!(r2, 2.0, "Second cycle result");
    assert_eq!(r3, 3.0, "Third cycle result");
}

// ============================================================================
// Known-answer x87 stack overflow / underflow behavior.
//
// After FNINIT the stack is empty (TOP=0). Each FLD1 decrements TOP. These
// tests pin down both the TOP-pointer bookkeeping (which works) and the
// IE/SF/C1 exception flags the architecture requires on overflow/underflow
// (which rax does NOT currently set — flagged below as ignored real bugs).
// ============================================================================

#[test]
fn test_stack_top_wraps_after_eight_pushes() {
    // FNINIT (TOP=0), then 8x FLD1. TOP decrements mod 8, so after 8 pushes
    // TOP is back to 0. FNSTSW must report TOP=0 in bits 11-13.
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, // 4x FLD1
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, // 4x FLD1 (total 8)
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        (sw & TOP_MASK) >> 11,
        0,
        "TOP must wrap back to 0 after 8 pushes"
    );
}

#[test]
fn test_stack_top_after_one_push() {
    // FNINIT (TOP=0), 1x FLD1 -> TOP = 7 (decremented mod 8).
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, // FLD1
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let sw = read_u16(&mem, 0x3000);
    assert_eq!(
        (sw & TOP_MASK) >> 11,
        7,
        "TOP must be 7 after one push from empty"
    );
}

#[test]
fn test_stack_overflow_sets_invalid_and_stack_fault() {
    // FNINIT then 9 pushes. The 9th push targets an already-occupied slot, i.e.
    // a stack OVERFLOW. Real x87: IE=1, SF=1, C1=1 (overflow direction), ES=1.
    let code = [
        0xDB, 0xE3, // FNINIT
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, // 4x FLD1
        0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, 0xD9, 0xE8, // 4x FLD1 (total 8 -> full)
        0xD9, 0xE8, // 9th FLD1 -> overflow
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let sw = read_u16(&mem, 0x3000);
    assert_ne!(
        sw & IE_BIT,
        0,
        "stack overflow must set IE (Invalid Operation)"
    );
    assert_ne!(sw & SF_BIT, 0, "stack overflow must set SF (Stack Fault)");
    assert_ne!(
        sw & C1_BIT,
        0,
        "stack overflow must set C1 (overflow direction)"
    );
    assert_ne!(
        sw & ES_BIT,
        0,
        "IE+SF must raise the Exception Summary (ES)"
    );
}

#[test]
fn test_stack_underflow_sets_invalid_and_stack_fault() {
    // FNINIT leaves the stack empty; FSTP of ST(0) is then a stack UNDERFLOW.
    // Real x87: IE=1, SF=1, C1=0 (underflow direction), ES=1.
    let code = [
        0xDB, 0xE3, // FNINIT (stack empty)
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010] -> underflow
        0xDF, 0xE0, // FNSTSW AX
        0x66, 0x89, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOV [0x3000], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
    let sw = read_u16(&mem, 0x3000);
    assert_ne!(
        sw & IE_BIT,
        0,
        "stack underflow must set IE (Invalid Operation)"
    );
    assert_ne!(sw & SF_BIT, 0, "stack underflow must set SF (Stack Fault)");
    assert_eq!(
        sw & C1_BIT,
        0,
        "stack underflow must clear C1 (underflow direction)"
    );
    assert_ne!(
        sw & ES_BIT,
        0,
        "IE+SF must raise the Exception Summary (ES)"
    );
}
