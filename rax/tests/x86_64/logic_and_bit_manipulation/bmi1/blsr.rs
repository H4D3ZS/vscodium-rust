use crate::common::*;
use rax::cpu::Registers;

// BLSR - Reset Lowest Set Bit (BMI1)
// Resets the lowest set bit in the source operand and writes the result to the destination.
// All other bits are unchanged.
// This is equivalent to: dest = src & (src - 1)
// ZF is set if result is zero, CF is set if source is zero, SF is updated based on result, OF is cleared.
//
// Opcodes:
// VEX.NDD.LZ.0F38.W0 F3 /1   BLSR r32, r/m32   - Reset lowest set bit
// VEX.NDD.LZ.0F38.W1 F3 /1   BLSR r64, r/m64   - Reset lowest set bit

#[test]
fn test_blsr_eax_ebx_bit_0() {
    // BLSR EAX, EBX - reset bit 0 (only bit set)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001; // bit 0 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0000,
        "EAX should be zero (bit 0 reset)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (source was non-zero)"
    );
}

#[test]
fn test_blsr_eax_ebx_bit_3() {
    // BLSR EAX, EBX - reset bit 3
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_1000; // only bit 3 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_0000, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_eax_ebx_multiple_bits() {
    // BLSR EAX, EBX - reset lowest of multiple bits
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1010_1000; // bits 3, 5, 7 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b1010_0000,
        "EAX should have bit 3 reset (bits 5,7 remain)"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}

#[test]
fn test_blsr_eax_ebx_bit_31() {
    // BLSR EAX, EBX - reset bit 31
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // only bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_rax_rbx_bit_0() {
    // BLSR RAX, RBX - 64-bit version
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0b0000_0000, "RAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_rax_rbx_bit_63() {
    // BLSR RAX, RBX - reset bit 63
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0000; // only bit 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000_0000_0000_0000, "RAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_zero_source() {
    // BLSR with zero source
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0; // zero source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero (0 & -1 = 0)");
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(cf_set(regs.rflags), "CF should be set (source was zero)");
}

#[test]
fn test_blsr_all_bits_set() {
    // BLSR with all bits set should reset bit 0
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX should have bit 0 reset"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsr_alternating_pattern() {
    // BLSR with alternating pattern 1010...1010
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010 (bit 1 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAA8,
        "EAX should have bit 1 reset"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsr_alternating_pattern_inverted() {
    // BLSR with alternating pattern 0101...0101
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x55555555; // 0101...0101 (bit 0 is lowest set bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555554,
        "EAX should have bit 0 reset"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsr_single_bit_positions() {
    // Test each individual bit position
    for bit_pos in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            0,
            "EAX should be zero for single bit {}",
            bit_pos
        );
        assert!(zf_set(regs.rflags), "ZF should be set for bit {}", bit_pos);
    }
}

#[test]
fn test_blsr_with_extended_registers() {
    // BLSR R8D, R9D
    let code = [
        0xc4, 0xc2, 0x38, 0xf3, 0xc9, // BLSR R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b0001_1000; // bits 3 and 4 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0001_0000,
        "R8D should have bit 3 reset (bit 4 remains)"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_blsr_r15() {
    // BLSR R15, R15
    let code = [
        0xc4, 0xc2, 0x80, 0xf3, 0xcf, // BLSR R15, R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x1_0000_0001; // bits 0 and 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x1_0000_0000,
        "R15 should have bit 0 reset (bit 32 remains)"
    );
}

#[test]
fn test_blsr_mem32() {
    // BLSR EAX, [mem]
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR EAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFF000); // bits 12-31 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFE000,
        "EAX should have bit 12 reset"
    );
}

#[test]
fn test_blsr_mem64() {
    // BLSR RAX, [mem]
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BLSR RAX, [DATA_ADDR]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x100_0000_0001); // bits 0 and 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x100_0000_0000,
        "RAX should have bit 0 reset (bit 40 remains)"
    );
}

#[test]
fn test_blsr_trailing_zeros() {
    // BLSR resets bit at position of trailing zeros count
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFF000; // 12 trailing zeros, bit 12 is lowest set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFE000,
        "EAX should have bit 12 reset"
    );
}

#[test]
fn test_blsr_sparse_pattern() {
    // BLSR with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001000; // bits 12 and 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should have bit 12 reset (bit 31 remains)"
    );
}

#[test]
fn test_blsr_preserves_source() {
    // BLSR should not modify source operand
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_blsr_vs_and_sub1() {
    // BLSR is equivalent to src & (src - 1)
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let value = 0x12345678u32;
    let mut regs = Registers::default();
    regs.rbx = value as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = value & (value.wrapping_sub(1));
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "BLSR should equal src & (src-1)"
    );
}

#[test]
fn test_blsr_power_of_two() {
    // BLSR of power of two returns zero
    for i in 0..32 {
        let code = [
            0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFFFFFF, 0, "BLSR(2^{}) should be zero", i);
        assert!(zf_set(regs.rflags), "ZF should be set for 2^{}", i);
    }
}

#[test]
fn test_blsr_consecutive_bits() {
    // BLSR with consecutive bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00FF0000; // bits 16-23 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00FE0000,
        "EAX should have bit 16 reset (bits 17-23 remain)"
    );
}

#[test]
fn test_blsr_sign_bit() {
    // BLSR with sign bit set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // sign bit set (bit 31)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_iterative_clearing() {
    // Use BLSR to clear bits one at a time
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0b1111u64; // 4 bits set
    let expected_values = vec![0b1110, 0b1100, 0b1000, 0b0000];

    for &expected in &expected_values {
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Should progressively clear bits"
        );
        value = regs.rax;
    }
}

#[test]
fn test_blsr_high_bits_64() {
    // BLSR with high bits in 64-bit operand
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0800_0000_0000_0000; // bit 59 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_blsr_mixed_high_low() {
    // BLSR with both high and low bits, should reset lowest
    let code = [
        0xc4, 0xe2, 0xf8, 0xf3, 0xcb, // BLSR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000_0000_0000_0100; // bits 8 and 63 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX should have bit 8 reset (bit 63 remains)"
    );
}

#[test]
fn test_blsr_count_set_bits() {
    // BLSR can be used to count set bits by iterating until zero
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];

    let mut value = 0x12345678u64;
    let mut count = 0;

    while value != 0 {
        count += 1;
        let mut regs = Registers::default();
        regs.rbx = value;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        value = regs.rax & 0xFFFFFFFF;
    }

    assert_eq!(
        count,
        0x12345678u32.count_ones(),
        "Should count all set bits"
    );
}

#[test]
fn test_blsr_clear_sf_of() {
    // BLSR clears OF and updates SF based on result
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
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

#[test]
fn test_blsr_complement_of_blsi() {
    // BLSR removes lowest bit, BLSI isolates it - they're complementary
    let code_blsr = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let code_blsi = [
        0xc4, 0xe2, 0x78, 0xf3, 0xdb, // BLSI EAX, EBX
        0xf4,
    ];

    let value = 0xAAAAAAAAu64;

    // Get BLSR result
    let mut regs = Registers::default();
    regs.rbx = value;
    let (mut vcpu, _) = setup_vm(&code_blsr, Some(regs));
    let regs_blsr = run_until_hlt(&mut vcpu).unwrap();

    // Get BLSI result
    let mut regs = Registers::default();
    regs.rbx = value;
    let (mut vcpu, _) = setup_vm(&code_blsi, Some(regs));
    let regs_blsi = run_until_hlt(&mut vcpu).unwrap();

    // BLSR | BLSI should equal original value
    let combined = (regs_blsr.rax | regs_blsi.rax) & 0xFFFFFFFF;
    assert_eq!(
        combined, value,
        "BLSR | BLSI should reconstruct original value"
    );
}

#[test]
fn test_blsr_two_bits_set() {
    // BLSR with exactly two bits set
    let code = [
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0101; // bits 0 and 2 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0100,
        "EAX should have bit 0 reset (bit 2 remains)"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
}
