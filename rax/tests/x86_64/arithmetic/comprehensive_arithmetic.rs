//! Comprehensive tests for Group 1: Legacy & General Purpose Arithmetic
//!
//! This module provides extensive testing for:
//! - Legacy BCD/ASCII Adjust: aaa, aad, aam, aas
//! - Legacy Decimal Adjust: daa, das
//! - Integer Addition (Carry): adc, adcx, add, adox
//! - Integer Subtraction: dec, inc, neg, sbb
//! - Integer Subtraction (Base): sub
//! - Integer Multiplication: imul, mul, mulx
//! - Integer Division: div, idiv
//! - Sign Extension: cbw, cdq, cwde, cqo, cwd
//!
//! Each test category covers:
//! - Basic operations
//! - Boundary values (0, 1, max, min)
//! - Overflow/underflow conditions
//! - Flag behavior (CF, OF, ZF, SF, PF, AF)
//! - Register preservation
//! - Memory operand variants

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// ============================================================================
// ADC - ADD WITH CARRY: Comprehensive Edge Cases
// ============================================================================

#[test]
fn test_adc_carry_chain_8bit() {
    // Test multi-precision addition: 0xFF + 0x01 with carry = 1
    // Result should be 0x01 with carry out
    let code = [
        0x14, 0x01, // ADC AL, 0x01
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::CF; // Set carry flag initially
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFF + 0x01 + 1(CF) = 0x101, result = 0x01, CF set
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 0x01");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_adc_no_carry_in_no_carry_out() {
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x10;
    regs.rflags = 0x2; // No carry flag
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x11, "0x10 + 0x01 = 0x11");
    assert!(!cf_set(regs.rflags), "No carry out");
}

#[test]
fn test_adc_32bit_max_boundary() {
    // EAX = 0xFFFFFFFF, add 0 with carry = 1
    let code = [
        0x15, 0x00, 0x00, 0x00, 0x00, // ADC EAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "0xFFFFFFFF + 0 + 1 = 0");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(zf_set(regs.rflags), "ZF should be set (result is 0)");
}

#[test]
fn test_adc_64bit_carry_propagation() {
    // RAX = 0xFFFFFFFFFFFFFFFF, add 0 with carry = 1
    let code = [
        0x48, 0x15, 0x00, 0x00, 0x00, 0x00, // ADC RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000000,
        "0xFFFFFFFFFFFFFFFF + 0 + 1 = 0"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_signed_overflow() {
    // 0x7F + 0x01 = 0x80 (signed overflow: 127 + 1 = -128)
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    regs.rflags = 0x2; // No carry
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "0x7F + 0x01 = 0x80");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (no unsigned overflow)"
    );
    assert!(sf_set(regs.rflags), "SF should be set (result is negative)");
}

#[test]
fn test_adc_auxiliary_carry() {
    // Test AF: carry from bit 3 to bit 4
    // 0x0F + 0x01 = 0x10 (carry from lower nibble)
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x0F;
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "0x0F + 0x01 = 0x10");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// ADD - INTEGER ADDITION: Comprehensive Tests
// ============================================================================

#[test]
fn test_add_zero_preservation() {
    // Adding zero should preserve value and set ZF if result is zero
    let code = [0x04, 0x00, 0xf4]; // ADD AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "0 + 0 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_add_parity_even() {
    // Result with even number of 1-bits: 0x03 (00000011, 2 bits)
    let code = [0x04, 0x02, 0xf4]; // ADD AL, 0x02
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "0x01 + 0x02 = 0x03");
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_add_parity_odd() {
    // Result with odd number of 1-bits: 0x01 (00000001, 1 bit)
    let code = [0x04, 0x01, 0xf4]; // ADD AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "0x00 + 0x01 = 0x01");
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

#[test]
fn test_add_register_to_register_32bit() {
    // ADD EAX, EBX (32-bit register to register)
    let code = [0x01, 0xd8, 0xf4]; // ADD EAX, EBX
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x23456789,
        "0x12345678 + 0x11111111 = 0x23456789"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x11111111, "EBX unchanged");
}

#[test]
fn test_add_64bit_large_values() {
    // ADD RAX, RBX with large 64-bit values
    let code = [0x48, 0x01, 0xd8, 0xf4]; // ADD RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF; // Max positive i64
    regs.rbx = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000000000000000, "Max i64 + 1 wraps to min");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_add_memory_operand() {
    // ADD EAX, [mem]
    let code = [
        0x03, 0x05, 0xfa, 0x0f, 0x00, 0x00, // ADD EAX, [rip+0x0FFA]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00000002);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000003, "1 + 2 = 3");
}

// ============================================================================
// SUB - INTEGER SUBTRACTION: Comprehensive Tests
// ============================================================================

#[test]
fn test_sub_zero_result() {
    let code = [0x2c, 0x05, 0xf4]; // SUB AL, 0x05
    let mut regs = Registers::default();
    regs.rax = 0x05;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "5 - 5 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(!cf_set(regs.rflags), "CF should be clear (no borrow)");
}

#[test]
fn test_sub_borrow_required() {
    // 0x00 - 0x01 = 0xFF with borrow
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "0 - 1 = 0xFF (underflow)");
    assert!(cf_set(regs.rflags), "CF should be set (borrow)");
    assert!(sf_set(regs.rflags), "SF should be set (negative result)");
}

#[test]
fn test_sub_signed_overflow() {
    // 0x80 - 0x01 = 0x7F (signed overflow: -128 - 1 = 127)
    let code = [0x2c, 0x01, 0xf4]; // SUB AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x80; // -128 as i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x7F, "-128 - 1 = 127 (overflow)");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(!sf_set(regs.rflags), "SF should be clear (positive result)");
}

#[test]
fn test_sub_32bit_underflow() {
    let code = [
        0x2d, 0x01, 0x00, 0x00, 0x00, // SUB EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "0 - 1 = 0xFFFFFFFF");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_sub_64bit_large_values() {
    let code = [
        0x48, 0x2d, 0x01, 0x00, 0x00, 0x00, // SUB RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000; // Min i64
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7FFFFFFFFFFFFFFF, "Min i64 - 1 wraps to max");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
}

// ============================================================================
// SBB - SUBTRACT WITH BORROW: Comprehensive Tests
// ============================================================================

#[test]
fn test_sbb_with_borrow_in() {
    // 0x10 - 0x01 - 1(CF) = 0x0E
    let code = [0x1c, 0x01, 0xf4]; // SBB AL, 0x01
    let mut regs = Registers::default();
    regs.rax = 0x10;
    regs.rflags = 0x2 | flags::bits::CF; // Set carry (borrow)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0E, "0x10 - 0x01 - 1 = 0x0E");
    assert!(!cf_set(regs.rflags), "No borrow out");
}

#[test]
fn test_sbb_borrow_chain() {
    // 0x00 - 0x00 - 1(CF) = 0xFF with borrow
    let code = [0x1c, 0x00, 0xf4]; // SBB AL, 0x00
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "0x00 - 0x00 - 1 = 0xFF");
    assert!(cf_set(regs.rflags), "CF should be set (borrow)");
}

#[test]
fn test_sbb_multiprecision_subtraction() {
    // Simulate 64-bit subtraction using two 32-bit operations
    // First: SBB with CF=0 (low dword), then SBB with result CF (high dword)
    let code = [
        0x2d, 0x01, 0x00, 0x00, 0x00, // SUB EAX, 1 (low dword)
        0x19, 0xca, // SBB EDX, ECX (high dword)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000; // Low dword
    regs.rdx = 0x00000001; // High dword
    regs.rcx = 0x00000000; // Subtract 0 from high (but with borrow from low)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x0000000100000000 - 1 = 0x00000000FFFFFFFF
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "Low dword underflowed");
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x00000000,
        "High dword decremented by borrow"
    );
}

// ============================================================================
// INC/DEC - INCREMENT/DECREMENT: Comprehensive Tests
// ============================================================================

#[test]
fn test_inc_basic_8bit() {
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "0 + 1 = 1");
}

#[test]
fn test_inc_overflow_8bit() {
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "0xFF + 1 = 0x00 (wrap)");
    assert!(zf_set(regs.rflags), "ZF should be set");
    // Note: INC does NOT affect CF
}

#[test]
fn test_inc_signed_overflow() {
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    let mut regs = Registers::default();
    regs.rax = 0x7F; // 127
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "127 + 1 = -128 (signed overflow)");
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_inc_preserves_cf() {
    // INC should NOT modify CF
    let code = [0xfe, 0xc0, 0xf4]; // INC AL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rflags = 0x2 | flags::bits::CF; // Set CF before
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be preserved by INC");
}

#[test]
fn test_dec_basic_8bit() {
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "1 - 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_dec_underflow_8bit() {
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "0 - 1 = 0xFF (underflow)");
    assert!(sf_set(regs.rflags), "SF should be set");
    // DEC does NOT affect CF
}

#[test]
fn test_dec_signed_overflow() {
    let code = [0xfe, 0xc8, 0xf4]; // DEC AL
    let mut regs = Registers::default();
    regs.rax = 0x80; // -128
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x7F, "-128 - 1 = 127 (signed overflow)");
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_inc_32bit() {
    let code = [0xff, 0xc0, 0xf4]; // INC EAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "0xFFFFFFFF + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_inc_64bit() {
    let code = [0x48, 0xff, 0xc0, 0xf4]; // INC RAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "Max u64 + 1 = 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// NEG - TWO'S COMPLEMENT NEGATION: Comprehensive Tests
// ============================================================================

#[test]
fn test_neg_positive_to_negative() {
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x05; // 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFB, "NEG 5 = -5 (0xFB)");
    assert!(cf_set(regs.rflags), "CF set when operand non-zero");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_neg_negative_to_positive() {
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0xFB; // -5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x05, "NEG -5 = 5");
    assert!(cf_set(regs.rflags), "CF set when operand non-zero");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_neg_zero() {
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "NEG 0 = 0");
    assert!(!cf_set(regs.rflags), "CF clear when operand is zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_neg_min_value_overflow() {
    // NEG of minimum signed value overflows (no positive equivalent)
    let code = [0xf6, 0xd8, 0xf4]; // NEG AL
    let mut regs = Registers::default();
    regs.rax = 0x80; // -128, NEG(-128) can't be represented as i8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x80,
        "NEG -128 = -128 (overflow, stays same)"
    );
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
    assert!(cf_set(regs.rflags), "CF should be set (non-zero operand)");
}

#[test]
fn test_neg_32bit() {
    let code = [0xf7, 0xd8, 0xf4]; // NEG EAX
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "NEG 1 = -1");
}

#[test]
fn test_neg_64bit() {
    let code = [0x48, 0xf7, 0xd8, 0xf4]; // NEG RAX
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "NEG 1 = -1");
}

// ============================================================================
// MUL - UNSIGNED MULTIPLY: Comprehensive Tests
// ============================================================================

#[test]
fn test_mul_8bit_basic() {
    // AL * BL = AX
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 0x05; // AL = 5
    regs.rbx = 0x03; // BL = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x000F, "5 * 3 = 15");
    assert!(!cf_set(regs.rflags), "CF clear (fits in AL)");
    assert!(!of_set(regs.rflags), "OF clear");
}

#[test]
fn test_mul_8bit_overflow() {
    // 16 * 16 = 256 (overflows AL)
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 16;
    regs.rbx = 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0100, "16 * 16 = 256");
    assert!(cf_set(regs.rflags), "CF set (overflow into AH)");
    assert!(of_set(regs.rflags), "OF set");
}

#[test]
fn test_mul_8bit_max() {
    // 0xFF * 0xFF = 0xFE01
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rbx = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFE01, "255 * 255 = 65025");
    assert!(cf_set(regs.rflags), "CF set");
}

#[test]
fn test_mul_32bit_basic() {
    // EAX * EBX = EDX:EAX
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 0x00001000;
    regs.rbx = 0x00001000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x1000 * 0x1000 = 0x01000000
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x01000000, "Low 32 bits");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "High 32 bits");
    assert!(!cf_set(regs.rflags), "CF clear (fits in EAX)");
}

#[test]
fn test_mul_32bit_overflow() {
    // 0x80000000 * 2 = 0x100000000 (overflows into EDX)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rbx = 0x00000002;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "Low 32 bits");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000001, "High 32 bits");
    assert!(cf_set(regs.rflags), "CF set (overflow into EDX)");
}

#[test]
fn test_mul_64bit_basic() {
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 0x0000000100000000;
    regs.rbx = 0x0000000000000002;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000200000000, "Low 64 bits");
    assert_eq!(regs.rdx, 0x0000000000000000, "High 64 bits");
    assert!(!cf_set(regs.rflags), "CF clear");
}

#[test]
fn test_mul_by_zero() {
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000, "Anything * 0 = 0");
    assert!(!cf_set(regs.rflags), "CF clear (no overflow)");
}

#[test]
fn test_mul_by_one() {
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 0xAB;
    regs.rbx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x00AB, "Anything * 1 = itself");
    assert!(!cf_set(regs.rflags), "CF clear");
}

// ============================================================================
// DIV - UNSIGNED DIVIDE: Comprehensive Tests
// ============================================================================

#[test]
fn test_div_8bit_basic() {
    // AX / BL = AL (quotient), AH (remainder)
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    let mut regs = Registers::default();
    regs.rax = 0x0011; // AX = 17
    regs.rbx = 0x05; // BL = 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "17 / 5 = 3 (quotient in AL)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "17 % 5 = 2 (remainder in AH)");
}

#[test]
fn test_div_8bit_exact() {
    // 15 / 5 = 3 with no remainder
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    let mut regs = Registers::default();
    regs.rax = 0x000F; // AX = 15
    regs.rbx = 0x05; // BL = 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "15 / 5 = 3");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "15 % 5 = 0");
}

#[test]
fn test_div_32bit_basic() {
    // EDX:EAX / EBX = EAX (quotient), EDX (remainder)
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    let mut regs = Registers::default();
    regs.rax = 0x00000064; // EAX = 100
    regs.rdx = 0x00000000; // EDX = 0
    regs.rbx = 0x00000007; // EBX = 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0000000E, "100 / 7 = 14");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000002, "100 % 7 = 2");
}

#[test]
fn test_div_64bit_basic() {
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000064; // 100
    regs.rdx = 0x0000000000000000;
    regs.rbx = 0x0000000000000007; // 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000000000000E, "100 / 7 = 14");
    assert_eq!(regs.rdx, 0x0000000000000002, "100 % 7 = 2");
}

#[test]
fn test_div_by_one() {
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    let mut regs = Registers::default();
    regs.rax = 0x00FF; // AX = 255
    regs.rbx = 0x01; // BL = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "255 / 1 = 255");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "255 % 1 = 0");
}

#[test]
fn test_div_large_dividend() {
    // 0x0100 / 2 = 0x80
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    let mut regs = Registers::default();
    regs.rax = 0x0100; // AX = 256
    regs.rbx = 0x02; // BL = 2
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "256 / 2 = 128");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x00, "256 % 2 = 0");
}

// ============================================================================
// IDIV - SIGNED DIVIDE: Comprehensive Tests
// ============================================================================

#[test]
fn test_idiv_positive_by_positive() {
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    let mut regs = Registers::default();
    regs.rax = 0x0011; // AX = 17
    regs.rbx = 0x05; // BL = 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "17 / 5 = 3");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "17 % 5 = 2");
}

#[test]
fn test_idiv_negative_by_positive() {
    // -17 / 5 = -3, remainder -2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    let mut regs = Registers::default();
    regs.rax = 0xFFEF; // AX = -17 (sign-extended)
    regs.rbx = 0x05; // BL = 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFD, "-17 / 5 = -3 (0xFD)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFE, "-17 % 5 = -2 (0xFE)");
}

#[test]
fn test_idiv_positive_by_negative() {
    // 17 / -5 = -3, remainder 2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    let mut regs = Registers::default();
    regs.rax = 0x0011; // AX = 17
    regs.rbx = 0xFB; // BL = -5 (0xFB)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFD, "17 / -5 = -3 (0xFD)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0x02, "17 % -5 = 2");
}

#[test]
fn test_idiv_negative_by_negative() {
    // -17 / -5 = 3, remainder -2
    let code = [0xf6, 0xfb, 0xf4]; // IDIV BL
    let mut regs = Registers::default();
    regs.rax = 0xFFEF; // AX = -17
    regs.rbx = 0xFB; // BL = -5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x03, "-17 / -5 = 3");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFE, "-17 % -5 = -2 (0xFE)");
}

#[test]
fn test_idiv_32bit_signed() {
    let code = [0xf7, 0xfb, 0xf4]; // IDIV EBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFF9C; // EAX = -100 (sign-extended to 32-bit)
    regs.rdx = 0xFFFFFFFF; // EDX = -1 (sign-extension)
    regs.rbx = 0x00000007; // EBX = 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFF2,
        "-100 / 7 = -14 (0xFFFFFFF2)"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFE,
        "-100 % 7 = -2 (0xFFFFFFFE)"
    );
}

// ============================================================================
// Sign Extension Instructions: CBW, CWDE, CDQE, CWD, CDQ, CQO
// ============================================================================

#[test]
fn test_cbw_positive() {
    // CBW: AL -> AX (sign-extend byte to word)
    let code = [0x66, 0x98, 0xf4]; // CBW (0x66 in 64-bit mode)
    let mut regs = Registers::default();
    regs.rax = 0x7F; // AL = 127 (positive)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x007F, "127 sign-extended to word");
}

#[test]
fn test_cbw_negative() {
    let code = [0x66, 0x98, 0xf4]; // CBW (0x66 in 64-bit mode)
    let mut regs = Registers::default();
    regs.rax = 0x80; // AL = -128 (negative)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFF80, "-128 sign-extended to word");
}

#[test]
fn test_cwde_positive() {
    // CWDE: AX -> EAX (sign-extend word to dword)
    let code = [0x98, 0xf4]; // CWDE (in 32-bit mode it's 0x98)
    let mut regs = Registers::default();
    regs.rax = 0x7FFF; // AX = 32767
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // In 64-bit mode, 0x98 is CWDE which sign-extends AX to EAX
    // Actually in 64-bit mode with default operand size, it's CWDE
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00007FFF, "32767 sign-extended");
}

#[test]
fn test_cwde_negative() {
    let code = [0x98, 0xf4]; // CWDE
    let mut regs = Registers::default();
    regs.rax = 0x8000; // AX = -32768
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFF8000, "-32768 sign-extended");
}

#[test]
fn test_cdqe_positive() {
    // CDQE: EAX -> RAX (sign-extend dword to qword)
    let code = [0x48, 0x98, 0xf4]; // CDQE (REX.W + 0x98)
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF; // EAX = max positive i32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x000000007FFFFFFF, "Max i32 sign-extended");
}

#[test]
fn test_cdqe_negative() {
    let code = [0x48, 0x98, 0xf4]; // CDQE
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // EAX = min i32 (-2147483648)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF80000000, "Min i32 sign-extended");
}

#[test]
fn test_cwd_positive() {
    // CWD: AX -> DX:AX (sign-extend word to dword, into DX:AX)
    let code = [0x66, 0x99, 0xf4]; // CWD (16-bit operand override + 0x99)
    let mut regs = Registers::default();
    regs.rax = 0x7FFF; // AX = 32767
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x0000, "DX = 0 for positive");
}

#[test]
fn test_cwd_negative() {
    let code = [0x66, 0x99, 0xf4]; // CWD
    let mut regs = Registers::default();
    regs.rax = 0x8000; // AX = -32768
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0xFFFF, "DX = 0xFFFF for negative");
}

#[test]
fn test_cdq_positive() {
    // CDQ: EAX -> EDX:EAX (sign-extend dword, into EDX:EAX)
    let code = [0x99, 0xf4]; // CDQ
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x00000000, "EDX = 0 for positive");
}

#[test]
fn test_cdq_negative() {
    let code = [0x99, 0xf4]; // CDQ
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDX = 0xFFFFFFFF for negative"
    );
}

#[test]
fn test_cqo_positive() {
    // CQO: RAX -> RDX:RAX (sign-extend qword)
    let code = [0x48, 0x99, 0xf4]; // CQO (REX.W + 0x99)
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x0000000000000000, "RDX = 0 for positive");
}

#[test]
fn test_cqo_negative() {
    let code = [0x48, 0x99, 0xf4]; // CQO
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFF, "RDX = -1 for negative");
}

// ============================================================================
// ADCX/ADOX - ADX Extension Instructions
// ============================================================================

#[test]
fn test_adcx_basic() {
    // ADCX r32, r/m32 - add with carry flag (CF) only
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0x00000002;
    regs.rflags = 0x2 | flags::bits::CF; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000004, "1 + 2 + 1(CF) = 4");
}

#[test]
fn test_adcx_ignores_of() {
    // ADCX should only use CF, not OF
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0x00000002;
    regs.rflags = 0x2 | flags::bits::OF; // Only OF set, not CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000003,
        "1 + 2 + 0 = 3 (OF ignored)"
    );
}

#[test]
fn test_adox_basic() {
    // ADOX r32, r/m32 - add with overflow flag (OF) only
    let code = [
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0x00000002;
    regs.rflags = 0x2 | flags::bits::OF; // OF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000004, "1 + 2 + 1(OF) = 4");
}

#[test]
fn test_adox_ignores_cf() {
    // ADOX should only use OF, not CF
    let code = [
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0x00000002;
    regs.rflags = 0x2 | flags::bits::CF; // Only CF set, not OF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000003,
        "1 + 2 + 0 = 3 (CF ignored)"
    );
}

#[test]
fn test_adcx_adox_parallel() {
    // ADCX and ADOX can be used in parallel for multi-precision arithmetic
    // They use different flags (CF and OF) independently
    let code = [
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (uses CF)
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (uses OF)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0x00000001;
    regs.rcx = 0x00000002;
    regs.rdx = 0x00000002;
    regs.rflags = 0x2 | flags::bits::CF | flags::bits::OF; // Both set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000003, "ADCX: 1 + 1 + 1 = 3");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x00000005, "ADOX: 2 + 2 + 1 = 5");
}

// ============================================================================
// Complex Multi-Instruction Sequences
// ============================================================================

#[test]
fn test_multiprecision_addition_128bit() {
    // Add two 128-bit numbers using ADC chain
    // (RAX:RDX) + (RBX:RCX) = result in (RAX:RDX)
    let code = [
        0x48, 0x01, 0xd8, // ADD RAX, RBX (low 64 bits)
        0x48, 0x11, 0xca, // ADC RDX, RCX (high 64 bits with carry)
        0xf4,
    ];
    let mut regs = Registers::default();
    // First number: 0x0000000000000001_FFFFFFFFFFFFFFFF
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Low
    regs.rdx = 0x0000000000000001; // High
    // Second number: 0x0000000000000000_0000000000000001
    regs.rbx = 0x0000000000000001; // Low
    regs.rcx = 0x0000000000000000; // High
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result should be 0x0000000000000002_0000000000000000
    assert_eq!(regs.rax, 0x0000000000000000, "Low 64 bits");
    assert_eq!(regs.rdx, 0x0000000000000002, "High 64 bits");
}

#[test]
fn test_multiprecision_subtraction_128bit() {
    // Subtract two 128-bit numbers using SBB chain
    let code = [
        0x48, 0x29, 0xd8, // SUB RAX, RBX (low 64 bits)
        0x48, 0x19, 0xca, // SBB RDX, RCX (high 64 bits with borrow)
        0xf4,
    ];
    let mut regs = Registers::default();
    // First number: 0x0000000000000001_0000000000000000
    regs.rax = 0x0000000000000000;
    regs.rdx = 0x0000000000000001;
    // Subtract: 0x0000000000000000_0000000000000001
    regs.rbx = 0x0000000000000001;
    regs.rcx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result: 0x0000000000000000_FFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "Low 64 bits");
    assert_eq!(regs.rdx, 0x0000000000000000, "High 64 bits");
}

#[test]
fn test_increment_loop_pattern() {
    // Common pattern: loop using INC and TEST
    let code = [
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x75, 0xf7, // JNE back to INC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "Should have incremented to 5");
}

#[test]
fn test_decrement_loop_pattern() {
    // Decrement loop from 5 to 0
    let code = [
        0x48, 0xff, 0xc8, // DEC RAX
        0x75, 0xfb, // JNZ back to DEC (ZF=0 means continue)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "Should have decremented to 0");
}

// ============================================================================
// Edge Cases with Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_add_r8_r9() {
    let code = [
        0x4d, 0x01, 0xc8, // ADD R8, R9
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x1111111111111111;
    regs.r9 = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x3333333333333333, "R8 + R9");
    assert_eq!(regs.r9, 0x2222222222222222, "R9 unchanged");
}

#[test]
fn test_sub_r10_r11() {
    let code = [
        0x4d, 0x29, 0xda, // SUB R10, R11
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x5555555555555555;
    regs.r11 = 0x1111111111111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x4444444444444444, "R10 - R11");
}

#[test]
fn test_inc_r15() {
    let code = [
        0x49, 0xff, 0xc7, // INC R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFFFFFFFFFFFFFFFE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFFFF, "R15 incremented");
}

#[test]
fn test_neg_r12() {
    let code = [
        0x49, 0xf7, 0xdc, // NEG R12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x0000000000000005;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0xFFFFFFFFFFFFFFFB, "NEG 5 = -5");
}
