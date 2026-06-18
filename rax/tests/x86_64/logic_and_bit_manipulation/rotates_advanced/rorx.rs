// RORX (Rotate Right Logical Without Affecting Flags) instruction tests
//
// Opcodes:
// VEX.LZ.F2.0F3A.W0 F0 /r ib    RORX r32, r/m32, imm8
// VEX.LZ.F2.0F3A.W1 F0 /r ib    RORX r64, r/m64, imm8
//
// RORX rotates the source operand right by imm8 bits without affecting flags.
// Unlike ROR, RORX:
// - Has separate destination and source operands
// - Does NOT affect any flags
// - Only accepts immediate count (no CL variant)
// - Requires BMI2 CPU feature
//
// Flags:
// - None affected (unlike ROR which sets CF and OF)

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::{Registers, VCpu};

// ============================================================================
// 32-bit RORX tests
// ============================================================================

#[test]
fn test_rorx_eax_ebx_imm8() {
    // RORX EAX, EBX, imm8 (VEX.LZ.F2.0F3A.W0 F0 /r ib)
    // VEX encoding: C4 E3 7B F0 C3 04
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
        "EAX: 0x12345678 RORX 4 = 0x81234567"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX: source unchanged");
}

#[test]
fn test_rorx_eax_no_flags() {
    // RORX should not affect any flags
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    // Set all flags
    regs.rflags = 0x2
        | flags::bits::CF
        | flags::bits::PF
        | flags::bits::AF
        | flags::bits::ZF
        | flags::bits::SF
        | flags::bits::OF;
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags, initial_flags,
        "Flags: RORX does not affect flags"
    );
}

#[test]
fn test_rorx_eax_rotate_8() {
    // Rotate by 8 bits
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
        "EAX: 0x12345678 RORX 8 = 0x78123456"
    );
}

#[test]
fn test_rorx_eax_rotate_1() {
    // Rotate by 1 bit
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
        "EAX: 0x12345678 RORX 1 = 0x091A2B3C"
    );
}

#[test]
fn test_rorx_eax_full_rotation() {
    // Rotate by 32 bits should return to original
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x20, // RORX EAX, EBX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX: full rotation returns to original"
    );
}

#[test]
fn test_rorx_eax_count_masked() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x28, // RORX EAX, EBX, 0x28 (40)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x28 & 0x1F = 8
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX: count masked to 8");
}

// ============================================================================
// 64-bit RORX tests
// ============================================================================

#[test]
fn test_rorx_rax_rbx_imm8() {
    // RORX RAX, RBX, imm8 (VEX.LZ.F2.0F3A.W1 F0 /r ib)
    // VEX encoding: C4 E3 FB F0 C3 04
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
        "RAX: 0x123456789ABCDEF0 RORX 4"
    );
    assert_eq!(regs.rbx, 0x123456789ABCDEF0, "RBX: source unchanged");
}

#[test]
fn test_rorx_rax_no_flags() {
    // RORX should not affect any flags (64-bit)
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rflags = 0x2 | flags::bits::CF | flags::bits::ZF | flags::bits::OF;
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags, initial_flags,
        "Flags: RORX does not affect flags"
    );
}

#[test]
fn test_rorx_rax_rotate_16() {
    // Rotate by 16 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x10, // RORX RAX, RBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xDEF0123456789ABC, "RAX: RORX 16");
}

#[test]
fn test_rorx_rax_rotate_32() {
    // Rotate by 32 bits
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x20, // RORX RAX, RBX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x9ABCDEF012345678, "RAX: RORX 32");
}

#[test]
fn test_rorx_rax_full_rotation() {
    // Rotate by 64 bits should return to original
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x40, // RORX RAX, RBX, 64
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX: full rotation returns to original"
    );
}

#[test]
fn test_rorx_rax_count_masked() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0xc4, 0xe3, 0xfb, 0xf0, 0xc3, 0x50, // RORX RAX, RBX, 0x50 (80)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x50 & 0x3F = 16
    assert_eq!(regs.rax, 0xDEF0123456789ABC, "RAX: count masked to 16");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rorx_r8d_r9d_imm8() {
    // RORX R8D, R9D, imm8
    let code = [
        0xc4, 0x43, 0x7b, 0xf0, 0xc1, 0x08, // RORX R8D, R9D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFFFFFF, 0x78123456, "R8D: RORX from R9D");
}

#[test]
fn test_rorx_r14_r15_imm8() {
    // RORX R14, R15, imm8
    let code = [
        0xc4, 0x43, 0xfb, 0xf0, 0xf7, 0x10, // RORX R14, R15, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0xDEF0123456789ABC, "R14: RORX from R15");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rorx_eax_dword_ptr() {
    // RORX EAX, dword ptr [DATA_ADDR], imm8
    let code = [
        0xc4,
        0xe3,
        0x7b,
        0xf0,
        0x04,
        0x25, // RORX EAX, dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);
    let regs = vcpu.get_regs().unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX: RORX from memory");
    assert_eq!(result, 0x12345678, "Memory: unchanged");
}

#[test]
fn test_rorx_rax_qword_ptr() {
    // RORX RAX, qword ptr [DATA_ADDR], imm8
    let code = [
        0xc4,
        0xe3,
        0xfb,
        0xf0,
        0x04,
        0x25, // RORX RAX, qword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);
    let regs = vcpu.get_regs().unwrap();

    assert_eq!(regs.rax, 0xDEF0123456789ABC, "RAX: RORX from memory");
    assert_eq!(result, 0x123456789ABCDEF0, "Memory: unchanged");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rorx_separate_dest_source() {
    // RORX can use different registers for dest and source
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF; // Will be overwritten
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x81234567, "EAX: new rotated value");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX: preserved");
}

#[test]
fn test_rorx_vs_ror_flags() {
    // Compare RORX (no flags) vs ROR (sets flags)
    // First do ROR
    let code_ror = [
        0xc1, 0xc8, 0x04, // ROR EAX, 4
        0xf4,
    ];
    let mut regs_ror = Registers::default();
    regs_ror.rax = 0x12345678;
    regs_ror.rflags = 0x2; // Clear all flags
    let (mut vcpu_ror, _) = setup_vm(&code_ror, Some(regs_ror));
    let regs_ror = run_until_hlt(&mut vcpu_ror).unwrap();

    // ROR should set CF
    let ror_cf = cf_set(regs_ror.rflags);

    // Now do RORX
    let code_rorx = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut regs_rorx = Registers::default();
    regs_rorx.rbx = 0x12345678;
    regs_rorx.rflags = 0x2 | flags::bits::CF; // Set CF
    let (mut vcpu_rorx, _) = setup_vm(&code_rorx, Some(regs_rorx));
    let regs_rorx = run_until_hlt(&mut vcpu_rorx).unwrap();

    // RORX should not change CF
    assert_eq!(regs_rorx.rax & 0xFFFFFFFF, 0x81234567, "RORX result");
    assert!(cf_set(regs_rorx.rflags), "RORX: CF preserved");
}

#[test]
fn test_rorx_byte_swap() {
    // Use RORX for byte manipulation
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x08, // RORX EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX: bytes rotated");
}

#[test]
fn test_rorx_all_ones() {
    // Rotate all ones
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: all ones stay all ones"
    );
}

#[test]
fn test_rorx_alternating_bits() {
    // Test with alternating bit pattern
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x01, // RORX EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010_1010...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "EAX: alternating bits rotated"
    );
}

#[test]
fn test_rorx_nibble_swap() {
    // Swap nibbles within bytes
    let code = [
        0xc4, 0xe3, 0x7b, 0xf0, 0xc3, 0x04, // RORX EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x81234567, "EAX: nibbles rotated");
}
