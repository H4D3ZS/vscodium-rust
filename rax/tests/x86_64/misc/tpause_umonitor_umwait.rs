//! Tests for User-Level Wait and Monitor Instructions.
//!
//! This module covers user-level wait and monitor instructions that provide
//! efficient power management and synchronization primitives.
//!
//! Instructions covered:
//! - TPAUSE - Timed Pause
//! - UMONITOR - Set up linear address range for monitoring
//! - UMWAIT - User-level Wait
//!
//! These instructions are part of the WAITPKG extension (introduced in Tremont).
//!
//! References: Intel SDM Vol. 2, WAITPKG instruction set documentation

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// TPAUSE Tests - Timed Pause
// ============================================================================

#[test]
fn test_tpause_basic_c01() {
    // TPAUSE - Timed pause with C0.1 state
    // 66 0F AE /6
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX (EDX:EAX = TSC deadline)
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x31, 0xC9, // XOR ECX, ECX (state = C0.1)
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_basic_c02() {
    // TPAUSE - Timed pause with C0.2 state (deeper power saving)
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (state = C0.2)
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_short_timeout() {
    // TPAUSE with very short timeout
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100 (short timeout)
        0x31, 0xC9, // XOR ECX, ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_long_timeout() {
    // TPAUSE with longer timeout
    let code = [
        0xBA, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1 (high 32 bits)
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (low 32 bits)
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_zero_timeout() {
    // TPAUSE with zero timeout (immediate return)
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x31, 0xC9, // XOR ECX, ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_multiple_sequential() {
    // Multiple sequential TPAUSE instructions
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x32, 0x00, 0x00, 0x00, // MOV EAX, 50
        0x31, 0xC9, // XOR ECX, ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_with_different_states() {
    // Test TPAUSE with different C-states
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        // C0.1 state
        0x31, 0xC9, // XOR ECX, ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        // C0.2 state
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// UMONITOR Tests - User Monitor
// ============================================================================

#[test]
fn test_umonitor_basic() {
    // UMONITOR - Set up address range for monitoring
    // F3 0F AE /6
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_different_addresses() {
    // Test UMONITOR with different addresses
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_with_rbx() {
    // Test UMONITOR with different register
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0xF3, 0x0F, 0xAE, 0xF3, // UMONITOR RBX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_with_rcx() {
    // Test UMONITOR with RCX
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xF3, 0x0F, 0xAE, 0xF1, // UMONITOR RCX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_cache_line_aligned() {
    // Test UMONITOR with cache-line aligned address
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000 (aligned)
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_unaligned() {
    // Test UMONITOR with unaligned address
    let code = [
        0x48, 0xC7, 0xC0, 0x07, 0x70, 0x00, 0x00, // MOV RAX, 0x7007 (unaligned)
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// UMWAIT Tests - User Wait
// ============================================================================

#[test]
fn test_umwait_basic_c01() {
    // UMWAIT - User-level wait with C0.1 state
    // F2 0F AE /6
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x31, 0xC9, // XOR ECX, ECX (state = C0.1)
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umwait_basic_c02() {
    // UMWAIT - User-level wait with C0.2 state
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (state = C0.2)
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umwait_short_timeout() {
    // UMWAIT with short timeout
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0xC8, 0x00, 0x00, 0x00, // MOV EAX, 200
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umwait_zero_timeout() {
    // UMWAIT with zero timeout
    let code = [
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Tests - UMONITOR and UMWAIT
// ============================================================================

#[test]
fn test_umonitor_umwait_basic() {
    // UMONITOR followed by UMWAIT
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_umwait_with_memory_write() {
    // UMONITOR, write to memory, then UMWAIT
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        // Write to monitored address (should wake UMWAIT early)
        0x48, 0xC7, 0x00, 0x42, 0x00, 0x00, 0x00, // MOV QWORD PTR [RAX], 0x42
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x00, 0x10, 0x00, 0x00, // MOV EAX, 0x1000 (long timeout)
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX (should wake immediately)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_umwait_different_addresses() {
    // Monitor one address but write to different address
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0x48, 0xC7, 0xC1, 0x00, 0x40, 0x00, 0x00, // MOV RCX, 0x4000 (different address)
        0x48, 0xC7, 0x01, 0x55, 0x00, 0x00, 0x00, // MOV QWORD PTR [RCX], 0x55
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX (should timeout)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_multiple_umonitor_umwait_cycles() {
    // Multiple UMONITOR/UMWAIT cycles
    let code = [
        // Cycle 1
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x32, 0x00, 0x00, 0x00, // MOV EAX, 50
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        // Cycle 2
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xB8, 0x32, 0x00, 0x00, 0x00, // MOV EAX, 50
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_vs_umwait() {
    // Compare TPAUSE vs UMWAIT behavior
    let code = [
        // TPAUSE (unconditional wait)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        0x31, 0xC9, // XOR ECX, ECX
        0x66, 0x0F, 0xAE, 0xF1, // TPAUSE ECX
        // UMWAIT (can be interrupted by memory writes)
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        0x31, 0xC9, // XOR ECX, ECX
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_waitpkg_power_management() {
    // Test power management scenarios
    let code = [
        // Setup monitoring
        0x48, 0xC7, 0xC0, 0x00, 0x80, 0x00, 0x00, // MOV RAX, 0x8000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        // Short wait with C0.1 (lighter sleep)
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0xB8, 0x32, 0x00, 0x00, 0x00, // MOV EAX, 50
        0x31, 0xC9, // XOR ECX, ECX (C0.1)
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        // Longer wait with C0.2 (deeper sleep)
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR RAX
        0xB8, 0xE8, 0x03, 0x00, 0x00, // MOV EAX, 1000
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (C0.2)
        0xF2, 0x0F, 0xAE, 0xF1, // UMWAIT ECX
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
