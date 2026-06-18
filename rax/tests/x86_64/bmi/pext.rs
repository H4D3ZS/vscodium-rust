use crate::common::*;
use rax::cpu::Registers;

// PEXT - Parallel Bits Extract (BMI2)
// Extracts bits from source operand at positions specified by mask.
// Each set bit in the mask corresponds to an extracted bit from source.
// Extracted bits are packed contiguously in the result starting from LSB.
//
// Opcodes:
// VEX.NDS.LZ.F3.0F38.W0 F5 /r   PEXT r32, r32, r/m32   - Parallel bits extract (32-bit)
// VEX.NDS.LZ.F3.0F38.W1 F5 /r   PEXT r64, r64, r/m64   - Parallel bits extract (64-bit)

#[test]
fn test_pext_basic_single_bit() {
    // PEXT EAX, EBX, ECX - extract bit 0
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
        "Bit 0 should be extracted to position 0"
    );
}

#[test]
fn test_pext_extract_from_higher_position() {
    // PEXT EAX, EBX, ECX - extract bit 4 to position 0
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b10000; // source: bit 4 set
    regs.rcx = 0b10000; // mask: bit 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0001,
        "Bit 4 from source should be extracted to position 0"
    );
}

#[test]
fn test_pext_multiple_bits() {
    // PEXT with multiple bits
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010100; // source: bits 2, 4, 6 set
    regs.rcx = 0b1010100; // mask: bits 2, 4, 6 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bits from positions 2, 4, 6 -> packed to positions 0, 1, 2
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b111, "Bits should be packed to LSB");
}

#[test]
fn test_pext_sparse_extraction() {
    // PEXT with sparse mask
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x88000000; // source: bits 27, 31 set
    regs.rcx = 0x88000000; // mask: bits 27, 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bits at positions 27, 31 extracted to positions 0, 1
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b11, "Sparse bits extracted to LSB");
}

#[test]
fn test_pext_zero_source() {
    // PEXT with zero source
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_zero_mask() {
    // PEXT with zero mask
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_identity() {
    // PEXT with mask = all 1s should be identity
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_64bit_basic() {
    // PEXT RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0001_0000_0100; // bits 8, 32 set
    regs.rcx = 0x0000_0001_0000_0100; // mask: bits 8, 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bits at positions 8, 32 extracted to positions 0, 1
    assert_eq!(regs.rax, 0b11, "64-bit PEXT should work");
}

#[test]
fn test_pext_64bit_high_bits() {
    // PEXT with high bits in 64-bit operands
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8080_8080_8080_8080; // every 8th bit
    regs.rcx = 0x8080_8080_8080_8080; // mask: every 8th bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "8 sparse bits extracted to 8 LSBs");
}

#[test]
fn test_pext_alternating_pattern() {
    // PEXT with alternating pattern
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // source: alternating pattern (even bits)
    regs.rcx = 0xAAAAAAAA; // mask: alternating pattern (even bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF,
        "16 alternating bits extracted to lower 16"
    );
}

#[test]
fn test_pext_inverse_alternating() {
    // PEXT with inverse alternating pattern
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // source: inverse alternating (odd bits)
    regs.rcx = 0x55555555; // mask: inverse alternating (odd bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF,
        "16 odd bits extracted to lower 16"
    );
}

#[test]
fn test_pext_byte_gather() {
    // PEXT to gather bits from across dword
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x01010101; // byte pattern
    regs.rcx = 0x01010101; // mask: gather LSB from each byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0F, "4 bits gathered from 4 bytes");
}

#[test]
fn test_pext_extended_registers_r8_r9_r10() {
    // PEXT R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x32, 0xf5, 0xc2, // PEXT R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x0F0F0F0F; // Source with all mask bits set
    regs.r10 = 0x0F0F0F0F; // mask: 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0xFFFF,
        "Extended registers should work"
    );
}

#[test]
fn test_pext_r15_r14_r13() {
    // PEXT R15, R14, R13
    let code = [
        0xc4, 0x42, 0x8a, 0xf5, 0xfd, // PEXT R15, R14, R13
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x8040_2010_0804_0201;
    regs.r13 = 0x8040_2010_0804_0201; // mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xFF, "R15/R14/R13 should work");
}

#[test]
fn test_pext_mem32() {
    // PEXT EAX, EBX, [mem]
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // PEXT EAX, EBX, [DATA_ADDR]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xF0F0F0F0; // Source has all mask bits set
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xF0F0F0F0); // mask from memory (16 bits)
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF, "Memory operand should work");
}

#[test]
fn test_pext_mem64() {
    // PEXT RAX, RBX, [mem]
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // PEXT RAX, RBX, [DATA_ADDR]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00_FF00_FF00_FF00; // Source has all mask bits set
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0xFF00_FF00_FF00_FF00); // mask: 32 bits total
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "64-bit memory operand should work");
}

#[test]
fn test_pext_single_mask_bit_positions() {
    // Test extracting from each bit position
    for pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << pos; // source: single bit at position
        regs.rcx = 1u64 << pos; // mask: same bit
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1,
            "Extract from position {} to bit 0",
            pos
        );
    }
}

#[test]
fn test_pext_contiguous_mask() {
    // PEXT with contiguous mask (bitfield extraction)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12FFFF34; // source
    regs.rcx = 0x00FFFF00; // mask: bits 8-23 (16 bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF, "Extract contiguous field");
}

#[test]
fn test_pext_partial_match() {
    // PEXT where only some mask bits match source bits
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010; // source: bits 1, 3 set
    regs.rcx = 0b1111; // mask: bits 0-3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract bits at positions 0,1,2,3 -> values 0,1,0,1 -> 0b1010
    assert_eq!(regs.rax & 0xFFFFFFFF, 0b1010, "Extract with partial match");
}

#[test]
fn test_pext_power_of_two_mask() {
    // PEXT with power of two masks
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1,
            "Power of 2 mask at position {} extracts to bit 0",
            i
        );
    }
}

#[test]
fn test_pext_64bit_all_positions() {
    // Test 64-bit extraction from various positions
    let test_cases = [
        (
            0x0000_0000_0000_0001u64,
            0x0000_0000_0000_0001u64,
            0x0000_0000_0000_0001u64,
        ),
        (
            0x8000_0000_0000_0000u64,
            0x8000_0000_0000_0000u64,
            0x0000_0000_0000_0001u64,
        ),
        (
            0x0000_0001_0000_0001u64,
            0x0000_0001_0000_0001u64,
            0x0000_0000_0000_0003u64,
        ),
        (
            0x0101_0101_0101_0101u64,
            0x0101_0101_0101_0101u64,
            0x0000_0000_0000_00FFu64,
        ),
    ];

    for (src, mask, expected) in &test_cases {
        let code = [
            0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = *src;
        regs.rcx = *mask;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, *expected,
            "PEXT({:016x}, {:016x}) = {:016x}",
            src, mask, expected
        );
    }
}

#[test]
fn test_pext_nibble_extraction() {
    // PEXT to extract nibbles
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000F000; // nibble at bits 12-15
    regs.rcx = 0x0000F000; // mask for nibble
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0F, "Nibble extracted to LSB");
}

#[test]
fn test_pext_preserves_source() {
    // PEXT should not modify source operands
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_flags_unaffected() {
    // PEXT should not modify flags
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
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
fn test_pext_byte_extraction() {
    // Extract specific bytes
    for byte_pos in 0..4 {
        let code = [
            0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = 0xFFu64 << (byte_pos * 8);
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFFFFFF, 0xFF, "Extract byte {}", byte_pos);
    }
}

#[test]
fn test_pext_morton_code_helper() {
    // PEXT can help with Morton codes (Z-order curves)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // odd bit positions (Y coordinate in Morton)
    regs.rcx = 0x55555555; // mask: odd positions
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF, "Morton code extraction");
}

#[test]
fn test_pext_color_channel_extraction() {
    // Extract color channel from pixel (practical graphics use)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // pixel with red channel
    regs.rcx = 0x00FF0000; // red channel mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFF, "Color channel extracted");
}

#[test]
fn test_pext_bit_counting_helper() {
    // PEXT for gathering specific bits
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b10101010;
    regs.rcx = 0xFF; // extract lower byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b10101010, "Bits gathered");
}

#[test]
fn test_pext_signed_interpretation() {
    // PEXT with values that look negative in signed interpretation
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000001; // bits 0 and 31
    regs.rcx = 0x80000001; // mask: same bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b11,
        "Signed values handled correctly"
    );
}

#[test]
fn test_pext_consecutive_calls() {
    // Multiple PEXT operations in sequence
    let code = [
        0xc4, 0xe2, 0x73, 0xf5, 0xc1, // PEXT EAX, ECX, ECX
        0xc4, 0xe2, 0x62, 0xf5, 0xd0, // PEXT EDX, EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF0000;
    regs.rcx = 0x0000FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: PEXT EAX, ECX, ECX = 0xFFFF (identity on lower bits)
    // Second: PEXT EDX, EBX, EAX = extract from EBX using EAX mask
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0, "Consecutive PEXT operations");
}

#[test]
fn test_pext_all_bits_from_sparse() {
    // Extract all 1s from sparse pattern
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x88888888; // bits 3, 7, 11, 15, 19, 23, 27, 31
    regs.rcx = 0x88888888; // same mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFF, "8 sparse bits to 8 LSBs");
}

#[test]
fn test_pext_64bit_boundary_crossing() {
    // Test extraction across 32-bit boundary
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0001_0000_0001; // bits at positions 0 and 32
    regs.rcx = 0x0000_0001_0000_0001; // same mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0b11, "Extraction across 32-bit boundary");
}

#[test]
fn test_pext_maximum_gather() {
    // PEXT with maximum bit gathering
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0101_0101_0101_0101; // spread across 64 bits
    regs.rcx = 0x0101_0101_0101_0101; // same mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "Maximum gather to 8 LSBs");
}

#[test]
fn test_pext_bitboard_chess() {
    // Chess bitboard application - extract pieces from rank
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_00FF; // first rank with pieces
    regs.rcx = 0x0000_0000_0000_00FF; // extract first rank
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "Bitboard extraction");
}

#[test]
fn test_pext_pdep_inverse() {
    // PEXT is inverse of PDEP
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX (pp=3 = F2)
        0xc4, 0xe2, 0x7a, 0xf5, 0xd0, // PEXT EDX, EAX, ECX (pp=2 = F3, vvvv=0=EAX)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0xABCDEF00; // arbitrary mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PEXT(PDEP(x, mask), mask) should equal x (for bits that fit in mask)
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xFF, "PEXT is inverse of PDEP");
}

#[test]
fn test_pext_high_mask_low_source() {
    // High bits in mask, extract from low source
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0000_0000_00FF; // low bits
    regs.rcx = 0xFF00_0000_0000_0000; // high mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // High mask bits don't match low source - result is 0
    assert_eq!(regs.rax, 0, "High mask with low source gives zero");
}

#[test]
fn test_pext_extract_high_byte() {
    // Extract highest byte
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xABCDEF12;
    regs.rcx = 0xFF000000; // highest byte mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xAB, "Extract highest byte");
}

#[test]
fn test_pext_every_other_bit() {
    // Extract every other bit
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0xAAAAAAAA; // even positions
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF, "Every other bit extracted");
}

#[test]
fn test_pext_word_extraction() {
    // Extract specific word (16-bit)
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1234ABCD;
    regs.rcx = 0x0000FFFF; // lower word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xABCD, "Extract lower word");
}

#[test]
fn test_pext_upper_word() {
    // Extract upper word
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1234ABCD;
    regs.rcx = 0xFFFF0000; // upper word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x1234, "Extract upper word");
}

#[test]
fn test_pext_bit_permutation() {
    // PEXT for bit permutation applications
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0x0F0F0F0F; // extract nibbles
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468,
        "Bit permutation via extraction"
    );
}

#[test]
fn test_pext_network_byte_order() {
    // Practical use: extract specific fields from network protocol
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678; // protocol header
    regs.rcx = 0x00FFF000; // extract 12-bit field
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x345, "Protocol field extracted");
}

#[test]
fn test_pext_all_registers_32bit() {
    // Test various register combinations
    let test_cases = [
        ([0xc4, 0xe2, 0x73, 0xf5, 0xc2], 1, 2, 0xFFFF, 0xAAAA), // PEXT EAX, ECX, EDX
        ([0xc4, 0xe2, 0x5b, 0xf5, 0xe0], 4, 5, 0xFFFF, 0x5555), // PEXT ESP, EDX, EBP
        ([0xc4, 0xe2, 0x43, 0xf5, 0xf7], 6, 7, 0xFFFF, 0xFFFF), // PEXT ESI, EBX, EDI
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
fn test_pext_compress_sparse_data() {
    // Compress sparse data structure
    let code = [
        0xc4, 0xe2, 0xe2, 0xf5, 0xc1, // PEXT RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0001_0001_0001_0001; // sparse pattern
    regs.rcx = 0xFFFF_FFFF_FFFF_FFFF; // extract all
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0001_0001_0001_0001,
        "Sparse data identity with full mask"
    );
}

#[test]
fn test_pext_interleaved_data() {
    // Extract interleaved data
    let code = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xCCCCCCCC; // 11001100 pattern
    regs.rcx = 0xF0F0F0F0; // extract high nibbles
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xCCCC, "Interleaved data extracted");
}
