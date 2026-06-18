use crate::common::*;

// XADD - Exchange and Add
// Opcode: 0F C0 (8-bit), 0F C1 (16/32/64-bit)
//
// Operand 1 (Destination): r/m (register or memory)
// Operand 2 (Source): r (register)
//
// Semantics:
//   temp = src + dest
//   src = dest
//   dest = temp
//
// Flags: Sets CF, PF, AF, ZF, SF, OF based on the addition result

// ===== BASIC 8-BIT REGISTER-REGISTER TESTS =====

#[test]
fn test_xadd_8bit_reg_reg_basic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (BL = 5)
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (CL = 3)
        0x0f, 0xc0, 0xcb, // XADD BL, CL (BL gets 8, CL gets 5)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 8, "BL should be 8 (sum)");
    assert_eq!(regs.rcx & 0xFF, 5, "CL should be 5 (old BL)");
}

#[test]
fn test_xadd_8bit_zero_plus_value() {
    let code = [
        0x48, 0x31, 0xdb, // XOR RBX, RBX (BL = 0)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0x0f, 0xc0, 0xcb, // XADD BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x42, "BL should be 0x42");
    assert_eq!(regs.rcx & 0xFF, 0, "CL should be 0 (old BL)");
}

#[test]
fn test_xadd_8bit_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc0, 0xcb, // XADD BL, CL (0xFF + 1 = 0x100, wraps to 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0, "BL should be 0 (wrapped)");
    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL should be 0xFF (old BL)");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set (overflow)");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set (result is 0)");
}

#[test]
fn test_xadd_8bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x0f, 0xc0, 0xcb, // XADD BL, CL (0xFF + 0xFF = 0x1FE, wraps to 0xFE)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0xFE, "BL should be 0xFE");
    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL should be 0xFF");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_8bit_signed_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0x7f, 0x00, 0x00, 0x00, // MOV RBX, 0x7F (max positive)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc0, 0xcb, // XADD BL, CL (0x7F + 1 = 0x80, overflow to negative)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x80, "BL should be 0x80");
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set (signed overflow)");
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set (negative result)");
}

// ===== 16-BIT REGISTER-REGISTER TESTS =====

#[test]
fn test_xadd_16bit_reg_reg_basic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0x3000, "BX should be 0x3000");
    assert_eq!(regs.rcx & 0xFFFF, 0x1000, "CX should be 0x1000 (old BX)");
}

#[test]
fn test_xadd_16bit_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0x00, 0x00, // MOV RBX, 0xFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0, "BX should be 0 (wrapped)");
    assert_eq!(regs.rcx & 0xFFFF, 0xFFFF, "CX should be 0xFFFF");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_xadd_16bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0x00, 0x00, // MOV RBX, 0xFFFF
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0x00, 0x00, // MOV RCX, 0xFFFF
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0xFFFE, "BX should be 0xFFFE");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_16bit_signed_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0x7f, 0x00, 0x00, // MOV RBX, 0x7FFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0x8000, "BX should be 0x8000");
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set");
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set");
}

// ===== 32-BIT REGISTER-REGISTER TESTS =====

#[test]
fn test_xadd_32bit_reg_reg_basic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x10, // MOV RBX, 0x10000000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x20, // MOV RCX, 0x20000000
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x30000000,
        "EBX should be 0x30000000"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x10000000,
        "ECX should be 0x10000000"
    );
}

#[test]
fn test_xadd_32bit_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "ECX should be 0xFFFFFFFF"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_xadd_32bit_zeros_upper() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0x89, 0xc3, // MOV RBX, RAX (RBX = 0xFFFFFFFFFFFFFFFF)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (32-bit operation)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0, "RBX upper 32 bits should be zeroed");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "ECX should be 0xFFFFFFFF"
    );
}

#[test]
fn test_xadd_32bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EBX should be 0xFFFFFFFE"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_32bit_signed_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0x7f, // MOV RBX, 0x7FFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x80000000,
        "EBX should be 0x80000000"
    );
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set");
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set");
}

// ===== 64-BIT REGISTER-REGISTER TESTS =====

#[test]
fn test_xadd_64bit_reg_reg_basic() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x100000000
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 0x200000000
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x300000000, "RBX should be 0x300000000");
    assert_eq!(regs.rcx, 0x100000000, "RCX should be 0x100000000");
}

#[test]
fn test_xadd_64bit_overflow() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_eq!(
        regs.rcx, 0xFFFFFFFFFFFFFFFF,
        "RCX should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_xadd_64bit_max_values() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0xFFFFFFFFFFFFFFFE,
        "RBX should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_64bit_signed_overflow() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0x8000000000000000,
        "RBX should be 0x8000000000000000"
    );
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set");
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set");
}

// ===== FLAG BEHAVIOR TESTS =====

#[test]
fn test_xadd_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 1, "EBX should be 1");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_zero_flag() {
    let code = [
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0xc7, 0xc1, 0xfb, 0xff, 0xff, 0xff, // MOV RCX, -5 (0xFFFFFFFB)
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (5 + -5 = 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_xadd_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x80, // MOV RBX, 0x80000000 (negative)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x80000001,
        "EBX should be 0x80000001"
    );
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set (negative)");
}

#[test]
fn test_xadd_parity_flag_even() {
    let code = [
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (result 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 3, "EBX should be 3");
    // 3 = 0b11 has 2 set bits = even parity, so PF = 1
    assert_eq!(regs.rflags & 0x04, 0x04, "PF should be set (even parity)");
}

#[test]
fn test_xadd_parity_flag_odd() {
    // To get odd parity, result should have an odd number of set bits
    // Use 2 + 2 = 4 = 0b100, which has 1 set bit (odd parity)
    let code = [
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (result 4)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 4, "EBX should be 4");
    // 4 = 0b100 has 1 set bit = odd parity, so PF = 0
    assert_eq!(regs.rflags & 0x04, 0, "PF should be clear (odd parity)");
}

#[test]
fn test_xadd_auxiliary_flag() {
    let code = [
        0x48, 0xc7, 0xc3, 0x0f, 0x00, 0x00, 0x00, // MOV RBX, 0x0F
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (0x0F + 1 = 0x10, AF set)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x10, "EBX should be 0x10");
    assert_ne!(regs.rflags & 0x10, 0, "AF should be set");
}

#[test]
fn test_xadd_overflow_flag_set() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0x7f, // MOV RBX, 0x7FFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set");
}

#[test]
fn test_xadd_overflow_flag_clear() {
    let code = [
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 0x10
        0x48, 0xc7, 0xc1, 0x20, 0x00, 0x00, 0x00, // MOV RCX, 0x20
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & 0x800, 0, "OF should be clear");
}

// ===== DIFFERENT REGISTER COMBINATIONS =====

#[test]
fn test_xadd_rax_rdx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xc7, 0xc2, 0x22, 0x22, 0x22, 0x22, // MOV RDX, 0x22222222
        0x48, 0x0f, 0xc1, 0xd0, // XADD RAX, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x33333333, "RAX should be 0x33333333");
    assert_eq!(regs.rdx, 0x11111111, "RDX should be 0x11111111");
}

#[test]
fn test_xadd_rsi_rdi() {
    let code = [
        0x48, 0xc7, 0xc6, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RSI, 0xAAAAAAAA
        0x48, 0xc7, 0xc7, 0x55, 0x55, 0x55, 0x55, // MOV RDI, 0x55555555
        0x48, 0x0f, 0xc1, 0xfe, // XADD RSI, RDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends: 0xAAAAAAAA -> 0xFFFFFFFFAAAAAAAA
    // Sum = 0xFFFFFFFFAAAAAAAA + 0x55555555 = 0xFFFFFFFFFFFFFFFF
    assert_eq!(
        regs.rsi, 0xFFFFFFFFFFFFFFFFu64,
        "RSI should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(
        regs.rdi, 0xFFFFFFFFAAAAAAAAu64,
        "RDI should be old RSI value"
    );
}

#[test]
fn test_xadd_r8_r9() {
    let code = [
        0x49, 0xc7, 0xc0, 0x12, 0x34, 0x56, 0x78, // MOV R8, 0x78563412
        0x49, 0xc7, 0xc1, 0x9a, 0xbc, 0xde, 0xf0, // MOV R9, 0xF0DEBC9A
        0x4d, 0x0f, 0xc1, 0xc8, // XADD R8, R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends: 0xF0DEBC9A -> 0xFFFFFFFFF0DEBC9A
    // 64-bit sum: 0x78563412 + 0xFFFFFFFFF0DEBC9A = 0x6934F0AC (wraps)
    assert_eq!(regs.r8, 0x6934F0AC, "R8 should be 64-bit sum");
    assert_eq!(regs.r9, 0x78563412, "R9 should be old R8");
}

// ===== EDGE CASE TESTS =====

#[test]
fn test_xadd_same_register() {
    // XADD with same register doubles the value
    let code = [
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x0f, 0xc1, 0xdb, // XADD EBX, EBX (should double)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 10, "EBX should be 10 (doubled)");
}

#[test]
fn test_xadd_with_negative_values() {
    let code = [
        0x48, 0xc7, 0xc3, 0xf0, 0xff, 0xff, 0xff, // MOV RBX, -16 (0xFFFFFFF0)
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xFFFFFFFA, "EBX should be -6");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xFFFFFFF0, "ECX should be -16");
}

#[test]
fn test_xadd_large_values_64bit() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, // MOV RAX, 0x4000000000000000
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0xC000000000000000,
        "RBX should be 0xC000000000000000"
    );
    assert_eq!(
        regs.rcx, 0x8000000000000000,
        "RCX should be 0x8000000000000000"
    );
}

// ===== MEMORY OPERAND TESTS =====

#[test]
fn test_xadd_8bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (memory address)
        0xc6, 0x03, 0x0a, // MOV BYTE PTR [RBX], 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x0f, 0xc0, 0x0b, // XADD [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 1];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();

    assert_eq!(buf[0], 15, "Memory should contain 15 (sum)");
    assert_eq!(regs.rcx & 0xFF, 10, "CL should be 10 (old memory value)");
}

#[test]
fn test_xadd_16bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0xc7, 0x03, 0x00, 0x10, // MOV WORD PTR [RBX], 0x1000
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x66, 0x0f, 0xc1, 0x0b, // XADD [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 2];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u16::from_le_bytes(buf);

    assert_eq!(mem_val, 0x3000, "Memory should contain 0x3000");
    assert_eq!(regs.rcx & 0xFFFF, 0x1000, "CX should be 0x1000");
}

#[test]
fn test_xadd_32bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xc7, 0x03, 0x00, 0x00, 0x00, 0x10, // MOV DWORD PTR [RBX], 0x10000000
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x20, // MOV RCX, 0x20000000
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u32::from_le_bytes(buf);

    assert_eq!(mem_val, 0x30000000, "Memory should contain 0x30000000");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x10000000,
        "ECX should be 0x10000000"
    );
}

#[test]
fn test_xadd_64bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0x11, 0x11, 0x11, 0x11, // MOV QWORD PTR [RBX], 0x11111111
        0x48, 0xc7, 0xc1, 0x22, 0x22, 0x22, 0x22, // MOV RCX, 0x22222222
        0x48, 0x0f, 0xc1, 0x0b, // XADD [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u64::from_le_bytes(buf);

    assert_eq!(mem_val, 0x33333333, "Memory should contain 0x33333333");
    assert_eq!(regs.rcx, 0x11111111, "RCX should be 0x11111111");
}

#[test]
fn test_xadd_mem_overflow() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xc7, 0x03, 0xff, 0xff, 0xff, 0xff, // MOV DWORD PTR [RBX], 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u32::from_le_bytes(buf);

    assert_eq!(mem_val, 0, "Memory should wrap to 0");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "ECX should be 0xFFFFFFFF"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== PRACTICAL PATTERNS =====

#[test]
fn test_xadd_atomic_increment_pattern() {
    // Simulates atomic increment
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xc7, 0x03, 0x64, 0x00, 0x00, 0x00, // MOV DWORD PTR [RBX], 100
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u32::from_le_bytes(buf);

    assert_eq!(mem_val, 101, "Memory should be incremented to 101");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 100, "ECX should have old value");
}

#[test]
fn test_xadd_fetch_and_add_pattern() {
    // Fetch-and-add pattern
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xc7, 0x03, 0x2a, 0x00, 0x00, 0x00, // MOV DWORD PTR [RBX], 42
        0x48, 0xc7, 0xc1, 0x0d, 0x00, 0x00, 0x00, // MOV RCX, 13
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (fetch old value, add 13)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, vm_memory::GuestAddress(0x2000))
        .unwrap();
    let mem_val = u32::from_le_bytes(buf);

    assert_eq!(mem_val, 55, "Memory should be 55 (42 + 13)");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        42,
        "ECX contains old value (fetched)"
    );
}

#[test]
fn test_xadd_8bit_different_registers() {
    let code = [
        0x48, 0xc7, 0xc2, 0x10, 0x00, 0x00, 0x00, // MOV RDX, 0x10
        0x48, 0xc7, 0xc6, 0x20, 0x00, 0x00, 0x00, // MOV RSI, 0x20
        0x40, 0x0f, 0xc0, 0xf2, // XADD DL, SIL (REX prefix required for SIL)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFF, 0x30, "DL should be 0x30");
    assert_eq!(regs.rsi & 0xFF, 0x10, "SIL should be 0x10");
}
