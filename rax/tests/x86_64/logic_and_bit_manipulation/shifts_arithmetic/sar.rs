// SAR (Shift Arithmetic Right) instruction tests
//
// Opcodes:
// D0 /7       SAR r/m8, 1
// D2 /7       SAR r/m8, CL
// C0 /7 ib    SAR r/m8, imm8
// D1 /7       SAR r/m16, 1
// D3 /7       SAR r/m16, CL
// C1 /7 ib    SAR r/m16, imm8
// D1 /7       SAR r/m32, 1
// D3 /7       SAR r/m32, CL
// C1 /7 ib    SAR r/m32, imm8
// REX.W + D1 /7    SAR r/m64, 1
// REX.W + D3 /7    SAR r/m64, CL
// REX.W + C1 /7 ib SAR r/m64, imm8
//
// SAR performs signed division by powers of 2
// Fills empty bit positions with the sign bit (MSB)
// Rounding is toward negative infinity (not the same as IDIV)
//
// Flags:
// - CF: Last bit shifted out
// - OF: Cleared for all 1-bit shifts
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

use crate::common::*;

// ============================================================================
// 8-bit SAR tests
// ============================================================================

#[test]
fn test_sar_al_1_positive() {
    // SAR AL, 1 with positive number (opcode D0 /7)
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010 (positive)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x21,
        "AL: 0x42 >> 1 = 0x21 (sign extended)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB shifted out was 0");
    assert!(!of_set(regs.rflags), "OF: always cleared for 1-bit SAR");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
    assert!(!zf_set(regs.rflags), "ZF: result is not zero");
}

#[test]
fn test_sar_al_1_negative() {
    // SAR AL, 1 with negative number (sign bit set)
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x82; // 1000_0010 (negative in signed interpretation)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xC1,
        "AL: 0x82 >> 1 = 0xC1 (sign bit extended)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB shifted out was 0");
    assert!(!of_set(regs.rflags), "OF: always cleared for 1-bit SAR");
    assert!(sf_set(regs.rflags), "SF: result is still negative");
}

#[test]
fn test_sar_al_1_with_carry() {
    // SAR with LSB set
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x43; // 0100_0011
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x43 >> 1 = 0x21");
    assert!(cf_set(regs.rflags), "CF: LSB shifted out was 1");
    assert!(!of_set(regs.rflags), "OF: cleared for 1-bit shifts");
}

#[test]
fn test_sar_al_cl() {
    // SAR AL, CL (opcode D2 /7)
    let code = [
        0xd2, 0xf8, // SAR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 1000_0000 (most negative 8-bit value)
    regs.rcx = 0x07; // Shift by 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xFF,
        "AL: 0x80 >> 7 = 0xFF (all bits set by sign extension)"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_al_imm8() {
    // SAR AL, imm8 (opcode C0 /7 ib)
    let code = [
        0xc0, 0xf8, 0x03, // SAR AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88; // 1000_1000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xF1,
        "AL: 0x88 >> 3 = 0xF1 (sign extended)"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_signed_division_by_2() {
    // SAR divides by 2 (signed)
    // -8 / 2 = -4
    let code = [
        0xd0, 0xf8, // SAR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF8; // -8 in 8-bit two's complement
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFC, "AL: -8 / 2 = -4 (0xFC)");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_sar_count_masked_8bit() {
    // Count is masked to 5 bits
    let code = [
        0xd2, 0xf8, // SAR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88;
    regs.rcx = 0x23; // 35 & 0x1F = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xF1, "AL: 0x88 >> 3 = 0xF1 (count masked)");
}

#[test]
fn test_sar_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xf8, 0x00, // SAR AL, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rflags = 0x2 | flags::bits::CF | flags::bits::ZF | flags::bits::OF;
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(
        regs.rflags, initial_flags,
        "Flags unchanged when count is 0"
    );
}

// ============================================================================
// 16-bit SAR tests
// ============================================================================

#[test]
fn test_sar_ax_1_positive() {
    // SAR AX, 1 with positive number (opcode 66 D1 /7)
    let code = [
        0x66, 0xd1, 0xf8, // SAR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x2190, "AX: 0x4321 >> 1 = 0x2190");
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
    assert!(!of_set(regs.rflags), "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_ax_1_negative() {
    // SAR AX, 1 with negative number
    let code = [
        0x66, 0xd1, 0xf8, // SAR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // Most negative 16-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0xC000,
        "AX: 0x8000 >> 1 = 0xC000 (sign extended)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_ax_cl() {
    // SAR AX, CL (opcode 66 D3 /7)
    let code = [
        0x66, 0xd3, 0xf8, // SAR AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF00; // Negative
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0xFFFF,
        "AX: 0xFF00 >> 8 = 0xFFFF (sign extended)"
    );
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_ax_imm8() {
    // SAR AX, imm8 (opcode 66 C1 /7 ib)
    let code = [
        0x66, 0xc1, 0xf8, 0x04, // SAR AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1230;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0123, "AX: 0x1230 >> 4 = 0x0123");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
}

// ============================================================================
// 32-bit SAR tests
// ============================================================================

#[test]
fn test_sar_eax_1_positive() {
    // SAR EAX, 1 with positive number (opcode D1 /7)
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x091A2B3C,
        "EAX: 0x12345678 >> 1 = 0x091A2B3C"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(!of_set(regs.rflags), "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_eax_1_negative() {
    // SAR EAX, 1 with negative number
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // Most negative 32-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xC0000000,
        "EAX: 0x80000000 >> 1 = 0xC0000000"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(sf_set(regs.rflags), "SF: result is still negative");
}

#[test]
fn test_sar_eax_cl() {
    // SAR EAX, CL (opcode D3 /7)
    let code = [
        0xd3, 0xf8, // SAR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF0000; // Negative
    regs.rcx = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: 0xFFFF0000 >> 16 = 0xFFFFFFFF"
    );
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_eax_imm8() {
    // SAR EAX, imm8 (opcode C1 /7 ib)
    let code = [
        0xc1, 0xf8, 0x08, // SAR EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345600;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00123456,
        "EAX: 0x12345600 >> 8 = 0x00123456"
    );
    assert!(!sf_set(regs.rflags), "SF: result is positive");
}

#[test]
fn test_sar_count_masked_32bit() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0xd3, 0xf8, // SAR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rcx = 0x3F; // 63 & 0x1F = 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: 0x80000000 >> 31 = 0xFFFFFFFF (all ones)"
    );
}

// ============================================================================
// 64-bit SAR tests
// ============================================================================

#[test]
fn test_sar_rax_1_positive() {
    // SAR RAX, 1 with positive number (opcode 48 D1 /7)
    let code = [
        0x48, 0xd1, 0xf8, // SAR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x091A2B3C4D5E6F78, "RAX: 0x123456789ABCDEF0 >> 1");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(!of_set(regs.rflags), "OF: cleared for 1-bit SAR");
}

#[test]
fn test_sar_rax_1_negative() {
    // SAR RAX, 1 with negative number
    let code = [
        0x48, 0xd1, 0xf8, // SAR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000; // Most negative 64-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xC000000000000000,
        "RAX: 0x8000000000000000 >> 1 = 0xC000000000000000"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(sf_set(regs.rflags), "SF: result is still negative");
}

#[test]
fn test_sar_rax_cl() {
    // SAR RAX, CL (opcode 48 D3 /7)
    let code = [
        0x48, 0xd3, 0xf8, // SAR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFF0000; // Negative
    regs.rcx = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX: 0xFFFFFFFFFFFF0000 >> 16 = 0xFFFFFFFFFFFFFFFF"
    );
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_rax_imm8() {
    // SAR RAX, imm8 (opcode 48 C1 /7 ib)
    let code = [
        0x48, 0xc1, 0xf8, 0x20, // SAR RAX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000012345678,
        "RAX: high 32 bits shifted to low 32"
    );
    assert!(!sf_set(regs.rflags), "SF: result is positive");
}

#[test]
fn test_sar_count_masked_64bit() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0x48, 0xd3, 0xf8, // SAR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 0x7F; // 127 & 0x3F = 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX: sign extended to all ones"
    );
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_sar_r8b_1() {
    // SAR R8B, 1
    let code = [
        0x41, 0xd0, 0xf8, // SAR R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA; // 1010_1010 (negative)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFF,
        0xD5,
        "R8B: 0xAA >> 1 = 0xD5 (sign extended)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_r10w_cl() {
    // SAR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xfa, // SAR R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0xF000; // Negative
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r10 & 0xFFFF,
        0xFF00,
        "R10W: 0xF000 >> 4 = 0xFF00 (sign extended)"
    );
}

#[test]
fn test_sar_r12d_imm8() {
    // SAR R12D, imm8
    let code = [
        0x41, 0xc1, 0xfc, 0x08, // SAR R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0x00123456,
        "R12D: 0x12345678 >> 8 = 0x00123456"
    );
}

#[test]
fn test_sar_r15_1() {
    // SAR R15, 1
    let code = [
        0x49, 0xd1, 0xff, // SAR R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFEDCBA9876543210; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xFF6E5D4C3B2A1908, "R15: signed right shift by 1");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_sar_byte_ptr_1() {
    // SAR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x3c,
        0x25, // SAR byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x82); // Negative

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0xC1, "Memory: 0x82 >> 1 = 0xC1 (sign extended)");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_sar_word_ptr_cl() {
    // SAR word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x3c,
        0x25, // SAR word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x08;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0xF000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(
        result, 0xFFF0,
        "Memory: 0xF000 >> 8 = 0xFFF0 (sign extended)"
    );
}

#[test]
fn test_sar_dword_ptr_imm8() {
    // SAR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x3c,
        0x25, // SAR dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x80000000); // Negative

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0xFFFF8000, "Memory: 0x80000000 >> 16 = 0xFFFF8000");
}

#[test]
fn test_sar_qword_ptr_cl() {
    // SAR qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x3c,
        0x25, // SAR qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0xFFFFFFFF00000000); // Negative

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(
        result, 0xFFFFFFFFFFFFFFFF,
        "Memory: 0xFFFFFFFF00000000 >> 32 = all ones"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_sar_signed_divide_by_power_of_2() {
    // SAR can divide signed numbers by powers of 2
    // -16 / 4 = -4
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2 (divide by 4)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-16i32) as u32 as u64; // -16
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rax & 0xFFFFFFFF) as i32, -4, "EAX: -16 / 4 = -4");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_rounding_toward_negative_infinity() {
    // SAR rounds toward negative infinity (unlike IDIV which rounds toward zero)
    // -9 >> 2 = -3 (not -2 like IDIV would give)
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-9i32) as u32 as u64; // -9
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        (regs.rax & 0xFFFFFFFF) as i32,
        -3,
        "EAX: -9 >> 2 = -3 (rounds toward -∞)"
    );
}

#[test]
fn test_sar_positive_divide() {
    // SAR divides positive numbers correctly
    // 100 / 4 = 25
    let code = [
        0xc1, 0xf8, 0x02, // SAR EAX, 2
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 25, "EAX: 100 / 4 = 25");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
}

#[test]
fn test_sar_all_ones_stays_negative_one() {
    // Shifting -1 right (all ones) stays -1
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: -1 >> 1 = -1 (stays all ones)"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_to_zero_positive() {
    // Shift positive number to zero
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678; // Positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX: positive number >> 31 = 0");
    assert!(zf_set(regs.rflags), "ZF: result is zero");
    assert!(!sf_set(regs.rflags), "SF: result is not negative");
}

#[test]
fn test_sar_to_negative_one() {
    // Shift negative number completely
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: negative number >> 31 = -1 (all ones)"
    );
    assert!(sf_set(regs.rflags), "SF: result is negative");
}

#[test]
fn test_sar_extract_sign_bit() {
    // SAR by width-1 extracts sign bit (0 or -1)
    let code = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF; // Most positive 32-bit value
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX: positive >> 31 = 0");

    // Try with negative
    let code2 = [
        0xc1, 0xf8, 0x1F, // SAR EAX, 31
        0xf4,
    ];
    let mut regs2 = Registers::default();
    regs2.rax = 0x80000000; // Most negative
    let (mut vcpu2, _) = setup_vm(&code2, Some(regs2));
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert_eq!(
        regs2.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: negative >> 31 = -1"
    );
}

#[test]
fn test_sar_parity_flag() {
    // PF is set based on low byte parity
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x06; // 0000_0110
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x03, "EAX: 0x06 >> 1 = 0x03");
    // 0x03 = 0000_0011, two 1 bits (even), so PF should be set
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_sar_chained_shifts() {
    // Multiple SAR in sequence
    let code = [
        0xd1, 0xf8, // SAR EAX, 1
        0xd1, 0xf8, // SAR EAX, 1
        0xd1, 0xf8, // SAR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF8000000; // Negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFF000000,
        "EAX: 0xF8000000 >> 3 = 0xFF000000"
    );
    assert!(sf_set(regs.rflags), "SF: result is still negative");
}

#[test]
fn test_sar_sign_extension_propagation() {
    // Verify sign extension propagates correctly
    let code = [
        0x48, 0xc1, 0xf8, 0x01, // SAR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFEDCBA9876543210; // Negative (MSB = 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should preserve sign bit in MSB
    assert_eq!(
        regs.rax & 0x8000000000000000,
        0x8000000000000000,
        "MSB should remain 1"
    );
    assert!(sf_set(regs.rflags), "SF: result is negative");
}
