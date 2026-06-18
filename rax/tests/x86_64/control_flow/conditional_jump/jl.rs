use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 3,8 (signed 3<8 => SF!=OF) => JL taken; sentinel + RIP proof.
#[test]
fn test_jl_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x08, // JL +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JL taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JL/JNGE - Jump if Less / Jump if Not Greater or Equal
// Jumps to target if SF != OF (signed comparison)

// Basic JL with less than (positive < positive)
#[test]
fn test_jl_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8: SF!=OF)
        0x7c, 0x02, // JL +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with negative < negative
#[test]
fn test_jl_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xf6, 0xff, 0xff, 0xff, // MOV RAX, -10
        0x48, 0xc7, 0xc3, 0xfb, 0xff, 0xff, 0xff, // MOV RBX, -5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-10 < -5)
        0x7c, 0x02, // JL +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with negative < positive
#[test]
fn test_jl_taken_neg_vs_pos() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-5 < 5)
        0x7c, 0x02, // JL +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with condition not met (equal)
#[test]
fn test_jl_not_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 == 5)
        0x7c, 0x05, // JL +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JL with condition not met (greater than)
#[test]
fn test_jl_not_taken_greater() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5)
        0x7c, 0x05, // JL +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JNGE (alias for JL)
#[test]
fn test_jnge_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (2 < 10)
        0x7c, 0x02, // JNGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL forward jump
#[test]
fn test_jl_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 100)
        0x7c, 0x07, // JL +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "RAX should remain 5");
}

// JL backward jump
#[test]
fn test_jl_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (target)
        // loop (offset 14):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0xf7, // JL -9 (loop while RAX < RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10);
}

// JL preserves all registers
#[test]
fn test_jl_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "RAX preserved");
    assert_eq!(regs.rbx, 8, "RBX preserved");
    assert_eq!(regs.rcx, 0x11, "RCX preserved");
    assert_eq!(regs.rdx, 0x22, "RDX preserved");
}

// JL does not affect flags
#[test]
fn test_jl_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets SF!=OF)
        0x7c, 0x02, // JL +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with zero offset
#[test]
fn test_jl_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x00, // JL +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with maximum forward offset
#[test]
fn test_jl_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x7f, // JL +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with maximum backward offset
#[test]
fn test_jl_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x80, // JL -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with 32-bit operands
#[test]
fn test_jl_32bit() {
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0x39, 0xd8, // CMP EAX, EBX
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with 16-bit operands
#[test]
fn test_jl_16bit() {
    let code = [
        0x66, 0xb8, 0x03, 0x00, // MOV AX, 3
        0x66, 0xbb, 0x08, 0x00, // MOV BX, 8
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with 8-bit operands
#[test]
fn test_jl_8bit() {
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0xb3, 0x08, // MOV BL, 8
        0x38, 0xd8, // CMP AL, BL
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL signed comparison
#[test]
fn test_jl_signed() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 < 1 signed)
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL with immediate CMP
#[test]
fn test_jl_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL chained with other conditions
#[test]
fn test_jl_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8)
        0x7c, 0x05, // JL +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JL loop counting up
#[test]
fn test_jl_count_up_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (sum)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (limit)
        // loop (offset 21):
        0x48, 0x01, 0xc1, // ADD RCX, RAX
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0xf4, // JL -12 (loop while RAX < RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5);
    assert_eq!(regs.rcx, 10, "Sum 0+1+2+3+4 = 10");
}

// JL range check
#[test]
fn test_jl_range_check() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (max)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (50 < 100)
        0x7c, 0x09, // JL +9 (below max)
        // at or above max:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (skip)
        // below max:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Below max");
}

// JL with negative numbers
#[test]
fn test_jl_negative_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0xf9, 0xff, 0xff, 0xff, // MOV RAX, -7
        0x48, 0xc7, 0xc3, 0xfd, 0xff, 0xff, 0xff, // MOV RBX, -3
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-7 < -3)
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL boundary at zero
#[test]
fn test_jl_boundary_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 < 0)
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL multiple conditions
#[test]
fn test_jl_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        // check if < 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7c, 0x05, // JL +5 (yes, < 10)
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2 (skipped)
        // check if < 3
        0x48, 0x83, 0xf8, 0x03, // CMP RAX, 3
        0x7c, 0x08, // JL +8 (no, not < 3)
        // in range 3 <= x < 10
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xeb, 0x07, // JMP +7 (exit)
        // < 3
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42, "In range");
}

// JL with DEC
#[test]
fn test_jl_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4)
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7c, 0x02, // JL +2 (4 < 10)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JL with SUB
#[test]
fn test_jl_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3 (result 7)
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7c, 0x02, // JL +2 (7 < 10)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 7);
}

// JL with overflow scenario
#[test]
fn test_jl_with_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (most negative)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (most negative < 1)
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL practical: minimum finder
#[test]
fn test_jl_find_minimum() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (value 1)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (value 2)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x05, // JL +5 (RAX < RBX?)
        // RBX is min:
        0x48, 0x89, 0xd9, // MOV RCX, RBX
        0xeb, 0x03, // JMP +3 (exit)
        // RAX is min:
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 5, "Minimum is 5");
}

// JL zero crossing
#[test]
fn test_jl_zero_crossing() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfe, 0xff, 0xff, 0xff, // MOV RAX, -2
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-2 < 2)
        0x7c, 0x02, // JL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JL array bounds validation (upper)
#[test]
fn test_jl_array_bounds() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (index)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (size)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (index < size?)
        0x7c, 0x09, // JL +9 (valid)
        // invalid index:
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

// JL with positive < negative (unsigned perspective)
#[test]
fn test_jl_pos_lt_neg_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0xfb, 0xff, 0xff, 0xff, // MOV RBX, -5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 is NOT < -5 signed)
        0x7c, 0x05, // JL +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}
