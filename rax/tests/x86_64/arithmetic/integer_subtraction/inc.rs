//! Tests for the INC instruction.
//!
//! INC - Increment by 1
//!
//! Adds 1 to the destination operand while preserving the CF flag.
//!
//! Flags affected: OF, SF, ZF, AF, PF (CF is NOT affected)
//!
//! Reference: docs/inc.txt

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// ============================================================================
// INC r/m8 (opcode FE /0)
// ============================================================================

#[test]
fn test_inc_rm8_register_basic() {
    // INC AL
    // FE C0 = INC AL
    // f4 = HLT
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 11, "INC AL: 10 + 1 = 11");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_inc_rm8_register_zero_result() {
    // INC AL when AL = 0xFF -> 0x00
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0, "INC AL: 0xFF + 1 = 0x00 (wraps)");
    assert!(zf_set(regs.rflags), "ZF should be set (result = 0)");
}

#[test]
fn test_inc_rm8_register_overflow() {
    // INC AL when AL = 0x7F -> 0x80 (signed overflow)
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80, "INC AL: 0x7F + 1 = 0x80");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set (result negative)");
}

#[test]
fn test_inc_rm8_preserves_cf() {
    // INC should NOT affect CF flag
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::CF; // CF=1 initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0);
    assert!(cf_set(regs.rflags), "CF should be preserved (still set)");
}

#[test]
fn test_inc_rm8_memory() {
    // INC BYTE PTR [RBX]
    // FE 03 = INC BYTE PTR [RBX]
    let code = [0xfe, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 42);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 43, "INC [RBX]: 42 + 1 = 43");
}

// ============================================================================
// INC r/m16 (opcode FF /0 with 66 prefix)
// ============================================================================

#[test]
fn test_inc_rm16_register() {
    // INC AX
    // 66 FF C0 = INC AX
    let code = [0x66, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1235, "INC AX: 0x1234 + 1 = 0x1235");
}

#[test]
fn test_inc_rm16_overflow() {
    // INC AX when AX = 0xFFFF -> 0x0000
    let code = [0x66, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0, "INC AX: 0xFFFF + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_inc_rm16_memory() {
    // INC WORD PTR [RBX]
    // 66 FF 03 = INC WORD PTR [RBX]
    let code = [0x66, 0xff, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u16(&mem, 0x1234);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u16(&mem), 0x1235);
}

// ============================================================================
// INC r/m32 (opcode FF /0)
// ============================================================================

#[test]
fn test_inc_rm32_register() {
    // INC EAX
    // FF C0 = INC EAX
    let code = [0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x12345679, "INC EAX: 0x12345678 + 1");
}

#[test]
fn test_inc_rm32_overflow() {
    // INC EAX when EAX = 0xFFFFFFFF -> 0x00000000
    let code = [0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "INC EAX: 0xFFFFFFFF + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_inc_rm32_signed_overflow() {
    // INC EAX when EAX = 0x7FFFFFFF -> 0x80000000 (signed overflow)
    let code = [0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x80000000);
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
}

#[test]
fn test_inc_rm32_memory() {
    // INC DWORD PTR [RBX]
    // FF 03 = INC DWORD PTR [RBX]
    let code = [0xff, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x12345679);
}

// ============================================================================
// INC r/m64 (opcode REX.W FF /0)
// ============================================================================

#[test]
fn test_inc_rm64_register() {
    // INC RAX
    // 48 FF C0 = INC RAX
    let code = [0x48, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF1, "INC RAX");
}

#[test]
fn test_inc_rm64_overflow() {
    // INC RAX when RAX = 0xFFFFFFFFFFFFFFFF -> 0x0000000000000000
    let code = [0x48, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "INC RAX: max + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_inc_rm64_signed_overflow() {
    // INC RAX when RAX = 0x7FFFFFFFFFFFFFFF -> 0x8000000000000000
    let code = [0x48, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8000000000000000);
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_inc_rm64_memory() {
    // INC QWORD PTR [RBX]
    // 48 FF 03 = INC QWORD PTR [RBX]
    let code = [0x48, 0xff, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u64(&mem, 0xFEDCBA9876543210);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u64(&mem), 0xFEDCBA9876543211);
}

// ============================================================================
// Different Registers
// ============================================================================

#[test]
fn test_inc_different_registers() {
    // INC ECX
    // FF C1 = INC ECX
    let code = [0xff, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 99;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 100, "INC ECX: 99 + 1 = 100");
}

#[test]
fn test_inc_cl() {
    // INC CL
    // FE C1 = INC CL
    let code = [0xfe, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 255;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0, "INC CL: 255 + 1 = 0 (wraps)");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_inc_parity_flag() {
    // INC should set parity based on result
    // 0x02 + 1 = 0x03 (binary 00000011, 2 bits = even parity)
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x02;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x03);
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_inc_auxiliary_flag() {
    // AF should be set when carry from bit 3 to bit 4
    // 0x0F + 1 = 0x10 (carry from bit 3)
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x10);
    assert!(af_set(regs.rflags), "AF should be set (carry from bit 3)");
}

#[test]
fn test_inc_no_auxiliary_flag() {
    // 0x0E + 1 = 0x0F (no carry from bit 3)
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x0E;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x0F);
    assert!(!af_set(regs.rflags), "AF should be clear");
}

#[test]
fn test_inc_cf_independence() {
    // Verify CF is completely independent
    // Test 1: CF=0 initially
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2; // CF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!cf_set(regs.rflags), "CF should remain clear");

    // Test 2: CF=1 initially
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::CF; // CF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(cf_set(regs.rflags), "CF should remain set");
}

// ============================================================================
// Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_inc_r8_extended() {
    // INC R8D
    // 41 FF C0 = INC R8D
    let code = [0x41, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.r8 = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 101, "INC R8D: 100 + 1 = 101");
}

#[test]
fn test_inc_r15_extended() {
    // INC R15
    // 49 FF C7 = INC R15
    let code = [0x49, 0xff, 0xc7, 0xf4];
    let mut regs = Registers::default();
    regs.r15 = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0, "INC R15: max + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_inc_r8l_byte() {
    // INC R8L (low byte of R8)
    // 41 FE C0 = INC R8L
    let code = [0x41, 0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFF, 0, "INC R8L: 0xFF + 1 = 0x00");
}

// ============================================================================
// Loop Counter Use Case
// ============================================================================

#[test]
fn test_inc_as_loop_counter() {
    // Simulating loop: increment counter 3 times
    // INC EAX
    // INC EAX
    // INC EAX
    let code = [
        0xff, 0xc0, // INC EAX
        0xff, 0xc0, // INC EAX
        0xff, 0xc0, // INC EAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "Three INC operations: 0 + 1 + 1 + 1 = 3");
}

#[test]
fn test_inc_preserves_high_bytes() {
    // Verify INC AL doesn't affect high bytes
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x79);
    assert_eq!(
        regs.rax & !0xFF,
        0xDEADBEEF_12345600,
        "High bytes should be preserved"
    );
}
