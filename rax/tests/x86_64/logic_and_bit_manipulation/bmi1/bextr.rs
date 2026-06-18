use crate::common::*;
use rax::cpu::Registers;

// BEXTR - Bit Field Extract (BMI1)
// Extracts contiguous bits from the first source operand using an index and length specified
// in the second source operand. Bits[7:0] of the second source specify the starting bit position.
// Bits[15:8] specify the length in bits to extract.
// The extracted bits are written to the destination register with zero extension.
// ZF is set if the extracted field is all zeros, CF is cleared, OF/SF/AF/PF are undefined.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F7 /r   BEXTR r32, r/m32, r32   - Extract bits from r/m32 using r32
// VEX.NDS.LZ.0F38.W1 F7 /r   BEXTR r64, r/m64, r64   - Extract bits from r/m64 using r64

#[test]
fn test_bextr_eax_ebx_ecx_basic() {
    // BEXTR EAX, EBX, ECX - extract 8 bits starting at bit 4
    // VEX.NDS.LZ.0F38.W0 F7 /r
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = (8 << 8) | 4; // length=8, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 8 bits starting at bit 4: bits 4-11 of 0x12345678
    // 0x12345678 >> 4 = 0x01234567, mask 8 bits = 0x67
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x67,
        "EAX should contain extracted bits"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_zero_result() {
    // BEXTR that extracts all zeros
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    regs.rcx = (8 << 8) | 4; // length=8, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_start_0() {
    // BEXTR starting at bit 0
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (16 << 8) | 0; // length=16, start=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF,
        "EAX should contain lower 16 bits"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_length_1() {
    // BEXTR with length=1 (extract single bit)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000010; // bit 4 set
    regs.rcx = (1 << 8) | 4; // length=1, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain 1");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_eax_ebx_ecx_length_0() {
    // BEXTR with length=0 should return 0
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (0 << 8) | 4; // length=0, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero (length=0)");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_bextr_eax_ebx_ecx_full_32bits() {
    // BEXTR extracting all 32 bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = (32 << 8) | 0; // length=32, start=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should contain all bits"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_rax_rbx_rcx_basic() {
    // BEXTR RAX, RBX, RCX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = (16 << 8) | 8; // length=16, start=8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 16 bits starting at bit 8
    // 0x123456789ABCDEF0 >> 8 = 0x00123456789ABCDE, mask 16 bits = 0xBCDE
    assert_eq!(regs.rax, 0xBCDE, "RAX should contain extracted bits");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_rax_rbx_rcx_high_bits() {
    // BEXTR extracting from high bits of 64-bit value
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF00_0000_0000_0000;
    regs.rcx = (8 << 8) | 56; // length=8, start=56
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFF, "RAX should contain top 8 bits");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_start_beyond_operand_size() {
    // BEXTR with start position beyond operand size
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (8 << 8) | 32; // length=8, start=32 (beyond 32-bit operand)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be zero (start beyond size)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_bextr_length_exceeds_remaining() {
    // BEXTR with length that would exceed operand size
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (20 << 8) | 20; // length=20, start=20 (would go to bit 40)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should extract bits 20-31 (12 bits), zero-extended
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFF,
        "EAX should contain remaining bits"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bextr_with_extended_registers() {
    // BEXTR R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x28, 0xf7, 0xc1, // BEXTR R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xABCDEF01;
    regs.r10 = (12 << 8) | 4; // length=12, start=4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract 12 bits starting at bit 4
    let expected = (0xABCDEF01 >> 4) & 0xFFF;
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        expected,
        "R8D should contain extracted bits"
    );
}

#[test]
fn test_bextr_mem32() {
    // BEXTR EAX, [mem], ECX
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BEXTR EAX, [DATA_ADDR], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = (8 << 8) | 8; // length=8, start=8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xAABBCCDD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract bits 8-15: 0xCC
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xCC,
        "EAX should contain extracted bits from memory"
    );
}

#[test]
fn test_bextr_mem64() {
    // BEXTR RAX, [mem], RCX
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BEXTR RAX, [DATA_ADDR], RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = (16 << 8) | 16; // length=16, start=16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Extract bits 16-31: 0x89AB
    assert_eq!(
        regs.rax, 0x89AB,
        "RAX should contain extracted bits from memory"
    );
}

#[test]
fn test_bextr_nibble_extraction() {
    // Extract individual nibbles (4-bit fields)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    for nibble_idx in 0..8 {
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = (4 << 8) | (nibble_idx * 4); // length=4, start=nibble_idx*4
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (value >> (nibble_idx * 4)) & 0xF;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Should extract nibble {}",
            nibble_idx
        );
    }
}

#[test]
fn test_bextr_byte_extraction() {
    // Extract individual bytes
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x12345678u64;
    for byte_idx in 0..4 {
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = (8 << 8) | (byte_idx * 8); // length=8, start=byte_idx*8
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (value >> (byte_idx * 8)) & 0xFF;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Should extract byte {}",
            byte_idx
        );
    }
}

#[test]
fn test_bextr_alternating_pattern() {
    // Extract from alternating bit pattern
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010
    regs.rcx = (8 << 8) | 0; // length=8, start=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAA,
        "EAX should contain extracted pattern"
    );
}

#[test]
fn test_bextr_single_bit_scan() {
    // Extract each individual bit
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];

    let value = 0x80000001u64; // bits 0 and 31 set
    for bit_idx in 0..32 {
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = (1 << 8) | bit_idx; // length=1, start=bit_idx
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = ((value >> bit_idx) & 1) as u64;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Should extract bit {}",
            bit_idx
        );
    }
}

#[test]
fn test_bextr_max_length_255() {
    // BEXTR with maximum length value (255)
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (255 << 8) | 0; // length=255, start=0 (will be clamped to 32 bits)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should contain all 32 bits"
    );
}

#[test]
fn test_bextr_preserves_source() {
    // BEXTR should not modify source operand
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = (8 << 8) | 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        (8 << 8) | 4,
        "ECX should be unchanged"
    );
}

#[test]
fn test_bextr_zero_extension() {
    // Verify that result is zero-extended
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Set all bits in RAX
    regs.rbx = 0x000000FF;
    regs.rcx = (8 << 8) | 0; // length=8, start=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 32-bit operation should zero upper 32 bits of RAX
    assert_eq!(
        regs.rax, 0xFF,
        "RAX should be zero-extended (upper bits cleared)"
    );
}

#[test]
fn test_bextr_mask_creation() {
    // BEXTR can be used to create bit masks
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (16 << 8) | 0; // length=16, start=0 - creates 16-bit mask
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF,
        "EAX should contain 16-bit mask"
    );
}

#[test]
fn test_bextr_field_alignment() {
    // Extract aligned multi-bit fields
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF;
    regs.rcx = (16 << 8) | 16; // length=16, start=16 - extract upper 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDEAD,
        "EAX should contain upper 16 bits"
    );
}

#[test]
fn test_bextr_unaligned_field() {
    // Extract unaligned field
    let code = [
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = (10 << 8) | 7; // length=10, start=7 - extract bits 7-16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (0xFFFFFFFF >> 7) & 0x3FF;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected,
        "EAX should contain 10 bits"
    );
}

#[test]
fn test_bextr_64bit_full_extraction() {
    // Extract full 64 bits
    let code = [
        0xc4, 0xe2, 0xf0, 0xf7, 0xc3, // BEXTR RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = (64 << 8) | 0; // length=64, start=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX should contain all 64 bits"
    );
}
