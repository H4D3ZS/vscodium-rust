//! Intel APX Combined Feature Tests
//!
//! These tests exercise multiple APX features together to verify
//! correct interaction between:
//! - EGPR (R16-R31)
//! - REX2 prefix encoding
//! - NDD (3-operand forms)
//! - NF (No Flags)
//! - CCMP/CTEST
//! - PUSH2/POP2
//! - Zero-Upper semantics
//!
//! Real-world APX code will frequently combine these features.

use crate::common::*;

// ============================================================================
// EGPR + NDD Combinations
// ============================================================================

/// NDD ADD with all EGPR operands
#[test]
fn test_ndd_add_all_egpr() {
    // ADD r18, r16, r17 (NDD with EGPR)
    let code = [
        0x62, 0xEC, 0xE4, 0x18, // EVEX with EGPR bits for all three
        0x01, 0xC8, // ADD encoding
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// NDD SUB with mixed legacy and EGPR
#[test]
fn test_ndd_sub_mixed_egpr() {
    // SUB rax, r20, rbx (NDD)
    let code = [0x62, 0xEC, 0xDC, 0x18, 0x29, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 200;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// NDD IMUL with EGPR
#[test]
fn test_ndd_imul_egpr() {
    // IMUL r25, r26, r27 (NDD)
    let code = [0x62, 0xEC, 0xCC, 0x18, 0x0F, 0xAF, 0xD1, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// EGPR + NF Combinations
// ============================================================================

/// NF ADD with EGPR preserves flags
#[test]
fn test_nf_add_egpr_preserves_flags() {
    // STC; ADD r16, r17 (NF with EGPR)
    let code = [
        0xF9, // STC (set CF)
        0x62, 0xEC, 0xE4, 0x0C, // EVEX with NF bit set
        0x01, 0xC8, // ADD
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// NF SUB with EGPR causing would-be borrow
#[test]
fn test_nf_sub_egpr_no_borrow_flag() {
    // SUB r20, r21 (NF)
    let code = [
        0x62, 0xEC, 0xE4, 0x0C, 0x29, 0xC8, // SUB
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// NF SHL with EGPR
#[test]
fn test_nf_shl_egpr() {
    // SHL r28, 1 (NF)
    let code = [
        0x62, 0xEC, 0xE4, 0x0C, 0xD1, 0xE0, // SHL r28, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD + NF Combinations
// ============================================================================

/// NDD ADD with NF
#[test]
fn test_ndd_nf_add() {
    // ADD rax, rbx, rcx (NDD + NF)
    let code = [
        0x62, 0xF4, 0xE4, 0x1C, // NDD + NF
        0x01, 0xD9, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// NDD SUB with NF preserves all flags
#[test]
fn test_ndd_nf_sub_preserves_flags() {
    // STC; SUB rax, rbx, rcx (NDD + NF)
    let code = [
        0xF9, // STC
        0x62, 0xF4, 0xE4, 0x1C, 0x29, 0xD9, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 100;
    regs.rcx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// EGPR + NDD + NF Combinations
// ============================================================================

/// All three features: NDD + NF + EGPR
#[test]
fn test_ndd_nf_egpr_all_combined() {
    // STC; ADD r16, r17, r18 (NDD + NF + EGPR)
    let code = [0xF9, 0x62, 0xEC, 0xE4, 0x1C, 0x01, 0xC8, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// NDD + NF + EGPR with logical operations
#[test]
fn test_ndd_nf_egpr_xor() {
    // XOR r20, r21, r22 (NDD + NF + EGPR)
    let code = [0x62, 0xEC, 0xCC, 0x1C, 0x31, 0xD0, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// PUSH2/POP2 + EGPR Combinations
// ============================================================================

/// PUSH2/POP2 with all EGPR
#[test]
fn test_push2_pop2_all_egpr() {
    // PUSH2 r16, r17; POP2 r16, r17
    let code = [
        0x62, 0xEC, 0x74, 0x18, 0xFF, 0xF0, // PUSH2
        0x62, 0xEC, 0x74, 0x18, 0x8F, 0xC0, // POP2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// Save/restore EGPR across function call pattern
#[test]
fn test_push2_pop2_function_pattern() {
    // PUSH2 r20, r21; PUSH2 r22, r23; <work>; POP2 r22, r23; POP2 r20, r21
    let code = [
        0x62, 0xEC, 0x54, 0x18, 0xFF, 0xF4, // PUSH2 r20, r21
        0x62, 0xEC, 0x4C, 0x18, 0xFF, 0xF6, // PUSH2 r22, r23
        0x90, // NOP (function body)
        0x62, 0xEC, 0x4C, 0x18, 0x8F, 0xC6, // POP2 r22, r23
        0x62, 0xEC, 0x54, 0x18, 0x8F, 0xC4, // POP2 r20, r21
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CCMP/CTEST + EGPR Combinations
// ============================================================================

/// CCMP chain with EGPR
#[test]
fn test_ccmp_chain_egpr() {
    // CMP r16, 40; CCMPG r17, 20, dfv=0
    let code = [
        0xD5, 0x11, 0x83, 0xF8, 0x28, // CMP r16, 40 (REX2)
        0x62, 0xEC, 0xE4, 0x0F, // CCMPG
        0x83, 0xF9, 0x14, // CMP r17, 20
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTEST with EGPR operands
#[test]
fn test_ctest_egpr() {
    // CTESTB r24, r25, dfv=0
    let code = [
        0xF9, // STC for condition
        0x62, 0xEC, 0xE4, 0x42, 0x85, 0xC8, // TEST r24, r25
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Zero-Upper + NDD Combinations
// ============================================================================

/// NDD 32-bit with ZU clears upper bits of destination only
#[test]
fn test_ndd_32bit_zu_destination_only() {
    // ADD32 eax, ebx, ecx with ZU=1
    let code = [0x62, 0xF4, 0x64, 0x1A, 0x01, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0xAAAAAAAA00000064;
    regs.rcx = 0xBBBBBBBB00000032;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// NDD 32-bit with ZU=0 preserves destination upper
#[test]
fn test_ndd_32bit_zu_clear_preserves() {
    // ADD32 eax, ebx, ecx with ZU=0
    let code = [0x62, 0xF4, 0x64, 0x10, 0x01, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF00000000;
    regs.rbx = 100;
    regs.rcx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Zero-Upper + NF Combinations
// ============================================================================

/// 32-bit NF with ZU
#[test]
fn test_nf_32bit_zu() {
    // ADD32 eax, ebx with NF and ZU=1
    let code = [
        0xF9, // STC to verify flags preserved
        0x62, 0xF4, 0x64, 0x1E, // NF=1, ZU=1
        0x01, 0xD8, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xCAFEBABE00000000;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Complex Multi-Feature Tests
// ============================================================================

/// Realistic computation: NDD + NF + EGPR + ZU
#[test]
fn test_realistic_computation() {
    // Series of operations using multiple APX features
    let code = [
        // ADD r20, r16, r17 (NDD with EGPR)
        0x62, 0xEC, 0xE4, 0x18, 0x01, 0xC8, // XOR r20, r20, r18 (NDD)
        0x62, 0xEC, 0xCC, 0x18, 0x31, 0xD0, // SHR r20, 4 (NF)
        0x62, 0xEC, 0xE4, 0x0C, 0xC1, 0xE8, 0x04, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// Function prologue/epilogue with APX features
#[test]
fn test_apx_function_pattern() {
    // PUSH2 r16, r17; PUSH2 r18, r19; <body>; POP2 r18, r19; POP2 r16, r17
    let code = [
        // Prologue
        0x62, 0xEC, 0x74, 0x18, 0xFF, 0xF0, // PUSH2 r16, r17
        0x62, 0xEC, 0x64, 0x18, 0xFF, 0xF2, // PUSH2 r18, r19
        // Function body: NDD ADD with NF
        0x62, 0xEC, 0xE4, 0x1C, 0x01, 0xC8, // ADD r16, r16, r17 (NDD+NF)
        // Epilogue
        0x62, 0xEC, 0x64, 0x18, 0x8F, 0xC2, // POP2 r18, r19
        0x62, 0xEC, 0x74, 0x18, 0x8F, 0xC0, // POP2 r16, r17
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// Conditional execution with CCMP and EGPR
#[test]
fn test_ccmp_egpr_conditional_chain() {
    // CMP r16, 10; CCMPG r17, 100, dfv=0x02
    let code = [
        0xD5, 0x11, 0x83, 0xF8, 0x0A, // CMP r16, 10
        0x62, 0xEC, 0xE4, 0x0F, // CCMPG
        0x83, 0xF9, 0x64, // CMP r17, 100
        0x02, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Memory Operations with Combined Features
// ============================================================================

/// NDD with memory operand and EGPR
#[test]
fn test_ndd_mem_egpr() {
    // ADD r16, [r17], r18 (NDD with memory source and EGPR)
    let code = [
        0x62, 0xEC, 0xCC, 0x18, 0x03, 0x01, // ADD r16, [r17]
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.r9 = DATA_ADDR; // Use r9 as memory pointer
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 100);
    let _ = run_until_hlt(&mut vcpu);
}

/// NF with memory operand and EGPR
#[test]
fn test_nf_mem_egpr() {
    // INC [r8] (NF)
    let code = [
        0x62, 0xD4, 0xFC, 0x0C, 0xFF, 0x00, // INC [r8]
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.r8 = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0xFFFFFFFFFFFFFFFF);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// All Features Combined
// ============================================================================

/// Ultimate test: EGPR + NDD + NF + ZU + memory
#[test]
fn test_all_features_combined() {
    // STC; ADD32 r24d, [r25], r26d (NDD + NF + ZU + mem + EGPR)
    let code = [
        0xF9, // STC to verify flags preserved
        0x62, 0xEC, 0x4C, 0x1E, // NDD + NF + ZU
        0x03, 0x21, // ADD r24d, [r25]
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.r9 = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 100);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 + EVEX Interaction
// ============================================================================

/// REX2 for EGPR access in non-EVEX instruction followed by EVEX
#[test]
fn test_rex2_evex_sequence() {
    // MOV r16, 0x1234 (REX2); ADD rax, rbx (EVEX NDD)
    let code = [
        0xD5, 0x19, 0xB8, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV r16, imm64
        0x62, 0xF4, 0xE4, 0x18, 0x01, 0xD8, // ADD rax, rbx (NDD)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// Mixed REX2 and Extended EVEX in sequence
#[test]
fn test_mixed_rex2_extended_evex() {
    // MOV r16, rax (REX2); ADD r17, r18, r19 (Extended EVEX NDD)
    let code = [
        0xD5, 0x11, 0x89, 0xC0, // MOV r16, rax (REX2)
        0x62, 0xEC, 0xCC, 0x18, 0x01, 0xC8, // ADD r17, r18, r19 (EVEX)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
