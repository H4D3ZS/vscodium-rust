use crate::common::*;
use rax::cpu::Registers;

// TBM (Trailing Bit Manipulation) Instructions - Remaining Set
// BLCS - Set Lowest Clear Bit (AMD TBM)
// BLSFILL - Fill From Lowest Set Bit (AMD TBM)
// BLSIC - Isolate Lowest Set Bit and Complement (AMD TBM)
// T1MSKC - Inverse Mask From Trailing Ones (AMD TBM)
// TZMSK - Mask From Trailing Zeros (AMD TBM)

// BLCS - Set Lowest Clear Bit
// Sets the lowest clear bit. Equivalent to: dest = src | (src + 1)
// VEX.NDD.LZ.0F38.W0 01 /3   BLCS r32, r/m32
// VEX.NDD.LZ.0F38.W1 01 /3   BLCS r64, r/m64

#[test]
fn test_blcs_basic() {
    // BLCS EAX, EBX - set lowest clear bit
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX (/3)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_1101; // bit 1 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should set bit 1: 0b1111_1111
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b1111_1111, "Set lowest clear bit");
}

#[test]
fn test_blcs_bit_0_clear() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1010;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b1010_1011, "Set bit 0");
}

#[test]
fn test_blcs_all_set() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All set: src | (src+1) = 0xFFFFFFFF | 0 = 0xFFFFFFFF
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "All set overflow");
}

#[test]
fn test_blcs_formula() {
    // Verify: dest = src | (src + 1)
    let test_values = [0x12u32, 0x1234, 0x123456];
    for &val in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = val as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = val | val.wrapping_add(1);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "BLCS(0x{:x})", val);
    }
}

// BLSFILL - Fill From Lowest Set Bit
// Fills all bits from bit 0 up to and including the lowest set bit.
// Equivalent to: dest = src | (src - 1)
// VEX.NDD.LZ.0F38.W0 01 /2   BLSFILL r32, r/m32
// VEX.NDD.LZ.0F38.W1 01 /2   BLSFILL r64, r/m64

#[test]
fn test_blsfill_basic() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xd3, // BLSFILL EAX, EBX (/2)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Fill from bit 0 to bit 3: 0b0000_1111
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_1111, "Fill from lowest set");
}

#[test]
fn test_blsfill_bit_0_set() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xd3, // BLSFILL EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bit 0 is lowest, so just sets bit 0: 0b1010_1001 | 0 = 0b1010_1001
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b1010_1001, "Bit 0 set");
}

#[test]
fn test_blsfill_zero() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xd3, // BLSFILL EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0 | (0-1) = 0 | 0xFFFFFFFF = 0xFFFFFFFF
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "Zero fills all");
}

#[test]
fn test_blsfill_formula() {
    // Verify: dest = src | (src - 1)
    let test_values = [0x10u32, 0x100, 0x1000];
    for &val in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xd3, // BLSFILL EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = val as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = val | val.wrapping_sub(1);
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "BLSFILL(0x{:x})",
            val
        );
    }
}

// BLSIC - Isolate Lowest Set Bit and Complement
// Isolates the lowest set bit and inverts all bits.
// Equivalent to: dest = ~src | (src - 1)
// VEX.NDD.LZ.0F38.W0 01 /6   BLSIC r32, r/m32
// VEX.NDD.LZ.0F38.W1 01 /6   BLSIC r64, r/m64

#[test]
fn test_blsic_basic() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xf3, // BLSIC EAX, EBX (/6)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0b0000_1000 | 0b0000_0111 = 0b1111_0111 | 0b0000_0111 = 0b1111_0111
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFF7, "BLSIC basic");
}

#[test]
fn test_blsic_bit_0() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xf3, // BLSIC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~1 | 0 = 0xFFFFFFFE
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFE, "BLSIC bit 0");
}

#[test]
fn test_blsic_formula() {
    // Verify: dest = ~src | (src - 1)
    let test_values = [0x8u32, 0x80, 0x800];
    for &val in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xf3, // BLSIC EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = val as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = !val | val.wrapping_sub(1);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "BLSIC(0x{:x})", val);
    }
}

// T1MSKC - Inverse Mask From Trailing Ones
// Creates a mask of all bits below the lowest zero bit, then inverts.
// Equivalent to: dest = ~src | (src + 1)
// VEX.NDD.LZ.0F38.W0 01 /7   T1MSKC r32, r/m32
// VEX.NDD.LZ.0F38.W1 01 /7   T1MSKC r64, r/m64

#[test]
fn test_t1mskc_basic() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xfb, // T1MSKC EAX, EBX (/7)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_1101; // bit 1 is first zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0b1111_1101 | 0b1111_1110 = 0b0000_0010 | 0b1111_1110 = 0xFFFFFFFE
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFE, "T1MSKC basic");
}

#[test]
fn test_t1mskc_no_trailing_ones() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xfb, // T1MSKC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1010; // bit 0 is zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0b1010_1010 | 0b1010_1011 = 0x55555555 | 0xAB = mix
    let expected = !0xAAu32 | 0xABu32;
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "No trailing ones");
}

#[test]
fn test_t1mskc_formula() {
    // Verify: dest = ~src | (src + 1)
    let test_values = [0x3u32, 0x7, 0xF, 0x1F];
    for &val in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xfb, // T1MSKC EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = val as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = !val | val.wrapping_add(1);
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected as u64,
            "T1MSKC(0x{:x})",
            val
        );
    }
}

// TZMSK - Mask From Trailing Zeros
// Creates a mask of all trailing zeros.
// Equivalent to: dest = ~src & (src - 1)
// VEX.NDD.LZ.0F38.W0 01 /4   TZMSK r32, r/m32
// VEX.NDD.LZ.0F38.W1 01 /4   TZMSK r64, r/m64

#[test]
fn test_tzmsk_basic() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX (/4)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // 3 trailing zeros
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0b0000_1000 & 0b0000_0111 = 0xFFFFFFF7 & 0x7 = 0x7
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0111, "Mask 3 trailing zeros");
}

#[test]
fn test_tzmsk_no_trailing_zeros() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1011; // bit 0 set, no trailing zeros
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~src & (src-1) = with no trailing zeros gives 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "No trailing zeros");
}

#[test]
fn test_tzmsk_all_zeros() {
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0 & (0-1) = 0xFFFFFFFF & 0xFFFFFFFF = 0xFFFFFFFF
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "All zeros");
}

#[test]
fn test_tzmsk_formula() {
    // Verify: dest = ~src & (src - 1)
    let test_values = [0x10u32, 0x100, 0x1000, 0x10000];
    for &val in &test_values {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = val as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = !val & val.wrapping_sub(1);
        assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "TZMSK(0x{:x})", val);
    }
}

// Additional comprehensive tests

#[test]
fn test_tbm_64bit_versions() {
    // Test 64-bit versions of each instruction
    let code1 = [
        0xc4, 0xe2, 0xf8, 0x01, 0xdb, // BLCS RAX, RBX (64-bit)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF_FFFF_FFFF_FFFE;
    let (mut vcpu, _) = setup_vm(&code1, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFF_FFFF_FFFF_FFFF, "BLCS 64-bit");

    let code2 = [
        0xc4, 0xe2, 0xf8, 0x01, 0xd3, // BLSFILL RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0001_0000_0000;
    let (mut vcpu, _) = setup_vm(&code2, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_0001_FFFF_FFFF, "BLSFILL 64-bit");
}

#[test]
fn test_tbm_extended_registers() {
    // Test with R8-R15
    let code = [
        0xc4, 0x42, 0x38, 0x01, 0xd9, // BLCS R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFFFFFFFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFFFFFFFF, 0xFFFFFFFF, "Extended regs");
}

#[test]
fn test_tbm_memory_operands() {
    // Test memory operands
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLCS EAX, [mem]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFFFFE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "Memory operand");
}

#[test]
fn test_tbm_chained_operations() {
    // Chain multiple TBM operations
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
        0xc4, 0xe2, 0x78, 0x01, 0xd0, // BLSFILL EAX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // Just verify execution
}

#[test]
fn test_blcs_power_of_two() {
    // BLCS on powers of 2
    for i in 0..16 {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // Verify execution
    }
}

#[test]
fn test_blsfill_consecutive_bits() {
    // BLSFILL with various consecutive bit patterns
    let test_cases = [
        (0x01u32, 0x01u32),
        (0x02u32, 0x03u32),
        (0x04u32, 0x07u32),
        (0x08u32, 0x0Fu32),
    ];
    for (src, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xd3, // BLSFILL EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            *expected as u64,
            "BLSFILL(0x{:x})",
            src
        );
    }
}

#[test]
fn test_blsic_alternating() {
    // BLSIC with alternating patterns
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xf3, // BLSIC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = !0xAAAAAAAAu32 | 0xAAAAAAA9u32;
    assert_eq!(regs.rax & 0xFFFFFFFF, expected as u64, "BLSIC alternating");
}

#[test]
fn test_t1mskc_all_ones() {
    // T1MSKC with all ones
    let code = [
        0xc4, 0xe2, 0x78, 0x01, 0xfb, // T1MSKC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ~0xFFFFFFFF | 0 = 0
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "T1MSKC all ones");
}

#[test]
fn test_tzmsk_byte_boundaries() {
    // TZMSK at byte boundaries
    for byte_pos in 0..4 {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x01u64 << (byte_pos * 8);
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << (byte_pos * 8)) - 1;
        assert_eq!(regs.rax & 0xFFFFFFFF, expected, "TZMSK byte {}", byte_pos);
    }
}

#[test]
fn test_all_tbm_preserve_source() {
    // All TBM instructions should preserve source
    let instructions = [
        [0xc4, 0xe2, 0x78, 0x01, 0xdb], // BLCS
        [0xc4, 0xe2, 0x78, 0x01, 0xd3], // BLSFILL
        [0xc4, 0xe2, 0x78, 0x01, 0xf3], // BLSIC
        [0xc4, 0xe2, 0x78, 0x01, 0xfb], // T1MSKC
        [0xc4, 0xe2, 0x78, 0x01, 0xe3], // TZMSK
    ];

    for instr in &instructions {
        let mut code = instr.to_vec();
        code.push(0xf4);
        let mut regs = Registers::default();
        regs.rbx = 0x12345678;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "Source preserved");
    }
}

#[test]
fn test_tbm_practical_patterns() {
    // Practical bit manipulation patterns
    let patterns = [
        (0x00000001u32, 0x00000100u32),
        (0xFFFFFFFEu32, 0x55555555u32),
        (0x80000000u32, 0x00000001u32),
    ];

    for (val1, val2) in &patterns {
        let code = [
            0xc4, 0xe2, 0x78, 0x01, 0xdb, // BLCS EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *val1 as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        let code2 = [
            0xc4, 0xe2, 0x78, 0x01, 0xe3, // TZMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *val2 as u64;
        let (mut vcpu, _) = setup_vm(&code2, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
    }
}
