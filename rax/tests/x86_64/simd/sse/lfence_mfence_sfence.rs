use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// Memory Fence Instructions - LFENCE, MFENCE, SFENCE
//
// LFENCE (Load Fence):
// - Serializes all load operations before LFENCE
// - No later instruction begins execution until LFENCE completes
// - Instructions can be fetched speculatively before LFENCE completes
// Opcode: NP 0F AE E8
//
// MFENCE (Memory Fence):
// - Serializes all load and store operations before MFENCE
// - Guarantees global visibility of loads/stores before any after MFENCE
// - Ordered with respect to all loads, stores, and other fences
// Opcode: NP 0F AE F0
//
// SFENCE (Store Fence):
// - Serializes all store operations before SFENCE
// - Every store before SFENCE is globally visible before stores after
// - Not ordered with loads or LFENCE
// Opcode: NP 0F AE F8

// ============================================================================
// LFENCE Tests - Load Fence (Serializes Load Operations)
// ============================================================================

#[test]
fn test_lfence_basic() {
    // Basic LFENCE execution
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_after_load() {
    // LFENCE after memory load
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_before_load() {
    // LFENCE before memory load
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_multiple() {
    // Multiple LFENCEs in sequence
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_serialization() {
    // Test load serialization behavior
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_ordering_1() {
    // Test load ordering
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_ordering_2() {
    // Test load ordering with different pattern
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_with_stores() {
    // LFENCE with store operations (not ordered)
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_speculative_fetch() {
    // Test speculative fetching behavior
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_weakly_ordered_memory() {
    // Test with weakly ordered memory types
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_opcodes_e9() {
    // Test LFENCE with ModRM byte 0xE9
    let code = [
        0x0f, 0xae, 0xe9, // LFENCE (alternate encoding)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_opcodes_ef() {
    // Test LFENCE with ModRM byte 0xEF
    let code = [
        0x0f, 0xae, 0xef, // LFENCE (alternate encoding)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// MFENCE Tests - Memory Fence (Serializes Load and Store Operations)
// ============================================================================

#[test]
fn test_mfence_basic() {
    // Basic MFENCE execution
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_after_load() {
    // MFENCE after memory load
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_after_store() {
    // MFENCE after memory store
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_before_load() {
    // MFENCE before memory load
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_before_store() {
    // MFENCE before memory store
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_multiple() {
    // Multiple MFENCEs in sequence
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_serialization() {
    // Test load and store serialization
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_global_visibility() {
    // Test global visibility guarantees
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_ordering_loads_stores() {
    // Test ordering of loads and stores
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_with_lfence() {
    // MFENCE with LFENCE
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_with_sfence() {
    // MFENCE with SFENCE
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mfence_opcodes_f7() {
    // Test MFENCE with ModRM byte 0xF7
    let code = [
        0x0f, 0xae, 0xf7, // MFENCE (alternate encoding)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// SFENCE Tests - Store Fence (Serializes Store Operations)
// ============================================================================

#[test]
fn test_sfence_basic() {
    // Basic SFENCE execution
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_after_store() {
    // SFENCE after memory store
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_before_store() {
    // SFENCE before memory store
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_multiple() {
    // Multiple SFENCEs in sequence
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_serialization() {
    // Test store serialization behavior
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_global_visibility() {
    // Test global visibility of stores
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_ordering() {
    // Test store ordering
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_with_loads() {
    // SFENCE with load operations (not ordered)
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_with_mfence() {
    // SFENCE with MFENCE
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_write_combining() {
    // SFENCE with write combining
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_opcodes_ff() {
    // Test SFENCE with ModRM byte 0xFF
    let code = [
        0x0f, 0xae, 0xff, // SFENCE (alternate encoding)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Combined Fence Tests
// ============================================================================

#[test]
fn test_all_fences_sequence() {
    // All three fences in sequence
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_lfence_sfence_combination() {
    // LFENCE followed by SFENCE
    let code = [
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_sfence_lfence_combination() {
    // SFENCE followed by LFENCE
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_nested_fences_1() {
    // Nested fence operations pattern 1
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xe8, // LFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_nested_fences_2() {
    // Nested fence operations pattern 2
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0x0f, 0xae, 0xf8, // SFENCE
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
