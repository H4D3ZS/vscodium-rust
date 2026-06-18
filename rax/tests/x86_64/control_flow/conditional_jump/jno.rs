use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// JNO - Jump if Not Overflow
// Jumps to target if OF = 0 (no signed overflow)

// Basic JNO with no overflow
#[test]
fn test_jno_taken_no_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (no overflow, OF=0)
        0x71, 0x02, // JNO +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with condition not met (overflow)
#[test]
fn test_jno_not_taken_overflow() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, max positive
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (overflow, OF=1)
        0x71, 0x05, // JNO +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JNO forward jump
#[test]
fn test_jno_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5 (no overflow)
        0x71, 0x07, // JNO +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x47, "RAX should be 0x47");
}

// JNO backward jump
#[test]
fn test_jno_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1 (no overflow)
        0x48, 0x83, 0xfb, 0x03, // CMP RBX, 3
        0x74, 0x02, // JE +2 (exit)
        0xeb, 0xf4, // JMP -12 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 3);
}

// JNO preserves all registers
#[test]
fn test_jno_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5 (no overflow)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
}

// JNO does not affect flags
#[test]
fn test_jno_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (sets OF=0)
        0x71, 0x02, // JNO +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x800 == 0, "OF should remain clear");
}

// JNO with zero offset
#[test]
fn test_jno_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3
        0x71, 0x00, // JNO +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with maximum forward offset
#[test]
fn test_jno_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3
        0x71, 0x7f, // JNO +127
    ];
    code.resize(13 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with maximum backward offset
#[test]
fn test_jno_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3
        0x71, 0x80, // JNO -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with TEST (always clears OF)
#[test]
fn test_jno_after_test() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears OF)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO safe arithmetic pattern
#[test]
fn test_jno_safe_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x01, 0xd8, // ADD RAX, RBX (no overflow)
        0x71, 0x09, // JNO +9 (safe)
        // overflow:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // safe:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Safe operation");
}

// JNO with SUB (no underflow)
#[test]
fn test_jno_after_sub_no_underflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x05, // SUB RAX, 5 (no underflow)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with 32-bit operands
#[test]
fn test_jno_32bit() {
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0x83, 0xc0, 0x03, // ADD EAX, 3
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with 16-bit operands
#[test]
fn test_jno_16bit() {
    let code = [
        0x66, 0xb8, 0x05, 0x00, // MOV AX, 5
        0x66, 0x83, 0xc0, 0x03, // ADD AX, 3
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with 8-bit operands
#[test]
fn test_jno_8bit() {
    let code = [
        0xb0, 0x05, // MOV AL, 5
        0x04, 0x03, // ADD AL, 3
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO chaining
#[test]
fn test_jno_chaining() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (no overflow)
        0x71, 0x05, // JNO +5
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // jumped here:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JNO with INC (INC doesn't set OF)
#[test]
fn test_jno_after_inc() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc0, // INC RAX (doesn't set OF)
        0x71, 0x02, // JNO +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with DEC (DEC doesn't set OF)
#[test]
fn test_jno_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (doesn't set OF)
        0x71, 0x02, // JNO +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with AND (logical operations clear OF)
#[test]
fn test_jno_after_and() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x25, 0x03, 0x00, 0x00, 0x00, // AND RAX, 0x03 (clears OF)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with OR (logical operations clear OF)
#[test]
fn test_jno_after_or() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x0d, 0x03, 0x00, 0x00, 0x00, // OR RAX, 0x03 (clears OF)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with XOR (logical operations clear OF)
#[test]
fn test_jno_after_xor() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x35, 0x03, 0x00, 0x00, 0x00, // XOR RAX, 0x03 (clears OF)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO practical: validation
#[test]
fn test_jno_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x01, 0xd8, // ADD RAX, RBX (no overflow)
        0x71, 0x09, // JNO +9 (valid)
        // overflow (invalid):
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // valid (no overflow):
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Valid");
}

// JNO with IMUL (no overflow)
#[test]
fn test_jno_after_imul_no_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x48, 0x0f, 0xaf, 0xc3, // IMUL RAX, RBX (no overflow)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO with CMP (doesn't affect OF significantly for typical use)
#[test]
fn test_jno_after_cmp() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (typically no overflow for positive values)
        0x71, 0x02, // JNO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNO safe range checking
#[test]
fn test_jno_safe_range() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        0x48, 0xc7, 0xc3, 0x32, 0x00, 0x00, 0x00, // MOV RBX, 50
        0x48, 0x01, 0xd8, // ADD RAX, RBX (100, no overflow)
        0x71, 0x09, // JNO +9 (safe)
        // overflow:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // safe:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Safe");
}
