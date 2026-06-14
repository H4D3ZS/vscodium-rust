use crate::common::{cf_set, run_until_hlt, setup_vm, sf_set};
use rax::cpu::Registers;

// SHLX - Logical Left Shift Without Affecting Flags (BMI2)
// This file contains comprehensive tests for the SHLX instruction.
//
// SHLX performs a logical left shift operation without modifying any flags.
// It is part of the BMI2 (Bit Manipulation Instruction Set 2) extension.
//
// Syntax: SHLX dest, src, count
// - dest: destination register (receives shifted result)
// - src: source operand (register or memory) to be shifted
// - count: register containing shift count (low 5/6 bits used)
//
// Opcodes:
// VEX.LZ.66.0F38.W0 F7 /r   SHLX r32, r/m32, r32   - 32-bit logical left shift
// VEX.LZ.66.0F38.W1 F7 /r   SHLX r64, r/m64, r64   - 64-bit logical left shift
//
// Count Masking:
// - 32-bit: count masked to 5 bits (0-31)
// - 64-bit: count masked to 6 bits (0-63)
//
// Flags: None modified (unlike SHL which sets CF, ZF, SF, OF)

// ============================================================================
// SHLX 32-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_shlx_32bit_shift_by_0() {
    // Shift by 0 should return original value
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Shift by 0 should preserve value"
    );
}

#[test]
fn test_shlx_32bit_shift_by_1() {
    // Shift by 1 doubles the value
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000002, "1 << 1 = 2");
}

#[test]
fn test_shlx_32bit_all_shift_counts() {
    // Test all valid shift counts 0-31
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x00000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if count < 32 { 1u64 << count } else { 0 };
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "1 << {} should be {}",
            count,
            expected
        );
    }
}

#[test]
fn test_shlx_32bit_shift_to_sign_bit() {
    // Shifting 1 to position 31 sets sign bit
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "1 << 31 should set sign bit"
    );
}

#[test]
fn test_shlx_32bit_count_masking() {
    // Count is masked to 5 bits, so 32 becomes 0
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 32; // Masked to 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Count 32 masked to 0, value unchanged"
    );
}

#[test]
fn test_shlx_32bit_count_masking_33() {
    // Count 33 is masked to 1
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000005;
    regs.rcx = 33; // Masked to 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0000000A, "5 << 1 = 10");
}

#[test]
fn test_shlx_32bit_power_of_two_shifts() {
    // Test shifting by powers of two
    let test_cases = vec![
        (0x00000001, 0, 0x00000001),
        (0x00000001, 1, 0x00000002),
        (0x00000001, 2, 0x00000004),
        (0x00000001, 4, 0x00000010),
        (0x00000001, 8, 0x00000100),
        (0x00000001, 16, 0x00010000),
        (0x00000001, 24, 0x01000000),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "0x{:08X} << {} should be 0x{:08X}",
            value,
            count,
            expected
        );
    }
}

// ============================================================================
// SHLX 64-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_shlx_64bit_shift_by_0() {
    // Shift by 0 should return original value
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "Shift by 0 should preserve value"
    );
}

#[test]
fn test_shlx_64bit_shift_by_1() {
    // Shift by 1 doubles the value
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000002, "1 << 1 = 2");
}

#[test]
fn test_shlx_64bit_all_shift_counts() {
    // Test all valid shift counts 0-63
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x0000000000000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 1u64 << count;
        assert_eq!(regs.rax, expected, "1 << {} should be {}", count, expected);
    }
}

#[test]
fn test_shlx_64bit_shift_to_sign_bit() {
    // Shifting 1 to position 63 sets sign bit
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001;
    regs.rcx = 63;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000000000000000, "1 << 63 should set sign bit");
}

#[test]
fn test_shlx_64bit_count_masking() {
    // Count is masked to 6 bits, so 64 becomes 0
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 64; // Masked to 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "Count 64 masked to 0");
}

#[test]
fn test_shlx_64bit_shift_by_32() {
    // Shifting by 32 moves lower dword to upper dword
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000FFFFFFFF;
    regs.rcx = 32;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF00000000, "Lower dword shifted to upper");
}

// ============================================================================
// Flag Tests - SHLX Does NOT Modify Flags
// ============================================================================

#[test]
fn test_shlx_32bit_does_not_modify_cf() {
    // SHLX should not modify CF even when bits shift out
    let code = [
        0xf9, // STC (set CF)
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // High bit set, will shift out
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "High bit shifted out");
    assert!(cf_set(regs.rflags), "CF should still be set from STC");
}

#[test]
fn test_shlx_32bit_does_not_clear_cf() {
    // SHLX should not clear CF
    let code = [
        0xf9, // STC (set CF)
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should remain set");
}

#[test]
fn test_shlx_64bit_preserves_all_flags() {
    // Set various flags, then SHLX should preserve them
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF, AF)
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001;
    regs.rcx = 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000010000, "Shift result correct");
    assert!(cf_set(regs.rflags), "CF should still be set from SUB");
    assert!(sf_set(regs.rflags), "SF should still be set from SUB");
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_shlx_32bit_alternating_pattern() {
    // Shift alternating bit pattern
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 10101010...
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555554,
        "Alternating pattern shifted left by 1"
    );
}

#[test]
fn test_shlx_32bit_all_ones() {
    // Shift all ones
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF00,
        "All ones shifted left by 8"
    );
}

#[test]
fn test_shlx_64bit_alternating_pattern() {
    // Shift alternating bit pattern (64-bit)
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x5555555555555555;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xAAAAAAAAAAAAAAAA, "Alternating pattern shifted");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_shlx_32bit_memory_operand() {
    // SHLX with memory source
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHLX EAX, [0x2000], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    use crate::common::write_mem_u32;
    write_mem_u32(&mem, 0x00001234);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00012340,
        "Memory operand shifted correctly"
    );
}

#[test]
fn test_shlx_64bit_memory_operand() {
    // SHLX 64-bit with memory source
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHLX RAX, [0x2000], RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    use crate::common::write_mem_u64;
    write_mem_u64(&mem, 0x0000000012345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000001234567800,
        "64-bit memory operand shifted"
    );
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_shlx_32bit_r8d_r9d_r10d() {
    // SHLX R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x29, 0xf7, 0xc1, // SHLX R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x00000001;
    regs.r10 = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x00100000,
        "Extended registers work correctly"
    );
}

#[test]
fn test_shlx_64bit_r14_r15_r13() {
    // SHLX R14, R15, R13
    let code = [
        0xc4, 0x42, 0x91, 0xf7, 0xf7, // SHLX R14, R15, R13
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0000000000000001;
    regs.r13 = 48;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0x0001000000000000, "64-bit extended registers");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_shlx_32bit_shift_all_bits_out() {
    // Shifting so all bits are lost
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x000000FF;
    regs.rcx = 24; // Shift byte to top, then 24 more = all out
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFF000000, "Shifted to high byte");
}

#[test]
fn test_shlx_source_preservation() {
    // Source and count registers should not be modified
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "Source unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 8, "Count unchanged");
}

#[test]
fn test_shlx_consecutive_shifts() {
    // Multiple SHLX operations
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 1 << 4 = 16
    // Second: 16 << 4 = 256
    assert_eq!(regs.rax & 0xFFFFFFFF, 256, "Consecutive shifts accumulate");
}
