use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 10,5 (signed 10>5 => ZF=0,SF=OF) => JG taken; sentinel + RIP.
#[test]
fn test_jg_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x08, // JG +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JG taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JG/JNLE - Jump if Greater / Jump if Not Less or Equal
// Jumps to target if ZF = 0 AND SF = OF (signed comparison)

// Basic JG with condition met (positive > positive)
#[test]
fn test_jg_taken_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5: ZF=0, SF=OF=0)
        0x7f, 0x02, // JG +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with negative > negative
#[test]
fn test_jg_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        0x48, 0xc7, 0xc3, 0xf6, 0xff, 0xff, 0xff, // MOV RBX, -10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-5 > -10: ZF=0, SF=OF)
        0x7f, 0x02, // JG +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with positive > negative
#[test]
fn test_jg_taken_pos_vs_neg() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0xfb, 0xff, 0xff, 0xff, // MOV RBX, -5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 > -5)
        0x7f, 0x02, // JG +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with condition not met (equal)
#[test]
fn test_jg_not_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 == 5: ZF=1)
        0x7f, 0x05, // JG +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JG with condition not met (less than)
#[test]
fn test_jg_not_taken_less() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8)
        0x7f, 0x05, // JG +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JNLE (alias for JG)
#[test]
fn test_jnle_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (15 > 10)
        0x7f, 0x02, // JNLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG forward jump
#[test]
fn test_jg_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0x32, 0x00, 0x00, 0x00, // MOV RBX, 50
        0x48, 0x39, 0xd8, // CMP RAX, RBX (100 > 50)
        0x7f, 0x07, // JG +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 100, "RAX should remain 100");
}

// JG backward jump
#[test]
fn test_jg_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0xf7, // JG -9 (loop while RAX > RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10);
    assert_eq!(regs.rbx, 10, "RBX incremented to 10");
}

// JG preserves all registers
#[test]
fn test_jg_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10, "RAX preserved");
    assert_eq!(regs.rbx, 5, "RBX preserved");
    assert_eq!(regs.rcx, 0x11, "RCX preserved");
    assert_eq!(regs.rdx, 0x22, "RDX preserved");
}

// JG does not affect flags
#[test]
fn test_jg_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF=0, SF=0, OF=0)
        0x7f, 0x02, // JG +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 == 0, "ZF should remain clear");
}

// JG with zero offset
#[test]
fn test_jg_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x00, // JG +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with maximum forward offset
#[test]
fn test_jg_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x7f, // JG +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with maximum backward offset
#[test]
fn test_jg_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x80, // JG -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with 32-bit operands
#[test]
fn test_jg_32bit() {
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with 16-bit operands
#[test]
fn test_jg_16bit() {
    let code = [
        0x66, 0xb8, 0x0a, 0x00, // MOV AX, 10
        0x66, 0xbb, 0x05, 0x00, // MOV BX, 5
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG with 8-bit operands
#[test]
fn test_jg_8bit() {
    let code = [
        0xb0, 0x0a, // MOV AL, 10
        0xb3, 0x05, // MOV BL, 5
        0x38, 0xd8, // CMP AL, BL
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG signed vs unsigned (negative value)
#[test]
fn test_jg_signed_vs_unsigned() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF (-1 as signed)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 < 1 signed)
        0x7f, 0x05, // JG +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JG with immediate CMP
#[test]
fn test_jg_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG chained with other conditions
#[test]
fn test_jg_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5)
        0x7f, 0x05, // JG +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JG with TEST (always SF=0, OF=0)
#[test]
fn test_jg_after_test_nonzero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0xa9, 0x0f, 0x00, 0x00, 0x00, // TEST RAX, 0x0F (ZF=0, SF=0, OF=0)
        0x7f, 0x02, // JG +2 (should jump, ZF=0 and SF=OF)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG countdown loop
#[test]
fn test_jg_countdown_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (sum)
        // loop (offset 14):
        0x48, 0x01, 0xc3, // ADD RBX, RAX
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0
        0x7f, 0xf4, // JG -12 (loop while RAX > 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 15, "Sum 5+4+3+2+1 = 15");
    assert_eq!(regs.rax, 0);
}

// JG range check
#[test]
fn test_jg_range_check() {
    let code = [
        0x48, 0xc7, 0xc0, 0x96, 0x00, 0x00, 0x00, // MOV RAX, 150 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (threshold)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (150 > 100)
        0x7f, 0x09, // JG +9 (above threshold)
        // below or equal:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (skip above)
        // above threshold:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Above threshold");
}

// JG with negative numbers
#[test]
fn test_jg_negative_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfd, 0xff, 0xff, 0xff, // MOV RAX, -3
        0x48, 0xc7, 0xc3, 0xf9, 0xff, 0xff, 0xff, // MOV RBX, -7
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-3 > -7)
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG boundary at zero
#[test]
fn test_jg_boundary_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (1 > 0)
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JG multiple conditions
#[test]
fn test_jg_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        // check if > 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7f, 0x05, // JG +5 (yes, > 10)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // check if > 100
        0x48, 0x3d, 0x64, 0x00, 0x00, 0x00, // CMP EAX, 100
        0x7f, 0x08, // JG +8 (no, not > 100)
        // in range 10 < x <= 100
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xeb, 0x07, // JMP +7 (exit)
        // > 100
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42, "In range");
}

// JG with DEC
#[test]
fn test_jg_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4)
        0x48, 0x83, 0xf8, 0x02, // CMP RAX, 2
        0x7f, 0x02, // JG +2 (4 > 2)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JG with overflow scenario
#[test]
fn test_jg_with_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (most negative)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (most negative < 1)
        0x7f, 0x05, // JG +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JG practical: maximum finder
#[test]
fn test_jg_find_maximum() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (value 1)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (value 2)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x05, // JG +5 (RAX > RBX?)
        // RBX is max:
        0x48, 0x89, 0xd9, // MOV RCX, RBX
        0xeb, 0x03, // JMP +3 (exit)
        // RAX is max:
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 10, "Maximum is 10");
}

// JG with SUB
#[test]
fn test_jg_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3 (result 7)
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x7f, 0x02, // JG +2 (7 > 5)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 7);
}

// JG sorting/comparison pattern
#[test]
fn test_jg_sorting_pattern() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // MOV RBX, 7
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x08, // JG +8 (swap if RAX > RBX)
        // no swap needed, already sorted
        0x48, 0x89, 0xc1, // MOV RCX, RAX (smaller)
        0x48, 0x89, 0xda, // MOV RDX, RBX (larger)
        0xeb, 0x06, // JMP +6 (exit)
        // swap:
        0x48, 0x89, 0xd9, // MOV RCX, RBX (smaller)
        0x48, 0x89, 0xc2, // MOV RDX, RAX (larger)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 3, "Smaller value");
    assert_eq!(regs.rdx, 7, "Larger value");
}

// JG with zero crossing
#[test]
fn test_jg_zero_crossing() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc3, 0xfe, 0xff, 0xff, 0xff, // MOV RBX, -2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (2 > -2)
        0x7f, 0x02, // JG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}
