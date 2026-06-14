use crate::common::*;
use rax::cpu::Registers;

// BLSMSK - Get Mask Up to Lowest Set Bit (BMI1)
// Sets all the lower bits of the destination operand to 1 up to and including the lowest set bit
// in the source operand. All other bits are cleared.
// This is equivalent to: dest = src ^ (src - 1)
// ZF is cleared, CF is set if source is zero, SF is updated based on result, OF is cleared.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /2   BLSMSK r32, r/m32   - Create mask from lowest set bit
// VEX.NDD.LZ.0F38.W1 F3 /2   BLSMSK r64, r/m64   - Create mask from lowest set bit

#[test]
fn test_blsmsk_eax_ebx_bit_0() {
    // BLSMSK EAX, EBX - mask up to bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0001,
        "EAX should contain mask up to bit 0"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (source is non-zero)"
    );
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source is non-zero)"
    );
}

#[test]
fn test_blsmsk_eax_ebx_bit_3() {
    // BLSMSK EAX, EBX - mask up to bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1111,
        "EAX should contain mask up to bit 3 (bits 0-3)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_blsmsk_eax_ebx_bit_31() {
    // BLSMSK EAX, EBX - mask up to bit 31
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // only bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should contain mask up to bit 31 (all bits)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_rax_rbx_bit_0() {
    // BLSMSK RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0b0000_0001, "RAX should contain mask up to bit 0");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_rax_rbx_bit_63() {
    // BLSMSK RAX, RBX - mask up to bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // only bit 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFF_FFFF_FFFF_FFFF,
        "RAX should contain mask up to bit 63 (all bits)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_zero_source() {
    // BLSMSK with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be all ones (src ^ (src-1) = 0 ^ -1)"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (BLSMSK clears ZF)"
    );
    assert!(cf_set(regs.rflags), "CF should be set (source is zero)");
}

#[test]
fn test_blsmsk_multiple_bits_uses_lowest() {
    // BLSMSK should use only the lowest set bit
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1000; // bits 3, 5, 7 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_1111,
        "EAX should contain mask up to bit 3 (lowest)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_all_bits_set() {
    // BLSMSK with all bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1,
        "EAX should contain mask up to bit 0"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_alternating_pattern() {
    // BLSMSK with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b11,
        "EAX should contain mask up to bit 1"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_alternating_pattern_inverted() {
    // BLSMSK with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1,
        "EAX should contain mask up to bit 0"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_single_bit_positions() {
    // Test each individual bit position
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << (bit_pos + 1)) - 1;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "EAX should contain mask for bit {}",
            bit_pos
        );
        assert!(
            !zf_set(regs.rflags),
            "ZF should be clear for bit {}",
            bit_pos
        );
    }
}

#[test]
fn test_blsmsk_with_extended_registers() {
    // BLSMSK R8D, R9D
    let code = [
        0xc4, 0xc2, 0x38, 0xf3, 0xd1, // BLSMSK R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b0001_0000; // bit 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0001_1111,
        "R8D should contain mask up to bit 4"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsmsk_r15() {
    // BLSMSK R15, R15
    let code = [
        0xc4, 0xc2, 0x80, 0xf3, 0xd7, // BLSMSK R15, R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x1_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u64 << 33) - 1;
    assert_eq!(regs.r15, expected, "R15 should contain mask up to bit 32");
}

#[test]
fn test_blsmsk_mem32() {
    // BLSMSK EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00001000); // bit 12 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "EAX should contain mask up to bit 12"
    );
}

#[test]
fn test_blsmsk_mem64() {
    // BLSMSK RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSMSK RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u64 << 41) - 1;
    assert_eq!(regs.rax, expected, "RAX should contain mask up to bit 40");
}

#[test]
fn test_blsmsk_trailing_zeros() {
    // BLSMSK creates mask for trailing zeros + 1
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "EAX should contain 13-bit mask"
    );
}

#[test]
fn test_blsmsk_sparse_pattern() {
    // BLSMSK with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u32 << 13) - 1;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "EAX should contain mask up to bit 12 (lowest)"
    );
}

#[test]
fn test_blsmsk_preserves_source() {
    // BLSMSK should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsmsk_vs_xor_sub1() {
    // BLSMSK is equivalent to src ^ (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    let mut regs = Registers::default();
    regs.rbx = value as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = value ^ (value.wrapping_sub(1));
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "BLSMSK should equal src ^ (src-1)"
    );
}

#[test]
fn test_blsmsk_power_of_two() {
    // BLSMSK of power of two creates mask with that many bits
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = (1u64 << (i + 1)) - 1;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BLSMSK(2^{}) should create {}-bit mask",
            i,
            i + 1
        );
    }
}

#[test]
fn test_blsmsk_consecutive_bits() {
    // BLSMSK with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u32 << 17) - 1;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "EAX should contain mask up to bit 16"
    );
}

#[test]
fn test_blsmsk_sign_bit() {
    // BLSMSK with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set (bit 31)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should contain all bits"
    );
}

#[test]
fn test_blsmsk_creates_bit_masks() {
    // BLSMSK can create various bit masks
    let test_cases = vec![
        (0x00000001, 0x00000001), // 1-bit mask
        (0x00000002, 0x00000003), // 2-bit mask
        (0x00000004, 0x00000007), // 3-bit mask
        (0x00000008, 0x0000000F), // 4-bit mask
        (0x00000010, 0x0000001F), // 5-bit mask
        (0x00000100, 0x000001FF), // 9-bit mask
        (0x00010000, 0x0001FFFF), // 17-bit mask
    ];

    for (input, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BLSMSK(0x{:08X}) should be 0x{:08X}",
            input,
            expected
        );
    }
}

#[test]
fn test_blsmsk_high_bits_64() {
    // BLSMSK with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u64 << 60) - 1;
    assert_eq!(regs.rax, expected, "RAX should contain 60-bit mask");
}

#[test]
fn test_blsmsk_mixed_high_low() {
    // BLSMSK with both high and low bits, should use lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xd3, // BLSMSK RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u64 << 9) - 1;
    assert_eq!(
        regs.rax, expected,
        "RAX should contain 9-bit mask (up to bit 8)"
    );
}

#[test]
fn test_blsmsk_clear_sf_of() {
    // BLSMSK clears OF and updates SF based on result
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xd3, // BLSMSK EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00001000;
    regs.rflags = 0x2 | (1 << 7) | (1 << 11); // Set SF and OF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!sf_set(regs.rflags), "SF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}
