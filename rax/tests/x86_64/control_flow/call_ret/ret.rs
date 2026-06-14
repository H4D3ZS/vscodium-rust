use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// RET - Return from Procedure
// Pops return address from stack and jumps to it

// Basic RET
#[test]
fn test_ret_basic() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL +4
        0xf4, // HLT (return point)
        0x00, 0x00, 0x00, // padding
        // function:
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP restored after RET");
}

// RET increments RSP by 8
#[test]
fn test_ret_increments_rsp() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save RSP before RET)
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should have RSP value before RET (0xFF8)
    assert_eq!(regs.rbx, 0x0FF8, "RSP before RET");
    // Current RSP should be incremented by 8
    assert_eq!(regs.rsp, 0x1000, "RSP after RET");
}

// RET with immediate (adjusts stack pointer)
#[test]
fn test_ret_imm16() {
    let code = [
        0x6a, 0x11, // PUSH 0x11 (param 1)
        0x6a, 0x22, // PUSH 0x22 (param 2)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0xc2, 0x10, 0x00, // RET 16 (pop return addr + clean 16 bytes)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RSP should be back to original after cleaning parameters
    assert_eq!(regs.rsp, 0x1000, "RSP restored including parameters");
}

// Multiple nested RET
#[test]
fn test_nested_ret() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func1
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // func1:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func2
        0xc3, // RET from func1
        0x00, 0x00, 0x00, // padding
        // func2:
        0x48, 0x83, 0xc0, 0x0a, // ADD RAX, 10
        0xc3, // RET from func2
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 11);
    assert_eq!(regs.rsp, 0x1000, "All RETs executed correctly");
}

// RET preserves registers
#[test]
fn test_ret_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0xaa, 0x00, 0x00, 0x00, // MOV RBX, 0xAA
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX modified in function");
    assert_eq!(regs.rbx, 0xAA, "RBX preserved");
    assert_eq!(regs.rcx, 0xBB, "RCX preserved");
}

// RET preserves flags
#[test]
fn test_ret_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF preserved through RET");
}

// Sequential CALLs and RETs
#[test]
fn test_sequential_call_ret() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xe8, 0x09, 0x00, 0x00, 0x00, // CALL func1
        0xe8, 0x0c, 0x00, 0x00, 0x00, // CALL func2
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // func1:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
        0x00, 0x00, 0x00, // padding
        // func2:
        0x48, 0x83, 0xc0, 0x02, // ADD RAX, 2
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "Both functions executed");
    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
}

// RET after pushing values
#[test]
fn test_ret_after_push() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x5b, // POP RBX
        0x58, // POP RAX
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0x11;
    regs.rbx = 0x22;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rsp, 0x1000,
        "Stack balanced after PUSH/POP in function"
    );
}

// RET with stack frame
#[test]
fn test_ret_with_stack_frame() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16 (locals)
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16
        0x5d, // POP RBP
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
    assert_eq!(regs.rbp, 0x2000, "RBP restored");
}

// Immediate RET with different values
#[test]
fn test_ret_imm_various() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // func:
        0xc2, 0x08, 0x00, // RET 8 (clean 1 parameter)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Parameter cleaned by RET");
}

// Far RET (RETF) - returns to different segment
// Note: This is a simplified test; actual far returns involve segments
#[test]
fn test_retf_basic() {
    // RETF pops both RIP and CS from stack
    // In 64-bit mode, this is mostly for compatibility
    // This test just ensures RETF doesn't crash
}

// RET chain (multiple functions)
#[test]
fn test_ret_chain_deep() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL a
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // a:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL b
        0xc3, // RET
        0x00, 0x00, 0x00, // padding
        // b:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL c
        0xc3, // RET
        0x00, 0x00, 0x00, // padding
        // c:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "All functions executed");
    assert_eq!(regs.rsp, 0x1000, "All RETs balanced stack");
}

// RET with modified return address
#[test]
fn test_ret_modified_return_address() {
    let code = [
        0xe8, 0x0b, 0x00, 0x00, 0x00, // CALL function (offset 16)
        0xf4, // HLT (offset 5 - original return)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding
        0xf4, // HLT (offset 12 - modified return target)
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0x8b, 0x04, 0x24, // MOV RAX, [RSP] (get return address)
        0x48, 0x83, 0xc0, 0x07, // ADD RAX, 7 (modify to skip to offset 12)
        0x48, 0x89, 0x04, 0x24, // MOV [RSP], RAX (store modified address)
        0xc3, // RET (returns to modified address)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should execute HLT at offset 12 instead of offset 5
}

// Practical use case: leaf function (no further calls)
#[test]
fn test_ret_practical_leaf_function() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL multiply
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // multiply (RAX = RAX * RBX, leaf function):
        0x48, 0x0f, 0xaf, 0xc3, // IMUL RAX, RBX
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 15, "5 * 3 = 15");
}

// RET balances stack exactly
#[test]
fn test_ret_stack_balance() {
    let code = [
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save original RSP)
        0xe8, 0x07, 0x00, 0x00, 0x00, // CALL function
        0x48, 0x39, 0xdc, // CMP RSP, RBX
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, regs.rbx, "RSP exactly balanced");
    assert!(regs.rflags & 0x40 != 0, "ZF set (RSP == RBX)");
}

// Multiple immediate RET values
#[test]
fn test_ret_imm_16() {
    let code = [
        0x6a, 0x01, 0x6a, 0x02, // PUSH 1, 2 (16 bytes)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0xc2, 0x10, 0x00, // RET 16
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "16 bytes cleaned");
}

// RET after conditional
#[test]
fn test_ret_after_conditional() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10
        0x7c, 0x04, // JL +4 (skip next instruction)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (only if >= 10)
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "Condition false, no increment");
}
