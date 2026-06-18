use crate::common::*;
use rax::cpu::{Registers, VCpu};

// IDIV — Signed Divide
//
// Opcodes:
// - F6 /7       IDIV r/m8      AL := AX / r/m8; AH := AX % r/m8 (signed)
// - F7 /7       IDIV r/m16     AX := DX:AX / r/m16; DX := DX:AX % r/m16 (signed)
// - F7 /7       IDIV r/m32     EAX := EDX:EAX / r/m32; EDX := EDX:EAX % r/m32 (signed)
// - REX.W+F7 /7 IDIV r/m64     RAX := RDX:RAX / r/m64; RDX := RDX:RAX % r/m64 (signed)
//
// Operation: dividend / divisor = quotient, remainder (all signed)
//
// Flags: Undefined (not set by IDIV)
//
// Exceptions:
// - #DE (Divide Error): if divisor is 0 or quotient doesn't fit
//
// CRITICAL: IDIV works with SIGNED integers (two's complement).
// The dividend must be sign-extended into the upper register:
// - For 8-bit:  sign-extend AL into AH (use CBW instruction)
// - For 16-bit: sign-extend AX into DX (use CWD instruction)
// - For 32-bit: sign-extend EAX into EDX (use CDQ instruction)
// - For 64-bit: sign-extend RAX into RDX (use CQO instruction)

// ============================================================================
// 8-bit IDIV (opcode F6 /7)
// ============================================================================

#[test]
fn test_idiv_al_positive() {
    // 100 / 10 = 10 remainder 0 (both positive)
    // Need to sign-extend AL into AH: CBW (0x98)
    let code = [
        0x66, 0x98, // CBW (sign-extend AL to AX) - needs 0x66 in 64-bit mode
        0xf6, 0xfb, // IDIV BL (F6 /7)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 100; // AL = 100
    regs.rbx = 10; // BL = 10
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 10, "AL (quotient) = 100 / 10 = 10");
    assert_eq!((regs.rax >> 8) & 0xFF, 0, "AH (remainder) = 0");
}

#[test]
fn test_idiv_al_negative_dividend() {
    // -100 / 10 = -10 remainder 0
    // -100 in two's complement (i8) = 0x9C
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-100i32) as u64 & 0xFF; // AL = 0x9C (-100 in i8)
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = (regs.rax & 0xFF) as i8;
    assert_eq!(quotient, -10, "-100 / 10 = -10");
}

#[test]
fn test_idiv_al_negative_divisor() {
    // 100 / -10 = -10 remainder 0
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = (-10i32) as u64 & 0xFF; // BL = 0xF6 (-10 in i8)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = (regs.rax & 0xFF) as i8;
    assert_eq!(quotient, -10, "100 / -10 = -10");
}

#[test]
fn test_idiv_al_both_negative() {
    // -100 / -10 = 10 remainder 0
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-100i32) as u64 & 0xFF;
    regs.rbx = (-10i32) as u64 & 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = (regs.rax & 0xFF) as i8;
    assert_eq!(quotient, 10, "-100 / -10 = 10");
}

#[test]
fn test_idiv_al_with_remainder() {
    // 100 / 7 = 14 remainder 2
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 14, "AL (quotient)");
    assert_eq!((regs.rax >> 8) & 0xFF, 2, "AH (remainder)");
}

#[test]
fn test_idiv_al_negative_with_remainder() {
    // -100 / 7 = -14 remainder -2 (in two's complement, remainder has same sign as dividend)
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-100i32) as u64 & 0xFF;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = (regs.rax & 0xFF) as i8;
    let remainder = ((regs.rax >> 8) & 0xFF) as i8;
    assert_eq!(quotient, -14, "Quotient");
    assert_eq!(remainder, -2, "Remainder");
}

// ============================================================================
// 16-bit IDIV (opcode F7 /7 with 0x66 prefix)
// ============================================================================

#[test]
fn test_idiv_ax_positive() {
    // 1000 / 10 = 100 remainder 0
    // Need CWD (0x99) to sign-extend AX into DX
    let code = [
        0x66, 0x99, // CWD (sign-extend AX to DX:AX) - needs 0x66 in 64-bit mode
        0x66, 0xf7, 0xfb, // IDIV BX (66 F7 /7)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 100, "AX (quotient)");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX (remainder)");
}

#[test]
fn test_idiv_ax_negative_dividend() {
    // -1000 / 10 = -100 remainder 0
    let code = [
        0x66, 0x99, // CWD (needs 0x66 in 64-bit mode)
        0x66, 0xf7, 0xfb, // IDIV BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000i32) as u64 & 0xFFFF;
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = (regs.rax & 0xFFFF) as i16;
    assert_eq!(quotient, -100, "-1000 / 10 = -100");
}

#[test]
fn test_idiv_ax_with_remainder() {
    // 1000 / 7 = 142 remainder 6
    let code = [
        0x66, 0x99, // CWD (needs 0x66 in 64-bit mode)
        0x66, 0xf7, 0xfb, // IDIV BX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 142, "AX (quotient)");
    assert_eq!(regs.rdx & 0xFFFF, 6, "DX (remainder)");
}

// ============================================================================
// 32-bit IDIV (opcode F7 /7)
// ============================================================================

#[test]
fn test_idiv_eax_positive() {
    // 1000000 / 1000 = 1000 remainder 0
    // CDQ (0x99) sign-extends EAX into EDX (in 32-bit context)
    let code = [
        0x99, // CDQ (sign-extend EAX to EDX:EAX in 32-bit mode)
        0xf7, 0xfb, // IDIV EBX (F7 /7)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000, "EAX (quotient)");
    assert_eq!(regs.rdx, 0, "EDX (remainder)");
}

#[test]
fn test_idiv_eax_negative_dividend() {
    // -1000000 / 1000 = -1000 remainder 0
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000i32) as u64;
    regs.rbx = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i32;
    assert_eq!(quotient, -1000, "-1000000 / 1000 = -1000");
}

#[test]
fn test_idiv_eax_negative_divisor() {
    // 1000000 / -1000 = -1000 remainder 0
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = (-1000i32) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i32;
    assert_eq!(quotient, -1000, "1000000 / -1000 = -1000");
}

#[test]
fn test_idiv_eax_both_negative() {
    // -1000000 / -1000 = 1000 remainder 0
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000i32) as u64;
    regs.rbx = (-1000i32) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i32;
    assert_eq!(quotient, 1000, "-1000000 / -1000 = 1000");
}

#[test]
fn test_idiv_eax_with_remainder() {
    // 1000000 / 7 = 142857 remainder 1
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 142857, "EAX (quotient)");
    assert_eq!(regs.rdx, 1, "EDX (remainder)");
}

#[test]
fn test_idiv_eax_negative_dividend_remainder() {
    // -1000000 / 7 = -142857 remainder -1
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000i32) as u64;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i32;
    let remainder = regs.rdx as i32;
    assert_eq!(quotient, -142857, "EAX (quotient)");
    assert_eq!(remainder, -1, "EDX (remainder)");
}

// ============================================================================
// 64-bit IDIV (opcode REX.W + F7 /7)
// ============================================================================

#[test]
fn test_idiv_rax_positive() {
    // 1000000000000 / 1000000 = 1000000 remainder 0
    // CQO (0x48 0x99) sign-extends RAX into RDX
    let code = [
        0x48, 0x99, // CQO (sign-extend RAX to RDX:RAX)
        0x48, 0xf7, 0xfb, // IDIV RBX (REX.W F7 /7)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000000000;
    regs.rbx = 1000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000000, "RAX (quotient)");
    assert_eq!(regs.rdx, 0, "RDX (remainder)");
}

#[test]
fn test_idiv_rax_negative_dividend() {
    // -1000000000000 / 1000000 = -1000000 remainder 0
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000000000i64) as u64;
    regs.rbx = 1000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i64;
    assert_eq!(quotient, -1000000, "-1000000000000 / 1000000 = -1000000");
}

#[test]
fn test_idiv_rax_negative_divisor() {
    // 1000000000000 / -1000000 = -1000000 remainder 0
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000000000;
    regs.rbx = (-1000000i64) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i64;
    assert_eq!(quotient, -1000000, "1000000000000 / -1000000 = -1000000");
}

#[test]
fn test_idiv_rax_both_negative() {
    // -1000000000000 / -1000000 = 1000000 remainder 0
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000000000i64) as u64;
    regs.rbx = (-1000000i64) as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i64;
    assert_eq!(quotient, 1000000, "-1000000000000 / -1000000 = 1000000");
}

#[test]
fn test_idiv_rax_with_remainder() {
    // 1000000000000 / 7 = 142857142857 remainder 1
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000000000;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 142857142857, "RAX (quotient)");
    assert_eq!(regs.rdx, 1, "RDX (remainder)");
}

#[test]
fn test_idiv_rax_negative_dividend_remainder() {
    // -1000000000000 / 7 = -142857142857 remainder -1
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-1000000000000i64) as u64;
    regs.rbx = 7;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i64;
    let remainder = regs.rdx as i64;
    assert_eq!(quotient, -142857142857, "RAX (quotient)");
    assert_eq!(remainder, -1, "RDX (remainder)");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_idiv_cl_register() {
    // IDIV CL (8-bit)
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xf9, // IDIV CL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 10, "AL (quotient)");
    assert_eq!((regs.rax >> 8) & 0xFF, 0, "AH (remainder)");
}

#[test]
fn test_idiv_ecx_32bit() {
    // IDIV ECX (32-bit)
    let code = [
        0x99, // CDQ
        0xf7, 0xf9, // IDIV ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rcx = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000, "EAX (quotient)");
    assert_eq!(regs.rdx, 0, "EDX (remainder)");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_idiv_r8b() {
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0x41, 0xf6, 0xf8, // IDIV R8B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.r8 = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 10, "AL (quotient)");
}

#[test]
fn test_idiv_r10d() {
    let code = [
        0x99, // CDQ
        0x41, 0xf7, 0xfa, // IDIV R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.r10 = 1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000, "EAX (quotient)");
}

#[test]
fn test_idiv_r15() {
    let code = [
        0x48, 0x99, // CQO
        0x49, 0xf7, 0xff, // IDIV R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000000000;
    regs.r15 = 1000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000000, "RAX (quotient)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_idiv_byte_ptr_mem() {
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0x3d, 0xf8, 0x0f, 0x00,
        0x00, // IDIV BYTE PTR [rip+0x0FF8] (DATA_ADDR=0x2000, RIP after=0x1008)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 10);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 100;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 10, "AL (quotient)");
}

#[test]
fn test_idiv_dword_ptr_mem() {
    let code = [
        0x99, // CDQ
        0xf7, 0x3d, 0xf9, 0x0f, 0x00,
        0x00, // IDIV DWORD PTR [rip+0x0FF9] (DATA_ADDR=0x2000, RIP after=0x1007)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1000);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 1000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000, "EAX (quotient)");
    assert_eq!(regs.rdx, 0, "EDX (remainder)");
}

#[test]
fn test_idiv_qword_ptr_mem() {
    let code = [
        0x48, 0x99, // CQO
        0x48, 0xf7, 0x3d, 0xf7, 0x0f, 0x00,
        0x00, // IDIV QWORD PTR [rip+0x0FF7] (DATA_ADDR=0x2000, RIP after=0x1009)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 1000000);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 1000000000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 1000000, "RAX (quotient)");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_idiv_small_dividend() {
    // 5 / 10 = 0 remainder 5
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "Quotient = 0");
    assert_eq!(regs.rdx, 5, "Remainder = 5");
}

#[test]
fn test_idiv_negative_small_dividend() {
    // -5 / 10 = 0 remainder -5
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = (-5i32) as u64;
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let quotient = regs.rax as i32;
    let remainder = regs.rdx as i32;
    assert_eq!(quotient, 0, "Quotient = 0");
    assert_eq!(remainder, -5, "Remainder = -5 (sign of dividend)");
}

#[test]
fn test_idiv_power_of_two() {
    // 1024 / 256 = 4 remainder 0
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1024;
    regs.rbx = 256;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4, "Quotient");
    assert_eq!(regs.rdx, 0, "Remainder");
}

#[test]
fn test_idiv_max_positive() {
    // 0x7FFFFFFF / 1 = 0x7FFFFFFF remainder 0 (max i32)
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    regs.rbx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7FFFFFFF, "Quotient = max i32");
    assert_eq!(regs.rdx, 0, "Remainder = 0");
}

// ============================================================================
// #DE (Divide Error, vector 0) regression tests
//
// These verify the emulator raises #DE instead of panicking the host process.
// We use `setup_vm_no_idt` so that exception delivery surfaces as an Err (no
// IDT entry is present for vector 0), mirroring the pattern in misc/ud.rs.
// A non-erroring run that reaches HLT means the guest divide either completed
// or panicked the host -- both are failures here.
// ============================================================================

/// Shared helper: single-step the VM and assert it raises #DE (vector 0).
///
/// With `setup_vm_no_idt` there is no IDT entry for vector 0, so exception
/// delivery fails with an error mentioning "IDT entry 0" -- this is how we
/// observe that the divide-error vector (0) was raised. Reaching HLT means the
/// divide wrongly completed; a host panic (the bug being fixed) would abort the
/// test process outright, which is also a failure.
fn assert_raises_de(vcpu: &mut rax::backend::emulator::x86_64::X86_64Vcpu) {
    let mut reached_hlt = false;
    let mut raised_de = false;
    for _ in 0..10 {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => {
                reached_hlt = true;
                break;
            }
            Ok(_) => continue,
            Err(e) => {
                let msg = format!("{e}");
                assert!(
                    msg.contains("IDT entry 0 "),
                    "expected #DE (vector 0) delivery, got: {msg}"
                );
                raised_de = true;
                break;
            }
        }
    }
    assert!(!reached_hlt, "divide must raise #DE, not complete");
    assert!(raised_de, "divide must raise #DE (vector 0)");
}

#[test]
fn test_idiv_eax_int_min_div_neg_one_raises_de() {
    // INT32_MIN (0x80000000) IDIV -1 overflows: |quotient| = 2^31 does not fit
    // in a signed 32-bit result. Must raise #DE (vector 0).
    let code = [
        0x99, // CDQ (sign-extend EAX into EDX => EDX:EAX = -2147483648)
        0xf7, 0xfb, // IDIV EBX
        0xf4, // HLT (must NOT be reached)
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000; // EAX = INT32_MIN
    regs.rbx = 0xFFFF_FFFF; // EBX = -1
    let (mut vcpu, _) = setup_vm_no_idt(&code, Some(regs));
    assert_raises_de(&mut vcpu);
}

#[test]
fn test_idiv_eax_dividend_i64_min_div_neg_one_no_host_panic() {
    // The genuine host-panic path: the 32-bit IDIV widens EDX:EAX to i64, and
    // i64::MIN / -1 overflows -- `dividend / divisor` would panic the whole
    // emulator process. The fix must turn this into a #DE (vector 0) instead.
    // EDX:EAX = 0x8000000000000000 = i64::MIN (set directly; CDQ cannot produce it).
    let code = [
        0xf7, 0xfb, // IDIV EBX
        0xf4, // HLT (must NOT be reached)
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000_0000; // EAX (low dword) = 0
    regs.rdx = 0x8000_0000; // EDX (high dword) = 0x80000000 => EDX:EAX = i64::MIN
    regs.rbx = 0xFFFF_FFFF; // EBX = -1
    let (mut vcpu, _) = setup_vm_no_idt(&code, Some(regs));
    assert_raises_de(&mut vcpu);
}

#[test]
fn test_idiv_rax_dividend_i128_min_div_neg_one_no_host_panic() {
    // 64-bit IDIV widens RDX:RAX to i128; i128::MIN / -1 would panic the host.
    // RDX:RAX = i128::MIN (RDX high bit set, rest zero). Must raise #DE.
    let code = [
        0x48, 0xf7, 0xfb, // IDIV RBX (REX.W F7 /7)
        0xf4, // HLT (must NOT be reached)
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rdx = 0x8000_0000_0000_0000; // RDX:RAX = i128::MIN
    regs.rbx = (-1i64) as u64; // RBX = -1
    let (mut vcpu, _) = setup_vm_no_idt(&code, Some(regs));
    assert_raises_de(&mut vcpu);
}

#[test]
fn test_idiv_eax_div_by_zero_raises_de() {
    // IDIV by zero must raise #DE (vector 0), not panic the host.
    let code = [
        0x99, // CDQ
        0xf7, 0xfb, // IDIV EBX  (EBX = 0)
        0xf4, // HLT (must NOT be reached)
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 0; // divisor == 0
    let (mut vcpu, _) = setup_vm_no_idt(&code, Some(regs));
    assert_raises_de(&mut vcpu);
}
