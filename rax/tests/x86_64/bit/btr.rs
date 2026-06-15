use crate::common::*;
use rax::cpu::Registers;

// BTR - Bit Test and Reset
// Tests a bit in a bit string (first operand) and stores the value in CF flag,
// then resets (clears) the bit in the bit string.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F B3 /r       BTR r/m16, r16     - Test and reset bit in r/m16
// 0F B3 /r       BTR r/m32, r32     - Test and reset bit in r/m32
// REX.W 0F B3 /r BTR r/m64, r64     - Test and reset bit in r/m64
// 0F BA /6 ib    BTR r/m16, imm8    - Test and reset bit in r/m16
// 0F BA /6 ib    BTR r/m32, imm8    - Test and reset bit in r/m32
// REX.W 0F BA /6 ib BTR r/m64, imm8 - Test and reset bit in r/m64

#[test]
fn test_btr_ax_bx_bit_0_set() {
    // BTR AX, BX - test and reset bit 0 (initially set)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // bit 0 set
    regs.rbx = 0; // test bit 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0b0000_0000_0000_0000,
        "AX: bit 0 should be reset to 0"
    );
}

#[test]
fn test_btr_ax_bx_bit_0_clear() {
    // BTR AX, BX - test and reset bit 0 (initially clear)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0000; // bit 0 clear
    regs.rbx = 0; // test bit 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0b0000_0000_0000_0000,
        "AX: bit 0 should remain 0"
    );
}

#[test]
fn test_btr_ax_bx_bit_15() {
    // BTR AX, BX - test and reset MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // bit 15 set
    regs.rbx = 15; // test bit 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 15 was 1)");
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX: bit 15 should be reset to 0");
}

#[test]
fn test_btr_eax_ebx_bit_0() {
    // BTR EAX, EBX - test and reset bit 0 (32-bit)
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0000,
        "EAX: bit 0 should be reset to 0"
    );
}

#[test]
fn test_btr_eax_ebx_bit_31() {
    // BTR EAX, EBX - test and reset MSB (bit 31)
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // bit 31 set
    regs.rbx = 31; // test bit 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: bit 31 should be reset to 0"
    );
}

#[test]
fn test_btr_rax_rbx_bit_0() {
    // BTR RAX, RBX - test and reset bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax, 0b0000_0000, "RAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_rax_rbx_bit_63() {
    // BTR RAX, RBX - test and reset MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0000; // bit 63 set
    regs.rbx = 63; // test bit 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 63 was 1)");
    assert_eq!(
        regs.rax, 0x0000_0000_0000_0000,
        "RAX: bit 63 should be reset to 0"
    );
}

#[test]
fn test_btr_ax_imm8_bit_0() {
    // BTR AX, imm8 - test and reset bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xf0, 0x00, // BTR AX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0b0000_0000_0000_0000,
        "AX: bit 0 should be reset to 0"
    );
}

#[test]
fn test_btr_ax_imm8_bit_15() {
    // BTR AX, imm8 - test and reset bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xf0, 0x0f, // BTR AX, 15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // bit 15 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 15 was 1)");
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX: bit 15 should be reset to 0");
}

#[test]
fn test_btr_eax_imm8_bit_0() {
    // BTR EAX, imm8 - test and reset bit 0
    let code = [
        0x0f, 0xba, 0xf0, 0x00, // BTR EAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0000,
        "EAX: bit 0 should be reset to 0"
    );
}

#[test]
fn test_btr_eax_imm8_bit_31() {
    // BTR EAX, imm8 - test and reset bit 31
    let code = [
        0x0f, 0xba, 0xf0, 0x1f, // BTR EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 31 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: bit 31 should be reset to 0"
    );
}

#[test]
fn test_btr_rax_imm8_bit_0() {
    // BTR RAX, imm8 - test and reset bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xf0, 0x00, // BTR RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax, 0b0000_0000, "RAX: bit 0 should be reset to 0");
}

#[test]
fn test_btr_rax_imm8_bit_63() {
    // BTR RAX, imm8 - test and reset bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xf0, 0x3f, // BTR RAX, 63
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 63 was 1)");
    assert_eq!(
        regs.rax, 0x0000_0000_0000_0000,
        "RAX: bit 63 should be reset to 0"
    );
}

#[test]
fn test_btr_idempotent() {
    // Resetting twice should have same effect as once
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX (again)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = 0x12345678 & !(1 << 5);
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected,
        "EAX: double reset should have same result"
    );
}

#[test]
fn test_btr_alternating_bits() {
    // Test alternating bit pattern
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010...1010
    regs.rbx = 1; // reset bit 1 (currently 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 1 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA & !0x2,
        "EAX: bit 1 should be reset"
    );
}

#[test]
fn test_btr_preserves_other_bits() {
    // BTR should only modify the specified bit
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 5; // reset bit 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF & !(1 << 5),
        "EAX: only bit 5 should change"
    );
}

#[test]
fn test_btr_with_extended_registers() {
    // BTR R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xb3, 0xc8, // BTR R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0b0000_0001;
    regs.r9 = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0000_0000,
        "R8D: bit 0 should be reset to 0"
    );
}

#[test]
fn test_btr_r15_imm8() {
    // BTR R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xf7, 0x20, // BTR R15, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x1_0000_0000; // bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 32 was 1)");
    assert_eq!(regs.r15, 0x0, "R15: bit 32 should be reset to 0");
}

#[test]
fn test_btr_mem16_reg() {
    // BTR [mem], BX
    let code = [
        0x66, 0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 8; // reset bit 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x0100); // bit 8 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 8 was 1)");
    let result = read_mem_u16(&mem);
    assert_eq!(result, 0x0000, "Memory: bit 8 should be reset to 0");
}

#[test]
fn test_btr_mem32_reg() {
    // BTR [mem], EBX
    let code = [
        0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 16; // reset bit 16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00010000); // bit 16 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 16 was 1)");
    let result = read_mem_u32(&mem);
    assert_eq!(result, 0x00000000, "Memory: bit 16 should be reset to 0");
}

#[test]
fn test_btr_mem64_reg() {
    // BTR [mem], RBX
    let code = [
        0x48, 0x0f, 0xb3, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTR [DATA_ADDR], RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 40; // reset bit 40
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 40 was 1)");
    let result = read_mem_u64(&mem);
    assert_eq!(
        result, 0x000_0000_0000,
        "Memory: bit 40 should be reset to 0"
    );
}

#[test]
fn test_btr_mem32_imm8() {
    // BTR [mem], imm8
    let code = [
        0x0f, 0xba, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BTR [DATA_ADDR], 12
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x1000); // bit 12 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 12 was 1)");
    let result = read_mem_u32(&mem);
    assert_eq!(result, 0x0000, "Memory: bit 12 should be reset to 0");
}

#[test]
fn test_btr_all_bits_set() {
    // Test with all bits set
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 17; // reset bit 17
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 17 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF & !(1 << 17),
        "EAX: bit 17 should be clear"
    );
}

#[test]
fn test_btr_all_bits_clear() {
    // Test with all bits clear
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 17; // reset bit 17
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 17 was 0)");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX: should remain zero");
}

#[test]
fn test_btr_bit_position_modulo_16() {
    // For 16-bit operands, bit position is taken modulo 16
    let code = [
        0x66, 0x0f, 0xb3, 0xd8, // BTR AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // bit 0 set
    regs.rbx = 16; // position 16 % 16 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX: bit 0 should be reset");
}

#[test]
fn test_btr_bit_position_modulo_32() {
    // For 32-bit operands, bit position is taken modulo 32
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 32; // position 32 % 32 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0000, "EAX: bit 0 should be reset");
}

#[test]
fn test_btr_bit_position_modulo_64() {
    // For 64-bit operands, bit position is taken modulo 64
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 64; // position 64 % 64 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax, 0x0000, "RAX: bit 0 should be reset");
}

#[test]
fn test_btr_clears_to_zero() {
    // Starting from single bit set, clear it to zero
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1 << 20;
    regs.rbx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX: should be zero after resetting only bit"
    );
}

#[test]
fn test_btr_multiple_bits_sequential() {
    // Reset multiple bits sequentially
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX (bit 5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (1 << 5) | (1 << 10) | (1 << 15);
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let mut regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        (1 << 10) | (1 << 15),
        "EAX: bit 5 should be clear"
    );

    // Reset bit 10
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1 << 15,
        "EAX: bits 5 and 10 should be clear"
    );
}

#[test]
fn test_btr_no_effect_on_clear_bit() {
    // BTR on already clear bit should not change operand
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0; // bit 0 is clear in 0x12345678
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX: should be unchanged"
    );
}

#[test]
fn test_btr_creates_mask() {
    // BTR can be used to create bit masks
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF & !(1 << 8),
        "EAX: creates mask with bit 8 clear"
    );
}

#[test]
fn test_btr_sparse_bits() {
    // Test with sparse bit pattern
    let code = [
        0x0f, 0xb3, 0xd8, // BTR EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001; // bits 0 and 31 set
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: bit 0 should be clear, bit 31 remains"
    );
}

#[test]
fn test_btr_high_bit_64() {
    // Test resetting high bits in 64-bit register
    let code = [
        0x48, 0x0f, 0xb3, 0xd8, // BTR RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 59;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax,
        0xFFFF_FFFF_FFFF_FFFF & !(1u64 << 59),
        "RAX: bit 59 should be clear"
    );
}
