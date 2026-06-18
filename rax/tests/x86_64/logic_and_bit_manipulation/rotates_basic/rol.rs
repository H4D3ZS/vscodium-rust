// ROL (Rotate Left) instruction tests
//
// Opcodes:
// D0 /0       ROL r/m8, 1
// D2 /0       ROL r/m8, CL
// C0 /0 ib    ROL r/m8, imm8
// D1 /0       ROL r/m16, 1
// D3 /0       ROL r/m16, CL
// C1 /0 ib    ROL r/m16, imm8
// D1 /0       ROL r/m32, 1
// D3 /0       ROL r/m32, CL
// C1 /0 ib    ROL r/m32, imm8
// REX.W + D1 /0    ROL r/m64, 1
// REX.W + D3 /0    ROL r/m64, CL
// REX.W + C1 /0 ib ROL r/m64, imm8
//
// ROL rotates bits left. MSB is shifted into LSB and CF.
// Unlike RCL, CF does not participate in the rotation (it only receives MSB).
//
// Flags:
// - CF: Receives MSB shifted out
// - OF: Only for 1-bit rotates (CF XOR new MSB)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 8-bit ROL tests
// ============================================================================

#[test]
fn test_rol_al_1() {
    // ROL AL, 1 (opcode D0 /0)
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x84, "AL: 0x42 ROL 1 = 0x84");
    assert!(!cf_set(regs.rflags), "CF: receives MSB (was 0)");
    assert!(of_set(regs.rflags), "OF: CF XOR new MSB = 0 XOR 1 = 1");
}

#[test]
fn test_rol_al_1_with_msb() {
    // ROL AL, 1 with MSB set
    let code = [
        0xd0, 0xc0, // ROL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x81; // 1000_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x03,
        "AL: 0x81 ROL 1 = 0x03 (MSB rotates to LSB)"
    );
    assert!(cf_set(regs.rflags), "CF: receives MSB (was 1)");
}

#[test]
fn test_rol_al_cl() {
    // ROL AL, CL (opcode D2 /0)
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x01; // 0000_0001
    regs.rcx = 0x04; // Rotate by 4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "AL: 0x01 ROL 4 = 0x10");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_rol_al_imm8() {
    // ROL AL, imm8 (opcode C0 /0 ib)
    let code = [
        0xc0, 0xc0, 0x03, // ROL AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11; // 0001_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "AL: 0x11 ROL 3 = 0x88");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_rol_full_rotation_8bit() {
    // ROL by 8 should return to original value
    let code = [
        0xc0, 0xc0, 0x08, // ROL AL, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0x42,
        "AL: full rotation returns to original"
    );
}

#[test]
fn test_rol_count_masked_8bit() {
    // Count is masked to 5 bits for 8-bit operands
    let code = [
        0xd2, 0xc0, // ROL AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11;
    regs.rcx = 0x1B; // 27 & 0x1F = 27, but for 8-bit it's mod 9 in some CPUs
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Count masking for ROL is 5 bits, but modulo size for effect
    // 27 % 8 = 3 for 8-bit operand
    assert_eq!(regs.rax & 0xFF, 0x88, "AL: rotation count masked");
}

#[test]
fn test_rol_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xc0, 0x00, // ROL AL, 0
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
// 16-bit ROL tests
// ============================================================================

#[test]
fn test_rol_ax_1() {
    // ROL AX, 1 (opcode 66 D1 /0)
    let code = [
        0x66, 0xd1, 0xc0, // ROL AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8642, "AX: 0x4321 ROL 1 = 0x8642");
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rol_ax_cl() {
    // ROL AX, CL (opcode 66 D3 /0)
    let code = [
        0x66, 0xd3, 0xc0, // ROL AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    regs.rcx = 0x0F; // Rotate by 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: 0x0001 ROL 15 = 0x8000");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_rol_ax_imm8() {
    // ROL AX, imm8 (opcode 66 C1 /0 ib)
    let code = [
        0x66, 0xc1, 0xc0, 0x04, // ROL AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1230, "AX: 0x0123 ROL 4 = 0x1230");
}

#[test]
fn test_rol_full_rotation_16bit() {
    // ROL by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc0, 0x10, // ROL AX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x1234,
        "AX: full rotation returns to original"
    );
}

// ============================================================================
// 32-bit ROL tests
// ============================================================================

#[test]
fn test_rol_eax_1() {
    // ROL EAX, 1 (opcode D1 /0)
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x2468ACF0,
        "EAX: 0x12345678 ROL 1 = 0x2468ACF0"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rol_eax_cl() {
    // ROL EAX, CL (opcode D3 /0)
    let code = [
        0xd3, 0xc0, // ROL EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rcx = 0x1F; // Rotate by 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0x00000001 ROL 31 = 0x80000000"
    );
}

#[test]
fn test_rol_eax_imm8() {
    // ROL EAX, imm8 (opcode C1 /0 ib)
    let code = [
        0xc1, 0xc0, 0x08, // ROL EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x34567812,
        "EAX: 0x12345678 ROL 8 = 0x34567812"
    );
}

#[test]
fn test_rol_eax_with_msb() {
    // ROL with MSB set
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000003,
        "EAX: 0x80000001 ROL 1 = 0x00000003"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
}

#[test]
fn test_rol_full_rotation_32bit() {
    // ROL by 32 should return to original value
    let code = [
        0xd3, 0xc0, // ROL EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0x20; // Rotate by 32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX: full rotation returns to original"
    );
}

// ============================================================================
// 64-bit ROL tests
// ============================================================================

#[test]
fn test_rol_rax_1() {
    // ROL RAX, 1 (opcode 48 D1 /0)
    let code = [
        0x48, 0xd1, 0xc0, // ROL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x2468ACF13579BDE0,
        "RAX: 0x123456789ABCDEF0 ROL 1"
    );
    assert!(!cf_set(regs.rflags), "CF: MSB was 0");
}

#[test]
fn test_rol_rax_cl() {
    // ROL RAX, CL (opcode 48 D3 /0)
    let code = [
        0x48, 0xd3, 0xc0, // ROL RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rcx = 0x3F; // Rotate by 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX: 0x0000000000000001 ROL 63"
    );
}

#[test]
fn test_rol_rax_imm8() {
    // ROL RAX, imm8 (opcode 48 C1 /0 ib)
    let code = [
        0x48, 0xc1, 0xc0, 0x10, // ROL RAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x56789ABCDEF01234,
        "RAX: 0x123456789ABCDEF0 ROL 16"
    );
}

#[test]
fn test_rol_rax_with_msb() {
    // ROL with MSB set
    let code = [
        0x48, 0xd1, 0xc0, // ROL RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000003,
        "RAX: 0x8000000000000001 ROL 1"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
}

#[test]
fn test_rol_full_rotation_64bit() {
    // ROL by 64 should return to original value
    let code = [
        0x48, 0xd3, 0xc0, // ROL RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rcx = 0x40; // Rotate by 64
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX: full rotation returns to original"
    );
}

// ============================================================================
// Extended register tests (R8-R15)
// ============================================================================

#[test]
fn test_rol_r8b_1() {
    // ROL R8B, 1
    let code = [
        0x41, 0xd0, 0xc0, // ROL R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x55; // 0101_0101
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0xAA, "R8B: 0x55 ROL 1 = 0xAA");
}

#[test]
fn test_rol_r10w_cl() {
    // ROL R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xc2, // ROL R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1234;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x2341, "R10W: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_r12d_imm8() {
    // ROL R12D, imm8
    let code = [
        0x41, 0xc1, 0xc4, 0x08, // ROL R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0x34567812,
        "R12D: 0x12345678 ROL 8 = 0x34567812"
    );
}

#[test]
fn test_rol_r15_1() {
    // ROL R15, 1
    let code = [
        0x49, 0xd1, 0xc7, // ROL R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0123456789ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x02468ACF13579BDE,
        "R15: 0x0123456789ABCDEF ROL 1"
    );
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_rol_byte_ptr_1() {
    // ROL byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x04,
        0x25, // ROL byte ptr [DATA_ADDR], 1
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

    assert_eq!(result, 0x84, "Memory: 0x42 ROL 1 = 0x84");
}

#[test]
fn test_rol_word_ptr_cl() {
    // ROL word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x04,
        0x25, // ROL word ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0x2341, "Memory: 0x1234 ROL 4 = 0x2341");
}

#[test]
fn test_rol_dword_ptr_imm8() {
    // ROL dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x04,
        0x25, // ROL dword ptr [DATA_ADDR], imm8
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

    assert_eq!(result, 0x34567812, "Memory: 0x12345678 ROL 8 = 0x34567812");
}

#[test]
fn test_rol_qword_ptr_cl() {
    // ROL qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x04,
        0x25, // ROL qword ptr [DATA_ADDR], CL
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
        result, 0x56789ABCDEF01234,
        "Memory: 0x123456789ABCDEF0 ROL 16"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_rol_bit_permutation() {
    // ROL can permute bits
    let code = [
        0xc0, 0xc0, 0x04, // ROL AL, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF0; // 1111_0000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL: 0xF0 ROL 4 = 0x0F");
}

#[test]
fn test_rol_overflow_flag_1bit_same() {
    // OF is clear when MSB doesn't change after rotation
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x40000000; // 0100...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x40000000 ROL 1");
    // Old MSB = 0, new MSB = 1, OF = 0 XOR 1 = 1
    assert!(of_set(regs.rflags), "OF: MSB changed from 0 to 1");
}

#[test]
fn test_rol_overflow_flag_1bit_different() {
    // OF when MSB changes
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xC0000000; // 1100...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000001, "EAX: 0xC0000000 ROL 1");
    // Old MSB = 1, new MSB = 1, CF = 1, OF = 1 XOR 1 = 0
    assert!(!of_set(regs.rflags), "OF: MSB stayed the same");
}

#[test]
fn test_rol_circular_buffer_indexing() {
    // ROL can implement circular buffer indexing
    let code = [
        0xc1, 0xc0, 0x03, // ROL EAX, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x91A2B3C0, "EAX: rotated by 3 bits");
}

#[test]
fn test_rol_chained_rotations() {
    // Multiple ROL in sequence
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xd1, 0xc0, // ROL EAX, 1
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x91A2B3C0,
        "EAX: three 1-bit rotations"
    );
}

#[test]
fn test_rol_all_ones() {
    // Rotate all 1s
    let code = [
        0xd1, 0xc0, // ROL EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX: all ones stay all ones"
    );
    assert!(cf_set(regs.rflags), "CF: MSB was 1");
}

#[test]
fn test_rol_byte_swap_high_low() {
    // Swap high and low nibbles
    let code = [
        0xc0, 0xc0, 0x04, // ROL AL, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: nibbles swapped");
}
