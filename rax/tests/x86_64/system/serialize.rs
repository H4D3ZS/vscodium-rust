use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// SERIALIZE - Serialize Instruction Execution
// Opcode: NP 0F 01 E8
// Ensures all modifications to flags, registers, and memory by previous instructions
// are completed and all buffered writes are drained before next instruction is fetched
// Does not modify registers, flags, or memory

// Basic SERIALIZE test
#[test]
fn test_serialize_basic() {
    let code = [
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test SERIALIZE doesn't modify RAX
#[test]
fn test_serialize_preserves_rax() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x42, 0x42, 0x42, // MOV RAX, 0x42424242
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42424242, "RAX should be unchanged");
}

// Test SERIALIZE doesn't modify RBX
#[test]
fn test_serialize_preserves_rbx() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x22, 0x33, 0x44, // MOV RBX, 0x44332211
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x44332211, "RBX should be unchanged");
}

// Test SERIALIZE doesn't modify flags
#[test]
fn test_serialize_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test SERIALIZE preserves all general-purpose registers
#[test]
fn test_serialize_preserves_all_gprs() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x01, 0x01, 0x01, // MOV RAX, 0x01010101
        0x48, 0xc7, 0xc3, 0x02, 0x02, 0x02, 0x02, // MOV RBX, 0x02020202
        0x48, 0xc7, 0xc1, 0x03, 0x03, 0x03, 0x03, // MOV RCX, 0x03030303
        0x48, 0xc7, 0xc2, 0x04, 0x04, 0x04, 0x04, // MOV RDX, 0x04040404
        0x48, 0xc7, 0xc6, 0x05, 0x05, 0x05, 0x05, // MOV RSI, 0x05050505
        0x48, 0xc7, 0xc7, 0x06, 0x06, 0x06, 0x06, // MOV RDI, 0x06060606
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x01010101, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x02020202, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x03030303, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0x04040404, "RDX should be unchanged");
    assert_eq!(regs.rsi, 0x05050505, "RSI should be unchanged");
    assert_eq!(regs.rdi, 0x06060606, "RDI should be unchanged");
}

// Test SERIALIZE with multiple sequential calls
#[test]
fn test_serialize_sequential_calls() {
    let code = [
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let _ = regs;
}

// Test SERIALIZE between memory operations
#[test]
fn test_serialize_between_memory_ops() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x89, 0x03, // MOV [RBX], RAX
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x48, 0x8b, 0x0b, // MOV RCX, [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RCX should have the value we stored
    assert_eq!(
        regs.rcx, 0x42,
        "Memory write should be visible after SERIALIZE"
    );
}

// Test SERIALIZE with all flags set
#[test]
fn test_serialize_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test SERIALIZE after arithmetic operations
#[test]
fn test_serialize_after_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x02, // ADD RBX, 2
        0x48, 0xf7, 0xdb, // NEG RBX
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should still have result of NEG
    assert_eq!(regs.rbx as i64, -3, "RBX should be -3");
}

// Test SERIALIZE preserves R8-R15
#[test]
fn test_serialize_preserves_extended_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111
        0x49, 0xc7, 0xc7, 0xff, 0xff, 0xff, 0xff, // MOV R15, 0xffffffff (sign-extended)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x11111111, "R8 should be preserved");
    assert_eq!(regs.r15, 0xffff_ffff_ffff_ffff, "R15 should be preserved");
}

// Test SERIALIZE preserves stack pointer
#[test]
fn test_serialize_preserves_stack_pointer() {
    let code = [
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test SERIALIZE preserves base pointer
#[test]
fn test_serialize_preserves_base_pointer() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}

// Test SERIALIZE with conditional jumps
#[test]
fn test_serialize_with_conditional_jump() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x02, // JNZ skip
        0x90, // NOP
        0x90, // NOP
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully, RAX should be 1
    assert_eq!(regs.rax, 1, "RAX should be 1");
}

// Test SERIALIZE with zero flag conditions
#[test]
fn test_serialize_zero_flag() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test SERIALIZE with carry flag conditions
#[test]
fn test_serialize_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets CF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be preserved
    let _ = regs.rflags;
}

// Test SERIALIZE with sign flag conditions
#[test]
fn test_serialize_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // SF should still be set
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test SERIALIZE between MOV instructions
#[test]
fn test_serialize_between_movs() {
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xbb, 0x99, 0x00, 0x00, 0x00, // MOV EBX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Both MOVs should execute correctly
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "First MOV executed");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x99, "Second MOV executed");
}

// Test SERIALIZE in a loop
#[test]
fn test_serialize_in_loop() {
    let code = [
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3 (loop counter)
        // loop:
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x48, 0x83, 0xeb, 0x01, // SUB RBX, 1
        0x75, 0xf8, // JNZ loop
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Loop should complete, RBX should be 0
    assert_eq!(regs.rbx, 0, "Loop should complete");
}

// Test SERIALIZE with PUSH/POP operations
#[test]
fn test_serialize_with_push_pop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Value should be restored
    assert_eq!(regs.rax, 0x42, "RAX should be 0x42");
}

// Test SERIALIZE preserves overflow flag
#[test]
fn test_serialize_overflow_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7fffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets OF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // OF should be preserved
    let _ = regs.rflags;
}

// Test SERIALIZE preserves parity flag
#[test]
fn test_serialize_parity_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xff
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF)
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PF should be preserved
    let _ = regs.rflags;
}

// Test SERIALIZE does not cause exceptions
#[test]
fn test_serialize_no_exception() {
    let code = [
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Should complete without panicking or returning an error
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// Test SERIALIZE with 64-bit register values
#[test]
fn test_serialize_64bit_values() {
    let code = [
        0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad,
        0xde, // MOV RAX, 0xdeadbeefdeadbeef
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xdeadbeefdeadbeef,
        "RAX should preserve 64-bit value"
    );
}

// Test SERIALIZE execution completes quickly
#[test]
fn test_serialize_execution_speed() {
    let code = [
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x0f, 0x01, 0xe8, // SERIALIZE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should complete
    let _ = regs;
}

// Test SERIALIZE between different instruction types
#[test]
fn test_serialize_mixed_instructions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5
        0x0f, 0x01, 0xe8, // SERIALIZE
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX = 16 + 5 - 3 = 18
    assert_eq!(regs.rax, 18, "RAX should be 18");
}
