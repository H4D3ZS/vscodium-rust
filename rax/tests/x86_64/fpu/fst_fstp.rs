//! Tests for the FST and FSTP instructions.
//!
//! FST - Store Floating-Point Value (without pop)
//! FSTP - Store Floating-Point Value and Pop
//!
//! The FST instruction copies the value in the ST(0) register to the destination operand,
//! which can be a memory location or another register in the FPU register stack.
//!
//! The FSTP instruction performs the same operation as the FST instruction and then pops
//! the register stack. The FSTP instruction can also store values in memory in double
//! extended-precision floating-point format.
//!
//! Reference: /Users/int/dev/rax/docs/fst:fstp.txt

use crate::common::*;
use std::sync::Arc;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

const DATA_ADDR: u64 = 0x2000;

// Helper to write f64 to memory
fn write_f64(mem: &Arc<GuestMemoryMmap>, addr: u64, value: f64) {
    mem.write_slice(&value.to_le_bytes(), GuestAddress(addr))
        .unwrap();
}

// Helper to read f32 from memory
fn read_f32(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f32 {
    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f32::from_le_bytes(buf)
}

// Helper to read f64 from memory
fn read_f64(mem: &Arc<GuestMemoryMmap>, addr: u64) -> f64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    f64::from_le_bytes(buf)
}

// ============================================================================
// FST m32fp (opcode D9 /2) - Store 32-bit float without pop
// ============================================================================

#[test]
fn test_fst_m32fp_positive_one() {
    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST dword ptr [0x3000]  ; Store as f32
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fst_m32fp_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fst_m32fp_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fst_m32fp_negative_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert_eq!(result, -1.0);
}

#[test]
fn test_fst_m32fp_infinity_positive() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_positive());
}

#[test]
fn test_fst_m32fp_infinity_negative() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NEG_INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result.is_infinite() && result.is_sign_negative());
}

#[test]
fn test_fst_m32fp_nan() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result.is_nan());
}

#[test]
fn test_fst_m32fp_no_pop() {
    // Verify FST does not pop the stack
    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST dword ptr [0x3000]  ; Store (no pop)
    // FST dword ptr [0x3004]  ; Store again (should still be 1.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xD9, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // FST dword ptr [0x3004]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f32(&mem, 0x3000), 1.0);
    assert_eq!(read_f32(&mem, 0x3004), 1.0);
}

// ============================================================================
// FST m64fp (opcode DD /2) - Store 64-bit float without pop
// ============================================================================

#[test]
fn test_fst_m64fp_positive_one() {
    // FLD qword ptr [0x2000]  ; Load 1.0
    // FST qword ptr [0x3000]  ; Store as f64
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fst_m64fp_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fst_m64fp_negative_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert!(result.is_sign_negative() && result == 0.0);
}

#[test]
fn test_fst_m64fp_pi() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, std::f64::consts::PI);
}

#[test]
fn test_fst_m64fp_no_pop() {
    // Verify FST does not pop the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, std::f64::consts::E);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), std::f64::consts::E);
    assert_eq!(read_f64(&mem, 0x3008), std::f64::consts::E);
}

// ============================================================================
// FSTP m32fp (opcode D9 /3) - Store 32-bit float and pop
// ============================================================================

#[test]
fn test_fstp_m32fp_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fstp_m32fp_large_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 123456.78);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!((result - 123456.78).abs() < 0.01);
}

#[test]
fn test_fstp_m32fp_with_pop() {
    // Verify FSTP pops the stack
    // FLD qword ptr [0x2000]  ; Load 1.0
    // FLD qword ptr [0x2008]  ; Load 2.0 (now ST(0))
    // FSTP dword ptr [0x3000] ; Store 2.0 and pop
    // FSTP dword ptr [0x3004] ; Store 1.0 and pop
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xD9, 0x1C, 0x25, 0x04, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3004]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f32(&mem, 0x3000), 2.0);
    assert_eq!(read_f32(&mem, 0x3004), 1.0);
}

// ============================================================================
// FSTP m64fp (opcode DD /3) - Store 64-bit float and pop
// ============================================================================

#[test]
fn test_fstp_m64fp_positive_one() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 1.0);
}

#[test]
fn test_fstp_m64fp_zero() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, 0.0);
}

#[test]
fn test_fstp_m64fp_negative_value() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, -12345.6789);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f64(&mem, 0x3000);
    assert_eq!(result, -12345.6789);
}

#[test]
fn test_fstp_m64fp_with_pop() {
    // Verify FSTP pops the stack
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
    assert_eq!(read_f64(&mem, 0x3008), 2.0);
    assert_eq!(read_f64(&mem, 0x3010), 1.0);
}

// ============================================================================
// FST ST(i) (opcode DD D0+i) - Store to register without pop
// ============================================================================

#[test]
fn test_fst_st1() {
    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD qword ptr [0x2008]  ; Load 2.0 into ST(0), 1.0 -> ST(1)
    // FST ST(1)               ; Copy ST(0) to ST(1) (both should be 2.0)
    // FSTP qword ptr [0x3000] ; Pop ST(0) (2.0)
    // FSTP qword ptr [0x3008] ; Pop ST(0) (was ST(1), now 2.0)
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0xD1, // FST ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0);
    assert_eq!(read_f64(&mem, 0x3008), 2.0);
}

#[test]
fn test_fst_st2() {
    // Test FST to ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xDD, 0xD2, // FST ST(2) ; Copy 3.0 to ST(2)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000] ; 3.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; 2.0
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010] ; 3.0 (was ST(2))
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 3.0);
    assert_eq!(read_f64(&mem, 0x3008), 2.0);
    assert_eq!(read_f64(&mem, 0x3010), 3.0);
}

// ============================================================================
// FSTP ST(i) (opcode DD D8+i) - Store to register and pop
// ============================================================================

#[test]
fn test_fstp_st1() {
    // FLD qword ptr [0x2000]  ; Load 1.0 into ST(0)
    // FLD qword ptr [0x2008]  ; Load 2.0 into ST(0), 1.0 -> ST(1)
    // FSTP ST(1)              ; Copy ST(0) to ST(1) and pop
    // FSTP qword ptr [0x3000] ; Pop remaining value
    // HLT
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0xD9, // FSTP ST(1)
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0);
}

#[test]
fn test_fstp_st2() {
    // Test FSTP to ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010] ; 3.0
        0xDD, 0xDA, // FSTP ST(2) ; Copy 3.0 to ST(2) and pop
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000] ; 2.0
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; 3.0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);
    write_f64(&mem, DATA_ADDR + 16, 3.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 2.0);
    assert_eq!(read_f64(&mem, 0x3008), 3.0);
}

// ============================================================================
// Precision and rounding tests
// ============================================================================

#[test]
fn test_fst_m32fp_precision_loss() {
    // Test that storing to f32 loses precision
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // This value has more precision than f32 can represent
    write_f64(&mem, DATA_ADDR, 1.2345678901234567);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    // f32 should round/truncate the value
    assert!((result - 1.234568).abs() < 0.0001);
}

#[test]
fn test_fstp_m32fp_very_small() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e-40);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result > 0.0 && result < 1.0e-38);
}

#[test]
fn test_fstp_m32fp_very_large() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword ptr [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0e38);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_f32(&mem, 0x3000);
    assert!(result > 1.0e37);
}

// ============================================================================
// Mixed format tests
// ============================================================================

#[test]
fn test_mixed_fst_fstp() {
    // Test mixing FST (no pop) and FSTP (with pop)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000] ; 1.0
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008] ; 2.0
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00,
        0x00, // FST dword ptr [0x3000]  ; Store 2.0, no pop
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008] ; Store 2.0, pop
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010] ; Store 1.0, pop
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0);
    write_f64(&mem, DATA_ADDR + 8, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f32(&mem, 0x3000), 2.0);
    assert_eq!(read_f64(&mem, 0x3008), 2.0);
    assert_eq!(read_f64(&mem, 0x3010), 1.0);
}

#[test]
fn test_fst_multiple_formats() {
    // Store same value in different formats
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xD9, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST dword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, std::f64::consts::PI);

    run_until_hlt(&mut vcpu).unwrap();
    let result_f32 = read_f32(&mem, 0x3000);
    let result_f64 = read_f64(&mem, 0x3008);
    assert!((result_f32 as f64 - std::f64::consts::PI).abs() < 1e-6);
    assert_eq!(result_f64, std::f64::consts::PI);
}

// ============================================================================
// Special value tests
// ============================================================================

#[test]
fn test_fstp_special_values_sequence() {
    // Test sequence of special values
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword ptr [0x2008]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword ptr [0x2010]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3010]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 0.0);
    write_f64(&mem, DATA_ADDR + 8, -0.0);
    write_f64(&mem, DATA_ADDR + 16, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();
    let r1 = read_f64(&mem, 0x3000);
    let r2 = read_f64(&mem, 0x3008);
    let r3 = read_f64(&mem, 0x3010);
    assert!(r1.is_infinite() && r1.is_sign_positive());
    assert!(r2.is_sign_negative() && r2 == 0.0);
    assert_eq!(r3, 0.0);
}

#[test]
fn test_fst_preserves_nan() {
    // Test that NaN is preserved through FST
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword ptr [0x3000]
        0xDD, 0x14, 0x25, 0x08, 0x30, 0x00, 0x00, // FST qword ptr [0x3008]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::NAN);

    run_until_hlt(&mut vcpu).unwrap();
    assert!(read_f64(&mem, 0x3000).is_nan());
    assert!(read_f64(&mem, 0x3008).is_nan());
}

#[test]
fn test_fstp_extreme_values() {
    // Test f64 MIN and MAX
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword ptr [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword ptr [0x3000]
        0xf4,
    ];

    // Test MAX
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::MAX);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), f64::MAX);

    // Test MIN_POSITIVE
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, f64::MIN_POSITIVE);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), f64::MIN_POSITIVE);
}

// ============================================================================
// Known-answer FLD/FST/FSTP round-trip tests with EXACT bit-for-bit checks.
// ============================================================================

#[test]
fn test_fld_fstp_m64_roundtrip_exact_bits() {
    // FLD [m64] then FSTP [m64] must preserve the exact bit pattern.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xf4,
    ];
    for v in [1.0_f64, -2.5, 1234.5, std::f64::consts::PI, 0.1, -0.0] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        write_f64(&mem, DATA_ADDR, v);
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            read_f64(&mem, 0x3000).to_bits(),
            v.to_bits(),
            "FLD/FSTP m64 must be bit-exact for {v}"
        );
    }
}

#[test]
fn test_fld_m32_fstp_m32_roundtrip_exact() {
    // FLD m32 widens f32->f64, FSTP m32 narrows back; for f32-representable
    // values the round-trip is exact.
    let code = [
        0xD9, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD dword [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword [0x3000]
        0xf4,
    ];
    for v in [1.0_f32, -2.5, 0.5, 256.0, 0.25] {
        let (mut vcpu, mem) = setup_vm(&code, None);
        mem.write_slice(&v.to_le_bytes(), GuestAddress(DATA_ADDR))
            .unwrap();
        run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            read_f32(&mem, 0x3000).to_bits(),
            v.to_bits(),
            "FLD/FSTP m32 exact for {v}"
        );
    }
}

#[test]
fn test_fst_m64_does_not_pop() {
    // FST (no pop) leaves ST(0) intact; a following FSTP reads the same value.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // FST qword [0x3000] (no pop)
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 9.75);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f64(&mem, 0x3000), 9.75);
    assert_eq!(read_f64(&mem, 0x3008), 9.75, "FST must not pop ST(0)");
}

#[test]
fn test_fld_fst_m32_downconvert_known() {
    // FLD m64 of 1.0/3.0, then FST m32 rounds to nearest f32.
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xD9, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP dword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, DATA_ADDR, 1.0_f64 / 3.0);
    run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_f32(&mem, 0x3000), (1.0_f64 / 3.0) as f32);
}
