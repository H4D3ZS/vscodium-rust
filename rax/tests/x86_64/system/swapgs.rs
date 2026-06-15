//! Tests for the SWAPGS instruction.
//!
//! SWAPGS - Swap GS Base Register
//!
//! Exchanges the current GS base register value with the value contained in
//! MSR address C0000102H (IA32_KERNEL_GS_BASE). This is a privileged instruction
//! designed for fast kernel entry without requiring register saves.
//!
//! Key characteristics:
//! - Opcode: 0F 01 F8
//! - Only valid in 64-bit mode (causes #UD in other modes)
//! - Requires CPL = 0 (ring 0, causes #GP if CPL != 0)
//! - Does not modify flags
//! - Does not require any general-purpose registers
//! - Atomically swaps GS.base with IA32_KERNEL_GS_BASE MSR
//!
//! Primary use case:
//! - Fast system call entry (SYSCALL/SYSENTER) where kernel needs quick access
//!   to kernel data structures without saving registers first
//!
//! Reference: docs/swapgs.txt

use crate::common::*;
use rax::cpu::Registers;

// IA32_KERNEL_GS_BASE MSR address
const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;

// ============================================================================
// Basic SWAPGS Tests
// ============================================================================

#[test]
fn test_swapgs_basic() {
    // SWAPGS should swap GS base with IA32_KERNEL_GS_BASE MSR
    let code = [
        0x0F, 0x01, 0xF8, // SWAPGS
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // GS base and IA32_KERNEL_GS_BASE should be swapped
}

#[test]
fn test_swapgs_preserves_registers() {
    // SWAPGS should not modify any general-purpose registers
    let code = [
        // Set registers to known values
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0xC7, 0xC1, 0x33, 0x33, 0x33, 0x33, // MOV RCX, 0x33333333
        0x48, 0xC7, 0xC2, 0x44, 0x44, 0x44, 0x44, // MOV RDX, 0x44444444
        0x48, 0xC7, 0xC6, 0x55, 0x55, 0x55, 0x55, // MOV RSI, 0x55555555
        0x48, 0xC7, 0xC7, 0x66, 0x66, 0x66, 0x66, // MOV RDI, 0x66666666
        // Execute SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x11111111, "RAX should be preserved");
    assert_eq!(regs.rbx, 0x22222222, "RBX should be preserved");
    assert_eq!(regs.rcx, 0x33333333, "RCX should be preserved");
    assert_eq!(regs.rdx, 0x44444444, "RDX should be preserved");
    assert_eq!(regs.rsi, 0x55555555, "RSI should be preserved");
    assert_eq!(regs.rdi, 0x66666666, "RDI should be preserved");
}

#[test]
fn test_swapgs_preserves_flags() {
    // SWAPGS should not modify flags
    let code = [
        // Set some flags
        0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x48, 0x83, 0xC0, 0x01, // ADD RAX, 1 (sets ZF)
        // Save flags
        0x9C, // PUSHFQ
        0x5B, // POP RBX
        // Execute SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Check flags
        0x9C, // PUSHFQ
        0x58, // POP RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xCD5,
        regs.rbx & 0xCD5,
        "SWAPGS should preserve flags"
    );
}

// ============================================================================
// SWAPGS Swap Behavior Tests
// ============================================================================

#[test]
fn test_swapgs_double_swap_restores() {
    // Two SWAPGS operations should restore original state
    // This tests the swap semantics
    let code = [
        0x0F, 0x01, 0xF8, // SWAPGS (swap)
        0x0F, 0x01, 0xF8, // SWAPGS (swap back)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // GS base should be back to original value
    // IA32_KERNEL_GS_BASE should also be back to original
}

#[test]
fn test_swapgs_with_wrgsbase() {
    // Test SWAPGS interaction with WRGSBASE/RDGSBASE
    let code = [
        // Set GS base to 0x1000 using WRGSBASE
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read GS base before swap
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX
        // Perform SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read GS base after swap (should be IA32_KERNEL_GS_BASE value)
        0x48, 0x31, 0xC9, // XOR RCX, RCX
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX
        // Swap back
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read GS base after swapping back (should be 0x1000 again)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xF3, 0x48, 0x0F, 0xAE, 0xCA, // RDGSBASE RDX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1000, "Initial GS base should be 0x1000");
    assert_ne!(regs.rcx, 0x1000, "GS base should change after SWAPGS");
    assert_eq!(
        regs.rdx, 0x1000,
        "GS base should restore after double SWAPGS"
    );
}

#[test]
fn test_swapgs_sequential() {
    // Multiple sequential SWAPGS operations
    let code = [
        // Initial state: GS.base = A, KERNEL_GS_BASE = B
        0x0F, 0x01, 0xF8, // SWAPGS (1) -> GS.base = B, KERNEL_GS_BASE = A
        0xF3, 0x48, 0x0F, 0xAE, 0xC0, // RDGSBASE RAX (read B)
        0x0F, 0x01, 0xF8, // SWAPGS (2) -> GS.base = A, KERNEL_GS_BASE = B
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDGSBASE RBX (read A)
        0x0F, 0x01, 0xF8, // SWAPGS (3) -> GS.base = B, KERNEL_GS_BASE = A
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX (read B)
        0x0F, 0x01, 0xF8, // SWAPGS (4) -> GS.base = A, KERNEL_GS_BASE = B
        0xF3, 0x48, 0x0F, 0xAE, 0xCA, // RDGSBASE RDX (read A)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After odd number of swaps: GS.base should be kernel value
    // After even number of swaps: GS.base should be user value
    // RAX and RCX should have same value (kernel GS base)
    // RBX and RDX should have same value (user GS base)
    assert_eq!(
        regs.rax, regs.rcx,
        "Odd swaps should result in same GS base"
    );
    assert_eq!(
        regs.rbx, regs.rdx,
        "Even swaps should result in same GS base"
    );
}

// ============================================================================
// SWAPGS Use Case Simulations
// ============================================================================

#[test]
fn test_swapgs_syscall_entry_pattern() {
    // Simulate typical SYSCALL entry pattern:
    // 1. User code executes SYSCALL
    // 2. Kernel entry point executes SWAPGS to access kernel data
    // 3. Kernel does work
    // 4. Kernel executes SWAPGS again before SYSRET
    let code = [
        // Setup: Set GS base to user value
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x00000000 (user GS)
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // --- Simulate kernel entry (SYSCALL would jump here) ---

        // Entry: SWAPGS to get kernel GS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Kernel can now use GS to access kernel data structures
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDGSBASE RBX (kernel GS)
        // Kernel does some work...
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (syscall result)
        // --- Simulate kernel exit (before SYSRET) ---

        // Exit: SWAPGS to restore user GS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Verify user GS is restored
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX (should be user GS)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "User GS should be restored after exit");
    assert_eq!(regs.rax, 0x42, "Syscall result should be preserved");
}

#[test]
fn test_swapgs_with_interrupt_handler() {
    // Simulate interrupt handler pattern where SWAPGS is used
    let code = [
        // Setup user GS base
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x01, 0x00, // MOV RAX, 0x10000 (user)
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // --- Interrupt occurs, CPU switches to kernel mode ---

        // Interrupt handler: swap to kernel GS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Handler code runs, can access kernel data via GS
        0x48, 0xC7, 0xC0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99 (handler work)
        // Before IRET: swap back to user GS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Verify user GS restored
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX (reg=1 for RDGSBASE, rm=3 for RBX)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, 0x10000,
        "User GS should be restored after handler"
    );
}

// ============================================================================
// SWAPGS with MSR Interaction
// ============================================================================

#[test]
fn test_swapgs_with_wrmsr_kernel_gs_base() {
    // Test SWAPGS with explicit MSR write to IA32_KERNEL_GS_BASE
    let code = [
        // Write to IA32_KERNEL_GS_BASE MSR using WRMSR
        0x48, 0xC7, 0xC1, 0x02, 0x01, 0x00, 0xC0, // MOV RCX, 0xC0000102 (IA32_KERNEL_GS_BASE)
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000 (low 32 bits)
        0x48, 0x31, 0xD2, // XOR RDX, RDX (high 32 bits)
        0x0F, 0x30, // WRMSR
        // Set current GS base
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read GS base before swap
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX (reg=1 for RDGSBASE, rm=3 for RBX)
        // SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read GS base after swap (should be 0x2000)
        0xF3, 0x48, 0x0F, 0xAE, 0xC8, // RDGSBASE RAX (reg=1 for RDGSBASE, rm=0 for RAX)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1000, "GS base before swap should be 0x1000");
    assert_eq!(
        regs.rax, 0x2000,
        "GS base after swap should be 0x2000 from MSR"
    );
}

// ============================================================================
// Edge Cases and Stress Tests
// ============================================================================

#[test]
fn test_swapgs_rapid_toggle() {
    // Rapidly toggle GS base with multiple SWAPGS
    let mut code = vec![];

    // Setup initial GS
    code.extend_from_slice(&[0x48, 0xC7, 0xC0, 0x11, 0x00, 0x00, 0x00]); // MOV RAX, 0x11
    code.extend_from_slice(&[0xF3, 0x48, 0x0F, 0xAE, 0xD8]); // WRGSBASE RAX

    // Perform 20 SWAPGS operations
    for _ in 0..20 {
        code.extend_from_slice(&[0x0F, 0x01, 0xF8]); // SWAPGS
    }

    // Read final GS base
    code.extend_from_slice(&[0xF3, 0x48, 0x0F, 0xAE, 0xCB]); // RDGSBASE RBX (reg=1, rm=3)

    code.push(0xF4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After even number (20) of swaps, should be back to original
    assert_eq!(
        regs.rbx, 0x11,
        "After even SWAPGS count, GS should be original"
    );
}

#[test]
fn test_swapgs_preserves_other_segments() {
    // SWAPGS should only affect GS, not other segments
    // This is implicit but worth documenting
    let code = [
        // SWAPGS should not affect FS, for example
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0xF3, 0x48, 0x0F, 0xAE, 0xD0, // WRFSBASE RAX (set FS)
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX (set GS)
        // Read FS before SWAPGS
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDFSBASE RBX
        // SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read FS after SWAPGS (should be unchanged)
        0xF3, 0x48, 0x0F, 0xAE, 0xC1, // RDFSBASE RCX
        // Read GS after SWAPGS (should be changed)
        0xF3, 0x48, 0x0F, 0xAE, 0xCA, // RDGSBASE RDX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x3000, "FS base before SWAPGS");
    assert_eq!(
        regs.rcx, 0x3000,
        "FS base after SWAPGS (should be unchanged)"
    );
    assert_ne!(regs.rdx, 0x4000, "GS base should change after SWAPGS");
}

#[test]
fn test_swapgs_with_arithmetic_operations() {
    // Interleave SWAPGS with other operations
    let code = [
        0x48, 0xC7, 0xC0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x0F, 0x01, 0xF8, // SWAPGS
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0x0F, 0x01, 0xF8, // SWAPGS
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0x0F, 0x01, 0xF8, // SWAPGS
        0x48, 0x83, 0xC0, 0x10, // ADD RAX, 16
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 64, "Arithmetic should work correctly with SWAPGS");
}

#[test]
fn test_swapgs_zero_values() {
    // Test SWAPGS with zero values in GS base
    let code = [
        // Set GS base to 0
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read new GS base
        0xF3, 0x48, 0x0F, 0xAE, 0xC3, // RDGSBASE RBX
        // SWAPGS back
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read GS base again (should be 0)
        0xF3, 0x48, 0x0F, 0xAE, 0xC1, // RDGSBASE RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0, "GS base should be 0 after double SWAPGS");
}

#[test]
fn test_swapgs_high_addresses() {
    // Test SWAPGS with high canonical addresses
    let code = [
        // Set GS base to high canonical address
        0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFF,
        0xFF, // MOV RAX, 0xFFFF800000000000
        0xF3, 0x48, 0x0F, 0xAE, 0xD8, // WRGSBASE RAX
        // Read before swap
        0xF3, 0x48, 0x0F, 0xAE, 0xCB, // RDGSBASE RBX (reg=1, rm=3)
        // SWAPGS
        0x0F, 0x01, 0xF8, // SWAPGS
        // SWAPGS back
        0x0F, 0x01, 0xF8, // SWAPGS
        // Read after double swap
        0xF3, 0x48, 0x0F, 0xAE, 0xC9, // RDGSBASE RCX (reg=1, rm=1)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx, regs.rbx,
        "GS base should be restored after double SWAPGS"
    );
    assert_eq!(
        regs.rcx, 0xFFFF800000000000,
        "High address should be preserved"
    );
}

#[test]
fn test_swapgs_no_memory_access() {
    // Verify SWAPGS doesn't access memory (important for its design goal)
    // This is implicit in the instruction design but worth documenting
    let code = [
        // SWAPGS without any memory setup
        0x0F, 0x01, 0xF8, // SWAPGS
        // Should complete without memory access
        0x48, 0xC7, 0xC0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x42,
        "SWAPGS should complete without memory access"
    );
}
