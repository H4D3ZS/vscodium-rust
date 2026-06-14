use crate::common::*;
use rax::cpu::Registers;

// PDEP - Parallel Bits Deposit (BMI2)
// Deposits bits from source operand to destination using mask.
// Each set bit in the mask corresponds to a deposited bit from source.
// Bits are deposited in LSB order from source to mask positions.
//
// Opcodes:
// VEX.NDS.LZ.F2.0F38.W0 F5 /r   PDEP r32, r32, r/m32   - Parallel bits deposit (32-bit)
// VEX.NDS.LZ.F2.0F38.W1 F5 /r   PDEP r64, r64, r/m64   - Parallel bits deposit (64-bit)

#[test]
fn test_pdep_basic_single_bit() {
    // PDEP EAX, EBX, ECX - deposit bit 0 to position 0
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0001; // source: bit 0 set
    regs.rcx = 0b0001; // mask: bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0001,
        "Bit 0 should be deposited to position 0"
    );
}

#[test]
fn test_pdep_deposit_to_higher_position() {
    // PDEP EAX, EBX, ECX - deposit bit 0 to position 4
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0001; // source: bit 0 set
    regs.rcx = 0b10000; // mask: bit 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b10000,
        "Bit 0 from source should be deposited to position 4"
    );
}

#[test]
fn test_pdep_multiple_bits() {
    // PDEP with multiple bits
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b111; // source: bits 0, 1, 2 set
    regs.rcx = 0b1010100; // mask: bits 2, 4, 6 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bits from source (0,1,2) -> mask positions (2,4,6)
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b1010100,
        "Bits should be deposited to mask positions"
    );
}

#[test]
fn test_pdep_sparse_deposition() {
    // PDEP with sparse mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111; // source: bits 0-3 set
    regs.rcx = 0x88000000; // mask: bits 27, 31 set (only 2 bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only first 2 bits from source deposited to positions 27, 31
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x88000000,
        "First 2 source bits deposited to positions 27, 31"
    );
}

#[test]
fn test_pdep_zero_source() {
    // PDEP with zero source
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // source: zero
    regs.rcx = 0xFFFFFFFF; // mask: all bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "Zero source should produce zero result"
    );
}

#[test]
fn test_pdep_zero_mask() {
    // PDEP with zero mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // source: all bits set
    regs.rcx = 0; // mask: zero
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "Zero mask should produce zero result"
    );
}

#[test]
fn test_pdep_identity() {
    // PDEP with mask = all 1s should be identity
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678; // source
    regs.rcx = 0xFFFFFFFF; // mask: all bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Full mask should give identity"
    );
}

#[test]
fn test_pdep_64bit_basic() {
    // PDEP RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b111; // source
    regs.rcx = 0x0000_0001_0000_0100; // mask: bits 8, 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First 2 bits from source deposited to positions 8, 32
    assert_eq!(regs.rax, 0x0000_0001_0000_0100, "64-bit PDEP should work");
}

#[test]
fn test_pdep_64bit_high_bits() {
    // PDEP with high bits in 64-bit operands
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // source: 8 bits set
    regs.rcx = 0x8080_8080_8080_8080; // mask: every 8th bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8080_8080_8080_8080,
        "All source bits deposited to sparse positions"
    );
}

#[test]
fn test_pdep_alternating_pattern() {
    // PDEP with alternating pattern
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // source: lower 16 bits set
    regs.rcx = 0xAAAAAAAA; // mask: alternating pattern (even bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA,
        "Should deposit to alternating positions"
    );
}

#[test]
fn test_pdep_inverse_alternating() {
    // PDEP with inverse alternating pattern
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // source: lower 16 bits set
    regs.rcx = 0x55555555; // mask: inverse alternating (odd bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "Should deposit to odd positions"
    );
}

#[test]
fn test_pdep_byte_scatter() {
    // PDEP to scatter a byte across word
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // source: byte
    regs.rcx = 0x01010101; // mask: byte scatter pattern
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x01010101,
        "Byte scattered across dword"
    );
}

#[test]
fn test_pdep_extended_registers_r8_r9_r10() {
    // PDEP R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x33, 0xf5, 0xc2, // PDEP R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x1111; // One bit per nibble group
    regs.r10 = 0x0F0F0F0F; // mask: 4 nibbles, 4 bits each
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x01010101,
        "Extended registers should work"
    );
}

#[test]
fn test_pdep_r15_r14_r13() {
    // PDEP R15, R14, R13
    let code = [
        0xc4, 0x42, 0x8b, 0xf5, 0xfd, // PDEP R15, R14, R13
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0xFF;
    regs.r13 = 0x8040_2010_0804_0201; // mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x8040_2010_0804_0201, "R15/R14/R13 should work");
}

#[test]
fn test_pdep_mem32() {
    // PDEP EAX, EBX, [mem]
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // PDEP EAX, EBX, [DATA_ADDR]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111; // One bit per nibble to spread across mask groups
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xF0F0F0F0); // mask from memory - 4 groups of 4 bits
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x10101010,
        "Memory operand should work"
    );
}

#[test]
fn test_pdep_mem64() {
    // PDEP RAX, RBX, [mem]
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // PDEP RAX, RBX, [DATA_ADDR]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x01_01_01_01; // One bit per byte group
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0xFF00_FF00_FF00_FF00); // mask: 4 groups of 8 bits at bytes 1,3,5,7
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0100_0100_0100_0100,
        "64-bit memory operand should work"
    );
}

#[test]
fn test_pdep_single_mask_bit_positions() {
    // Test depositing to each bit position
    for pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1; // source: single bit
        regs.rcx = 1u64 << pos; // mask: single bit at position
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << pos,
            "Deposit to position {}",
            pos
        );
    }
}

#[test]
fn test_pdep_contiguous_mask() {
    // PDEP with contiguous mask (bitfield insertion)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // 8 bits to deposit
    regs.rcx = 0x00FFFF00; // mask: bits 8-23 (16 bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000FF00,
        "Deposit to contiguous field"
    );
}

#[test]
fn test_pdep_excess_source_bits() {
    // PDEP with more source bits than mask bits
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // source: all bits set
    regs.rcx = 0x000000FF; // mask: only 8 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFF, "Only mask bits should be set");
}

#[test]
fn test_pdep_power_of_two_mask() {
    // PDEP with power of two masks
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1;
        regs.rcx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << i,
            "Power of 2 mask at position {}",
            i
        );
    }
}

#[test]
fn test_pdep_64bit_all_positions() {
    // Test 64-bit deposition to various positions
    let test_cases = [
        (0x01u64, 0x0000_0000_0000_0001u64, 0x0000_0000_0000_0001u64),
        (0x01u64, 0x8000_0000_0000_0000u64, 0x8000_0000_0000_0000u64),
        (0x03u64, 0x0000_0001_0000_0001u64, 0x0000_0001_0000_0001u64),
        (0xFFu64, 0x0101_0101_0101_0101u64, 0x0101_0101_0101_0101u64),
    ];

    for (src, mask, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src;
        regs.rcx = *mask;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, *expected,
            "PDEP({:016x}, {:016x}) = {:016x}",
            src, mask, expected
        );
    }
}

#[test]
fn test_pdep_nibble_extraction() {
    // PDEP to extract and deposit nibbles
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0F; // nibble
    regs.rcx = 0x0000F000; // mask for nibble at bits 12-15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0000F000, "Nibble deposited");
}

#[test]
fn test_pdep_bit_reversal_helper() {
    // PDEP can help with bit manipulation
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010;
    regs.rcx = 0x00F00000; // deposit to high nibble
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00A00000,
        "Bits deposited to target field"
    );
}

#[test]
fn test_pdep_preserves_source() {
    // PDEP should not modify source operands
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0xABCDEF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "RBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xABCDEF00, "RCX should be unchanged");
}

#[test]
fn test_pdep_flags_unaffected() {
    // PDEP should not modify flags
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0xFF00FF00;
    regs.rflags = 0x2 | (1 << 0) | (1 << 6) | (1 << 7) | (1 << 11); // Set CF, ZF, SF, OF
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_pdep_sequential_bits() {
    // PDEP with sequential source bits
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b11110000; // bits 4-7
    regs.rcx = 0x0000FFFF; // mask: lower 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xF0, "Sequential bits deposited");
}

#[test]
fn test_pdep_maximum_spread() {
    // PDEP with maximum bit spreading
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // 8 bits
    regs.rcx = 0x0101_0101_0101_0101; // spread across 64 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0101_0101_0101_0101, "Maximum spread deposition");
}

#[test]
fn test_pdep_mask_equals_source() {
    // PDEP when mask pattern equals source pattern
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0F0F0F0F;
    regs.rcx = 0xFFFFFFFF; // Full mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0F0F0F0F, "Identity with full mask");
}

#[test]
fn test_pdep_single_byte_masks() {
    // Test depositing to each byte separately
    for byte_pos in 0..4 {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFF;
        regs.rcx = 0xFFu64 << (byte_pos * 8);
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0xFFu64 << (byte_pos * 8),
            "Deposit to byte {}",
            byte_pos
        );
    }
}

#[test]
fn test_pdep_complex_pattern() {
    // PDEP with complex bit pattern
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // 16 source bits (all 1s)
    regs.rcx = 0xFF00FF00; // 16 mask positions (bytes 1 and 3)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // With 16 bits in mask, deposit first 16 bits from source to bytes 1 and 3
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFF00FF00,
        "Complex pattern deposited"
    );
}

#[test]
fn test_pdep_bit_packing() {
    // Practical use: pack bits from source to compact form
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // 8 bits to pack (one per nibble)
    regs.rcx = 0x88888888; // scatter to bit 3 of each nibble (8 positions)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x88888888, "Bit packing complete");
}

#[test]
fn test_pdep_field_insertion() {
    // Use PDEP for bitfield insertion
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3F; // 6-bit value
    regs.rcx = 0x00003F00; // mask for bits 8-13
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00003F00,
        "Field insertion successful"
    );
}

#[test]
fn test_pdep_morton_code_helper() {
    // PDEP can help with Morton codes (Z-order curves)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // 16 bits
    regs.rcx = 0x55555555; // odd bit positions
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x55555555, "Morton code deposition");
}

#[test]
fn test_pdep_color_channel_scatter() {
    // Scatter color channel bits (practical graphics use)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // 8-bit color value
    regs.rcx = 0x00FF0000; // red channel position
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00FF0000, "Color channel scattered");
}

#[test]
fn test_pdep_complement_patterns() {
    // Test complementary mask patterns
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000FFFF;
    regs.rcx = 0xFFFF0000; // upper half
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF0000, "Complementary pattern");
}

#[test]
fn test_pdep_signed_interpretation() {
    // PDEP with values that look negative in signed interpretation
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // -1 in signed
    regs.rcx = 0x80000001; // bits 0 and 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000001,
        "Signed values handled correctly"
    );
}

#[test]
fn test_pdep_consecutive_calls() {
    // Multiple PDEP operations in sequence
    let code = [
        0xc4, 0xe2, 0x72, 0xf5, 0xc1, // PDEP EAX, ECX, ECX
        0xc4, 0xe2, 0x63, 0xf5, 0xd0, // PDEP EDX, EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0x0000FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: PDEP EAX, ECX, ECX = 0xFFFF (identity on lower bits)
    // Second: PDEP EDX, EBX, EAX = deposit EBX bits using EAX mask
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xFF, "Consecutive PDEP operations");
}

#[test]
fn test_pdep_one_bit_per_byte() {
    // Deposit one bit to each byte
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0F; // 4 bits
    regs.rcx = 0x01010101; // one bit per byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x01010101, "One bit per byte");
}

#[test]
fn test_pdep_64bit_boundary_crossing() {
    // Test deposition across 32-bit boundary
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0x0000_0001_0000_0001; // bits at positions 0 and 32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000_0001_0000_0001,
        "Deposition across 32-bit boundary"
    );
}

#[test]
fn test_pdep_high_source_low_mask() {
    // High bits in source, low bits in mask
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00_0000_0000_0000; // high bits
    regs.rcx = 0x0000_0000_0000_00FF; // low mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only first 8 bits of source used (all 0s in this case)
    assert_eq!(regs.rax, 0, "High source bits not used with low mask");
}

#[test]
fn test_pdep_bitboard_chess() {
    // Chess bitboard application
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // pieces
    regs.rcx = 0x0000_0000_0000_00FF; // first rank
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "Bitboard deposition");
}

#[test]
fn test_pdep_all_registers_32bit() {
    // Test various register combinations
    let test_cases = [
        ([0xc4, 0xe2, 0x73, 0xf5, 0xc2], 1, 2, 0xFF, 0xAA), // PDEP EAX, ECX, EDX
        ([0xc4, 0xe2, 0x5b, 0xf5, 0xe0], 4, 5, 0xFF, 0x55), // PDEP ESP, EDX, EBP
        ([0xc4, 0xe2, 0x43, 0xf5, 0xf7], 6, 7, 0xFF, 0xFF), // PDEP ESI, EBX, EDI
    ];

    for (encoding, dst_idx, src_idx, src_val, mask_val) in test_cases {
        let mut code = encoding.to_vec();
        code.push(0xf4);

        let mut regs = Registers::default();
        match src_idx {
            1 => regs.rcx = src_val,
            2 => regs.rdx = src_val,
            3 => regs.rbx = src_val,
            4 => regs.rsp = src_val,
            5 => regs.rbp = src_val,
            _ => {}
        }

        let mask_reg_val = mask_val as u64;
        match dst_idx {
            2 => regs.rdx = mask_reg_val,
            5 => regs.rbp = mask_reg_val,
            7 => regs.rdi = mask_reg_val,
            _ => {}
        }

        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();
        // Just verify it executes without error
    }
}

#[test]
fn test_pdep_sparse_random_pattern() {
    // Random sparse pattern test
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xF; // 4 source bits all set
    regs.rcx = 0x10204080; // sparse mask: bits 7, 14, 21, 28
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should deposit first 4 bits of source to positions 7, 14, 21, 28
    let expected = 0x10204080; // all 4 mask bits will be set
    assert_eq!(regs.rax & 0xFFFFFFFF, expected, "Sparse random pattern");
}
