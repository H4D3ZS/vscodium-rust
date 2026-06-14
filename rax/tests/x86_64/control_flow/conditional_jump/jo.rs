use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// JO - Jump if Overflow
// Jumps to target if OF = 1 (signed overflow occurred)

// Basic JO with signed overflow
#[test]
fn test_jo_taken_overflow_add() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF (max positive)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflows to negative, OF=1)
        0x70, 0x02, // JO +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with condition not met (no overflow)
#[test]
fn test_jo_not_taken_no_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (no overflow, OF=0)
        0x70, 0x05, // JO +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JO with SUB overflow
#[test]
fn test_jo_after_sub_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (min negative)
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1 (overflow, OF=1)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO forward jump
#[test]
fn test_jo_forward() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow)
        0x70, 0x07, // JO +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO backward jump
#[test]
fn test_jo_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // target (offset 7):
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x01, 0xd8, // ADD RAX, RBX (overflow)
        0x70, 0x02, // JO +2 (exit)
        0xeb, 0xe8, // JMP -24 (back to target, should not execute)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO preserves all registers
#[test]
fn test_jo_preserves_registers() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
}

// JO does not affect flags
#[test]
fn test_jo_preserves_flags() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets OF=1)
        0x70, 0x02, // JO +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x800 != 0, "OF should remain set");
}

// JO with zero offset
#[test]
fn test_jo_zero_offset() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x70, 0x00, // JO +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with maximum forward offset
#[test]
fn test_jo_max_forward_offset() {
    let mut code = vec![
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x70, 0x7f, // JO +127
    ];
    code.resize(16 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with maximum backward offset
#[test]
fn test_jo_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x70, 0x80, // JO -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with NEG overflow
#[test]
fn test_jo_after_neg_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (min negative)
        0x48, 0xf7, 0xd8, // NEG RAX (overflow, OF=1)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO overflow detection pattern
#[test]
fn test_jo_overflow_detection() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow)
        0x70, 0x09, // JO +9 (overflow detected)
        // no overflow:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // overflow detected:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Overflow detected");
}

// JO with positive + positive overflow
#[test]
fn test_jo_positive_plus_positive() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, // MOV RAX, 0x4000000000000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, // MOV RBX, 0x4000000000000000
        0x48, 0x01, 0xd8, // ADD RAX, RBX (overflow to negative)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with negative + negative overflow
#[test]
fn test_jo_negative_plus_negative() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xc0, // MOV RAX, 0xC000000000000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xc0, // MOV RBX, 0xC000000000000000
        0x48, 0x01, 0xd8, // ADD RAX, RBX (overflow to positive)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with 32-bit overflow
#[test]
fn test_jo_32bit_overflow() {
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (32-bit overflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with 16-bit overflow
#[test]
fn test_jo_16bit_overflow() {
    let code = [
        0x66, 0xb8, 0xff, 0x7f, // MOV AX, 0x7FFF
        0x66, 0x83, 0xc0, 0x01, // ADD AX, 1 (16-bit overflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with 8-bit overflow
#[test]
fn test_jo_8bit_overflow() {
    let code = [
        0xb0, 0x7f, // MOV AL, 0x7F
        0x04, 0x01, // ADD AL, 1 (8-bit overflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO chaining
#[test]
fn test_jo_chaining() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow)
        0x70, 0x07, // JO +7
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // jumped here:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JO practical: saturation arithmetic
#[test]
fn test_jo_saturation() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x01, 0xd8, // ADD RAX, RBX (overflow)
        0x70, 0x02, // JO +2 (saturate)
        0xeb, 0x0a, // JMP +10 (skip saturation)
        // saturate to max:
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7FFFFFFFFFFFFFFF, "Saturated to max");
}

// JO with IMUL overflow
#[test]
fn test_jo_after_imul_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, // MOV RAX, 0x4000000000000000
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x0f, 0xaf, 0xc3, // IMUL RAX, RBX (overflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with INC (doesn't set OF)
#[test]
fn test_jo_after_inc_no_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc0, // INC RAX (INC doesn't set OF)
        0x70, 0x05, // JO +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JO subtraction underflow
#[test]
fn test_jo_sub_underflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, // MOV RAX, min negative
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x29, 0xd8, // SUB RAX, RBX (underflow)
        0x70, 0x02, // JO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JO with TEST (never sets OF)
#[test]
fn test_jo_after_test_no_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (OF=0)
        0x70, 0x05, // JO +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JO error handling pattern
#[test]
fn test_jo_error_handling() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow)
        0x70, 0x09, // JO +9 (handle overflow)
        // success:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // overflow error:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Overflow error");
}
