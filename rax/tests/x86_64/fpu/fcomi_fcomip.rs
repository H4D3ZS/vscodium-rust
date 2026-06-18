//! Tests for the FCOMI and FCOMIP instructions.
//!
//! FCOMI/FCOMIP - Ordered Compare and Set EFLAGS
//!
//! FCOMI performs an ordered comparison of ST(0) with ST(i) and sets
//! the ZF, PF, and CF flags in the EFLAGS register according to the result.
//! Unlike FUCOMI, it generates an exception for any NaN operand (QNaN or SNaN).
//!
//! FCOMIP performs the same comparison, sets the EFLAGS, and then pops
//! the FPU stack.
//!
//! Comparison Results (EFLAGS):
//! - ST(0) > SRC: ZF=0, PF=0, CF=0
//! - ST(0) < SRC: ZF=0, PF=0, CF=1
//! - ST(0) = SRC: ZF=1, PF=0, CF=0
//! - Unordered:   ZF=1, PF=1, CF=1 (NaN operand, with exception)
//!
//! Opcodes:
//! - FCOMI: DB F0+i
//! - FCOMIP: DF F0+i
//!
//! Flags affected: ZF, PF, CF
//! Flags cleared: OF, SF, AF
//!
//! Reference: /Users/int/dev/rax/docs/fcomi:fcomip:fcomi:fcomip.txt

use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

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

// Helper function to read u64 from memory
fn read_u64(mem: &vm_memory::GuestMemoryMmap, addr: u64) -> u64 {
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(addr)).unwrap();
    u64::from_le_bytes(buf)
}

// EFLAGS bit positions
const CF_BIT: u64 = 1 << 0;
const PF_BIT: u64 = 1 << 2;
const ZF_BIT: u64 = 1 << 6;

// ============================================================================
// FCOMI - Ordered Compare and Set EFLAGS
// ============================================================================

#[test]
fn test_fcomi_equal() {
    // Compare 5.0 with 5.0 (equal) -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val1 = read_f64(&mem, 0x3008);
    let val2 = read_f64(&mem, 0x3010);
    assert_eq!(val1, 5.0);
    assert_eq!(val2, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for equal");
}

#[test]
fn test_fcomi_greater_than() {
    // Compare 10.0 > 5.0 -> ZF=0, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (10.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val1 = read_f64(&mem, 0x3008);
    let val2 = read_f64(&mem, 0x3010);
    assert_eq!(val1, 10.0);
    assert_eq!(val2, 5.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for greater");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for greater");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for greater");
}

#[test]
fn test_fcomi_less_than() {
    // Compare 3.0 < 7.0 -> ZF=0, PF=0, CF=1
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (7.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (3.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 7.0);
    write_f64(&mem, 0x2008, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val1 = read_f64(&mem, 0x3008);
    let val2 = read_f64(&mem, 0x3010);
    assert_eq!(val1, 3.0);
    assert_eq!(val2, 7.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for less");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for less");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for less");
}

#[test]
fn test_fcomi_st2() {
    // FCOMI with ST(2)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xDB, 0xF2, // FCOMI ST(2)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xDD, 0x1C, 0x25, 0x18, 0x30, 0x00, 0x00, // FSTP qword [0x3018]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (3.0 > 1.0)");
}

#[test]
fn test_fcomi_st3() {
    // FCOMI with ST(3)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xDD, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // FLD qword [0x2018] (4.0)
        0xDB, 0xF3, // FCOMI ST(3)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);
    write_f64(&mem, 0x2018, 4.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (4.0 > 1.0)");
}

#[test]
fn test_fcomi_infinity_greater() {
    // +infinity > finite -> ZF=0, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (100.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 100.0);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear");
}

#[test]
fn test_fcomi_infinities_equal() {
    // +inf == +inf -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (+inf)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, f64::INFINITY);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
    assert_eq!(flags & PF_BIT, 0, "PF should be clear for equal");
}

#[test]
fn test_fcomi_positive_negative_zero() {
    // +0.0 == -0.0 -> ZF=1, PF=0, CF=0
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-0.0)
        0xD9, 0xEE, // FLDZ (+0.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -0.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
}

// ============================================================================
// FCOMIP - Ordered Compare, Set EFLAGS, and Pop
// ============================================================================

#[test]
fn test_fcomip_equal() {
    // Compare and pop
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val = read_f64(&mem, 0x3008);
    assert_eq!(val, 5.0);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for equal");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for equal");
}

#[test]
fn test_fcomip_greater() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (3.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (8.0)
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 3.0);
    write_f64(&mem, 0x2008, 8.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val = read_f64(&mem, 0x3008);
    assert_eq!(val, 3.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for greater");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear for greater");
}

#[test]
fn test_fcomip_less() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (9.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 9.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val = read_f64(&mem, 0x3008);
    assert_eq!(val, 9.0);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear for less");
    assert_ne!(flags & CF_BIT, 0, "CF should be set for less");
}

#[test]
fn test_fcomip_st2() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xDF, 0xF2, // FCOMIP ST(2)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x1C, 0x25, 0x08, 0x30, 0x00, 0x00, // FSTP qword [0x3008]
        0xDD, 0x1C, 0x25, 0x10, 0x30, 0x00, 0x00, // FSTP qword [0x3010]
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    let val1 = read_f64(&mem, 0x3008);
    let val2 = read_f64(&mem, 0x3010);
    assert_eq!(val1, 2.0);
    assert_eq!(val2, 1.0);
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (3.0 > 1.0)");
}

// ============================================================================
// Conditional Branching After FCOMI/FCOMIP
// ============================================================================

#[test]
fn test_fcomi_conditional_je() {
    // Use FCOMI result for conditional jump (JE)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xF1, // FCOMI ST(1)
        0x74, 0x07, // JE +7 (skip if equal)
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 5.0);

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_fcomi_conditional_jb() {
    // Use FCOMI result for conditional jump (JB - jump if below)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (5.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x72, 0x07, // JB +7 (jump if below)
        0xD9, 0xE8, // FLD1
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, 1.0, "Jump should not be taken, FLD1 should execute");
}

#[test]
fn test_fcomi_conditional_ja() {
    // Use FCOMI result for conditional jump (JA - jump if above)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (5.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (10.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x77, 0x07, // JA +7 (jump if above)
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 5.0);
    write_f64(&mem, 0x2008, 10.0);

    run_until_hlt(&mut vcpu).unwrap();

    let val = read_f64(&mem, 0x3000);
    assert_eq!(val, 0.0, "Jump should not be taken, FLDZ should execute");
}

#[test]
fn test_fcomip_conditional_jne() {
    // Use FCOMIP result for conditional jump (JNE - jump if not equal)
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDF, 0xF1, // FCOMIP ST(1)
        0x75, 0x07, // JNE +7 (jump if not equal)
        0xD9, 0xEE, // FLDZ
        0xDD, 0x1C, 0x25, 0x00, 0x30, 0x00, 0x00, // FSTP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Mixed Operations
// ============================================================================

#[test]
fn test_fcomi_sequence() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (3.0)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // POP qword [0x3008]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 3.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags1 = read_u64(&mem, 0x3000);
    let flags2 = read_u64(&mem, 0x3008);
    assert_eq!(flags1 & CF_BIT, 0, "First comparison: 2.0 > 1.0");
    assert_eq!(flags2 & CF_BIT, 0, "Second comparison: 3.0 > 2.0");
}

#[test]
fn test_fcomip_chain() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (2.0)
        0xDD, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // FLD qword [0x2010] (2.0)
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // POP qword [0x3008]
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1.0);
    write_f64(&mem, 0x2008, 2.0);
    write_f64(&mem, 0x2010, 2.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags1 = read_u64(&mem, 0x3000);
    let flags2 = read_u64(&mem, 0x3008);
    assert_ne!(flags1 & ZF_BIT, 0, "First comparison: 2.0 == 2.0");
    assert_eq!(flags2 & CF_BIT, 0, "Second comparison: 2.0 > 1.0");
}

#[test]
fn test_fcomi_zero_comparison() {
    let code = [
        0xD9, 0xEE, // FLDZ
        0xD9, 0xEE, // FLDZ
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_ne!(flags & ZF_BIT, 0, "ZF should be set for 0.0 == 0.0");
}

#[test]
fn test_fcomi_inf_vs_finite() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (1000.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (+inf)
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1000.0);
    write_f64(&mem, 0x2008, f64::INFINITY);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (+inf > 1000.0)");
}

#[test]
fn test_fcomip_negative_numbers() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000] (-10.0)
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008] (-5.0)
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, -10.0);
    write_f64(&mem, 0x2008, -5.0);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (-5.0 > -10.0)");
}

#[test]
fn test_fcomi_denormals() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    let denormal1 = f64::MIN_POSITIVE / 2.0;
    let denormal2 = f64::MIN_POSITIVE / 4.0;
    write_f64(&mem, 0x2000, denormal2);
    write_f64(&mem, 0x2008, denormal1);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(
        flags & CF_BIT,
        0,
        "CF should be clear (denormal1 > denormal2)"
    );
}

#[test]
fn test_fcomi_huge_numbers() {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, 1e100);
    write_f64(&mem, 0x2008, 2e100);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_eq!(flags & CF_BIT, 0, "CF should be clear (2e100 > 1e100)");
}

#[test]
fn test_fcomip_constants() {
    let code = [
        0xD9, 0xEB, // FLDPI
        0xD9, 0xEA, // FLDL2E
        0xDF, 0xF1, // FCOMIP ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xDD, 0xD8, // FSTP ST(0)
        0xF4, // HLT
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    run_until_hlt(&mut vcpu).unwrap();

    let flags = read_u64(&mem, 0x3000);
    assert_eq!(flags & ZF_BIT, 0, "ZF should be clear");
    assert_ne!(flags & CF_BIT, 0, "CF should be set (LOG2_E < PI)");
}

// ============================================================================
// Known-answer FCOMI EFLAGS tests (ZF/PF/CF) for the full ordering matrix,
// including the unordered (NaN) case which sets ZF=PF=CF=1.
// ============================================================================

/// FLD a (-> ST(1)), FLD b (-> ST(0)), FCOMI ST(1), PUSHFQ, store flags.
/// Compares ST(0)=b against ST(1)=a.
fn kat_fcomi_flags(a: f64, b: f64) -> u64 {
    let code = [
        0xDD, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // FLD qword [0x2000]
        0xDD, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // FLD qword [0x2008]
        0xDB, 0xF1, // FCOMI ST(1)
        0x9C, // PUSHFQ
        0x8F, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // POP qword [0x3000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_f64(&mem, 0x2000, a);
    write_f64(&mem, 0x2008, b);
    run_until_hlt(&mut vcpu).unwrap();
    read_u64(&mem, 0x3000)
}

#[test]
fn test_fcomi_flags_matrix() {
    // equal -> ZF=1, PF=0, CF=0
    let eq = kat_fcomi_flags(5.0, 5.0);
    assert_ne!(eq & ZF_BIT, 0);
    assert_eq!(eq & PF_BIT, 0);
    assert_eq!(eq & CF_BIT, 0);
    // greater (ST0=10 > ST1=5) -> all clear
    let gt = kat_fcomi_flags(5.0, 10.0);
    assert_eq!(gt & ZF_BIT, 0);
    assert_eq!(gt & PF_BIT, 0);
    assert_eq!(gt & CF_BIT, 0);
    // less (ST0=3 < ST1=7) -> CF=1 only
    let lt = kat_fcomi_flags(7.0, 3.0);
    assert_eq!(lt & ZF_BIT, 0);
    assert_eq!(lt & PF_BIT, 0);
    assert_ne!(lt & CF_BIT, 0);
}

#[test]
fn test_fcomi_unordered_sets_zf_pf_cf() {
    // NaN operand -> unordered -> ZF=1, PF=1, CF=1 (exceptions masked by default).
    let un = kat_fcomi_flags(1.0, f64::NAN);
    assert_ne!(un & ZF_BIT, 0, "unordered -> ZF=1");
    assert_ne!(un & PF_BIT, 0, "unordered -> PF=1");
    assert_ne!(un & CF_BIT, 0, "unordered -> CF=1");
}

#[test]
fn test_fcomi_clears_of_sf_af() {
    // FCOMI must clear OF, SF, AF regardless of the comparison outcome.
    const OF_BIT: u64 = 1 << 11;
    const SF_BIT: u64 = 1 << 7;
    const AF_BIT: u64 = 1 << 4;
    for (a, b) in [(5.0, 5.0), (5.0, 10.0), (7.0, 3.0), (1.0, f64::NAN)] {
        let f = kat_fcomi_flags(a, b);
        assert_eq!(f & OF_BIT, 0, "OF must be cleared");
        assert_eq!(f & SF_BIT, 0, "SF must be cleared");
        assert_eq!(f & AF_BIT, 0, "AF must be cleared");
    }
}
