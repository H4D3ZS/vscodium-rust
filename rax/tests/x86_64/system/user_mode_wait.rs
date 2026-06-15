//! Tests for User-Mode Wait and Interrupt Instructions.
//!
//! This module covers user-mode wait, interrupt, and trace instructions.
//!
//! Instructions covered:
//! - UMONITOR - User-level Monitor Address
//! - UMWAIT - User-level Wait
//! - TPAUSE - Timed Pause
//! - CLUI - Clear User Interrupt Flag
//! - STUI - Set User Interrupt Flag
//! - PTWRITE - Write to Processor Trace Packet
//!
//! References: docs/umonitor.txt, docs/umwait.txt, docs/tpause.txt,
//!            docs/clui.txt, docs/stui.txt, docs/ptwrite.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// UMONITOR Tests - User-Level Monitor
// ============================================================================

#[test]
fn test_umonitor_basic() {
    // UMONITOR - Set up address range to monitor
    // Opcode: F3 0F AE /6
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR rax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_different_addresses() {
    // Monitor different addresses
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xF3, 0x0F, 0xAE, 0xF3, // UMONITOR rbx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umonitor_multiple() {
    // Multiple UMONITOR calls (last one wins)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR rax
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR rax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// UMWAIT Tests - User-Level Wait
// ============================================================================

#[test]
fn test_umwait_basic() {
    // UMWAIT - Wait until timeout or wake event
    // Opcode: F2 0F AE /6
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (C0.1 state)
        0x48, 0xC7, 0xC2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000 (timeout high)
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (timeout low)
        0xF2, 0x0F, 0xAE, 0xF0, // UMWAIT eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umwait_c0_2_state() {
    // UMWAIT with C0.2 state (EAX bit 0 = 1)
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (C0.2 state)
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0x00, 0x00, // MOV RDX, 0xFFFF
        0x48, 0xC7, 0xC3, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RBX, 0xFFFFFFFF
        0xF2, 0x0F, 0xAE, 0xF0, // UMWAIT eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_umwait_after_umonitor() {
    // UMWAIT after setting up monitor
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0xF3, 0x0F, 0xAE, 0xF1, // UMONITOR rcx
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC2, 0x10, 0x00, 0x00, 0x00, // MOV RDX, 0x10
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0xF2, 0x0F, 0xAE, 0xF0, // UMWAIT eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TPAUSE Tests - Timed Pause
// ============================================================================

#[test]
fn test_tpause_basic() {
    // TPAUSE - Timed pause
    // Opcode: 66 0F AE /6
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (C0.1 state)
        0x48, 0xC7, 0xC2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0x66, 0x0F, 0xAE, 0xF0, // TPAUSE eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_c0_2_state() {
    // TPAUSE with C0.2 state
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0x00, 0x00, // MOV RDX, 0xFFFF
        0x48, 0xC7, 0xC3, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RBX, 0xFFFFFFFF
        0x66, 0x0F, 0xAE, 0xF0, // TPAUSE eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tpause_short_timeout() {
    // Short timeout
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0x31, 0xD2, // XOR RDX, RDX
        0x48, 0xC7, 0xC3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 0x10
        0x66, 0x0F, 0xAE, 0xF0, // TPAUSE eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CLUI/STUI Tests - User Interrupt Flag Control
// ============================================================================

#[test]
fn test_clui_basic() {
    // CLUI - Clear User Interrupt Flag
    // Opcode: F3 0F 01 EE
    let code = [
        0xF3, 0x0F, 0x01, 0xEE, // CLUI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_stui_basic() {
    // STUI - Set User Interrupt Flag
    // Opcode: F3 0F 01 EF
    let code = [
        0xF3, 0x0F, 0x01, 0xEF, // STUI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_clui_stui_sequence() {
    // Set and clear user interrupt flag
    let code = [
        0xF3, 0x0F, 0x01, 0xEF, // STUI
        0xF3, 0x0F, 0x01, 0xEE, // CLUI
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// PTWRITE Tests - Processor Trace Write
// ============================================================================

#[test]
fn test_ptwrite_basic() {
    // PTWRITE - Write data to processor trace
    // Opcode: F3 0F AE /4
    let code = [
        0x48, 0xC7, 0xC0, 0x42, 0x42, 0x42, 0x42, // MOV RAX, 0x42424242
        0xF3, 0x0F, 0xAE, 0xE0, // PTWRITE eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ptwrite64() {
    // PTWRITE with 64-bit value
    let code = [
        0x48, 0xB8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, // MOV RAX, 0x1122334455667788
        0xF3, 0x48, 0x0F, 0xAE, 0xE0, // PTWRITE rax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ptwrite_multiple() {
    // Multiple trace writes
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0xAE, 0xE0, // PTWRITE eax
        0x48, 0xC7, 0xC1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xF3, 0x0F, 0xAE, 0xE1, // PTWRITE ecx
        0x48, 0xC7, 0xC2, 0x03, 0x00, 0x00, 0x00, // MOV RDX, 3
        0xF3, 0x0F, 0xAE, 0xE2, // PTWRITE edx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Operation Tests
// ============================================================================

#[test]
fn test_monitor_wait_workflow() {
    // Complete monitor-wait workflow
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xF3, 0x0F, 0xAE, 0xF0, // UMONITOR rax
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC2, 0x10, 0x00, 0x00, 0x00, // MOV RDX, 0x10
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0xF2, 0x0F, 0xAE, 0xF0, // UMWAIT eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_user_interrupt_control() {
    // User interrupt flag control workflow
    let code = [
        0xF3, 0x0F, 0x01, 0xEF, // STUI (enable user interrupts)
        // Critical section where user interrupts are enabled
        0xF3, 0x0F, 0x01, 0xEE, // CLUI (disable user interrupts)
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ptwrite_with_pause() {
    // Trace writes with pause
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xF3, 0x0F, 0xAE, 0xE0, // PTWRITE eax
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC2, 0x10, 0x00, 0x00, 0x00, // MOV RDX, 0x10
        0x48, 0x31, 0xDB, // XOR RBX, RBX
        0x66, 0x0F, 0xAE, 0xF0, // TPAUSE eax
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xF3, 0x0F, 0xAE, 0xE0, // PTWRITE eax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
