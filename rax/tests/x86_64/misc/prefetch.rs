// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PREFETCHW - Prefetch Data Into Caches in Anticipation of a Write
// PREFETCHWT1 - Prefetch Vector Data Into Caches With Intent to Write and T1 Hint
//
// These instructions are hints and do not affect program behavior.
// They do not raise exceptions on invalid addresses.
// They are unordered with respect to fence instructions and locked operations.
//
// Opcodes:
// 0F 0D /1            PREFETCHW m8           - Prefetch for write
// 0F 0D /2            PREFETCHWT1 m8         - Prefetch to L2 cache with write intent

// PREFETCHW Tests

#[test]
fn test_prefetchw_basic() {
    // Basic PREFETCHW with memory operand [rax]
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PREFETCHW is a hint - registers should be unchanged
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_with_displacement() {
    // PREFETCHW with displacement [rax + 0x10]
    let code = [
        0x0f, 0x0d, 0x48, 0x10, // PREFETCHW [rax + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_with_negative_displacement() {
    // PREFETCHW with negative displacement [rax - 0x10]
    let code = [
        0x0f, 0x0d, 0x48, 0xf0, // PREFETCHW [rax - 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2100, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_rbx_base() {
    // PREFETCHW using RBX as base [rbx]
    let code = [
        0x0f, 0x0d, 0x0b, // PREFETCHW [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
}

#[test]
fn test_prefetchw_rcx_base() {
    // PREFETCHW using RCX as base [rcx]
    let code = [
        0x0f, 0x0d, 0x09, // PREFETCHW [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_prefetchw_rdx_base() {
    // PREFETCHW using RDX as base [rdx]
    let code = [
        0x0f, 0x0d, 0x0a, // PREFETCHW [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x5000, "RDX should be unchanged");
}

#[test]
fn test_prefetchw_rsi_base() {
    // PREFETCHW using RSI as base [rsi]
    let code = [
        0x0f, 0x0d, 0x0e, // PREFETCHW [rsi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x6000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x6000, "RSI should be unchanged");
}

#[test]
fn test_prefetchw_rdi_base() {
    // PREFETCHW using RDI as base [rdi]
    let code = [
        0x0f, 0x0d, 0x0f, // PREFETCHW [rdi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x7000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x7000, "RDI should be unchanged");
}

#[test]
fn test_prefetchw_large_displacement() {
    // PREFETCHW with large displacement [rax + 0x1000]
    let code = [
        0x0f, 0x0d, 0x88, 0x00, 0x10, 0x00, 0x00, // PREFETCHW [rax + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_sib_addressing() {
    // PREFETCHW with SIB addressing [rax + rbx*4]
    let code = [
        0x0f, 0x0d, 0x0c, 0x98, // PREFETCHW [rax + rbx*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x10, "RBX should be unchanged");
}

#[test]
fn test_prefetchw_sib_scale_2() {
    // PREFETCHW with SIB scale 2 [rax + rbx*2]
    let code = [
        0x0f, 0x0d, 0x0c, 0x58, // PREFETCHW [rax + rbx*2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x100, "RBX should be unchanged");
}

#[test]
fn test_prefetchw_sib_scale_8() {
    // PREFETCHW with SIB scale 8 [rax + rbx*8]
    let code = [
        0x0f, 0x0d, 0x0c, 0xd8, // PREFETCHW [rax + rbx*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x20, "RBX should be unchanged");
}

#[test]
fn test_prefetchw_no_flags_modified() {
    // PREFETCHW doesn't modify flags
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_prefetchw_multiple_sequential() {
    // Multiple sequential PREFETCHW instructions
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0x0f, 0x0d, 0x0b, // PREFETCHW [rbx]
        0x0f, 0x0d, 0x09, // PREFETCHW [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x3000;
    regs.rcx = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_prefetchw_with_valid_memory() {
    // PREFETCHW on valid memory location
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory
    mem.write_slice(&[0x42u8], GuestAddress(0x2000)).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_aligned_address() {
    // PREFETCHW on cache-line aligned address
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // 64-byte aligned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_unaligned_address() {
    // PREFETCHW on unaligned address (should work - it's a hint)
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2007; // Unaligned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2007, "RAX should be unchanged");
}

#[test]
fn test_prefetchw_with_r8_base() {
    // PREFETCHW using R8 as base [r8]
    let code = [
        0x41, 0x0f, 0x0d, 0x08, // PREFETCHW [r8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x8000, "R8 should be unchanged");
}

#[test]
fn test_prefetchw_with_r15_base() {
    // PREFETCHW using R15 as base [r15]
    let code = [
        0x41, 0x0f, 0x0d, 0x0f, // PREFETCHW [r15]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xF000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xF000, "R15 should be unchanged");
}

// PREFETCHWT1 Tests

#[test]
fn test_prefetchwt1_basic() {
    // Basic PREFETCHWT1 with memory operand [rax]
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // PREFETCHWT1 is a hint - registers should be unchanged
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_with_displacement() {
    // PREFETCHWT1 with displacement [rax + 0x20]
    let code = [
        0x0f, 0x0d, 0x50, 0x20, // PREFETCHWT1 [rax + 0x20]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_with_negative_displacement() {
    // PREFETCHWT1 with negative displacement [rax - 0x20]
    let code = [
        0x0f, 0x0d, 0x50, 0xe0, // PREFETCHWT1 [rax - 0x20]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2100, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_rbx_base() {
    // PREFETCHWT1 using RBX as base [rbx]
    let code = [
        0x0f, 0x0d, 0x13, // PREFETCHWT1 [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
}

#[test]
fn test_prefetchwt1_rcx_base() {
    // PREFETCHWT1 using RCX as base [rcx]
    let code = [
        0x0f, 0x0d, 0x11, // PREFETCHWT1 [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_prefetchwt1_rdx_base() {
    // PREFETCHWT1 using RDX as base [rdx]
    let code = [
        0x0f, 0x0d, 0x12, // PREFETCHWT1 [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx, 0x5000, "RDX should be unchanged");
}

#[test]
fn test_prefetchwt1_rsi_base() {
    // PREFETCHWT1 using RSI as base [rsi]
    let code = [
        0x0f, 0x0d, 0x16, // PREFETCHWT1 [rsi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x6000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x6000, "RSI should be unchanged");
}

#[test]
fn test_prefetchwt1_rdi_base() {
    // PREFETCHWT1 using RDI as base [rdi]
    let code = [
        0x0f, 0x0d, 0x17, // PREFETCHWT1 [rdi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x7000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x7000, "RDI should be unchanged");
}

#[test]
fn test_prefetchwt1_large_displacement() {
    // PREFETCHWT1 with large displacement [rax + 0x2000]
    let code = [
        0x0f, 0x0d, 0x90, 0x00, 0x20, 0x00, 0x00, // PREFETCHWT1 [rax + 0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_sib_addressing() {
    // PREFETCHWT1 with SIB addressing [rax + rbx*4]
    let code = [
        0x0f, 0x0d, 0x14, 0x98, // PREFETCHWT1 [rax + rbx*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x10, "RBX should be unchanged");
}

#[test]
fn test_prefetchwt1_sib_scale_2() {
    // PREFETCHWT1 with SIB scale 2 [rax + rbx*2]
    let code = [
        0x0f, 0x0d, 0x14, 0x58, // PREFETCHWT1 [rax + rbx*2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x100, "RBX should be unchanged");
}

#[test]
fn test_prefetchwt1_sib_scale_8() {
    // PREFETCHWT1 with SIB scale 8 [rax + rbx*8]
    let code = [
        0x0f, 0x0d, 0x14, 0xd8, // PREFETCHWT1 [rax + rbx*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x20, "RBX should be unchanged");
}

#[test]
fn test_prefetchwt1_no_flags_modified() {
    // PREFETCHWT1 doesn't modify flags
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_prefetchwt1_multiple_sequential() {
    // Multiple sequential PREFETCHWT1 instructions
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0x0f, 0x0d, 0x13, // PREFETCHWT1 [rbx]
        0x0f, 0x0d, 0x11, // PREFETCHWT1 [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x3000;
    regs.rcx = 0x4000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_prefetchwt1_with_valid_memory() {
    // PREFETCHWT1 on valid memory location
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory
    mem.write_slice(&[0x42u8], GuestAddress(0x2000)).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_aligned_address() {
    // PREFETCHWT1 on cache-line aligned address
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // 64-byte aligned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_unaligned_address() {
    // PREFETCHWT1 on unaligned address (should work - it's a hint)
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2007; // Unaligned
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2007, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_with_r8_base() {
    // PREFETCHWT1 using R8 as base [r8]
    let code = [
        0x41, 0x0f, 0x0d, 0x10, // PREFETCHWT1 [r8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x8000, "R8 should be unchanged");
}

#[test]
fn test_prefetchwt1_with_r15_base() {
    // PREFETCHWT1 using R15 as base [r15]
    let code = [
        0x41, 0x0f, 0x0d, 0x17, // PREFETCHWT1 [r15]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xF000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0xF000, "R15 should be unchanged");
}

// Mixed PREFETCHW and PREFETCHWT1 tests

#[test]
fn test_prefetch_mixed_sequence() {
    // Mix of PREFETCHW and PREFETCHWT1
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0x0f, 0x0d, 0x13, // PREFETCHWT1 [rbx]
        0x0f, 0x0d, 0x09, // PREFETCHW [rcx]
        0x0f, 0x0d, 0x12, // PREFETCHWT1 [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x3000;
    regs.rcx = 0x4000;
    regs.rdx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0x5000, "RDX should be unchanged");
}

#[test]
fn test_prefetch_interleaved_with_operations() {
    // Prefetch instructions interleaved with actual operations
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV rax, 0x42
        0x0f, 0x0d, 0x0b, // PREFETCHW [rbx]
        0x48, 0xc7, 0xc1, 0x84, 0x00, 0x00, 0x00, // MOV rcx, 0x84
        0x0f, 0x0d, 0x12, // PREFETCHWT1 [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    regs.rdx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should be 0x42");
    assert_eq!(regs.rcx, 0x84, "RCX should be 0x84");
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
    assert_eq!(regs.rdx, 0x5000, "RDX should be unchanged");
}

#[test]
fn test_prefetchw_same_location_twice() {
    // PREFETCHW on the same location twice
    let code = [
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0x0f, 0x0d, 0x08, // PREFETCHW [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_prefetchwt1_same_location_twice() {
    // PREFETCHWT1 on the same location twice
    let code = [
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0x0f, 0x0d, 0x10, // PREFETCHWT1 [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}
