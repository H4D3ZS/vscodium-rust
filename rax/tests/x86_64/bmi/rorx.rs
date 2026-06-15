use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// RORX - Rotate Right Logical Without Affecting Flags (BMI2)
// Rotates the source operand right by imm8 bits without affecting any flags.
//
// Unlike the traditional ROR instruction, RORX:
// - Has separate destination and source operands (3-operand form)
// - Does NOT modify any flags (unlike ROR which sets CF and OF)
// - Only accepts an immediate byte count (no CL variant)
// - Is part of the BMI2 instruction set extension
//
// Opcodes:
// VEX.LZ.F2.0F3A.W0 F0 /r ib   RORX r32, r/m32, imm8   - Rotate 32-bit operand right
// VEX.LZ.F2.0F3A.W1 F0 /r ib   RORX r64, r/m64, imm8   - Rotate 64-bit operand right
//
// Operation:
// IF (OperandSize = 32)
//   y := imm8 AND 1FH
//   DEST := (SRC >> y) | (SRC << (32-y))
// ELSE IF (OperandSize = 64)
//   y := imm8 AND 3FH
//   DEST := (SRC >> y) | (SRC << (64-y))
//
// Flags: None affected

// ============================================================================
// RORX 32-bit Basic Tests
// ============================================================================

#[test]
fn test_rorx_32bit_rotate_0() {
    // RORX EAX, EBX, 0 - rotate by 0 (no change)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x00, // RORX EAX, EBX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX should be unchanged");
}

#[test]
fn test_rorx_32bit_rotate_1() {
    // RORX EAX, EBX, 1 - rotate by 1 bit
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x091A2B3C,
        "EAX should be rotated right by 1"
    );
}

#[test]
fn test_rorx_32bit_rotate_4() {
    // RORX EAX, EBX, 4 - rotate by 4 bits (nibble)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x81234567,
        "EAX should be rotated right by 4"
    );
}

#[test]
fn test_rorx_32bit_rotate_8() {
    // RORX EAX, EBX, 8 - rotate by 8 bits (byte)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78123456,
        "EAX should be rotated right by 8"
    );
}

#[test]
fn test_rorx_32bit_rotate_16() {
    // RORX EAX, EBX, 16 - rotate by 16 bits (word swap)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x10, // RORX EAX, EBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x56781234,
        "EAX should be rotated right by 16"
    );
}

#[test]
fn test_rorx_32bit_rotate_24() {
    // RORX EAX, EBX, 24 - rotate by 24 bits
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x18, // RORX EAX, EBX, 24
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x34567812,
        "EAX should be rotated right by 24"
    );
}

#[test]
fn test_rorx_32bit_rotate_31() {
    // RORX EAX, EBX, 31 - rotate by 31 bits (almost full rotation)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x1F, // RORX EAX, EBX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468ACF0,
        "EAX should be rotated right by 31"
    );
}

#[test]
fn test_rorx_32bit_rotate_32() {
    // RORX EAX, EBX, 32 - count masked to 0 (full rotation)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x20, // RORX EAX, EBX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 32 & 0x1F = 0, so no rotation
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should be unchanged (count masked to 0)"
    );
}

#[test]
fn test_rorx_32bit_count_masked() {
    // RORX EAX, EBX, 40 - verify count is masked to 5 bits
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x28, // RORX EAX, EBX, 40 (0x28)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 40 & 0x1F = 8
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78123456,
        "EAX should be rotated by 8 (40 masked to 8)"
    );
}

// ============================================================================
// RORX 64-bit Basic Tests
// ============================================================================

#[test]
fn test_rorx_64bit_rotate_0() {
    // RORX RAX, RBX, 0 - rotate by 0 (no change)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x00, // RORX RAX, RBX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX should be unchanged");
}

#[test]
fn test_rorx_64bit_rotate_1() {
    // RORX RAX, RBX, 1 - rotate by 1 bit
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x01, // RORX RAX, RBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x091A2B3C4D5E6F78,
        "RAX should be rotated right by 1"
    );
}

#[test]
fn test_rorx_64bit_rotate_4() {
    // RORX RAX, RBX, 4 - rotate by 4 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x04, // RORX RAX, RBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0123456789ABCDEF,
        "RAX should be rotated right by 4"
    );
}

#[test]
fn test_rorx_64bit_rotate_8() {
    // RORX RAX, RBX, 8 - rotate by 8 bits (byte)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x08, // RORX RAX, RBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xF0123456789ABCDE,
        "RAX should be rotated right by 8"
    );
}

#[test]
fn test_rorx_64bit_rotate_16() {
    // RORX RAX, RBX, 16 - rotate by 16 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xDEF0123456789ABC,
        "RAX should be rotated right by 16"
    );
}

#[test]
fn test_rorx_64bit_rotate_32() {
    // RORX RAX, RBX, 32 - rotate by 32 bits (dword swap)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x20, // RORX RAX, RBX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x9ABCDEF012345678,
        "RAX should be rotated right by 32"
    );
}

#[test]
fn test_rorx_64bit_rotate_48() {
    // RORX RAX, RBX, 48 - rotate by 48 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x30, // RORX RAX, RBX, 48
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x56789ABCDEF01234,
        "RAX should be rotated right by 48"
    );
}

#[test]
fn test_rorx_64bit_rotate_63() {
    // RORX RAX, RBX, 63 - rotate by 63 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x3F, // RORX RAX, RBX, 63
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x2468ACF13579BDE0,
        "RAX should be rotated right by 63"
    );
}

#[test]
fn test_rorx_64bit_rotate_64() {
    // RORX RAX, RBX, 64 - count masked to 0 (full rotation)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x40, // RORX RAX, RBX, 64
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 64 & 0x3F = 0, so no rotation
    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX should be unchanged (count masked to 0)"
    );
}

#[test]
fn test_rorx_64bit_count_masked() {
    // RORX RAX, RBX, 80 - verify count is masked to 6 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x50, // RORX RAX, RBX, 80 (0x50)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 80 & 0x3F = 16
    assert_eq!(
        regs.rax, 0xDEF0123456789ABC,
        "RAX should be rotated by 16 (80 masked to 16)"
    );
}

// ============================================================================
// Flag Tests - RORX Does NOT Modify Flags
// ============================================================================

#[test]
fn test_rorx_32bit_flags_not_modified_cf_set() {
    // RORX should not modify CF even when set
    let code = [
        0xf9, // STC (set CF)
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX should be rotated");
    assert!(
        (regs.rflags & 1) != 0,
        "CF should still be set (RORX doesn't modify flags)"
    );
}

#[test]
fn test_rorx_32bit_flags_not_modified_cf_clear() {
    // RORX should not modify CF even when clear
    let code = [
        0xf8, // CLC (clear CF)
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX should be rotated");
    assert!(
        (regs.rflags & 1) == 0,
        "CF should still be clear (RORX doesn't modify flags)"
    );
}

#[test]
fn test_rorx_64bit_preserves_all_flags() {
    // Test that RORX preserves all arithmetic flags
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF, AF)
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xDEF0123456789ABC, "RAX should be rotated");
    assert!((regs.rflags & 1) != 0, "CF should still be set from SUB");
    assert!((regs.rflags & 0x80) != 0, "SF should still be set from SUB");
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_rorx_32bit_all_zeros() {
    // RORX with all zeros
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "All zeros remain all zeros"
    );
}

#[test]
fn test_rorx_32bit_all_ones() {
    // RORX with all ones
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "All ones remain all ones"
    );
}

#[test]
fn test_rorx_32bit_alternating_pattern() {
    // RORX with alternating bits
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 10101010...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "Alternating pattern rotates correctly"
    );
}

#[test]
fn test_rorx_64bit_alternating_pattern() {
    // RORX with alternating bits (64-bit)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x01, // RORX RAX, RBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x5555555555555555,
        "Alternating pattern rotates correctly"
    );
}

#[test]
fn test_rorx_32bit_single_bit() {
    // RORX with single bit set
    // Rotate right: bit 8 moves to bit 0 when rotating by 8
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000100; // Bit 8 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x100 >> 8 = 0x1 (bit 8 rotates to bit 0)
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "Single bit rotates correctly"
    );
}

#[test]
fn test_rorx_64bit_single_bit() {
    // RORX with single bit set (64-bit)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x20, // RORX RAX, RBX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000100000000; // Bit 32 set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "Single bit rotates correctly");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_rorx_32bit_memory_operand() {
    // RORX EAX, [0x2000], 8
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x08, // RORX EAX, [0x2000], 8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use crate::common::write_mem_u32;
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78123456,
        "EAX should be rotated from memory"
    );
}

#[test]
fn test_rorx_64bit_memory_operand() {
    // RORX RAX, [0x2000], 16
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x10, // RORX RAX, [0x2000], 16
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use crate::common::write_mem_u64;
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xDEF0123456789ABC,
        "RAX should be rotated from memory"
    );
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_rorx_32bit_r8d_r9d() {
    // RORX R8D, R9D, 8
    let code = [
        0xc4, 0x43, 0x7b, 0xf0, 0xc1, 0x08, // RORX R8D, R9D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x78123456,
        "R8D should be rotated from R9D"
    );
}

#[test]
fn test_rorx_64bit_r14_r15() {
    // RORX R14, R15, 16
    let code = [
        0xc4, 0x43, 0xfb, 0xf0, 0xf7, 0x10, // RORX R14, R15, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r14, 0xDEF0123456789ABC,
        "R14 should be rotated from R15"
    );
}

#[test]
fn test_rorx_32bit_eax_r11d() {
    // RORX EAX, R11D, 4
    let code = [
        0xc4, 0xc3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, R11D, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x81234567,
        "EAX should be rotated from R11D"
    );
}

// ============================================================================
// Source Preservation Tests
// ============================================================================

#[test]
fn test_rorx_32bit_source_unchanged() {
    // RORX should not modify source register
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
}

#[test]
fn test_rorx_64bit_source_unchanged() {
    // RORX should not modify source register (64-bit)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x123456789ABCDEF0, "RBX should be unchanged");
}

// ============================================================================
// Edge Cases and Practical Uses
// ============================================================================

#[test]
fn test_rorx_32bit_byte_swap() {
    // Use RORX for endianness conversion (partial)
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAABBCCDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xDDAABBCC, "Bytes rotated");
}

#[test]
fn test_rorx_multiple_rotations() {
    // Chain multiple RORX operations
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xc4, 0xe3, 0x7b, 0xf0, 0xd8, 0x08, // RORX EBX, EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 0x12345678 RORX 8 = 0x78123456
    // Second: 0x78123456 RORX 8 = 0x56781234
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX after first RORX");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x56781234, "EBX after second RORX");
}

#[test]
fn test_rorx_32bit_various_counts() {
    // Test multiple rotation counts in sequence
    // ror32(0x12345678, n) = (0x12345678 >> n) | (0x12345678 << (32-n))
    let counts_and_results = vec![
        (0, 0x12345678),
        (1, 0x091A2B3C),
        (2, 0x048D159E),
        (3, 0x02468ACF),
        (5, 0xC091A2B3),
        (7, 0xF02468AC),
        (12, 0x67812345),
        (20, 0x45678123),
        (28, 0x23456781),
    ];

    for (count, expected) in counts_and_results {
        let code = [
            0xc4, 0xe3, 0x7b, 0xf0, 0xc3, count, // RORX EAX, EBX, count
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x12345678;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "RORX by {} should produce 0x{:08X}",
            count,
            expected
        );
    }
}

#[test]
fn test_rorx_64bit_various_counts() {
    // Test multiple rotation counts in 64-bit mode
    let counts_and_results = vec![
        (0, 0x123456789ABCDEF0),
        (4, 0x0123456789ABCDEF),
        (8, 0xF0123456789ABCDE),
        (12, 0xEF0123456789ABCD),
        (20, 0xCDEF0123456789AB),
        (32, 0x9ABCDEF012345678),
        (40, 0x789ABCDEF0123456),
        (56, 0x3456789ABCDEF012),
    ];

    for (count, expected) in counts_and_results {
        let code = [
            0xc4, 0xe3, 0xfb, 0xf0, 0xc3, count, // RORX RAX, RBX, count
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x123456789ABCDEF0;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax, expected,
            "RORX by {} should produce 0x{:016X}",
            count, expected
        );
    }
}
