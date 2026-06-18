// SHLD (Double Precision Shift Left) instruction tests
//
// Opcodes:
// 0F A4 /r ib      SHLD r/m16, r16, imm8
// 0F A5 /r         SHLD r/m16, r16, CL
// 0F A4 /r ib      SHLD r/m32, r32, imm8
// 0F A5 /r         SHLD r/m32, r32, CL
// REX.W + 0F A4 /r ib  SHLD r/m64, r64, imm8
// REX.W + 0F A5 /r     SHLD r/m64, r64, CL
//
// SHLD shifts the destination operand left by count bits.
// Bits shifted in from the right come from the source operand.
// Used for multi-precision shifts of 64 bits or more.
//
// Flags:
// - CF: Last bit shifted out of destination
// - OF: Only for 1-bit shifts (sign change)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 16-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_ax_bx_imm8() {
    // SHLD AX, BX, imm8 (opcode 66 0F A4 /r ib)
    let code = [
        0x66, 0x0f, 0xa4, 0xd8, 0x04, // SHLD AX, BX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234; // Destination
    regs.rbx = 0xABCD; // Source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX: 0001_0010_0011_0100 shifted left by 4
    // Bits from BX (1010_1011_1100_1101) fill from right
    // Result: 0010_0011_0100_1010
    assert_eq!(
        regs.rax & 0xFFFF,
        0x234A,
        "AX: 0x1234 SHLD 4 from 0xABCD = 0x234A"
    );
    assert!(cf_set(regs.rflags), "CF: bit shifted out was 1");
}

#[test]
fn test_shld_ax_bx_cl() {
    // SHLD AX, BX, CL (opcode 66 0F A5 /r)
    let code = [
        0x66, 0x0f, 0xa5, 0xd8, // SHLD AX, BX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0xABCD;
    regs.rcx = 0x08; // Shift by 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift AX left by 8, fill with 8 bits from BX
    // AX high byte = 0x34, fill with BX high byte = 0xAB
    assert_eq!(
        regs.rax & 0xFFFF,
        0x34AB,
        "AX: 0x1234 SHLD 8 from 0xABCD = 0x34AB"
    );
}

#[test]
fn test_shld_ax_bx_1bit() {
    // SHLD with 1-bit shift (tests OF flag)
    let code = [
        0x66, 0x0f, 0xa4, 0xd8, 0x01, // SHLD AX, BX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4000; // 0100_0000_0000_0000
    regs.rbx = 0x0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: 0x4000 SHLD 1 = 0x8000");
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
    assert!(of_set(regs.rflags), "OF: sign changed from + to -");
}

#[test]
fn test_shld_ax_full_rotation() {
    // SHLD by 16 should completely replace dest with source
    let code = [
        0x66, 0x0f, 0xa4, 0xd8, 0x10, // SHLD AX, BX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xABCD, "AX: completely replaced by BX");
}

// ============================================================================
// 32-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_eax_ebx_imm8() {
    // SHLD EAX, EBX, imm8 (opcode 0F A4 /r ib)
    let code = [
        0x0f, 0xa4, 0xd8, 0x04, // SHLD EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift EAX left by 4, fill with high 4 bits from EBX
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2345678A,
        "EAX: 0x12345678 SHLD 4 from 0xABCDEF01"
    );
}

#[test]
fn test_shld_eax_ebx_cl() {
    // SHLD EAX, EBX, CL (opcode 0F A5 /r)
    let code = [
        0x0f, 0xa5, 0xd8, // SHLD EAX, EBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x345678AB,
        "EAX: 0x12345678 SHLD 8 from 0xABCDEF01"
    );
}

#[test]
fn test_shld_eax_carry_flag() {
    // Test CF with bit shifted out
    let code = [
        0x0f, 0xa4, 0xd8, 0x04, // SHLD EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // MSB set
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX: shifted out");
    assert!(!cf_set(regs.rflags), "CF: bit 28 (4 bits from MSB) was 0");
}

#[test]
fn test_shld_eax_count_masked() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0x0f, 0xa5, 0xd8, // SHLD EAX, EBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    regs.rcx = 0x28; // 40 & 0x1F = 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x345678AB, "EAX: count masked to 8");
}

#[test]
fn test_shld_eax_count_zero() {
    // Count of 0 should not affect operands or flags
    let code = [
        0x0f, 0xa4, 0xd8, 0x00, // SHLD EAX, EBX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    regs.rflags = 0x2 | flags::bits::CF | flags::bits::OF;
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX: unchanged");
    assert_eq!(regs.rflags, initial_flags, "Flags: unchanged");
}

// ============================================================================
// 64-bit SHLD tests
// ============================================================================

#[test]
fn test_shld_rax_rbx_imm8() {
    // SHLD RAX, RBX, imm8 (opcode 48 0F A4 /r ib)
    let code = [
        0x48, 0x0f, 0xa4, 0xd8, 0x04, // SHLD RAX, RBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x23456789ABCDEF0F, "RAX: SHLD 4 from RBX");
}

#[test]
fn test_shld_rax_rbx_cl() {
    // SHLD RAX, RBX, CL (opcode 48 0F A5 /r)
    let code = [
        0x48, 0x0f, 0xa5, 0xd8, // SHLD RAX, RBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x10; // Shift by 16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x56789ABCDEF0FEDC, "RAX: SHLD 16 from RBX");
}

#[test]
fn test_shld_rax_count_masked_64bit() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0x48, 0x0f, 0xa5, 0xd8, // SHLD RAX, RBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x50; // 80 & 0x3F = 16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x56789ABCDEF0FEDC, "RAX: count masked to 16");
}

#[test]
fn test_shld_rax_full_width() {
    // Count is masked to 6 bits for 64-bit operands (64 becomes 0)
    let code = [
        0x48, 0x0f, 0xa5, 0xd8, // SHLD RAX, RBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x40; // Shift by 64
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX: unchanged when count masks to 0"
    );
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_shld_r8w_r9w_imm8() {
    // SHLD R8W, R9W, imm8
    let code = [
        0x66, 0x45, 0x0f, 0xa4, 0xc8, 0x04, // SHLD R8W, R9W, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x1234;
    regs.r9 = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFF, 0x234A, "R8W: SHLD from R9W");
}

#[test]
fn test_shld_r10d_r11d_cl() {
    // SHLD R10D, R11D, CL
    let code = [
        0x45, 0x0f, 0xa5, 0xda, // SHLD R10D, R11D, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    regs.r11 = 0xABCDEF01;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFFFFFF, 0x345678AB, "R10D: SHLD from R11D");
}

#[test]
fn test_shld_r14_r15_imm8() {
    // SHLD R14, R15, imm8
    let code = [
        0x4d, 0x0f, 0xa4, 0xfe, 0x10, // SHLD R14, R15, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x123456789ABCDEF0;
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0x56789ABCDEF0FEDC, "R14: SHLD from R15");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shld_word_ptr_imm8() {
    // SHLD word ptr [DATA_ADDR], BX, imm8
    let code = [
        0x66,
        0x0f,
        0xa4,
        0x1c,
        0x25, // SHLD word ptr [DATA_ADDR], BX, imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8 = 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xABCD;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x1234);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x234A, "Memory: 0x1234 SHLD 4 from 0xABCD");
}

#[test]
fn test_shld_dword_ptr_cl() {
    // SHLD dword ptr [DATA_ADDR], EBX, CL
    let code = [
        0x0f,
        0xa5,
        0x1c,
        0x25, // SHLD dword ptr [DATA_ADDR], EBX, CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xABCDEF01;
    regs.rcx = 0x08;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x12345678);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(
        result, 0x345678AB,
        "Memory: 0x12345678 SHLD 8 from 0xABCDEF01"
    );
}

#[test]
fn test_shld_qword_ptr_imm8() {
    // SHLD qword ptr [DATA_ADDR], RBX, imm8
    let code = [
        0x48,
        0x0f,
        0xa4,
        0x1c,
        0x25, // SHLD qword ptr [DATA_ADDR], RBX, imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFEDCBA9876543210;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x56789ABCDEF0FEDC, "Memory: SHLD 16 from RBX");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shld_multi_precision_shift() {
    // SHLD is used for multi-precision left shifts
    // Shift a 128-bit value left by 4 bits using two 64-bit operations
    let code = [
        // First shift low 64 bits, save what's shifted out
        0x48, 0xc1, 0xe0, 0x04, // SHL RAX, 4 (low 64 bits)
        // Then shift high 64 bits and fill with bits from low
        0x48, 0x0f, 0xa4, 0xc3, 0x04, // SHLD RBX, RAX, 4 (high 64 bits)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFEDCBA9876543210; // Low 64 bits (before shift)
    regs.rbx = 0x123456789ABCDEF0; // High 64 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After SHL RAX, 4: RAX = 0xEDCBA98765432100
    // After SHLD RBX, RAX, 4: RBX gets top 4 bits of shifted RAX
    // But SHLD uses original RAX value (before it was shifted)
    // Actually, SHLD happens after SHL, so it uses the shifted RAX
    assert_eq!(regs.rax, 0xEDCBA98765432100, "RAX: low 64 bits shifted");
    // RBX: 0x123456789ABCDEF0 << 4 with high 4 bits of 0xEDCBA98765432100
    assert_eq!(
        regs.rbx, 0x23456789ABCDEF0E,
        "RBX: high 64 bits with fill from RAX"
    );
}

#[test]
fn test_shld_extract_bits() {
    // SHLD can extract specific bit ranges
    let code = [
        0x0f, 0xa4, 0xd8, 0x10, // SHLD EAX, EBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 0xABCD0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000ABCD,
        "EAX: extracted high 16 bits from EBX"
    );
}

#[test]
fn test_shld_flag_behavior() {
    // Test all relevant flags
    let code = [
        0x0f, 0xa4, 0xd8, 0x01, // SHLD EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // MSB set
    regs.rbx = 0x00000001; // LSB set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: 0x80000000 SHLD 1 = 0x00000000"
    );
    assert!(cf_set(regs.rflags), "CF: bit shifted out was 1");
    // OF: sign changed from negative to positive
    assert!(of_set(regs.rflags), "OF: sign changed");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
    assert!(zf_set(regs.rflags), "ZF: result is zero");
}

#[test]
fn test_shld_concatenate_values() {
    // SHLD can concatenate parts of two values
    let code = [
        0x0f, 0xa4, 0xd8, 0x08, // SHLD EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF000000;
    regs.rbx = 0x00000055;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: concatenated result"
    );
}

#[test]
fn test_shld_max_shift() {
    // Maximum meaningful shift (operand size)
    let code = [
        0x0f, 0xa4, 0xd8, 0x1F, // SHLD EAX, EBX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift left by 31, only LSB of result comes from original EAX
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x7FFFFFFF, "EAX: SHLD by 31");
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
}
