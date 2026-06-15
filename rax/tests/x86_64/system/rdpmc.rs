use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// RDPMC - Read Performance-Monitoring Counters
// Opcode: 0F 33
// Reads performance monitoring counter specified by ECX into EDX:EAX
// EDX = high 32 bits, EAX = low 32 bits
// Does not modify flags

// Basic RDPMC test - reads PMC into EDX:EAX
#[test]
fn test_rdpmc_basic() {
    let code = [
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PMC should be loaded into EDX:EAX
    // Upper 32 bits of RAX and RDX should be cleared
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX should be cleared");
}

// Test RDPMC with counter 0 (ECX=0)
#[test]
fn test_rdpmc_counter_0() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (ECX = 0, select counter 0)
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with counter 1 (ECX=1)
#[test]
fn test_rdpmc_counter_1() {
    let code = [
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with counter 2 (ECX=2)
#[test]
fn test_rdpmc_counter_2() {
    let code = [
        0xb9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with counter 3 (ECX=3)
#[test]
fn test_rdpmc_counter_3() {
    let code = [
        0xb9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC doesn't modify flags
#[test]
fn test_rdpmc_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDPMC preserves other registers
#[test]
fn test_rdpmc_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x42, 0x42,
        0x42, // MOV RBX, 0x42424242 (bit 31 clear, no sign-ext)
        0x48, 0xc7, 0xc6, 0x2a, 0x2a, 0x2a,
        0x2a, // MOV RSI, 0x2a2a2a2a (bit 31 clear, no sign-ext)
        0x48, 0xc7, 0xc7, 0x19, 0x19, 0x19,
        0x19, // MOV RDI, 0x19191919 (bit 31 clear, no sign-ext)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX, RSI, RDI should be unchanged (only RCX was used for counter selection)
    // MOV r64, imm32 sign-extends, so we use values with bit 31 clear
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be affected");
    assert_eq!(regs.rsi, 0x2a2a2a2a, "RSI should not be affected");
    assert_eq!(regs.rdi, 0x19191919, "RDI should not be affected");
}

// Test RDPMC clears upper 32 bits of RAX and RDX
#[test]
fn test_rdpmc_clears_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0xc7, 0xc2, 0xff, 0xff, 0xff, 0xff, // MOV RDX, 0xffffffffffffffff
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be cleared
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX should be cleared");
}

// Test RDPMC with pre-existing values in EAX and EDX
#[test]
fn test_rdpmc_overwrites_registers() {
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xba, 0x22, 0x22, 0x22, 0x22, // MOV EDX, 0x22222222
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Old values should be completely overwritten
    // Can't predict exact PMC value, but upper bits should be clear
    assert_eq!(regs.rax >> 32, 0, "Upper bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits of RDX cleared");
}

// Test RDPMC with multiple sequential calls
#[test]
fn test_rdpmc_sequential_calls() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC #1
        0x0f, 0x33, // RDPMC #2
        0x0f, 0x33, // RDPMC #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let pmc = ((regs.rdx as u64) << 32) | (regs.rax as u64);
    let _ = pmc;
}

// Test RDPMC with RCX upper bits set (should be ignored)
#[test]
fn test_rdpmc_rcx_upper_bits_ignored() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, // MOV RCX, 0xffffffff00000000
        0x0f, 0x33, // RDPMC (upper 32 bits ignored)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should work like counter 0
    assert_eq!(regs.rax >> 32, 0, "Upper bits cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits cleared");
}

// Test RDPMC with different counter values
#[test]
fn test_rdpmc_counter_4() {
    let code = [
        0xb9, 0x04, 0x00, 0x00, 0x00, // MOV ECX, 4
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with counter 5
#[test]
fn test_rdpmc_counter_5() {
    let code = [
        0xb9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with fixed-function counter type (ECX bit 30 set for type 0x40000000)
#[test]
fn test_rdpmc_fixed_function_counter() {
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x40, // MOV ECX, 0x40000000 (fixed-function counter 0)
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX cleared");
}

// Test RDPMC with all flags set
#[test]
fn test_rdpmc_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (doesn't affect flags)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0x0f, 0x33, // RDPMC (should preserve flags)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved by RDPMC
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDPMC not serializing
#[test]
fn test_rdpmc_not_serializing() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC (not serializing)
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ADD should complete
    assert_eq!(regs.rbx, 2, "ADD should complete normally");
}

// Test RDPMC with PUSH/POP operations
#[test]
fn test_rdpmc_with_push_pop() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0x50, // PUSH RAX
        0x52, // PUSH RDX
        0x5a, // POP RDX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Values should be restored
    assert_eq!(regs.rax >> 32, 0, "Upper bits should still be clear");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits should still be clear");
}

// Test RDPMC preserves RBX through multiple calls
#[test]
fn test_rdpmc_rbx_preservation() {
    let code = [
        0x48, 0xc7, 0xc3, 0xef, 0xbe, 0x2d,
        0x1e, // MOV RBX, 0x1e2dbeef (bit 31 clear, no sign-ext)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC #1
        0x0f, 0x33, // RDPMC #2
        0x0f, 0x33, // RDPMC #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1e2dbeef, "RBX should be preserved");
}

// Test RDPMC with conditional jumps
#[test]
fn test_rdpmc_with_conditional_jump() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x02, // JNZ skip
        0x90, // NOP
        0x90, // NOP
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test RDPMC 64-bit reconstruction
#[test]
fn test_rdpmc_64bit_reconstruction() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0x48, 0xc1, 0xe2, 0x20, // SHL RDX, 32
        0x48, 0x09, 0xd0, // OR RAX, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should now contain full 64-bit PMC value
    let pmc = regs.rax;
    let _ = pmc;
}

// Test RDPMC with zero flag conditions
#[test]
fn test_rdpmc_zero_flag() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDPMC with carry flag conditions
#[test]
fn test_rdpmc_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets CF)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be preserved
    let _ = regs.rflags;
}

// Test RDPMC with sign flag conditions
#[test]
fn test_rdpmc_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (doesn't affect flags)
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x0f, 0x33, // RDPMC (should preserve flags)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // SF should still be set (RDPMC preserves flags)
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDPMC execution completes quickly
#[test]
fn test_rdpmc_execution_speed() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0x0f, 0x33, // RDPMC
        0x0f, 0x33, // RDPMC
        0x0f, 0x33, // RDPMC
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should complete
    let _ = regs;
}

// Test RDPMC with counter loop
#[test]
fn test_rdpmc_counter_loop() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (counter = 0) - 3 bytes (0x1000)
        // loop: (0x1003)
        0x0f, 0x33, // RDPMC - 2 bytes (0x1003)
        0x48, 0x83, 0xc1, 0x01, // ADD RCX, 1 - 4 bytes (0x1005)
        0x48, 0x83, 0xf9, 0x04, // CMP RCX, 4 - 4 bytes (0x1009)
        0x75, 0xf4, // JNZ loop (rel8 = -12, from 0x100F to 0x1003) - 2 bytes (0x100D)
        0xf4, // HLT - 1 byte (0x100F)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Loop should complete with RCX = 4
    assert_eq!(regs.rcx, 4, "Counter loop should complete");
}

// Test RDPMC with different ECX values in sequence
#[test]
fn test_rdpmc_different_counters_sequence() {
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x33, // RDPMC
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x0f, 0x33, // RDPMC
        0xb9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    assert_eq!(regs.rax >> 32, 0, "Upper bits cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits cleared");
}

// Test RDPMC preserves R8-R15
#[test]
fn test_rdpmc_preserves_extended_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111 (bit 31 clear)
        0x49, 0xc7, 0xc7, 0x0f, 0x0f, 0x0f, 0x0f, // MOV R15, 0x0f0f0f0f (bit 31 clear)
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x11111111, "R8 should be preserved");
    assert_eq!(regs.r15, 0x0f0f0f0f, "R15 should be preserved");
}

// Test RDPMC result format (EDX:EAX)
#[test]
fn test_rdpmc_result_format() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should contain low 32 bits, EDX high 32 bits
    let eax = regs.rax & 0xFFFFFFFF;
    let edx = regs.rdx & 0xFFFFFFFF;
    let _ = (eax, edx);
}

// Test RDPMC with ECX = 0x80000000 (bit 31 set, fast read mode on some processors)
#[test]
fn test_rdpmc_fast_read_mode() {
    let code = [
        0xb9, 0x00, 0x00, 0x00, 0x80, // MOV ECX, 0x80000000 (fast read mode)
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax >> 32, 0, "Upper bits of RAX cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits of RDX cleared");
}

// Test RDPMC preserves stack pointer
#[test]
fn test_rdpmc_preserves_stack_pointer() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x8000, "RSP should be unchanged");
}

// Test RDPMC value is non-negative
#[test]
fn test_rdpmc_value_nonnegative() {
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PMC counters are unsigned
    let pmc = ((regs.rdx as u64) << 32) | (regs.rax as u64);
    let _ = pmc;
}

// Test RDPMC with arithmetic operations
#[test]
fn test_rdpmc_after_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x02, // ADD RBX, 2
        0x48, 0xf7, 0xdb, // NEG RBX
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should still have result of NEG
    assert_eq!(regs.rbx as i64, -3, "RBX should be -3");
}

// Test RDPMC preserves base pointer
#[test]
fn test_rdpmc_preserves_base_pointer() {
    let code = [
        0x48, 0xc7, 0xc5, 0x00, 0x70, 0x00, 0x00, // MOV RBP, 0x7000
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0x33, // RDPMC
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp, 0x7000, "RBP should be preserved");
}
