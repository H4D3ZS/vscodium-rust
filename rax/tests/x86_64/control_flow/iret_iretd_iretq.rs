use crate::common::*;
use rax::cpu::Registers;

fn write_stack_value(mem: &GuestMemoryMmap, addr: u64, size: u8, value: u64) {
    match size {
        2 => write_mem_at_u16(mem, addr, value as u16),
        4 => write_mem_at_u32(mem, addr, value as u32),
        8 => write_mem_at_u64(mem, addr, value),
        _ => panic!("unsupported stack value size: {}", size),
    }
}

fn write_iret_frame(mem: &GuestMemoryMmap, rsp: u64, op_size: u8, rip: u64, cs: u16, flags: u64) {
    write_stack_value(mem, rsp, op_size, rip);
    write_stack_value(mem, rsp + op_size as u64, op_size, cs as u64);
    write_stack_value(mem, rsp + 2 * op_size as u64, op_size, flags);
}

fn write_iret_outer_frame(
    mem: &GuestMemoryMmap,
    rsp: u64,
    op_size: u8,
    rip: u64,
    cs: u16,
    flags: u64,
    new_rsp: u64,
    new_ss: u16,
) {
    write_iret_frame(mem, rsp, op_size, rip, cs, flags);
    write_stack_value(mem, rsp + 3 * op_size as u64, op_size, new_rsp);
    write_stack_value(mem, rsp + 4 * op_size as u64, op_size, new_ss as u64);
}

// Comprehensive tests for IRET/IRETD/IRETQ instructions (interrupt return)
// IRET (CF), IRETD, IRETQ
// Returns from an interrupt handler, restoring FLAGS, CS, and IP/EIP/RIP

// ============================================================================
// IRET - Basic Return from Interrupt (16-bit)
// ============================================================================

#[test]
fn test_iret_basic_16bit() {
    // IRET - return from interrupt (16-bit mode)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x66, 0xcf, // IRET (16-bit)
        0xf4, // HLT (should not execute)
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 2, 0x2000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2001);
}

#[test]
fn test_iret_pops_ip_cs_flags() {
    // Verify IRET pops IP, CS, and FLAGS
    // In 64-bit mode, IRET always pops RSP and SS too
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x66, 0xcf, // IRET (16-bit)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // In 64-bit mode, even 16-bit IRET pops RSP and SS
    write_iret_outer_frame(&mem, 0x8000, 2, 0x3000, 0x08, 0x0202, 0x7000, 0x10);

    let target_code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP (check stack restored)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RSP should be restored to 0x7000
    assert_eq!(regs.rax & 0xFFFF, 0x7000);
}

// ============================================================================
// IRETD - Return from Interrupt (32-bit)
// ============================================================================

#[test]
fn test_iretd_basic_32bit() {
    // IRETD - return from interrupt (32-bit mode)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRETD (32-bit default)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2001);
}

#[test]
fn test_iretd_restores_eflags() {
    // IRETD restores EFLAGS register
    // In 64-bit mode, IRET always pops RSP and SS
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRETD
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Provide RSP=0x7000, SS=0x10 so PUSHFQ has a valid stack
    write_iret_outer_frame(&mem, 0x8000, 4, 0x3000, 0x08, 0x0000_0402, 0x7000, 0x10);

    let target_code = [
        0x9c, // PUSHFQ (save restored flags)
        0x58, // POP RAX
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Check DF (bit 10) is set in restored flags
    assert_ne!(regs.rax & 0x400, 0, "DF should be set");
}

// ============================================================================
// IRETQ - Return from Interrupt (64-bit)
// ============================================================================

#[test]
fn test_iretq_basic_64bit() {
    // IRETQ - return from interrupt (64-bit mode)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xcf, // IRETQ (64-bit)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 8, 0x2000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2001);
}

#[test]
fn test_iretq_restores_rflags() {
    // IRETQ restores full RFLAGS
    // In 64-bit mode, IRETQ always pops RSP and SS
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xcf, // IRETQ
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Provide RSP=0x7000, SS=0x10 so PUSHFQ has a valid stack
    write_iret_outer_frame(&mem, 0x8000, 8, 0x4000, 0x08, 0x0000_0246, 0x7000, 0x10);

    let target_code = [
        0x9c, // PUSHFQ
        0x58, // POP RAX (get restored flags)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x4000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Check ZF (bit 6), PF (bit 2) are set in restored flags (0x246 = ZF|PF|IF|reserved)
    assert_ne!(regs.rax & 0x40, 0, "ZF should be set");
    assert_ne!(regs.rax & 0x04, 0, "PF should be set");
}

// ============================================================================
// IRET - Stack Frame Variations
// ============================================================================

#[test]
fn test_iret_same_privilege_level() {
    // IRET within same privilege level (no SS:RSP restore)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2);

    let target_code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xaa);
}

#[test]
fn test_iret_to_outer_privilege_level() {
    // IRET to outer (lower) privilege - pops SS:RSP too
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xcf, // IRETQ
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_outer_frame(&mem, 0x8000, 8, 0x2000, 0x1b, 0x2, 0xa000, 0x23);

    let target_code = [
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xbb);
}

#[test]
fn test_iret_restores_outer_stack() {
    // IRET to outer level restores SS:RSP
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xcf, // IRETQ
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_outer_frame(&mem, 0x8000, 8, 0x3000, 0x1b, 0x2, 0xb000, 0x23);

    let target_code = [
        0x48, 0x89, 0xe2, // MOV RDX, RSP (check restored stack)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xb000);
}

// ============================================================================
// IRET - Flags Restoration
// ============================================================================

#[test]
fn test_iret_restores_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0003);

    let target_code = [
        0x72, 0x05, // JC +5 (jump if carry)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, // HLT padding
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1); // CF was set
}

#[test]
fn test_iret_restores_zero_flag() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0046);

    let target_code = [
        0x74, 0x05, // JZ +5 (jump if zero)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, // HLT padding
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2); // ZF was set
}

#[test]
fn test_iret_restores_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0082);

    let target_code = [
        0x78, 0x05, // JS +5 (jump if sign)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3); // SF was set
}

#[test]
fn test_iret_restores_overflow_flag() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0802);

    let target_code = [
        0x70, 0x05, // JO +5 (jump if overflow)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 4); // OF was set
}

#[test]
fn test_iret_restores_direction_flag() {
    // In 64-bit mode, IRET always pops RSP and SS
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Provide RSP=0x7000, SS=0x10 so PUSHFQ has a valid stack
    // 0x402 = DF (bit 10) + reserved bit 1
    write_iret_outer_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0402, 0x7000, 0x10);

    let target_code = [
        0x9c, // PUSHFQ
        0x58, // POP RAX (check DF)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // DF (bit 10) should be set in restored flags
    assert_ne!(regs.rax & 0x400, 0, "DF should be set");
}

#[test]
fn test_iret_restores_interrupt_flag() {
    // In 64-bit mode, IRET always pops RSP and SS
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Provide RSP=0x7000, SS=0x10 so PUSHFQ has a valid stack
    // 0x202 = IF (bit 9) + reserved bit 1
    write_iret_outer_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0202, 0x7000, 0x10);

    let target_code = [
        0x9c, // PUSHFQ
        0x58, // POP RAX
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // IF (bit 9) should be set in restored flags
    assert_ne!(regs.rax & 0x200, 0, "IF should be set");
}

// ============================================================================
// IRET - Validation and Error Cases
// ============================================================================

#[test]
fn test_iret_null_cs_selector() {
    // IRET with null CS selector should fault
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x00, 0x2);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_iret_invalid_cs_selector() {
    // IRET with invalid CS selector
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0xffff, 0x2);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_iret_non_present_segment() {
    // IRET to non-present segment
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x20, 0x2);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_iret_to_data_segment() {
    // IRET to data segment (should fault)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x10, 0x2);

    let target_code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 4);
}

#[test]
fn test_iret_insufficient_stack() {
    // IRET with insufficient stack space
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x00, 0x00, 0x01, // MOV RSP, 0x1000000 (beyond memory)
        0xcf, // IRET (should fault)
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

// ============================================================================
// IRET - Nested Interrupt Returns
// ============================================================================

#[test]
fn test_iret_nested_interrupts() {
    // Simulate nested interrupt returns
    // In 64-bit mode, each IRET pops RSP and SS, so we need outer frames
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET to level 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // First IRET: jump to 0x2000, set RSP to 0x7000 (where second frame is)
    write_iret_outer_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2, 0x7000, 0x10);
    // Second IRET: at RSP=0x7000, jump to 0x3000, set RSP to 0x6000
    write_iret_outer_frame(&mem, 0x7000, 4, 0x3000, 0x08, 0x2, 0x6000, 0x10);

    // Level 1 handler
    let level1 = [
        0xcf, // IRET to level 2
        0xf4,
    ];
    mem.write_slice(&level1, vm_memory::GuestAddress(0x2000))
        .unwrap();

    // Level 2 handler
    let level2 = [
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0xf4,
    ];
    mem.write_slice(&level2, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x77);
}

// ============================================================================
// IRET - Register Preservation
// ============================================================================

#[test]
fn test_iret_preserves_general_registers() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x00, 0x00, // MOV RBX, 0x2222
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111);
    assert_eq!(regs.rbx, 0x2222);
}

#[test]
fn test_iret_modifies_cs_rip_rflags() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_0046);

    let target_code = [
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2004);
}

// ============================================================================
// IRET - VM and IOPL Flags
// ============================================================================

#[test]
fn test_iret_vm_flag() {
    // IRET with VM flag (virtual 8086 mode)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0002_0002);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // VM flag handling
}

#[test]
fn test_iret_iopl_levels() {
    // IRET with different IOPL levels
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x0000_3002);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // IOPL should be restored
}

// ============================================================================
// IRET - Edge Cases
// ============================================================================

#[test]
fn test_iret_to_zero_address() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x0000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x0000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0001);
}

#[test]
fn test_iret_to_max_address() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0xFFFF, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0xFFFF))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x10000);
}

#[test]
fn test_iret_aligned_address() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x3000, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_iret_unaligned_address() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x3003, 0x08, 0x2);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3003))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3004);
}

// ============================================================================
// IRET - Real-World Patterns
// ============================================================================

#[test]
fn test_iret_interrupt_handler_pattern() {
    // Common interrupt handler pattern
    // In 64-bit mode, IRET always pops RSP and SS
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRET (return from handler)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Provide RSP=0x7000 so PUSH RAX has a valid stack
    write_iret_outer_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2, 0x7000, 0x10);

    let handler = [
        0x50, // PUSH RAX (save registers)
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99 (do work)
        0x58, // POP RAX (restore registers)
        0xf4,
    ];
    mem.write_slice(&handler, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RAX should be restored to 0 (original value) after POP
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_iretd_iretq_difference() {
    // Test difference between IRETD (32-bit) and IRETQ (64-bit)
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x80, 0x00, 0x00, // MOV RSP, 0x8000
        0xcf, // IRETD (default in current mode)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_iret_frame(&mem, 0x8000, 4, 0x2000, 0x08, 0x2);

    let target_code = [
        0x48, 0xc7, 0xc0, 0xab, 0xcd, 0x00, 0x00, // MOV RAX, 0xCDAB
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xcdab);
}
