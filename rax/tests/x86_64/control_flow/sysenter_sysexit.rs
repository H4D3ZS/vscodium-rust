use crate::common::*;
use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::backend::emulator::x86_64::flags;
use rax::backend::emulator::x86_64::flags::bits;
use rax::cpu::Registers;

// Comprehensive tests for SYSENTER/SYSEXIT instructions
// SYSENTER (0F 34) - Fast system call (Intel)
// SYSEXIT (0F 35) - Return from fast system call (Intel)
// Intel's alternative to AMD's SYSCALL/SYSRET

const SYSENTER_CS: u64 = 0x8;
const SYSENTER_HANDLER_ADDR: u64 = 0x13000;
const SYSENTER_STACK_ADDR: u64 = 0x9000;

fn set_sysenter_msrs(vcpu: &mut X86_64Vcpu, cs: u64, esp: u64, eip: u64) {
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.sysenter_cs = cs;
    sregs.sysenter_esp = esp;
    sregs.sysenter_eip = eip;
    vcpu.set_sregs(&sregs).unwrap();
}

fn install_sysenter_hlt(mem: &GuestMemoryMmap, addr: u64) {
    mem.write_slice(&[0xf4], GuestAddress(addr)).unwrap();
}

fn install_sysenter_sysexit(mem: &GuestMemoryMmap, addr: u64) {
    mem.write_slice(&[0x48, 0x0f, 0x35, 0xf4], GuestAddress(addr))
        .unwrap();
}

// ============================================================================
// SYSENTER - Basic Operation
// ============================================================================

#[test]
fn test_sysenter_basic() {
    // SYSENTER - fast system call (Intel)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (syscall number)
        0x48, 0x8d, 0x15, 0x05, 0x00, 0x00, 0x00, // LEA RDX, [RIP + 5] (return RIP)
        0x48, 0x89, 0xe1, // MOV RCX, RSP (return RSP)
        0x0f, 0x34, // SYSENTER
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99 (after call)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x99);
}

#[test]
fn test_sysenter_loads_from_msrs() {
    // SYSENTER loads CS, EIP, ESP from MSRs
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, SYSENTER_CS as u16);
    assert_eq!(sregs.ss.selector, (SYSENTER_CS as u16).wrapping_add(8));
    assert_eq!(regs.rsp, SYSENTER_STACK_ADDR);
    assert_eq!(regs.rip, SYSENTER_HANDLER_ADDR + 1);
}

#[test]
fn test_sysenter_no_return_address_save() {
    // SYSENTER does NOT save return address (unlike SYSCALL)
    let code = [
        0x48, 0x8d, 0x15, 0x05, 0x00, 0x00, 0x00, // LEA RDX, [RIP + 5] (return RIP)
        0x48, 0x89, 0xe1, // MOV RCX, RSP (return RSP)
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let return_rip = CODE_ADDR + 12;
    assert_eq!(regs.rcx, STACK_ADDR);
    assert_eq!(regs.rdx, return_rip);
}

#[test]
fn test_sysenter_with_parameters() {
    // SYSENTER with parameters (calling convention varies by OS)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x48, 0xc7, 0xc2, 0x0c, 0x00, 0x00, 0x00, // MOV RDX, 12
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
    assert_eq!(regs.rcx, 0x2000);
    assert_eq!(regs.rdx, 12);
}

// ============================================================================
// SYSENTER - MSR Configuration
// ============================================================================

#[test]
fn test_sysenter_cs_msr() {
    // SYSENTER_CS_MSR (0x174) - CS selector
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let cs_selector = 0x18u64;
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        cs_selector,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, cs_selector as u16);
}

#[test]
fn test_sysenter_esp_msr() {
    // SYSENTER_ESP_MSR (0x175) - ESP value
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let sysenter_rsp = 0x7000u64;
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(&mut vcpu, SYSENTER_CS, sysenter_rsp, SYSENTER_HANDLER_ADDR);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, sysenter_rsp);
}

#[test]
fn test_sysenter_eip_msr() {
    // SYSENTER_EIP_MSR (0x176) - EIP value (kernel entry point)
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let sysenter_eip = 0x14000u64;
    install_sysenter_hlt(&mem, sysenter_eip);
    set_sysenter_msrs(&mut vcpu, SYSENTER_CS, SYSENTER_STACK_ADDR, sysenter_eip);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, sysenter_eip + 1);
}

// ============================================================================
// SYSEXIT - Basic Return
// ============================================================================

#[test]
fn test_sysexit_basic() {
    // SYSEXIT - return from system call
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000 (return RSP)
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000 (return RIP)
        0x0f, 0x35, // SYSEXIT
        0xf4, // HLT (should not execute)
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99);
}

#[test]
fn test_sysexit_loads_rip_from_rdx() {
    // SYSEXIT loads RIP from RDX
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3000 + 1);
}

#[test]
fn test_sysexit_loads_rsp_from_rcx() {
    // SYSEXIT loads RSP from RCX
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x90, 0x00, 0x00, // MOV RCX, 0x9000
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP (check stack)
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x9000);
}

// ============================================================================
// SYSENTER/SYSEXIT - Round Trip
// ============================================================================

#[test]
fn test_sysenter_sysexit_roundtrip() {
    // SYSENTER followed by SYSEXIT
    // Note: caller must save return address manually
    let code = [
        0x48, 0x8d, 0x15, 0x05, 0x00, 0x00, 0x00, // LEA RDX, [RIP + 5] (return RIP)
        0x48, 0x89, 0xe1, // MOV RCX, RSP (return RSP)
        0x0f, 0x34, // SYSENTER
        // Return point
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x42);
}

#[test]
fn test_sysenter_sysexit_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x11, 0x00, 0x00, // MOV RBX, 0x1111
        0x48, 0xc7, 0xc5, 0x22, 0x22, 0x00, 0x00, // MOV RBP, 0x2222
        0x49, 0xc7, 0xc4, 0x33, 0x33, 0x00, 0x00, // MOV R12, 0x3333
        0x48, 0x8d, 0x15, 0x05, 0x00, 0x00, 0x00, // LEA RDX, [return]
        0x48, 0x89, 0xe1, // MOV RCX, RSP
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1111);
    assert_eq!(regs.rbp, 0x2222);
}

// ============================================================================
// SYSENTER - Different Calling Conventions
// ============================================================================

#[test]
fn test_sysenter_windows_convention() {
    // Windows uses different convention than Linux
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (syscall number)
        0x48, 0x8b, 0x0c, 0x24, // MOV RCX, [RSP] (return address from stack)
        0x48, 0x89, 0xe2, // MOV RDX, RSP
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Windows-specific behavior
}

#[test]
fn test_sysenter_linux_convention() {
    // Linux convention (via int 0x80 emulation or vDSO)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

// ============================================================================
// SYSENTER - Privilege Level Transitions
// ============================================================================

#[test]
fn test_sysenter_sets_cpl_to_zero() {
    // SYSENTER always sets CPL to 0 (kernel mode)
    let code = [
        0x0f, 0x34, // SYSENTER (CPL = 0)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector & 0x3, 0);
}

#[test]
fn test_sysexit_sets_cpl_to_three() {
    // SYSEXIT always sets CPL to 3 (user mode)
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x35, // SYSEXIT (CPL = 3)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector & 0x3, 3);
}

// ============================================================================
// SYSENTER - Error Conditions
// ============================================================================

#[test]
fn test_sysenter_invalid_in_real_mode() {
    // SYSENTER is invalid in real mode
    let code = [
        0x0f, 0x34, // SYSENTER (invalid in real mode)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cr0 &= !0x1;
    vcpu.set_sregs(&sregs).unwrap();

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_sysenter_clears_vm_from_vm86() {
    // SYSENTER clears VM when invoked from virtual-8086 mode.
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= flags::bits::VM;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & flags::bits::VM, 0);
}

#[test]
fn test_sysexit_invalid_in_user_mode() {
    // SYSEXIT from user mode should fault
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x35, // SYSEXIT (invalid from user mode)
        0x48, 0xc7, 0xc0, 0xfd, 0x00, 0x00, 0x00, // MOV RAX, 0xFD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cs.selector = 0x3;
    vcpu.set_sregs(&sregs).unwrap();

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_sysenter_with_null_cs_msr() {
    // SYSENTER with null CS in MSR should fault
    let code = [
        0x0f, 0x34, // SYSENTER
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    set_sysenter_msrs(&mut vcpu, 0, SYSENTER_STACK_ADDR, SYSENTER_HANDLER_ADDR);

    assert!(run_until_hlt(&mut vcpu).is_err());
}

// ============================================================================
// SYSENTER - Register Usage
// ============================================================================

#[test]
fn test_sysenter_preserves_general_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x00, 0x00, // MOV RBX, 0x2222
        0x48, 0xc7, 0xc5, 0x33, 0x33, 0x00, 0x00, // MOV RBP, 0x3333
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111);
    assert_eq!(regs.rbx, 0x2222);
    assert_eq!(regs.rbp, 0x3333);
}

#[test]
fn test_sysenter_modifies_cs_eip_esp() {
    let code = [
        0x48, 0xc7, 0xc4, 0x00, 0x70, 0x00, 0x00, // MOV RSP, 0x7000
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let sysenter_rsp = 0x7200u64;
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(&mut vcpu, SYSENTER_CS, sysenter_rsp, SYSENTER_HANDLER_ADDR);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert_eq!(sregs.cs.selector, SYSENTER_CS as u16);
    assert_eq!(regs.rsp, sysenter_rsp);
}

#[test]
fn test_sysexit_preserves_general_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x44, 0x44, 0x00, 0x00, // MOV RAX, 0x4444
        0x48, 0xc7, 0xc3, 0x55, 0x55, 0x00, 0x00, // MOV RBX, 0x5555
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x4444);
    assert_eq!(regs.rbx, 0x5555);
}

// ============================================================================
// SYSENTER - Flags Handling
// ============================================================================

#[test]
fn test_sysenter_clears_vm_flag() {
    // SYSENTER clears VM flag
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= flags::bits::VM;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & flags::bits::VM, 0);
}

#[test]
fn test_sysenter_clears_if_flag() {
    // SYSENTER clears IF (interrupt enable) flag
    let code = [
        0xfb, // STI (set IF)
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & flags::bits::IF, 0);
}

#[test]
fn test_sysenter_preserves_rf_flag() {
    // SYSENTER does not modify RF (resume) flag.
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= flags::bits::RF;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & flags::bits::RF, 0);
}

// ============================================================================
// SYSEXIT - Flags Handling
// ============================================================================

#[test]
fn test_sysexit_preserves_if_flag() {
    // SYSEXIT leaves RFLAGS unchanged.
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= flags::bits::IF;
    vcpu.set_regs(&regs).unwrap();

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & flags::bits::IF, 0);
}

// ============================================================================
// SYSENTER - Edge Cases
// ============================================================================

#[test]
fn test_sysenter_with_zero_eip_msr() {
    // SYSENTER with EIP MSR = 0
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(&mut vcpu, SYSENTER_CS, SYSENTER_STACK_ADDR, 0);

    // If MSR points to 0
    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x0000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0000 + 1);
}

#[test]
fn test_sysexit_with_zero_rdx() {
    // SYSEXIT with RDX = 0
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x80, 0x00, 0x00, // MOV RCX, 0x8000
        0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x0000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0000 + 1);
}

#[test]
fn test_sysexit_with_high_addresses() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0xe0, 0x00, 0x00, // MOV RCX, 0xE000
        0x48, 0xc7, 0xc2, 0x00, 0xf0, 0x00, 0x00, // MOV RDX, 0xF000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0xF000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0xf000 + 1);
}

// ============================================================================
// SYSENTER - 32-bit vs 64-bit Mode
// ============================================================================

#[test]
fn test_sysenter_32bit_compatibility() {
    // SYSENTER in 32-bit compatibility mode
    let code = [
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cs.l = false;
    sregs.cs.db = true;
    vcpu.set_sregs(&sregs).unwrap();
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert!(sregs.cs.l);
}

#[test]
fn test_sysenter_64bit_mode() {
    // SYSENTER in 64-bit mode
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x34, // SYSENTER
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_hlt(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let _regs = run_until_hlt(&mut vcpu).unwrap();
    let sregs = vcpu.get_sregs().unwrap();
    assert!(sregs.cs.l);
}

#[test]
fn test_sysexit_32bit() {
    // SYSEXIT (32-bit form) - loads EIP from EDX
    let code = [
        0xb9, 0x00, 0x80, 0x00, 0x00, // MOV ECX, 0x8000
        0xba, 0x00, 0x20, 0x00, 0x00, // MOV EDX, 0x2000
        0x0f, 0x35, // SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x2000 + 1);
}

#[test]
fn test_sysexit_64bit() {
    // SYSEXIT (64-bit form) - loads RIP from RDX
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x90, 0x00, 0x00, // MOV RCX, 0x9000
        0x48, 0xc7, 0xc2, 0x00, 0x30, 0x00, 0x00, // MOV RDX, 0x3000
        0x48, 0x0f, 0x35, // REX.W SYSEXIT
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3000 + 1);
}

// ============================================================================
// SYSENTER/SYSEXIT - Real-World Patterns
// ============================================================================

#[test]
fn test_sysenter_windows_ntdll_pattern() {
    // Windows NTDLL uses SYSENTER
    let code = [
        0x48, 0x8b, 0xd4, // MOV RDX, RSP (save stack)
        0x0f, 0x05, // Could be SYSCALL on AMD, checking pattern
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_sysenter_vdso_pattern() {
    // Linux vDSO (virtual dynamic shared object) pattern
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x8d, 0x15, 0x07, 0x00, 0x00, 0x00, // LEA RDX, [RIP + 7] (return RIP)
        0x48, 0x89, 0xe1, // MOV RCX, RSP
        0x0f, 0x34, // SYSENTER
        // Kernel returns here
        0x5d, // POP RBP
        0xc3, // RET
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sysenter_sysexit_kernel_handler_pattern() {
    // Typical kernel handler pattern
    let code = [
        // User space
        0x48, 0x8d, 0x15, 0x05, 0x00, 0x00, 0x00, // LEA RDX, [return]
        0x48, 0x89, 0xe1, // MOV RCX, RSP
        0x0f, 0x34, // SYSENTER
        // Return point
        0x48, 0xc7, 0xc3, 0xaa, 0x00, 0x00, 0x00, // MOV RBX, 0xAA
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    install_sysenter_sysexit(&mem, SYSENTER_HANDLER_ADDR);
    set_sysenter_msrs(
        &mut vcpu,
        SYSENTER_CS,
        SYSENTER_STACK_ADDR,
        SYSENTER_HANDLER_ADDR,
    );

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xaa);
}
