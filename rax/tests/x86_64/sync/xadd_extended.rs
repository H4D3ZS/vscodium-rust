use crate::common::*;

// XADD Extended Tests - Comprehensive patterns for atomic exchange-and-add
// This extends the basic XADD tests with more complex scenarios

// ===== COMPLEX ARITHMETIC PATTERNS =====

#[test]
fn test_xadd_8bit_chain_operations() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xc0, 0xcb, // XADD BL, CL (BL=3, CL=1)
        0x48, 0xc7, 0xc2, 0x04, 0x00, 0x00, 0x00, // MOV RDX, 4
        0x0f, 0xc0, 0xd3, // XADD BL, DL (BL=7, DL=3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 7, "BL should be 7");
    assert_eq!(regs.rcx & 0xFF, 1, "CL should be 1");
    assert_eq!(regs.rdx & 0xFF, 3, "DL should be 3");
}

#[test]
fn test_xadd_16bit_fibonacci_pattern() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX (BX=2, CX=1)
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX (BX=3, CX=2)
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX (BX=5, CX=3)
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX (BX=8, CX=5)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 8, "BX should be 8 (fibonacci)");
    assert_eq!(regs.rcx & 0xFFFF, 5, "CX should be 5 (fibonacci)");
}

#[test]
fn test_xadd_32bit_accumulator_pattern() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (EAX=10, EBX=0)
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (EAX=30, EBX=10)
        0x48, 0xc7, 0xc3, 0x1e, 0x00, 0x00, 0x00, // MOV RBX, 30
        0x0f, 0xc1, 0xd8, // XADD EAX, EBX (EAX=60, EBX=30)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 60, "EAX should accumulate to 60");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 30, "EBX should be 30");
}

#[test]
fn test_xadd_64bit_large_values() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, // MOV RAX, 0x1000000000000000
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x20, // MOV RAX, 0x2000000000000000
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0x3000000000000000,
        "RBX should be sum of large values"
    );
    assert_eq!(regs.rcx, 0x1000000000000000, "RCX should be old RBX");
}

// ===== MEMORY OPERAND PATTERNS =====

#[test]
fn test_xadd_mem_sequential_increments() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=11, RCX=10)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=12, RCX=11)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=13, RCX=12)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 10);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 13, "Memory should be incremented to 13");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        12,
        "RCX should have value before last add"
    );
}

#[test]
fn test_xadd_mem_power_of_two() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=2, RCX=1)
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=3, RCX=2)
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=5, RCX=3)
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX (mem=8, RCX=5)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 8, "Memory should be 8");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 5, "RCX should be 5");
}

#[test]
fn test_xadd_mem_with_offset_addressing() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xc1, 0x4b, 0x08, // XADD [RBX+8], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&20u32.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2008)).unwrap();
    assert_eq!(u32::from_le_bytes(buf), 30, "Memory at offset should be 30");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 20, "RCX should be 20");
}

#[test]
fn test_xadd_mem_with_sib_addressing() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0xc7, 0xc2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x0f, 0xc1, 0x0c, 0x13, // XADD [RBX+RDX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&15u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    assert_eq!(
        u32::from_le_bytes(buf),
        20,
        "Memory at SIB address should be 20"
    );
    assert_eq!(regs.rcx & 0xFFFFFFFF, 15, "RCX should be 15");
}

// ===== LOCK PREFIX PATTERNS =====

#[test]
fn test_xadd_lock_8bit_counter() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf0, 0x0f, 0xc0, 0x0b, // LOCK XADD [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u8(&mem),
        100,
        "Memory should be atomically incremented"
    );
    assert_eq!(regs.rcx & 0xFF, 99, "CL should have old value");
}

#[test]
fn test_xadd_lock_16bit_counter() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0xe8, 0x03, 0x00, 0x00, // MOV RCX, 1000
        0xf0, 0x66, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 5000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u16(&mem),
        6000,
        "Memory should be atomically updated"
    );
    assert_eq!(regs.rcx & 0xFFFF, 5000, "CX should have old value");
}

#[test]
fn test_xadd_lock_32bit_multiple() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        1200,
        "Memory should be incremented twice"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        1100,
        "RCX should have intermediate value"
    );
}

#[test]
fn test_xadd_lock_64bit_large_counter() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xb9, 0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 100000000
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 500000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        600000000,
        "Memory should be atomically updated"
    );
    assert_eq!(regs.rcx, 500000000, "RCX should have old value");
}

// ===== FLAG BEHAVIOR PATTERNS =====

#[test]
fn test_xadd_sets_all_arithmetic_flags() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set (overflow)");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set (result is 0)");
    assert_ne!(regs.rflags & 0x04, 0, "PF should be set (even parity)");
}

#[test]
fn test_xadd_clears_overflow_on_normal_add() {
    let code = [
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & 0x800, 0, "OF should be clear (no overflow)");
    assert_eq!(regs.rflags & 0x01, 0, "CF should be clear (no carry)");
    assert_eq!(
        regs.rflags & 0x40,
        0,
        "ZF should be clear (non-zero result)"
    );
}

#[test]
fn test_xadd_sign_flag_transitions() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0x7f, // MOV RBX, 0x7FFFFFFF (max positive)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (becomes negative)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x80000000,
        "EBX should be 0x80000000"
    );
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set (negative)");
    assert_ne!(regs.rflags & 0x800, 0, "OF should be set (overflow)");
}

#[test]
fn test_xadd_parity_flag_patterns() {
    let code = [
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2 (0b00000010)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (0b00000001)
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (result 3 = 0b00000011)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 3, "EBX should be 3");
    // 3 = 0b11 has 2 set bits = even parity, so PF = 1
    assert_eq!(regs.rflags & 0x04, 0x04, "PF should be set (even parity)");
}

#[test]
fn test_xadd_auxiliary_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc3, 0x0f, 0x00, 0x00, 0x00, // MOV RBX, 15 (0x0F)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX (0x0F + 1 = 0x10, AF set)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x10, "EBX should be 0x10");
    assert_ne!(regs.rflags & 0x10, 0, "AF should be set");
}

// ===== EDGE CASES AND BOUNDARY CONDITIONS =====

#[test]
fn test_xadd_both_operands_zero() {
    let code = [
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0, "ECX should be 0");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
    assert_ne!(regs.rflags & 0x04, 0, "PF should be set");
}

#[test]
fn test_xadd_one_operand_zero() {
    let code = [
        0x48, 0xc7, 0xc3, 0x2a, 0x00, 0x00, 0x00, // MOV RBX, 42
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 42, "EBX should remain 42");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 42, "ECX should be 42");
}

#[test]
fn test_xadd_boundary_8bit_max() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x0f, 0xc0, 0xcb, // XADD BL, CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0xFE, "BL should be 0xFE");
    assert_eq!(regs.rcx & 0xFF, 0xFF, "CL should be 0xFF");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_boundary_16bit_max() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0x00, 0x00, // MOV RBX, 0xFFFF
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0x00, 0x00, // MOV RCX, 0xFFFF
        0x66, 0x0f, 0xc1, 0xcb, // XADD BX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0xFFFE, "BX should be 0xFFFE");
    assert_eq!(regs.rcx & 0xFFFF, 0xFFFF, "CX should be 0xFFFF");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_boundary_32bit_wraparound() {
    let code = [
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xc1, 0xcb, // XADD EBX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 1, "EBX should wrap to 1");
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "ECX should be 0xFFFFFFFF"
    );
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

#[test]
fn test_xadd_boundary_64bit_wraparound() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x0f, 0xc1, 0xcb, // XADD RBX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 2, "RBX should wrap to 2");
    assert_eq!(regs.rcx, 0xFFFFFFFFFFFFFFFF, "RCX should be max");
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set");
}

// ===== REGISTER COMBINATIONS =====

#[test]
fn test_xadd_r8_r9_registers() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV R8, 0x11
        0x49, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV R9, 0x22
        0x4d, 0x0f, 0xc1, 0xc8, // XADD R8D, R9D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x33, "R8 should be 0x33");
    assert_eq!(regs.r9, 0x11, "R9 should be 0x11");
}

#[test]
fn test_xadd_r12_r13_registers() {
    let code = [
        0x49, 0xc7, 0xc4, 0xaa, 0xaa, 0xaa,
        0xaa, // MOV R12, 0xAAAAAAAA (sign-extends to 0xFFFFFFFFAAAAAAAA)
        0x49, 0xc7, 0xc5, 0x55, 0x55, 0x55, 0x55, // MOV R13, 0x55555555
        0x4d, 0x0f, 0xc1, 0xec, // XADD R12, R13 (64-bit due to REX.W)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Sum = 0xFFFFFFFFAAAAAAAA + 0x55555555 = 0xFFFFFFFFFFFFFFFF
    assert_eq!(
        regs.r12, 0xFFFFFFFFFFFFFFFFu64,
        "R12 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(regs.r13, 0xFFFFFFFFAAAAAAAAu64, "R13 should be old R12");
}

#[test]
fn test_xadd_r14_r15_registers() {
    let code = [
        0x49, 0xc7, 0xc6, 0x01, 0x00, 0x00, 0x00, // MOV R14, 1
        0x49, 0xc7, 0xc7, 0x02, 0x00, 0x00, 0x00, // MOV R15, 2
        0x4d, 0x0f, 0xc1, 0xfe, // XADD R14D, R15D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r14, 3, "R14 should be 3");
    assert_eq!(regs.r15, 1, "R15 should be 1");
}

// ===== MULTIPLE MEMORY LOCATIONS =====

#[test]
fn test_xadd_multiple_memory_locations() {
    let code = [
        // Location 1: 0x2000
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX
        // Location 2: 0x2010
        0x48, 0xc7, 0xc3, 0x10, 0x20, 0x00, 0x00, // MOV RBX, 0x2010
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xc1, 0x0b, // XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&10u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&20u32.to_le_bytes(), GuestAddress(0x2010))
        .unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf1 = [0u8; 4];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(u32::from_le_bytes(buf1), 15, "First location should be 15");

    let mut buf2 = [0u8; 4];
    mem.read_slice(&mut buf2, GuestAddress(0x2010)).unwrap();
    assert_eq!(u32::from_le_bytes(buf2), 30, "Second location should be 30");
}

// ===== PRACTICAL ATOMIC PATTERNS =====

#[test]
fn test_xadd_reference_counting_pattern() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Add reference
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        // Add another reference
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        // Remove reference
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, -1
        0xf0, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 2, "Reference count should be 2");
}

#[test]
fn test_xadd_stat_accumulation_pattern() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Record stat value 1
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        // Record stat value 2
        0x48, 0xc7, 0xc1, 0xc8, 0x00, 0x00, 0x00, // MOV RCX, 200
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        // Record stat value 3
        0x48, 0xc7, 0xc1, 0x2c, 0x01, 0x00, 0x00, // MOV RCX, 300
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u64(&mem), 600, "Total stats should be 600");
}

#[test]
fn test_xadd_array_element_update() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (array base)
        0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0 (index 0)
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xc1, 0x0c, 0x93, // XADD [RBX+RDX*4], ECX
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1 (index 1)
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x0f, 0xc1, 0x0c, 0x93, // XADD [RBX+RDX*4], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&5u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&15u32.to_le_bytes(), GuestAddress(0x2004))
        .unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf0 = [0u8; 4];
    mem.read_slice(&mut buf0, GuestAddress(0x2000)).unwrap();
    assert_eq!(u32::from_le_bytes(buf0), 15, "Array[0] should be 15");

    let mut buf1 = [0u8; 4];
    mem.read_slice(&mut buf1, GuestAddress(0x2004)).unwrap();
    assert_eq!(u32::from_le_bytes(buf1), 35, "Array[1] should be 35");
}

#[test]
fn test_xadd_performance_counter_simulation() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xb9, 0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 100000000
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        0x48, 0xb9, 0x00, 0xe1, 0xf5, 0x05, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 100000000
        0xf0, 0x48, 0x0f, 0xc1, 0x0b, // LOCK XADD [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 1000000000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        1200000000,
        "Counter should increment by 200M"
    );
}
