use rax::cpu::Registers;

use crate::common::*;

// PUSH - Push Value onto Stack
// Decrements RSP and stores value at new RSP location

// Basic PUSH register (64-bit)
#[test]
fn test_push_rax() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x22, 0x33, 0x44, // MOV RAX, 0x44332211
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8, "RSP should be decremented by 8");

    // Read value from stack
    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(0x0FF8)).unwrap();
    let pushed_val = u64::from_le_bytes(stack_val);
    assert_eq!(pushed_val, 0x44332211, "Pushed value should be on stack");
}

// PUSH different registers
#[test]
fn test_push_rbx() {
    let code = [
        0x48, 0xc7, 0xc3, 0xaa, 0xbb, 0xcc, 0xdd, // MOV RBX, 0xDDCCBBAA
        0x53, // PUSH RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x2000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1FF8, "RSP decremented by 8");

    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(0x1FF8)).unwrap();
    // MOV r64, imm32 sign-extends: 0xDDCCBBAA (bit 31 set) -> 0xFFFFFFFFDDCCBBAA
    assert_eq!(
        u64::from_le_bytes(stack_val),
        0xFFFFFFFFDDCCBBAA,
        "RBX value on stack"
    );
}

#[test]
fn test_push_rcx() {
    let code = [0x51, 0xf4]; // PUSH RCX, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rcx = 0x1234567890ABCDEF;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8, "RSP decremented");

    let mut stack_val = [0u8; 8];
    vm.read_slice(&mut stack_val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(stack_val), 0x1234567890ABCDEF);
}

// PUSH all general purpose registers
#[test]
fn test_push_all_gp_registers() {
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    regs.rdx = 0x4444444444444444;
    regs.rsi = 0x5555555555555555;
    regs.rdi = 0x6666666666666666;
    regs.rbp = 0x7777777777777777;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1000 - 7 * 8, "RSP decremented by 7*8 = 56");

    // Verify each value on stack
    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x1000 - 8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x1111111111111111, "RAX");
    vm.read_slice(&mut val, GuestAddress(0x1000 - 16)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x2222222222222222, "RBX");
    vm.read_slice(&mut val, GuestAddress(0x1000 - 24)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x3333333333333333, "RCX");
}

// PUSH extended registers (R8-R15)
#[test]
fn test_push_r8() {
    let code = [0x41, 0x50, 0xf4]; // PUSH R8, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r8 = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8);

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xAAAAAAAAAAAAAAAA);
}

#[test]
fn test_push_r15() {
    let code = [0x41, 0x57, 0xf4]; // PUSH R15, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r15 = 0xBBBBBBBBBBBBBBBB;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8);

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xBBBBBBBBBBBBBBBB);
}

// PUSH immediate values
#[test]
fn test_push_imm8() {
    let code = [0x6a, 0x42, 0xf4]; // PUSH 0x42 (sign-extended), HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8);

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x42);
}

#[test]
fn test_push_imm8_negative() {
    let code = [0x6a, 0xff, 0xf4]; // PUSH -1 (0xFF sign-extended), HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8);

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_push_imm32() {
    let code = [0x68, 0x78, 0x56, 0x34, 0x12, 0xf4]; // PUSH 0x12345678, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8);

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x12345678);
}

// PUSH with zero value
#[test]
fn test_push_zero() {
    let code = [0x6a, 0x00, 0xf4]; // PUSH 0, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0);
}

// Multiple PUSH operations
#[test]
fn test_multiple_push() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1000 - 24, "RSP decremented by 3*8");

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x1000 - 8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x11, "First push (RAX)");
    vm.read_slice(&mut val, GuestAddress(0x1000 - 16)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x22, "Second push (RBX)");
    vm.read_slice(&mut val, GuestAddress(0x1000 - 24)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x33, "Third push (RCX)");
}

// PUSH preserves flags
#[test]
fn test_push_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set");
}

// PUSH RSP (special case - pushes value before decrement)
#[test]
fn test_push_rsp() {
    let code = [0x54, 0xf4]; // PUSH RSP, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x0FF8, "RSP decremented");

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x1000, "Original RSP value pushed");
}

// Test stack grows downward
#[test]
fn test_stack_grows_down() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1000 - 24, "Stack pointer decreased");

    // Most recent push is at lowest address
    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(regs.rsp)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 3, "Top of stack is 3");
}

// PUSH with maximum value
#[test]
fn test_push_max_value() {
    let code = [0x50, 0xf4]; // PUSH RAX, HLT
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xFFFFFFFFFFFFFFFF);
}

// PUSH followed by modification doesn't affect stack
#[test]
fn test_push_then_modify() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99, "RAX modified");

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x42, "Stack has original value");
}

// Practical use case: function prologue
#[test]
fn test_push_practical_function_prologue() {
    let code = [
        0x55, // PUSH RBP (save old base pointer)
        0x48, 0x89, 0xe5, // MOV RBP, RSP (set new base pointer)
        0x50, // PUSH RAX (save RAX)
        0x53, // PUSH RBX (save RBX)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = 0x2000;
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbp, 0x0FF8, "RBP set to RSP after first push");
    assert_eq!(regs.rsp, 0x0FF8 - 16, "RSP after pushing RAX and RBX");

    // Verify saved values
    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(0x0FF8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x2000, "Old RBP saved");
}

// Test with small stack space
#[test]
fn test_push_near_stack_bottom() {
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x10;
    regs.rax = 0xAAAA;
    regs.rbx = 0xBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x10 - 16, "RSP at near-zero address");
}

// Chain of pushes and verify order
#[test]
fn test_push_chain_order() {
    let code = [
        0x6a, 0x0a, // PUSH 10
        0x6a, 0x14, // PUSH 20
        0x6a, 0x1e, // PUSH 30
        0x6a, 0x28, // PUSH 40
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Stack layout (top to bottom): 40, 30, 20, 10
    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(regs.rsp)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 40, "Top of stack");
    vm.read_slice(&mut val, GuestAddress(regs.rsp + 8)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 30);
    vm.read_slice(&mut val, GuestAddress(regs.rsp + 16))
        .unwrap();
    assert_eq!(u64::from_le_bytes(val), 20);
    vm.read_slice(&mut val, GuestAddress(regs.rsp + 24))
        .unwrap();
    assert_eq!(u64::from_le_bytes(val), 10, "Bottom of our pushes");
}

// PUSH with all extended registers
#[test]
fn test_push_all_extended_regs() {
    let code = [
        0x41, 0x50, // PUSH R8
        0x41, 0x51, // PUSH R9
        0x41, 0x52, // PUSH R10
        0x41, 0x53, // PUSH R11
        0x41, 0x54, // PUSH R12
        0x41, 0x55, // PUSH R13
        0x41, 0x56, // PUSH R14
        0x41, 0x57, // PUSH R15
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r8 = 0x08;
    regs.r9 = 0x09;
    regs.r10 = 0x0A;
    regs.r11 = 0x0B;
    regs.r12 = 0x0C;
    regs.r13 = 0x0D;
    regs.r14 = 0x0E;
    regs.r15 = 0x0F;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x1000 - 64, "8 registers * 8 bytes");

    let mut val = [0u8; 8];
    vm.read_slice(&mut val, GuestAddress(regs.rsp)).unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x0F, "R15 on top");
}

// ============================================================================
// Strengthened PUSH tests (appended): exact RSP delta, exact bytes written at
// the new top-of-stack, immediate sign-extension, memory source, and 16-bit
// operand-size override (RSP -= 2 with exactly 2 bytes written).
// ============================================================================

#[test]
fn test_strict_push_r64_rsp_delta_and_bytes() {
    // PUSH RAX: RSP -= 8, [new RSP] = RAX (8 LE bytes).
    let code = [0x50, 0xf4]; // PUSH RAX
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rax = 0x1122_3344_5566_7788;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 8, "PUSH r64 decrements RSP by 8");
    assert_eq!(
        read_mem_at_u64(&mem, 0x4000 - 8),
        0x1122_3344_5566_7788,
        "exact value at TOS"
    );
}

#[test]
fn test_strict_push_imm32_sign_extended() {
    // PUSH imm32 (0x68): -1 (0xFFFFFFFF) sign-extended to 64 bits, RSP -= 8.
    let code = [0x68, 0xff, 0xff, 0xff, 0xff, 0xf4]; // PUSH -1
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 8, "PUSH imm32 decrements RSP by 8");
    assert_eq!(
        read_mem_at_u64(&mem, 0x4000 - 8),
        0xFFFF_FFFF_FFFF_FFFF,
        "imm32 sign-extended on stack"
    );
}

#[test]
fn test_strict_push_imm8_sign_extended() {
    // PUSH imm8 (0x6A): 0x80 (-128) sign-extended to 64 bits.
    let code = [0x6a, 0x80, 0xf4]; // PUSH -128
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 8);
    assert_eq!(
        read_mem_at_u64(&mem, 0x4000 - 8),
        0xFFFF_FFFF_FFFF_FF80,
        "imm8 sign-extended"
    );
}

#[test]
fn test_strict_push_mem64() {
    // PUSH qword [RBX] (FF /6): push memory operand.
    let code = [0xff, 0x33, 0xf4]; // PUSH [RBX]
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0xABCD_1234_5678_9ABC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 8);
    assert_eq!(
        read_mem_at_u64(&mem, 0x4000 - 8),
        0xABCD_1234_5678_9ABC,
        "memory value pushed"
    );
}

#[test]
fn test_strict_push_r16_operand_size() {
    // PUSH AX (0x66 prefix): RSP -= 2, exactly 2 bytes written.
    let code = [0x66, 0x50, 0xf4]; // PUSH AX
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.rax = 0x0000_0000_0000_BEEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 2, "16-bit PUSH decrements RSP by 2");
    assert_eq!(
        read_mem_at_u16(&mem, 0x4000 - 2),
        0xBEEF,
        "exact 16-bit value at TOS"
    );
}

#[test]
fn test_strict_push_extended_reg_r12() {
    // PUSH R12 (REX.B 0x54): RSP -= 8, value at TOS.
    let code = [0x41, 0x54, 0xf4]; // PUSH R12
    let mut regs = Registers::default();
    regs.rsp = 0x4000;
    regs.r12 = 0xFEED_FACE_DEAD_C0DE;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, 0x4000 - 8);
    assert_eq!(
        read_mem_at_u64(&mem, 0x4000 - 8),
        0xFEED_FACE_DEAD_C0DE,
        "R12 pushed"
    );
}
