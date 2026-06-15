use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 5,5 (equal => CF=0) => JAE taken; sentinel + RIP proof.
#[test]
fn test_jae_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x08, // JAE +8 (CF=0)
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JAE taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JAE/JNB/JNC - Jump if Above or Equal / Jump if Not Below / Jump if Not Carry
// Jumps to target if CF = 0 (unsigned comparison)

// Basic JAE with condition met (CF=0)
#[test]
fn test_jae_taken_above() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 5: CF=0)
        0x73, 0x02, // JAE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with equal values (CF=0)
#[test]
fn test_jae_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 >= 5: CF=0)
        0x73, 0x02, // JAE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with condition not met (CF=1)
#[test]
fn test_jae_not_taken_below() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8: CF=1)
        0x73, 0x05, // JAE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JNB (alias for JAE)
#[test]
fn test_jnb_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 10)
        0x73, 0x02, // JNB +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNC (alias for JAE) - Jump if No Carry
#[test]
fn test_jnc_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (no carry, CF=0)
        0x73, 0x02, // JNC +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE forward jump
#[test]
fn test_jae_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x39, 0xd8, // CMP RAX, RBX (100 >= 100)
        0x73, 0x07, // JAE +7 (next=19, target=26)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 100, "RAX should remain 100");
}

// JAE backward jump
#[test]
fn test_jae_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0xf7, // JAE -9 (loop while RAX >= RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10);
    assert_eq!(regs.rbx, 11, "RBX incremented to 11");
}

// JAE preserves all registers
#[test]
fn test_jae_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x02, // JAE +2
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

// JAE does not affect flags
#[test]
fn test_jae_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets CF=0)
        0x73, 0x02, // JAE +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x01 == 0, "CF should remain clear");
}

// JAE with SUB instruction (no borrow)
#[test]
fn test_jae_after_sub_no_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3 (no borrow, CF=0)
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 7);
}

// JAE with SUB instruction (borrow)
#[test]
fn test_jae_after_sub_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x83, 0xe8, 0x0a, // SUB RAX, 10 (borrow, CF=1)
        0x73, 0x05, // JAE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JAE with zero offset
#[test]
fn test_jae_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x00, // JAE +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE in range checking (lower bound)
#[test]
fn test_jae_range_check_lower() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (value)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (min)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (50 >= 10)
        0x73, 0x09, // JAE +9 (next=19, target=28)
        // below range:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (skip in range)
        // in range:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "In range");
}

// JAE with maximum forward offset
#[test]
fn test_jae_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x7f, // JAE +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with maximum backward offset
#[test]
fn test_jae_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x80, // JAE -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with 32-bit operands
#[test]
fn test_jae_32bit() {
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with 16-bit operands
#[test]
fn test_jae_16bit() {
    let code = [
        0x66, 0xb8, 0x0a, 0x00, // MOV AX, 10
        0x66, 0xbb, 0x0a, 0x00, // MOV BX, 10
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with 8-bit operands
#[test]
fn test_jae_8bit() {
    let code = [
        0xb0, 0x0a, // MOV AL, 10
        0xb3, 0x0a, // MOV BL, 10
        0x38, 0xd8, // CMP AL, BL
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with TEST instruction (always CF=0)
#[test]
fn test_jae_after_test() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0xa9, 0x0f, 0x00, 0x00, 0x00, // TEST RAX, 0x0F (CF=0)
        0x73, 0x02, // JAE +2 (should always jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE loop with boundary
#[test]
fn test_jae_loop_boundary() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (check RAX >= RBX)
        0x73, 0xf7, // JAE -9 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 6, "Loop exits when RBX > RAX");
}

// JAE with unsigned large value
#[test]
fn test_jae_unsigned_large() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0xFFFF... >= 1 unsigned)
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE with immediate CMP
#[test]
fn test_jae_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE chained with other conditions
#[test]
fn test_jae_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 >= 10)
        0x73, 0x05, // JAE +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JAE with ADD producing carry
#[test]
fn test_jae_after_add_carry() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (produces carry, CF=1)
        0x73, 0x05, // JAE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JAE with ADD no carry
#[test]
fn test_jae_after_add_no_carry() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (no carry, CF=0)
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE validation pattern
#[test]
fn test_jae_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (input)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (min valid)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x09, // JAE +9 (next=19, target=28)
        // invalid:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (invalid)
        0xeb, 0x07, // JMP +7 (exit)
        // valid:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (valid)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Valid input");
}

// JAE with DEC (doesn't affect CF)
#[test]
fn test_jae_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (doesn't affect CF)
        0x73, 0x02, // JAE +2 (CF still 0 from before)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JAE multiple conditions
#[test]
fn test_jae_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        // check if >= 10
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x73, 0x05, // JAE +5 (yes, >= 10)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // check if >= 100
        0x48, 0x3d, 0x64, 0x00, 0x00, 0x00, // CMP EAX, 100 (offset 23)
        0x73, 0x08, // JAE +8 (no, not >= 100)
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

// JAE with zero comparison
#[test]
fn test_jae_zero_comparison() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0 >= 0)
        0x73, 0x02, // JAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JAE array bounds check
#[test]
fn test_jae_array_bounds() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (index)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (min)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (index >= 0?)
        0x73, 0x09, // JAE +9 (next=19, target=28)
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
