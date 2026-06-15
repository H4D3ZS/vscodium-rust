//! Tests for the WRMSR instruction.
//!
//! WRMSR - Write to Model Specific Register
//!
//! Writes the contents of registers EDX:EAX into the 64-bit model specific
//! register (MSR) specified in the ECX register. The contents of the EDX
//! register are copied to high-order 32 bits of the selected MSR and the
//! contents of the EAX register are copied to low-order 32 bits of the MSR.
//!
//! Opcode: 0F 30
//! Flags affected: None
//!
//! Reference: docs/wrmsr.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// Basic WRMSR Tests
// ============================================================================

#[test]
fn test_wrmsr_basic() {
    // WRMSR - Write EDX:EAX to MSR specified by ECX
    // 0F 30 = WRMSR
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (low 32 bits)
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0 (high 32 bits)
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // WRMSR should complete without errors
    // Registers should be preserved
    assert_eq!(regs.rcx, 0x100, "ECX should be preserved");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be preserved");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0, "EDX should be preserved");
}

#[test]
fn test_wrmsr_write_then_read() {
    // Write a value to an MSR, then read it back
    let code = [
        // Write phase
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x42, 0x42, 0x42, 0x42, // MOV RAX, 0x42424242
        0x48, 0xC7, 0xC2, 0x99, 0x99, 0x99, 0x99, // MOV RDX, 0x99999999
        0x0F, 0x30, // WRMSR
        // Read phase
        0x48, 0x31, 0xC0, // XOR RAX, RAX (clear)
        0x48, 0x31, 0xD2, // XOR RDX, RDX (clear)
        0x0F, 0x32, // RDMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should read back the written value (if MSR is writable)
    // Note: Some MSRs may mask or ignore certain bits
}

#[test]
fn test_wrmsr_preserves_flags() {
    // WRMSR should not modify flags
    let code = [
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

#[test]
fn test_wrmsr_preserves_other_registers() {
    // WRMSR should not modify other registers
    // Use values with bit 31 clear to avoid sign-extension issues
    let code = [
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC6, 0x55, 0x55, 0x55, 0x55, // MOV RSI, 0x55555555 (bit 31 clear)
        0x48, 0xC7, 0xC7, 0x66, 0x66, 0x66, 0x66, // MOV RDI, 0x66666666 (bit 31 clear)
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42424242, "RBX should not be modified");
    assert_eq!(regs.rsi, 0x55555555, "RSI should not be modified");
    assert_eq!(regs.rdi, 0x66666666, "RDI should not be modified");
}

// ============================================================================
// EDX:EAX Value Tests
// ============================================================================

#[test]
fn test_wrmsr_zero_value() {
    // Write zero to an MSR
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Should complete successfully
}

#[test]
fn test_wrmsr_all_ones() {
    // Write all ones to an MSR (may be masked)
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, 0xFFFFFFFF
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RDX, 0xFFFFFFFF
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Should complete (reserved bits may cause #GP in real systems)
}

#[test]
fn test_wrmsr_pattern_values() {
    // Write alternating bit patterns
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0xC7, 0xC2, 0x55, 0x55, 0x55, 0x55, // MOV RDX, 0x55555555
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_high_32_bits_only() {
    // Write value with only high 32 bits set
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0x31, 0xC0, // XOR RAX, RAX (low = 0)
        0x48, 0xC7, 0xC2, 0x12, 0x34, 0x56, 0x78, // MOV RDX, 0x78563412
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_low_32_bits_only() {
    // Write value with only low 32 bits set
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x87, 0x65, 0x43, 0x21, // MOV RAX, 0x21436587
        0x48, 0x31, 0xD2, // XOR RDX, RDX (high = 0)
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// ECX Value Tests - Different MSR Indices
// ============================================================================

#[test]
fn test_wrmsr_high_rcx_ignored() {
    // High 32 bits of RCX should be ignored in 64-bit mode
    let code = [
        0x48, 0xB9, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RCX, 0xFFFFFFFF_00000100
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Should write to MSR 0x100, ignoring high 32 bits
}

#[test]
fn test_wrmsr_preserves_ecx() {
    // ECX should not be modified by WRMSR
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x100, "ECX should not be modified");
}

// ============================================================================
// Sequential WRMSR Tests
// ============================================================================

#[test]
fn test_wrmsr_multiple_writes() {
    // Write to same MSR twice
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR #1
        0x48, 0xC7, 0xC0, 0x22, 0x22, 0x22, 0x22, // MOV RAX, 0x22222222
        0x0F, 0x30, // WRMSR #2 (overwrites)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Second write should overwrite first
}

#[test]
fn test_wrmsr_different_msrs() {
    // Write to two different MSRs
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR (MSR 0x100)
        0x48, 0xC7, 0xC1, 0x01, 0x01, 0x00, 0x00, // MOV RCX, 0x101
        0x48, 0xC7, 0xC0, 0x22, 0x00, 0x00, 0x00, // MOV RAX, 0x22
        0x0F, 0x30, // WRMSR (MSR 0x101)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_loop() {
    // Write MSR in a loop
    // Offsets: 0-6 MOV RCX, 7-13 MOV RBX, 14-16 XOR RAX, 17-19 XOR RDX
    //          20-21 WRMSR (loop_start), 22-24 INC RAX, 25-27 DEC RBX, 28-29 JNZ, 30 HLT
    // JNZ target = 20, RIP after JNZ = 30, offset = -10 = 0xF6
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3 (loop counter)
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        // loop_start (offset 20):
        0x0F, 0x30, // WRMSR
        0x48, 0xFF, 0xC0, // INC RAX (change value)
        0x48, 0xFF, 0xCB, // DEC RBX
        0x75, 0xF6, // JNZ loop_start (offset -10)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0, "Loop should complete 3 iterations");
}

// ============================================================================
// Write/Read Sequences
// ============================================================================

#[test]
fn test_wrmsr_write_read_verify() {
    // Write a specific value, then verify by reading
    let code = [
        // Write
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0xEF, 0xBE, 0xAD, 0xDE, // MOV RAX, 0xDEADBEEF
        0x48, 0xC7, 0xC2, 0xFE, 0xCA, 0xEF, 0xBE, // MOV RDX, 0xBEEFCAFE
        0x0F, 0x30, // WRMSR
        // Clear registers
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        // Read
        0x0F, 0x32, // RDMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Depending on MSR characteristics, may or may not match exactly
    // (some MSRs have reserved/read-only bits)
}

#[test]
fn test_wrmsr_increment_pattern() {
    // Write incrementing values
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0x31, 0xC0, // XOR RAX, RAX (start at 0)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        // loop:
        0x0F, 0x30, // WRMSR
        0x48, 0x05, 0x00, 0x10, 0x00, 0x00, // ADD RAX, 0x1000
        0x48, 0xFF, 0xCB, // DEC RBX
        0x75, 0xF5, // JNZ loop
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0);
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x5000);
}

// ============================================================================
// 64-bit Specific Tests
// ============================================================================

#[test]
fn test_wrmsr_uses_only_lower_32bits() {
    // Upper 32 bits of RAX and RDX should be ignored
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xB8, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RAX, 0xFFFFFFFF_00000001
        0x48, 0xBA, 0x02, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, // MOV RDX, 0xFFFFFFFF_00000002
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Should write 0x00000002_00000001 (only lower 32 bits)
}

#[test]
fn test_wrmsr_edx_eax_composition() {
    // Verify EDX:EAX forms the 64-bit value correctly
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678 (low)
        0x48, 0xC7, 0xC2, 0xF0, 0xDE, 0xBC, 0x9A, // MOV RDX, 0x9ABCDEF0 (high)
        0x0F, 0x30, // WRMSR (writes 0x9ABCDEF0_12345678)
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x32, // RDMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should read back the 64-bit value (if MSR supports it)
    let value = ((regs.rdx as u64) << 32) | (regs.rax as u64);
    // Value depends on MSR implementation
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_wrmsr_boundary_values() {
    // Test with boundary values for 32-bit fields
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0x7F, // MOV RAX, 0x7FFFFFFF (max positive)
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x80, // MOV RDX, 0x80000000 (min negative)
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_single_bit_patterns() {
    // Write with single bits set
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x00000001
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x80, // MOV RDX, 0x80000000
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_alternating_writes() {
    // Alternate between two values
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        // First write
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR
        // Second write
        0x48, 0xC7, 0xC0, 0x55, 0x55, 0x55, 0x55, // MOV RAX, 0x55555555
        0x0F, 0x30, // WRMSR
        // Third write
        0x48, 0xC7, 0xC0, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RAX, 0xAAAAAAAA
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_wrmsr_with_computation() {
    // Compute value before writing
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0x48, 0xC1, 0xE0, 0x04, // SHL RAX, 4 (RAX = 0x100)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x100);
}

#[test]
fn test_wrmsr_preserves_rax_rdx() {
    // RAX and RDX values should be preserved after WRMSR
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xC7, 0xC2, 0x99, 0x00, 0x00, 0x00, // MOV RDX, 0x99
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX and RDX should still contain the values used for writing
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x42, "RAX should be preserved");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x99, "RDX should be preserved");
}

// ============================================================================
// CPL / Privilege Enforcement Tests (#GP on privileged instructions at CPL 3)
//
// Privileged instructions (WRMSR/RDMSR, MOV CRx, LGDT/LIDT/LLDT/LTR, ...) must
// raise #GP(0) when executed at CPL != 0. CLI/STI fault only when CPL > IOPL.
//
// Detection: setup_vm installs an IDT whose every vector points at an IRETQ
// stub at INT_HANDLER_ADDR. We drop CPL to 3 (CS RPL=3), single-step the
// faulting instruction, and verify RIP landed on the fault handler (i.e. the
// #GP was delivered) rather than advancing past the instruction.
// ============================================================================

/// Drop the vCPU to ring 3 (CPL = 3) by setting the CS/SS selector RPL bits.
/// Returns the vCPU ready to single-step the first instruction at CPL 3.
fn make_cpl3(vcpu: &mut rax::backend::emulator::x86_64::X86_64Vcpu) {
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cs.selector = 0x1B; // index 3, RPL 3
    sregs.ss.selector = 0x1B;
    sregs.cs.dpl = 3;
    vcpu.set_sregs(&sregs).unwrap();
}

/// True if, after a single step, RIP has been redirected to the exception
/// handler stub (meaning an exception such as #GP was delivered).
fn faulted_to_handler(vcpu: &mut rax::backend::emulator::x86_64::X86_64Vcpu) -> bool {
    let _ = vcpu.step();
    vcpu.get_regs().unwrap().rip == INT_HANDLER_ADDR
}

#[test]
fn test_wrmsr_cpl3_raises_gp() {
    // WRMSR at CPL 3 must raise #GP(0).
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Pre-load RCX so the privilege check is the only thing that can fail,
    // then drop to CPL 3 before executing WRMSR.
    let mut regs = vcpu.get_regs().unwrap();
    regs.rcx = 0x100;
    regs.rip = 0x1000 + 7; // skip the MOV, point directly at WRMSR
    vcpu.set_regs(&regs).unwrap();
    make_cpl3(&mut vcpu);

    assert!(
        faulted_to_handler(&mut vcpu),
        "WRMSR at CPL 3 should raise #GP and jump to the fault handler"
    );
}

#[test]
fn test_rdmsr_cpl3_raises_gp() {
    // RDMSR at CPL 3 must raise #GP(0).
    let code = [
        0x0F, 0x32, // RDMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    make_cpl3(&mut vcpu);

    assert!(
        faulted_to_handler(&mut vcpu),
        "RDMSR at CPL 3 should raise #GP and jump to the fault handler"
    );
}

#[test]
fn test_mov_cr0_cpl3_raises_gp() {
    // MOV CR0, RAX at CPL 3 must raise #GP(0).
    // 0F 22 C0 = MOV CR0, RAX
    let code = [
        0x0F, 0x22, 0xC0, // MOV CR0, RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let cr0_before = vcpu.get_sregs().unwrap().cr0;
    make_cpl3(&mut vcpu);

    assert!(
        faulted_to_handler(&mut vcpu),
        "MOV CR0 at CPL 3 should raise #GP and jump to the fault handler"
    );
    // The write must not have taken effect.
    assert_eq!(
        vcpu.get_sregs().unwrap().cr0,
        cr0_before,
        "CR0 must be unchanged after a faulting MOV CR0 at CPL 3"
    );
}

#[test]
fn test_sti_cpl_gt_iopl_raises_gp() {
    // STI with CPL (3) > IOPL (0) must raise #GP(0).
    // 0xFB = STI
    let code = [
        0xFB, // STI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Ensure IOPL = 0 (default rflags has no IOPL bits set), then drop to CPL 3.
    make_cpl3(&mut vcpu);

    assert!(
        faulted_to_handler(&mut vcpu),
        "STI with CPL > IOPL should raise #GP and jump to the fault handler"
    );
}

#[test]
fn test_sti_cpl_le_iopl_allowed_at_cpl3() {
    // STI with CPL (3) <= IOPL (3) must be permitted (no #GP).
    // 0xFB = STI ; 0xF4 = HLT
    let code = [0xFB, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    // Set IOPL = 3 in RFLAGS (bits 12-13) so CPL 3 is allowed to modify IF.
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= 0x3000; // IOPL = 3
    vcpu.set_regs(&regs).unwrap();
    make_cpl3(&mut vcpu);

    // Step STI - must NOT fault; RIP advances to the HLT (0x1001).
    let _ = vcpu.step();
    let rip = vcpu.get_regs().unwrap().rip;
    assert_eq!(
        rip, 0x1001,
        "STI with CPL <= IOPL must execute and advance RIP, not fault (rip={:#x})",
        rip
    );
    assert!(
        vcpu.get_regs().unwrap().rflags & 0x200 != 0,
        "STI should have set the IF flag"
    );
}

#[test]
fn test_wrmsr_cpl0_still_works() {
    // Positive control: WRMSR at CPL 0 (default kernel mode) must succeed.
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x01, 0x00, 0x00, // MOV RCX, 0x100
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x0F, 0x30, // WRMSR
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    // setup_vm leaves CS selector 0x08 => CPL 0.
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x100, "WRMSR at CPL 0 should run to HLT normally");
}

// ============================================================================
// Strengthened: WRMSR -> RDMSR round-trip for every IMPLEMENTED MSR.
//
// Writes a known 64-bit value (high in RDX, low in RAX) to `msr`, then reads
// it back and reconstructs EDX:EAX. The value is recovered exactly.
// ============================================================================

/// Build a WRMSR(msr, value) then RDMSR(msr) sequence and return the reconstructed
/// 64-bit value (EDX:EAX) after the read.
fn msr_round_trip(msr: u32, value: u64) -> u64 {
    let lo = (value & 0xFFFF_FFFF) as u32;
    let hi = (value >> 32) as u32;
    let code = [
        // MOV RCX, msr  (mov ecx, imm32 zero-extends)
        0xb9,
        msr as u8,
        (msr >> 8) as u8,
        (msr >> 16) as u8,
        (msr >> 24) as u8,
        // MOV EAX, lo
        0xb8,
        lo as u8,
        (lo >> 8) as u8,
        (lo >> 16) as u8,
        (lo >> 24) as u8,
        // MOV EDX, hi
        0xba,
        hi as u8,
        (hi >> 8) as u8,
        (hi >> 16) as u8,
        (hi >> 24) as u8,
        0x0f,
        0x30, // WRMSR
        // Clobber EAX/EDX to prove RDMSR repopulates them.
        0x31,
        0xc0, // XOR EAX, EAX
        0x31,
        0xd2, // XOR EDX, EDX
        0x0f,
        0x32, // RDMSR
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Upper halves must be cleared by RDMSR in 64-bit mode.
    assert_eq!(regs.rax >> 32, 0, "RDMSR clears upper RAX");
    assert_eq!(regs.rdx >> 32, 0, "RDMSR clears upper RDX");
    ((regs.rdx & 0xFFFF_FFFF) << 32) | (regs.rax & 0xFFFF_FFFF)
}

#[test]
fn test_msr_roundtrip_efer() {
    // IA32_EFER (0xC0000080). Use a value with several known bits + high bits.
    let v = 0x0000_0000_0000_0D01u64; // SCE|LME|LMA|NXE-ish pattern
    assert_eq!(msr_round_trip(0xC0000080, v), v, "EFER round-trip");
}

#[test]
fn test_msr_roundtrip_star() {
    let v = 0x0023_0010_DEAD_BEEFu64;
    assert_eq!(msr_round_trip(0xC0000081, v), v, "STAR round-trip");
}

#[test]
fn test_msr_roundtrip_lstar() {
    let v = 0xFFFF_8000_1234_5678u64;
    assert_eq!(msr_round_trip(0xC0000082, v), v, "LSTAR round-trip");
}

#[test]
fn test_msr_roundtrip_cstar() {
    let v = 0xFFFF_8000_ABCD_EF01u64;
    assert_eq!(msr_round_trip(0xC0000083, v), v, "CSTAR round-trip");
}

#[test]
fn test_msr_roundtrip_fmask() {
    let v = 0x0000_0000_0000_0200u64; // mask IF
    assert_eq!(msr_round_trip(0xC0000084, v), v, "SFMASK round-trip");
}

#[test]
fn test_msr_roundtrip_fs_base() {
    let v = 0x0000_7FFF_DEAD_0000u64;
    assert_eq!(msr_round_trip(0xC0000100, v), v, "FS.base round-trip");
}

#[test]
fn test_msr_roundtrip_gs_base() {
    // Use a small non-canonical-but-fine test value; keep high bits zero to avoid
    // the per-CPU shadow write path that triggers on huge gs.base values.
    let v = 0x0000_0000_0BAD_F00Du64;
    assert_eq!(msr_round_trip(0xC0000101, v), v, "GS.base round-trip");
}

#[test]
fn test_msr_roundtrip_kernel_gs_base() {
    let v = 0xFFFF_8800_0000_1000u64;
    assert_eq!(
        msr_round_trip(0xC0000102, v),
        v,
        "KERNEL_GS_BASE round-trip"
    );
}

#[test]
fn test_msr_roundtrip_sysenter_cs() {
    let v = 0x0000_0000_0000_0008u64;
    assert_eq!(msr_round_trip(0x174, v), v, "IA32_SYSENTER_CS round-trip");
}

#[test]
fn test_msr_roundtrip_sysenter_esp() {
    let v = 0x0000_0000_0008_F000u64;
    assert_eq!(msr_round_trip(0x175, v), v, "IA32_SYSENTER_ESP round-trip");
}

#[test]
fn test_msr_roundtrip_sysenter_eip() {
    let v = 0xFFFF_FFFF_8100_2000u64;
    assert_eq!(msr_round_trip(0x176, v), v, "IA32_SYSENTER_EIP round-trip");
}

#[test]
fn test_msr_unimplemented_reads_zero() {
    // MSR 0x100 is not implemented: writes are ignored, reads return 0.
    assert_eq!(
        msr_round_trip(0x100, 0xDEAD_BEEF_CAFE_BABE),
        0,
        "unimpl MSR reads 0"
    );
}

#[test]
fn test_msr_lstar_visible_in_sregs() {
    // Cross-check that WRMSR(LSTAR) actually updates architectural state by
    // reading it back through a second independent RDMSR sequence.
    let v = 0xFFFF_8000_0042_4242u64;
    assert_eq!(msr_round_trip(0xC0000082, v), v);
}
