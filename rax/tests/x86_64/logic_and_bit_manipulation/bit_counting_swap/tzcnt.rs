use crate::common::*;
use rax::cpu::Registers;

// TZCNT - Count Trailing Zero Bits
// Counts the number of trailing zero bits in the source operand.
// The count is written to the destination register.
// If the source is zero, the count equals the operand size in bits, and CF is set.
// If the source is non-zero, CF is cleared and ZF reflects whether the count is zero.
//
// Opcodes:
// F3 0F BC /r    TZCNT r16, r/m16    - Count trailing zeros in r/m16
// F3 0F BC /r    TZCNT r32, r/m32    - Count trailing zeros in r/m32
// F3 REX.W 0F BC /r TZCNT r64, r/m64 - Count trailing zeros in r/m64

#[test]
fn test_tzcnt_ax_bx_all_zeros() {
    // TZCNT AX, BX - all zeros
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        16,
        "AX should contain 16 (all bits are zero)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (count is non-zero)"
    );
}

#[test]
fn test_tzcnt_ax_bx_lsb_set() {
    // TZCNT AX, BX - LSB set (no trailing zeros)
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0xc3, // TZCNT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0,
        "AX should contain 0 (no trailing zeros)"
    );
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_all_zeros() {
    // TZCNT EAX, EBX - all zeros (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        32,
        "EAX should contain 32 (all bits are zero)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_lsb_set() {
    // TZCNT EAX, EBX - LSB set (32-bit)
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should contain 0 (no trailing zeros)"
    );
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_rax_rbx_all_zeros() {
    // TZCNT RAX, RBX - all zeros (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "RAX should contain 64 (all bits are zero)");
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
}

#[test]
fn test_tzcnt_rax_rbx_lsb_set() {
    // TZCNT RAX, RBX - LSB set (64-bit)
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should contain 0 (no trailing zeros)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (count is zero)");
}

#[test]
fn test_tzcnt_eax_ebx_one_trailing_zero() {
    // TZCNT with 1 trailing zero
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000002; // bit 1 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1,
        "EAX should contain 1 (one trailing zero)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_eax_ebx_multiple_trailing_zeros() {
    // TZCNT with multiple trailing zeros
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF0000; // bits 16-31 set (16 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        16,
        "EAX should contain 16 (sixteen trailing zeros)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_tzcnt_power_of_two() {
    // Test with powers of two
    for bit_pos in 0..32 {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            bit_pos as u64,
            "TZCNT(2^{}) should be {}",
            bit_pos,
            bit_pos
        );
    }
}

#[test]
fn test_tzcnt_with_extended_registers() {
    // TZCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xbc, 0xc1, // TZCNT R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x00001000; // bit 12 set (12 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 12, "R8D should contain 12");
}

#[test]
fn test_tzcnt_r15_64bit() {
    // TZCNT R15, R15
    let code = [
        0xf3, 0x4d, 0x0f, 0xbc, 0xff, // TZCNT R15, R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0100000000000000; // bit 56 set (56 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 56, "R15 should contain 56");
}

#[test]
fn test_tzcnt_mem16() {
    // TZCNT AX, [mem]
    let code = [
        0x66, 0xf3, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT AX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x0100); // bit 8 set (8 trailing zeros)
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 8, "AX should contain 8");
}

#[test]
fn test_tzcnt_mem32() {
    // TZCNT EAX, [mem]
    let code = [
        0xf3, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00010000); // bit 16 set (16 trailing zeros)
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "EAX should contain 16");
}

#[test]
fn test_tzcnt_mem64() {
    // TZCNT RAX, [mem]
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // TZCNT RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0100000000000000); // bit 56 set (56 trailing zeros)
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 56, "RAX should contain 56");
}

#[test]
fn test_tzcnt_preserves_source() {
    // TZCNT should not modify source operand
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_tzcnt_all_ones() {
    // TZCNT with all bits set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should contain 0 (no trailing zeros)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_tzcnt_single_bit_patterns() {
    // Test various single bit positions
    let test_cases = vec![
        (0x00000001, 0),  // bit 0
        (0x00000002, 1),  // bit 1
        (0x00000004, 2),  // bit 2
        (0x00000008, 3),  // bit 3
        (0x00000010, 4),  // bit 4
        (0x00000100, 8),  // bit 8
        (0x00010000, 16), // bit 16
        (0x01000000, 24), // bit 24
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "TZCNT(0x{:08X}) should be {}",
            value,
            expected
        );
    }
}

#[test]
fn test_tzcnt_alternating_pattern() {
    // TZCNT with alternating pattern
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is LSB set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1,
        "EAX should contain 1 (one trailing zero)"
    );
}

#[test]
fn test_tzcnt_consecutive_bits() {
    // TZCNT with consecutive bits set
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFF00000; // bits 20-31 set (20 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 20, "EAX should contain 20");
}

#[test]
fn test_tzcnt_64bit_low_bit() {
    // TZCNT in 64-bit with low bit set
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000020; // bit 5 set (5 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "RAX should contain 5");
}

#[test]
fn test_tzcnt_sparse_bits() {
    // TZCNT with sparse bits
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set (12 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 12, "EAX should contain 12");
}

#[test]
fn test_tzcnt_byte_values() {
    // Test with single byte set at different positions
    let test_cases = vec![
        (0x000000FF, 0),  // lower byte (LSB set)
        (0x0000FF00, 8),  // second byte
        (0x00FF0000, 16), // third byte
        (0xFF000000, 24), // upper byte
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "TZCNT(0x{:08X}) should be {}",
            value,
            expected
        );
    }
}

#[test]
fn test_tzcnt_vs_bsf_similarity() {
    // TZCNT and BSF give same result for non-zero values
    let code = [
        0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        16,
        "TZCNT should find first set bit at position 16"
    );
}

#[test]
fn test_tzcnt_64bit_lower_half() {
    // TZCNT 64-bit with bit in lower 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000008000; // bit 15 set (15 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 15, "RAX should contain 15");
}

#[test]
fn test_tzcnt_64bit_upper_half() {
    // TZCNT 64-bit with bit only in upper 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xbc, 0xc3, // TZCNT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0100000000000000; // bit 56 set (56 trailing zeros)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 56, "RAX should contain 56");
}

#[test]
fn test_tzcnt_alignment_detection() {
    // TZCNT can detect alignment (power of 2 divisibility)
    let test_cases = vec![
        (0x00000001, 0),  // 2^0 aligned
        (0x00000002, 1),  // 2^1 aligned
        (0x00000004, 2),  // 2^2 aligned
        (0x00000008, 3),  // 2^3 aligned
        (0x00001000, 12), // 2^12 aligned (4KB)
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Value 0x{:08X} is 2^{} aligned",
            value,
            expected
        );
    }
}

#[test]
fn test_tzcnt_odd_numbers() {
    // Odd numbers always have zero trailing zeros
    let odd_values = vec![1, 3, 5, 7, 9, 11, 13, 15];

    for value in odd_values {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0,
            "Odd number {} should have 0 trailing zeros",
            value
        );
    }
}

#[test]
fn test_tzcnt_even_numbers() {
    // Even numbers have at least one trailing zero
    let test_cases = vec![
        (2, 1),  // one trailing zero
        (4, 2),  // two trailing zeros
        (6, 1),  // one trailing zero
        (8, 3),  // three trailing zeros
        (12, 2), // two trailing zeros
        (16, 4), // four trailing zeros
    ];

    for (value, expected) in test_cases {
        let code = [
            0xf3, 0x0f, 0xbc, 0xc3, // TZCNT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Even number {} should have {} trailing zeros",
            value,
            expected
        );
    }
}
