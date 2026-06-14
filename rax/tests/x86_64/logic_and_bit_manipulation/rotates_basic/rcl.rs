// RCL (Rotate through Carry Left) instruction tests
//
// Opcodes:
// D0 /2       RCL r/m8, 1
// D2 /2       RCL r/m8, CL
// C0 /2 ib    RCL r/m8, imm8
// D1 /2       RCL r/m16, 1
// D3 /2       RCL r/m16, CL
// C1 /2 ib    RCL r/m16, imm8
// D1 /2       RCL r/m32, 1
// D3 /2       RCL r/m32, CL
// C1 /2 ib    RCL r/m32, imm8
// REX.W + D1 /2    RCL r/m64, 1
// REX.W + D3 /2    RCL r/m64, CL
// REX.W + C1 /2 ib RCL r/m64, imm8
//
// RCL rotates bits left through the carry flag.
// Unlike ROL, CF participates in the rotation:
// - 8-bit:  Rotates 9 bits (CF + r/m8)
// - 16-bit: Rotates 17 bits (CF + r/m16)
// - 32-bit: Rotates 33 bits (CF + r/m32)
// - 64-bit: Rotates 65 bits (CF + r/m64)
//
// Flags:
// - CF: Receives MSB shifted out, then participates in next rotation
// - OF: Only for 1-bit rotates (CF XOR new MSB)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 8-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_al_1_cf_clear() {
    // RCL AL, 1 with CF initially clear (opcode D0 /2)
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0_0100_0010 becomes 0100_0010_0
    assert_eq!(regs.rax & 0xFF, 0x84, "AL: 0x42 RCL 1 (CF=0) = 0x84");
    assert!(!cf_set(regs.rflags), "CF: receives old MSB (was 0)");
}

#[test]
fn test_rcl_al_1_cf_set() {
    // RCL AL, 1 with CF initially set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 1_0100_0010 becomes 0100_0010_1
    assert_eq!(regs.rax & 0xFF, 0x85, "AL: 0x42 RCL 1 (CF=1) = 0x85");
    assert!(!cf_set(regs.rflags), "CF: receives old MSB (was 0)");
}

#[test]
fn test_rcl_al_1_with_msb() {
    // RCL AL, 1 with MSB set
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x81; // 1000_0001
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0_1000_0001 becomes 1_0000_0010
    assert_eq!(regs.rax & 0xFF, 0x02, "AL: 0x81 RCL 1 (CF=0) = 0x02");
    assert!(cf_set(regs.rflags), "CF: receives old MSB (was 1)");
}

#[test]
fn test_rcl_al_cl() {
    // RCL AL, CL (opcode D2 /2)
    let code = [
        0xd2, 0xd0, // RCL AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    regs.rcx = 0x08; // Rotate by 8 (full byte + CF position)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 8 rotations of 9-bit value, CF should be in LSB
    assert_eq!(regs.rax & 0xFF, 0x80, "AL: after full 9-bit rotation");
}

#[test]
fn test_rcl_al_imm8() {
    // RCL AL, imm8 (opcode C0 /2 ib)
    let code = [
        0xc0, 0xd0, 0x03, // RCL AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11; // 0001_0001
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0_0001_0001 rotated left by 3
    assert_eq!(regs.rax & 0xFF, 0x88, "AL: 0x11 RCL 3 (CF=0) = 0x88");
}

#[test]
fn test_rcl_propagates_cf() {
    // Verify CF participates in rotation
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xd0, 0xd0, // RCL AL, 1 again (should use CF from first)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 1000_0000
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 0_1000_0000 -> CF=1, AL=0000_0000
    // Second: 1_0000_0000 -> CF=0, AL=0000_0001
    assert_eq!(regs.rax & 0xFF, 0x01, "AL: CF propagated through rotations");
    assert!(!cf_set(regs.rflags), "CF: cleared after second rotation");
}

#[test]
fn test_rcl_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xd0, 0x00, // RCL AL, 0
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
// 16-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_ax_1_cf_clear() {
    // RCL AX, 1 with CF initially clear (opcode 66 D1 /2)
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x8642,
        "AX: 0x4321 RCL 1 (CF=0) = 0x8642"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_1_cf_set() {
    // RCL AX, 1 with CF initially set
    let code = [
        0x66, 0xd1, 0xd0, // RCL AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x8643,
        "AX: 0x4321 RCL 1 (CF=1) = 0x8643"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_ax_cl() {
    // RCL AX, CL (opcode 66 D3 /2)
    let code = [
        0x66, 0xd3, 0xd0, // RCL AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    regs.rcx = 0x10; // Rotate by 16 (full word + CF)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After full 17-bit rotation, value should be back
    assert_eq!(
        regs.rax & 0xFFFF,
        0x8000,
        "AX: after 16 rotations of 17-bit value"
    );
}

#[test]
fn test_rcl_ax_imm8() {
    // RCL AX, imm8 (opcode 66 C1 /2 ib)
    let code = [
        0x66, 0xc1, 0xd0, 0x04, // RCL AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x1230,
        "AX: 0x0123 RCL 4 (CF=0) = 0x1230"
    );
}

// ============================================================================
// 32-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_eax_1_cf_clear() {
    // RCL EAX, 1 with CF initially clear (opcode D1 /2)
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468ACF0,
        "EAX: 0x12345678 RCL 1 (CF=0)"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_eax_1_cf_set() {
    // RCL EAX, 1 with CF initially set
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468ACF1,
        "EAX: 0x12345678 RCL 1 (CF=1)"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_eax_cl() {
    // RCL EAX, CL (opcode D3 /2)
    let code = [
        0xd3, 0xd0, // RCL EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rcx = 0x20; // Rotate by 32 (full dword + CF)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 32 rotations of 33-bit value, CF should be in LSB
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX: after full 33-bit rotation"
    );
}

#[test]
fn test_rcl_eax_imm8() {
    // RCL EAX, imm8 (opcode C1 /2 ib)
    let code = [
        0xc1, 0xd0, 0x08, // RCL EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x34567809,
        "EAX: 0x12345678 RCL 8 (CF=0)"
    );
}

#[test]
fn test_rcl_eax_with_msb() {
    // RCL with MSB set
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000002,
        "EAX: 0x80000001 RCL 1 (CF=0)"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
}

// ============================================================================
// 64-bit RCL tests
// ============================================================================

#[test]
fn test_rcl_rax_1_cf_clear() {
    // RCL RAX, 1 with CF initially clear (opcode 48 D1 /2)
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x2468ACF13579BDE0,
        "RAX: 0x123456789ABCDEF0 RCL 1 (CF=0)"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_rax_1_cf_set() {
    // RCL RAX, 1 with CF initially set
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x2468ACF13579BDE1,
        "RAX: 0x123456789ABCDEF0 RCL 1 (CF=1)"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rcl_rax_cl() {
    // RCL RAX, CL (opcode 48 D3 /2)
    let code = [
        0x48, 0xd3, 0xd0, // RCL RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rcx = 0x3F; // Rotate by 63
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 63 rotations, bit should be at bit 63
    assert_eq!(
        regs.rax, 0xC000000000000000,
        "RAX: bit rotated to MSB position"
    );
}

#[test]
fn test_rcl_rax_imm8() {
    // RCL RAX, imm8 (opcode 48 C1 /2 ib)
    let code = [
        0x48, 0xc1, 0xd0, 0x10, // RCL RAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x56789ABCDEF0091A,
        "RAX: 0x123456789ABCDEF0 RCL 16 (CF=0)"
    );
}

#[test]
fn test_rcl_rax_with_msb() {
    // RCL with MSB set
    let code = [
        0x48, 0xd1, 0xd0, // RCL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000001;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000002,
        "RAX: 0x8000000000000001 RCL 1 (CF=0)"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rcl_r8b_1() {
    // RCL R8B, 1
    let code = [
        0x41, 0xd0, 0xd0, // RCL R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x55; // 0101_0101
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xAB, "R8B: 0x55 RCL 1 (CF=1) = 0xAB");
}

#[test]
fn test_rcl_r10w_cl() {
    // RCL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xd2, // RCL R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1234;
    regs.rcx = 0x04;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r10 & 0xFFFF,
        0x2340,
        "R10W: 0x1234 RCL 4 (CF=0) = 0x2340"
    );
}

#[test]
fn test_rcl_r12d_imm8() {
    // RCL R12D, imm8
    let code = [
        0x41, 0xc1, 0xd4, 0x08, // RCL R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0x34567809,
        "R12D: 0x12345678 RCL 8 (CF=0)"
    );
}

#[test]
fn test_rcl_r15_1() {
    // RCL R15, 1
    let code = [
        0x49, 0xd1, 0xd7, // RCL R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0123456789ABCDEF;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x02468ACF13579BDF,
        "R15: 0x0123456789ABCDEF RCL 1 (CF=1)"
    );
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rcl_byte_ptr_1() {
    // RCL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x14,
        0x25, // RCL byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x42);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x84, "Memory: 0x42 RCL 1 (CF=0) = 0x84");
}

#[test]
fn test_rcl_word_ptr_cl() {
    // RCL word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x14,
        0x25, // RCL word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x04;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x1234);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x2348, "Memory: 0x1234 RCL 4 (CF=1) = 0x2348");
}

#[test]
fn test_rcl_dword_ptr_imm8() {
    // RCL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x14,
        0x25, // RCL dword ptr [DATA_ADDR], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x08, // imm8 = 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x12345678);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x34567809, "Memory: 0x12345678 RCL 8 (CF=0)");
}

#[test]
fn test_rcl_qword_ptr_cl() {
    // RCL qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x14,
        0x25, // RCL qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x10;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(
        result, 0x56789ABCDEF0091A,
        "Memory: 0x123456789ABCDEF0 RCL 16 (CF=0)"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rcl_multi_precision_shift() {
    // RCL is used for multi-precision shifts
    // Shift a 64-bit value left using two 32-bit operations
    let code = [
        0xd1, 0xd0, // RCL EAX, 1 (low 32 bits)
        0xd1, 0xd3, // RCL EBX, 1 (high 32 bits, receives CF from EAX)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // Low 32 bits with MSB set
    regs.rbx = 0x12345678; // High 32 bits
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX: low bits shifted");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x2468ACF1,
        "EBX: high bits with CF from EAX"
    );
}

#[test]
fn test_rcl_overflow_flag_1bit() {
    // OF is set to CF XOR MSB after rotation for 1-bit rotates
    let code = [
        0xd1, 0xd0, // RCL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x40000000; // 0100...
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result: 1000..., MSB=1, CF=0, OF = 0 XOR 1 = 1
    assert!(of_set(regs.rflags), "OF: CF XOR new MSB = 1");
}

#[test]
fn test_rcl_chained_with_different_cf() {
    // Chain multiple RCL operations
    let code = [
        0xd0, 0xd0, // RCL AL, 1
        0xd0, 0xd3, // RCL BL, 1
        0xd0, 0xd1, // RCL CL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // AL with MSB set
    regs.rbx = 0x00; // BL = 0
    regs.rcx = 0x00; // CL = 0
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL: 0_10000000 -> CF=1, AL=00000000
    // BL: 1_00000000 -> CF=0, BL=00000001
    // CL: 0_00000000 -> CF=0, CL=00000000
    assert_eq!(regs.rax & 0xFF, 0x00, "AL: rotated out");
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL: received CF from AL");
    assert_eq!(regs.rcx & 0xFF, 0x00, "CL: received CF from BL");
}
