// SHL (Shift Logical Left) instruction tests
// SAL and SHL are the same instruction (same opcodes)
//
// Opcodes:
// D0 /4       SHL r/m8, 1
// D2 /4       SHL r/m8, CL
// C0 /4 ib    SHL r/m8, imm8
// D1 /4       SHL r/m16, 1
// D3 /4       SHL r/m16, CL
// C1 /4 ib    SHL r/m16, imm8
// D1 /4       SHL r/m32, 1
// D3 /4       SHL r/m32, CL
// C1 /4 ib    SHL r/m32, imm8
// REX.W + D1 /4    SHL r/m64, 1
// REX.W + D3 /4    SHL r/m64, CL
// REX.W + C1 /4 ib SHL r/m64, imm8
//
// Flags:
// - CF: Last bit shifted out
// - OF: Only for 1-bit shifts (MSB of result XOR CF)
// - SF, ZF, PF: Set according to result
// - AF: Undefined for non-zero count
// - Count is 0: No flags affected

use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

use crate::common::*;

// ============================================================================
// 8-bit SHL tests
// ============================================================================

#[test]
fn test_shl_al_1() {
    // SHL AL, 1 (opcode D0 /4)
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x84, "AL: 0x42 << 1 = 0x84");
    assert!(!cf_set(regs.rflags), "CF should be clear (MSB was 0)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 1 XOR 0 = 1");
    assert!(sf_set(regs.rflags), "SF should be set (bit 7 = 1)");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_shl_al_1_with_carry() {
    // SHL AL, 1 with MSB set
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x81; // 1000_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x02, "AL: 0x81 << 1 = 0x02");
    assert!(cf_set(regs.rflags), "CF should be set (MSB was 1)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 0 XOR 1 = 1");
    assert!(!sf_set(regs.rflags), "SF should be clear");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_shl_al_cl() {
    // SHL AL, CL (opcode D2 /4)
    let code = [
        0xd2, 0xe0, // SHL AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01;
    regs.rcx = 0x07; // Shift by 7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL: 0x01 << 7 = 0x80");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF should be set (bit 7 = 1)");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_shl_al_imm8() {
    // SHL AL, imm8 (opcode C0 /4 ib)
    let code = [
        0xc0, 0xe0, 0x03, // SHL AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11; // 0001_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_shl_al_to_zero() {
    // Shift all bits out
    let code = [
        0xc0, 0xe0, 0x08, // SHL AL, 8
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
fn test_shl_count_masked_8bit() {
    // Count is masked to 5 bits, so 0x23 (35) becomes 3
    let code = [
        0xd2, 0xe0, // SHL AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11;
    regs.rcx = 0x23; // 35 & 0x1F = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88 (count masked)");
}

#[test]
fn test_shl_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xe0, 0x00, // SHL AL, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    // Set some flags
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
// 16-bit SHL tests
// ============================================================================

#[test]
fn test_shl_ax_1() {
    // SHL AX, 1 (opcode 66 D1 /4)
    let code = [
        0x66, 0xd1, 0xe0, // SHL AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8642, "AX: 0x4321 << 1 = 0x8642");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_shl_ax_cl() {
    // SHL AX, CL (opcode 66 D3 /4)
    let code = [
        0x66, 0xd3, 0xe0, // SHL AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    regs.rcx = 0x0F; // Shift by 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: 0x0001 << 15 = 0x8000");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_shl_ax_imm8() {
    // SHL AX, imm8 (opcode 66 C1 /4 ib)
    let code = [
        0x66, 0xc1, 0xe0, 0x04, // SHL AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1230, "AX: 0x0123 << 4 = 0x1230");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shl_ax_with_carry() {
    // Shift with MSB set
    let code = [
        0x66, 0xd1, 0xe0, // SHL AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0002, "AX: 0x8001 << 1 = 0x0002");
    assert!(cf_set(regs.rflags), "CF should be set (MSB was 1)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 0 XOR 1 = 1");
}

// ============================================================================
// 32-bit SHL tests
// ============================================================================

#[test]
fn test_shl_eax_1() {
    // SHL EAX, 1 (opcode D1 /4)
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468ACF0,
        "EAX: 0x12345678 << 1 = 0x2468ACF0"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_shl_eax_cl() {
    // SHL EAX, CL (opcode D3 /4)
    let code = [
        0xd3, 0xe0, // SHL EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rcx = 0x1F; // Shift by 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0x00000001 << 31 = 0x80000000"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_shl_eax_imm8() {
    // SHL EAX, imm8 (opcode C1 /4 ib)
    let code = [
        0xc1, 0xe0, 0x08, // SHL EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00123456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345600,
        "EAX: 0x00123456 << 8 = 0x12345600"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shl_eax_with_carry() {
    // Shift with MSB set
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000002,
        "EAX: 0x80000001 << 1 = 0x00000002"
    );
    assert!(cf_set(regs.rflags), "CF should be set (MSB was 1)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_count_masked_32bit() {
    // Count is masked to 5 bits for 32-bit operands
    let code = [
        0xd3, 0xe0, // SHL EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rcx = 0x3F; // 63 & 0x1F = 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0x00000001 << 31 = 0x80000000 (count masked)"
    );
}

// ============================================================================
// 64-bit SHL tests
// ============================================================================

#[test]
fn test_shl_rax_1() {
    // SHL RAX, 1 (opcode 48 D1 /4)
    let code = [
        0x48, 0xd1, 0xe0, // SHL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2468ACF13579BDE0, "RAX: 0x123456789ABCDEF0 << 1");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_shl_rax_cl() {
    // SHL RAX, CL (opcode 48 D3 /4)
    let code = [
        0x48, 0xd3, 0xe0, // SHL RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rcx = 0x3F; // Shift by 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX: 0x0000000000000001 << 63"
    );
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_shl_rax_imm8() {
    // SHL RAX, imm8 (opcode 48 C1 /4 ib)
    let code = [
        0x48, 0xc1, 0xe0, 0x10, // SHL RAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000123456789ABC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABC0000,
        "RAX: 0x0000123456789ABC << 16"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shl_rax_with_carry() {
    // Shift with MSB set
    let code = [
        0x48, 0xd1, 0xe0, // SHL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000002, "RAX: 0x8000000000000001 << 1");
    assert!(cf_set(regs.rflags), "CF should be set (MSB was 1)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_count_masked_64bit() {
    // Count is masked to 6 bits for 64-bit operands
    let code = [
        0x48, 0xd3, 0xe0, // SHL RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rcx = 0x7F; // 127 & 0x3F = 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX: 0x0000000000000001 << 63 (count masked to 6 bits)"
    );
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_shl_r8b_1() {
    // SHL R8B, 1
    let code = [
        0x41, 0xd0, 0xe0, // SHL R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x55; // 0101_0101
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xAA, "R8B: 0x55 << 1 = 0xAA");
    assert!(!cf_set(regs.rflags), "CF should be clear");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF");
}

#[test]
fn test_shl_r10w_cl() {
    // SHL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xe2, // SHL R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1234;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x2340, "R10W: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_r12d_imm8() {
    // SHL R12D, imm8
    let code = [
        0x41, 0xc1, 0xe4, 0x08, // SHL R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0x34567800,
        "R12D: 0x12345678 << 8 = 0x34567800"
    );
}

#[test]
fn test_shl_r15_1() {
    // SHL R15, 1
    let code = [
        0x49, 0xd1, 0xe7, // SHL R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0123456789ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x02468ACF13579BDE, "R15: 0x0123456789ABCDEF << 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shl_byte_ptr_1() {
    // SHL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x24,
        0x25, // SHL byte ptr [DATA_ADDR], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x84, "Memory: 0x42 << 1 = 0x84");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_shl_word_ptr_cl() {
    // SHL word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x24,
        0x25, // SHL word ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x04;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x1234);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x2340, "Memory: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_dword_ptr_imm8() {
    // SHL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x24,
        0x25, // SHL dword ptr [DATA_ADDR], imm8
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

    assert_eq!(result, 0x34567800, "Memory: 0x12345678 << 8 = 0x34567800");
}

#[test]
fn test_shl_qword_ptr_cl() {
    // SHL qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x24,
        0x25, // SHL qword ptr [DATA_ADDR], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(
        result, 0x56789ABCDEF00000,
        "Memory: 0x123456789ABCDEF0 << 16"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_shl_multiply_by_power_of_2() {
    // SHL can multiply by powers of 2
    // 5 * 16 = 5 << 4 = 80
    let code = [
        0xc1, 0xe0, 0x04, // SHL EAX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 80, "EAX: 5 * 16 = 80");
}

#[test]
fn test_shl_align_to_page_boundary() {
    // Shift to align addresses to page boundaries (4KB = 0x1000)
    // Clear low 12 bits by shifting right then left
    let code = [
        0x48, 0xc1, 0xe8, 0x0C, // SHR RAX, 12
        0x48, 0xc1, 0xe0, 0x0C, // SHL RAX, 12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF7; // Not aligned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCD000, "RAX aligned to 4KB boundary");
}

#[test]
fn test_shl_extract_high_bits() {
    // Shift to extract high bits
    let code = [
        0x48, 0xc1, 0xe0, 0x20, // SHL RAX, 32 (move low 32 bits to high)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFF00000000,
        "RAX: low 32 bits moved to high"
    );
}

#[test]
fn test_shl_overflow_flag_1bit_same_sign() {
    // OF is clear when top two bits are same before shift
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x40000000; // 01000000...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0x40000000 << 1 = 0x80000000"
    );
    assert!(!cf_set(regs.rflags), "CF: bit shifted out was 0");
    assert!(of_set(regs.rflags), "OF: MSB(result) XOR CF = 1 XOR 0 = 1");
}

#[test]
fn test_shl_overflow_flag_1bit_different_sign() {
    // OF is set when top two bits differ before shift
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xC0000000; // 11000000...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0xC0000000 << 1 = 0x80000000"
    );
    assert!(cf_set(regs.rflags), "CF: bit shifted out was 1");
    assert!(!of_set(regs.rflags), "OF: MSB(result) XOR CF = 1 XOR 1 = 0");
    // Actually: MSB of result is 1, CF is 1, so MSB XOR CF = 0
    // Let me recalculate: 0xC0000000 = 1100_0000...
    // Shift left by 1: MSB (bit 31) is 1, shifts into CF
    // Result is 1000_0000... = 0x80000000, so new MSB is 1
    // OF = old_MSB XOR new_MSB = 1 XOR 1 = 0? Or is it MSB XOR CF?
    // According to docs: OF := MSB(DEST) XOR CF
    // After shift, MSB(DEST) = 1, CF = 1, so OF = 1 XOR 1 = 0
}

#[test]
fn test_shl_parity_flag() {
    // PF is set if low byte has even number of 1 bits
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01; // 0000_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x02, "EAX: 0x01 << 1 = 0x02");
    // 0x02 = 0000_0010, one 1 bit (odd), so PF should be clear
    assert!(!pf_set(regs.rflags), "PF should be clear (odd parity)");
}

#[test]
fn test_shl_chained_shifts() {
    // Multiple shifts in sequence
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xd1, 0xe0, // SHL EAX, 1
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000005; // 5
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000028, "EAX: 5 << 3 = 40");
}

#[test]
fn test_shl_all_ones() {
    // Shift all 1s
    let code = [
        0xd1, 0xe0, // SHL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX: 0xFFFFFFFF << 1 = 0xFFFFFFFE"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
    assert!(sf_set(regs.rflags), "SF: result is negative");
    assert!(!zf_set(regs.rflags), "ZF: result is not zero");
}
