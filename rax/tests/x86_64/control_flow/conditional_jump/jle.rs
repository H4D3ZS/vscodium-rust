use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 5,5 (equal => ZF=1) => JLE taken; sentinel + RIP proof.
#[test]
fn test_jle_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x08, // JLE +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JLE taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JLE/JNG - Jump if Less or Equal / Jump if Not Greater
// Jumps to target if ZF = 1 OR SF != OF (signed comparison)

// Basic JLE with less than
#[test]
fn test_jle_taken_less() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 <= 8: SF!=OF)
        0x7e, 0x02, // JLE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with equal
#[test]
fn test_jle_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 <= 5: ZF=1)
        0x7e, 0x02, // JLE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with negative <= negative
#[test]
fn test_jle_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xf6, 0xff, 0xff, 0xff, // MOV RAX, -10
        0x48, 0xc7, 0xc3, 0xfb, 0xff, 0xff, 0xff, // MOV RBX, -5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-10 <= -5)
        0x7e, 0x02, // JLE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with condition not met (greater than)
#[test]
fn test_jle_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5: ZF=0, SF=OF)
        0x7e, 0x05, // JLE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JNG (alias for JLE)
#[test]
fn test_jng_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 <= 5)
        0x7e, 0x02, // JNG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE forward jump
#[test]
fn test_jle_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 <= 100)
        0x7e, 0x07, // JLE +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "RAX should remain 5");
}

// JLE backward jump
#[test]
fn test_jle_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (target)
        // loop (offset 14):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0xf7, // JLE -9 (loop while RAX <= RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 6, "Loop exits when RAX > RBX");
}

// JLE preserves all registers
#[test]
fn test_jle_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "RAX preserved");
    assert_eq!(regs.rbx, 5, "RBX preserved");
    assert_eq!(regs.rcx, 0x11, "RCX preserved");
    assert_eq!(regs.rdx, 0x22, "RDX preserved");
}

// JLE does not affect flags
#[test]
fn test_jle_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF=1)
        0x7e, 0x02, // JLE +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should remain set");
}

// JLE with zero offset
#[test]
fn test_jle_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x00, // JLE +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with maximum forward offset
#[test]
fn test_jle_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x7f, // JLE +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with maximum backward offset
#[test]
fn test_jle_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x80, // JLE -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with 32-bit operands
#[test]
fn test_jle_32bit() {
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with 16-bit operands
#[test]
fn test_jle_16bit() {
    let code = [
        0x66, 0xb8, 0x05, 0x00, // MOV AX, 5
        0x66, 0xbb, 0x05, 0x00, // MOV BX, 5
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE with 8-bit operands
#[test]
fn test_jle_8bit() {
    let code = [
        0xb0, 0x05, // MOV AL, 5
        0xb3, 0x05, // MOV BL, 5
        0x38, 0xd8, // CMP AL, BL
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE loop with inclusive bound
#[test]
fn test_jle_inclusive_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (inclusive limit)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (sum)
        // loop (offset 21):
        0x48, 0x01, 0xc1, // ADD RCX, RAX
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0xf4, // JLE -12 (loop while RAX <= RBX, inclusive)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 6);
    assert_eq!(regs.rcx, 15, "Sum 0+1+2+3+4+5 = 15");
}

// JLE with immediate CMP
#[test]
fn test_jle_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE chained with other conditions
#[test]
fn test_jle_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 <= 5)
        0x7e, 0x05, // JLE +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JLE with negative comparison
#[test]
fn test_jle_negative_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 <= 0)
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE range validation
#[test]
fn test_jle_range_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (max)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (50 <= 100)
        0x7e, 0x09, // JLE +9 (valid)
        // above maximum:
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

// JLE boundary at zero
#[test]
fn test_jle_boundary_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0 <= 0)
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE multiple conditions
#[test]
fn test_jle_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        // check if <= 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7e, 0x14, // JLE +20 (<= 10)
        // check if <= 100
        0x48, 0x3d, 0x64, 0x00, 0x00, 0x00, // CMP EAX, 100
        0x7e, 0x09, // JLE +9 (10 < x <= 100)
        // > 100:
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xeb, 0x0f, // JMP +15 (exit)
        // 10 < x <= 100:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xeb, 0x07, // JMP +7 (exit)
        // <= 10:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        // exit:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "In range 11-100");
}

// JLE with DEC
#[test]
fn test_jle_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4)
        0x48, 0x83, 0xf8, 0x04, // CMP RAX, 4
        0x7e, 0x02, // JLE +2 (4 <= 4)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JLE with SUB
#[test]
fn test_jle_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x05, // SUB RAX, 5 (result 5)
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x7e, 0x02, // JLE +2 (5 <= 5)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5);
}

// JLE with overflow scenario
#[test]
fn test_jle_with_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000 (most negative)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (most negative <= 0)
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE saturation pattern
#[test]
fn test_jle_saturation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x96, 0x00, 0x00, 0x00, // MOV RAX, 150 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (max)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x05, // JLE +5 (within limit)
        // saturate:
        0x48, 0x89, 0xd8, // MOV RAX, RBX (clamp to max)
        0xeb, 0x00, // JMP +0 (continue)
        // within limit, RAX unchanged
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 100, "Saturated to max");
}

// JLE zero crossing
#[test]
fn test_jle_zero_crossing() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfe, 0xff, 0xff, 0xff, // MOV RAX, -2
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-2 <= 2)
        0x7e, 0x02, // JLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JLE array bounds check (inclusive)
#[test]
fn test_jle_array_bounds_inclusive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x09, 0x00, 0x00, 0x00, // MOV RAX, 9 (index)
        0x48, 0xc7, 0xc3, 0x09, 0x00, 0x00, 0x00, // MOV RBX, 9 (max index)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (index <= max?)
        0x7e, 0x09, // JLE +9 (valid)
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

// JLE sum loop
#[test]
fn test_jle_sum_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (counter)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (sum)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (limit)
        // loop (offset 21):
        0x48, 0x01, 0xc1, // ADD RCX, RAX
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0xf4, // JLE -12 (loop while RAX <= 10)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 11);
    assert_eq!(regs.rcx, 55, "Sum 1+2+...+10 = 55");
}
