// ROR (Rotate Right) instruction tests
//
// Opcodes:
// D0 /1       ROR r/m8, 1
// D2 /1       ROR r/m8, CL
// C0 /1 ib    ROR r/m8, imm8
// D1 /1       ROR r/m16, 1
// D3 /1       ROR r/m16, CL
// C1 /1 ib    ROR r/m16, imm8
// D1 /1       ROR r/m32, 1
// D3 /1       ROR r/m32, CL
// C1 /1 ib    ROR r/m32, imm8
// REX.W + D1 /1    ROR r/m64, 1
// REX.W + D3 /1    ROR r/m64, CL
// REX.W + C1 /1 ib ROR r/m64, imm8
//
// ROR rotates bits right. LSB is shifted into MSB and CF.
// Unlike RCR, CF does not participate in the rotation (it only receives LSB).
//
// Flags:
// - CF: Receives LSB shifted out
// - OF: Only for 1-bit rotates (MSB XOR next-to-MSB of result)
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;
use std::sync::Arc;

// ============================================================================
// 8-bit ROR tests
// ============================================================================

#[test]
fn test_ror_al_1() {
    // ROR AL, 1 (opcode D0 /1)
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x42 ROR 1 = 0x21");
    assert!(!cf_set(regs.rflags), "CF: receives LSB (was 0)");
}

#[test]
fn test_ror_al_1_with_lsb() {
    // ROR AL, 1 with LSB set
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x43; // 0100_0011
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xA1,
        "AL: 0x43 ROR 1 = 0xA1 (LSB rotates to MSB)"
    );
    assert!(cf_set(regs.rflags), "CF: receives LSB (was 1)");
}

#[test]
fn test_ror_al_cl() {
    // ROR AL, CL (opcode D2 /1)
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 1000_0000
    regs.rcx = 0x04; // Rotate by 4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x08, "AL: 0x80 ROR 4 = 0x08");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_ror_al_imm8() {
    // ROR AL, imm8 (opcode C0 /1 ib)
    let code = [
        0xc0, 0xc8, 0x03, // ROR AL, 3
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11; // 0001_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x22, "AL: 0x11 ROR 3 = 0x22");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_ror_full_rotation_8bit() {
    // ROR by 8 should return to original value
    let code = [
        0xc0, 0xc8, 0x08, // ROR AL, 8
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
fn test_ror_count_zero_preserves_flags() {
    // Count of 0 should not affect flags
    let code = [
        0xc0, 0xc8, 0x00, // ROR AL, 0
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
// 16-bit ROR tests
// ============================================================================

#[test]
fn test_ror_ax_1() {
    // ROR AX, 1 (opcode 66 D1 /1)
    let code = [
        0x66, 0xd1, 0xc8, // ROR AX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xA190, "AX: 0x4321 ROR 1 = 0xA190");
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_ax_cl() {
    // ROR AX, CL (opcode 66 D3 /1)
    let code = [
        0x66, 0xd3, 0xc8, // ROR AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000;
    regs.rcx = 0x0F; // Rotate by 15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX: 0x8000 ROR 15 = 0x0001");
    assert!(!cf_set(regs.rflags), "CF: last bit rotated was 0");
}

#[test]
fn test_ror_ax_imm8() {
    // ROR AX, imm8 (opcode 66 C1 /1 ib)
    let code = [
        0x66, 0xc1, 0xc8, 0x04, // ROR AX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x4123, "AX: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_full_rotation_16bit() {
    // ROR by 16 should return to original value
    let code = [
        0x66, 0xc1, 0xc8, 0x10, // ROR AX, 16
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
// 32-bit ROR tests
// ============================================================================

#[test]
fn test_ror_eax_1() {
    // ROR EAX, 1 (opcode D1 /1)
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x091A2B3C,
        "EAX: 0x12345678 ROR 1 = 0x091A2B3C"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_ror_eax_cl() {
    // ROR EAX, CL (opcode D3 /1)
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rcx = 0x1F; // Rotate by 31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX: 0x80000000 ROR 31 = 0x00000001"
    );
}

#[test]
fn test_ror_eax_imm8() {
    // ROR EAX, imm8 (opcode C1 /1 ib)
    let code = [
        0xc1, 0xc8, 0x08, // ROR EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78123456,
        "EAX: 0x12345678 ROR 8 = 0x78123456"
    );
}

#[test]
fn test_ror_eax_with_lsb() {
    // ROR with LSB set
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000000,
        "EAX: 0x00000001 ROR 1 = 0x80000000"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_full_rotation_32bit() {
    // ROR by 32 should return to original value
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
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
// 64-bit ROR tests
// ============================================================================

#[test]
fn test_ror_rax_1() {
    // ROR RAX, 1 (opcode 48 D1 /1)
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x091A2B3C4D5E6F78,
        "RAX: 0x123456789ABCDEF0 ROR 1"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_ror_rax_cl() {
    // ROR RAX, CL (opcode 48 D3 /1)
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 0x3F; // Rotate by 63
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX: 0x8000000000000000 ROR 63"
    );
}

#[test]
fn test_ror_rax_imm8() {
    // ROR RAX, imm8 (opcode 48 C1 /1 ib)
    let code = [
        0x48, 0xc1, 0xc8, 0x10, // ROR RAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xDEF0123456789ABC,
        "RAX: 0x123456789ABCDEF0 ROR 16"
    );
}

#[test]
fn test_ror_rax_with_lsb() {
    // ROR with LSB set
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x8000000000000000,
        "RAX: 0x0000000000000001 ROR 1"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_full_rotation_64bit() {
    // ROR by 64 should return to original value
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
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
fn test_ror_r8b_1() {
    // ROR R8B, 1
    let code = [
        0x41, 0xd0, 0xc8, // ROR R8B, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA; // 1010_1010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: 0xAA ROR 1 = 0x55");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_ror_r10w_cl() {
    // ROR R10W, CL
    let code = [
        0x66, 0x41, 0xd3, 0xca, // ROR R10W, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1234;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x4123, "R10W: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_r12d_imm8() {
    // ROR R12D, imm8
    let code = [
        0x41, 0xc1, 0xcc, 0x08, // ROR R12D, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r12 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r12 & 0xFFFFFFFF,
        0x78123456,
        "R12D: 0x12345678 ROR 8 = 0x78123456"
    );
}

#[test]
fn test_ror_r15_1() {
    // ROR R15, 1
    let code = [
        0x49, 0xd1, 0xcf, // ROR R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0x7F6E5D4C3B2A1908,
        "R15: 0xFEDCBA9876543210 ROR 1"
    );
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_ror_byte_ptr_1() {
    // ROR byte ptr [DATA_ADDR], 1
    let code = [
        0xd0,
        0x0c,
        0x25, // ROR byte ptr [DATA_ADDR], 1
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

    assert_eq!(result, 0x21, "Memory: 0x42 ROR 1 = 0x21");
}

#[test]
fn test_ror_word_ptr_cl() {
    // ROR word ptr [DATA_ADDR], CL
    let code = [
        0x66,
        0xd3,
        0x0c,
        0x25, // ROR word ptr [DATA_ADDR], CL
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

    assert_eq!(result, 0x4123, "Memory: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_dword_ptr_imm8() {
    // ROR dword ptr [DATA_ADDR], imm8
    let code = [
        0xc1,
        0x0c,
        0x25, // ROR dword ptr [DATA_ADDR], imm8
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

    assert_eq!(result, 0x78123456, "Memory: 0x12345678 ROR 8 = 0x78123456");
}

#[test]
fn test_ror_qword_ptr_cl() {
    // ROR qword ptr [DATA_ADDR], CL
    let code = [
        0x48,
        0xd3,
        0x0c,
        0x25, // ROR qword ptr [DATA_ADDR], CL
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
        result, 0xDEF0123456789ABC,
        "Memory: 0x123456789ABCDEF0 ROR 16"
    );
}

// ============================================================================
// Practical use cases and edge cases
// ============================================================================

#[test]
fn test_ror_byte_swap_endianness() {
    // Swap byte order (combined with other operations)
    let code = [
        0xc1, 0xc8, 0x08, // ROR EAX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78123456, "EAX: bytes rotated");
}

#[test]
fn test_ror_overflow_flag_1bit() {
    // OF is set to MSB XOR next-to-MSB for 1-bit rotates
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001; // ...0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000000, "EAX: 0x00000001 ROR 1");
    // Result: 1000_0000..., MSB=1, next-to-MSB=0, OF = 1 XOR 0 = 1
    assert!(of_set(regs.rflags), "OF: MSB XOR next-to-MSB = 1");
}

#[test]
fn test_ror_bit_permutation() {
    // ROR can permute bits
    let code = [
        0xc0, 0xc8, 0x04, // ROR AL, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xF0; // 1111_0000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL: 0xF0 ROR 4 = 0x0F");
}

#[test]
fn test_ror_chained_rotations() {
    // Multiple ROR in sequence
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xd1, 0xc8, // ROR EAX, 1
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x02468ACF,
        "EAX: three 1-bit rotations"
    );
}

#[test]
fn test_ror_all_ones() {
    // Rotate all 1s
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
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
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_nibble_swap() {
    // Swap high and low nibbles
    let code = [
        0xc0, 0xc8, 0x04, // ROR AL, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: nibbles swapped");
}

#[test]
fn test_ror_extract_low_bits() {
    // ROR can help extract specific bit fields
    let code = [
        0xc1, 0xc8, 0x10, // ROR EAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12340000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00001234,
        "EAX: high word moved to low"
    );
}

#[test]
fn test_ror_alternating_bits() {
    // Test with alternating bit pattern
    let code = [
        0xd1, 0xc8, // ROR EAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA; // 1010_1010...
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55555555,
        "EAX: alternating bits rotated"
    );
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}
