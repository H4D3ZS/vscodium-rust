use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// JMP - Unconditional Jump
// Transfers control to target address

// Basic JMP with short relative offset
#[test]
fn test_jmp_short() {
    let code = [
        0xeb, 0x02, // JMP +2 (skip next 2 bytes)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    // If we reach here, jump worked (otherwise would HLT earlier)
}

// JMP forward with near relative offset
#[test]
fn test_jmp_near_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0xe9, 0x07, 0x00, 0x00, 0x00, // JMP +7 (skip next instruction)
        0x48, 0xc7, 0xc0, 0x22, 0x00, 0x00, 0x00, // MOV RAX, 0x22 (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "Skipped instruction not executed");
}

// JMP backward (loop-like behavior)
#[test]
fn test_jmp_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (offset 0)
        0x48, 0x83, 0xe8, 0x01, // DEC RAX (offset 7)
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0 (offset 11)
        0x74, 0x02, // JE +2 (exit loop) (offset 15)
        0xeb, 0xf5, // JMP -11 (back to DEC) (offset 17)
        0xf4, // HLT (offset 19)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Counter decremented to 0");
}

// JMP over data
#[test]
fn test_jmp_over_data() {
    let code = [
        0xeb, 0x08, // JMP +8 (skip data)
        0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, // data
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42, "Jumped over data successfully");
}

// JMP to exact next instruction (JMP +0)
#[test]
fn test_jmp_zero_offset() {
    let code = [
        0xeb, 0x00, // JMP +0 (next instruction)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99);
}

// Multiple JMPs in sequence
#[test]
fn test_multiple_jmps() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xeb, 0x07, // JMP +7 (to label1)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF (skipped)
        // label1:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xeb, 0x07, // JMP +7 (to label2)
        0x48, 0xc7, 0xc0, 0xee, 0x00, 0x00, 0x00, // MOV RAX, 0xEE (skipped)
        // label2:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "RAX = 1 + 1 + 1");
}

// JMP doesn't affect flags
#[test]
fn test_jmp_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xeb, 0x02, // JMP +2
        0xf4, 0xf4, // (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF preserved through JMP");
}

// JMP doesn't affect registers
#[test]
fn test_jmp_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0xeb, 0x07, // JMP +7
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "RAX preserved");
    assert_eq!(regs.rbx, 0x22, "RBX preserved");
}

// JMP doesn't affect stack
#[test]
fn test_jmp_preserves_stack() {
    let code = [
        0xeb, 0x02, // JMP +2
        0x50, 0x50, // PUSH RAX, PUSH RAX (skipped)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1000, "RSP unchanged");
}

// Long JMP (32-bit offset)
#[test]
fn test_jmp_long() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xe9, 0x80, 0x00, 0x00, 0x00, // JMP +128
    ];
    // Padding
    code.resize(12 + 128, 0x90); // NOP padding
    // Target:
    code.extend_from_slice(&[
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x43);
}

// JMP in conditional context (always taken)
#[test]
fn test_jmp_always_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0xeb, 0x07, // JMP +7 (always jumps, ignoring comparison)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5, "JMP always taken regardless of flags");
}

// Practical use case: skip error handling
#[test]
fn test_jmp_practical_skip_error() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (success)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x07, // JNZ +7 (skip error handling if success)
        // error handling:
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        // continue:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax as i64, -1, "Error handling executed");
}

// Practical use case: simple loop with counter
#[test]
fn test_jmp_practical_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (counter)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (limit)
        // loop_start:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x02, // JAE +2 (exit if RAX >= 5)
        0xeb, 0xf5, // JMP -11 (back to ADD)
        // loop_end:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5, "Looped 5 times");
}

// Practical use case: state machine
#[test]
fn test_jmp_practical_state_machine() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (state)
        0x48, 0x83, 0xf8, 0x01, // CMP RAX, 1
        0x75, 0x0e, // JNE +14 (not state 1)
        // state 1:
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (next state)
        0xeb, 0x0c, // JMP +12 (to end)
        // state 2:
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // end:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x11, "State 1 executed");
    assert_eq!(regs.rax, 2, "Transitioned to state 2");
}

// JMP to beginning of code
#[test]
fn test_jmp_to_start() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x74, 0x02, // JE +2 (exit)
        0xeb, 0xf5, // JMP -11 (back to ADD)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5);
}

// Nested jumps
#[test]
fn test_jmp_nested() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xeb, 0x07, // JMP +7 (to outer)
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA (skipped)
        // outer:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xeb, 0x07, // JMP +7 (to inner)
        0x48, 0xc7, 0xc0, 0xbb, 0x00, 0x00, 0x00, // MOV RAX, 0xBB (skipped)
        // inner:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "1 + 1 + 1");
}

// JMP with various offset encodings
#[test]
fn test_jmp_short_max_forward() {
    let mut code = vec![
        0xeb, 0x7f, // JMP +127 (max short jump forward)
    ];
    code.resize(2 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT at target

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jmp_short_max_backward() {
    // Layout: MOV(7) + DEC(4) + JNZ(2) + JMP(2) + JMP(2) + HLT(1)
    // Offsets: 0-6    7-10    11-12   13-14   15-16    17
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (offset 0-6)
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1 (offset 7-10)
        0x75, 0x02, // JNZ +2 (skip next JMP if not zero, offset 11-12)
        0xeb, 0x02, // JMP +2 (exit loop, offset 13-14)
        0xeb, 0xf6, // JMP -10 (back to SUB at offset 7, offset 15-16)
        0xf4, // HLT (offset 17)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
}

// JMP across NOP sled
#[test]
fn test_jmp_across_nops() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0xeb, 0x0a, // JMP +10 (skip NOPs)
    ];
    // 10 NOPs
    for _ in 0..10 {
        code.push(0x90); // NOP
    }
    code.extend_from_slice(&[
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xf4, // HLT
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x12);
}
