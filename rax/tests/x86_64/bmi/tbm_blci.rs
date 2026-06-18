use crate::common::*;
use rax::cpu::Registers;

// BLCI - Isolate Lowest Clear Bit (TBM)
// Isolates the lowest clear bit, clearing all other bits.
// Equivalent to: dest = ~src & (src + 1)
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 02 /6   BLCI r32, r/m32   - Isolate lowest clear (32-bit)
// VEX.NDD.LZ.0F38.W1 02 /6   BLCI r64, r/m64   - Isolate lowest clear (64-bit)

#[test]
fn test_blci_basic() {
    // BLCI EAX, EBX - basic test
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX (/6 = ModRM 0xF3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_1101; // bit 1 is clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should isolate bit 1: 0b0000_0010
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0010,
        "Isolate lowest clear bit"
    );
}

#[test]
fn test_blci_bit_0_clear() {
    // BLCI when bit 0 is clear
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1010; // bit 0 is clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0001, "Bit 0 clear");
}

#[test]
fn test_blci_all_bits_set() {
    // BLCI with all bits set (no clear bits in 32-bit range)
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0xFFFFFFFF & (0xFFFFFFFF + 1) = 0 & 0x100000000 = 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "All bits set gives 0");
}

#[test]
fn test_blci_zero() {
    // BLCI with zero (all bits clear, bit 0 is lowest)
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0 & 1 = 0xFFFFFFFF & 1 = 1
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Zero gives 1");
}

#[test]
fn test_blci_single_bit_set() {
    // BLCI with single bit set
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1000; // Only bit 3 set, bits 0-2 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Isolate bit 0 (first clear): 0b0001
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0001, "Single bit set");
}

#[test]
fn test_blci_64bit() {
    // BLCI RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0x02, 0xf3, // BLCI RAX, RBX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF_FFFF_FFFF_FFFE; // bit 0 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "64-bit isolate bit 0");
}

#[test]
fn test_blci_extended_registers() {
    // BLCI R8D, R9D
    let code = [
        0xc4, 0x42, 0x38, 0x02, 0xf1, // BLCI R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b1111_0111; // bit 3 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 0b0000_1000, "Extended registers");
}

#[test]
fn test_blci_pattern_1() {
    // Test pattern: alternating with gap
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_1011; // bit 2 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0100, "Pattern with gap");
}

#[test]
fn test_blci_high_bit_clear() {
    // BLCI with only high bit clear
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // bit 31 clear, all others set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000000, "High bit isolated");
}

#[test]
fn test_blci_mem_operand() {
    // BLCI EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, // BLCI EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0b1101_1101); // bits 1, 5 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Isolate bit 1: 0b0000_0010
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0010, "Memory operand");
}

#[test]
fn test_blci_power_of_two_minus_one() {
    // BLCI with 2^n - 1 (all lower bits set)
    for i in 1..16 {
        let code = [
            0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = (1u64 << i) - 1; // 2^i - 1
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Lowest clear bit is at position i
        let expected = 1u64 << i;
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "2^{} - 1", i);
    }
}

#[test]
fn test_blci_formula() {
    // Verify BLCI formula: ~src & (src + 1)
    let test_values = [0x12u32, 0x1234, 0x123456, 0x12345678];

    for &value in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = !value & value.wrapping_add(1);
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "Formula for 0x{:08x}",
            value
        );
    }
}

#[test]
fn test_blci_consecutive_set_bits() {
    // BLCI with consecutive bits set from LSB
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0111_1111; // bits 0-6 set, bit 7 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b1000_0000, "Consecutive bits");
}

#[test]
fn test_blci_alternating_pattern() {
    // BLCI with alternating pattern
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010... pattern (bit 0 clear)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Alternating pattern");
}

#[test]
fn test_blci_64bit_patterns() {
    // 64-bit patterns
    let test_cases = [
        (0xFFFF_FFFF_FFFF_FFFEu64, 0x0000_0000_0000_0001u64), // bit 0 clear
        (0xFFFF_FFFF_0000_0000u64, 0x0000_0000_0000_0001u64), // lower 32 bits clear
    ];

    for (src, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0xf8, 0x02, 0xf3, // BLCI RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax, *expected, "BLCI({:016x})", src);
    }
}

#[test]
fn test_blci_preserves_source() {
    // BLCI should not modify source
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "Source unchanged");
}

#[test]
fn test_blci_find_lowest_clear() {
    // Use BLCI to find position of lowest clear bit
    let test_cases = [
        (0b1111_1110u32, 0), // bit 0 clear
        (0b1111_1101u32, 1), // bit 1 clear
        (0b1111_1011u32, 2), // bit 2 clear
        (0b1111_0111u32, 3), // bit 3 clear
    ];

    for (value, expected_pos) in &test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = 1u64 << expected_pos;
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "Value 0x{:08x}", value);
    }
}

#[test]
fn test_blci_sparse_pattern() {
    // BLCI with sparse clear bits
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFEFF; // bit 8 clear (among others)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Lowest clear is bit 8
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x100, "Sparse pattern");
}

#[test]
fn test_blci_byte_boundaries() {
    // Test at byte boundaries
    for byte_pos in 0..4 {
        let code = [
            0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        // All bits set except one at byte boundary
        regs.rbx = 0xFFFFFFFFu64 & !(0xFFu64 << (byte_pos * 8));
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // Just verify execution
    }
}

#[test]
fn test_blci_complement_blsfill() {
    // BLCI finds the bit that BLSFILL would set
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00; // bits 0-7 clear, 8-15 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Lowest clear bit");
}

#[test]
fn test_blci_nibble_patterns() {
    // Test with nibble-sized patterns
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFF0; // lower nibble has bit 0-3 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Nibble pattern");
}

#[test]
fn test_blci_practical_gap_finding() {
    // Practical: find gaps in bitmaps
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_1101_1111_1111; // gap at bit 9
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Lowest clear is bit 9 (bits 0-8 are set)
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x200, "Bitmap gap");
}

#[test]
fn test_blci_all_even_bits_set() {
    // All even bits set, odd bits clear
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555554; // 0x55555555 with bit 0 cleared
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Even bits set");
}

#[test]
fn test_blci_max_minus_one() {
    // Maximum value minus one
    let code = [
        0xc4, 0xe2, 0x78, 0x02, 0xf3, // BLCI EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "Max - 1");
}

#[test]
fn test_blci_64bit_high_clear() {
    // 64-bit with high bit clear
    let code = [
        0xc4, 0xe2, 0xf8, 0x02, 0xf3, // BLCI RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFF_FFFF_FFFF_FFFF; // bit 63 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000_0000_0000_0000, "64-bit high clear");
}
