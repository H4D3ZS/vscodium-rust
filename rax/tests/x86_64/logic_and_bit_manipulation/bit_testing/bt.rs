use crate::common::*;
use rax::cpu::Registers;

// BT - Bit Test
// Tests a bit in a bit string (first operand) and stores the value in CF flag.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F A3 /r       BT r/m16, r16     - Test bit in r/m16, bit position in r16
// 0F A3 /r       BT r/m32, r32     - Test bit in r/m32, bit position in r32
// REX.W 0F A3 /r BT r/m64, r64     - Test bit in r/m64, bit position in r64
// 0F BA /4 ib    BT r/m16, imm8    - Test bit in r/m16, bit position = imm8
// 0F BA /4 ib    BT r/m32, imm8    - Test bit in r/m32, bit position = imm8
// REX.W 0F BA /4 ib BT r/m64, imm8 - Test bit in r/m64, bit position = imm8

#[test]
fn test_bt_ax_bx_bit_0() {
    // BT AX, BX - test bit 0
    let code = [
        0x66, 0x0f, 0xa3, 0xd8, // BT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // bit 0 set
    regs.rbx = 0; // test bit 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_ax_bx_bit_clear() {
    // BT AX, BX - test bit that is clear
    let code = [
        0x66, 0x0f, 0xa3, 0xd8, // BT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // only bit 0 set
    regs.rbx = 1; // test bit 1 (clear)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 1 is 0)");
}

#[test]
fn test_bt_ax_bx_bit_15() {
    // BT AX, BX - test MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xa3, 0xd8, // BT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // bit 15 set
    regs.rbx = 15; // test bit 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 15 is 1)");
}

#[test]
fn test_bt_eax_ebx_bit_0() {
    // BT EAX, EBX - test bit 0 (32-bit)
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_eax_ebx_bit_31() {
    // BT EAX, EBX - test MSB (bit 31)
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // bit 31 set
    regs.rbx = 31; // test bit 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 is 1)");
}

#[test]
fn test_bt_rax_rbx_bit_0() {
    // BT RAX, RBX - test bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xa3, 0xd8, // BT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_rax_rbx_bit_63() {
    // BT RAX, RBX - test MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xa3, 0xd8, // BT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0000; // bit 63 set
    regs.rbx = 63; // test bit 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 63 is 1)");
}

#[test]
fn test_bt_ax_imm8_bit_0() {
    // BT AX, imm8 - test bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xe0, 0x00, // BT AX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_ax_imm8_bit_15() {
    // BT AX, imm8 - test bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xe0, 0x0f, // BT AX, 15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 15 is 1)");
}

#[test]
fn test_bt_eax_imm8_bit_0() {
    // BT EAX, imm8 - test bit 0
    let code = [
        0x0f, 0xba, 0xe0, 0x00, // BT EAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_eax_imm8_bit_31() {
    // BT EAX, imm8 - test bit 31
    let code = [
        0x0f, 0xba, 0xe0, 0x1f, // BT EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 is 1)");
}

#[test]
fn test_bt_rax_imm8_bit_0() {
    // BT RAX, imm8 - test bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xe0, 0x00, // BT RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
}

#[test]
fn test_bt_rax_imm8_bit_63() {
    // BT RAX, imm8 - test bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xe0, 0x3f, // BT RAX, 63
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 63 is 1)");
}

#[test]
fn test_bt_eax_ebx_alternating_bits() {
    // Test alternating bit pattern
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010...1010
    regs.rbx = 1; // test bit 1 (should be 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 1 is 1 in 0xAAAAAAAA)"
    );
}

#[test]
fn test_bt_eax_ebx_alternating_bits_clear() {
    // Test alternating bit pattern - clear bit
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010...1010
    regs.rbx = 0; // test bit 0 (should be 0)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (bit 0 is 0 in 0xAAAAAAAA)"
    );
}

#[test]
fn test_bt_does_not_modify_operand() {
    // BT should not modify the operand, only set CF
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX: BT should not modify operand"
    );
}

#[test]
fn test_bt_with_extended_registers() {
    // BT R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xa3, 0xc8, // BT R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0b0000_0001;
    regs.r9 = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0000_0001,
        "R8D: BT should not modify operand"
    );
}

#[test]
fn test_bt_r15_imm8() {
    // BT R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xe7, 0x20, // BT R15, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x1_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 32 is 1)");
}

#[test]
fn test_bt_mem16_reg() {
    // BT [mem], BX
    let code = [
        0x66, 0x0f, 0xa3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BT [DATA_ADDR], BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 8; // test bit 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x0100); // bit 8 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 8 is 1 in memory)"
    );
}

#[test]
fn test_bt_mem32_reg() {
    // BT [mem], EBX
    let code = [
        0x0f, 0xa3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BT [DATA_ADDR], EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 16; // test bit 16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00010000); // bit 16 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 16 is 1 in memory)"
    );
}

#[test]
fn test_bt_mem64_reg() {
    // BT [mem], RBX
    let code = [
        0x48, 0x0f, 0xa3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BT [DATA_ADDR], RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 40; // test bit 40
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 40 is 1 in memory)"
    );
}

#[test]
fn test_bt_mem32_imm8() {
    // BT [mem], imm8
    let code = [
        0x0f, 0xba, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BT [DATA_ADDR], 12
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x1000); // bit 12 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 12 is 1 in memory)"
    );
}

#[test]
fn test_bt_all_bits_set() {
    // Test with all bits set
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 17; // test any bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (all bits are 1)");
}

#[test]
fn test_bt_all_bits_clear() {
    // Test with all bits clear
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 17; // test any bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (all bits are 0)");
}

#[test]
fn test_bt_bit_position_modulo_16() {
    // For 16-bit operands, bit position is taken modulo 16
    let code = [
        0x66, 0x0f, 0xa3, 0xd8, // BT AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // bit 0 set
    regs.rbx = 16; // position 16 % 16 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit position 16 wraps to 0)"
    );
}

#[test]
fn test_bt_bit_position_modulo_32() {
    // For 32-bit operands, bit position is taken modulo 32
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 32; // position 32 % 32 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit position 32 wraps to 0)"
    );
}

#[test]
fn test_bt_bit_position_modulo_64() {
    // For 64-bit operands, bit position is taken modulo 64
    let code = [
        0x48, 0x0f, 0xa3, 0xd8, // BT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 64; // position 64 % 64 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit position 64 wraps to 0)"
    );
}

#[test]
fn test_bt_sequential_bits() {
    // Test multiple bits in sequence
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX (test bit 0)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0101_0101; // bits 0, 2, 4, 6 set
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 is 1)");

    // Test bit 1 (clear)
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0101_0101;
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 1 is 0)");
}

#[test]
fn test_bt_preserves_other_registers() {
    // BT should only affect CF, no other registers
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 5;
    regs.rcx = 0xABCDEF00;
    regs.rdx = 0x11223344;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX should be unchanged");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 5, "EBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xABCDEF00, "ECX should be unchanged");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x11223344, "EDX should be unchanged");
}

#[test]
fn test_bt_high_bit_positions() {
    // Test high bit positions in 64-bit operand
    let code = [
        0x48, 0x0f, 0xa3, 0xd8, // BT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0800_0000_0000_0000; // bit 59 set
    regs.rbx = 59;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 59 is 1)");
}

#[test]
fn test_bt_single_bit_patterns() {
    // Test each individual bit in a 32-bit value
    for bit_pos in 0..32 {
        let code = [
            0x0f, 0xa3, 0xd8, // BT EAX, EBX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = 1u64 << bit_pos;
        regs.rbx = bit_pos as u64;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set for bit {}", bit_pos);
    }
}

#[test]
fn test_bt_negative_like_values() {
    // Test with sign bit set (treated as unsigned by BT)
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // MSB set (would be negative if signed)
    regs.rbx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 is 1)");
}

#[test]
fn test_bt_zero_operand() {
    // Test with zero operand
    let code = [
        0x0f, 0xa3, 0xd8, // BT EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 15;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (operand is zero)");
}

#[test]
fn test_bt_max_value() {
    // Test with maximum value
    let code = [
        0x48, 0x0f, 0xa3, 0xd8, // BT RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (all bits are 1)");
}

#[test]
fn test_bt_imm8_upper_bits_ignored() {
    // Immediate is only 8 bits, upper bits are ignored
    let code = [
        0x0f, 0xba, 0xe0,
        0xff, // BT EAX, 0xFF (only lower 5 bits matter for 32-bit: 0x1F = 31)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // bit 31 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 is 1)");
}
