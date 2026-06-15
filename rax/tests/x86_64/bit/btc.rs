use crate::common::*;
use rax::cpu::Registers;

// BTC - Bit Test and Complement
// Tests a bit in a bit string (first operand) and stores the value in CF flag,
// then complements (inverts) the bit in the bit string.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F BB /r       BTC r/m16, r16     - Test and complement bit in r/m16
// 0F BB /r       BTC r/m32, r32     - Test and complement bit in r/m32
// REX.W 0F BB /r BTC r/m64, r64     - Test and complement bit in r/m64
// 0F BA /7 ib    BTC r/m16, imm8    - Test and complement bit in r/m16
// 0F BA /7 ib    BTC r/m32, imm8    - Test and complement bit in r/m32
// REX.W 0F BA /7 ib BTC r/m64, imm8 - Test and complement bit in r/m64

#[test]
fn test_btc_ax_bx_bit_0_set() {
    // BTC AX, BX - test and complement bit 0 (initially set)
    let code = [
        0x66, 0x0f, 0xbb, 0xd8, // BTC AX, BX
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
        "AX: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_ax_bx_bit_0_clear() {
    // BTC AX, BX - test and complement bit 0 (initially clear)
    let code = [
        0x66, 0x0f, 0xbb, 0xd8, // BTC AX, BX
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
        0b0000_0000_0000_0001,
        "AX: bit 0 should be complemented to 1"
    );
}

#[test]
fn test_btc_ax_bx_bit_15() {
    // BTC AX, BX - test and complement MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xbb, 0xd8, // BTC AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // bit 15 set
    regs.rbx = 15; // test bit 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 15 was 1)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0000,
        "AX: bit 15 should be complemented to 0"
    );
}

#[test]
fn test_btc_eax_ebx_bit_0() {
    // BTC EAX, EBX - test and complement bit 0 (32-bit)
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
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
        "EAX: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_eax_ebx_bit_31() {
    // BTC EAX, EBX - test and complement MSB (bit 31)
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
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
        "EAX: bit 31 should be complemented to 0"
    );
}

#[test]
fn test_btc_rax_rbx_bit_0() {
    // BTC RAX, RBX - test and complement bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xbb, 0xd8, // BTC RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax, 0b0000_0000,
        "RAX: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_rax_rbx_bit_63() {
    // BTC RAX, RBX - test and complement MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xbb, 0xd8, // BTC RAX, RBX
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
        "RAX: bit 63 should be complemented to 0"
    );
}

#[test]
fn test_btc_ax_imm8_bit_0() {
    // BTC AX, imm8 - test and complement bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xf8, 0x00, // BTC AX, 0
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
        "AX: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_ax_imm8_bit_15() {
    // BTC AX, imm8 - test and complement bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xf8, 0x0f, // BTC AX, 15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // bit 15 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 15 was 0)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0x8000,
        "AX: bit 15 should be complemented to 1"
    );
}

#[test]
fn test_btc_eax_imm8_bit_0() {
    // BTC EAX, imm8 - test and complement bit 0
    let code = [
        0x0f, 0xba, 0xf8, 0x00, // BTC EAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0001,
        "EAX: bit 0 should be complemented to 1"
    );
}

#[test]
fn test_btc_eax_imm8_bit_31() {
    // BTC EAX, imm8 - test and complement bit 31
    let code = [
        0x0f, 0xba, 0xf8, 0x1f, // BTC EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 31 was 0)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: bit 31 should be complemented to 1"
    );
}

#[test]
fn test_btc_rax_imm8_bit_0() {
    // BTC RAX, imm8 - test and complement bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xf8, 0x00, // BTC RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax, 0b0000_0000,
        "RAX: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_rax_imm8_bit_63() {
    // BTC RAX, imm8 - test and complement bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xf8, 0x3f, // BTC RAX, 63
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000_0000_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 63 was 0)");
    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX: bit 63 should be complemented to 1"
    );
}

#[test]
fn test_btc_double_complement() {
    // Complementing twice should return to original value
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX (again)
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
        "EAX: double complement should return to original"
    );
}

#[test]
fn test_btc_alternating_bits() {
    // Test alternating bit pattern
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010...1010
    regs.rbx = 1; // complement bit 1 (currently 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 1 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA & !0x2,
        "EAX: bit 1 should be complemented"
    );
}

#[test]
fn test_btc_preserves_other_bits() {
    // BTC should only modify the specified bit
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 5; // complement bit 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF & !(1 << 5),
        "EAX: only bit 5 should change"
    );
}

#[test]
fn test_btc_with_extended_registers() {
    // BTC R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xbb, 0xc8, // BTC R8D, R9D
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
        "R8D: bit 0 should be complemented to 0"
    );
}

#[test]
fn test_btc_r15_imm8() {
    // BTC R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xff, 0x20, // BTC R15, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0; // bit 32 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 32 was 0)");
    assert_eq!(
        regs.r15, 0x1_0000_0000,
        "R15: bit 32 should be complemented to 1"
    );
}

#[test]
fn test_btc_mem16_reg() {
    // BTC [mem], BX
    let code = [
        0x66, 0x0f, 0xbb, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTC [DATA_ADDR], BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 8; // complement bit 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x0100); // bit 8 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 8 was 1)");
    let result = read_mem_u16(&mem);
    assert_eq!(result, 0x0000, "Memory: bit 8 should be complemented to 0");
}

#[test]
fn test_btc_mem32_reg() {
    // BTC [mem], EBX
    let code = [
        0x0f, 0xbb, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTC [DATA_ADDR], EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 16; // complement bit 16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00000000); // bit 16 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 16 was 0)");
    let result = read_mem_u32(&mem);
    assert_eq!(
        result, 0x00010000,
        "Memory: bit 16 should be complemented to 1"
    );
}

#[test]
fn test_btc_mem64_reg() {
    // BTC [mem], RBX
    let code = [
        0x48, 0x0f, 0xbb, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTC [DATA_ADDR], RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 40; // complement bit 40
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x100_0000_0000); // bit 40 set
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 40 was 1)");
    let result = read_mem_u64(&mem);
    assert_eq!(
        result, 0x000_0000_0000,
        "Memory: bit 40 should be complemented to 0"
    );
}

#[test]
fn test_btc_mem32_imm8() {
    // BTC [mem], imm8
    let code = [
        0x0f, 0xba, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BTC [DATA_ADDR], 12
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x0000); // bit 12 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 12 was 0)");
    let result = read_mem_u32(&mem);
    assert_eq!(result, 0x1000, "Memory: bit 12 should be complemented to 1");
}

#[test]
fn test_btc_toggle_pattern() {
    // Toggle a bit multiple times
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX (1st)
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX (2nd)
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX (3rd)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 3 toggles: 0 -> 1 -> 0 -> 1
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1 << 10,
        "EAX: bit 10 should be set after 3 toggles"
    );
}

#[test]
fn test_btc_all_bits_set() {
    // Test with all bits set
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 17; // complement bit 17
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
fn test_btc_all_bits_clear() {
    // Test with all bits clear
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 17; // complement bit 17
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 17 was 0)");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1 << 17, "EAX: bit 17 should be set");
}

#[test]
fn test_btc_bit_position_modulo_16() {
    // For 16-bit operands, bit position is taken modulo 16
    let code = [
        0x66, 0x0f, 0xbb, 0xd8, // BTC AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0001; // bit 0 set
    regs.rbx = 16; // position 16 % 16 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0000,
        "AX: bit 0 should be complemented"
    );
}

#[test]
fn test_btc_bit_position_modulo_32() {
    // For 32-bit operands, bit position is taken modulo 32
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 32; // position 32 % 32 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000,
        "EAX: bit 0 should be complemented"
    );
}

#[test]
fn test_btc_bit_position_modulo_64() {
    // For 64-bit operands, bit position is taken modulo 64
    let code = [
        0x48, 0x0f, 0xbb, 0xd8, // BTC RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0001; // bit 0 set
    regs.rbx = 64; // position 64 % 64 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 0 was 1)");
    assert_eq!(regs.rax, 0x0000, "RAX: bit 0 should be complemented");
}

#[test]
fn test_btc_creates_single_bit_set() {
    // Starting from zero, create a single bit set
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1 << 20,
        "EAX: only bit 20 should be set"
    );
}

#[test]
fn test_btc_clears_single_bit() {
    // Starting from all ones, clear a single bit
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF & !(1 << 20),
        "EAX: only bit 20 should be clear"
    );
}

#[test]
fn test_btc_multiple_different_bits() {
    // Complement multiple different bits
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX (bit 5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let mut regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1 << 5, "EAX: bit 5 should be set");

    // Complement bit 10
    let code = [
        0x0f, 0xbb, 0xd8, // BTC EAX, EBX
        0xf4,
    ];
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        (1 << 5) | (1 << 10),
        "EAX: bits 5 and 10 should be set"
    );
}
