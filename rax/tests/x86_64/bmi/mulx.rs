use crate::common::*;
use rax::cpu::Registers;

// MULX - Unsigned Multiply Without Affecting Flags (BMI2)
// Performs an unsigned multiplication of the implicit source operand (EDX/RDX) and the specified
// source operand (third operand) and stores the low half of the result in the second destination
// (second operand), the high half of the result in the first destination operand (first operand),
// without reading or writing the arithmetic flags.
//
// Opcodes:
// VEX.LZ.F2.0F38.W0 F6 /r   MULX r32a, r32b, r/m32   - dest1 = (EDX * r/m32)[63:32], dest2 = (EDX * r/m32)[31:0]
// VEX.LZ.F2.0F38.W1 F6 /r   MULX r64a, r64b, r/m64   - dest1 = (RDX * r/m64)[127:64], dest2 = (RDX * r/m64)[63:0]
//
// Important: The implicit source is EDX/RDX, the third operand (ModRM:r/m) is the explicit source,
// dest1 (ModRM:reg) receives the high half, and dest2 (VEX.vvvv) receives the low half.

// ============================================================================
// Basic 32-bit MULX Tests
// ============================================================================

#[test]
fn test_mulx_basic_32bit() {
    // MULX EAX, EBX, ECX - EAX = high(EDX * ECX), EBX = low(EDX * ECX)
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 10;
    regs.rcx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 10 * 20 = 200 (0xC8)
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        200,
        "EBX should contain low 32 bits (200)"
    );
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should contain high 32 bits (0)"
    );
}

#[test]
fn test_mulx_32bit_overflow_to_high() {
    // MULX EAX, EBX, ECX - test where result needs high 32 bits
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF; // Max 32-bit value
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFF * 2 = 0x1FFFFFFFE
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EBX should contain low 32 bits"
    );
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain high 32 bits");
}

#[test]
fn test_mulx_32bit_max_values() {
    // MULX EAX, EBX, ECX - max * max
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFF * 0xFFFFFFFF = 0xFFFFFFFE00000001
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x00000001,
        "EBX should contain low 32 bits"
    );
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX should contain high 32 bits"
    );
}

#[test]
fn test_mulx_32bit_zero_multiplier() {
    // MULX EAX, EBX, ECX - multiply by zero
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_mulx_32bit_zero_multiplicand() {
    // MULX EAX, EBX, ECX - multiply zero
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_mulx_32bit_multiply_by_one() {
    // MULX EAX, EBX, ECX - multiply by 1
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x12345678;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should equal EDX");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_mulx_32bit_power_of_two() {
    // MULX EAX, EBX, ECX - multiply by power of 2
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x80000000; // 2^31
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x80000000 * 2 = 0x100000000
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1");
}

#[test]
fn test_mulx_32bit_same_dest_regs() {
    // MULX EAX, EAX, ECX - both destinations are the same register
    // According to spec, it will contain the high half
    let code = [
        0xc4, 0xe2, 0x7b, 0xf6, 0xc1, // MULX EAX, EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // When both destinations are the same, it contains the high half
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX should contain high 32 bits"
    );
}

// ============================================================================
// 64-bit MULX Tests
// ============================================================================

#[test]
fn test_mulx_basic_64bit() {
    // MULX RAX, RBX, RCX - RAX = high(RDX * RCX), RBX = low(RDX * RCX)
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 10;
    regs.rcx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 10 * 20 = 200
    assert_eq!(regs.rbx, 200, "RBX should contain low 64 bits (200)");
    assert_eq!(regs.rax, 0, "RAX should contain high 64 bits (0)");
}

#[test]
fn test_mulx_64bit_overflow_to_high() {
    // MULX RAX, RBX, RCX - test where result needs high 64 bits
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFFFFFFFFFF * 2 = 0x1FFFFFFFFFFFFFFFE
    assert_eq!(
        regs.rbx, 0xFFFFFFFFFFFFFFFE,
        "RBX should contain low 64 bits"
    );
    assert_eq!(regs.rax, 1, "RAX should contain high 64 bits");
}

#[test]
fn test_mulx_64bit_max_values() {
    // MULX RAX, RBX, RCX - max * max
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFFFFFFFFFF * 0xFFFFFFFFFFFFFFFF = 0xFFFFFFFFFFFFFFFE0000000000000001
    assert_eq!(
        regs.rbx, 0x0000000000000001,
        "RBX should contain low 64 bits"
    );
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFE,
        "RAX should contain high 64 bits"
    );
}

#[test]
fn test_mulx_64bit_zero_multiplier() {
    // MULX RAX, RBX, RCX - multiply by zero
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

#[test]
fn test_mulx_64bit_multiply_by_one() {
    // MULX RAX, RBX, RCX - multiply by 1
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x123456789ABCDEF0;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x123456789ABCDEF0, "RBX should equal RDX");
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

#[test]
fn test_mulx_64bit_power_of_two() {
    // MULX RAX, RBX, RCX - multiply by power of 2
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x8000000000000000; // 2^63
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x8000000000000000 * 2 = 0x10000000000000000
    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_mulx_64bit_large_numbers() {
    // MULX RAX, RBX, RCX - test with large numbers
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x0000000100000000; // 2^32
    regs.rcx = 0x0000000100000000; // 2^32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 2^32 * 2^32 = 2^64 = 0x10000000000000000
    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_eq!(regs.rax, 1, "RAX should be 1");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_mulx_32bit_memory_operand() {
    // MULX EAX, EBX, [addr] - multiply with memory operand
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MULX EAX, EBX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 100;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 50);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 100 * 50 = 5000 (0x1388)
    assert_eq!(regs.rbx & 0xFFFFFFFF, 5000, "EBX should contain result");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_mulx_64bit_memory_operand() {
    // MULX RAX, RBX, [addr] - multiply with memory operand
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MULX RAX, RBX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x0000000200000000; // 2 * 2^32
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0000000200000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // (2 * 2^32) * (2 * 2^32) = 4 * 2^64 = 0x40000000000000000
    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_eq!(regs.rax, 4, "RAX should be 4");
}

// ============================================================================
// Flag Tests - MULX does NOT modify flags
// ============================================================================

#[test]
fn test_mulx_32bit_flags_not_modified() {
    // Test that MULX does not modify flags
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
        0x48, 0x83, 0xe8, 0x02, // sub rax, 2 (sets CF, SF, AF)
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF;
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should still be set from the SUB instruction
    assert!(cf_set(regs.rflags), "CF should still be set");
    assert!(sf_set(regs.rflags), "SF should still be set");
}

#[test]
fn test_mulx_64bit_flags_not_modified() {
    // Test that MULX does not modify flags
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
        0x48, 0xff, 0xc0, // inc rax (sets flags)
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should be clear from the INC instruction
    assert!(!zf_set(regs.rflags), "ZF should still be clear");
}

#[test]
fn test_mulx_preserves_all_flags() {
    // Set all flags, then execute MULX, verify flags unchanged
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -1
        0x48, 0x83, 0xc0, 0x01, // add rax, 1 (sets ZF, clears CF)
        0xf8, // clc
        0xf9, // stc (set CF)
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 12345;
    regs.rcx = 67890;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should still be set from STC
    assert!(cf_set(regs.rflags), "CF should still be set");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_mulx_32bit_with_r8d() {
    // MULX R8D, EBX, ECX
    let code = [
        0xc4, 0x62, 0x63, 0xf6, 0xc1, // MULX R8D, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 1000;
    regs.rcx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 1000 * 2000 = 2,000,000 (0x1E8480)
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        2000000,
        "EBX should contain low 32 bits"
    );
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0,
        "R8D should contain high 32 bits (0)"
    );
}

#[test]
fn test_mulx_64bit_with_r9() {
    // MULX R9, R10, R11
    let code = [
        0xc4, 0x02, 0xab, 0xf6, 0xcb, // MULX R9, R10, R11
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x100000000; // 2^32
    regs.r11 = 0x100000000; // 2^32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 2^32 * 2^32 = 2^64
    assert_eq!(regs.r10, 0, "R10 should be 0");
    assert_eq!(regs.r9, 1, "R9 should be 1");
}

#[test]
fn test_mulx_32bit_r15d_source() {
    // MULX EAX, EBX, R15D
    let code = [
        0xc4, 0xc2, 0x63, 0xf6, 0xc7, // MULX EAX, EBX, R15D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 999;
    regs.r15 = 1001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 999 * 1001 = 999999 (0xF423F)
    assert_eq!(regs.rbx & 0xFFFFFFFF, 999999, "EBX should contain result");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

// ============================================================================
// Various Operand Combinations
// ============================================================================

#[test]
fn test_mulx_32bit_small_numbers() {
    // MULX EAX, EBX, ECX - small numbers
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 3;
    regs.rcx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 21, "EBX should be 21");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_mulx_32bit_medium_numbers() {
    // MULX EAX, EBX, ECX - medium numbers
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x10000;
    regs.rcx = 0x10000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x10000 * 0x10000 = 0x100000000 (overflows 32-bit)
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1");
}

#[test]
fn test_mulx_64bit_alternating_bits() {
    // MULX RAX, RBX, RCX - alternating bit patterns
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xAAAAAAAAAAAAAAAA;
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xAAAAAAAAAAAAAAAA * 2 = 0x15555555555555554
    assert_eq!(
        regs.rbx, 0x5555555555555554,
        "RBX should contain low 64 bits"
    );
    assert_eq!(regs.rax, 1, "RAX should contain high 64 bits");
}

#[test]
fn test_mulx_32bit_half_max() {
    // MULX EAX, EBX, ECX - test with half of max value
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x80000000; // 2^31
    regs.rcx = 0x80000000; // 2^31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 2^31 * 2^31 = 2^62 = 0x4000000000000000
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x40000000,
        "EAX should be 0x40000000"
    );
}

#[test]
fn test_mulx_64bit_sequential_multiplications() {
    // Multiple MULX operations to verify state is maintained
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX (first)
        0x48, 0x89, 0xda, // mov rdx, rbx (move result to rdx)
        0xc4, 0x62, 0xb3, 0xf6,
        0xc1, // MULX R8, R9, RCX (R~=0 for R8, vvvv=0110=R9, B~=1 for RCX)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 10;
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 10 * 5 = 50
    // Second: 50 * 5 = 250
    assert_eq!(regs.r9, 250, "R9 should be 250");
    assert_eq!(regs.r8, 0, "R8 should be 0");
}

#[test]
fn test_mulx_32bit_prime_numbers() {
    // MULX EAX, EBX, ECX - multiply prime numbers
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 104729; // Prime
    regs.rcx = 104743; // Prime
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 104729 * 104743 = 10969629647
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        10969629647u64 & 0xFFFFFFFF,
        "EBX should contain low bits"
    );
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        10969629647u64 >> 32,
        "EAX should contain high bits"
    );
}

#[test]
fn test_mulx_64bit_fibonacci_like() {
    // MULX RAX, RBX, RCX - Fibonacci-like numbers
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 6765;
    regs.rcx = 10946;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 6765 * 10946 = 74049690
    assert_eq!(regs.rbx, 74049690, "RBX should be 74049690");
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

#[test]
fn test_mulx_32bit_bit_position_edge_cases() {
    // Test multiplying numbers with bits at edge positions
    let code = [
        0xc4, 0xe2, 0x63, 0xf6, 0xc1, // MULX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x80000001; // High and low bits set
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x80000001 * 2 = 0x100000002
    assert_eq!(regs.rbx & 0xFFFFFFFF, 2, "EBX should be 2");
    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1");
}

#[test]
fn test_mulx_64bit_carries_across_boundaries() {
    // Test that carries propagate correctly
    let code = [
        0xc4, 0xe2, 0xe3, 0xf6, 0xc1, // MULX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x00000000FFFFFFFF;
    regs.rcx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFF * 0xFFFFFFFF = 0xFFFFFFFE00000001
    assert_eq!(
        regs.rbx, 0xFFFFFFFE00000001,
        "RBX should have correct low bits"
    );
    assert_eq!(regs.rax, 0, "RAX should be 0");
}
