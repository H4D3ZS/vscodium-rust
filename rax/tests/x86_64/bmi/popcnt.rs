use crate::common::*;
use rax::cpu::Registers;

// POPCNT - Return the Count of Number of Bits Set to 1
// This instruction calculates the number of bits set to 1 in the second operand (source)
// and returns the count in the first operand (a destination register).
//
// Opcodes:
// F3 0F B8 /r           POPCNT r16, r/m16   - Count bits set in r/m16
// F3 0F B8 /r           POPCNT r32, r/m32   - Count bits set in r/m32
// F3 REX.W 0F B8 /r     POPCNT r64, r/m64   - Count bits set in r/m64
//
// Flags:
// OF, SF, AF, CF, PF are all cleared
// ZF is set if SRC = 0, otherwise ZF is cleared

// ============================================================================
// Basic 16-bit POPCNT Tests
// ============================================================================

#[test]
fn test_popcnt_16bit_zero() {
    // POPCNT AX, CX - count bits in zero
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "AX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set when source is 0");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
}

#[test]
fn test_popcnt_16bit_all_ones() {
    // POPCNT AX, CX - count bits in all ones
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 16, "AX should be 16");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_popcnt_16bit_single_bit() {
    // POPCNT AX, CX - count single bit
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0b0000_0000_1000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 1, "AX should be 1");
}

#[test]
fn test_popcnt_16bit_alternating_pattern() {
    // POPCNT AX, CX - alternating bits
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0b1010_1010_1010_1010;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 8, "AX should be 8");
}

#[test]
fn test_popcnt_16bit_high_byte_only() {
    // POPCNT AX, CX - bits only in high byte
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 8, "AX should be 8");
}

// ============================================================================
// Basic 32-bit POPCNT Tests
// ============================================================================

#[test]
fn test_popcnt_32bit_zero() {
    // POPCNT EAX, ECX - count bits in zero
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set when source is 0");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
}

#[test]
fn test_popcnt_32bit_all_ones() {
    // POPCNT EAX, ECX - count bits in all ones
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 32, "EAX should be 32");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_popcnt_32bit_single_bit() {
    // POPCNT EAX, ECX - count single bit
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1");
}

#[test]
fn test_popcnt_32bit_alternating_pattern() {
    // POPCNT EAX, ECX - alternating bits
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "EAX should be 16");
}

#[test]
fn test_popcnt_32bit_two_bits() {
    // POPCNT EAX, ECX - two bits set
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x00000003; // bits 0 and 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 2, "EAX should be 2");
}

#[test]
fn test_popcnt_32bit_sparse_bits() {
    // POPCNT EAX, ECX - sparse bit pattern
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x80000001; // bits at edges
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 2, "EAX should be 2");
}

#[test]
fn test_popcnt_32bit_consecutive_bits() {
    // POPCNT EAX, ECX - consecutive bits
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x000000FF; // 8 consecutive bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 8, "EAX should be 8");
}

// ============================================================================
// Basic 64-bit POPCNT Tests
// ============================================================================

#[test]
fn test_popcnt_64bit_zero() {
    // POPCNT RAX, RCX - count bits in zero
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set when source is 0");
}

#[test]
fn test_popcnt_64bit_all_ones() {
    // POPCNT RAX, RCX - count bits in all ones
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "RAX should be 64");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_popcnt_64bit_single_bit() {
    // POPCNT RAX, RCX - count single bit
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_popcnt_64bit_alternating_pattern() {
    // POPCNT RAX, RCX - alternating bits
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 32, "RAX should be 32");
}

#[test]
fn test_popcnt_64bit_lower_half_only() {
    // POPCNT RAX, RCX - bits only in lower 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 32, "RAX should be 32");
}

#[test]
fn test_popcnt_64bit_upper_half_only() {
    // POPCNT RAX, RCX - bits only in upper 32 bits
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFF00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 32, "RAX should be 32");
}

#[test]
fn test_popcnt_64bit_edge_bits() {
    // POPCNT RAX, RCX - bits at both edges
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x8000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2, "RAX should be 2");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_popcnt_16bit_memory_operand() {
    // POPCNT AX, [addr]
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // POPCNT AX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0b1111_0000_1111_0000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 8, "AX should be 8");
}

#[test]
fn test_popcnt_32bit_memory_operand() {
    // POPCNT EAX, [addr]
    let code = [
        0xf3, 0x0f, 0xb8, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // POPCNT EAX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xF0F0F0F0);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "EAX should be 16");
}

#[test]
fn test_popcnt_64bit_memory_operand() {
    // POPCNT RAX, [addr]
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // POPCNT RAX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0F0F0F0F0F0F0F0F);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 32, "RAX should be 32");
}

#[test]
fn test_popcnt_32bit_memory_zero() {
    // POPCNT EAX, [addr] - memory contains zero
    let code = [
        0xf3, 0x0f, 0xb8, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // POPCNT EAX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// ============================================================================
// Flag Behavior Tests
// ============================================================================

#[test]
fn test_popcnt_flags_cleared_on_nonzero() {
    // Test that OF, SF, AF, CF, PF are cleared
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // mov rax, -1
        0x48, 0x83, 0xc0, 0x01, // add rax, 1 (sets flags)
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xFF; // Non-zero value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF should be cleared");
    assert!(!sf_set(regs.rflags), "SF should be cleared");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
}

#[test]
fn test_popcnt_zf_set_on_zero() {
    // Test that ZF is set when source is zero
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(zf_set(regs.rflags), "ZF should be set when source is 0");
}

#[test]
fn test_popcnt_zf_clear_on_nonzero() {
    // Test that ZF is clear when source is non-zero
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear when source is non-zero"
    );
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_popcnt_32bit_r8d() {
    // POPCNT R8D, R9D
    let code = [
        0xf3, 0x45, 0x0f, 0xb8, 0xc1, // POPCNT R8D, R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 32, "R8D should be 32");
}

#[test]
fn test_popcnt_64bit_r10() {
    // POPCNT R10, R11
    let code = [
        0xf3, 0x4d, 0x0f, 0xb8, 0xd3, // POPCNT R10, R11
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x5555555555555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 32, "R10 should be 32");
}

#[test]
fn test_popcnt_32bit_from_r15d() {
    // POPCNT EAX, R15D
    let code = [
        0xf3, 0x41, 0x0f, 0xb8, 0xc7, // POPCNT EAX, R15D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Count bits in 0x12345678
    let expected = 0x12345678u32.count_ones();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "EAX should match bit count"
    );
}

// ============================================================================
// Various Bit Patterns
// ============================================================================

#[test]
fn test_popcnt_32bit_power_of_two_minus_one() {
    // POPCNT EAX, ECX - 2^n - 1 patterns
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x0000FFFF; // 2^16 - 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 16, "EAX should be 16");
}

#[test]
fn test_popcnt_64bit_checkerboard_pattern() {
    // POPCNT RAX, RCX - checkerboard pattern
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x5555555555555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 32, "RAX should be 32");
}

#[test]
fn test_popcnt_32bit_byte_patterns() {
    // POPCNT EAX, ECX - byte-wise patterns
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x01010101; // One bit per byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 4, "EAX should be 4");
}

#[test]
fn test_popcnt_64bit_nibble_patterns() {
    // POPCNT RAX, RCX - nibble patterns
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x1111111111111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 16, "RAX should be 16");
}

#[test]
fn test_popcnt_32bit_mersenne_pattern() {
    // POPCNT EAX, ECX - Mersenne-like patterns
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x7FFFFFFF; // 2^31 - 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 31, "EAX should be 31");
}

#[test]
fn test_popcnt_64bit_fibonacci_bits() {
    // POPCNT RAX, RCX - Fibonacci-inspired bit positions
    let code = [
        0xf3, 0x48, 0x0f, 0xb8, 0xc1, // POPCNT RAX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    // Bits at positions: 1, 2, 3, 5, 8, 13, 21, 34 (Fibonacci sequence)
    regs.rcx =
        (1u64 << 1) | (1 << 2) | (1 << 3) | (1 << 5) | (1 << 8) | (1 << 13) | (1 << 21) | (1 << 34);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 8, "RAX should be 8");
}

#[test]
fn test_popcnt_16bit_each_bit_position() {
    // Test that each bit position is counted correctly
    let code = [
        0x66, 0xf3, 0x0f, 0xb8, 0xc1, // POPCNT AX, CX
        0xf4,
    ];

    for i in 0..16 {
        let mut regs = Registers::default();
        regs.rcx = 1u64 << i;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(regs.rax & 0xFFFF, 1, "AX should be 1 for bit {}", i);
    }
}

#[test]
fn test_popcnt_32bit_progressive_bits() {
    // Test progressive bit patterns
    let code = [
        0xf3, 0x0f, 0xb8, 0xc1, // POPCNT EAX, ECX
        0xf4,
    ];

    let patterns = [
        0x00000001, // 1 bit
        0x00000003, // 2 bits
        0x00000007, // 3 bits
        0x0000000F, // 4 bits
        0x000000FF, // 8 bits
    ];

    for (idx, &pattern) in patterns.iter().enumerate() {
        let mut regs = Registers::default();
        regs.rcx = pattern;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = pattern.count_ones() as u64;
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Pattern {} should have {} bits",
            idx,
            expected
        );
    }
}

// ============================================================================
// Known-answer value tests (exact counts + ZF) for POPCNT.
// ============================================================================

#[test]
fn kat_popcnt_r64_known() {
    // POPCNT RAX, RCX (F3 REX.W 0F B8 C1). 0x0F0F0F0F0F0F0F0F has 32 bits set.
    let code = [0xf3, 0x48, 0x0f, 0xb8, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x0F0F0F0F0F0F0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 32, "POPCNT RAX = {}", regs.rax);
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(!cf_set(regs.rflags) && !of_set(regs.rflags) && !sf_set(regs.rflags));
}

#[test]
fn kat_popcnt_r32_known() {
    // POPCNT EAX, ECX (F3 0F B8 C1). 0xABCD1234 has 15 bits set.
    let code = [0xf3, 0x0f, 0xb8, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0xABCD1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0xABCD1234u32.count_ones() as u64);
    assert_eq!(regs.rax & 0xFFFF_FFFF, 15);
}
