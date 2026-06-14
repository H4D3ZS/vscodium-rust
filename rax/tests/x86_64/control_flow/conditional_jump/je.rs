use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 5,5 (equal => ZF=1) => JE taken; sentinel + RIP proof.
#[test]
fn test_je_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x74, 0x08, // JE +8 (ZF=1)
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JE taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JE/JZ - Jump if Equal / Jump if Zero
// Jumps to target if ZF = 1

// Basic JE with ZF set
#[test]
fn test_je_taken_zf_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x74, 0x02, // JE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be 0");
}

// JE with ZF clear (should not jump)
#[test]
fn test_je_not_taken_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF)
        0x74, 0x05, // JE +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (should execute)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should be 0x42");
}

// JZ (alias for JE) with ZF set
#[test]
fn test_jz_taken() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x74, 0x02, // JZ +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE forward jump
#[test]
fn test_je_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF)
        0x74, 0x07, // JE +7
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should remain 0");
}

// JE backward jump (loop)
#[test]
fn test_je_backward_loop() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (loop start, offset 11)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0x48, 0x85, 0xc9, // TEST RCX, RCX
        0x75, 0xf1, // JNZ -15 (back to loop start) (NOTE: using JNZ for loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "RAX should be 3");
    assert_eq!(regs.rcx, 0, "RCX should be 0");
}

// JE with CMP instruction
#[test]
fn test_je_after_cmp_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with CMP instruction (not equal)
#[test]
fn test_je_after_cmp_not_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x43, 0x00, 0x00, 0x00, // MOV RBX, 0x43
        0x48, 0x39, 0xd8, // CMP RAX, RBX (clears ZF)
        0x74, 0x05, // JE +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x99);
}

// JE with SUB resulting in zero
#[test]
fn test_je_after_sub_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xe8, 0x05, // SUB RAX, 5 (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// JE preserves all registers
#[test]
fn test_je_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x31, 0xd2, // XOR RDX, RDX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11, "RAX preserved");
    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
    assert_eq!(regs.rdx, 0, "RDX is 0");
}

// JE does not affect flags
#[test]
fn test_je_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF, clears CF, OF, SF)
        0x74, 0x02, // JE +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF should remain set");
}

// JE with maximum forward offset (127 bytes)
#[test]
fn test_je_max_forward_offset() {
    let mut code = vec![
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x74, 0x7f, // JE +127
    ];
    // Add 127 bytes of padding
    code.resize(5 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT at offset 132

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with maximum backward offset (-128 bytes)
#[test]
fn test_je_max_backward_offset() {
    let mut code = vec![];
    // HLT at start
    code.push(0xf4);
    // Add padding to reach -128 offset
    code.resize(129, 0x90); // NOPs
    // Setup ZF and jump back
    code.extend_from_slice(&[
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x74, 0x80, // JE -128 (back to HLT)
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with zero offset (jumps to next instruction)
#[test]
fn test_je_zero_offset() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x74, 0x00, // JE +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE in if-then-else pattern
#[test]
fn test_je_if_then_else() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF)
        0x74, 0x09, // JE +9 (to then branch)
        // else branch:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (skip then branch)
        // then branch:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Then branch executed");
}

// JE with DEC instruction
#[test]
fn test_je_after_dec_to_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xff, 0xc8, // DEC RAX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// JE with INC instruction
#[test]
fn test_je_after_inc_to_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xff, 0xc0, // INC RAX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// JE chained with other conditional jumps
#[test]
fn test_je_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF)
        0x74, 0x05, // JE +5
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (skipped)
        // jumped here
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 2);
}

// JE with AND instruction
#[test]
fn test_je_after_and_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x25, 0xf0, 0x00, 0x00, 0x00, // AND RAX, 0xF0 (result 0, sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// JE with OR instruction
#[test]
fn test_je_after_or_nonzero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x09, 0xc0, // OR RAX, RAX (clears ZF)
        0x74, 0x05, // JE +5 (should not jump)
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42);
}

// JE practical: array search termination
#[test]
fn test_je_array_search() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (index)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (target)
        // loop:
        0x48, 0x39, 0xd8, // CMP RAX, RBX (offset 18)
        0x74, 0x05, // JE +5 (found)
        0x48, 0x83, 0xc0, 0x01, // INC RAX
        0xeb, 0xf5, // JMP -11 (loop)
        // found:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "Found at index 5");
}

// JE with 32-bit operands (EAX)
#[test]
fn test_je_32bit() {
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x85, 0xc0, // TEST EAX, EAX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with 16-bit operands (AX)
#[test]
fn test_je_16bit() {
    let code = [
        0x66, 0xb8, 0x00, 0x00, // MOV AX, 0
        0x66, 0x85, 0xc0, // TEST AX, AX (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with 8-bit operands (AL)
#[test]
fn test_je_8bit() {
    let code = [
        0xb0, 0x00, // MOV AL, 0
        0x84, 0xc0, // TEST AL, AL (sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE with NEG instruction resulting in zero
#[test]
fn test_je_after_neg_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xf7, 0xd8, // NEG RAX (0 negated is 0, sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE in state machine pattern
#[test]
fn test_je_state_machine() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (state)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // state 0:
        0x48, 0x85, 0xc0, // TEST RAX, RAX (offset 18)
        0x74, 0x0a, // JE +10 (handle state 0)
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF (wrong state)
        0xeb, 0x08, // JMP +8 (exit)
        // handle state 0:
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42 (state 0 handled)
        // exit:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42, "State 0 handled");
}

// JE with LOOP interaction
#[test]
fn test_je_with_loop_pattern() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop start (offset 14):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xe9, 0x01, // SUB RCX, 1
        0x48, 0x83, 0xf9, 0x00, // CMP RCX, 0
        0x74, 0x02, // JE +2 (exit loop)
        0xeb, 0xf1, // JMP -15 (loop start)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5);
    assert_eq!(regs.rcx, 0);
}

// JE with shift instruction resulting in zero
#[test]
fn test_je_after_shift_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xd1, 0xe0, // SHL RAX, 1 (0 shifted is 0, sets ZF)
        0x74, 0x02, // JE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JE multiple sequential checks
#[test]
fn test_je_multiple_checks() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x74, 0x05, // JE +5 (first check passed)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x85, 0xc9, // TEST RCX, RCX
        0x74, 0x05, // JE +5 (second check passed)
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2 (skipped)
        // second check passed:
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 3, "Both checks passed");
}
