// RCR (Rotate through Carry Right) instruction tests
//
// Opcodes:
// D0 /3       RCR r/m8, 1
// D2 /3       RCR r/m8, CL
// C0 /3 ib    RCR r/m8, imm8
// D1 /3       RCR r/m16, 1
// D3 /3       RCR r/m16, CL
// C1 /3 ib    RCR r/m16, imm8
// D1 /3       RCR r/m32, 1
// D3 /3       RCR r/m32, CL
// C1 /3 ib    RCR r/m32, imm8
// REX.W + D1 /3    RCR r/m64, 1
// REX.W + D3 /3    RCR r/m64, CL
// REX.W + C1 /3 ib RCR r/m64, imm8
//
// RCR rotates bits right through the carry flag.
// Unlike ROR, CF participates in the rotation:
// - 8-bit:  Rotates 9 bits (r/m8 + CF)
// - 16-bit: Rotates 17 bits (r/m16 + CF)
// - 32-bit: Rotates 33 bits (r/m32 + CF)
// - 64-bit: Rotates 65 bits (r/m64 + CF)
//
// Flags:
// - CF: Receives LSB shifted out, then participates in next rotation
// - OF: Only for 1-bit rotates (MSB XOR next-to-MSB of result)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 8-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_al_1_cf_clear() {
    // RCR AL, 1 with CF initially clear (opcode D0 /3)
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0100_0010_0 becomes 0_0100_0010
    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x42 RCR 1 (CF=0) = 0x21");
    assert!(!cf_set(regs.rflags), "CF: receives old LSB (was 0)");
}

#[test]
fn test_rcr_al_1_cf_set() {
    // RCR AL, 1 with CF initially set
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0100_0010_1 becomes 1_0100_0010
    assert_eq!(regs.rax & 0xFF, 0xA1, "AL: 0x42 RCR 1 (CF=1) = 0xA1");
    assert!(!cf_set(regs.rflags), "CF: receives old LSB (was 0)");
}

#[test]
fn test_rcr_al_1_with_lsb() {
    // RCR AL, 1 with LSB set
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x43; // 0100_0011
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0100_0011_0 becomes 0_0100_0011 with CF=1
    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x43 RCR 1 (CF=0) = 0x21");
    assert!(cf_set(regs.rflags), "CF: receives old LSB (was 1)");
}

#[test]
fn test_rcr_al_cl() {
    // RCR AL, CL (opcode D2 /3)
    let code = [
        0xd2, 0xd8, // RCR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80;
    regs.rcx = 0x08; // Rotate by 8 (full byte + CF position)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 8 rotations of 9-bit value, should have CF at MSB
    assert_eq!(regs.rax & 0xFF, 0x01, "AL: after full 9-bit rotation");
}

#[test]
fn test_rcr_al_imm8() {
    // RCR AL, imm8 (opcode C0 /3 ib)
    let code = [
        0xc0, 0xd8, 0x03, // RCR AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88; // 1000_1000
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 1000_1000_0 rotated right by 3
    assert_eq!(regs.rax & 0xFF, 0x11, "AL: 0x88 RCR 3 (CF=0) = 0x11");
}

#[test]
fn test_rcr_propagates_cf() {
    // Verify CF participates in rotation
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xd0, 0xd8, // RCR AL, 1 again (should use CF from first)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01; // 0000_0001
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // First: 0000_0001_0 -> CF=1, AL=0000_0000
    // Second: 0000_0000_1 -> CF=0, AL=1000_0000
    assert_eq!(regs.rax & 0xFF, 0x80, "AL: CF propagated through rotations");
    assert!(!cf_set(regs.rflags), "CF: cleared after second rotation");
}

#[test]
fn test_rcr_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xd8, 0x00, // RCR AL, 0
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
// 16-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_ax_1_cf_clear() {
    // RCR AX, 1 with CF initially clear (opcode 66 D1 /3)
    let code = [
        0x66, 0xd1, 0xd8, // RCR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x2190,
        "AX: 0x4321 RCR 1 (CF=0) = 0x2190"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_rcr_ax_1_cf_set() {
    // RCR AX, 1 with CF initially set
    let code = [
        0x66, 0xd1, 0xd8, // RCR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4320;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0xA190,
        "AX: 0x4320 RCR 1 (CF=1) = 0xA190"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_rcr_ax_cl() {
    // RCR AX, CL (opcode 66 D3 /3)
    let code = [
        0x66, 0xd3, 0xd8, // RCR AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    regs.rcx = 0x10; // Rotate by 16 (full word + CF)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After full 17-bit rotation, value should be back
    assert_eq!(
        regs.rax & 0xFFFF,
        0x0001,
        "AX: after 16 rotations of 17-bit value"
    );
}

#[test]
fn test_rcr_ax_imm8() {
    // RCR AX, imm8 (opcode 66 C1 /3 ib)
    let code = [
        0x66, 0xc1, 0xd8, 0x04, // RCR AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x8123,
        "AX: 0x1234 RCR 4 (CF=0) = 0x8123"
    );
}

// ============================================================================
// 32-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_eax_1_cf_clear() {
    // RCR EAX, 1 with CF initially clear (opcode D1 /3)
    let code = [
        0xd1, 0xd8, // RCR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x091A2B3C,
        "EAX: 0x12345678 RCR 1 (CF=0)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_rcr_eax_1_cf_set() {
    // RCR EAX, 1 with CF initially set
    let code = [
        0xd1, 0xd8, // RCR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x891A2B3C,
        "EAX: 0x12345678 RCR 1 (CF=1)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_rcr_eax_cl() {
    // RCR EAX, CL (opcode D3 /3)
    let code = [
        0xd3, 0xd8, // RCR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rcx = 0x20; // Rotate by 32 (full dword + CF)
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 32 rotations of 33-bit value, CF should be at MSB
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: after full 33-bit rotation"
    );
}

#[test]
fn test_rcr_eax_imm8() {
    // RCR EAX, imm8 (opcode C1 /3 ib)
    let code = [
        0xc1, 0xd8, 0x08, // RCR EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xF0123456,
        "EAX: 0x12345678 RCR 8 (CF=0)"
    );
}

#[test]
fn test_rcr_eax_with_lsb() {
    // RCR with LSB set
    let code = [
        0xd1, 0xd8, // RCR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: 0x00000001 RCR 1 (CF=0)"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

// ============================================================================
// 64-bit RCR tests
// ============================================================================

#[test]
fn test_rcr_rax_1_cf_clear() {
    // RCR RAX, 1 with CF initially clear (opcode 48 D1 /3)
    let code = [
        0x48, 0xd1, 0xd8, // RCR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x091A2B3C4D5E6F78,
        "RAX: 0x123456789ABCDEF0 RCR 1 (CF=0)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_rcr_rax_1_cf_set() {
    // RCR RAX, 1 with CF initially set
    let code = [
        0x48, 0xd1, 0xd8, // RCR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x891A2B3C4D5E6F78,
        "RAX: 0x123456789ABCDEF0 RCR 1 (CF=1)"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_rcr_rax_cl() {
    // RCR RAX, CL (opcode 48 D3 /3)
    let code = [
        0x48, 0xd3, 0xd8, // RCR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 0x3F; // Rotate by 63
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After 63 rotations, bit should be at bit 1
    assert_eq!(regs.rax, 0x0000000000000003, "RAX: bit rotated from MSB");
}

#[test]
fn test_rcr_rax_imm8() {
    // RCR RAX, imm8 (opcode 48 C1 /3 ib)
    let code = [
        0x48, 0xc1, 0xd8, 0x10, // RCR RAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xBDE0123456789ABC,
        "RAX: 0x123456789ABCDEF0 RCR 16 (CF=0)"
    );
}

#[test]
fn test_rcr_rax_with_lsb() {
    // RCR with LSB set
    let code = [
        0x48, 0xd1, 0xd8, // RCR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX: 0x0000000000000001 RCR 1 (CF=0)"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rcr_r8b_1() {
    // RCR R8B, 1
    let code = [
        0x41, 0xd0, 0xd8, // RCR R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA; // 1010_1010
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xD5, "R8B: 0xAA RCR 1 (CF=1) = 0xD5");
}

#[test]
fn test_rcr_r10w_cl() {
    // RCR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xda, // RCR R10W, CL
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
        0x8123,
        "R10W: 0x1234 RCR 4 (CF=0) = 0x8123"
    );
}

#[test]
fn test_rcr_r12d_imm8() {
    // RCR R12D, imm8
    let code = [
        0x41, 0xc1, 0xdc, 0x08, // RCR R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0xF0123456,
        "R12D: 0x12345678 RCR 8 (CF=0)"
    );
}

#[test]
fn test_rcr_r15_1() {
    // RCR R15, 1
    let code = [
        0x49, 0xd1, 0xdf, // RCR R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFEDCBA9876543210;
    regs.rflags = 0x2 | flags::bits::CF; // CF = 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0xFF6E5D4C3B2A1908,
        "R15: 0xFEDCBA9876543210 RCR 1 (CF=1)"
    );
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rcr_byte_ptr_1() {
    // RCR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x1c,
        0x25, // RCR byte ptr [DATA_ADDR], 1
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

    assert_eq!(result, 0x21, "Memory: 0x42 RCR 1 (CF=0) = 0x21");
}

#[test]
fn test_rcr_word_ptr_cl() {
    // RCR word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x1c,
        0x25, // RCR word ptr [DATA_ADDR], CL
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
    write_mem_u16(&mem, 0x1230);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x1123, "Memory: 0x1230 RCR 4 (CF=1) = 0x1123");
}

#[test]
fn test_rcr_dword_ptr_imm8() {
    // RCR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x1c,
        0x25, // RCR dword ptr [DATA_ADDR], imm8
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

    assert_eq!(result, 0xF0123456, "Memory: 0x12345678 RCR 8 (CF=0)");
}

#[test]
fn test_rcr_qword_ptr_cl() {
    // RCR qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x1c,
        0x25, // RCR qword ptr [DATA_ADDR], CL
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
        result, 0xBDE0123456789ABC,
        "Memory: 0x123456789ABCDEF0 RCR 16 (CF=0)"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rcr_multi_precision_shift() {
    // RCR is used for multi-precision shifts
    // Shift a 64-bit value right using two 32-bit operations
    let code = [
        0xd1, 0xdb, // RCR EBX, 1 (high 32 bits)
        0xd1, 0xd8, // RCR EAX, 1 (low 32 bits, receives CF from EBX)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678; // High 32 bits
    regs.rax = 0x00000001; // Low 32 bits with LSB set
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x091A2B3C, "EBX: high bits shifted");
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX: low bits with CF from EBX"
    );
    assert!(cf_set(regs.rflags), "CF: LSB from EAX");
}

#[test]
fn test_rcr_overflow_flag_1bit() {
    // OF is set to MSB XOR next-to-MSB for 1-bit rotates
    let code = [
        0xd1, 0xd8, // RCR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001; // ...0001
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result: 0000_0000..., MSB=0, next-to-MSB=0, OF = 0 XOR 0 = 0
    assert!(!of_set(regs.rflags), "OF: MSB XOR next-to-MSB = 0");
}

#[test]
fn test_rcr_chained_with_different_cf() {
    // Chain multiple RCR operations
    let code = [
        0xd0, 0xd8, // RCR AL, 1
        0xd0, 0xdb, // RCR BL, 1
        0xd0, 0xd9, // RCR CL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01; // AL with LSB set
    regs.rbx = 0x00; // BL = 0
    regs.rcx = 0x00; // CL = 0
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL: 00000001_0 -> CF=1, AL=00000000
    // BL: 00000000_1 -> CF=0, BL=10000000
    // CL: 00000000_0 -> CF=0, CL=00000000
    assert_eq!(regs.rax & 0xFF, 0x00, "AL: rotated out");
    assert_eq!(regs.rbx & 0xFF, 0x80, "BL: received CF from AL");
    assert_eq!(regs.rcx & 0xFF, 0x00, "CL: received CF from BL");
}

#[test]
fn test_rcr_bit_extraction() {
    // RCR can extract LSB into CF
    let code = [
        0xd1, 0xd8, // RCR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345679; // LSB = 1
    regs.rflags = 0x2; // CF = 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF: extracted LSB = 1");
}
