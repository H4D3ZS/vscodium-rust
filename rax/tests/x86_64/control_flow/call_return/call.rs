use crate::common::{
    CODE_ADDR, VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm,
};

// CALL - Call Procedure
// Pushes return address (RIP) onto stack and jumps to target
// Opcodes:
//   E8 cw/cd - CALL rel16/rel32 (near, relative)
//   FF /2 - CALL r/m64 (near, absolute indirect)

#[test]
fn test_call_near_relative_basic() {
    let code = [
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL +4 (to function)
        // return point (offset 5):
        0x48, 0xff, 0xc0, // INC RAX
        0xf4, // HLT
        // function (offset 9):
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x43); // Function sets to 0x42, INC makes it 0x43
}

#[test]
fn test_call_pushes_return_address() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000 (setup stack)
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL +1 (to function)
        // return point (offset 12):
        0xf4, // HLT
        // function (offset 13):
        0x58, // POP RAX (get return address from stack)
        0xc3, // RET (will use popped address, causing infinite loop - but we check RAX)
        0xf4, // Won't reach here initially
    ];
    let vm = setup_vm(&code);
    // Run just until the POP RAX in the function
    let vm = vm.step().step().step(); // MOV, CALL, POP
    assert_eq!(vm.rax, 0x100C); // Return address should be right after CALL
}

#[test]
fn test_call_decrements_rsp() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL +1
        // return point:
        0xf4, // HLT
        // function:
        0x48, 0x89, 0xe0, // MOV RAX, RSP (save stack pointer)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x2000 - 8); // RSP should be decremented by 8 inside function
}

#[test]
fn test_call_ret_round_trip() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL +1
        // return point (offset 19):
        0xf4, // HLT
        // function (offset 20):
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
    assert_eq!(vm.rsp, 0x2000); // Stack should be balanced after return
    assert_eq!(vm.rip, CODE_ADDR + 20); // RIP advances past HLT
}

#[test]
fn test_call_nested_functions() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL func1
        // main_return:
        0xf4, // HLT
        // func1 (offset 13):
        0x48, 0xff, 0xc0, // INC RAX
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func2
        // func1_return:
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
        // func2 (offset 25):
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // INC in func1, INC in func2, INC after func2 returns
    assert_eq!(vm.rsp, 0x2000); // Stack balanced
}

#[test]
fn test_call_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x48, 0xc7, 0xc2, 0x33, 0x00, 0x00, 0x00, // MOV RDX, 0x33
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL function
        // return:
        0xf4, // HLT
        // function:
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
    assert_eq!(vm.rbx, 0x11); // Preserved
    assert_eq!(vm.rcx, 0x22); // Preserved
    assert_eq!(vm.rdx, 0x33); // Preserved
}

#[test]
fn test_call_forward() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL +4 (forward to function)
        // return point:
        0x48, 0xff, 0xc0, // INC RAX
        0xf4, // HLT
        // function:
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 6);
}

#[test]
fn test_call_backward() {
    let code = [
        0xeb, 0x08, // JMP +8 (skip function)
        // function (offset 2):
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
        // main (offset 10):
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0xec, 0xff, 0xff, 0xff, // CALL -20 (backward to function)
        // return point:
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
}

#[test]
fn test_call_with_arguments_in_registers() {
    // Simulate passing arguments via registers (calling convention)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc7, 0x05, 0x00, 0x00, 0x00, // MOV RDI, 5 (first arg)
        0x48, 0xc7, 0xc6, 0x03, 0x00, 0x00, 0x00, // MOV RSI, 3 (second arg)
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL add_function
        // return:
        0xf4, // HLT
        // add_function:
        0x48, 0x89, 0xf8, // MOV RAX, RDI
        0x48, 0x01, 0xf0, // ADD RAX, RSI
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 8); // 5 + 3
}

#[test]
fn test_call_with_local_variables() {
    // Function allocates stack space for locals
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL function
        // return:
        0xf4, // HLT
        // function:
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16 (allocate 16 bytes)
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16 (deallocate)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
    assert_eq!(vm.rsp, 0x2000); // Stack balanced
}

#[test]
fn test_call_indirect_register() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xb8, 0x14, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, func_addr
        0xff, 0xd0, // CALL RAX
        // return:
        0xf4, // HLT
        // function (at 0x1014):
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0x99);
}

#[test]
fn test_call_indirect_rbx() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xbb, 0x14, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, func_addr
        0xff, 0xd3, // CALL RBX
        // return:
        0xf4, // HLT
        // function (at 0x1014):
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
}

#[test]
fn test_call_indirect_rcx() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xb9, 0x14, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, func_addr
        0xff, 0xd1, // CALL RCX
        // return:
        0xf4, // HLT
        // function (at 0x1014):
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x77);
}

#[test]
fn test_call_function_pointer_table() {
    // Simulate calling through function pointer table
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (function index)
        0x48, 0xc1, 0xe0, 0x03, // SHL RAX, 3 (multiply by 8)
        0x48, 0x05, 0x1e, 0x10, 0x00, 0x00, // ADD RAX, table_base (0x101E)
        0x48, 0x8b, 0x00, // MOV RAX, [RAX] (load function pointer)
        0xff, 0xd0, // CALL RAX
        // return:
        0xf4, // HLT
        // function_table (at 0x101E):
        0x2e, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // func0 address
        0x36, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // func1 address
        // func0 (at 0x102E):
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xAA
        0xc3, // RET
        // func1 (at 0x1036):
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0xBB); // func1 was called
}

#[test]
fn test_call_recursive_countdown() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (counter)
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL countdown
        // main_return:
        0xf4, // HLT
        // countdown function (offset 20):
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x74, 0x08, // JZ +8 (return if zero)
        0x48, 0xff, 0xc8, // DEC RAX
        0xe8, 0xf3, 0xff, 0xff, 0xff, // CALL countdown (recursive)
        0xc3, // RET
        // base case:
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0); // Counted down to 0
    assert_eq!(vm.rsp, 0x2000); // Stack balanced after all returns
}

#[test]
fn test_call_recursive_factorial() {
    // Compute 5! = 120
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (input)
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL factorial
        // main_return:
        0xf4, // HLT
        // factorial(n) in RAX, returns result in RAX
        // factorial (offset 20):
        0x48, 0x83, 0xf8, 0x01, // CMP RAX, 1
        0x7e, 0x12, // JLE +18 (base case: return 1)
        // recursive case:
        0x53, // PUSH RBX
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save n)
        0x48, 0xff, 0xc8, // DEC RAX (n-1)
        0xe8, 0xee, 0xff, 0xff, 0xff, // CALL factorial
        0x48, 0x0f, 0xaf, 0xc3, // IMUL RAX, RBX (n * factorial(n-1))
        0x5b, // POP RBX
        0xc3, // RET
        // base case:
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 120); // 5! = 120
    assert_eq!(vm.rsp, 0x2000); // Stack balanced
}

#[test]
fn test_call_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF=1, CF=1)
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL function
        // return:
        0x74, 0x01, // JZ +1 (should jump if ZF still set)
        0xf4, // HLT (should not reach)
        0xf4, // HLT (target)
        // function:
        0xc3, // RET (does nothing, preserves flags)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, CODE_ADDR + 27);
}

#[test]
fn test_call_multiple_sequential() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xe8, 0x0b, 0x00, 0x00, 0x00, // CALL func1
        0xe8, 0x0b, 0x00, 0x00, 0x00, // CALL func2
        0xe8, 0x0b, 0x00, 0x00, 0x00, // CALL func3
        0xf4, // HLT
        // func1:
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0xc3, // RET
        // func2:
        0x48, 0x83, 0xc0, 0x02, // ADD RAX, 2
        0xc3, // RET
        // func3:
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 6); // 1 + 2 + 3
}

#[test]
fn test_call_stack_growth() {
    // Verify stack grows downward
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save initial SP)
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL function
        // return:
        0xf4, // HLT
        // function:
        0x48, 0x89, 0xe0, // MOV RAX, RSP (get SP inside function)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x2000); // Initial SP
    assert_eq!(vm.rax, 0x2000 - 8); // SP decremented by 8
}

#[test]
fn test_call_return_address_is_next_instruction() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x08, 0x00, 0x00, 0x00, // CALL function (at offset 7)
        // return point (offset 12):
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
        // function (offset 20):
        0x48, 0x8b, 0x04, 0x24, // MOV RAX, [RSP] (load return address)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x100C); // Return address = offset 12
    assert_eq!(vm.rcx, 0x42); // Execution continued after return
}

#[test]
fn test_call_deeply_nested() {
    // Test calling depth of 4
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL func1
        0xf4, // HLT
        // func1:
        0x48, 0xff, 0xc0, // INC RAX
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func2
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
        // func2:
        0x48, 0xff, 0xc0, // INC RAX
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func3
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
        // func3:
        0x48, 0xff, 0xc0, // INC RAX
        0xe8, 0x04, 0x00, 0x00, 0x00, // CALL func4
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
        // func4:
        0x48, 0xff, 0xc0, // INC RAX
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7); // 1 in each func + 1 after each return (except last)
    assert_eq!(vm.rsp, 0x2000); // Stack balanced
}

#[test]
fn test_call_with_stack_arguments() {
    // Pass arguments on stack (7th argument onwards in System V ABI)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x50, // PUSH RAX (stack argument)
        0xe8, 0x05, 0x00, 0x00, 0x00, // CALL function
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8 (clean up stack arg)
        0xf4, // HLT
        // function:
        0x48, 0x8b, 0x44, 0x24, 0x08, // MOV RAX, [RSP+8] (get stack arg, skip return addr)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 10); // Retrieved stack argument
}

#[test]
fn test_call_saves_and_restores_callee_saved() {
    // Callee-saved registers: RBX, RBP, R12-R15
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xe8, 0x01, 0x00, 0x00, 0x00, // CALL function
        0xf4, // HLT
        // function (must preserve RBX):
        0x53, // PUSH RBX
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42 (temp value)
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0x5b, // POP RBX (restore)
        0xc3, // RET
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42); // Function used RBX
    assert_eq!(vm.rbx, 0x99); // But RBX was preserved
}

#[test]
fn test_call_zero_offset() {
    // CALL to next instruction (odd but valid)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0xe8, 0x00, 0x00, 0x00, 0x00, // CALL +0 (next instruction)
        // Next instruction (also return point):
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8 (clean up return address)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42);
}
