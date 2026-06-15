use crate::common::*;
use rax::cpu::Registers;

// PDEP - Parallel Bits Deposit (BMI2)
// Deposits bits from the source operand into positions specified by the mask (second source).
// For each set bit in the mask, the corresponding bit position receives the next bit from the source.
// Bits in positions corresponding to clear mask bits are zeroed.
// This is the inverse operation of PEXT.
//
// Opcodes:
// VEX.NDS.LZ.F2.0F38.W0 F5 /r   PDEP r32, r32, r/m32   - Parallel deposit of bits
// VEX.NDS.LZ.F2.0F38.W1 F5 /r   PDEP r64, r64, r/m64   - Parallel deposit of bits

#[test]
fn test_pdep_eax_ebx_ecx_all_mask() {
    // PDEP EAX, EBX, ECX - mask all ones (identity)
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0xFFFFFFFF; // all mask bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should equal source (identity with full mask)"
    );
}

#[test]
fn test_pdep_eax_ebx_ecx_zero_mask() {
    // PDEP EAX, EBX, ECX - mask all zeros
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0x00000000; // no mask bits set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be zero (no deposit positions)"
    );
}

#[test]
fn test_pdep_eax_ebx_ecx_single_bit_mask() {
    // PDEP with single bit mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 from source
    regs.rcx = 0b0000_1000; // deposit to bit 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1000,
        "EAX should have bit 3 set"
    );
}

#[test]
fn test_pdep_eax_ebx_ecx_alternating_mask() {
    // PDEP with alternating mask 0101...0101
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // lower 16 bits set
    regs.rcx = 0x55555555; // alternating bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Deposits 16 bits into 16 alternating positions
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "EAX should have alternating pattern"
    );
}

#[test]
fn test_pdep_eax_ebx_ecx_low_nibble() {
    // Deposit lower 4 bits into specific positions
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111; // 4 bits
    regs.rcx = 0x0F00; // mask for bits 8-11
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0F00,
        "EAX should have bits 8-11 set"
    );
}

#[test]
fn test_pdep_rax_rbx_rcx_64bit() {
    // PDEP RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0xFF00000000000000; // deposit to high byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFF00000000000000,
        "RAX should have high byte set"
    );
}

#[test]
fn test_pdep_sparse_mask() {
    // PDEP with sparse mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b111; // 3 bits
    regs.rcx = 0x80001001; // bits 0, 12, 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80001001,
        "EAX should have bits at mask positions"
    );
}

#[test]
fn test_pdep_with_extended_registers() {
    // PDEP R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x33, 0xf5, 0xc2, // PDEP R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFF;
    regs.r10 = 0x00FF0000; // deposit to bits 16-23
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x00FF0000,
        "R8D should have bits 16-23 set"
    );
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
    regs.rbx = 0xF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x000F0000); // mask bits 16-19
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000F0000,
        "EAX should have bits deposited from memory mask"
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
    regs.rbx = 0xFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x00FF000000000000); // mask bits 48-55
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x00FF000000000000,
        "RAX should have bits deposited from memory mask"
    );
}

#[test]
fn test_pdep_preserves_sources() {
    // PDEP should not modify source operands
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0xAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xAAAAAAAA, "ECX should be unchanged");
}

#[test]
fn test_pdep_sequential_deposits() {
    // Test depositing bits sequentially
    let test_cases = vec![
        (0b1, 0x00000001, 0x00000001),
        (0b11, 0x00000003, 0x00000003),
        (0b111, 0x00000007, 0x00000007),
        (0b1111, 0x0000000F, 0x0000000F),
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = src;
        regs.rcx = mask;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "PDEP(0x{:X}, 0x{:X}) should be 0x{:X}",
            src,
            mask,
            expected
        );
    }
}

#[test]
fn test_pdep_extract_nibbles() {
    // Deposit nibbles to specific positions
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xABCD; // 4 nibbles
    regs.rcx = 0x0F0F0F0F; // every other nibble position
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Each nibble deposited to alternating positions
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0A0B0C0D,
        "Should deposit nibbles to alternating positions"
    );
}

#[test]
fn test_pdep_bit_scatter() {
    // Scatter bits according to mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // 8 bits
    regs.rcx = 0x01010101; // scatter to bytes
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x01010101,
        "Should scatter bits across byte positions"
    );
}

#[test]
fn test_pdep_power_of_two_masks() {
    // Test with power of 2 masks
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1; // single bit
        regs.rcx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            1u64 << bit_pos,
            "Should deposit to bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_pdep_inverse_of_pext() {
    // PDEP is the inverse of PEXT with the same mask
    let code_pext = [
        0xc4, 0xe2, 0x62, 0xf5, 0xc1, // PEXT EAX, EBX, ECX
        0xf4,
    ];
    let code_pdep = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    let mask = 0x0F0F0F0Fu64;

    // Apply PEXT
    let mut regs = Registers::default();
    regs.rbx = value;
    regs.rcx = mask;
    let (mut vcpu, _) = setup_vm(&code_pext, Some(regs));
    let regs_pext = run_until_hlt(&mut vcpu).unwrap();
    let extracted = regs_pext.rax & 0xFFFFFFFF;

    // Apply PDEP to result
    let mut regs = Registers::default();
    regs.rbx = extracted;
    regs.rcx = mask;
    let (mut vcpu, _) = setup_vm(&code_pdep, Some(regs));
    let regs_pdep = run_until_hlt(&mut vcpu).unwrap();

    let masked_original = value & mask;
    assert_eq!(
        regs_pdep.rax & 0xFFFFFFFF,
        masked_original,
        "PDEP(PEXT(x, mask), mask) should equal x & mask"
    );
}

#[test]
fn test_pdep_excess_source_bits() {
    // More source bits than mask bits - excess ignored
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // all bits
    regs.rcx = 0x0000000F; // only 4 mask bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000F,
        "Should use only first 4 source bits"
    );
}

#[test]
fn test_pdep_byte_expansion() {
    // Expand byte to word with gaps
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rcx = 0x00FF00FF; // two bytes separated
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x000000FF,
        "Should expand byte to separated positions"
    );
}

#[test]
fn test_pdep_zero_source() {
    // Zero source always produces zero
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "Zero source should produce zero");
}

#[test]
fn test_pdep_64bit_high_positions() {
    // Deposit to high positions in 64-bit
    let code = [
        0xc4, 0xe2, 0xe3, 0xf5, 0xc1, // PDEP RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF;
    regs.rcx = 0xFFFF000000000000; // high 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFF000000000000,
        "Should deposit to high positions"
    );
}

#[test]
fn test_pdep_pattern_generation() {
    // Generate patterns by depositing bits
    let test_cases = vec![
        (0x1, 0x11111111, 0x00000001), // deposit single bit
        (0x3, 0x33333333, 0x00000003), // deposit two bits
        (0xF, 0x0F0F0F0F, 0x0000000F), // deposit nibble
    ];

    for (src, mask, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = src;
        regs.rcx = mask;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "PDEP(0x{:X}, 0x{:X}) should be 0x{:X}",
            src,
            mask,
            expected
        );
    }
}

#[test]
fn test_pdep_consecutive_mask_bits() {
    // Consecutive mask bits act like simple masking
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0x0000FFFF; // lower 16 bits consecutive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00005678,
        "Consecutive mask preserves lower bits"
    );
}

#[test]
fn test_pdep_field_packing() {
    // Pack multiple fields using mask
    let code = [
        0xc4, 0xe2, 0x63, 0xf5, 0xc1, // PDEP EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b11111111; // 8 bits to pack
    regs.rcx = 0x0F0000F0; // two 4-bit fields
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0F0000F0,
        "Should pack bits into fields"
    );
}
