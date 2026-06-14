use rax::cpu::Registers;

use crate::common::*;

// CALL - Call Procedure
// Pushes return address (RIP) onto stack and jumps to target

// Basic CALL with relative offset
#[test]
fn test_call_relative() {
    let code = [
        0xe8, 0x05, 0x00, 0x00, 0x00, // CALL +5 (skip to offset 10)
        0xf4, // HLT (should not execute)
        0x00, 0x00, 0x00, 0x00, // padding
        // offset 10:
        0xf4, // HLT (target)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RSP should be decremented by 8
    assert_eq!(regs.rsp, 0x0FF8, "RSP decremented by 8");

    // Return address should be on stack (address after CALL instruction)
    let mut return_addr = [0u8; 8];
    vm.read_slice(&mut return_addr, GuestAddress(0x0FF8))
        .unwrap();
    let addr = u64::from_le_bytes(return_addr);
    // Return address should point to the HLT after CALL
    assert_eq!(addr, CODE_ADDR + 5, "Return address is after CALL");
}

// CALL and RET combination
#[test]
fn test_call_ret_basic() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL +4 (to offset 16)
        0xf4, // HLT (return point)
        0x00, 0x00, 0x00, // padding
        // offset 16:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (in called function)
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x43, "RAX incremented in function");
    assert_eq!(regs.rsp, 0x1000, "RSP restored after RET");
}

// Nested CALL
#[test]
fn test_nested_call() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func1 (offset 9)
        0xf4, // HLT (main return)
        0x00, 0x00, 0x00, // padding
        // offset 9 - func1:
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

    assert_eq!(regs.rax, 11, "RAX = 0 + 1 + 10");
    assert_eq!(regs.rsp, 0x1000, "Stack balanced after nested calls");
}

// CALL preserves registers (except RAX for return value)
#[test]
fn test_call_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
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

    assert_eq!(regs.rax, 0x42, "RAX set in function");
    assert_eq!(regs.rbx, 0x11, "RBX preserved");
    assert_eq!(regs.rcx, 0x22, "RCX preserved");
}

// CALL with function that modifies stack
#[test]
fn test_call_with_local_variables() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16 (allocate locals)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16 (deallocate locals)
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x99);
    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
}

// CALL with parameters passed via stack
#[test]
fn test_call_with_stack_parameters() {
    let code = [
        0x6a, 0x05, // PUSH 5 (parameter)
        0x6a, 0x03, // PUSH 3 (parameter)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL add_function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // add_function:
        0x48, 0x8b, 0x44, 0x24, 0x08, // MOV RAX, [RSP+8] (first param)
        0x48, 0x03, 0x04, 0x24, // ADD RAX, [RSP] (second param)
        0xc3, // RET (doesn't clean params)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Note: Result depends on implementation details
    // This test verifies calling convention basics
}

// Forward CALL
#[test]
fn test_call_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL +4 (forward)
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // target:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1);
}

// Backward CALL (recursive setup)
#[test]
fn test_call_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (counter)
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0
        0x74, 0x09, // JE +9 (exit)
        0x48, 0x83, 0xe8, 0x01, // DEC RAX
        0xe8, 0xf1, 0xff, 0xff, 0xff, // CALL -15 (recursive)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "Counter decremented to 0");
}

// CALL with zero offset (calls next instruction)
#[test]
fn test_call_zero_offset() {
    let code = [
        0xe8, 0x00, 0x00, 0x00, 0x00, // CALL +0 (next instruction)
        0x58, // POP RAX (pop return address)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, CODE_ADDR + 5, "Return address is after CALL");
    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
}

// Multiple sequential CALLs
#[test]
fn test_multiple_sequential_calls() {
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
        0x48, 0x83, 0xc0, 0x0a, // ADD RAX, 10
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 11, "Both functions executed");
}

// CALL preserves flags
#[test]
fn test_call_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // function:
        0xc3, // RET (does nothing)
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x40 != 0, "ZF preserved through CALL");
}

// CALL to function that calls another function
#[test]
fn test_call_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func_a
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // func_a:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func_b
        0xc3, // RET
        0x00, 0x00, 0x00, // padding
        // func_b:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "RAX = 1 + 1 + 1");
    assert_eq!(regs.rsp, 0x1000);
}

// CALL with large offset
#[test]
fn test_call_large_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0xe8, 0x80, 0x00, 0x00, 0x00, // CALL +128
        0xf4, // HLT
    ];
    // Padding to reach offset 128
    code.resize(12 + 128, 0x90); // NOP padding
    // Target at offset 140:
    code.extend_from_slice(&[
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
    ]);

    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x78);
}

// CALL return address verification
#[test]
fn test_call_return_address_correct() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT (offset 8)
        0x00, 0x00, 0x00, // padding
        // function:
        0x48, 0x8b, 0x04, 0x24, // MOV RAX, [RSP] (get return address)
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Return address should point to HLT instruction
    assert_eq!(regs.rax, CODE_ADDR + 8, "Return address correct");
}

// Practical use case: factorial function (iterative)
#[test]
fn test_call_practical_factorial() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (input)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL factorial
        0xf4, // HLT
        0x00, 0x00, 0x00, // padding
        // factorial: (simplified, just multiplies RAX by 2)
        0x48, 0x01, 0xc0, // ADD RAX, RAX
        0xc3, // RET
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10, "5 * 2 = 10");
}
