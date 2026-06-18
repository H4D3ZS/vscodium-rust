use crate::common::*;
use rax::cpu::Registers;

// BTS - Bit Test and Set
// Tests a bit in a bit string (first operand) and stores the value in CF flag,
// then sets the bit in the bit string.
// The bit string is a sequence of bits in memory or a register.
// The bit position is specified by the second operand (immediate or register).
// Only CF flag is affected; other flags are undefined.
//
// Opcodes:
// 0F AB /r       BTS r/m16, r16     - Test and set bit in r/m16
// 0F AB /r       BTS r/m32, r32     - Test and set bit in r/m32
// REX.W 0F AB /r BTS r/m64, r64     - Test and set bit in r/m64
// 0F BA /5 ib    BTS r/m16, imm8    - Test and set bit in r/m16
// 0F BA /5 ib    BTS r/m32, imm8    - Test and set bit in r/m32
// REX.W 0F BA /5 ib BTS r/m64, imm8 - Test and set bit in r/m64

#[test]
fn test_bts_ax_bx_bit_0_set() {
    // BTS AX, BX - test and set bit 0 (initially set)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
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
        0b0000_0000_0000_0001,
        "AX: bit 0 should remain set"
    );
}

#[test]
fn test_bts_ax_bx_bit_0_clear() {
    // BTS AX, BX - test and set bit 0 (initially clear)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
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
        "AX: bit 0 should be set to 1"
    );
}

#[test]
fn test_bts_ax_bx_bit_15() {
    // BTS AX, BX - test and set MSB (bit 15)
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // bit 15 clear
    regs.rbx = 15; // test bit 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 15 was 0)");
    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: bit 15 should be set to 1");
}

#[test]
fn test_bts_eax_ebx_bit_0() {
    // BTS EAX, EBX - test and set bit 0 (32-bit)
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0b0000_0001,
        "EAX: bit 0 should be set to 1"
    );
}

#[test]
fn test_bts_eax_ebx_bit_31() {
    // BTS EAX, EBX - test and set MSB (bit 31)
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000; // bit 31 clear
    regs.rbx = 31; // test bit 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 31 was 0)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: bit 31 should be set to 1"
    );
}

#[test]
fn test_bts_rax_rbx_bit_0() {
    // BTS RAX, RBX - test and set bit 0 (64-bit)
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(regs.rax, 0b0000_0001, "RAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_rax_rbx_bit_63() {
    // BTS RAX, RBX - test and set MSB (bit 63)
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000_0000_0000_0000; // bit 63 clear
    regs.rbx = 63; // test bit 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 63 was 0)");
    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX: bit 63 should be set to 1"
    );
}

#[test]
fn test_bts_ax_imm8_bit_0() {
    // BTS AX, imm8 - test and set bit 0
    let code = [
        0x66, 0x0f, 0xba, 0xe8, 0x00, // BTS AX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.rax & 0xFFFF,
        0b0000_0000_0000_0001,
        "AX: bit 0 should be set to 1"
    );
}

#[test]
fn test_bts_ax_imm8_bit_15() {
    // BTS AX, imm8 - test and set bit 15
    let code = [
        0x66, 0x0f, 0xba, 0xe8, 0x0f, // BTS AX, 15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // bit 15 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 15 was 0)");
    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: bit 15 should be set to 1");
}

#[test]
fn test_bts_eax_imm8_bit_0() {
    // BTS EAX, imm8 - test and set bit 0
    let code = [
        0x0f, 0xba, 0xe8, 0x00, // BTS EAX, 0
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
        "EAX: bit 0 should be set to 1"
    );
}

#[test]
fn test_bts_eax_imm8_bit_31() {
    // BTS EAX, imm8 - test and set bit 31
    let code = [
        0x0f, 0xba, 0xe8, 0x1f, // BTS EAX, 31
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
        "EAX: bit 31 should be set to 1"
    );
}

#[test]
fn test_bts_rax_imm8_bit_0() {
    // BTS RAX, imm8 - test and set bit 0
    let code = [
        0x48, 0x0f, 0xba, 0xe8, 0x00, // BTS RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(regs.rax, 0b0000_0001, "RAX: bit 0 should be set to 1");
}

#[test]
fn test_bts_rax_imm8_bit_63() {
    // BTS RAX, imm8 - test and set bit 63
    let code = [
        0x48, 0x0f, 0xba, 0xe8, 0x3f, // BTS RAX, 63
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000_0000_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 63 was 0)");
    assert_eq!(
        regs.rax, 0x8000_0000_0000_0000,
        "RAX: bit 63 should be set to 1"
    );
}

#[test]
fn test_bts_idempotent() {
    // Setting twice should have same effect as once
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0x0f, 0xab, 0xd8, // BTS EAX, EBX (again)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = 0x12345678 | (1 << 5);
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected,
        "EAX: double set should have same result"
    );
}

#[test]
fn test_bts_alternating_bits() {
    // Test alternating bit pattern
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010...1010
    regs.rbx = 0; // set bit 0 (currently 0)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA | 0x1,
        "EAX: bit 0 should be set"
    );
}

#[test]
fn test_bts_preserves_other_bits() {
    // BTS should only modify the specified bit
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 5; // set bit 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1 << 5,
        "EAX: only bit 5 should be set"
    );
}

#[test]
fn test_bts_with_extended_registers() {
    // BTS R8D, R9D - test with extended registers
    let code = [
        0x45, 0x0f, 0xab, 0xc8, // BTS R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0b0000_0000;
    regs.r9 = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0b0000_0001,
        "R8D: bit 0 should be set to 1"
    );
}

#[test]
fn test_bts_r15_imm8() {
    // BTS R15, imm8
    let code = [
        0x49, 0x0f, 0xba, 0xef, 0x20, // BTS R15, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0; // bit 32 clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 32 was 0)");
    assert_eq!(regs.r15, 0x1_0000_0000, "R15: bit 32 should be set to 1");
}

#[test]
fn test_bts_mem16_reg() {
    // BTS [mem], BX
    let code = [
        0x66, 0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 8; // set bit 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x0000); // bit 8 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 8 was 0)");
    let result = read_mem_u16(&mem);
    assert_eq!(result, 0x0100, "Memory: bit 8 should be set to 1");
}

#[test]
fn test_bts_mem32_reg() {
    // BTS [mem], EBX
    let code = [
        0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 16; // set bit 16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00000000); // bit 16 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 16 was 0)");
    let result = read_mem_u32(&mem);
    assert_eq!(result, 0x00010000, "Memory: bit 16 should be set to 1");
}

#[test]
fn test_bts_mem64_reg() {
    // BTS [mem], RBX
    let code = [
        0x48, 0x0f, 0xab, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BTS [DATA_ADDR], RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 40; // set bit 40
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x000_0000_0000); // bit 40 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 40 was 0)");
    let result = read_mem_u64(&mem);
    assert_eq!(result, 0x100_0000_0000, "Memory: bit 40 should be set to 1");
}

#[test]
fn test_bts_mem32_imm8() {
    // BTS [mem], imm8
    let code = [
        0x0f, 0xba, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0c, // BTS [DATA_ADDR], 12
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x0000); // bit 12 clear
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 12 was 0)");
    let result = read_mem_u32(&mem);
    assert_eq!(result, 0x1000, "Memory: bit 12 should be set to 1");
}

#[test]
fn test_bts_all_bits_set() {
    // Test with all bits set
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 17; // set bit 17 (already set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set (bit 17 was 1)");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: should remain all ones"
    );
}

#[test]
fn test_bts_all_bits_clear() {
    // Test with all bits clear
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 17; // set bit 17
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 17 was 0)");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1 << 17, "EAX: bit 17 should be set");
}

#[test]
fn test_bts_bit_position_modulo_16() {
    // For 16-bit operands, bit position is taken modulo 16
    let code = [
        0x66, 0x0f, 0xab, 0xd8, // BTS AX, BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000_0000_0000;
    regs.rbx = 16; // position 16 % 16 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX: bit 0 should be set");
}

#[test]
fn test_bts_bit_position_modulo_32() {
    // For 32-bit operands, bit position is taken modulo 32
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    regs.rbx = 32; // position 32 % 32 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0001, "EAX: bit 0 should be set");
}

#[test]
fn test_bts_bit_position_modulo_64() {
    // For 64-bit operands, bit position is taken modulo 64
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0b0000_0000;
    regs.rbx = 64; // position 64 % 64 = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!cf_set(regs.rflags), "CF should be clear (bit 0 was 0)");
    assert_eq!(regs.rax, 0x0001, "RAX: bit 0 should be set");
}

#[test]
fn test_bts_creates_single_bit() {
    // Starting from zero, create a single bit set
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
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
fn test_bts_multiple_bits_sequential() {
    // Set multiple bits sequentially
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX (bit 5)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let mut regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1 << 5, "EAX: bit 5 should be set");

    // Set bit 10
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
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

#[test]
fn test_bts_no_effect_on_set_bit() {
    // BTS on already set bit should not change operand
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345679; // bit 0 is set
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345679,
        "EAX: should be unchanged"
    );
}

#[test]
fn test_bts_creates_mask() {
    // BTS can be used to create bit masks
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        1 << 8,
        "EAX: creates mask with bit 8 set"
    );
}

#[test]
fn test_bts_sparse_bits() {
    // Test with sparse bit pattern
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // only bit 31 set
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000001,
        "EAX: bits 0 and 31 should be set"
    );
}

#[test]
fn test_bts_high_bit_64() {
    // Test setting high bits in 64-bit register
    let code = [
        0x48, 0x0f, 0xab, 0xd8, // BTS RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000_0000_0000_0000;
    regs.rbx = 59;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1u64 << 59, "RAX: bit 59 should be set");
}

#[test]
fn test_bts_build_bitmask() {
    // Build a bitmask by setting multiple bits
    let code = [
        0x0f, 0xab, 0xd8, // BTS EAX, EBX
        0xf4,
    ];

    let mut result = 0u32;
    for bit_pos in [0, 4, 8, 12, 16, 20, 24, 28] {
        let mut regs = Registers::default();
        regs.rax = result as u64;
        regs.rbx = bit_pos;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();
        result = (regs.rax & 0xFFFFFFFF) as u32;
    }

    assert_eq!(result, 0x11111111, "EAX: should have pattern 0x11111111");
}
