use crate::common::*;
use rax::cpu::Registers;

// BMI2 Extended Tests
// Additional edge cases and combinations for BMI2 instructions
// Tests PDEP/PEXT interaction, BZHI edge cases, and comprehensive scenarios

#[test]
fn test_pdep_pext_round_trip() {
    // PEXT(PDEP(x, mask), mask) == x (for x that fits in mask bit count)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xc4, 0xe2, 0x7a, 0xf5, 0xd0, // PEXT EDX, EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // source
    regs.rcx = 0xF0F0F0F0; // mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFF,
        "PEXT(PDEP(x, mask), mask) == x"
    );
}

#[test]
fn test_pext_pdep_round_trip() {
    // PDEP(PEXT(x, mask), mask) == (x & mask)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xc4, 0xe2, 0x7b, 0xf5, 0xd1, // PDEP EDX, EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0xF0F0F0F0; // mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = 0x12345678u32 & 0xF0F0F0F0;
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        expected as u64,
        "PDEP(PEXT(x, mask), mask) == x & mask"
    );
}

#[test]
fn test_bzhi_with_count_0() {
    // BZHI with count 0 should zero all bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0; // count = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "BZHI with count 0 gives 0");
}

#[test]
fn test_bzhi_with_count_32() {
    // BZHI with count 32 should keep all bits (32-bit)
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 32; // count = 32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "BZHI with count 32 is identity"
    );
}

#[test]
fn test_bzhi_with_count_64() {
    // BZHI with count 64 should keep all bits (64-bit)
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 64; // count = 64
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "BZHI with count 64 is identity"
    );
}

#[test]
fn test_bzhi_count_greater_than_operand_size() {
    // BZHI with count > operand size (should wrap mod 256)
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 256 + 16; // wraps to 16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF, "Count wraps mod 256");
}

#[test]
fn test_mulx_high_bits() {
    // MULX with values that produce large results
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let result = 0xFFFFFFFFu64 * 0xFFFFFFFFu64;
    let high = (result >> 32) as u32;
    let low = result as u32;
    assert_eq!(regs.rax & 0xFFFFFFFF, high as u64, "High 32 bits");
    assert_eq!(regs.rbx & 0xFFFFFFFF, low as u64, "Low 32 bits");
}

#[test]
fn test_mulx_64bit_overflow() {
    // MULX 64-bit with overflow
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result is 128-bit
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFE, "High 64 bits");
    assert_eq!(regs.rbx, 0x0000000000000001, "Low 64 bits");
}

#[test]
fn test_rorx_all_positions_32bit() {
    // RORX through all rotation positions
    for imm in 0..32 {
        let code = [
            0xc4, 0xe3, 0x7b, 0xf0, 0xc3, imm, // RORX EAX, EBX, imm
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x80000001;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (0x80000001u32).rotate_right(imm as u32);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "RORX by {}", imm);
    }
}

#[test]
fn test_sarx_shlx_shrx_composition() {
    // Test that SHLX followed by SHRX gives identity for appropriate values
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xc4, 0xe2, 0x73, 0xf7, 0xd0, // SHRX EDX, EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00001234; // value with high bits clear
    regs.rcx = 8; // shift amount
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x00001234,
        "SHRX(SHLX(x, n), n) == x"
    );
}

#[test]
fn test_pdep_all_combinations_small() {
    // Test all combinations for small bit patterns
    let masks = [0x0F, 0xF0, 0x55, 0xAA];
    let sources = [0x0, 0x1, 0xF, 0xFF];

    for &mask in &masks {
        for &src in &sources {
            let code = [
                0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
                0xf4,
            ];
            let mut regs = Registers::default();
            regs.rbx = src;
            regs.rcx = mask;
            let (mut vcpu, _) = setup_vm(&code, Some(regs));
            let _regs = run_until_hlt(&mut vcpu).unwrap();
            // Just verify it executes
        }
    }
}

#[test]
fn test_pext_all_combinations_small() {
    // Test all combinations for small bit patterns
    let masks = [0x0F, 0xF0, 0x55, 0xAA];
    let sources = [0x0, 0x1, 0xF, 0xFF];

    for &mask in &masks {
        for &src in &sources {
            let code = [
                0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
                0xf4,
            ];
            let mut regs = Registers::default();
            regs.rbx = src;
            regs.rcx = mask;
            let (mut vcpu, _) = setup_vm(&code, Some(regs));
            let _regs = run_until_hlt(&mut vcpu).unwrap();
            // Just verify it executes
        }
    }
}

#[test]
fn test_bzhi_sequential_counts() {
    // BZHI with sequential counts
    for count in 0..=32 {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = count as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if count >= 32 {
            0xFFFFFFFFu64
        } else {
            (1u64 << count) - 1
        };
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "BZHI count {}", count);
    }
}

#[test]
fn test_shift_instructions_preserve_flags() {
    // SARX, SHLX, SHRX should not modify flags
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 8;
    regs.rflags = 0x2 | (1 << 0) | (1 << 6) | (1 << 7) | (1 << 11); // Set CF, ZF, SF, OF
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_mulx_zero_multiplicand() {
    // MULX with zero multiplicand
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "High = 0");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "Low = 0");
}

#[test]
fn test_mulx_one_multiplicand() {
    // MULX with one as multiplicand
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 1;
    regs.rcx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "High = 0");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "Low = operand");
}

#[test]
fn test_rorx_zero_rotation() {
    // RORX with zero rotation
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x00, // RORX EAX, EBX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "No rotation with 0");
}

#[test]
fn test_rorx_full_rotation_32() {
    // RORX with full 32-bit rotation
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x20, // RORX EAX, EBX, 32 (wraps to 0)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Full rotation is identity"
    );
}

#[test]
fn test_sarx_negative_value() {
    // SARX with negative value (arithmetic shift preserves sign)
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // negative in signed interpretation
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xC0000000, "Sign extended");
}

#[test]
fn test_shrx_negative_value() {
    // SHRX with negative value (logical shift doesn't preserve sign)
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x40000000, "Not sign extended");
}

#[test]
fn test_pdep_pext_complementary_masks() {
    // Using complementary masks with PDEP and PEXT
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0x00FF00FF; // alternating bytes
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x000000FF, "Complementary mask PDEP");
}

#[test]
fn test_bzhi_with_all_bits_set() {
    // BZHI on all 1s with various counts
    for count in [1, 8, 16, 24, 31] {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = count;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << count) - 1;
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "BZHI count {}", count);
    }
}

#[test]
fn test_extended_register_combinations() {
    // Test various extended register combinations
    let code = [
        0xc4, 0x42, 0x32, 0xf5, 0xc2, // PEXT R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xF0F0F0F0;
    regs.r10 = 0xF0F0F0F0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 0xFFFF, "Extended reg PEXT");
}

#[test]
fn test_memory_operand_combinations() {
    // PDEP with memory operand
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // PDEP EAX, EBX, [mem]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x0F0F0F0F);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000F0F, "Memory operand PDEP");
}

#[test]
fn test_chained_bmi2_operations() {
    // Chain multiple BMI2 operations
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xc4, 0xe2, 0x70, 0xf5, 0xc0, // BZHI EAX, EAX, ECX (reuse ECX as count)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 16; // also used as BZHI count
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Just verify it executes
}

#[test]
fn test_bit_manipulation_patterns() {
    // Common bit manipulation patterns
    let patterns = [
        (0x12345678u32, 0xF0F0F0F0u32),
        (0xAAAAAAAAu32, 0x55555555u32),
        (0x0000FFFFu32, 0xFFFF0000u32),
    ];

    for (src, mask) in &patterns {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src as u64;
        regs.rcx = *mask as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // Verify it executes
    }
}

#[test]
fn test_64bit_boundary_conditions() {
    // Test 64-bit boundary conditions
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF00000000; // upper 32 bits only
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF00000000, "64-bit boundary PDEP");
}

#[test]
fn test_performance_critical_patterns() {
    // Common performance-critical bit manipulation patterns
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xc4, 0xe2, 0x7a, 0xf5, 0xd0, // PEXT EDX, EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0F;
    regs.rcx = 0xF000F000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x0F, "Round-trip pattern");
}
