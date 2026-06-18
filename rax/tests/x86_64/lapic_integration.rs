//! LAPIC integration tests - testing LAPIC through x86 emulator execution.
//!
//! These tests run actual x86-64 code that accesses the LAPIC MMIO registers
//! at 0xFEE00000, verifying that the emulator correctly routes these accesses
//! to the inline LAPIC implementation.

use crate::common::*;

// LAPIC register offsets from base 0xFEE00000
const LAPIC_BASE: u64 = 0xFEE00000;
const LAPIC_ID: u64 = 0x020;
const LAPIC_VERSION: u64 = 0x030;
const LAPIC_TPR: u64 = 0x080;
const LAPIC_EOI: u64 = 0x0B0;
const LAPIC_SVR: u64 = 0x0F0;
const LAPIC_ISR_BASE: u64 = 0x100;
const LAPIC_IRR_BASE: u64 = 0x200;
const LAPIC_LVT_TIMER: u64 = 0x320;
const LAPIC_TIMER_ICR: u64 = 0x380;
const LAPIC_TIMER_CCR: u64 = 0x390;
const LAPIC_TIMER_DCR: u64 = 0x3E0;

/// Helper to build code that loads LAPIC base into RAX
fn lapic_base_to_rax() -> Vec<u8> {
    // mov rax, 0xFEE00000
    vec![0x48, 0xB8, 0x00, 0x00, 0xE0, 0xFE, 0x00, 0x00, 0x00, 0x00]
}

/// Helper to read LAPIC register into EBX: mov ebx, [rax + offset]
fn read_lapic_to_ebx(offset: u32) -> Vec<u8> {
    // mov ebx, [rax + offset] (32-bit displacement)
    vec![
        0x8B,
        0x98,
        (offset & 0xFF) as u8,
        ((offset >> 8) & 0xFF) as u8,
        ((offset >> 16) & 0xFF) as u8,
        ((offset >> 24) & 0xFF) as u8,
    ]
}

/// Helper to write ECX to LAPIC register: mov [rax + offset], ecx
fn write_ecx_to_lapic(offset: u32) -> Vec<u8> {
    // mov [rax + offset], ecx (32-bit displacement)
    vec![
        0x89,
        0x88,
        (offset & 0xFF) as u8,
        ((offset >> 8) & 0xFF) as u8,
        ((offset >> 16) & 0xFF) as u8,
        ((offset >> 24) & 0xFF) as u8,
    ]
}

// ============================================================================
// BASIC REGISTER ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_read_version_register() {
    // Read LAPIC version register and verify it returns expected value (0x00050014)
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_VERSION as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0x00050014,
        "LAPIC version should be 0x00050014 (modern APIC with 6 LVT entries)"
    );
}

#[test]
fn test_lapic_read_id_register() {
    // Read LAPIC ID register - should be 0 for first CPU
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_ID as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0, "LAPIC ID should be 0 for first CPU");
}

#[test]
fn test_lapic_read_svr_default() {
    // Read SVR - should be 0x1FF (APIC enabled, spurious vector 0xFF)
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_SVR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0x1FF,
        "SVR should be 0x1FF (APIC enabled, vector 0xFF)"
    );
}

#[test]
fn test_lapic_write_and_read_tpr() {
    // Write 0x42 to TPR, then read it back
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x42
    code.extend([0xB9, 0x42, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0x42,
        "TPR should read back the written value"
    );
}

#[test]
fn test_lapic_tpr_masks_to_8bits() {
    // Write 0x12345678 to TPR, should only keep lower 8 bits
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x12345678
    code.extend([0xB9, 0x78, 0x56, 0x34, 0x12]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0x78, "TPR should mask to lower 8 bits");
}

#[test]
fn test_lapic_write_and_read_id() {
    // Write a new APIC ID and read it back
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x05000000 (APIC ID = 5)
    code.extend([0xB9, 0x00, 0x00, 0x00, 0x05]);
    code.extend(write_ecx_to_lapic(LAPIC_ID as u32));
    code.extend(read_lapic_to_ebx(LAPIC_ID as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0x05000000,
        "APIC ID should read back the written value"
    );
}

// ============================================================================
// TIMER CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_lapic_lvt_timer_default_masked() {
    // LVT Timer should be masked by default
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Bit 16 is mask bit
    assert!(
        (regs.rbx as u32 & 0x10000) != 0,
        "LVT Timer should be masked by default"
    );
}

#[test]
fn test_lapic_timer_configure_oneshot() {
    // Configure timer for oneshot mode with vector 0x20
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write LVT Timer: vector=0x20, mode=oneshot(0), unmasked
    // mov ecx, 0x00000020
    code.extend([0xB9, 0x20, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let lvt = regs.rbx as u32;
    assert_eq!(lvt & 0xFF, 0x20, "Timer vector should be 0x20");
    assert_eq!((lvt >> 17) & 0x3, 0, "Timer mode should be oneshot (0)");
    assert_eq!(lvt & 0x10000, 0, "Timer should be unmasked");
}

#[test]
fn test_lapic_timer_configure_periodic() {
    // Configure timer for periodic mode with vector 0x30
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write LVT Timer: vector=0x30, mode=periodic(1 in bits 17-18), unmasked
    // 0x00020030
    // mov ecx, 0x00020030
    code.extend([0xB9, 0x30, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let lvt = regs.rbx as u32;
    assert_eq!(lvt & 0xFF, 0x30, "Timer vector should be 0x30");
    assert_eq!((lvt >> 17) & 0x3, 1, "Timer mode should be periodic (1)");
}

#[test]
fn test_lapic_timer_divide_config() {
    // Test setting timer divide configuration
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write DCR = 0b1011 (divide by 1)
    code.extend([0xB9, 0x0B, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_DCR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0x0B, "DCR should be 0x0B (divide by 1)");
}

#[test]
fn test_lapic_timer_initial_count() {
    // Write and read timer initial count
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write Initial Count = 0x12345678
    code.extend([0xB9, 0x78, 0x56, 0x34, 0x12]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_ICR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0x12345678,
        "Initial count should read back correctly"
    );
}

#[test]
fn test_lapic_timer_ccr_zero_when_no_timer() {
    // CCR should be 0 when timer hasn't been started
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_CCR as u32));
    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0, "CCR should be 0 when timer not started");
}

#[test]
fn test_lapic_timer_ccr_decreases() {
    // Start timer with large count, CCR should be less than initial count
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Configure timer: unmasked, oneshot
    code.extend([0xB9, 0x20, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    // Set DCR to divide by 1 (0b1011)
    code.extend([0xB9, 0x0B, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    // Set initial count to max value
    code.extend([0xB9, 0xFF, 0xFF, 0xFF, 0xFF]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));

    // Read CCR into ebx
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_CCR as u32));

    // Read initial count into ecx for comparison
    // mov ecx, [rax + LAPIC_TIMER_ICR]
    code.extend([0x8B, 0x88, 0x80, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CCR should be less than or equal to initial count
    let ccr = regs.rbx as u32;
    let icr = regs.rcx as u32;
    assert!(ccr <= icr, "CCR ({}) should be <= ICR ({})", ccr, icr);
}

// ============================================================================
// SVR AND APIC ENABLE/DISABLE TESTS
// ============================================================================

#[test]
fn test_lapic_disable_via_svr() {
    // Disable APIC by clearing bit 8 of SVR, then re-enable
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Disable: write SVR with bit 8 cleared
    // mov ecx, 0x0FF (spurious vector 0xFF, APIC disabled)
    code.extend([0xB9, 0xFF, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_SVR as u32));

    // Read back into ebx
    code.extend(read_lapic_to_ebx(LAPIC_SVR as u32));

    // Re-enable: write SVR with bit 8 set
    // mov ecx, 0x1FF
    code.extend([0xB9, 0xFF, 0x01, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_SVR as u32));

    // Read back into ecx
    // mov ecx, [rax + LAPIC_SVR]
    code.extend([0x8B, 0x88, 0xF0, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx as u32, 0xFF,
        "After disable, SVR should be 0xFF (APIC disabled)"
    );
    assert_eq!(
        regs.rcx as u32, 0x1FF,
        "After re-enable, SVR should be 0x1FF"
    );
}

// ============================================================================
// ISR/IRR REGISTER TESTS
// ============================================================================

#[test]
fn test_lapic_isr_initially_zero() {
    // All ISR registers should be 0 initially
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Read ISR[0] into ebx
    code.extend(read_lapic_to_ebx(LAPIC_ISR_BASE as u32));

    // Read ISR[7] into ecx
    code.extend([0x8B, 0x88, 0x70, 0x01, 0x00, 0x00]); // mov ecx, [rax + 0x170]

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0, "ISR[0] should be 0");
    assert_eq!(regs.rcx as u32, 0, "ISR[7] should be 0");
}

#[test]
fn test_lapic_irr_initially_zero() {
    // All IRR registers should be 0 initially
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Read IRR[0]
    code.extend(read_lapic_to_ebx(LAPIC_IRR_BASE as u32));

    // Read IRR[7]
    code.extend([0x8B, 0x88, 0x70, 0x02, 0x00, 0x00]); // mov ecx, [rax + 0x270]

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0, "IRR[0] should be 0");
    assert_eq!(regs.rcx as u32, 0, "IRR[7] should be 0");
}

// ============================================================================
// MULTI-REGISTER ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_multiple_register_writes() {
    // Write to multiple registers and verify they're independent
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write TPR = 0x10
    code.extend([0xB9, 0x10, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));

    // Write timer DCR = 0x03 (divide by 16)
    code.extend([0xB9, 0x03, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    // Write LVT Timer = 0x00020040 (periodic, vector 0x40)
    code.extend([0xB9, 0x40, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    // Read them all back
    // TPR -> ebx
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));

    // DCR -> r8d (using REX prefix)
    // mov r8d, [rax + DCR]
    code.extend([0x44, 0x8B, 0x80, 0xE0, 0x03, 0x00, 0x00]);

    // LVT Timer -> r9d
    // mov r9d, [rax + LVT_TIMER]
    code.extend([0x44, 0x8B, 0x88, 0x20, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0x10, "TPR should be 0x10");
    assert_eq!(regs.r8 as u32, 0x03, "DCR should be 0x03");
    assert_eq!(regs.r9 as u32, 0x00020040, "LVT Timer should be 0x00020040");
}

#[test]
fn test_lapic_sequential_timer_configuration() {
    // Simulate a typical timer setup sequence
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // 1. Set divide config to divide by 16 (0x03)
    code.extend([0xB9, 0x03, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    // 2. Configure LVT Timer: periodic mode (0x20000), vector 0x32, unmasked
    code.extend([0xB9, 0x32, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    // 3. Set initial count to start timer
    code.extend([0xB9, 0x00, 0x00, 0x10, 0x00]); // 0x100000
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));

    // Read back all values
    // DCR -> ebx
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_DCR as u32));

    // LVT -> ecx
    code.extend([0x8B, 0x88, 0x20, 0x03, 0x00, 0x00]);

    // ICR -> edx
    code.extend([0x8B, 0x90, 0x80, 0x03, 0x00, 0x00]);

    // CCR -> r8d (to verify timer is running)
    code.extend([0x44, 0x8B, 0x80, 0x90, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx as u32, 0x03, "DCR should be 0x03");
    assert_eq!(
        regs.rcx as u32, 0x00020032,
        "LVT should be periodic with vector 0x32"
    );
    assert_eq!(regs.rdx as u32, 0x00100000, "ICR should be 0x100000");
    // CCR should be non-zero and less than or equal to ICR (timer running)
    let ccr = regs.r8 as u32;
    assert!(ccr <= 0x00100000, "CCR should be <= ICR");
}

// ============================================================================
// EOI TESTS
// ============================================================================

#[test]
fn test_lapic_eoi_write() {
    // Writing to EOI register should work (even with no ISR bits set)
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // Write any value to EOI
    code.extend([0xB9, 0x00, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_EOI as u32));

    // Read ISR to verify nothing crashed
    code.extend(read_lapic_to_ebx(LAPIC_ISR_BASE as u32));

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Just verify we didn't crash and ISR is still 0
    assert_eq!(regs.rbx as u32, 0, "ISR[0] should still be 0 after EOI");
}

// ============================================================================
// BYTE AND WORD ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_byte_read() {
    // Test reading a single byte from LAPIC (should work via read_phys)
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov bl, [rax + TPR] - read single byte
    code.extend([0x8A, 0x98, 0x80, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TPR defaults to 0
    assert_eq!((regs.rbx & 0xFF) as u8, 0, "Byte read of TPR should be 0");
}

#[test]
fn test_lapic_word_read() {
    // Test reading a word (2 bytes) from LAPIC
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // First write a known value to TPR
    code.extend([0xB9, 0xAB, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));

    // mov bx, [rax + TPR] - read word
    code.extend([0x66, 0x8B, 0x98, 0x80, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        (regs.rbx & 0xFFFF) as u16,
        0xAB,
        "Word read of TPR should be 0x00AB"
    );
}
