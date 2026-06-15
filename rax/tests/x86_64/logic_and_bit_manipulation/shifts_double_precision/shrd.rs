// SHRD (Double Precision Shift Right) instruction tests
//
// Opcodes:
// 0F AC /r ib      SHRD r/m16, r16, imm8
// 0F AD /r         SHRD r/m16, r16, CL
// 0F AC /r ib      SHRD r/m32, r32, imm8
// 0F AD /r         SHRD r/m32, r32, CL
// REX.W + 0F AC /r ib  SHRD r/m64, r64, imm8
// REX.W + 0F AD /r     SHRD r/m64, r64, CL
//
// SHRD shifts the destination operand right by count bits.
// Bits shifted in from the left come from the source operand.
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
// 16-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_ax_bx_imm8() {
    // SHRD AX, BX, imm8 (opcode 66 0F AC /r ib)
    let code = [
        0x66, 0x0f, 0xac, 0xd8, 0x04, // SHRD AX, BX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234; // Destination
    regs.rbx = 0xABCD; // Source
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AX: 0001_0010_0011_0100 shifted right by 4
    // Bits from BX (1010_1011_1100_1101) fill from left
    // Result: 1101_0001_0010_0011
    assert_eq!(
        regs.rax & 0xFFFF,
        0xD123,
        "AX: 0x1234 SHRD 4 from 0xABCD = 0xD123"
    );
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
}

#[test]
fn test_shrd_ax_bx_cl() {
    // SHRD AX, BX, CL (opcode 66 0F AD /r)
    let code = [
        0x66, 0x0f, 0xad, 0xd8, // SHRD AX, BX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0xABCD;
    regs.rcx = 0x08; // Shift by 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift AX right by 8, fill with 8 bits from BX
    // AX low byte = 0x12, fill with BX low byte = 0xCD
    assert_eq!(
        regs.rax & 0xFFFF,
        0xCD12,
        "AX: 0x1234 SHRD 8 from 0xABCD = 0xCD12"
    );
}

#[test]
fn test_shrd_ax_bx_1bit() {
    // SHRD with 1-bit shift (tests OF flag)
    let code = [
        0x66, 0x0f, 0xac, 0xd8, 0x01, // SHRD AX, BX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0002; // 0000_0000_0000_0010
    regs.rbx = 0x8000; // Source with MSB set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX: 0x0002 SHRD 1 = 0x0001");
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
    assert!(!of_set(regs.rflags), "OF: no sign change");
}

#[test]
fn test_shrd_ax_full_replacement() {
    // SHRD by 16 should completely replace dest with source
    let code = [
        0x66, 0x0f, 0xac, 0xd8, 0x10, // SHRD AX, BX, 16
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
// 32-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_eax_ebx_imm8() {
    // SHRD EAX, EBX, imm8 (opcode 0F AC /r ib)
    let code = [
        0x0f, 0xac, 0xd8, 0x04, // SHRD EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift EAX right by 4, fill with low 4 bits from EBX
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x11234567,
        "EAX: 0x12345678 SHRD 4 from 0xABCDEF01"
    );
}

#[test]
fn test_shrd_eax_ebx_cl() {
    // SHRD EAX, EBX, CL (opcode 0F AD /r)
    let code = [
        0x0f, 0xad, 0xd8, // SHRD EAX, EBX, CL
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
        0x01123456,
        "EAX: 0x12345678 SHRD 8 from 0xABCDEF01"
    );
}

#[test]
fn test_shrd_eax_carry_flag() {
    // Test CF with bit shifted out
    let code = [
        0x0f, 0xac, 0xd8, 0x04, // SHRD EAX, EBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000F; // Low 4 bits set
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX: shifted out");
    assert!(cf_set(regs.rflags), "CF: bit 3 (4th bit from LSB) was 1");
}

#[test]
fn test_shrd_eax_count_masked() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0x0f, 0xad, 0xd8, // SHRD EAX, EBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xABCDEF01;
    regs.rcx = 0x28; // 40 & 0x1F = 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x01123456, "EAX: count masked to 8");
}

#[test]
fn test_shrd_eax_count_zero() {
    // Count of 0 should not affect operands or flags
    let code = [
        0x0f, 0xac, 0xd8, 0x00, // SHRD EAX, EBX, 0
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
// 64-bit SHRD tests
// ============================================================================

#[test]
fn test_shrd_rax_rbx_imm8() {
    // SHRD RAX, RBX, imm8 (opcode 48 0F AC /r ib)
    let code = [
        0x48, 0x0f, 0xac, 0xd8, 0x04, // SHRD RAX, RBX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0123456789ABCDEF, "RAX: SHRD 4 from RBX");
}

#[test]
fn test_shrd_rax_rbx_cl() {
    // SHRD RAX, RBX, CL (opcode 48 0F AD /r)
    let code = [
        0x48, 0x0f, 0xad, 0xd8, // SHRD RAX, RBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x10; // Shift by 16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3210123456789ABC, "RAX: SHRD 16 from RBX");
}

#[test]
fn test_shrd_rax_count_masked_64bit() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0x48, 0x0f, 0xad, 0xd8, // SHRD RAX, RBX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x50; // 80 & 0x3F = 16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3210123456789ABC, "RAX: count masked to 16");
}

#[test]
fn test_shrd_rax_full_width() {
    // Count is masked to 6 bits for 64-bit operands (64 becomes 0)
    let code = [
        0x48, 0x0f, 0xad, 0xd8, // SHRD RAX, RBX, CL
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
fn test_shrd_r8w_r9w_imm8() {
    // SHRD R8W, R9W, imm8
    let code = [
        0x66, 0x45, 0x0f, 0xac, 0xc8, 0x04, // SHRD R8W, R9W, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x1234;
    regs.r9 = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFFFF, 0xD123, "R8W: SHRD from R9W");
}

#[test]
fn test_shrd_r10d_r11d_cl() {
    // SHRD R10D, R11D, CL
    let code = [
        0x45, 0x0f, 0xad, 0xda, // SHRD R10D, R11D, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    regs.r11 = 0xABCDEF01;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFFFFFF, 0x01123456, "R10D: SHRD from R11D");
}

#[test]
fn test_shrd_r14_r15_imm8() {
    // SHRD R14, R15, imm8
    let code = [
        0x4d, 0x0f, 0xac, 0xfe, 0x10, // SHRD R14, R15, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x123456789ABCDEF0;
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0x3210123456789ABC, "R14: SHRD from R15");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shrd_word_ptr_imm8() {
    // SHRD word ptr [DATA_ADDR], BX, imm8
    let code = [
        0x66,
        0x0f,
        0xac,
        0x1c,
        0x25, // SHRD word ptr [DATA_ADDR], BX, imm8
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

    assert_eq!(result, 0xD123, "Memory: 0x1234 SHRD 4 from 0xABCD");
}

#[test]
fn test_shrd_dword_ptr_cl() {
    // SHRD dword ptr [DATA_ADDR], EBX, CL
    let code = [
        0x0f,
        0xad,
        0x1c,
        0x25, // SHRD dword ptr [DATA_ADDR], EBX, CL
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
        result, 0x01123456,
        "Memory: 0x12345678 SHRD 8 from 0xABCDEF01"
    );
}

#[test]
fn test_shrd_qword_ptr_imm8() {
    // SHRD qword ptr [DATA_ADDR], RBX, imm8
    let code = [
        0x48,
        0x0f,
        0xac,
        0x1c,
        0x25, // SHRD qword ptr [DATA_ADDR], RBX, imm8
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

    assert_eq!(result, 0x3210123456789ABC, "Memory: SHRD 16 from RBX");
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shrd_multi_precision_shift() {
    // SHRD is used for multi-precision right shifts
    // Shift a 128-bit value right by 4 bits using two 64-bit operations
    let code = [
        // First shift high 64 bits and fill with bits from low
        0x48, 0x0f, 0xac, 0xc3, 0x04, // SHRD RBX, RAX, 4 (high 64 bits)
        // Then shift low 64 bits
        0x48, 0xc1, 0xe8, 0x04, // SHR RAX, 4 (low 64 bits)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0; // High 64 bits (in multi-precision context)
    regs.rbx = 0xFEDCBA9876543210; // Low 64 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After SHRD RBX, RAX, 4: RBX gets low 4 bits of RAX
    // RAX: 0x123456789ABCDEF0, low 4 bits = 0x0
    // RBX: 0xFEDCBA9876543210 >> 4 with 0x0 from top = 0x0FEDCBA987654321
    assert_eq!(
        regs.rbx, 0x0FEDCBA987654321,
        "RBX: low 64 bits with fill from RAX"
    );
    assert_eq!(regs.rax, 0x0123456789ABCDEF, "RAX: high 64 bits shifted");
}

#[test]
fn test_shrd_extract_bits() {
    // SHRD can extract specific bit ranges
    let code = [
        0x0f, 0xac, 0xd8, 0x10, // SHRD EAX, EBX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 0x0000ABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xABCD0000,
        "EAX: extracted low 16 bits from EBX"
    );
}

#[test]
fn test_shrd_flag_behavior() {
    // Test all relevant flags
    let code = [
        0x0f, 0xac, 0xd8, 0x01, // SHRD EAX, EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001; // LSB set
    regs.rbx = 0x80000000; // MSB set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: 0x00000001 SHRD 1 = 0x00000000"
    );
    assert!(cf_set(regs.rflags), "CF: bit shifted out was 1");
    // OF: sign changed from positive to negative? No, result is still positive
    assert!(!of_set(regs.rflags), "OF: no sign change");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
    assert!(zf_set(regs.rflags), "ZF: result is zero");
}

#[test]
fn test_shrd_concatenate_values() {
    // SHRD can concatenate parts of two values
    let code = [
        0x0f, 0xac, 0xd8, 0x08, // SHRD EAX, EBX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x000000FF;
    regs.rbx = 0x55000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: concatenated result"
    );
}

#[test]
fn test_shrd_max_shift() {
    // Maximum meaningful shift (operand size)
    let code = [
        0x0f, 0xac, 0xd8, 0x1F, // SHRD EAX, EBX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Shift right by 31, only MSB of result comes from original EAX
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFE, "EAX: SHRD by 31");
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
}

#[test]
fn test_shrd_reverse_bytes() {
    // SHRD combined with other operations can manipulate byte order
    let code = [
        0x0f, 0xac, 0xd8, 0x18, // SHRD EAX, EBX, 24
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12000000;
    regs.rbx = 0x00345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x34567812, "EAX: byte manipulation");
}
