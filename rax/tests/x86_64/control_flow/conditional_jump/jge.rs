use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 5,5 (equal => SF=OF) => JGE taken; sentinel + RIP proof.
#[test]
fn test_jge_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x08, // JGE +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JGE taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JGE/JNL - Jump if Greater or Equal / Jump if Not Less
// Jumps to target if SF = OF (signed comparison)

// Basic JGE with greater (positive > positive)
#[test]
fn test_jge_taken_greater() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 5: SF=OF=0)
        0x7d, 0x02, // JGE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with equal
#[test]
fn test_jge_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 >= 5: SF=OF)
        0x7d, 0x02, // JGE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with negative >= negative
#[test]
fn test_jge_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        0x48, 0xc7, 0xc3, 0xf6, 0xff, 0xff, 0xff, // MOV RBX, -10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-5 >= -10)
        0x7d, 0x02, // JGE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with condition not met (less than)
#[test]
fn test_jge_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8: SF!=OF)
        0x7d, 0x05, // JGE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JNL (alias for JGE)
#[test]
fn test_jnl_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 10)
        0x7d, 0x02, // JNL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE forward jump
#[test]
fn test_jge_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x39, 0xd8, // CMP RAX, RBX (100 >= 100)
        0x7d, 0x07, // JGE +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 100, "RAX should remain 100");
}

// JGE backward jump
#[test]
fn test_jge_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0xf7, // JGE -9 (loop while RAX >= RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10);
    assert_eq!(regs.rbx, 11, "RBX incremented to 11");
}

// JGE preserves all registers
#[test]
fn test_jge_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10, "RAX preserved");
    assert_eq!(regs.rbx, 10, "RBX preserved");
    assert_eq!(regs.rcx, 0x11, "RCX preserved");
    assert_eq!(regs.rdx, 0x22, "RDX preserved");
}

// JGE does not affect flags
#[test]
fn test_jge_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF=1, SF=0, OF=0)
        0x7d, 0x02, // JGE +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should remain set");
}

// JGE with zero offset
#[test]
fn test_jge_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x00, // JGE +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with maximum forward offset
#[test]
fn test_jge_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x7f, // JGE +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with maximum backward offset
#[test]
fn test_jge_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x80, // JGE -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with 32-bit operands
#[test]
fn test_jge_32bit() {
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with 16-bit operands
#[test]
fn test_jge_16bit() {
    let code = [
        0x66, 0xb8, 0x0a, 0x00, // MOV AX, 10
        0x66, 0xbb, 0x0a, 0x00, // MOV BX, 10
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with 8-bit operands
#[test]
fn test_jge_8bit() {
    let code = [
        0xb0, 0x0a, // MOV AL, 10
        0xb3, 0x0a, // MOV BL, 10
        0x38, 0xd8, // CMP AL, BL
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE signed comparison with negatives
#[test]
fn test_jge_signed_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0xfe, 0xff, 0xff, 0xff, // MOV RBX, -2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 >= -2)
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with immediate CMP
#[test]
fn test_jge_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE chained with other conditions
#[test]
fn test_jge_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 10)
        0x7d, 0x05, // JGE +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JGE loop with inclusive bound
#[test]
fn test_jge_inclusive_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (sum)
        // loop (offset 14):
        0x48, 0x01, 0xc3, // ADD RBX, RAX
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0
        0x7d, 0xf4, // JGE -12 (loop while RAX >= 0, inclusive)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 15, "Sum 5+4+3+2+1+0 = 15");
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX is -1");
}

// JGE with TEST (always SF=0, OF=0)
#[test]
fn test_jge_after_test_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0xa9, 0x0f, 0x00, 0x00, 0x00, // TEST RAX, 0x0F (SF=0, OF=0)
        0x7d, 0x02, // JGE +2 (should jump, SF=OF)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE range validation (lower bound)
#[test]
fn test_jge_range_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (value)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (min)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (50 >= 10)
        0x7d, 0x09, // JGE +9 (valid)
        // below minimum:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // valid:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Valid");
}

// JGE boundary at zero
#[test]
fn test_jge_boundary_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0 >= 0)
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE multiple conditions
#[test]
fn test_jge_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        // check if >= 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7d, 0x05, // JGE +5 (yes, >= 10)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // check if >= 100
        0x48, 0x3d, 0x64, 0x00, 0x00, 0x00, // CMP EAX, 100
        0x7d, 0x08, // JGE +8 (no, not >= 100)
        // in range 10 <= x < 100
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xeb, 0x07, // JMP +7 (exit)
        // >= 100
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42, "In range");
}

// JGE with DEC
#[test]
fn test_jge_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4)
        0x48, 0x83, 0xf8, 0x04, // CMP RAX, 4
        0x7d, 0x02, // JGE +2 (4 >= 4)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JGE with SUB
#[test]
fn test_jge_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x05, // SUB RAX, 5 (result 5)
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x7d, 0x02, // JGE +2 (5 >= 5)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5);
}

// JGE with positive >= negative
#[test]
fn test_jge_pos_vs_neg() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0xfb, 0xff, 0xff, 0xff, // MOV RBX, -5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 >= -5)
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE with zero >= negative
#[test]
fn test_jge_zero_vs_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, -1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0 >= -1)
        0x7d, 0x02, // JGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JGE clamping pattern
#[test]
fn test_jge_clamping() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5 (value)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (min)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x05, // JGE +5 (within range)
        // below minimum:
        0x48, 0x89, 0xd8, // MOV RAX, RBX (clamp to min)
        0xeb, 0x00, // JMP +0 (continue)
        // within range, RAX unchanged
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "Clamped to minimum");
}

// JGE with overflow scenario
#[test]
fn test_jge_with_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (most negative)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (most negative < 0)
        0x7d, 0x05, // JGE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JGE array access validation
#[test]
fn test_jge_array_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (index)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (min index)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (index >= 0?)
        0x7d, 0x09, // JGE +9 (valid)
        // negative index (invalid):
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0xeb, 0x07, // JMP +7 (exit)
        // valid index:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42, "Valid index");
}
