use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// XADD - Exchange and Add
// Exchanges first operand with second, then loads sum into first operand

// Basic XADD - 32-bit registers
#[test]
fn test_xadd_eax_ebx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x1E, "EAX should be 10 + 20 = 30");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x0A,
        "EBX should have old EAX value (10)"
    );
}

// Test that flags are set correctly
#[test]
fn test_xadd_sets_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (0xFFFFFFFF)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (0xFFFFFFFF + 1 = 0, sets ZF and CF)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0 (overflow)");
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EBX should have old EAX value"
    );
    assert!(regs.rflags & 0x40 != 0, "ZF should be set (result is zero)");
    assert!(regs.rflags & 0x01 != 0, "CF should be set (carry occurred)");
}

// 8-bit XADD
#[test]
fn test_xadd_al_bl() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x0f, 0xc0, 0xd8, // XADD AL, BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x08, "AL should be 5 + 3 = 8");
    assert_eq!(regs.rbx & 0xFF, 0x05, "BL should have old AL value (5)");
}

#[test]
fn test_xadd_byte_overflow() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0xc0, 0xd8, // XADD AL, BL (0xFF + 1 = 0, with carry)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0, "AL should be 0");
    assert_eq!(regs.rbx & 0xFF, 0xFF, "BL should be 0xFF");
    assert!(regs.rflags & 0x01 != 0, "CF should be set");
}

// 16-bit XADD
#[test]
fn test_xadd_ax_bx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0xc8, 0x00, 0x00, 0x00, // MOV RBX, 200
        0x66, 0x0f, 0xc1, 0xd8, // XADD AX, BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 300, "AX should be 100 + 200 = 300");
    assert_eq!(regs.rbx & 0xFFFF, 100, "BX should have old AX value");
}

// 64-bit XADD
#[test]
fn test_xadd_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x0f, 0xc1, 0xd8, // XADD RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x3000, "RAX should be 0x1000 + 0x2000 = 0x3000");
    assert_eq!(regs.rbx, 0x1000, "RBX should have old RAX value");
}

// Test different register combinations
#[test]
fn test_xadd_ecx_edx() {
    let code = [
        0x48, 0xc7, 0xc1, 0x32, 0x00, 0x00, 0x00, // MOV RCX, 50
        0x48, 0xc7, 0xc2, 0x64, 0x00, 0x00, 0x00, // MOV RDX, 100
        0x0f, 0xc1, 0xd1, // XADD ECX, EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFFFFFF, 150, "ECX should be 50 + 100 = 150");
    assert_eq!(regs.rdx & 0xFFFFFFFF, 50, "EDX should have old ECX value");
}

#[test]
fn test_xadd_esi_edi() {
    let code = [
        0x48, 0xc7, 0xc6, 0x0a, 0x00, 0x00, 0x00, // MOV RSI, 10
        0x48, 0xc7, 0xc7, 0x05, 0x00, 0x00, 0x00, // MOV RDI, 5
        0x0f, 0xc1, 0xfe, // XADD ESI, EDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 15, "ESI should be 10 + 5 = 15");
    assert_eq!(regs.rdi & 0xFFFFFFFF, 10, "EDI should have old ESI value");
}

// Test with extended registers (R8-R15)
#[test]
fn test_xadd_r8_r9() {
    let code = [
        0x49, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV R8, 100
        0x49, 0xc7, 0xc1, 0xc8, 0x00, 0x00, 0x00, // MOV R9, 200
        0x4d, 0x0f, 0xc1, 0xc8, // XADD R8, R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 300, "R8 should be 100 + 200 = 300");
    assert_eq!(regs.r9, 100, "R9 should have old R8 value");
}

#[test]
fn test_xadd_rax_r10() {
    let code = [
        0x48, 0xc7, 0xc0, 0x2c, 0x01, 0x00, 0x00, // MOV RAX, 300
        0x49, 0xc7, 0xc2, 0x58, 0x02, 0x00, 0x00, // MOV R10, 600
        0x49, 0x0f, 0xc1, 0xc2, // XADD R10, RAX (ModRM: rm=R10, reg=RAX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // XADD r/m, r: DEST = DEST + SRC, SRC = old DEST
    // R10 = R10 + RAX = 600 + 300 = 900
    // RAX = old R10 = 600
    assert_eq!(regs.r10, 900, "R10 should be 300 + 600 = 900");
    assert_eq!(regs.rax, 600, "RAX should have old R10 value");
}

// Test with zero
#[test]
fn test_xadd_with_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 10, "EAX should be 0 + 10 = 10");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
}

#[test]
fn test_xadd_both_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert!(regs.rflags & 0x40 != 0, "ZF should be set");
}

// Test with negative values (signed addition)
#[test]
fn test_xadd_signed_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0xfe, 0xff, 0xff, 0xff, // MOV RBX, -2
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFD,
        "EAX should be -1 + -2 = -3"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xFFFFFFFF, "EBX should be -1");
}

// Test that 32-bit XADD zeros upper 32 bits
#[test]
fn test_xadd_32bit_zeros_upper() {
    let code = [
        0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad,
        0xde, // MOV RAX, 0xDEADBEEFDEADBEEF
        0x48, 0xbb, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad,
        0xde, // MOV RBX, 0xDEADBEEFDEADBEEF
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000000000001E,
        "RAX upper bits should be zeroed"
    );
    assert_eq!(
        regs.rbx, 0x000000000000000A,
        "RBX upper bits should be zeroed"
    );
}

// Test sign flag
#[test]
fn test_xadd_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x80000000 (negative in signed)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x80000001,
        "EAX should be 0x80000001"
    );
    assert!(
        regs.rflags & 0x80 != 0,
        "SF should be set (negative result)"
    );
}

// Test overflow flag
#[test]
fn test_xadd_overflow_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7FFFFFFF (max positive signed)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (overflow to negative)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x80000000, "EAX should overflow");
    assert!(
        regs.rflags & 0x800 != 0,
        "OF should be set (signed overflow)"
    );
}

// Test parity flag
#[test]
fn test_xadd_parity_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (result = 3 = 0b11, even parity)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 3, "EAX should be 3");
    assert!(
        regs.rflags & 0x04 != 0,
        "PF should be set (even parity in low byte)"
    );
}

// Test auxiliary carry flag
#[test]
fn test_xadd_auxiliary_carry() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (0x0F + 1 = 0x10, carry from bit 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x10, "EAX should be 0x10");
    assert!(regs.rflags & 0x10 != 0, "AF should be set");
}

// Test practical use case: atomic increment with old value retrieval
#[test]
fn test_xadd_practical_atomic_increment() {
    // Atomic fetch-and-add: old_value = *ptr; *ptr += increment; return old_value;
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100 (current counter)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (increment)
        0x48, 0x0f, 0xc1, 0xd8, // XADD RAX, RBX
        // RAX now contains 101 (new value)
        // RBX now contains 100 (old value)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 101, "Counter should be incremented to 101");
    assert_eq!(regs.rbx, 100, "Old value should be 100");
}

// Test maximum values
#[test]
fn test_xadd_max_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFE,
        "EAX should be 0xFFFFFFFE"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EBX should be 0xFFFFFFFF"
    );
    assert!(regs.rflags & 0x01 != 0, "CF should be set (carry out)");
}

// Test chaining multiple XADD operations
#[test]
fn test_xadd_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x0f, 0xc1, 0xd8, // XADD RAX, RBX (RAX=3, RBX=1)
        0x48, 0x0f, 0xc1, 0xc8, // XADD RAX, RCX (RAX=6, RCX=3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 6, "RAX should be 6 (1+2+3)");
    assert_eq!(regs.rbx, 1, "RBX should be 1 (original RAX)");
    assert_eq!(regs.rcx, 3, "RCX should be 3 (RAX after first XADD)");
}

// Test with SP/ESP/RSP (stack pointer)
#[test]
fn test_xadd_with_rsp() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x48, 0xc7, 0xc4, 0x00, 0x20, 0x00, 0x00, // MOV RSP, 0x2000
        0x48, 0x0f, 0xc1, 0xe0, // XADD RAX, RSP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x3000, "RAX should be 0x3000");
    assert_eq!(regs.rsp, 0x1000, "RSP should be 0x1000");
}

// Test same register (not typical but valid)
#[test]
fn test_xadd_same_register() {
    let code = [
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x0f, 0xc1, 0xdb, // XADD EBX, EBX (doubles the value)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 10, "EBX should be doubled to 10");
}

// Test that XADD is atomic (single instruction)
#[test]
fn test_xadd_practical_lock_free_counter() {
    // In multithreaded context, LOCK XADD is used for atomic operations
    // Here we test the sequential behavior
    let code = [
        0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00, // MOV RAX, 42 (shared counter)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (thread's delta)
        0x48, 0x0f, 0xc1, 0xd8, // XADD RAX, RBX
        // RBX now has the old counter value (42)
        // RAX now has the new counter value (47)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 47, "Counter updated to 47");
    assert_eq!(regs.rbx, 42, "Old counter value preserved");
}
