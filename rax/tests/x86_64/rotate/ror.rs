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
// - OF: Only for 1-bit rotates (MSB XOR (MSB-1))
// - Other flags: Undefined
// - Count is 0: No flags affected

use crate::common::{cf_set, of_set, sf_set, zf_set};
use crate::common::{run_until_hlt, setup_vm};
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// ============================================================================
// 8-bit ROR tests
// ============================================================================

#[test]
fn test_ror_al_1() {
    // ROR AL, 1 (opcode D0 /1)
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x21, "AL: 0x42 ROR 1 = 0x21");
    assert!(!cf_set(regs.rflags), "CF: receives LSB (was 0)");
    assert!(!of_set(regs.rflags), "OF: MSB XOR (MSB-1) = 0 XOR 0 = 0");
}

#[test]
fn test_ror_al_1_with_lsb() {
    // ROR AL, 1 with LSB set
    let code = [
        0xd0, 0xc8, // ROR AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x81; // 1000_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFF,
        0xC0,
        "AL: 0x81 ROR 1 = 0xC0 (LSB rotates to MSB)"
    );
    assert!(cf_set(regs.rflags), "CF: receives LSB (was 1)");
    // OF = XOR of two most-significant bits of result: 0xC0 = 1100_0000, MSB=1, MSB-1=1, so 1 XOR 1 = 0
    assert!(!of_set(regs.rflags), "OF: MSB XOR (MSB-1) = 1 XOR 1 = 0");
}

#[test]
fn test_ror_al_cl() {
    // ROR AL, CL (opcode D2 /1)
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x10; // 0001_0000
    regs.rcx = 0x04; // Rotate by 4
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL: 0x10 ROR 4 = 0x01");
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
    regs.rax = 0x88; // 1000_1000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x11, "AL: 0x88 ROR 3 = 0x11");
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
fn test_ror_count_masked_8bit() {
    // Count is masked for 8-bit operands
    let code = [
        0xd2, 0xc8, // ROR AL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x88;
    regs.rcx = 0x1B; // 27 masked and modulo 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 27 % 8 = 3 for 8-bit operand
    assert_eq!(regs.rax & 0xFF, 0x11, "AL: rotation count masked");
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

    assert_eq!(regs.rax & 0xFF, 0x42, "AL: unchanged");
    assert_eq!(
        regs.rflags & (flags::bits::CF | flags::bits::ZF | flags::bits::OF),
        initial_flags & (flags::bits::CF | flags::bits::ZF | flags::bits::OF),
        "Flags preserved"
    );
}

#[test]
fn test_ror_bl() {
    // ROR BL, 1
    let code = [
        0xd0, 0xcb, // ROR BL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xC5; // 1100_0101
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0xE2, "BL: 0xC5 ROR 1 = 0xE2");
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_cl_reg() {
    // ROR CL, imm8
    let code = [
        0xc0, 0xc9, 0x02, // ROR CL, 2
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xCC; // 1100_1100
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0x33, "CL: 0xCC ROR 2 = 0x33");
}

#[test]
fn test_ror_dl() {
    // ROR DL, CL
    let code = [
        0xd2, 0xca, // ROR DL, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xF0;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFF, 0x0F, "DL: 0xF0 ROR 4 = 0x0F");
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
    regs.rax = 0x8642; // 1000_0110_0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x4321, "AX: 0x8642 ROR 1 = 0x4321");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_ror_ax_cl() {
    // ROR AX, CL (opcode 66 D3 /1)
    let code = [
        0x66, 0xd3, 0xc8, // ROR AX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rcx = 0x04;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x4123, "AX: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_ax_imm8() {
    // ROR AX, imm8 (opcode 66 C1 /1 ib)
    let code = [
        0x66, 0xc1, 0xc8, 0x08, // ROR AX, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x3412,
        "AX: 0x1234 ROR 8 = 0x3412 (byte swap)"
    );
}

#[test]
fn test_ror_ax_full_rotation() {
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

#[test]
fn test_ror_bx() {
    // ROR BX, 1
    let code = [
        0x66, 0xd1, 0xcb, // ROR BX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x8000, "BX: 0x0001 ROR 1 = 0x8000");
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_cx() {
    // ROR CX, imm8
    let code = [
        0x66, 0xc1, 0xc9, 0x04, // ROR CX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0xDABC, "CX: 0xABCD ROR 4 = 0xDABC");
}

#[test]
fn test_ror_dx_cl() {
    // ROR DX, CL
    let code = [
        0x66, 0xd3, 0xca, // ROR DX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFF00;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x00FF, "DX: 0xFF00 ROR 8 = 0x00FF");
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
    regs.rax = 0x86430ECA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x43218765,
        "EAX: 0x86430ECA ROR 1 = 0x43218765"
    );
}

#[test]
fn test_ror_eax_cl() {
    // ROR EAX, CL (opcode D3 /1)
    let code = [
        0xd3, 0xc8, // ROR EAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78123456,
        "EAX: 0x12345678 ROR 8 = 0x78123456"
    );
}

#[test]
fn test_ror_eax_imm8() {
    // ROR EAX, imm8 (opcode C1 /1 ib)
    let code = [
        0xc1, 0xc8, 0x10, // ROR EAX, 16
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x56781234,
        "EAX: 0x12345678 ROR 16 = 0x56781234"
    );
}

#[test]
fn test_ror_eax_full_rotation() {
    // ROR by 32 should return to original value
    let code = [
        0xc1, 0xc8, 0x20, // ROR EAX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX: full rotation returns to original"
    );
}

#[test]
fn test_ror_ebx() {
    // ROR EBX, 1
    let code = [
        0xd1, 0xcb, // ROR EBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x80000000,
        "EBX: 0x00000001 ROR 1 = 0x80000000"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_ecx() {
    // ROR ECX, imm8
    let code = [
        0xc1, 0xc9, 0x04, // ROR ECX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xABCDEF01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x1ABCDEF0,
        "ECX: 0xABCDEF01 ROR 4 = 0x1ABCDEF0"
    );
}

#[test]
fn test_ror_edx_cl() {
    // ROR EDX, CL
    let code = [
        0xd3, 0xca, // ROR EDX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFF000000;
    regs.rcx = 0x18; // 24 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x000000FF,
        "EDX: 0xFF000000 ROR 24 = 0x000000FF"
    );
}

#[test]
fn test_ror_esi() {
    // ROR ESI, 1
    let code = [
        0xd1, 0xce, // ROR ESI, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x40000000,
        "ESI: 0x80000000 ROR 1 = 0x40000000"
    );
}

#[test]
fn test_ror_edi() {
    // ROR EDI, imm8
    let code = [
        0xc1, 0xcf, 0x0C, // ROR EDI, 12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdi & 0xFFFFFFFF,
        0x67812345,
        "EDI: 0x12345678 ROR 12 = 0x67812345"
    );
}

// ============================================================================
// 64-bit ROR tests
// ============================================================================

#[test]
fn test_ror_rax_1() {
    // ROR RAX, 1 (opcode REX.W D1 /1)
    let code = [
        0x48, 0xd1, 0xc8, // ROR RAX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x86430ECA86430ECA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x4321876543218765, "RAX: ROR 1");
}

#[test]
fn test_ror_rax_cl() {
    // ROR RAX, CL (opcode REX.W D3 /1)
    let code = [
        0x48, 0xd3, 0xc8, // ROR RAX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rcx = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xF0123456789ABCDE,
        "RAX: 0x123456789ABCDEF0 ROR 8"
    );
}

#[test]
fn test_ror_rax_imm8() {
    // ROR RAX, imm8 (opcode REX.W C1 /1 ib)
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
fn test_ror_rax_32bits() {
    // ROR RAX, 32 (half rotation)
    let code = [
        0x48, 0xc1, 0xc8, 0x20, // ROR RAX, 32
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x9ABCDEF012345678,
        "RAX: ROR 32 swaps high/low dwords"
    );
}

#[test]
fn test_ror_rax_full_rotation() {
    // ROR by 64 should return to original value
    let code = [
        0x48, 0xc1, 0xc8, 0x40, // ROR RAX, 64
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX: full rotation returns to original"
    );
}

#[test]
fn test_ror_rbx() {
    // ROR RBX, 1
    let code = [
        0x48, 0xd1, 0xcb, // ROR RBX, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, 0x8000000000000000,
        "RBX: 0x0000000000000001 ROR 1 = 0x8000000000000000"
    );
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_rcx() {
    // ROR RCX, imm8
    let code = [
        0x48, 0xc1, 0xc9, 0x04, // ROR RCX, 4
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xABCDEF0123456789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x9ABCDEF012345678, "RCX: ROR 4");
}

#[test]
fn test_ror_rdx_cl() {
    // ROR RDX, CL
    let code = [
        0x48, 0xd3, 0xca, // ROR RDX, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xFF00000000000000;
    regs.rcx = 0x38; // 56 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x00000000000000FF, "RDX: ROR 56");
}

#[test]
fn test_ror_rsi() {
    // ROR RSI, 1
    let code = [
        0x48, 0xd1, 0xce, // ROR RSI, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x8000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x4000000000000000, "RSI: ROR 1");
}

#[test]
fn test_ror_rdi() {
    // ROR RDI, imm8
    let code = [
        0x48, 0xc1, 0xcf, 0x0C, // ROR RDI, 12
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0xEF0123456789ABCD, "RDI: ROR 12");
}

#[test]
fn test_ror_r8() {
    // ROR R8, 1 (REX.WB D1 /1)
    let code = [
        0x49, 0xd1, 0xc8, // ROR R8, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x7F6E5D4C3B2A1908, "R8: ROR 1");
    assert!(!cf_set(regs.rflags), "CF: LSB was 0");
}

#[test]
fn test_ror_r9_cl() {
    // ROR R9, CL
    let code = [
        0x49, 0xd3, 0xc9, // ROR R9, CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x0123456789ABCDEF;
    regs.rcx = 0x10; // 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9, 0xCDEF0123456789AB, "R9: ROR 16");
}

#[test]
fn test_ror_r10_imm8() {
    // ROR R10, imm8
    let code = [
        0x49, 0xc1, 0xca, 0x08, // ROR R10, 8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0xF0123456789ABCDE, "R10: ROR 8");
}

#[test]
fn test_ror_r15() {
    // ROR R15, 1
    let code = [
        0x49, 0xd1, 0xcf, // ROR R15, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x1111111111111111, "R15: ROR 1");
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_ror_mem8() {
    use crate::common::{DATA_ADDR, read_mem_u8, write_mem_u8};

    // ROR byte [DATA_ADDR], 1
    let code = [
        0xd0,
        0x0c,
        0x25, // ROR byte ptr [disp32], 1
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x81);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_u8(&mem), 0xC0, "Memory: 0x81 ROR 1 = 0xC0");
    assert!(cf_set(regs.rflags), "CF: LSB was 1");
}

#[test]
fn test_ror_mem16() {
    use crate::common::{DATA_ADDR, read_mem_u16, write_mem_u16};

    // ROR word [DATA_ADDR], 4
    let code = [
        0x66,
        0xc1,
        0x0c,
        0x25, // ROR word ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x04, // imm8: 4
        0xf4,
    ];
    let regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_u16(&mem), 0x4123, "Memory: 0x1234 ROR 4 = 0x4123");
}

#[test]
fn test_ror_mem32() {
    use crate::common::{DATA_ADDR, read_mem_u32, write_mem_u32};

    // ROR dword [DATA_ADDR], CL
    let code = [
        0xd3,
        0x0c,
        0x25, // ROR dword ptr [disp32], CL
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x08;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x78123456,
        "Memory: 0x12345678 ROR 8 = 0x78123456"
    );
}

#[test]
fn test_ror_mem64() {
    use crate::common::{DATA_ADDR, read_mem_u64, write_mem_u64};

    // ROR qword [DATA_ADDR], 16
    let code = [
        0x48,
        0xc1,
        0x0c,
        0x25, // ROR qword ptr [disp32], imm8
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x10, // imm8: 16
        0xf4,
    ];
    let regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_u64(&mem), 0xDEF0123456789ABC, "Memory: ROR 16");
}
