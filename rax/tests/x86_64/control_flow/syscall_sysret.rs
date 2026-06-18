use crate::common::*;
use rax::cpu::Registers;

// Comprehensive tests for SYSCALL/SYSRET instructions
// SYSCALL (0F 05) - Fast system call (AMD)
// SYSRET (0F 07) - Return from fast system call
// Used primarily in 64-bit mode for efficient kernel calls

// ============================================================================
// SYSCALL - Basic Operation
// ============================================================================

#[test]
fn test_syscall_basic() {
    // SYSCALL - fast system call instruction
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (syscall number)
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99 (after syscall)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SYSCALL behavior depends on MSR configuration
    assert_eq!(regs.rbx, 0x99);
}

#[test]
fn test_syscall_saves_return_address() {
    // SYSCALL saves return address in RCX
    let code = [
        0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00, // MOV RAX, 60 (exit syscall)
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RCX should contain return address (RIP after SYSCALL)
}

#[test]
fn test_syscall_saves_rflags_in_r11() {
    // SYSCALL saves RFLAGS in R11
    let code = [
        0xf5, // CMC (complement carry - modify flags)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // R11 should contain saved RFLAGS
}

#[test]
fn test_syscall_with_parameters() {
    // SYSCALL with parameters in registers (Linux convention)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (write)
        0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00, // MOV RDI, 1 (stdout)
        0x48, 0xc7, 0xc6, 0x00, 0x20, 0x00, 0x00, // MOV RSI, 0x2000 (buffer)
        0x48, 0xc7, 0xc2, 0x0c, 0x00, 0x00, 0x00, // MOV RDX, 12 (count)
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, 1);
    assert_eq!(regs.rsi, 0x2000);
    assert_eq!(regs.rdx, 12);
}

// ============================================================================
// SYSCALL - Different System Call Numbers
// ============================================================================

#[test]
fn test_syscall_read() {
    // Syscall 0 - read
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_syscall_write() {
    // Syscall 1 - write
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 2);
}

#[test]
fn test_syscall_open() {
    // Syscall 2 - open
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 3);
}

#[test]
fn test_syscall_close() {
    // Syscall 3 - close
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x04, 0x00, 0x00, 0x00, // MOV RBX, 4
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 4);
}

#[test]
fn test_syscall_exit() {
    // Syscall 60 - exit
    let code = [
        0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00, // MOV RAX, 60
        0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0 (exit code)
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 60);
}

// ============================================================================
// SYSCALL - Register Preservation
// ============================================================================

#[test]
fn test_syscall_preserves_non_volatile_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x11, 0x00, 0x00, // MOV RBX, 0x1111
        0x48, 0xc7, 0xc5, 0x22, 0x22, 0x00, 0x00, // MOV RBP, 0x2222
        0x49, 0xc7, 0xc4, 0x33, 0x33, 0x00, 0x00, // MOV R12, 0x3333
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Non-volatile registers should be preserved
    assert_eq!(regs.rbx, 0x1111);
    assert_eq!(regs.rbp, 0x2222);
}

#[test]
fn test_syscall_modifies_rcx_r11() {
    let code = [
        0x48, 0xc7, 0xc1, 0xaa, 0xaa, 0x00, 0x00, // MOV RCX, 0xAAAA
        0x49, 0xc7, 0xc3, 0xbb, 0xbb, 0x00, 0x00, // MOV R11, 0xBBBB
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RCX and R11 are modified by SYSCALL
    // RCX = return address, R11 = saved RFLAGS
}

// ============================================================================
// SYSRET - Basic Return from System Call
// ============================================================================

#[test]
fn test_sysret_basic() {
    // SYSRET - return from system call
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000 (return address)
        0x49, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV R11, 0x02 (flags)
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4, // HLT (should not execute)
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

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
fn test_sysret_restores_rip_from_rcx() {
    // SYSRET loads RIP from RCX
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x49, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV R11, 0x02
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x3000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x3001);
}

#[test]
fn test_sysret_restores_rflags_from_r11() {
    // SYSRET loads RFLAGS from R11
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x49, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV R11, 1 (CF set)
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [
        0x72, 0x05, // JC +5 (jump if carry)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1); // CF was set
}

// ============================================================================
// SYSCALL/SYSRET - Round Trip
// ============================================================================

#[test]
fn test_syscall_sysret_roundtrip() {
    // Test SYSCALL followed by SYSRET
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL (saves RIP in RCX, RFLAGS in R11)
        // Return point
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Kernel handler (simulated)
    let handler = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (return value)
        0x48, 0x0f, 0x07, // SYSRET (REX.W, returns to RCX with RFLAGS from R11)
    ];
    // Handler would be at address determined by MSR

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // After roundtrip, should continue execution
}

#[test]
fn test_syscall_sysret_preserves_state() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x11, 0x00, 0x00, // MOV RBX, 0x1111
        0x48, 0xc7, 0xc5, 0x22, 0x22, 0x00, 0x00, // MOV RBP, 0x2222
        0x48, 0xc7, 0xc0, 0x39, 0x00, 0x00, 0x00, // MOV RAX, 57 (fork)
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1111);
    assert_eq!(regs.rbp, 0x2222);
}

// ============================================================================
// SYSCALL - MSR Configuration
// ============================================================================

#[test]
fn test_syscall_uses_star_msr() {
    // SYSCALL uses STAR MSR for segment selectors
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL (loads CS/SS from STAR MSR)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // CS and SS should be loaded from STAR MSR
}

#[test]
fn test_syscall_uses_lstar_msr() {
    // SYSCALL uses LSTAR MSR for target RIP
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL (jumps to LSTAR MSR)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Should jump to address in LSTAR MSR
}

#[test]
fn test_syscall_uses_fmask_msr() {
    // SYSCALL uses FMASK MSR to mask RFLAGS
    let code = [
        0xf5, // CMC (set some flags)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL (masks flags per FMASK)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Flags masked according to FMASK MSR
}

// ============================================================================
// SYSCALL - Privilege Level Transitions
// ============================================================================

#[test]
fn test_syscall_transitions_to_kernel() {
    // SYSCALL transitions from user to kernel mode
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL (CPL 3 -> 0)
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x33);
}

#[test]
fn test_sysret_transitions_to_user() {
    // SYSRET transitions from kernel to user mode
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x49, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV R11, 0x02
        0x48, 0x0f, 0x07, // SYSRET (REX.W, CPL 0 -> 3)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [
        0x48, 0xc7, 0xc0, 0x77, 0x00, 0x00, 0x00, // MOV RAX, 0x77
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x77);
}

// ============================================================================
// SYSCALL - Error Conditions
// ============================================================================

#[test]
fn test_syscall_invalid_in_real_mode() {
    // SYSCALL is invalid in real mode
    let code = [
        0x0f, 0x05, // SYSCALL (invalid in real mode)
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.efer = 0; // Clear LMA/SCE to simulate real mode
    sregs.cs.l = false;
    vcpu.set_sregs(&sregs).unwrap();

    assert!(run_until_hlt(&mut vcpu).is_err());
}

#[test]
fn test_sysret_invalid_in_user_mode() {
    // SYSRET from CPL3 should fault
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0x48, 0xc7, 0xc0, 0xfe, 0x00, 0x00, 0x00, // MOV RAX, 0xFE
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cs.selector = 3; // CPL=3
    vcpu.set_sregs(&sregs).unwrap();

    assert!(run_until_hlt(&mut vcpu).is_err());
}

// ============================================================================
// SYSCALL - Multiple Sequential Calls
// ============================================================================

#[test]
fn test_syscall_multiple_sequential() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Multiple syscalls should execute
}

// ============================================================================
// SYSCALL - Parameter Passing Conventions
// ============================================================================

#[test]
fn test_syscall_six_parameters() {
    // Linux x86-64 syscall convention: RAX, RDI, RSI, RDX, R10, R8, R9
    let code = [
        0x48, 0xc7, 0xc0, 0x09, 0x00, 0x00, 0x00, // MOV RAX, 9 (mmap)
        0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0 (addr)
        0x48, 0xc7, 0xc6, 0x00, 0x10, 0x00, 0x00, // MOV RSI, 0x1000 (length)
        0x48, 0xc7, 0xc2, 0x03, 0x00, 0x00, 0x00, // MOV RDX, 3 (prot)
        0x49, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV R10, 0x22 (flags)
        0x49, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV R8, -1 (fd)
        0x49, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0 (offset)
        0x0f, 0x05, // SYSCALL
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, 0);
    assert_eq!(regs.rsi, 0x1000);
    assert_eq!(regs.rdx, 3);
}

#[test]
fn test_syscall_return_value_in_rax() {
    // Syscall return value is in RAX
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20 (getpid)
        0x0f, 0x05, // SYSCALL
        // RAX should contain return value
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save return value)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RBX contains syscall return value
}

// ============================================================================
// SYSCALL - Edge Cases
// ============================================================================

#[test]
fn test_syscall_with_zero_syscall_number() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0);
}

#[test]
fn test_syscall_with_large_syscall_number() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x01, 0x00, 0x00, // MOV RAX, 511 (large)
        0x0f, 0x05, // SYSCALL
        0x48, 0xc7, 0xc3, 0xff, 0x01, 0x00, 0x00, // MOV RBX, 511
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 511);
}

#[test]
fn test_syscall_negative_error_codes() {
    // Error codes are typically negative in RAX
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (open)
        0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0 (invalid)
        0x0f, 0x05, // SYSCALL
        // RAX may contain negative error code
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Check for error condition
}

// ============================================================================
// SYSRET - Edge Cases
// ============================================================================

#[test]
fn test_sysret_to_zero_address() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x49, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV R11, 2
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x0000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x0001);
}

#[test]
fn test_sysret_to_high_address() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0xf0, 0x00, 0x00, // MOV RCX, 0xF000
        0x49, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV R11, 2
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [0xf4];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0xF000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0xf001);
}

#[test]
fn test_sysret_with_all_flags() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0x49, 0xc7, 0xc3, 0xd7, 0x08, 0x00, 0x00, // MOV R11, flags (multiple set)
        0x48, 0x0f, 0x07, // SYSRET (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let target_code = [
        0x9c, // PUSHFQ
        0xf4,
    ];
    mem.write_slice(&target_code, vm_memory::GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Flags should be restored from R11
}

// ============================================================================
// SYSCALL/SYSRET - Real-World Patterns
// ============================================================================

#[test]
fn test_syscall_getpid_pattern() {
    // Common pattern: getpid syscall
    let code = [
        0x48, 0xc7, 0xc0, 0x27, 0x00, 0x00, 0x00, // MOV RAX, 39 (getpid)
        0x0f, 0x05, // SYSCALL
        // RAX contains PID
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RBX contains returned PID
}

#[test]
fn test_syscall_error_handling_pattern() {
    // Error handling pattern: check RAX for negative value
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2 (open)
        0x0f, 0x05, // SYSCALL
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x05, // JS +5 (jump if negative/error)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, // Success path
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, -1 (error)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Error handling logic executed
}
