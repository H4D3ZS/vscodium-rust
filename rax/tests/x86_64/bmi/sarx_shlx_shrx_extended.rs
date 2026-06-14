use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// SARX/SHLX/SHRX Extended Tests - BMI2
// This file contains additional edge case tests and comprehensive coverage
// beyond the basic sarx_shlx_shrx.rs file.
//
// These instructions perform shifts without modifying flags.
// SARX - Arithmetic right shift (sign-extends)
// SHLX - Logical left shift
// SHRX - Logical right shift
//
// Opcodes:
// VEX.LZ.F3.0F38.W0 F7 /r   SARX r32a, r/m32, r32b
// VEX.LZ.F3.0F38.W1 F7 /r   SARX r64a, r/m64, r64b
// VEX.LZ.66.0F38.W0 F7 /r   SHLX r32a, r/m32, r32b
// VEX.LZ.66.0F38.W1 F7 /r   SHLX r64a, r/m64, r64b
// VEX.LZ.F2.0F38.W0 F7 /r   SHRX r32a, r/m32, r32b
// VEX.LZ.F2.0F38.W1 F7 /r   SHRX r64a, r/m64, r64b

// ============================================================================
// SHLX - All Shift Counts (32-bit)
// ============================================================================

#[test]
fn test_shlx_32bit_all_counts_0_to_31() {
    // Test all valid shift counts 0-31
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x80000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x80000001u64 << count) & 0xFFFFFFFF;
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "SHLX by {} failed", count);
    }
}

#[test]
fn test_shlx_32bit_power_of_two_counts() {
    // Test power-of-two shift counts
    let counts = vec![0, 1, 2, 4, 8, 16];
    for count in counts {
        let code = [
            0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x00000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 1u64 << count;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "SHLX by {} should produce {}",
            count,
            expected
        );
    }
}

#[test]
fn test_shlx_32bit_boundary_values() {
    // Test boundary shift values
    let test_cases = vec![
        (0x00000001, 30, 0x40000000),
        (0x00000001, 31, 0x80000000),
        (0x7FFFFFFF, 1, 0xFFFFFFFE),
        (0xFFFFFFFF, 1, 0xFFFFFFFE),
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
            "SHLX(0x{:08X}, {}) should be 0x{:08X}",
            value,
            count,
            expected
        );
    }
}

// ============================================================================
// SHLX - All Shift Counts (64-bit)
// ============================================================================

#[test]
fn test_shlx_64bit_all_counts_0_to_63() {
    // Test all valid shift counts 0-63
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x8000000000000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x8000000000000001u64 << count;
        assert_eq!(regs.rax, expected, "SHLX by {} failed", count);
    }
}

#[test]
fn test_shlx_64bit_boundary_values() {
    // Test boundary shift values
    let test_cases = vec![
        (0x0000000000000001, 62, 0x4000000000000000),
        (0x0000000000000001, 63, 0x8000000000000000),
        (0x7FFFFFFFFFFFFFFF, 1, 0xFFFFFFFFFFFFFFFE),
        (0xFFFFFFFFFFFFFFFF, 1, 0xFFFFFFFFFFFFFFFE),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "SHLX(0x{:016X}, {}) should be 0x{:016X}",
            value, count, expected
        );
    }
}

// ============================================================================
// SHRX - All Shift Counts (32-bit)
// ============================================================================

#[test]
fn test_shrx_32bit_all_counts_0_to_31() {
    // Test all valid shift counts 0-31
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x80000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x80000001u32 >> count;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SHRX by {} failed",
            count
        );
    }
}

#[test]
fn test_shrx_32bit_boundary_values() {
    // Test boundary shift values
    let test_cases = vec![
        (0x80000000, 30, 0x00000002),
        (0x80000000, 31, 0x00000001),
        (0xFFFFFFFF, 1, 0x7FFFFFFF),
        (0xFFFFFFFF, 16, 0x0000FFFF),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
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
            "SHRX(0x{:08X}, {}) should be 0x{:08X}",
            value,
            count,
            expected
        );
    }
}

// ============================================================================
// SHRX - All Shift Counts (64-bit)
// ============================================================================

#[test]
fn test_shrx_64bit_all_counts_0_to_63() {
    // Test all valid shift counts 0-63
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x8000000000000001;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x8000000000000001u64 >> count;
        assert_eq!(regs.rax, expected, "SHRX by {} failed", count);
    }
}

#[test]
fn test_shrx_64bit_boundary_values() {
    // Test boundary shift values
    let test_cases = vec![
        (0x8000000000000000, 62, 0x0000000000000002),
        (0x8000000000000000, 63, 0x0000000000000001),
        (0xFFFFFFFFFFFFFFFF, 1, 0x7FFFFFFFFFFFFFFF),
        (0xFFFFFFFFFFFFFFFF, 32, 0x00000000FFFFFFFF),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "SHRX(0x{:016X}, {}) should be 0x{:016X}",
            value, count, expected
        );
    }
}

// ============================================================================
// SARX - All Shift Counts (32-bit)
// ============================================================================

#[test]
fn test_sarx_32bit_all_counts_positive() {
    // Test all shift counts with positive number
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x7FFFFFFF;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x7FFFFFFFi32 >> count) as u32;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SARX positive by {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_all_counts_negative() {
    // Test all shift counts with negative number
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x80000000;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x80000000u32 as i32 >> count) as u32;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SARX negative by {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_sign_extension_patterns() {
    // Test sign extension with various patterns
    let test_cases = vec![
        (0xF0000000u32, 4, 0xFF000000u32),
        (0xF0000000u32, 8, 0xFFF00000u32),
        (0xF0000000u32, 16, 0xFFFFF000u32),
        (0xF0000000u32, 24, 0xFFFFFFF0u32),
        (0xF0000000u32, 28, 0xFFFFFFFFu32),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value as u64;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SARX(0x{:08X}, {}) sign extension failed",
            value,
            count
        );
    }
}

// ============================================================================
// SARX - All Shift Counts (64-bit)
// ============================================================================

#[test]
fn test_sarx_64bit_all_counts_positive() {
    // Test all shift counts with positive number
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x7FFFFFFFFFFFFFFF;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x7FFFFFFFFFFFFFFFi64 >> count) as u64;
        assert_eq!(regs.rax, expected, "SARX positive by {} failed", count);
    }
}

#[test]
fn test_sarx_64bit_all_counts_negative() {
    // Test all shift counts with negative number
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x8000000000000000;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x8000000000000000u64 as i64 >> count) as u64;
        assert_eq!(regs.rax, expected, "SARX negative by {} failed", count);
    }
}

#[test]
fn test_sarx_64bit_sign_extension_patterns() {
    // Test sign extension with various patterns
    let test_cases = vec![
        (0xF000000000000000u64, 4, 0xFF00000000000000u64),
        (0xF000000000000000u64, 8, 0xFFF0000000000000u64),
        (0xF000000000000000u64, 16, 0xFFFFF00000000000u64),
        (0xF000000000000000u64, 32, 0xFFFFFFFFF0000000u64),
        (0xF000000000000000u64, 60, 0xFFFFFFFFFFFFFFFFu64),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "SARX(0x{:016X}, {}) sign extension failed",
            value, count
        );
    }
}

// ============================================================================
// Memory Operand Tests - Comprehensive
// ============================================================================

#[test]
fn test_shlx_32bit_memory_all_counts() {
    // Test SHLX with memory operand and various counts
    for count in vec![0, 1, 4, 8, 16, 24, 31] {
        let code = [
            0xc4, 0xe2, 0x71, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SHLX EAX, [0x2000], ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u32;
        write_mem_u32(&mem, 0x12345678);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x12345678u64 << count) & 0xFFFFFFFF;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "SHLX from memory by {} failed",
            count
        );
    }
}

#[test]
fn test_shrx_32bit_memory_all_counts() {
    // Test SHRX with memory operand and various counts
    for count in vec![0, 1, 4, 8, 16, 24, 31] {
        let code = [
            0xc4, 0xe2, 0x73, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SHRX EAX, [0x2000], ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u32;
        write_mem_u32(&mem, 0x12345678);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x12345678u32 >> count;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SHRX from memory by {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_memory_negative() {
    // Test SARX with memory operand (negative value)
    for count in vec![0, 1, 4, 8, 16, 24, 31] {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SARX EAX, [0x2000], ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u32;
        write_mem_u32(&mem, 0x80000000);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x80000000u32 as i32 >> count) as u32;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "SARX from memory by {} failed",
            count
        );
    }
}

#[test]
fn test_shlx_64bit_memory_all_counts() {
    // Test SHLX 64-bit with memory operand
    for count in vec![0, 1, 8, 16, 32, 48, 63] {
        let code = [
            0xc4, 0xe2, 0xf1, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SHLX RAX, [0x2000], RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u64;
        write_mem_u64(&mem, 0x123456789ABCDEF0);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x123456789ABCDEF0u64 << count;
        assert_eq!(
            regs.rax, expected,
            "SHLX 64-bit from memory by {} failed",
            count
        );
    }
}

#[test]
fn test_shrx_64bit_memory_all_counts() {
    // Test SHRX 64-bit with memory operand
    for count in vec![0, 1, 8, 16, 32, 48, 63] {
        let code = [
            0xc4, 0xe2, 0xf3, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SHRX RAX, [0x2000], RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u64;
        write_mem_u64(&mem, 0x123456789ABCDEF0);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 0x123456789ABCDEF0u64 >> count;
        assert_eq!(
            regs.rax, expected,
            "SHRX 64-bit from memory by {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_64bit_memory_negative() {
    // Test SARX 64-bit with memory operand (negative value)
    for count in vec![0, 1, 8, 16, 32, 48, 63] {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // SARX RAX, [0x2000], RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = count;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u64;
        write_mem_u64(&mem, 0x8000000000000000);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x8000000000000000u64 as i64 >> count) as u64;
        assert_eq!(
            regs.rax, expected,
            "SARX 64-bit from memory by {} failed",
            count
        );
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_all_shifts_with_zero_value() {
    // All shifts of zero should produce zero
    for count in vec![0, 1, 8, 16, 31] {
        // SHLX
        let code_shlx = [
            0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code_shlx, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rax & 0xFFFFFFFF, 0, "SHLX of zero by {}", count);

        // SHRX
        let code_shrx = [
            0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code_shrx, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rax & 0xFFFFFFFF, 0, "SHRX of zero by {}", count);

        // SARX
        let code_sarx = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code_sarx, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rax & 0xFFFFFFFF, 0, "SARX of zero by {}", count);
    }
}

#[test]
fn test_shift_count_from_high_bits() {
    // Verify only low bits of count register are used
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 0x0000000000000008 | 0xFFFFFFFFFFFFFF00; // High bits set, low bits = 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        256,
        "Only low 8 bits of count should be used"
    );
}

#[test]
fn test_shlx_shrx_inverse_operations() {
    // SHLX followed by SHRX should return to original (for valid ranges)
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00001234;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001234,
        "SHLX then SHRX should return original"
    );
}

#[test]
fn test_consecutive_shifts_accumulate() {
    // Multiple shifts should accumulate
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

    // Two shifts by 4 = shift by 8 total
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000100,
        "Consecutive shifts should accumulate"
    );
}

#[test]
fn test_sarx_preserves_all_ones() {
    // SARX of all ones (negative) should stay all ones
    for count in vec![1, 4, 8, 16, 31] {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0xFFFFFFFF,
            "SARX of all ones by {} should remain all ones",
            count
        );
    }
}

#[test]
fn test_sarx_64bit_preserves_all_ones() {
    // SARX 64-bit of all ones should stay all ones
    for count in vec![1, 8, 16, 32, 63] {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFFFFFFFFFF;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, 0xFFFFFFFFFFFFFFFF,
            "SARX 64-bit of all ones by {} should remain all ones",
            count
        );
    }
}

#[test]
fn test_different_source_dest_preservation() {
    // Ensure source register is not modified
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    regs.rbx = 0x12345678;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x12345678,
        "Source register should be unchanged"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        8,
        "Count register should be unchanged"
    );
    assert_ne!(
        regs.rax & 0xFFFFFFFF,
        0xDEADBEEF,
        "Destination should be modified"
    );
}
