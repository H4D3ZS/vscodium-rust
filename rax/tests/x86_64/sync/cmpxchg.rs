use rax::cpu::Registers;

use crate::common::run_until_hlt;
use crate::common::setup_vm;

// CMPXCHG - Compare and Exchange (atomic compare-and-swap)
// Opcode: 0F B0 (8-bit), 0F B1 (16/32/64-bit)
//
// Operand 1 (Destination): Register
// Operand 2 (Source): Register
// Implicit: RAX/EAX/AX/AL (destination when not equal)
//
// Semantics:
//   if (dest == RAX/EAX/AX/AL) {
//       ZF = 1
//       dest = src
//   } else {
//       ZF = 0
//       RAX/EAX/AX/AL = dest
//   }

// ===== BASIC 8-BIT REGISTER TESTS =====

#[test]
fn test_cmpxchg_8bit_equal_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA (AL = 0xAA)
        0x48, 0xc7, 0xc3, 0xaa, 0x00, 0x00, 0x00, // MOV RBX, 0xAA (BL = 0xAA)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB (CL = 0xBB)
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL (compare AL with BL, exchange if equal)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xAA, "AL should remain 0xAA");
    assert_eq!(regs.rbx & 0xFF, 0xBB, "BL should be set to CL (0xBB)");
    assert_ne!(
        regs.rflags & 0x40,
        0,
        "ZF should be set (values were equal)"
    );
}

#[test]
fn test_cmpxchg_8bit_not_equal_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0xcc, 0x00, 0x00, 0x00, // MOV RBX, 0xCC (different)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xCC, "AL should be loaded with BL (0xCC)");
    assert_eq!(regs.rbx & 0xFF, 0xCC, "BL should remain 0xCC");
    assert_eq!(
        regs.rflags & 0x40,
        0,
        "ZF should be clear (values were not equal)"
    );
}

#[test]
fn test_cmpxchg_8bit_with_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (RBX = 0)
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0xFF, "BL should be exchanged");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set (both were 0)");
}

#[test]
fn test_cmpxchg_8bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0, "BL should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== 16-BIT REGISTER TESTS =====

#[test]
fn test_cmpxchg_16bit_equal_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x11, 0x11, 0x00, 0x00, // MOV RBX, 0x1111
        0x48, 0xc7, 0xc1, 0x22, 0x22, 0x00, 0x00, // MOV RCX, 0x2222
        0x66, 0x0f, 0xb1, 0xcb, // CMPXCHG BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1111, "AX should remain 0x1111");
    assert_eq!(regs.rbx & 0xFFFF, 0x2222, "BX should be set to CX (0x2222)");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_16bit_not_equal_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x33, 0x33, 0x00, 0x00, // MOV RBX, 0x3333 (different)
        0x48, 0xc7, 0xc1, 0x22, 0x22, 0x00, 0x00, // MOV RCX, 0x2222
        0x66, 0x0f, 0xb1, 0xcb, // CMPXCHG BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x3333, "AX should be loaded with BX");
    assert_eq!(regs.rbx & 0xFFFF, 0x3333, "BX should remain unchanged");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_16bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0x00, 0x00, // MOV RAX, 0xFFFF
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0x00, 0x00, // MOV RBX, 0xFFFF
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x66, 0x0f, 0xb1, 0xcb, // CMPXCHG BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0, "BX should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== 32-BIT REGISTER TESTS =====

#[test]
fn test_cmpxchg_32bit_equal_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x22, 0x33, 0x44, // MOV RAX, 0x44332211
        0x48, 0xc7, 0xc3, 0x11, 0x22, 0x33, 0x44, // MOV RBX, 0x44332211
        0x48, 0xc7, 0xc1, 0xaa, 0xbb, 0xcc, 0xdd, // MOV RCX, 0xDDCCBBAA
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x44332211,
        "EAX should remain 0x44332211"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xDDCCBBAA, "EBX should be exchanged");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_32bit_not_equal_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x22, 0x33, 0x44, // MOV RAX, 0x44332211
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF (different)
        0x48, 0xc7, 0xc1, 0xaa, 0xbb, 0xcc, 0xdd, // MOV RCX, 0xDDCCBBAA
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be loaded with EBX"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EBX should remain unchanged"
    );
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_32bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_32bit_zeros_upper() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33 (not equal)
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (32-bit operation)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 32-bit operation should zero upper 32 bits of result
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x33, "EAX should be 0x33");
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be zeroed");
}

// ===== 64-BIT REGISTER TESTS =====

#[test]
fn test_cmpxchg_64bit_equal_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xc7, 0xc3, 0x11, 0x11, 0x11, 0x11, // MOV RBX, 0x11111111
        0x48, 0xc7, 0xc1, 0x22, 0x22, 0x22, 0x22, // MOV RCX, 0x22222222
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11111111, "RAX should remain 0x11111111");
    assert_eq!(regs.rbx, 0x22222222, "RBX should be set to RCX");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_64bit_not_equal_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xc7, 0xc3, 0x33, 0x33, 0x33, 0x33, // MOV RBX, 0x33333333
        0x48, 0xc7, 0xc1, 0x22, 0x22, 0x22, 0x22, // MOV RCX, 0x22222222
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x33333333, "RAX should be loaded with RBX");
    assert_eq!(regs.rbx, 0x33333333, "RBX should remain unchanged");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_64bit_max_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0x31, 0xc9, // XOR RCX, RCX (RCX = 0)
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0, "RBX should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== DIFFERENT REGISTER COMBINATIONS =====

#[test]
fn test_cmpxchg_edx_esi() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RAX, 0xAAAAAAAA
        0x48, 0xc7, 0xc2, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RDX, 0xAAAAAAAA
        0x48, 0xc7, 0xc6, 0xbb, 0xbb, 0xbb, 0xbb, // MOV RSI, 0xBBBBBBBB
        0x0f, 0xb1, 0xf2, // CMPXCHG EDX, ESI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0xBBBBBBBB, "EDX should be exchanged");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== FLAG BEHAVIOR TESTS =====

#[test]
fn test_cmpxchg_zf_set_on_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(
        regs.rflags & 0x40,
        0,
        "ZF should be set after successful exchange"
    );
}

#[test]
fn test_cmpxchg_zf_clear_on_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (different)
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rflags & 0x40,
        0,
        "ZF should be clear after failed exchange"
    );
}

#[test]
fn test_cmpxchg_sets_arithmetic_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (compares 5 vs 10)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // The comparison is EAX - EBX = 5 - 10 = -5 (borrow/carry set)
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set (5 < 10 unsigned)");
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set (negative result)");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear (not equal)");
}

#[test]
fn test_cmpxchg_cf_set_on_borrow() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (10 < 20, borrow needed)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_cmpxchg_parity_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (0 == 0, result is 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(
        regs.rflags & 0x04,
        0,
        "PF should be set (0 has even parity)"
    );
}

// ===== SIGNED VALUE TESTS =====

#[test]
fn test_cmpxchg_signed_negative_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (0xFFFFFFFF)
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, -1
        0x48, 0xc7, 0xc1, 0xfe, 0xff, 0xff, 0xff, // MOV RCX, -2 (0xFFFFFFFE)
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xFFFFFFFE, "EBX should be -2");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== REGISTER PROPERTY TESTS =====

#[test]
fn test_cmpxchg_src_unchanged() {
    // Verify source register is not modified by CMPXCHG
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0xbb, 0x00, 0x00, 0x00, // MOV RBX, 0xBB
        0x48, 0xc7, 0xc1, 0xcc, 0x00, 0x00, 0x00, // MOV RCX, 0xCC
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (fails, loads RBX into RAX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0xCC, "ECX should remain unchanged");
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xBB, "EAX should be loaded with EBX");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xBB, "EBX should remain unchanged");
}

#[test]
fn test_cmpxchg_with_rax_as_source() {
    // Edge case: when RAX is used as the source operand
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11
        0x0f, 0xb1, 0xc3, // CMPXCHG EBX, EAX (exchange with itself)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11, "EAX should remain 0x11");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x11,
        "EBX should be set to EAX (0x11)"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_8bit_preserves_upper_rax() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc3, 0xcc, 0x00, 0x00, 0x00, // MOV RBX, 0xCC (different from AL)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xCC, "AL should be loaded from BL");
    assert_eq!((regs.rax >> 8) & 0xFF, 0xFF, "AH should remain unchanged");
}

// ===== PRACTICAL PATTERNS =====

#[test]
fn test_cmpxchg_practical_cas_pattern() {
    // Practical compare-and-swap pattern
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100 (expected value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (current value)
        0x48, 0xc7, 0xc1, 0xc8, 0x00, 0x00, 0x00, // MOV RCX, 200 (desired value)
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 200, "Value should be updated to desired");
    assert_ne!(regs.rflags & 0x40, 0, "ZF set indicates CAS success");
}

#[test]
fn test_cmpxchg_cas_failure_pattern() {
    // CAS fails when value differs
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100 (expected value)
        0x48, 0xc7, 0xc3, 0x96, 0x00, 0x00, 0x00, // MOV RBX, 150 (current value, different)
        0x48, 0xc7, 0xc1, 0xc8, 0x00, 0x00, 0x00, // MOV RCX, 200 (desired value)
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 150, "Value should remain unchanged");
    assert_eq!(regs.rax, 150, "RAX should be loaded with actual value");
    assert_eq!(regs.rflags & 0x40, 0, "ZF clear indicates CAS failure");
}

#[test]
fn test_cmpxchg_chained_operations() {
    // Chain multiple CAS operations to test sequential behavior
    let code = [
        // First CMPXCHG: success
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX (RBX becomes 2)
        // Second CMPXCHG: failure (RAX still 1, RBX now 2)
        0x48, 0xc7, 0xc2, 0x03, 0x00, 0x00, 0x00, // MOV RDX, 3
        0x48, 0x0f, 0xb1, 0xd3, // CMPXCHG RBX, RDX (fails because RAX=1, RBX=2)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 2,
        "RAX should have loaded RBX's value from failed CMPXCHG"
    );
    assert_eq!(regs.rbx, 2, "RBX should remain 2");
    assert_eq!(
        regs.rflags & 0x40,
        0,
        "ZF should be clear from failed CMPXCHG"
    );
}

#[test]
fn test_cmpxchg_counter_increment_pattern() {
    // Simulate atomic counter increment pattern
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10 (current counter)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10 (comparison value)
        0x48, 0xc7, 0xc1, 0x0b, 0x00, 0x00, 0x00, // MOV RCX, 11 (incremented)
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 11, "Counter should be incremented");
    assert_ne!(
        regs.rflags & 0x40,
        0,
        "ZF set indicates increment succeeded"
    );
}

#[test]
fn test_cmpxchg_boundary_transition() {
    // Test behavior at 32-bit boundary
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7FFFFFFF
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0x7f, // MOV RBX, 0x7FFFFFFF
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x80, // MOV RCX, 0x80000000 (min signed)
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x80000000, "EBX should be exchanged");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}
