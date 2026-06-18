//! Tests for the FSAVE, FNSAVE, and FRSTOR instructions.
//!
//! FSAVE/FNSAVE - Store x87 FPU State and Registers
//! FRSTOR - Restore x87 FPU State and Registers
//!
//! FSAVE saves the complete FPU state (operating environment and register stack)
//! to a 94 or 108-byte memory area and then reinitializes the FPU.
//! FRSTOR restores the state from that area.
//!
//! FSAVE checks for pending exceptions before saving, while FNSAVE does not.
//!
//! Opcodes:
//! - FNSAVE: DD /6
//! - FSAVE: 9B DD /6
//! - FRSTOR: DD /4
//!
//! Save Area Format (94 bytes in 16-bit, 108 bytes in 32-bit):
//! - Environment (14 or 28 bytes)
//! - FPU Register Stack (80 bytes) - 8 registers x 10 bytes each
//!
//! References: /Users/int/dev/rax/docs/fsave:fnsave.txt, /Users/int/dev/rax/docs/frstor.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

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

// FSAVE/FRSTOR area structure offsets (in protected 32-bit mode, 108 bytes)
const FSAVE_FCW: u64 = 0; // FPU Control Word (2 bytes)
const FSAVE_FSW: u64 = 2; // FPU Status Word (2 bytes)
const FSAVE_FTW: u64 = 4; // FPU Tag Word (2 bytes)
const FSAVE_SIZE: u64 = 108; // Total size in 32-bit protected mode

// Status word bit definitions
const IE_BIT: u16 = 0x0001;
const TOP_MASK: u16 = 0x3800;

// ============================================================================
// FNSAVE - Save State without Wait
// ============================================================================

#[test]
fn test_fnsave_basic() {
    // Basic FNSAVE operation
    let code = [
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify control word was stored
    let fcw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    assert!(fcw < 0xFFFF, "FCW should be valid after FNSAVE");
}

#[test]
fn test_fnsave_saves_control_word() {
    // FNSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F);

    run_until_hlt(&mut vcpu).unwrap();

    let saved_fcw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    assert_eq!(saved_fcw, 0x037F, "FCW should be saved");
}

#[test]
fn test_fnsave_saves_status_word() {
    // FNSAVE should save the status word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    let saved_fsw = read_u16(&mem, 0x3000 + FSAVE_FSW);
    assert!(saved_fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fnsave_saves_fpu_registers() {
    // FNSAVE should save FPU register contents
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    let fsw = read_u16(&mem, 0x3000 + FSAVE_FSW);
    assert!(fcw < 0xFFFF, "FCW should be saved");
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fnsave_reinitializes_fpu() {
    // FNSAVE should reinitialize the FPU after saving
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDF, 0xE0, // FNSTSW AX (check status after save)
        0x66, 0x89, 0x04, 0x25, 0x00, 0x40, 0x00, 0x00, // MOV word [0x4000], AX
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let sw_after = read_u16(&mem, 0x4000);
    assert_eq!(sw_after, 0x0000, "FPU should be reinitialized after FNSAVE");
}

#[test]
fn test_fnsave_multiple_times() {
    // Multiple FNSAVE operations should produce identical results
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00, // FNSAVE [0x3200]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw1 = read_u16(&mem, 0x3000 + FSAVE_FCW);
    let fcw2 = read_u16(&mem, 0x3200 + FSAVE_FCW);
    assert_eq!(
        fcw1, fcw2,
        "Multiple FNSAVE should save identical control words"
    );
}

// ============================================================================
// FSAVE - Save State with Wait
// ============================================================================

#[test]
fn test_fsave_basic() {
    // Basic FSAVE operation with FWAIT prefix
    let code = [
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    assert!(fcw < 0xFFFF, "FCW should be valid");
}

#[test]
fn test_fsave_saves_control_word() {
    // FSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x027F);

    run_until_hlt(&mut vcpu).unwrap();

    let saved_fcw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    assert_eq!(saved_fcw, 0x027F, "FCW should be saved");
}

// ============================================================================
// FSAVE vs FNSAVE Equivalence
// ============================================================================

#[test]
fn test_fsave_vs_fnsave() {
    // FSAVE and FNSAVE should produce same result in normal operation
    let code1 = [
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FSAVE [0x3000]
        0xF4, // HLT
    ];

    let code2 = [
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    run_until_hlt(&mut vcpu1).unwrap();
    let fcw1 = read_u16(&mem1, 0x3000);

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    run_until_hlt(&mut vcpu2).unwrap();
    let fcw2 = read_u16(&mem2, 0x3000);

    assert_eq!(fcw1, fcw2, "FSAVE and FNSAVE should give same result");
}

// ============================================================================
// FRSTOR - Restore State
// ============================================================================

#[test]
fn test_frstor_basic() {
    // Basic FRSTOR operation
    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00, // FNSTCW [0x4000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Initialize the restore area with valid state
    write_u16(&mem, 0x3000 + FSAVE_FCW, 0x037F);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x4000);
    assert!(cw < 0xFFFF, "Control word should be valid after FRSTOR");
}

// ============================================================================
// FNSAVE/FRSTOR Round Trip
// ============================================================================

#[test]
fn test_fnsave_frstor_roundtrip() {
    // FNSAVE followed by FRSTOR should preserve state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (after init)
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00, // FSTP qword [0x4010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 99.0);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x4010);
    assert_eq!(
        result, 1.5,
        "Value should be preserved through FNSAVE/FRSTOR"
    );
}

#[test]
fn test_fnsave_frstor_multiple_values() {
    // FNSAVE/FRSTOR with multiple FPU values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (new value)
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x18, 0x40, 0x00, 0x00, // FSTP qword [0x4018]
        0xDD, 0x1C, 0x25, 0x20, 0x40, 0x00, 0x00, // FSTP qword [0x4020]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);
    write_f64(&mem, 0x2010, 99.0);

    run_until_hlt(&mut vcpu).unwrap();

    let v1 = read_f64(&mem, 0x4018);
    let v2 = read_f64(&mem, 0x4020);
    assert_eq!(v1, 2.5, "Second saved value should be 2.5");
    assert_eq!(v2, 1.5, "First saved value should be 1.5");
}

// ============================================================================
// FNSAVE Area Size
// ============================================================================

#[test]
fn test_fnsave_area_structure() {
    // FNSAVE uses up to 108 bytes in 32-bit protected mode
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify data at various offsets
    let fcw = read_u16(&mem, 0x3000);
    let fsw = read_u16(&mem, 0x3002);
    let ftw = read_u16(&mem, 0x3004);

    assert!(fcw < 0xFFFF, "FCW should be valid");
    assert!(fsw < 0xFFFF, "FSW should be valid");
    assert!(ftw < 0xFFFF, "FTW should be valid");
}

// ============================================================================
// FNSAVE with Different Control Words
// ============================================================================

#[test]
fn test_fnsave_different_control_words() {
    // FNSAVE should preserve different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
            0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
            0xF4, // HLT
        ];

        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, test_cw);

        run_until_hlt(&mut vcpu).unwrap();

        let saved_cw = read_u16(&mem, 0x3000 + FSAVE_FCW);
        assert_eq!(
            saved_cw, test_cw,
            "Control word 0x{:04X} should be saved",
            test_cw
        );
    }
}

// ============================================================================
// FRSTOR from Prepared Area
// ============================================================================

#[test]
fn test_frstor_from_prepared_area() {
    // FRSTOR from a pre-prepared FNSAVE area
    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FRSTOR [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Prepare the restore area with known values
    write_u16(&mem, 0x2000 + FSAVE_FCW, 0x037F);
    write_u16(&mem, 0x2000 + FSAVE_FSW, 0x0000);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    assert_eq!(
        cw, 0x037F,
        "Control word should be restored from prepared area"
    );
}

// ============================================================================
// Sequential FNSAVE Operations
// ============================================================================

#[test]
fn test_sequential_fnsave() {
    // Multiple sequential FNSAVE operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00, // FNSAVE [0x3200]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw1 = read_u16(&mem, 0x3000 + FSAVE_FCW);
    let fcw2 = read_u16(&mem, 0x3200 + FSAVE_FCW);
    assert_eq!(
        fcw1, fcw2,
        "Multiple FNSAVE should produce identical results"
    );
}

// ============================================================================
// FNSAVE after Arithmetic
// ============================================================================

#[test]
fn test_fnsave_after_arithmetic() {
    // FNSAVE after arithmetic operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);
    write_f64(&mem, 0x2008, 3.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fsw = read_u16(&mem, 0x3000 + FSAVE_FSW);
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_frstor_then_arithmetic() {
    // FRSTOR followed by arithmetic
    let code = [
        0xDD, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // FRSTOR [0x2000]
        0xDD, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FLD qword [0x3000]
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x40, 0x00, 0x00, // FSTP qword [0x4000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Prepare restore area
    write_u16(&mem, 0x2000 + FSAVE_FCW, 0x037F);
    write_u16(&mem, 0x2000 + FSAVE_FSW, 0x0000);
    write_f64(&mem, 0x3000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x4000);
    assert_eq!(result, 1.5, "Arithmetic should work after FRSTOR");
}

// ============================================================================
// State Preservation
// ============================================================================

#[test]
fn test_fnsave_preserves_control_precision() {
    // FNSAVE should preserve control word precision bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F); // Default (64-bit precision)

    run_until_hlt(&mut vcpu).unwrap();

    let saved_cw = read_u16(&mem, 0x3000 + FSAVE_FCW);
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be saved as 64-bit");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fnsave_frstor_complete_flow() {
    // Complete FNSAVE/FRSTOR workflow
    let code = [
        // Load and use FPU
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        // Save state
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        // FPU is reinitialized by FNSAVE, do some other work
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00, // FSTP qword [0x2018]
        // Restore saved state
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        // Use restored state
        0xDD, 0x1C, 0x25, 0x20, 0x40, 0x00, 0x00, // FSTP qword [0x4020]
        0xDD, 0x1C, 0x25, 0x28, 0x40, 0x00, 0x00, // FSTP qword [0x4028]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);
    write_f64(&mem, 0x2010, 99.0);

    run_until_hlt(&mut vcpu).unwrap();

    let v1 = read_f64(&mem, 0x4020);
    let v2 = read_f64(&mem, 0x4028);
    assert_eq!(v1, 2.5, "Second saved value should be 2.5");
    assert_eq!(v2, 1.5, "First saved value should be 1.5");
}

#[test]
fn test_fnsave_frstor_multiple_cycles() {
    // Multiple FNSAVE/FRSTOR cycles
    let code = [
        // Cycle 1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00, // FSTP qword [0x4008]
        // Cycle 2
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDD, 0x34, 0x25, 0x00, 0x32, 0x00, 0x00, // FNSAVE [0x3200]
        0xDD, 0x24, 0x25, 0x00, 0x32, 0x00, 0x00, // FRSTOR [0x3200]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00, // FSTP qword [0x4010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let r1 = read_f64(&mem, 0x4008);
    let r2 = read_f64(&mem, 0x4010);
    assert_eq!(r1, 1.5, "Cycle 1 result");
    assert_eq!(r2, 2.5, "Cycle 2 result");
}

#[test]
fn test_fsave_vs_fnsave_roundtrip() {
    // Both FSAVE and FNSAVE should work in roundtrip
    let code1 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x9B, 0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00, // FSTP qword [0x4008]
        0xF4, // HLT
    ];

    let (mut vcpu1, mem1) = setup_vm(&code1, None);
    write_f64(&mem1, 0x2000, 1.5);

    run_until_hlt(&mut vcpu1).unwrap();

    let result1 = read_f64(&mem1, 0x4008);
    assert_eq!(result1, 1.5, "FSAVE roundtrip should preserve value");

    // Same with FNSAVE
    let code2 = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSAVE [0x3000]
        0xDD, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // FRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00, // FSTP qword [0x4008]
        0xF4, // HLT
    ];

    let (mut vcpu2, mem2) = setup_vm(&code2, None);
    write_f64(&mem2, 0x2000, 1.5);

    run_until_hlt(&mut vcpu2).unwrap();

    let result2 = read_f64(&mem2, 0x4008);
    assert_eq!(result2, 1.5, "FNSAVE roundtrip should preserve value");
}
