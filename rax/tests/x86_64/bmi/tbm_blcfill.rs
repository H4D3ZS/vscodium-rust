use crate::common::*;
use rax::cpu::Registers;

// BLCFILL - Fill From Lowest Clear Bit (TBM)
// Clears trailing ones below the lowest clear bit.
// Equivalent to: dest = src & (src + 1)
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 01 /1   BLCFILL r32, r/m32   - Fill from lowest clear (32-bit)
// VEX.NDD.LZ.0F38.W1 01 /1   BLCFILL r64, r/m64   - Fill from lowest clear (64-bit)

#[test]
fn test_blcfill_basic() {
    // BLCFILL EAX, EBX - basic test
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX (/1 = ModRM 0xCB)
        0xf4,
    ];
    let src = 0b1111_1101u32; // bit 1 is clear (first clear bit)
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "Fill from lowest clear bit"
    );
}

#[test]
fn test_blcfill_bit_0_clear() {
    // BLCFILL when bit 0 is clear
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let src = 0b1010_1010u32; // bit 0 is clear
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "Bit 0 clear");
}

#[test]
fn test_blcfill_all_bits_set() {
    // BLCFILL with all bits set (no clear bits)
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // src & (src + 1) = 0xFFFFFFFF & 0 = 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "All bits set gives 0");
}

#[test]
fn test_blcfill_zero() {
    // BLCFILL with zero
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0 & 1 = 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "Zero gives zero");
}

#[test]
fn test_blcfill_single_bit_set() {
    // BLCFILL with single bit set
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let src = 0b1000u32; // Only bit 3 set, bits 0-2 clear
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "Single bit set");
}

#[test]
fn test_blcfill_64bit() {
    // BLCFILL RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0x01, 0xcb, // BLCFILL RAX, RBX (W1)
        0xf4,
    ];
    let src = 0xFFFF_FFFF_FFFF_FFFEu64; // bit 0 clear
    let mut regs = Registers::default();
    regs.rbx = src;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax, expected, "64-bit fill bit 0");
}

#[test]
fn test_blcfill_extended_registers() {
    // BLCFILL R8D, R9D
    let code = [
        0xc4, 0x42, 0x38, 0x01, 0xc9, // BLCFILL R8D, R9D
        0xf4,
    ];
    let src = 0b1111_0111u32; // bit 3 clear
    let mut regs = Registers::default();
    regs.r9 = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.r8 & 0xFFFFFFFF, expected as u64, "Extended registers");
}

#[test]
fn test_blcfill_pattern_1() {
    // Test pattern: alternating with gap
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let src = 0b1111_1011u32; // bit 2 clear
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "Pattern with gap");
}

#[test]
fn test_blcfill_high_bit_clear() {
    // BLCFILL with high bit clear
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let src = 0x7FFF_FFFFu32; // bit 31 clear
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "High bit clear");
}

#[test]
fn test_blcfill_mem_operand() {
    // BLCFILL EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00,
        0x00, // BLCFILL EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let src = 0b1101_1101u32; // bits 1, 5 clear
    write_mem_u32(&mem, src);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "Memory operand");
}

#[test]
fn test_blcfill_power_of_two() {
    // BLCFILL with powers of 2
    for i in 1..16 {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i; // Power of 2
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << i) & (1u64 << i).wrapping_add(1);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "Power of 2: {}", i);
    }
}

#[test]
fn test_blcfill_consecutive_bits() {
    // BLCFILL with consecutive bits set from LSB
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let src = 0b0111_1111u32; // bits 0-6 set, bit 7 clear
    let mut regs = Registers::default();
    regs.rbx = src as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = src & src.wrapping_add(1);
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "Consecutive bits");
}

#[test]
fn test_blcfill_formula() {
    // Verify BLCFILL formula: src & (src + 1)
    let test_values = [0x12u32, 0x1234, 0x123456, 0x12345678];

    for &value in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = value & value.wrapping_add(1);
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "Formula for 0x{:08x}",
            value
        );
    }
}

#[test]
fn test_blcfill_64bit_patterns() {
    // 64-bit patterns
    let test_cases = [0xFFFF_FFFF_0000_0001u64, 0x0000_0000_FFFF_FFFEu64];

    for src in &test_cases {
        let code = [
            0xc4, 0xe2, 0xf8, 0x01, 0xcb, // BLCFILL RAX, RBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = src & src.wrapping_add(1);
        assert_eq!(regs.rax, expected, "BLCFILL({:016x})", src);
    }
}

#[test]
fn test_blcfill_preserves_source() {
    // BLCFILL should not modify source
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "Source unchanged");
}

#[test]
fn test_blcfill_byte_patterns() {
    // Test byte-aligned patterns
    for byte in 0..4u32 {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xcb, // BLCFILL EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = (0xFEu64) << (byte * 8); // 0xFE in each byte position
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // Just verify execution
    }
}
