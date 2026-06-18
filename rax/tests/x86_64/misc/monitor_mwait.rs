// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MONITOR - Set Up Monitor Address
// MWAIT - Monitor Wait
//
// MONITOR sets up an address range for hardware monitoring.
// MWAIT enters an optimized state waiting for a write to the monitored address.
// These instructions are used for address-range monitoring and power management.
//
// Opcodes:
// 0F 01 C8              MONITOR              - Set up linear address range to be monitored
// 0F 01 C9              MWAIT                - Wait for write to monitored address range

// MONITOR Tests

#[test]
fn test_monitor_basic() {
    // Basic MONITOR instruction
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // Address to monitor
    regs.rcx = 0; // Extensions (must be 0)
    regs.rdx = 0; // Hints
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MONITOR is a hint - registers should be unchanged
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rcx, 0, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0, "RDX should be unchanged");
}

#[test]
fn test_monitor_different_address() {
    // MONITOR with different address
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3000, "RAX should be unchanged");
}

#[test]
fn test_monitor_with_valid_memory() {
    // MONITOR with valid memory location
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to monitored address
    write_mem_at_u32(&mem, 0x2000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_no_flags_modified() {
    // MONITOR doesn't modify flags
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_monitor_aligned_address() {
    // MONITOR with aligned address
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // Aligned
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_unaligned_address() {
    // MONITOR with unaligned address
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2007; // Unaligned
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2007, "RAX should be unchanged");
}

#[test]
fn test_monitor_preserves_other_registers() {
    // MONITOR preserves other registers
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x1111;
    regs.rcx = 0;
    regs.rdx = 0;
    regs.rsi = 0x2222;
    regs.rdi = 0x3333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111, "RBX should be unchanged");
    assert_eq!(regs.rsi, 0x2222, "RSI should be unchanged");
    assert_eq!(regs.rdi, 0x3333, "RDI should be unchanged");
}

#[test]
fn test_monitor_multiple_sequential() {
    // Multiple sequential MONITOR instructions
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_with_different_hints() {
    // MONITOR with different hint values in RDX
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0x1; // Hint value (implementation-dependent)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rdx, 0x1, "RDX should be unchanged");
}

#[test]
fn test_monitor_high_memory_address() {
    // MONITOR with higher memory address
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x100000; // 1MB
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x100000, "RAX should be unchanged");
}

// MWAIT Tests

#[test]
fn test_mwait_basic() {
    // Basic MWAIT instruction (without MONITOR - should not wait)
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0; // Hints
    regs.rcx = 0; // Extensions
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MWAIT without prior MONITOR should not wait
    assert_eq!(regs.rax, 0, "RAX should be unchanged");
    assert_eq!(regs.rcx, 0, "RCX should be unchanged");
}

#[test]
fn test_mwait_with_monitor() {
    // MONITOR followed by MWAIT
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // Address for MONITOR, hints for MWAIT
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without hanging
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_mwait_no_flags_modified() {
    // MWAIT doesn't modify flags
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rcx = 0;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_mwait_preserves_registers() {
    // MWAIT preserves all registers
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0x1111;
    regs.rcx = 0;
    regs.rdx = 0x2222;
    regs.rsi = 0x3333;
    regs.rdi = 0x4444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111, "RBX should be unchanged");
    assert_eq!(regs.rdx, 0x2222, "RDX should be unchanged");
    assert_eq!(regs.rsi, 0x3333, "RSI should be unchanged");
    assert_eq!(regs.rdi, 0x4444, "RDI should be unchanged");
}

#[test]
fn test_mwait_with_c1_state() {
    // MWAIT with C1 state hint
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x10; // C1 state (bits 7:4 = 1)
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x10, "RAX should be unchanged");
}

#[test]
fn test_mwait_with_c2_state() {
    // MWAIT with C2 state hint
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x20; // C2 state (bits 7:4 = 2)
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x20, "RAX should be unchanged");
}

#[test]
fn test_mwait_with_substate() {
    // MWAIT with substate hint
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x13; // C1 state, substate 3
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x13, "RAX should be unchanged");
}

// MONITOR + MWAIT combinations

#[test]
fn test_monitor_mwait_sequence() {
    // Typical MONITOR/MWAIT usage sequence
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Set up monitored memory
    write_mem_at_u32(&mem, 0x2000, 0xDEADBEEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_mwait_different_addresses() {
    // MONITOR on one address, then change RAX before MWAIT
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV rax, 0x2000
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV rax, 0x0 (hints for MWAIT)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0, "RAX should be 0 after MOV");
}

#[test]
fn test_multiple_monitor_mwait_pairs() {
    // Multiple MONITOR/MWAIT pairs
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_without_mwait() {
    // MONITOR without following MWAIT (should be harmless)
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xff, 0xc3, // INC rbx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x11, "RBX should be incremented");
}

#[test]
fn test_monitor_mwait_with_operations_between() {
    // MONITOR/MWAIT with operations in between
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV rbx, 0x42
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV rax, 0x0 (hints)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x42, "RBX should be 0x42");
}

#[test]
fn test_monitor_mwait_preserves_memory() {
    // MONITOR/MWAIT doesn't modify monitored memory
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0x1122334455667788);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Memory should be unchanged
    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    let value = u64::from_le_bytes(buf);
    assert_eq!(value, 0x1122334455667788, "Memory should be unchanged");
}

#[test]
fn test_monitor_stack_address() {
    // MONITOR on stack address
    let code = [
        0x48, 0x89, 0xe0, // MOV rax, rsp
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8000, "RAX should contain stack pointer");
}

#[test]
fn test_monitor_code_segment_address() {
    // MONITOR on code segment address
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1000; // Code address
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1000, "RAX should be unchanged");
}

#[test]
fn test_mwait_interleaved_with_operations() {
    // MWAIT interleaved with other operations
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x00, 0x00, 0x00, // MOV rbx, 0x11
        0x0f, 0x01, 0xc9, // MWAIT
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV rcx, 0x22
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x11, "RBX should be 0x11");
    assert_eq!(regs.rcx, 0x22, "RCX should be 0x22");
}

#[test]
fn test_monitor_mwait_loop_pattern() {
    // Simulated loop pattern with MONITOR/MWAIT
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV rbx, 0
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x48, 0xff, 0xc3, // INC rbx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1, "RBX should be incremented");
}

#[test]
fn test_monitor_near_page_boundary() {
    // MONITOR on address near page boundary
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2FFF; // Near 4K boundary
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2FFF, "RAX should be unchanged");
}

#[test]
fn test_mwait_with_zero_hints() {
    // MWAIT with all zero hints (default behavior)
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX should be 0");
    assert_eq!(regs.rcx, 0, "RCX should be 0");
}

#[test]
fn test_monitor_mwait_memory_read_after() {
    // MONITOR/MWAIT followed by memory read
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV rax, 0
        0x0f, 0x01, 0xc9, // MWAIT
        0x48, 0x8b, 0x1d, 0x00, 0x00, 0x00, 0x00, // MOV rbx, [rip]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

#[test]
fn test_monitor_with_r10_address() {
    // MONITOR with address in R10 (via RAX)
    let code = [
        0x4c, 0x89, 0xd0, // MOV rax, r10
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0xA000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xA000, "RAX should contain R10 value");
}

#[test]
fn test_mwait_with_c3_state() {
    // MWAIT with C3 state hint
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x30; // C3 state (bits 7:4 = 3)
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x30, "RAX should be unchanged");
}

#[test]
fn test_monitor_mwait_with_write_back_memory() {
    // MONITOR/MWAIT on write-back memory type
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0xFEEDFACECAFEBABE);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_preserves_r8_through_r15() {
    // MONITOR preserves extended registers
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    regs.r8 = 0x8888;
    regs.r9 = 0x9999;
    regs.r10 = 0xAAAA;
    regs.r11 = 0xBBBB;
    regs.r12 = 0xCCCC;
    regs.r13 = 0xDDDD;
    regs.r14 = 0xEEEE;
    regs.r15 = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x8888, "R8 should be unchanged");
    assert_eq!(regs.r9, 0x9999, "R9 should be unchanged");
    assert_eq!(regs.r10, 0xAAAA, "R10 should be unchanged");
    assert_eq!(regs.r11, 0xBBBB, "R11 should be unchanged");
    assert_eq!(regs.r12, 0xCCCC, "R12 should be unchanged");
    assert_eq!(regs.r13, 0xDDDD, "R13 should be unchanged");
    assert_eq!(regs.r14, 0xEEEE, "R14 should be unchanged");
    assert_eq!(regs.r15, 0xFFFF, "R15 should be unchanged");
}

#[test]
fn test_mwait_preserves_r8_through_r15() {
    // MWAIT preserves extended registers
    let code = [
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rcx = 0;
    regs.r8 = 0x1111;
    regs.r9 = 0x2222;
    regs.r10 = 0x3333;
    regs.r11 = 0x4444;
    regs.r12 = 0x5555;
    regs.r13 = 0x6666;
    regs.r14 = 0x7777;
    regs.r15 = 0x8888;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8, 0x1111, "R8 should be unchanged");
    assert_eq!(regs.r9, 0x2222, "R9 should be unchanged");
    assert_eq!(regs.r10, 0x3333, "R10 should be unchanged");
    assert_eq!(regs.r11, 0x4444, "R11 should be unchanged");
    assert_eq!(regs.r12, 0x5555, "R12 should be unchanged");
    assert_eq!(regs.r13, 0x6666, "R13 should be unchanged");
    assert_eq!(regs.r14, 0x7777, "R14 should be unchanged");
    assert_eq!(regs.r15, 0x8888, "R15 should be unchanged");
}

#[test]
fn test_monitor_mwait_three_iterations() {
    // Three consecutive MONITOR/MWAIT iterations
    let code = [
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_monitor_mwait_with_increments() {
    // MONITOR/MWAIT with counter increments
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV rbx, 0
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xff, 0xc3, // INC rbx
        0x0f, 0x01, 0xc9, // MWAIT
        0x48, 0xff, 0xc3, // INC rbx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x2, "RBX should be incremented twice");
}
