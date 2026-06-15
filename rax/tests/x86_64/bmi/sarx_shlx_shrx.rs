use crate::common::*;
use rax::cpu::Registers;

// SARX/SHLX/SHRX - Shift Without Affecting Flags (BMI2)
// These instructions perform shift operations without modifying any flags.
//
// SARX - Shift Arithmetic Right (sign-extending)
// SHLX - Shift Logical Left
// SHRX - Shift Logical Right
//
// Opcodes:
// VEX.LZ.F3.0F38.W0 F7 /r   SARX r32a, r/m32, r32b   - Arithmetic right shift r/m32 by r32b
// VEX.LZ.F3.0F38.W1 F7 /r   SARX r64a, r/m64, r64b   - Arithmetic right shift r/m64 by r64b
// VEX.LZ.66.0F38.W0 F7 /r   SHLX r32a, r/m32, r32b   - Logical left shift r/m32 by r32b
// VEX.LZ.66.0F38.W1 F7 /r   SHLX r64a, r/m64, r64b   - Logical left shift r/m64 by r64b
// VEX.LZ.F2.0F38.W0 F7 /r   SHRX r32a, r/m32, r32b   - Logical right shift r/m32 by r32b
// VEX.LZ.F2.0F38.W1 F7 /r   SHRX r64a, r/m64, r64b   - Logical right shift r/m64 by r64b
//
// Important: Shift count is masked to 5 bits (32-bit) or 6 bits (64-bit)
// Flags: None modified

// ============================================================================
// SHLX - Logical Left Shift Tests (32-bit)
// ============================================================================

#[test]
fn test_shlx_32bit_basic() {
    // SHLX EAX, EBX, ECX - basic left shift
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b0000_0001;
    regs.rcx = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0001_0000, "EAX should be 16");
}

#[test]
fn test_shlx_32bit_zero_shift() {
    // SHLX EAX, EBX, ECX - shift by 0
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX should be unchanged");
}

#[test]
fn test_shlx_32bit_shift_one() {
    // SHLX EAX, EBX, ECX - shift by 1
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555554,
        "EAX should be shifted left by 1"
    );
}

#[test]
fn test_shlx_32bit_shift_to_high_bit() {
    // SHLX EAX, EBX, ECX - shift bit to high position
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should have high bit set"
    );
}

#[test]
fn test_shlx_32bit_shift_overflow() {
    // SHLX EAX, EBX, ECX - shift out all bits
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 32; // Should be masked to 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Count is masked to 5 bits (32 & 0x1F = 0)
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be unchanged (count masked to 0)"
    );
}

#[test]
fn test_shlx_32bit_count_masked() {
    // SHLX EAX, EBX, ECX - verify count is masked to 5 bits
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 33; // Should be masked to 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        2,
        "EAX should be shifted by 1 (33 & 0x1F = 1)"
    );
}

// ============================================================================
// SHLX - Logical Left Shift Tests (64-bit)
// ============================================================================

#[test]
fn test_shlx_64bit_basic() {
    // SHLX RAX, RBX, RCX - basic left shift
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 256, "RAX should be 256");
}

#[test]
fn test_shlx_64bit_shift_to_high_bit() {
    // SHLX RAX, RBX, RCX - shift to highest bit
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 63;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000000000000000, "RAX should have high bit set");
}

#[test]
fn test_shlx_64bit_count_masked() {
    // SHLX RAX, RBX, RCX - verify count is masked to 6 bits
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 64; // Should be masked to 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be unchanged (64 & 0x3F = 0)");
}

#[test]
fn test_shlx_64bit_large_shift() {
    // SHLX RAX, RBX, RCX - large shift count
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000000FFFFFFFF;
    regs.rcx = 32;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFF00000000,
        "RAX should be shifted left by 32"
    );
}

// ============================================================================
// SHRX - Logical Right Shift Tests (32-bit)
// ============================================================================

#[test]
fn test_shrx_32bit_basic() {
    // SHRX EAX, EBX, ECX - basic right shift
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0b1111_0000;
    regs.rcx = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0b0000_1111, "EAX should be 15");
}

#[test]
fn test_shrx_32bit_zero_shift() {
    // SHRX EAX, EBX, ECX - shift by 0
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX should be unchanged");
}

#[test]
fn test_shrx_32bit_shift_high_bit() {
    // SHRX EAX, EBX, ECX - shift high bit down
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1");
}

#[test]
fn test_shrx_32bit_no_sign_extend() {
    // SHRX EAX, EBX, ECX - verify no sign extension (logical shift)
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x40000000,
        "EAX should not sign extend"
    );
}

#[test]
fn test_shrx_32bit_count_masked() {
    // SHRX EAX, EBX, ECX - verify count is masked
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 33; // Should be masked to 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFF,
        "EAX should be shifted by 1"
    );
}

// ============================================================================
// SHRX - Logical Right Shift Tests (64-bit)
// ============================================================================

#[test]
fn test_shrx_64bit_basic() {
    // SHRX RAX, RBX, RCX - basic right shift
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x100;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_shrx_64bit_shift_high_bit() {
    // SHRX RAX, RBX, RCX - shift from highest bit
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000;
    regs.rcx = 63;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_shrx_64bit_no_sign_extend() {
    // SHRX RAX, RBX, RCX - verify no sign extension
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x4000000000000000, "RAX should not sign extend");
}

#[test]
fn test_shrx_64bit_large_shift() {
    // SHRX RAX, RBX, RCX - large shift
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF00000000;
    regs.rcx = 32;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x00000000FFFFFFFF,
        "RAX should be shifted right by 32"
    );
}

// ============================================================================
// SARX - Arithmetic Right Shift Tests (32-bit)
// ============================================================================

#[test]
fn test_sarx_32bit_basic_positive() {
    // SARX EAX, EBX, ECX - shift positive number
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00001000;
    regs.rcx = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000100, "EAX should be 256");
}

#[test]
fn test_sarx_32bit_basic_negative() {
    // SARX EAX, EBX, ECX - shift negative number (sign extend)
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000u32 as u64; // Negative in 32-bit
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xC0000000, "EAX should sign extend");
}

#[test]
fn test_sarx_32bit_sign_extend_full() {
    // SARX EAX, EBX, ECX - shift negative number fully
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000u32 as u64;
    regs.rcx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be all ones");
}

#[test]
fn test_sarx_32bit_zero_shift() {
    // SARX EAX, EBX, ECX - shift by 0
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000u32 as u64;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000000, "EAX should be unchanged");
}

#[test]
fn test_sarx_32bit_positive_no_sign() {
    // SARX EAX, EBX, ECX - positive number doesn't sign extend
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x3FFFFFFF,
        "EAX should not sign extend"
    );
}

// ============================================================================
// SARX - Arithmetic Right Shift Tests (64-bit)
// ============================================================================

#[test]
fn test_sarx_64bit_basic_positive() {
    // SARX RAX, RBX, RCX - shift positive number
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000100000000;
    regs.rcx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000001000000, "RAX should be shifted right");
}

#[test]
fn test_sarx_64bit_basic_negative() {
    // SARX RAX, RBX, RCX - shift negative number
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000u64;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xC000000000000000, "RAX should sign extend");
}

#[test]
fn test_sarx_64bit_sign_extend_full() {
    // SARX RAX, RBX, RCX - shift negative fully
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000u64;
    regs.rcx = 63;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should be all ones");
}

#[test]
fn test_sarx_64bit_large_shift() {
    // SARX RAX, RBX, RCX - large shift on negative
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF00000000u64;
    regs.rcx = 32;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should sign extend");
}

// ============================================================================
// Flag Tests - All shift instructions do NOT modify flags
// ============================================================================

#[test]
fn test_shlx_32bit_flags_not_modified() {
    // Test that SHLX does not modify flags
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // mov rax, 1
        0x48, 0x83, 0xe8, 0x02, // sub rax, 2 (sets CF, SF, AF)
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 1;
    regs.rcx = 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should still be set from SUB");
    assert!(sf_set(regs.rflags), "SF should still be set from SUB");
}

#[test]
fn test_shrx_64bit_flags_not_modified() {
    // Test that SHRX does not modify flags
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
        0x48, 0xff, 0xc0, // inc rax (clears ZF)
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!zf_set(regs.rflags), "ZF should still be clear from INC");
}

#[test]
fn test_sarx_32bit_flags_not_modified() {
    // Test that SARX does not modify flags
    let code = [
        0xf9, // stc (set CF)
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should still be set from STC");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_shlx_32bit_memory_operand() {
    // SHLX EAX, [addr], ECX
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHLX EAX, [0x2000], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x00000001);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 256, "EAX should be 256");
}

#[test]
fn test_shrx_64bit_memory_operand() {
    // SHRX RAX, [addr], RCX
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHRX RAX, [0x2000], RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 16;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0000000100000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000010000, "RAX should be shifted");
}

#[test]
fn test_sarx_32bit_memory_operand() {
    // SARX EAX, [addr], ECX
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SARX EAX, [0x2000], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x80000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xF8000000, "EAX should sign extend");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_shlx_32bit_with_r8d() {
    // SHLX R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x29, 0xf7, 0xc1, // SHLX R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 1;
    regs.r10 = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 1 << 20, "R8D should be shifted");
}

#[test]
fn test_shrx_64bit_with_r11() {
    // SHRX R11, R12, R13
    let code = [
        0xc4, 0x42, 0x93, 0xf7, 0xdc, // SHRX R11, R12, R13
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x8000000000000000;
    regs.r13 = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0x0800000000000000, "R11 should be shifted");
}

#[test]
fn test_sarx_32bit_with_r15d() {
    // SARX R14D, R15D, EBX
    let code = [
        0xc4, 0x42, 0x62, 0xf7, 0xf7, // SARX R14D, R15D, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x80000000;
    regs.rbx = 8;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14 & 0xFFFFFFFF, 0xFF800000, "R14D should sign extend");
}

// ============================================================================
// Edge Cases and Corner Cases
// ============================================================================

#[test]
fn test_shlx_32bit_all_bits_shift_out() {
    // SHLX EAX, EBX, ECX - shift all bits out
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 31;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX should have only high bit"
    );
}

#[test]
fn test_shrx_64bit_all_bits_shift_out() {
    // SHRX RAX, RBX, RCX - shift almost all bits out
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000;
    regs.rcx = 63;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_sarx_64bit_negative_small_shift() {
    // SARX RAX, RBX, RCX - small shift on negative
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFF0u64;
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFC, "RAX should sign extend");
}

#[test]
fn test_shlx_64bit_alternating_pattern() {
    // SHLX RAX, RBX, RCX - shift alternating pattern
    let code = [
        0xc4, 0xe2, 0xf1, 0xf7, 0xc3, // SHLX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x5555555555555555;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xAAAAAAAAAAAAAAAA,
        "RAX should be shifted pattern"
    );
}

#[test]
fn test_shrx_32bit_alternating_pattern() {
    // SHRX EAX, EBX, ECX - shift alternating pattern
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "EAX should be shifted pattern"
    );
}

#[test]
fn test_comparison_shrx_vs_sarx_positive() {
    // Compare SHRX and SARX on positive number (should be same)
    let shrx_code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let sarx_code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];

    let mut regs1 = Registers::default();
    regs1.rbx = 0x7FFFFFFF;
    regs1.rcx = 4;
    let (mut vcpu1, _) = setup_vm(&shrx_code, Some(regs1));
    let regs1 = run_until_hlt(&mut vcpu1).unwrap();

    let mut regs2 = Registers::default();
    regs2.rbx = 0x7FFFFFFF;
    regs2.rcx = 4;
    let (mut vcpu2, _) = setup_vm(&sarx_code, Some(regs2));
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(
        regs1.rax & 0xFFFFFFFF,
        regs2.rax & 0xFFFFFFFF,
        "SHRX and SARX should be same for positive"
    );
}

#[test]
fn test_comparison_shrx_vs_sarx_negative() {
    // Compare SHRX and SARX on negative number (should be different)
    let shrx_code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    let sarx_code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];

    let mut regs1 = Registers::default();
    regs1.rbx = 0x80000000;
    regs1.rcx = 4;
    let (mut vcpu1, _) = setup_vm(&shrx_code, Some(regs1));
    let regs1 = run_until_hlt(&mut vcpu1).unwrap();

    let mut regs2 = Registers::default();
    regs2.rbx = 0x80000000;
    regs2.rcx = 4;
    let (mut vcpu2, _) = setup_vm(&sarx_code, Some(regs2));
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_ne!(
        regs1.rax & 0xFFFFFFFF,
        regs2.rax & 0xFFFFFFFF,
        "SHRX and SARX should differ for negative"
    );
    assert_eq!(
        regs1.rax & 0xFFFFFFFF,
        0x08000000,
        "SHRX should not sign extend"
    );
    assert_eq!(
        regs2.rax & 0xFFFFFFFF,
        0xF8000000,
        "SARX should sign extend"
    );
}

#[test]
fn test_shlx_32bit_count_63() {
    // SHLX EAX, EBX, ECX - count of 63 (should mask to 31)
    let code = [
        0xc4, 0xe2, 0x71, 0xf7, 0xc3, // SHLX EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 2;
    regs.rcx = 63; // Masks to 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be 0 (2 << 31 in 32-bit wraps)"
    );
}

#[test]
fn test_shrx_64bit_count_127() {
    // SHRX RAX, RBX, RCX - count of 127 (should mask to 63)
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x8000000000000000;
    regs.rcx = 127; // Masks to 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1, "RAX should be 1");
}

#[test]
fn test_sarx_32bit_multiple_operations() {
    // Multiple SARX operations to verify state
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0x48, 0x89, 0xc3, // mov rbx, rax
        0xc4, 0xe2, 0x6a, 0xf7, 0xc3, // SARX EAX, EBX, EDX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80000000;
    regs.rcx = 1;
    regs.rdx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 0x80000000 >> 1 = 0xC0000000
    // Second: 0xC0000000 >> 1 = 0xE0000000
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xE0000000,
        "EAX should be double shifted"
    );
}
