// Module path for tests run via x86_64.rs
use crate::common::{
    cf_set, of_set, pf_set, read_mem_at_u8, read_mem_at_u16, read_mem_at_u32, read_mem_at_u64,
    run_until_hlt, setup_vm, sf_set, write_mem_at_u8, write_mem_at_u16, write_mem_at_u32,
    write_mem_at_u64, zf_set,
};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// LOCK - Assert LOCK# Signal Prefix
//
// The LOCK prefix causes the processor to assert the LOCK# signal during
// execution of the instruction, ensuring atomic access to shared memory.
// Can be used with: ADD, ADC, AND, BTC, BTR, BTS, CMPXCHG, CMPXCH8B,
// CMPXCHG16B, DEC, INC, NEG, NOT, OR, SBB, SUB, XOR, XADD, and XCHG.
//
// Opcode:
// F0                    LOCK                 - Assert LOCK# signal prefix

// LOCK ADD tests

#[test]
fn test_lock_add_byte() {
    // LOCK ADD byte [rax], 0x10
    let code = [
        0xf0, 0x80, 0x00, 0x10, // LOCK ADD byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x20);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x30,
        "Memory should contain 0x30"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(!cf_set(regs.rflags), "CF should not be set");
    assert!(!zf_set(regs.rflags), "ZF should not be set");
}

#[test]
fn test_lock_add_word() {
    // LOCK ADD word [rax], 0x1000
    let code = [
        0x66, 0xf0, 0x81, 0x00, 0x00, 0x10, // LOCK ADD word [rax], 0x1000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u16(&mem, 0x2000, 0x2000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u16(&mem, 0x2000),
        0x3000,
        "Memory should contain 0x3000"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_add_dword() {
    // LOCK ADD dword [rax], 0x12345678
    let code = [
        0xf0, 0x81, 0x00, 0x78, 0x56, 0x34, 0x12, // LOCK ADD dword [rax], 0x12345678
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x11111111);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x23456789,
        "Memory should contain sum"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_add_qword() {
    // LOCK ADD qword [rax], 0x100
    let code = [
        0xf0, 0x48, 0x81, 0x00, 0x00, 0x01, 0x00, 0x00, // LOCK ADD qword [rax], 0x100
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0x1000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x2000),
        0x1100,
        "Memory should contain 0x1100"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_add_with_carry() {
    // LOCK ADD that sets carry flag
    let code = [
        0xf0, 0x80, 0x00, 0x10, // LOCK ADD byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x0F,
        "Memory should wrap to 0x0F"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

// LOCK SUB tests

#[test]
fn test_lock_sub_byte() {
    // LOCK SUB byte [rax], 0x10
    let code = [
        0xf0, 0x80, 0x28, 0x10, // LOCK SUB byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x30);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x20,
        "Memory should contain 0x20"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(!cf_set(regs.rflags), "CF should not be set");
}

#[test]
fn test_lock_sub_dword() {
    // LOCK SUB dword [rax], 0x1000
    let code = [
        0xf0, 0x81, 0x28, 0x00, 0x10, 0x00, 0x00, // LOCK SUB dword [rax], 0x1000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x3000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x2000,
        "Memory should contain 0x2000"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_sub_with_borrow() {
    // LOCK SUB that sets carry flag (borrow)
    let code = [
        0xf0, 0x80, 0x28, 0x10, // LOCK SUB byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x05);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xF5,
        "Memory should wrap to 0xF5"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_lock_sub_zero_result() {
    // LOCK SUB resulting in zero
    let code = [
        0xf0, 0x80, 0x28, 0x42, // LOCK SUB byte [rax], 0x42
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x00, "Memory should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK AND tests

#[test]
fn test_lock_and_byte() {
    // LOCK AND byte [rax], 0x0F
    let code = [
        0xf0, 0x80, 0x20, 0x0f, // LOCK AND byte [rax], 0x0F
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x0F,
        "Memory should contain 0x0F"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
    assert!(!of_set(regs.rflags), "OF should be cleared");
}

#[test]
fn test_lock_and_dword() {
    // LOCK AND dword [rax], 0xFF00FF00
    let code = [
        0xf0, 0x81, 0x20, 0x00, 0xff, 0x00, 0xff, // LOCK AND dword [rax], 0xFF00FF00
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x12005600,
        "Memory should contain ANDed result"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_and_zero_result() {
    // LOCK AND resulting in zero
    let code = [
        0xf0, 0x80, 0x20, 0x00, // LOCK AND byte [rax], 0x00
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x00, "Memory should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK OR tests

#[test]
fn test_lock_or_byte() {
    // LOCK OR byte [rax], 0xF0
    let code = [
        0xf0, 0x80, 0x08, 0xf0, // LOCK OR byte [rax], 0xF0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x0F);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xFF,
        "Memory should contain 0xFF"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
    assert!(!of_set(regs.rflags), "OF should be cleared");
}

#[test]
fn test_lock_or_dword() {
    // LOCK OR dword [rax], 0x0000FFFF
    let code = [
        0xf0, 0x81, 0x08, 0xff, 0xff, 0x00, 0x00, // LOCK OR dword [rax], 0x0000FFFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0xFFFF0000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xFFFFFFFF,
        "Memory should contain ORed result"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_or_no_change() {
    // LOCK OR with value that doesn't change memory
    let code = [
        0xf0, 0x80, 0x08, 0x00, // LOCK OR byte [rax], 0x00
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x42,
        "Memory should be unchanged"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

// LOCK XOR tests

#[test]
fn test_lock_xor_byte() {
    // LOCK XOR byte [rax], 0xFF
    let code = [
        0xf0, 0x80, 0x30, 0xff, // LOCK XOR byte [rax], 0xFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x55,
        "Memory should contain XORed result"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(!cf_set(regs.rflags), "CF should be cleared");
    assert!(!of_set(regs.rflags), "OF should be cleared");
}

#[test]
fn test_lock_xor_dword() {
    // LOCK XOR dword [rax], 0xFFFFFFFF
    let code = [
        0xf0, 0x81, 0x30, 0xff, 0xff, 0xff, 0xff, // LOCK XOR dword [rax], 0xFFFFFFFF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xEDCBA987,
        "Memory should contain XORed result"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_xor_same_value() {
    // LOCK XOR with same value results in zero
    let code = [
        0xf0, 0x80, 0x30, 0x42, // LOCK XOR byte [rax], 0x42
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x00, "Memory should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK INC tests

#[test]
fn test_lock_inc_byte() {
    // LOCK INC byte [rax]
    let code = [
        0xf0, 0xfe, 0x00, // LOCK INC byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x41);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x42,
        "Memory should be incremented to 0x42"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_inc_dword() {
    // LOCK INC dword [rax]
    let code = [
        0xf0, 0xff, 0x00, // LOCK INC dword [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x12345679,
        "Memory should be incremented"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_inc_overflow() {
    // LOCK INC that overflows
    let code = [
        0xf0, 0xfe, 0x00, // LOCK INC byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x00,
        "Memory should wrap to 0x00"
    );
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK DEC tests

#[test]
fn test_lock_dec_byte() {
    // LOCK DEC byte [rax]
    let code = [
        0xf0, 0xfe, 0x08, // LOCK DEC byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x41,
        "Memory should be decremented to 0x41"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_dec_dword() {
    // LOCK DEC dword [rax]
    let code = [
        0xf0, 0xff, 0x08, // LOCK DEC dword [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x12345677,
        "Memory should be decremented"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_dec_underflow() {
    // LOCK DEC that underflows
    let code = [
        0xf0, 0xfe, 0x08, // LOCK DEC byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x00);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xFF,
        "Memory should wrap to 0xFF"
    );
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_lock_dec_to_zero() {
    // LOCK DEC resulting in zero
    let code = [
        0xf0, 0xfe, 0x08, // LOCK DEC byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x01);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x00, "Memory should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK NEG tests

#[test]
fn test_lock_neg_byte() {
    // LOCK NEG byte [rax]
    let code = [
        0xf0, 0xf6, 0x18, // LOCK NEG byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x05);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xFB,
        "Memory should be negated"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_lock_neg_dword() {
    // LOCK NEG dword [rax]
    let code = [
        0xf0, 0xf7, 0x18, // LOCK NEG dword [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x00000001);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xFFFFFFFF,
        "Memory should be negated"
    );
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_lock_neg_zero() {
    // LOCK NEG of zero
    let code = [
        0xf0, 0xf6, 0x18, // LOCK NEG byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x00);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x00,
        "Memory should remain zero"
    );
    assert!(!cf_set(regs.rflags), "CF should not be set for zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

// LOCK NOT tests

#[test]
fn test_lock_not_byte() {
    // LOCK NOT byte [rax]
    let code = [
        0xf0, 0xf6, 0x10, // LOCK NOT byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x55,
        "Memory should be bitwise inverted"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_not_dword() {
    // LOCK NOT dword [rax]
    let code = [
        0xf0, 0xf7, 0x10, // LOCK NOT dword [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xEDCBA987,
        "Memory should be bitwise inverted"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_not_all_ones() {
    // LOCK NOT of all ones
    let code = [
        0xf0, 0xf6, 0x10, // LOCK NOT byte [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x00, "Memory should be zero");
}

// LOCK XCHG tests (XCHG always asserts LOCK even without prefix)

#[test]
fn test_lock_xchg_byte() {
    // LOCK XCHG byte [rax], bl
    let code = [
        0xf0, 0x86, 0x18, // LOCK XCHG byte [rax], bl
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x42;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x42,
        "Memory should contain BL value"
    );
    assert_eq!(regs.rbx & 0xFF, 0x99, "BL should contain memory value");
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_lock_xchg_dword() {
    // LOCK XCHG dword [rax], ebx
    let code = [
        0xf0, 0x87, 0x18, // LOCK XCHG dword [rax], ebx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0xAABBCCDD);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x12345678,
        "Memory should contain EBX value"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xAABBCCDD,
        "EBX should contain memory value"
    );
}

// LOCK with different addressing modes

#[test]
fn test_lock_add_with_displacement() {
    // LOCK ADD with displacement [rax + 0x10]
    let code = [
        0xf0, 0x80, 0x40, 0x10, 0x05, // LOCK ADD byte [rax + 0x10], 0x05
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2010, 0x10);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2010),
        0x15,
        "Memory should contain 0x15"
    );
}

#[test]
fn test_lock_add_with_sib() {
    // LOCK ADD with SIB addressing [rax + rbx*4]
    let code = [
        0xf0, 0x80, 0x04, 0x98, 0x20, // LOCK ADD byte [rax + rbx*4], 0x20
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x10*4 = 0x2040
    write_mem_at_u8(&mem, 0x2040, 0x10);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2040),
        0x30,
        "Memory should contain 0x30"
    );
    assert_eq!(regs.rbx, 0x10, "RBX should be unchanged");
}

#[test]
fn test_lock_operations_sequential() {
    // Multiple sequential LOCK operations
    let code = [
        0xf0, 0x80, 0x00, 0x10, // LOCK ADD byte [rax], 0x10
        0xf0, 0x80, 0x28, 0x05, // LOCK SUB byte [rax], 0x05
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x20);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x20 + 0x10 - 0x05 = 0x2B
    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x2B,
        "Memory should contain 0x2B"
    );
}

#[test]
fn test_lock_with_different_bases() {
    // LOCK operations with different base registers
    let code = [
        0xf0, 0x80, 0x03, 0x10, // LOCK ADD byte [rbx], 0x10
        0xf0, 0x80, 0x01, 0x20, // LOCK ADD byte [rcx], 0x20
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    regs.rcx = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x3000, 0x05);
    write_mem_at_u8(&mem, 0x4000, 0x10);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x3000),
        0x15,
        "Memory at RBX should be updated"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x4000),
        0x30,
        "Memory at RCX should be updated"
    );
}

#[test]
fn test_lock_adc_byte() {
    // LOCK ADC byte [rax], 0x10 with carry
    let code = [
        0xf9, // STC (set carry flag)
        0xf0, 0x80, 0x10, 0x10, // LOCK ADC byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x20);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x20 + 0x10 + 1 (carry) = 0x31
    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x31,
        "Memory should contain 0x31"
    );
}

#[test]
fn test_lock_sbb_byte() {
    // LOCK SBB byte [rax], 0x10 with carry
    let code = [
        0xf9, // STC (set carry flag)
        0xf0, 0x80, 0x18, 0x10, // LOCK SBB byte [rax], 0x10
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x30);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x30 - 0x10 - 1 (carry) = 0x1F
    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x1F,
        "Memory should contain 0x1F"
    );
}

#[test]
fn test_lock_add_qword_with_r8() {
    // LOCK ADD with R8 base register
    let code = [
        0xf0, 0x49, 0x81, 0x00, 0x00, 0x10, 0x00, 0x00, // LOCK ADD qword [r8], 0x1000
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x8000, 0x5000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x8000),
        0x6000,
        "Memory should contain 0x6000"
    );
    assert_eq!(regs.r8, 0x8000, "R8 should be unchanged");
}
