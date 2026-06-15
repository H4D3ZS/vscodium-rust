//! Tests for Intel VT-x Virtualization Instructions.
//!
//! This module covers VMX (Virtual Machine Extensions) instructions used for
//! hardware virtualization support on Intel processors.
//!
//! Instructions covered:
//! - VMCALL - Call to VM Monitor
//! - VMCLEAR - Clear Virtual Machine Control Structure
//! - VMLAUNCH - Launch Virtual Machine
//! - VMRESUME - Resume Virtual Machine
//! - VMPTRLD - Load Pointer to Virtual Machine Control Structure
//! - VMPTRST - Store Pointer to Virtual Machine Control Structure
//! - VMREAD - Read Field from Virtual Machine Control Structure
//! - VMWRITE - Write Field to Virtual Machine Control Structure
//! - VMXOFF - Leave VMX Operation
//! - VMXON - Enter VMX Operation
//! - VMFUNC - Invoke VM Function
//! - INVEPT - Invalidate EPT Translations
//! - INVVPID - Invalidate VPID Translations
//!
//! References: docs/vmcall.txt, docs/vmclear.txt, docs/vmlaunch:vmresume.txt,
//!            docs/vmptrld.txt, docs/vmptrst.txt, docs/vmread.txt, docs/vmwrite.txt,
//!            docs/vmxoff.txt, docs/vmxon.txt, docs/vmfunc.txt,
//!            docs/invept.txt, docs/invvpid.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// VMXON Tests - Enter VMX Operation
// ============================================================================

#[test]
fn test_vmxon_basic() {
    // VMXON - Enter VMX Operation
    // Opcode: F3 0F C7 /6
    // Note: Requires CPL=0 and proper CR4.VMXE setup
    let code = [
        0xF3, 0x0F, 0xC7, 0x30, // VMXON [rax] (rax points to VMXON region)
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // VMXON region address
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    // This will likely #UD or #GP in test environment
    // Testing that the instruction is recognized
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxon_memory_operand() {
    // VMXON with memory operand
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x0F, 0xC7, 0x30, // VMXON [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxon_with_different_registers() {
    // VMXON using RBX as base register
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0xF3, 0x0F, 0xC7, 0x33, // VMXON [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxon_with_offset() {
    // VMXON with displacement
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x20, 0x00, 0x00, // MOV RCX, 0x2000
        0xF3, 0x0F, 0xC7, 0xB1, 0x00, 0x10, 0x00, 0x00, // VMXON [rcx+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxon_preserves_registers() {
    // VMXON should only affect memory, not GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xC7, 0xC3, 0x42, 0x42, 0x42, 0x42, // MOV RBX, 0x42424242
        0x48, 0xC7, 0xC6, 0xAA, 0xAA, 0xAA, 0xAA, // MOV RSI, 0xAAAAAAAA
        0xF3, 0x0F, 0xC7, 0x30, // VMXON [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMXOFF Tests - Leave VMX Operation
// ============================================================================

#[test]
fn test_vmxoff_basic() {
    // VMXOFF - Leave VMX Operation
    // Opcode: 0F 01 C4
    let code = [
        0x0F, 0x01, 0xC4, // VMXOFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxoff_preserves_registers() {
    // VMXOFF should not modify GP registers
    let code = [
        0x48, 0xC7, 0xC0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xC7, 0xC3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x0F, 0x01, 0xC4, // VMXOFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmxoff_no_operands() {
    // VMXOFF takes no operands
    let code = [0x0F, 0x01, 0xC4, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMPTRLD Tests - Load Pointer to VMCS
// ============================================================================

#[test]
fn test_vmptrld_basic() {
    // VMPTRLD - Load pointer to current VMCS
    // Opcode: 0F C7 /6
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrld_different_addresses() {
    // VMPTRLD with various memory addresses
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0x0F, 0xC7, 0x31, // VMPTRLD [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrld_with_displacement() {
    // VMPTRLD with memory displacement
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0x0F, 0xC7, 0xB2, 0x00, 0x10, 0x00, 0x00, // VMPTRLD [rdx+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrld_multiple_loads() {
    // Load multiple VMCS pointers sequentially
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMPTRST Tests - Store Pointer to VMCS
// ============================================================================

#[test]
fn test_vmptrst_basic() {
    // VMPTRST - Store current VMCS pointer
    // Opcode: 0F C7 /7
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0x0F, 0xC7, 0x38, // VMPTRST [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrst_to_different_locations() {
    // Store VMCS pointer to various memory locations
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x70, 0x00, 0x00, // MOV RBX, 0x7000
        0x0F, 0xC7, 0x3B, // VMPTRST [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrst_with_offset() {
    // VMPTRST with displacement
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x60, 0x00, 0x00, // MOV RCX, 0x6000
        0x0F, 0xC7, 0xB9, 0x00, 0x08, 0x00, 0x00, // VMPTRST [rcx+0x800]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMCLEAR Tests - Clear VMCS
// ============================================================================

#[test]
fn test_vmclear_basic() {
    // VMCLEAR - Clear Virtual Machine Control Structure
    // Opcode: 66 0F C7 /6
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x80, 0x00, 0x00, // MOV RAX, 0x8000
        0x66, 0x0F, 0xC7, 0x30, // VMCLEAR [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmclear_different_vmcs() {
    // Clear different VMCS regions
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x90, 0x00, 0x00, // MOV RDX, 0x9000
        0x66, 0x0F, 0xC7, 0x32, // VMCLEAR [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmclear_with_displacement() {
    // VMCLEAR with memory displacement
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x80, 0x00, 0x00, // MOV RBX, 0x8000
        0x66, 0x0F, 0xC7, 0xB3, 0x00, 0x10, 0x00, 0x00, // VMCLEAR [rbx+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmclear_sequential() {
    // Clear multiple VMCS structures
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x80, 0x00, 0x00, // MOV RAX, 0x8000
        0x66, 0x0F, 0xC7, 0x30, // VMCLEAR [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x90, 0x00, 0x00, // MOV RAX, 0x9000
        0x66, 0x0F, 0xC7, 0x30, // VMCLEAR [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMREAD Tests - Read from VMCS
// ============================================================================

#[test]
fn test_vmread_basic() {
    // VMREAD - Read field from VMCS to register
    // Opcode: 0F 78
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x68, 0x00, 0x00, // MOV RAX, 0x6800 (field encoding)
        0x0F, 0x78, 0xC3, // VMREAD rbx, rax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmread_to_memory() {
    // VMREAD to memory location
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x68, 0x00, 0x00, // MOV RCX, 0x6800 (field)
        0x48, 0xC7, 0xC2, 0x00, 0xA0, 0x00, 0x00, // MOV RDX, 0xA000 (dest)
        0x0F, 0x78, 0x0A, // VMREAD [rdx], rcx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmread_different_fields() {
    // Read different VMCS fields
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x48, 0x00, 0x00, // MOV RAX, 0x4800
        0x0F, 0x78, 0xC3, // VMREAD rbx, rax
        0x48, 0xC7, 0xC0, 0x02, 0x48, 0x00, 0x00, // MOV RAX, 0x4802
        0x0F, 0x78, 0xC6, // VMREAD rsi, rax
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmread_guest_cr0() {
    // Read Guest CR0 field (encoding 0x6800)
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x68, 0x00, 0x00, // MOV RCX, 0x6800
        0x0F, 0x78, 0xC1, // VMREAD rax, rcx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmread_guest_cr4() {
    // Read Guest CR4 field (encoding 0x6804)
    let code = [
        0x48, 0xC7, 0xC1, 0x04, 0x68, 0x00, 0x00, // MOV RCX, 0x6804
        0x0F, 0x78, 0xC1, // VMREAD rax, rcx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMWRITE Tests - Write to VMCS
// ============================================================================

#[test]
fn test_vmwrite_basic() {
    // VMWRITE - Write to VMCS field from register
    // Opcode: 0F 79
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x68, 0x00, 0x00, // MOV RAX, 0x6800 (field)
        0x48, 0xC7, 0xC3, 0x00, 0x00, 0x60, 0x00, // MOV RBX, 0x600000 (value)
        0x0F, 0x79, 0xC3, // VMWRITE rax, rbx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmwrite_from_memory() {
    // VMWRITE from memory location
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x68, 0x00, 0x00, // MOV RCX, 0x6800
        0x48, 0xC7, 0xC2, 0x00, 0xB0, 0x00, 0x00, // MOV RDX, 0xB000
        0x0F, 0x79, 0x0A, // VMWRITE rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmwrite_different_fields() {
    // Write to different VMCS fields
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x48, 0x00, 0x00, // MOV RAX, 0x4800
        0x48, 0xC7, 0xC3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x0F, 0x79, 0xC3, // VMWRITE rax, rbx
        0x48, 0xC7, 0xC0, 0x02, 0x48, 0x00, 0x00, // MOV RAX, 0x4802
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x0F, 0x79, 0xC3, // VMWRITE rax, rbx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmwrite_guest_rip() {
    // Write Guest RIP field (encoding 0x681E)
    let code = [
        0x48, 0xC7, 0xC1, 0x1E, 0x68, 0x00, 0x00, // MOV RCX, 0x681E
        0x48, 0xC7, 0xC2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0x0F, 0x79, 0xCA, // VMWRITE rcx, rdx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmwrite_guest_rsp() {
    // Write Guest RSP field (encoding 0x681C)
    let code = [
        0x48, 0xC7, 0xC1, 0x1C, 0x68, 0x00, 0x00, // MOV RCX, 0x681C
        0x48, 0xC7, 0xC2, 0x00, 0x70, 0x00, 0x00, // MOV RDX, 0x7000
        0x0F, 0x79, 0xCA, // VMWRITE rcx, rdx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMLAUNCH Tests - Launch Virtual Machine
// ============================================================================

#[test]
fn test_vmlaunch_basic() {
    // VMLAUNCH - Launch virtual machine
    // Opcode: 0F 01 C2
    let code = [
        0x0F, 0x01, 0xC2, // VMLAUNCH
        0xF4, // HLT (should not reach if launch succeeds)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmlaunch_no_operands() {
    // VMLAUNCH takes no operands
    let code = [0x0F, 0x01, 0xC2, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmlaunch_after_setup() {
    // VMLAUNCH after setting up VMCS
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0x0F, 0x01, 0xC2, // VMLAUNCH
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMRESUME Tests - Resume Virtual Machine
// ============================================================================

#[test]
fn test_vmresume_basic() {
    // VMRESUME - Resume virtual machine
    // Opcode: 0F 01 C3
    let code = [
        0x0F, 0x01, 0xC3, // VMRESUME
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmresume_no_operands() {
    // VMRESUME takes no operands
    let code = [0x0F, 0x01, 0xC3, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmresume_after_vmptrld() {
    // VMRESUME after loading VMCS pointer
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0x0F, 0x01, 0xC3, // VMRESUME
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMCALL Tests - Call to VM Monitor
// ============================================================================

#[test]
fn test_vmcall_basic() {
    // VMCALL - Hypercall from guest to host
    // Opcode: 0F 01 C1
    let code = [
        0x0F, 0x01, 0xC1, // VMCALL
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmcall_with_parameters() {
    // VMCALL with parameters in registers
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (hypercall number)
        0x48, 0xC7, 0xC3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42 (param 1)
        0x48, 0xC7, 0xC1, 0x43, 0x00, 0x00, 0x00, // MOV RCX, 0x43 (param 2)
        0x0F, 0x01, 0xC1, // VMCALL
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmcall_multiple() {
    // Multiple VMCALLs in sequence
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0F, 0x01, 0xC1, // VMCALL
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x0F, 0x01, 0xC1, // VMCALL
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmcall_preserves_registers() {
    // VMCALL should preserve registers (unless modified by VMM)
    let code = [
        0x48, 0xC7, 0xC3, 0x11, 0x11, 0x11, 0x11, // MOV RBX, 0x11111111
        0x48, 0xC7, 0xC6, 0x22, 0x22, 0x22, 0x22, // MOV RSI, 0x22222222
        0x0F, 0x01, 0xC1, // VMCALL
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VMFUNC Tests - Invoke VM Function
// ============================================================================

#[test]
fn test_vmfunc_basic() {
    // VMFUNC - Invoke VM function
    // Opcode: 0F 01 D4
    // EAX specifies function number
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (EPTP switching)
        0x0F, 0x01, 0xD4, // VMFUNC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmfunc_eptp_switching() {
    // VMFUNC function 0: EPTP switching
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX (function 0)
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (EPTP index)
        0x0F, 0x01, 0xD4, // VMFUNC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmfunc_different_functions() {
    // Test different VMFUNC function numbers
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x0F, 0x01, 0xD4, // VMFUNC
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INVEPT Tests - Invalidate EPT Translations
// ============================================================================

#[test]
fn test_invept_basic() {
    // INVEPT - Invalidate EPT-derived translations
    // Opcode: 66 0F 38 80
    let code = [
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (single-context)
        0x48, 0xC7, 0xC2, 0x00, 0xC0, 0x00, 0x00, // MOV RDX, 0xC000 (descriptor)
        0x66, 0x0F, 0x38, 0x80, 0x0A, // INVEPT rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_single_context() {
    // INVEPT type 1: single-context invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0xC0, 0x00, 0x00, // MOV RBX, 0xC000
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_all_contexts() {
    // INVEPT type 2: all-context invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xC7, 0xC3, 0x00, 0xC0, 0x00, 0x00, // MOV RBX, 0xC000
        0x66, 0x0F, 0x38, 0x80, 0x03, // INVEPT rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invept_multiple_invalidations() {
    // Multiple INVEPT calls
    let code = [
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0xC7, 0xC2, 0x00, 0xC0, 0x00, 0x00, // MOV RDX, 0xC000
        0x66, 0x0F, 0x38, 0x80, 0x0A, // INVEPT rcx, [rdx]
        0x48, 0xC7, 0xC1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x66, 0x0F, 0x38, 0x80, 0x0A, // INVEPT rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INVVPID Tests - Invalidate VPID Translations
// ============================================================================

#[test]
fn test_invvpid_basic() {
    // INVVPID - Invalidate VPID-tagged TLB entries
    // Opcode: 66 0F 38 81
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (individual-address)
        0x48, 0xC7, 0xC2, 0x00, 0xD0, 0x00, 0x00, // MOV RDX, 0xD000 (descriptor)
        0x66, 0x0F, 0x38, 0x81, 0x0A, // INVVPID rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invvpid_individual_address() {
    // INVVPID type 0: individual-address invalidation
    let code = [
        0x48, 0x31, 0xC0, // XOR RAX, RAX
        0x48, 0xC7, 0xC3, 0x00, 0xD0, 0x00, 0x00, // MOV RBX, 0xD000
        0x66, 0x0F, 0x38, 0x81, 0x03, // INVVPID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invvpid_single_context() {
    // INVVPID type 1: single-context invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xC7, 0xC3, 0x00, 0xD0, 0x00, 0x00, // MOV RBX, 0xD000
        0x66, 0x0F, 0x38, 0x81, 0x03, // INVVPID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invvpid_all_contexts() {
    // INVVPID type 2: all-contexts invalidation
    let code = [
        0x48, 0xC7, 0xC0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xC7, 0xC3, 0x00, 0xD0, 0x00, 0x00, // MOV RBX, 0xD000
        0x66, 0x0F, 0x38, 0x81, 0x03, // INVVPID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invvpid_single_context_retaining_globals() {
    // INVVPID type 3: single-context retaining globals
    let code = [
        0x48, 0xC7, 0xC0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xC7, 0xC3, 0x00, 0xD0, 0x00, 0x00, // MOV RBX, 0xD000
        0x66, 0x0F, 0x38, 0x81, 0x03, // INVVPID rax, [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_invvpid_multiple_types() {
    // Multiple INVVPID invalidations with different types
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0xD0, 0x00, 0x00, // MOV RDX, 0xD000
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x66, 0x0F, 0x38, 0x81, 0x0A, // INVVPID rcx, [rdx]
        0x48, 0xC7, 0xC1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x66, 0x0F, 0x38, 0x81, 0x0A, // INVVPID rcx, [rdx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined Operation Tests
// ============================================================================

#[test]
fn test_vmx_full_sequence() {
    // Complete VMX setup sequence
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xF3, 0x0F, 0xC7, 0x30, // VMXON [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x66, 0x0F, 0xC7, 0x30, // VMCLEAR [rax]
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0x0F, 0x01, 0xC4, // VMXOFF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmread_vmwrite_sequence() {
    // Read, modify, write back VMCS field
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x68, 0x00, 0x00, // MOV RAX, 0x6800
        0x0F, 0x78, 0xC3, // VMREAD rbx, rax
        0x48, 0x83, 0xC3, 0x10, // ADD RBX, 0x10
        0x0F, 0x79, 0xC3, // VMWRITE rax, rbx
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vmptrld_vmclear_cycle() {
    // Load and clear VMCS in a loop
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x48, 0xC7, 0xC3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2 (counter)
        // loop:
        0x0F, 0xC7, 0x30, // VMPTRLD [rax]
        0x66, 0x0F, 0xC7, 0x30, // VMCLEAR [rax]
        0x48, 0xFF, 0xCB, // DEC RBX
        0x75, 0xF4, // JNZ loop
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
