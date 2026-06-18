// SHR (Shift Logical Right) instruction tests
//
// Opcodes:
// D0 /5       SHR r/m8, 1
// D2 /5       SHR r/m8, CL
// C0 /5 ib    SHR r/m8, imm8
// D1 /5       SHR r/m16, 1
// D3 /5       SHR r/m16, CL
// C1 /5 ib    SHR r/m16, imm8
// D1 /5       SHR r/m32, 1
// D3 /5       SHR r/m32, CL
// C1 /5 ib    SHR r/m32, imm8
// REX.W + D1 /5    SHR r/m64, 1
// REX.W + D3 /5    SHR r/m64, CL
// REX.W + C1 /5 ib SHR r/m64, imm8
//
// SHR performs unsigned division by powers of 2
// Fills empty bit positions with zeros (unlike SAR which fills with sign bit)
//
// Flags:
// - CF: Last bit shifted out
// - OF: MSB of original operand (only for 1-bit shifts)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 8-bit SHR tests
// ============================================================================

#[test]
fn test_shr_al_1() {
    // SHR AL, 1 (opcode D0 /5)
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x42 >> 1 = 0x21");
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
    assert!(
        !of_set(regs.rflags),
        "OF should be clear (MSB of original was 0)"
    );
    assert!(!sf_set(regs.rflags), "SF should be clear");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_shr_al_1_with_carry() {
    // SHR AL, 1 with LSB set
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x43; // 0100_0011
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x43 >> 1 = 0x21");
    assert!(cf_set(regs.rflags), "CF should be set (LSB was 1)");
    assert!(!of_set(regs.rflags), "OF: MSB of original was 0");
}

#[test]
fn test_shr_al_1_msb_set() {
    // SHR with MSB set (tests OF flag for 1-bit shift)
    let code = [
        0xd0, 0xe8, // SHR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 1000_0000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x40, "AL: 0x80 >> 1 = 0x40");
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
    assert!(
        of_set(regs.rflags),
        "OF should be set (MSB of original was 1)"
    );
}

#[test]
fn test_shr_al_cl() {
    // SHR AL, CL (opcode D2 /5)
    let code = [
        0xd2, 0xe8, // SHR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 1000_0000
    regs.rcx = 0x07; // Shift by 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL: 0x80 >> 7 = 0x01");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_shr_al_imm8() {
    // SHR AL, imm8 (opcode C0 /5 ib)
    let code = [
        0xc0, 0xe8, 0x03, // SHR AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88; // 1000_1000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x11, "AL: 0x88 >> 3 = 0x11");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_shr_al_to_zero() {
    // Shift all bits out
    let code = [
        0xc0, 0xe8, 0x08, // SHR AL, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL: all bits shifted out");
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(!sf_set(regs.rflags), "SF should be clear");
}

#[test]
fn test_shr_count_masked_8bit() {
    // Count is masked to 5 bits, so 0x23 (35) becomes 3
    let code = [
        0xd2, 0xe8, // SHR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88;
    regs.rcx = 0x23; // 35 & 0x1F = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x11, "AL: 0x88 >> 3 = 0x11 (count masked)");
}

#[test]
fn test_shr_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xe8, 0x00, // SHR AL, 0
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
// 16-bit SHR tests
// ============================================================================

#[test]
fn test_shr_ax_1() {
    // SHR AX, 1 (opcode 66 D1 /5)
    let code = [
        0x66, 0xd1, 0xe8, // SHR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x2190, "AX: 0x4321 >> 1 = 0x2190");
    assert!(cf_set(regs.rflags), "CF should be set (LSB was 1)");
    assert!(!of_set(regs.rflags), "OF: MSB of original was 0");
}

#[test]
fn test_shr_ax_cl() {
    // SHR AX, CL (opcode 66 D3 /5)
    let code = [
        0x66, 0xd3, 0xe8, // SHR AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    regs.rcx = 0x0F; // Shift by 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX: 0x8000 >> 15 = 0x0001");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_ax_imm8() {
    // SHR AX, imm8 (opcode 66 C1 /5 ib)
    let code = [
        0x66, 0xc1, 0xe8, 0x04, // SHR AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0123, "AX: 0x1234 >> 4 = 0x0123");
    // CF = bit 3 of 0x1234 = 0 (last bit shifted out)
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (bit 3 of 0x1234 was 0)"
    );
}

#[test]
fn test_shr_ax_with_msb() {
    // Unlike SAR, SHR fills with zeros
    let code = [
        0x66, 0xd1, 0xe8, // SHR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x4000,
        "AX: 0x8000 >> 1 = 0x4000 (zero fill)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
    assert!(
        of_set(regs.rflags),
        "OF should be set (MSB of original was 1)"
    );
}

// ============================================================================
// 32-bit SHR tests
// ============================================================================

#[test]
fn test_shr_eax_1() {
    // SHR EAX, 1 (opcode D1 /5)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
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
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_shr_eax_cl() {
    // SHR EAX, CL (opcode D3 /5)
    let code = [
        0xd3, 0xe8, // SHR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rcx = 0x1F; // Shift by 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX: 0x80000000 >> 31 = 0x00000001"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_eax_imm8() {
    // SHR EAX, imm8 (opcode C1 /5 ib)
    let code = [
        0xc1, 0xe8, 0x08, // SHR EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00123456,
        "EAX: 0x12345678 >> 8 = 0x00123456"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shr_eax_with_msb() {
    // SHR fills with zeros (unlike SAR)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x40000000,
        "EAX: 0x80000000 >> 1 = 0x40000000 (zero fill)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
    assert!(
        of_set(regs.rflags),
        "OF should be set (MSB of original was 1)"
    );
}

#[test]
fn test_shr_count_masked_32bit() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0xd3, 0xe8, // SHR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rcx = 0x3F; // 63 & 0x1F = 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX: 0x80000000 >> 31 (count masked)"
    );
}

// ============================================================================
// 64-bit SHR tests
// ============================================================================

#[test]
fn test_shr_rax_1() {
    // SHR RAX, 1 (opcode 48 D1 /5)
    let code = [
        0x48, 0xd1, 0xe8, // SHR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x091A2B3C4D5E6F78, "RAX: 0x123456789ABCDEF0 >> 1");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_shr_rax_cl() {
    // SHR RAX, CL (opcode 48 D3 /5)
    let code = [
        0x48, 0xd3, 0xe8, // SHR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 0x3F; // Shift by 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX: 0x8000000000000000 >> 63"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
}

#[test]
fn test_shr_rax_imm8() {
    // SHR RAX, imm8 (opcode 48 C1 /5 ib)
    let code = [
        0x48, 0xc1, 0xe8, 0x20, // SHR RAX, 32
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
    // CF = bit 31 of original = MSB of 0x9ABCDEF0 = 1
    assert!(
        cf_set(regs.rflags),
        "CF should be set (bit 31 of 0x9ABCDEF0 is 1)"
    );
}

#[test]
fn test_shr_rax_with_msb() {
    // SHR fills with zeros
    let code = [
        0x48, 0xd1, 0xe8, // SHR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x4000000000000000,
        "RAX: 0x8000000000000000 >> 1 (zero fill)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
    assert!(
        of_set(regs.rflags),
        "OF should be set (MSB of original was 1)"
    );
}

#[test]
fn test_shr_count_masked_64bit() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0x48, 0xd3, 0xe8, // SHR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 0x7F; // 127 & 0x3F = 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX: 0x8000000000000000 >> 63 (count masked to 6 bits)"
    );
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_shr_r8b_1() {
    // SHR R8B, 1
    let code = [
        0x41, 0xd0, 0xe8, // SHR R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA; // 1010_1010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: 0xAA >> 1 = 0x55");
    assert!(!cf_set(regs.rflags), "CF should be clear (LSB was 0)");
}

#[test]
fn test_shr_r10w_cl() {
    // SHR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xea, // SHR R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1234;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x0123, "R10W: 0x1234 >> 4 = 0x0123");
}

#[test]
fn test_shr_r12d_imm8() {
    // SHR R12D, imm8
    let code = [
        0x41, 0xc1, 0xec, 0x08, // SHR R12D, 8
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
fn test_shr_r15_1() {
    // SHR R15, 1
    let code = [
        0x49, 0xd1, 0xef, // SHR R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x7F6E5D4C3B2A1908,
        "R15: logical right shift by 1 (zero fill)"
    );
    assert!(!sf_set(regs.rflags), "SF should be clear (result < 2^63)");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shr_byte_ptr_1() {
    // SHR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x2c,
        0x25, // SHR byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x82);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x41, "Memory: 0x82 >> 1 = 0x41 (zero fill)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shr_word_ptr_cl() {
    // SHR word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x2c,
        0x25, // SHR word ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0x00F0, "Memory: 0xF000 >> 8 = 0x00F0 (zero fill)");
}

#[test]
fn test_shr_dword_ptr_imm8() {
    // SHR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x2c,
        0x25, // SHR dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8 = 16
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x80000000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x00008000, "Memory: 0x80000000 >> 16 = 0x00008000");
}

#[test]
fn test_shr_qword_ptr_cl() {
    // SHR qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x2c,
        0x25, // SHR qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0xFFFFFFFF00000000);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(
        result, 0x00000000FFFFFFFF,
        "Memory: 0xFFFFFFFF00000000 >> 32"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shr_unsigned_divide_by_power_of_2() {
    // SHR can divide unsigned numbers by powers of 2
    // 100 / 4 = 25
    let code = [
        0xc1, 0xe8, 0x02, // SHR EAX, 2 (divide by 4)
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
fn test_shr_vs_sar_negative_values() {
    // SHR treats operand as unsigned, SAR as signed
    // 0xFFFFFFFF >> 1 = 0x7FFFFFFF (SHR)
    // 0xFFFFFFFF >> 1 = 0xFFFFFFFF (SAR would do this)
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFF,
        "EAX: SHR fills with zero (not sign)"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
    assert!(!sf_set(regs.rflags), "SF: result is positive");
}

#[test]
fn test_shr_extract_high_bits() {
    // Shift to extract high bits
    let code = [
        0x48, 0xc1, 0xe8, 0x20, // SHR RAX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x00000000FFFFFFFF,
        "RAX: high 32 bits moved to low"
    );
}

#[test]
fn test_shr_overflow_flag_1bit() {
    // OF is set to MSB of original operand for 1-bit shifts
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x40000000; // MSB = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!of_set(regs.rflags), "OF: MSB of original was 0");

    // Try with MSB = 1
    let code2 = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs2 = Registers::default();
    regs2.rax = 0x80000000; // MSB = 1
    let (mut vcpu2, _) = setup_vm(&code2, Some(regs2));
    let regs2 = run_until_hlt(&mut vcpu2).unwrap();

    assert!(of_set(regs2.rflags), "OF: MSB of original was 1");
}

#[test]
fn test_shr_parity_flag() {
    // PF is set based on low byte parity
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
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
fn test_shr_chained_shifts() {
    // Multiple SHR in sequence
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xd1, 0xe8, // SHR EAX, 1
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF8000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1F000000,
        "EAX: 0xF8000000 >> 3 = 0x1F000000"
    );
}

#[test]
fn test_shr_all_ones() {
    // Shift all 1s
    let code = [
        0xd1, 0xe8, // SHR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x7FFFFFFF,
        "EAX: 0xFFFFFFFF >> 1 = 0x7FFFFFFF"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
    assert!(!sf_set(regs.rflags), "SF: result is positive (MSB = 0)");
    assert!(!zf_set(regs.rflags), "ZF: result is not zero");
}

#[test]
fn test_shr_isolate_bits() {
    // Use SHR to isolate specific bit fields
    // Extract bits 8-15 from a 32-bit value
    let code = [
        0xc1, 0xe8, 0x08, // SHR EAX, 8
        0x25, 0xFF, 0x00, 0x00, 0x00, // AND EAX, 0xFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x56,
        "EAX: extracted byte at bits 8-15"
    );
}
