use crate::common::{cf_set, run_until_hlt, setup_vm, sf_set, zf_set};
use rax::cpu::Registers;

// BZHI Extended Tests - Zero High Bits Starting with Specified Bit Position (BMI2)
// This file contains additional comprehensive tests beyond the basic bzhi.rs tests
// in the logic_and_bit_manipulation/bmi2 directory.
//
// BZHI copies bits from source and zeros all bits starting from the index position.
// Index is taken from bits [7:0] of the second source operand.
// CF is set if index >= operand size, otherwise cleared.
// ZF is set if result is zero, SF based on result sign.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F5 /r   BZHI r32, r/m32, r32
// VEX.NDS.LZ.0F38.W1 F5 /r   BZHI r64, r/m64, r64

// ============================================================================
// BZHI 32-bit - All Index Values
// ============================================================================

#[test]
fn test_bzhi_32bit_all_indices_0_to_32() {
    // Test all valid index values 0-32
    for index in 0..=32 {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if index >= 32 {
            0xFFFFFFFF
        } else {
            ((1u64 << index) - 1) & 0xFFFFFFFF
        };
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI with index {} failed",
            index
        );

        // Check CF flag
        if index >= 32 {
            assert!(cf_set(regs.rflags), "CF should be set for index {}", index);
        } else {
            assert!(
                !cf_set(regs.rflags),
                "CF should be clear for index {}",
                index
            );
        }
    }
}

#[test]
fn test_bzhi_32bit_power_of_two_indices() {
    // Test power-of-two indices
    let test_cases = vec![
        (1, 0x00000001),
        (2, 0x00000003),
        (4, 0x0000000F),
        (8, 0x000000FF),
        (16, 0x0000FFFF),
    ];

    for (index, expected_mask) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected_mask,
            "BZHI index {} should create mask 0x{:08X}",
            index,
            expected_mask
        );
    }
}

#[test]
fn test_bzhi_32bit_boundary_indices() {
    // Test boundary cases around 32-bit limit
    let test_cases = vec![
        (30, 0x3FFFFFFF, false),
        (31, 0x7FFFFFFF, false),
        (32, 0xFFFFFFFF, true),
        (33, 0xFFFFFFFF, true),
    ];

    for (index, expected, expect_cf) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI index {} result mismatch",
            index
        );
        assert_eq!(
            cf_set(regs.rflags),
            expect_cf,
            "CF flag mismatch for index {}",
            index
        );
    }
}

// ============================================================================
// BZHI 64-bit - All Index Values
// ============================================================================

#[test]
fn test_bzhi_64bit_all_indices_0_to_64() {
    // Test all valid index values 0-64
    for index in 0..=64 {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFFFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if index >= 64 {
            0xFFFFFFFFFFFFFFFF
        } else {
            (1u64 << index).wrapping_sub(1)
        };
        assert_eq!(
            regs.rax, expected,
            "BZHI 64-bit with index {} failed",
            index
        );

        // Check CF flag
        if index >= 64 {
            assert!(cf_set(regs.rflags), "CF should be set for index {}", index);
        } else {
            assert!(
                !cf_set(regs.rflags),
                "CF should be clear for index {}",
                index
            );
        }
    }
}

#[test]
fn test_bzhi_64bit_power_of_two_indices() {
    // Test power-of-two indices
    let test_cases = vec![
        (1, 0x0000000000000001),
        (2, 0x0000000000000003),
        (4, 0x000000000000000F),
        (8, 0x00000000000000FF),
        (16, 0x000000000000FFFF),
        (32, 0x00000000FFFFFFFF),
    ];

    for (index, expected_mask) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFFFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected_mask,
            "BZHI 64-bit index {} should create mask 0x{:016X}",
            index, expected_mask
        );
    }
}

#[test]
fn test_bzhi_64bit_boundary_indices() {
    // Test boundary cases around 64-bit limit
    let test_cases = vec![
        (62, 0x3FFFFFFFFFFFFFFF, false),
        (63, 0x7FFFFFFFFFFFFFFF, false),
        (64, 0xFFFFFFFFFFFFFFFF, true),
        (65, 0xFFFFFFFFFFFFFFFF, true),
    ];

    for (index, expected, expect_cf) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFFFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "BZHI 64-bit index {} result mismatch",
            index
        );
        assert_eq!(
            cf_set(regs.rflags),
            expect_cf,
            "CF flag mismatch for index {}",
            index
        );
    }
}

// ============================================================================
// Flag Behavior Tests
// ============================================================================

#[test]
fn test_bzhi_32bit_zf_set_on_zero_result() {
    // ZF should be set when result is zero
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0; // Keep 0 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "Result should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set for zero result");
}

#[test]
fn test_bzhi_32bit_zf_clear_on_nonzero_result() {
    // ZF should be clear when result is non-zero
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 8; // Keep lower 8 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rax & 0xFFFFFFFF, 0, "Result should be non-zero");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear for non-zero result"
    );
}

#[test]
fn test_bzhi_32bit_sf_set_on_negative_result() {
    // SF should be set when result has high bit set
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 31; // Keep lower 31 bits (bit 30 set, bit 31 clear)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFF,
        "Result should have bit 30 set"
    );
    assert!(!sf_set(regs.rflags), "SF should be clear (bit 31 clear)");
}

#[test]
fn test_bzhi_32bit_sf_clear_on_positive_result() {
    // SF should be clear when high bit is not set
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF;
    regs.rcx = 16; // Keep lower 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000FFFF,
        "Result should be 16-bit mask"
    );
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_bzhi_64bit_sf_behavior() {
    // Test SF with 64-bit operands
    let test_cases = vec![
        (0xFFFFFFFFFFFFFFFF, 63, false), // Bit 63 clear
        (0xFFFFFFFFFFFFFFFF, 64, true),  // All bits set, bit 63 set
    ];

    for (value, index, expect_sf) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            sf_set(regs.rflags),
            expect_sf,
            "SF mismatch for index {}",
            index
        );
    }
}

// ============================================================================
// Pattern and Data Tests
// ============================================================================

#[test]
fn test_bzhi_32bit_extract_specific_fields() {
    // Extract specific bit fields from complex patterns
    let test_cases = vec![
        (0xABCD1234, 4, 0x00000004),  // Lower nibble
        (0xABCD1234, 8, 0x00000034),  // Lower byte
        (0xABCD1234, 12, 0x00000234), // Lower 12 bits
        (0xABCD1234, 16, 0x00001234), // Lower word
        (0xABCD1234, 20, 0x000D1234), // Lower 20 bits
        (0xABCD1234, 24, 0x00CD1234), // Lower 24 bits
        (0xABCD1234, 28, 0x0BCD1234), // Lower 28 bits
    ];

    for (value, index, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI(0x{:08X}, {}) should be 0x{:08X}",
            value,
            index,
            expected
        );
    }
}

#[test]
fn test_bzhi_64bit_extract_specific_fields() {
    // Extract specific bit fields from 64-bit patterns
    let test_cases = vec![
        (0xFEDCBA9876543210, 8, 0x0000000000000010),
        (0xFEDCBA9876543210, 16, 0x0000000000003210),
        (0xFEDCBA9876543210, 32, 0x0000000076543210),
        (0xFEDCBA9876543210, 48, 0x0000BA9876543210),
        (0xFEDCBA9876543210, 56, 0x00DCBA9876543210),
    ];

    for (value, index, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "BZHI(0x{:016X}, {}) should be 0x{:016X}",
            value, index, expected
        );
    }
}

#[test]
fn test_bzhi_32bit_alternating_patterns() {
    // Test with alternating bit patterns
    let test_cases = vec![
        (0xAAAAAAAA, 8, 0x000000AA),
        (0xAAAAAAAA, 16, 0x0000AAAA),
        (0x55555555, 8, 0x00000055),
        (0x55555555, 16, 0x00005555),
    ];

    for (value, index, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI alternating pattern failed"
        );
    }
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_bzhi_32bit_memory_all_indices() {
    // Test BZHI with memory operand for various indices
    for index in vec![0, 4, 8, 12, 16, 20, 24, 28, 32] {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // BZHI EAX, [0x2000], ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = index;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u32;
        write_mem_u32(&mem, 0xFFFFFFFF);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if index >= 32 {
            0xFFFFFFFF
        } else {
            ((1u64 << index) - 1) & 0xFFFFFFFF
        };
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI from memory with index {} failed",
            index
        );
    }
}

#[test]
fn test_bzhi_64bit_memory_all_indices() {
    // Test BZHI 64-bit with memory operand
    for index in vec![0, 8, 16, 24, 32, 40, 48, 56, 64] {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // BZHI RAX, [0x2000], RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = index;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u64;
        write_mem_u64(&mem, 0xFFFFFFFFFFFFFFFF);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if index >= 64 {
            0xFFFFFFFFFFFFFFFF
        } else {
            (1u64 << index).wrapping_sub(1)
        };
        assert_eq!(
            regs.rax, expected,
            "BZHI 64-bit from memory with index {} failed",
            index
        );
    }
}

#[test]
fn test_bzhi_32bit_memory_complex_patterns() {
    // Test extracting fields from complex patterns in memory
    let patterns = vec![
        (0x12345678, 4, 0x00000008),
        (0x12345678, 8, 0x00000078),
        (0x12345678, 16, 0x00005678),
        (0xABCDEF01, 12, 0x00000F01),
    ];

    for (value, index, expected) in patterns {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
            0x00, // BZHI EAX, [0x2000], ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rcx = index;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        use crate::common::write_mem_u32;
        write_mem_u32(&mem, value);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Memory BZHI(0x{:08X}, {}) failed",
            value,
            index
        );
    }
}

// ============================================================================
// Edge Cases and Special Behaviors
// ============================================================================

#[test]
fn test_bzhi_index_only_uses_low_8_bits() {
    // Index uses only bits [7:0], so 256 wraps to 0
    let test_cases = vec![
        (256, 0),  // 256 & 0xFF = 0
        (257, 1),  // 257 & 0xFF = 1
        (264, 8),  // 264 & 0xFF = 8
        (272, 16), // 272 & 0xFF = 16
    ];

    for (count_value, effective_index) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = count_value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if effective_index >= 32 {
            0xFFFFFFFF
        } else {
            ((1u64 << effective_index) - 1) & 0xFFFFFFFF
        };
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Index {} should wrap to {}",
            count_value,
            effective_index
        );
    }
}

#[test]
fn test_bzhi_preserves_source_operands() {
    // BZHI should not modify source or index registers
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x12345678,
        "Source should be unchanged"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        16,
        "Index register should be unchanged"
    );
}

#[test]
fn test_bzhi_consecutive_operations() {
    // Chain multiple BZHI operations
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 16; // First operation: keep lower 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First BZHI: 0xFFFFFFFF -> 0x0000FFFF
    // Second BZHI: 0x0000FFFF -> 0x000000FF
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "Consecutive BZHI operations failed"
    );
}

#[test]
fn test_bzhi_creating_bit_masks() {
    // Use BZHI to create various bit masks
    let mask_sizes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25, 31];

    for size in mask_sizes {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = size;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = ((1u64 << size) - 1) & 0xFFFFFFFF;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "{}-bit mask creation failed",
            size
        );
    }
}

#[test]
fn test_bzhi_with_zero_source() {
    // BZHI of zero should always be zero
    for index in vec![0, 8, 16, 32, 64] {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, 0,
            "BZHI of zero should be zero for index {}",
            index
        );
        assert!(zf_set(regs.rflags), "ZF should be set for zero result");
    }
}

#[test]
fn test_bzhi_sparse_bit_patterns() {
    // Test with sparse bit patterns
    let test_cases = vec![
        (0x80000001, 1, 0x00000001),  // Keep only bit 0
        (0x80000001, 2, 0x00000001),  // Keep bits 0-1
        (0x80001000, 16, 0x00001000), // Keep bits 0-15 (bit 12 set)
        (0x00010000, 20, 0x00010000), // Keep bits 0-19 (bit 16 set)
    ];

    for (value, index, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Sparse pattern BZHI(0x{:08X}, {}) failed",
            value,
            index
        );
    }
}
