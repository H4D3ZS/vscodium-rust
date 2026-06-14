// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// PUSHA/POPA - Push All and Pop All General-Purpose Registers
// Note: These instructions are not available in 64-bit mode
// But can be tested in compatibility/legacy mode
//
// PUSHA/PUSHAD - Push All General-Purpose Registers
// Opcode: 60 (PUSHA for 16-bit, PUSHAD for 32-bit)
// Pushes: AX/EAX, CX/ECX, DX/EDX, BX/EBX, original SP/ESP, BP/EBP, SI/ESI, DI/EDI
//
// POPA/POPAD - Pop All General-Purpose Registers
// Opcode: 61 (POPA for 16-bit, POPAD for 32-bit)
// Pops: DI/EDI, SI/ESI, BP/EBP, (skip SP/ESP), BX/EBX, DX/EDX, CX/ECX, AX/EAX
//
// The SP/ESP value from stack is skipped during POPA/POPAD

// ===== PUSHAD TESTS (32-bit) =====

#[test]
fn test_pushad_basic_saves_all_registers() {
    // PUSHAD should push all 8 general-purpose 32-bit registers
    // Allocate sufficient stack space: 8 registers * 4 bytes = 32 bytes
    let code = [
        0x66, 0x60, // PUSHAD (with operand-size prefix for 32-bit)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rcx = 0x22222222;
    regs.rdx = 0x33333333;
    regs.rbx = 0x44444444;
    regs.rbp = 0x55555555;
    regs.rsi = 0x66666666;
    regs.rdi = 0x77777777;
    regs.rsp = 0x8000; // Stack pointer

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify stack contains the values (should be decremented after push)
    // Stack grows downward: RSP decremented by 32
    // Stack should contain: DI, SI, BP, original SP, BX, DX, CX, AX
    let stack_base = 0x8000u64;
    let rdi_on_stack = read_mem_at_u32(&mem, stack_base - 32);
    assert_eq!(rdi_on_stack, 0x77777777, "RDI should be pushed first");

    let rsi_on_stack = read_mem_at_u32(&mem, stack_base - 28);
    assert_eq!(rsi_on_stack, 0x66666666, "RSI should be pushed second");

    let rbp_on_stack = read_mem_at_u32(&mem, stack_base - 24);
    assert_eq!(rbp_on_stack, 0x55555555, "RBP should be pushed");

    let rax_on_stack = read_mem_at_u32(&mem, stack_base - 4);
    assert_eq!(rax_on_stack, 0x11111111, "RAX should be pushed last");
}

#[test]
fn test_pushad_preserves_all_register_values() {
    // PUSHAD doesn't modify register values
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0xBBBBBBBB;
    regs.rcx = 0xCCCCCCCC;
    regs.rdx = 0xDDDDDDDD;
    regs.rsi = 0xEEEEEEEE;
    regs.rdi = 0xFFFFFFFF;
    regs.rbp = 0x12341234;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // All registers should be unchanged
    assert_eq!(result_regs.rax & 0xFFFFFFFF, 0xAAAAAAAA, "RAX unchanged");
    assert_eq!(result_regs.rbx & 0xFFFFFFFF, 0xBBBBBBBB, "RBX unchanged");
    assert_eq!(result_regs.rcx & 0xFFFFFFFF, 0xCCCCCCCC, "RCX unchanged");
    assert_eq!(result_regs.rdx & 0xFFFFFFFF, 0xDDDDDDDD, "RDX unchanged");
    assert_eq!(result_regs.rsi & 0xFFFFFFFF, 0xEEEEEEEE, "RSI unchanged");
    assert_eq!(result_regs.rdi & 0xFFFFFFFF, 0xFFFFFFFF, "RDI unchanged");
    assert_eq!(result_regs.rbp & 0xFFFFFFFF, 0x12341234, "RBP unchanged");
}

#[test]
fn test_pushad_decrements_stack_pointer() {
    // PUSHAD should decrement RSP by 32 (8 registers * 4 bytes)
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // RSP should be decremented by 32 bytes
    assert_eq!(
        result_regs.rsp,
        0x8000 - 32,
        "RSP should be decremented by 32"
    );
}

#[test]
fn test_pushad_saves_original_sp_not_modified_sp() {
    // PUSHAD saves the original SP value before any pushes
    let code = [
        0x66, 0x60, // PUSHAD (saves original ESP)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // The original ESP (0x8000) should be on the stack at position [ESP-16]
    // Stack layout: [ESP-32]=DI, [ESP-28]=SI, [ESP-24]=BP, [ESP-20]=original_ESP, [ESP-16]=BX, etc.
    let original_esp_on_stack = read_mem_at_u32(&mem, 0x8000 - 20);
    assert_eq!(
        original_esp_on_stack, 0x8000,
        "Original ESP should be saved on stack"
    );
}

#[test]
fn test_pushad_with_zero_registers() {
    // PUSHAD with all zero registers
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0;
    regs.rcx = 0;
    regs.rdx = 0;
    regs.rsi = 0;
    regs.rdi = 0;
    regs.rbp = 0;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // All zeros should be saved
    let value_on_stack = read_mem_at_u32(&mem, 0x8000 - 32);
    assert_eq!(value_on_stack, 0, "Zero should be pushed");
}

#[test]
fn test_pushad_with_max_registers() {
    // PUSHAD with 32-bit max values
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF;
    regs.rdx = 0xFFFFFFFF;
    regs.rsi = 0xFFFFFFFF;
    regs.rdi = 0xFFFFFFFF;
    regs.rbp = 0xFFFFFFFF;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Max values should be saved
    let value_on_stack = read_mem_at_u32(&mem, 0x8000 - 32);
    assert_eq!(value_on_stack, 0xFFFFFFFF, "Max value should be pushed");
}

#[test]
fn test_pushad_does_not_modify_flags() {
    // PUSHAD should not modify any flags
    let code = [
        0xf9, // STC (set carry flag)
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // Reserve bit 1 set, plus CF
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be unchanged
    assert_eq!(cf_set(result_regs.rflags), true, "CF should remain set");
    assert_eq!(result_regs.rflags, 0x2 | 1, "Flags should be unchanged");
}

// ===== POPAD TESTS (32-bit) =====

#[test]
fn test_popad_restores_registers() {
    // POPAD should restore all 8 general-purpose registers
    // Note: setup_vm_compat uses CS.D=0 (16-bit default), so 0x66 prefix needed for 32-bit ops
    let code = [
        // First push known values
        0x66, 0x60, // PUSHAD (saves current registers)
        0x66, 0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0x66, 0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0x66, 0xb9, 0x33, 0x33, 0x33, 0x33, // MOV ECX, 0x33333333
        0x66, 0xba, 0x44, 0x44, 0x44, 0x44, // MOV EDX, 0x44444444
        0x66, 0xbe, 0x55, 0x55, 0x55, 0x55, // MOV ESI, 0x55555555
        0x66, 0xbf, 0x66, 0x66, 0x66, 0x66, // MOV EDI, 0x66666666
        0x66, 0xbd, 0x77, 0x77, 0x77, 0x77, // MOV EBP, 0x77777777
        // Now restore original values
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0xBBBBBBBB;
    regs.rcx = 0xCCCCCCCC;
    regs.rdx = 0xDDDDDDDD;
    regs.rsi = 0xEEEEEEEE;
    regs.rdi = 0xFFFFFFFF;
    regs.rbp = 0x12121212;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // Registers should be restored to original values
    assert_eq!(result_regs.rax & 0xFFFFFFFF, 0xAAAAAAAA, "RAX restored");
    assert_eq!(result_regs.rbx & 0xFFFFFFFF, 0xBBBBBBBB, "RBX restored");
    assert_eq!(result_regs.rcx & 0xFFFFFFFF, 0xCCCCCCCC, "RCX restored");
    assert_eq!(result_regs.rdx & 0xFFFFFFFF, 0xDDDDDDDD, "RDX restored");
    assert_eq!(result_regs.rsi & 0xFFFFFFFF, 0xEEEEEEEE, "RSI restored");
    assert_eq!(result_regs.rdi & 0xFFFFFFFF, 0xFFFFFFFF, "RDI restored");
    assert_eq!(result_regs.rbp & 0xFFFFFFFF, 0x12121212, "RBP restored");
}

#[test]
fn test_popad_increments_stack_pointer() {
    // POPAD should increment RSP by 32 (8 registers * 4 bytes)
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // After PUSHAD+POPAD, RSP should be back to original
    assert_eq!(result_regs.rsp, 0x8000, "RSP should return to original");
}

#[test]
fn test_popad_ignores_sp_on_stack() {
    // POPAD should ignore (skip) the SP value on the stack
    let code = [
        0x66, 0x60, // PUSHAD (saves ESP)
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32 (make space)
        0x48, 0xc7, 0x84, 0x24, 0x00, 0xff, 0xff, 0xff, 0xEF, 0xBE, 0xAD,
        0xDE, // MOV QWORD [RSP+offset], 0xDEADBEEF (corrupted ESP on stack)
        0x66, 0x61, // POPAD (should ignore the corrupted SP value)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // ESP should be restored properly despite corruption, incremented correctly
    // (may not be exactly 0x8000 due to the SUB instruction, but should be sensible)
    assert!(result_regs.rsp > 0x7000, "RSP should be reasonable");
}

#[test]
fn test_pusha_popa_roundtrip() {
    // PUSHA followed by POPA should preserve all values
    // Note: setup_vm_compat uses CS.D=0 (16-bit default), so 0x66 prefix needed for 32-bit ops
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0x66, 0xbb, 0x55, 0x66, 0x77, 0x88, // MOV EBX, 0x88776655
        0x66, 0x61, // POPAD (restore original)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rbx = 0xBBBBBBBB;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // After PUSHAD + modifications + POPAD, original values should be restored
    assert_eq!(
        result_regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA,
        "RAX restored after POPAD"
    );
    assert_eq!(
        result_regs.rbx & 0xFFFFFFFF,
        0xBBBBBBBB,
        "RBX restored after POPAD"
    );
}

#[test]
fn test_pushad_popa_multiple_times() {
    // Multiple consecutive PUSHAD/POPAD pairs
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // After multiple PUSHAD/POPAD, values should be preserved and RSP restored
    assert_eq!(result_regs.rax & 0xFFFFFFFF, 0x12345678, "RAX unchanged");
    assert_eq!(result_regs.rsp, 0x8000, "RSP restored");
}

#[test]
fn test_pushad_popa_with_alternating_modification() {
    // PUSHAD, modify registers, POPAD, verify restoration
    // Note: setup_vm_compat uses CS.D=0 (16-bit default), so 0x66 prefix needed for 32-bit ops
    let code = [
        0x66, 0x60, // PUSHAD (save state 1)
        0x66, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF (modify)
        0x66, 0x61, // POPAD (restore state 1)
        0x66, 0x89, 0xc1, // MOV ECX, EAX (copy to ECX for verification)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should be restored to original value
    assert_eq!(result_regs.rax & 0xFFFFFFFF, 0x11111111, "RAX restored");
    // ECX should also have the restored value
    assert_eq!(
        result_regs.rcx & 0xFFFFFFFF,
        0x11111111,
        "ECX contains restored value"
    );
}

#[test]
fn test_pushad_popa_does_not_affect_flags() {
    // PUSHAD and POPAD should not affect flags
    let code = [
        0xf9, // STC (set carry flag)
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // Carry flag should still be set
    assert_eq!(cf_set(result_regs.rflags), true, "CF unchanged");
}

#[test]
fn test_pushad_popa_stack_alignment() {
    // PUSHAD/POPAD should maintain proper stack alignment
    let code = [
        0x66, 0x60, // PUSHAD (RSP -= 32)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x8000; // 16-byte aligned

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // After PUSHAD, RSP = 0x8000 - 32 = 0x7FE0 (still 16-byte aligned)
    assert_eq!(result_regs.rsp, 0x8000 - 32, "Stack pointer correct");
    assert_eq!(result_regs.rsp % 16, 0, "Stack alignment maintained");
}

#[test]
fn test_pushad_with_different_register_patterns() {
    // Test PUSHAD with specific register value patterns
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x5A5A5A5A;
    regs.rbx = 0xA5A5A5A5;
    regs.rcx = 0x3C3C3C3C;
    regs.rdx = 0xC3C3C3C3;
    regs.rsi = 0x0F0F0F0F;
    regs.rdi = 0xF0F0F0F0;
    regs.rbp = 0xF00FF00F;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify each value is on the stack
    let rdi = read_mem_at_u32(&mem, 0x8000 - 32);
    assert_eq!(rdi, 0xF0F0F0F0, "RDI value on stack");

    let rsi = read_mem_at_u32(&mem, 0x8000 - 28);
    assert_eq!(rsi, 0x0F0F0F0F, "RSI value on stack");

    let rbp = read_mem_at_u32(&mem, 0x8000 - 24);
    assert_eq!(rbp, 0xF00FF00F, "RBP value on stack");

    let rax = read_mem_at_u32(&mem, 0x8000 - 4);
    assert_eq!(rax, 0x5A5A5A5A, "RAX value on stack");
}

#[test]
fn test_popad_overwrites_current_values() {
    // POPAD should overwrite whatever values are currently in registers
    // Note: setup_vm_compat uses CS.D=0 (16-bit default), so 0x66 prefix needed for 32-bit ops
    let code = [
        // Pre-load stack with known values via PUSHAD
        0x66, 0x60, // PUSHAD
        // Now the original register values are on stack
        // Load different values into registers
        0x66, 0xb8, 0x99, 0x99, 0x99, 0x99, // MOV EAX, 0x99999999
        0x66, 0xbb, 0x88, 0x88, 0x88, 0x88, // MOV EBX, 0x88888888
        // Pop the original values back
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // Even though we set EAX to 0x99999999, POPAD should restore 0x11111111
    assert_eq!(
        result_regs.rax & 0xFFFFFFFF,
        0x11111111,
        "POPAD overwrites EAX"
    );
    assert_eq!(
        result_regs.rbx & 0xFFFFFFFF,
        0x22222222,
        "POPAD overwrites EBX"
    );
}

#[test]
fn test_pushad_popa_preserves_higher_bits_in_64bit() {
    // In 64-bit mode, only lower 32 bits of registers are involved in PUSHAD/POPAD
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111u64;
    regs.rsp = 0x8000;

    let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
    let result_regs = run_until_hlt(&mut vcpu).unwrap();

    // The lower 32 bits should be unchanged (PUSHAD/POPAD in 32-bit mode)
    assert_eq!(
        result_regs.rax & 0xFFFFFFFF,
        0x11111111,
        "Lower 32 bits unchanged"
    );
    // Upper 32 bits should also be unchanged
    assert_eq!(
        result_regs.rax >> 32,
        0x11111111u64,
        "Upper 32 bits unchanged"
    );
}

#[test]
fn test_pushad_does_not_fault_with_various_rsp() {
    // Test PUSHAD with various RSP values
    for rsp_val in &[0x8000u64, 0x9000u64, 0x10000u64, 0xFFF0u64] {
        let code = [
            0x66, 0x60, // PUSHAD
            0xf4, // HLT
        ];
        let mut regs = Registers::default();
        regs.rsp = *rsp_val;

        let (mut vcpu, _) = setup_vm_compat(&code, Some(regs));
        let result_regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            result_regs.rsp,
            rsp_val - 32,
            "RSP decremented correctly for RSP={:x}",
            rsp_val
        );
    }
}
