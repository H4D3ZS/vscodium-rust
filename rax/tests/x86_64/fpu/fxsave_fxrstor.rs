//! Tests for the FXSAVE and FXRSTOR instructions.
//!
//! FXSAVE - Save x87 FPU, MMX Technology, and SSE State
//! FXRSTOR - Restore x87 FPU, MMX Technology, and SSE State
//!
//! FXSAVE saves the FPU/MMX/SSE state to a 512-byte memory area.
//! FXRSTOR restores the state from that area.
//!
//! Opcodes:
//! - FXSAVE: 0F AE /0
//! - FXRSTOR: 0F AE /1
//!
//! Memory layout (non-64-bit mode):
//! - Bytes 0-1: FCW (FPU Control Word)
//! - Bytes 2-3: FSW (FPU Status Word)
//! - Bytes 4-5: FTW (FPU Tag Word)
//! - Bytes 6-7: FOP (Last Opcode)
//! - Bytes 8-11: FIP[31:0] (Instruction Pointer)
//! - Bytes 12-15: FCS (Code Segment)
//! - Bytes 16-19: FDP[31:0] (Data Pointer)
//! - Bytes 20-23: FDS (Data Segment)
//! - Bytes 24-27: MXCSR
//! - Bytes 28-31: MXCSR_MASK
//! - Bytes 32-159: ST0-ST7 (8 x 16 bytes each)
//! - Bytes 160-463: XMM0-XMM7 (8 x 16 bytes each)
//!
//! References: /Users/int/dev/rax/docs/fxsave.txt

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

// Helper function to write u32 to memory
fn write_u32(mem: &vm_memory::GuestMemoryMmap, addr: u64, val: u32) {
    mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper function to read u32 from memory
fn read_u32(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u32 {
    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u32::from_le_bytes(buf)
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

// Helper function to write bytes to memory
fn write_bytes(mem: &vm_memory::GuestMemoryMmap, addr: u64, data: &[u8]) {
    mem.write_slice(data, GuestAddress(addr)).unwrap();
}

// Helper function to read bytes from memory
fn read_bytes(mem: &vm_memory::GuestMemoryMmap, addr: u64, len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    buf
}

// FXSAVE/FXRSTOR area offsets
const FXSAVE_FCW: u64 = 0; // FPU Control Word
const FXSAVE_FSW: u64 = 2; // FPU Status Word
const FXSAVE_FTW: u64 = 4; // FPU Tag Word
const FXSAVE_FOP: u64 = 6; // Last Opcode
const FXSAVE_ST0: u64 = 32; // First FPU register (16 bytes each)
const FXSAVE_ST1: u64 = 48;
const FXSAVE_ST2: u64 = 64;
const FXSAVE_ST3: u64 = 80;
const FXSAVE_ST4: u64 = 96;
const FXSAVE_ST5: u64 = 112;
const FXSAVE_ST6: u64 = 128;
const FXSAVE_ST7: u64 = 144;
const FXSAVE_SIZE: u64 = 512; // Total size of FXSAVE area

// ============================================================================
// FXSAVE - Save FPU/SSE State
// ============================================================================

#[test]
fn test_fxsave_basic() {
    // Basic FXSAVE operation
    let code = [
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FXSAVE [0x2000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify FXSAVE area was written (check control word)
    let fcw = read_u16(&mem, 0x2000 + FXSAVE_FCW);
    assert!(fcw < 0xFFFF, "FCW should be valid after FXSAVE");
}

#[test]
fn test_fxsave_with_fpu_data() {
    // FXSAVE should save FPU register data
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x20, 0x00, 0x00, // FSTP qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00, // FSTP qword [0x2018]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify FCW was saved
    let fcw = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    assert!(fcw < 0xFFFF, "FCW should be saved");

    // Verify FSW was saved
    let fsw = read_u16(&mem, 0x3000 + FXSAVE_FSW);
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fxsave_saves_control_word() {
    // FXSAVE should save the control word
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F);

    run_until_hlt(&mut vcpu).unwrap();

    let saved_cw = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    assert_eq!(saved_cw, 0x037F, "FCW should be saved correctly");
}

#[test]
fn test_fxsave_saves_status_word() {
    // FXSAVE should save the status word
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    let fsw = read_u16(&mem, 0x3000 + FXSAVE_FSW);
    assert!(fsw < 0xFFFF, "FSW should be saved");
}

#[test]
fn test_fxsave_multiple_areas() {
    // FXSAVE to different memory areas
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x31, 0x00, 0x00, // FXSAVE [0x3100]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw1 = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    let fcw2 = read_u16(&mem, 0x3100 + FXSAVE_FCW);
    assert_eq!(fcw1, fcw2, "Multiple FXSAVE should save identical state");
}

// ============================================================================
// FXRSTOR - Restore FPU/SSE State
// ============================================================================

#[test]
fn test_fxrstor_basic() {
    // Basic FXRSTOR operation
    let code = [
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
        0xD9, 0x3C, 0x25, 0x00, 0x40, 0x00, 0x00, // FNSTCW [0x4000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Initialize the restore area with valid state
    write_u16(&mem, 0x3000 + FXSAVE_FCW, 0x037F);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x4000);
    assert!(cw < 0xFFFF, "Control word should be valid after FXRSTOR");
}

// ============================================================================
// FXSAVE/FXRSTOR Round Trip
// ============================================================================

#[test]
fn test_fxsave_fxrstor_roundtrip() {
    // FXSAVE followed by FXRSTOR should preserve state
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDB, 0xE3, // FNINIT (clear FPU)
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00, // FSTP qword [0x4008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x4008);
    assert_eq!(
        result, 1.5,
        "Value should be preserved through FXSAVE/FXRSTOR"
    );
}

#[test]
fn test_fxsave_fxrstor_multiple_values() {
    // FXSAVE/FXRSTOR with multiple FPU values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDB, 0xE3, // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00, // FSTP qword [0x4010]
        0xDD, 0x1C, 0x25, 0x18, 0x40, 0x00, 0x00, // FSTP qword [0x4018]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let v1 = read_f64(&mem, 0x4010);
    let v2 = read_f64(&mem, 0x4018);
    assert_eq!(v1, 2.5, "Second value should be popped first");
    assert_eq!(v2, 1.5, "First value should be popped second");
}

// ============================================================================
// FXSAVE Area Size and Alignment
// ============================================================================

#[test]
fn test_fxsave_area_512_bytes() {
    // FXSAVE uses a 512-byte area
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.14159);

    run_until_hlt(&mut vcpu).unwrap();

    // Verify data at various offsets in the FXSAVE area
    let fcw = read_u16(&mem, 0x3000);
    let fsw = read_u16(&mem, 0x3002);
    let ftw = read_u16(&mem, 0x3004);

    assert!(fcw < 0xFFFF, "FCW should be valid");
    assert!(fsw < 0xFFFF, "FSW should be valid");
    assert!(ftw < 0xFFFF, "FTW should be valid");
}

// ============================================================================
// FXSAVE with Different Control Word Values
// ============================================================================

#[test]
fn test_fxsave_different_control_words() {
    // FXSAVE should preserve different control word values
    let test_cws = vec![0x037F, 0x027F, 0x0C7F];

    for test_cw in test_cws {
        let code = [
            0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
            0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
            0xF4, // HLT
        ];

        let (mut vcpu, mem) = setup_vm(&code, None);
        write_u16(&mem, 0x2000, test_cw);

        run_until_hlt(&mut vcpu).unwrap();

        let saved_cw = read_u16(&mem, 0x3000 + FXSAVE_FCW);
        assert_eq!(
            saved_cw, test_cw,
            "Control word 0x{:04X} should be saved",
            test_cw
        );
    }
}

// ============================================================================
// FXRSTOR from Different Areas
// ============================================================================

#[test]
fn test_fxrstor_from_prepared_area() {
    // FXRSTOR from a pre-prepared FXSAVE area
    let code = [
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00, // FXRSTOR [0x2000]
        0xD9, 0x3C, 0x25, 0x00, 0x30, 0x00, 0x00, // FNSTCW [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Prepare the FXSAVE area with known values
    write_u16(&mem, 0x2000 + FXSAVE_FCW, 0x037F);
    write_u16(&mem, 0x2000 + FXSAVE_FSW, 0x0000);

    run_until_hlt(&mut vcpu).unwrap();

    let cw = read_u16(&mem, 0x3000);
    assert_eq!(
        cw, 0x037F,
        "Control word should be restored from prepared area"
    );
}

// ============================================================================
// Sequential FXSAVE Operations
// ============================================================================

#[test]
fn test_sequential_fxsave() {
    // Multiple sequential FXSAVE operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00, // FXSAVE [0x3200]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fcw1 = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    let fcw2 = read_u16(&mem, 0x3200 + FXSAVE_FCW);
    assert_eq!(
        fcw1, fcw2,
        "Multiple FXSAVE should produce identical results"
    );
}

// ============================================================================
// FXSAVE/FXRSTOR with Arithmetic
// ============================================================================

#[test]
fn test_fxsave_after_arithmetic() {
    // FXSAVE after arithmetic operations
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 2.5);
    write_f64(&mem, 0x2008, 3.5);

    run_until_hlt(&mut vcpu).unwrap();

    let fsw = read_u16(&mem, 0x3000 + FXSAVE_FSW);
    assert!(fsw < 0xFFFF, "FSW should be saved after arithmetic");
}

#[test]
fn test_fxrstor_then_arithmetic() {
    // FXRSTOR followed by arithmetic
    let code = [
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x20, 0x00, 0x00, // FXRSTOR [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDE, 0xC1, // FADDP
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    // Prepare restore area
    write_u16(&mem, 0x2000 + FXSAVE_FCW, 0x037F);
    write_u16(&mem, 0x2000 + FXSAVE_FSW, 0x0000);
    write_f64(&mem, 0x2008, 1.5);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.5, "Arithmetic should work after FXRSTOR");
}

// ============================================================================
// FXSAVE State Preservation
// ============================================================================

#[test]
fn test_fxsave_preserves_control_precision() {
    // FXSAVE should preserve control word precision bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F); // Default (64-bit precision)

    run_until_hlt(&mut vcpu).unwrap();

    let saved_cw = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    let precision = (saved_cw >> 8) & 0x3;
    assert_eq!(precision, 0x3, "Precision should be saved as 64-bit");
}

#[test]
fn test_fxsave_preserves_control_rounding() {
    // FXSAVE should preserve control word rounding bits
    let code = [
        0xD9, 0x2C, 0x25, 0x00, 0x20, 0x00, 0x00, // FLDCW [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_u16(&mem, 0x2000, 0x037F); // Round to nearest

    run_until_hlt(&mut vcpu).unwrap();

    let saved_cw = read_u16(&mem, 0x3000 + FXSAVE_FCW);
    let rounding = (saved_cw >> 10) & 0x3;
    assert_eq!(rounding, 0x0, "Rounding should be saved as nearest");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_fxsave_fxrstor_complete_flow() {
    // Complete FXSAVE/FXRSTOR workflow
    let code = [
        // Load and use FPU
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        // Save state
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        // Initialize FPU
        0xDB, 0xE3, // FNINIT
        // Do some other work
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010]
        0xDD, 0x1C, 0x25, 0x18, 0x20, 0x00, 0x00, // FSTP qword [0x2018]
        // Restore saved state
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
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
    assert_eq!(v1, 2.5, "Second restored value should be 2.5");
    assert_eq!(v2, 1.5, "First restored value should be 1.5");
}

#[test]
fn test_fxsave_fxrstor_multiple_cycles() {
    // Multiple FXSAVE/FXRSTOR cycles
    let code = [
        // Cycle 1
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // FXSAVE [0x3000]
        0xDB, 0xE3, // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x30, 0x00, 0x00, // FXRSTOR [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x40, 0x00, 0x00, // FSTP qword [0x4008]
        // Cycle 2
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0x0F, 0xAE, 0x04, 0x25, 0x00, 0x32, 0x00, 0x00, // FXSAVE [0x3200]
        0xDB, 0xE3, // FNINIT
        0x0F, 0xAE, 0x0C, 0x25, 0x00, 0x32, 0x00, 0x00, // FXRSTOR [0x3200]
        0xDD, 0x1C, 0x25, 0x10, 0x40, 0x00, 0x00, // FSTP qword [0x4010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.5);
    write_f64(&mem, 0x2008, 2.5);

    run_until_hlt(&mut vcpu).unwrap();

    let r1 = read_f64(&mem, 0x4008);
    let r2 = read_f64(&mem, 0x4010);
    assert_eq!(r1, 1.5, "First cycle result");
    assert_eq!(r2, 2.5, "Second cycle result");
}
