use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMPXCHG - Compare and Exchange
// Compares AL/AX/EAX/RAX with destination, if equal sets ZF and loads source into destination
// If not equal, clears ZF and loads destination into AL/AX/EAX/RAX

// Basic CMPXCHG - values equal (exchange happens)
#[test]
fn test_cmpxchg_eax_ebx_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11 (same as RAX)
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX (compare EAX with EBX, exchange if equal)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11, "EAX should remain 0x11");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x22,
        "EBX should be set to ECX (0x22)"
    );
    assert!(regs.rflags & 0x40 != 0, "ZF should be set (equal)");
}

// CMPXCHG - values not equal (no exchange)
#[test]
fn test_cmpxchg_eax_ebx_not_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33 (different from RAX)
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x33,
        "EAX should be loaded with EBX (0x33)"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x33, "EBX should remain 0x33");
    assert!(regs.rflags & 0x40 == 0, "ZF should be clear (not equal)");
}

// 8-bit CMPXCHG with AL
#[test]
fn test_cmpxchg_bl_cl_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA (AL = 0xAA)
        0x48, 0xc7, 0xc3, 0xaa, 0x00, 0x00, 0x00, // MOV RBX, 0xAA (BL = 0xAA)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB (CL = 0xBB)
        0x0f, 0xb0, 0xcb, // CMPXCHG BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xAA, "AL should remain 0xAA");
    assert_eq!(regs.rbx & 0xFF, 0xBB, "BL should be set to CL (0xBB)");
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_bl_cl_not_equal() {
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
    assert!(regs.rflags & 0x40 == 0, "ZF should be clear");
}

// 16-bit CMPXCHG with AX
#[test]
fn test_cmpxchg_bx_cx_equal() {
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
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// 64-bit CMPXCHG with RAX
#[test]
fn test_cmpxchg_rbx_rcx_equal() {
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
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_rbx_rcx_not_equal() {
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
    assert!(regs.rflags & 0x40 == 0, "ZF should be clear");
}

// Test with different register combinations
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
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test with extended registers
#[test]
fn test_cmpxchg_r8_r9() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111
        0x49, 0xc7, 0xc1, 0x22, 0x22, 0x22, 0x22, // MOV R9, 0x22222222
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x4d, 0x0f, 0xb1, 0xc8, // CMPXCHG R8, R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x22222222, "R8 should be exchanged with R9");
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test with zero values
#[test]
fn test_cmpxchg_with_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (RBX = 0)
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xFFFFFFFF, "EBX should be exchanged");
    assert!(regs.rflags & 0x40 != 0, "ZF should be set (both were 0)");
}

// Test that other flags are set correctly (like CF, SF, OF, PF, AF)
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
    // The comparison is EAX - EBX = 5 - 10 = -5 (borrow/carry)
    assert!(
        regs.rflags & 0x01 != 0,
        "CF should be set (5 < 10 unsigned)"
    );
    assert!(
        regs.rflags & 0x80 != 0,
        "SF should be set (negative result)"
    );
    assert!(regs.rflags & 0x40 == 0, "ZF should be clear (not equal)");
}

// Test 32-bit operation zeros upper 32 bits
#[test]
fn test_cmpxchg_32bit_zeros_upper() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33 (not equal)
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad,
        0xde, // MOV RAX, 0xDEADBEEFDEADBEEF
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11 again
        0x48, 0xbb, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad, 0xde, // MOV RBX, garbage
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33
        0x0f, 0xb1, 0xcb, // CMPXCHG EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000033,
        "RAX upper bits should be zeroed"
    );
}

// Test practical use case: atomic compare-and-swap
#[test]
fn test_cmpxchg_practical_cas_success() {
    // Simulate atomic CAS: if (*ptr == expected) { *ptr = desired; return true; }
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
    assert!(regs.rflags & 0x40 != 0, "ZF set indicates success");
}

#[test]
fn test_cmpxchg_practical_cas_failure() {
    // Simulate atomic CAS failure: if (*ptr != expected) { return false; }
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
    assert!(regs.rflags & 0x40 == 0, "ZF clear indicates failure");
}

// Test maximum values
#[test]
fn test_cmpxchg_max_values() {
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
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test chaining multiple CMPXCHG operations
#[test]
fn test_cmpxchg_chain() {
    let code = [
        // First CMPXCHG: success
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0x0f, 0xb1, 0xcb, // CMPXCHG RBX, RCX (RBX becomes 2)
        // Second CMPXCHG: failure (RAX still 1, RBX now 2)
        0x48, 0xc7, 0xc2, 0x03, 0x00, 0x00, 0x00, // MOV RDX, 3
        0x48, 0x0f, 0xb1, 0xd3, // CMPXCHG RBX, RDX (fails, RAX becomes 2)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 2,
        "RAX should have loaded RBX's value from failed CMPXCHG"
    );
    assert_eq!(regs.rbx, 2, "RBX should remain 2");
    assert!(
        regs.rflags & 0x40 == 0,
        "ZF should be clear from failed CMPXCHG"
    );
}

// Test boundary case: same register (though not typical)
#[test]
fn test_cmpxchg_same_register_dest_src() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV RBX, 0x11
        0x0f, 0xb1, 0xdb, // CMPXCHG EBX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x11, "EBX should remain 0x11");
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test with negative signed values
#[test]
fn test_cmpxchg_signed_negative() {
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
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test that CMPXCHG doesn't modify source register
#[test]
fn test_cmpxchg_source_unchanged() {
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
