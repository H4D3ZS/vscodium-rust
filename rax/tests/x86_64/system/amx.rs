//! Tests for Intel AMX (Advanced Matrix Extensions) Instructions.
//!
//! This module covers AMX tile instructions for matrix operations.
//!
//! Instructions covered:
//! - TILELOADD/TILELOADDT1 - Load tile from memory
//! - TILESTORED - Store tile to memory
//! - TILERELEASE - Release tile resources
//! - TILEZERO - Zero a tile
//! - LDTILECFG - Load tile configuration
//! - STTILECFG - Store tile configuration
//! - TDPBF16PS - Dot product of BF16 pairs accumulated into FP32
//! - TDPBSSD - Dot product of signed bytes to signed dwords
//! - TDPBSUD - Dot product of signed/unsigned bytes to signed dwords
//! - TDPBUSD - Dot product of unsigned/signed bytes to signed dwords
//! - TDPBUUD - Dot product of unsigned bytes to signed dwords
//!
//! References: docs/tileloadd:tileloaddt1.txt, docs/tilestored.txt,
//!            docs/tilerelease.txt, docs/tilezero.txt, docs/ldtilecfg.txt,
//!            docs/sttilecfg.txt, docs/tdpbf16ps.txt,
//!            docs/tdpbssd:tdpbsud:tdpbusd:tdpbuud.txt

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// LDTILECFG/STTILECFG Tests - Tile Configuration
// ============================================================================

#[test]
fn test_ldtilecfg_basic() {
    // LDTILECFG - Load tile configuration from memory
    // Opcode: VEX.128.66.0F38.W0 49
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xC4, 0xE2, 0x78, 0x49, 0x00, // LDTILECFG [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_sttilecfg_basic() {
    // STTILECFG - Store tile configuration to memory
    // Opcode: VEX.128.66.0F38.W0 49 /r (reg=1)
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xC4, 0xE2, 0x78, 0x49, 0x0B, // STTILECFG [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ldtilecfg_different_addresses() {
    // Load tile config from various addresses
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0xC4, 0xE2, 0x78, 0x49, 0x01, // LDTILECFG [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ldtilecfg_with_displacement() {
    // LDTILECFG with displacement
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0xC4, 0xE2, 0x78, 0x49, 0x82, 0x00, 0x04, 0x00, 0x00, // LDTILECFG [rdx+0x400]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tile_config_save_restore() {
    // Save and restore tile configuration
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xC4, 0xE2, 0x78, 0x49, 0x00, // LDTILECFG [rax]
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xC4, 0xE2, 0x78, 0x49, 0x0B, // STTILECFG [rbx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TILELOADD/TILELOADDT1 Tests - Load Tile
// ============================================================================

#[test]
fn test_tileloadd_basic() {
    // TILELOADD - Load tile from memory (dense format)
    // Opcode: VEX.128.F2.0F38.W0 4B
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tileloaddt1_basic() {
    // TILELOADDT1 - Load tile from memory (T1 format)
    // Opcode: VEX.128.66.0F38.W0 4B
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x50, 0x00, 0x00, // MOV RCX, 0x5000
        0xC4, 0xE2, 0x79, 0x4B, 0x09, // TILELOADDT1 tmm1, [rcx]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tileloadd_different_tiles() {
    // Load different tiles
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC4, 0xE2, 0x7A, 0x4B, 0x08, // TILELOADD tmm1, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tileloadd_with_displacement() {
    // TILELOADD with displacement
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x82, 0x00, 0x10, 0x00, 0x00, // TILELOADD tmm0, [rdx+0x1000]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tileloadd_all_tiles() {
    // Load all available tiles (tmm0-tmm7)
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x08, // TILELOADD tmm1, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x10, // TILELOADD tmm2, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x18, // TILELOADD tmm3, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x20, // TILELOADD tmm4, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x28, // TILELOADD tmm5, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x30, // TILELOADD tmm6, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x38, // TILELOADD tmm7, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TILESTORED Tests - Store Tile
// ============================================================================

#[test]
fn test_tilestored_basic() {
    // TILESTORED - Store tile to memory
    // Opcode: VEX.128.F3.0F38.W0 4B
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x00, // TILESTORED [rax], tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilestored_different_tiles() {
    // Store different tiles
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x00, // TILESTORED [rax], tmm0
        0x48, 0xC7, 0xC0, 0x00, 0x70, 0x00, 0x00, // MOV RAX, 0x7000
        0xC4, 0xE2, 0x7B, 0x4B, 0x08, // TILESTORED [rax], tmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilestored_with_displacement() {
    // TILESTORED with displacement
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x60, 0x00, 0x00, // MOV RBX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x83, 0x00, 0x10, 0x00, 0x00, // TILESTORED [rbx+0x1000], tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tile_load_store_roundtrip() {
    // Load and store tile
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x00, // TILESTORED [rax], tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TILEZERO Tests - Zero Tile
// ============================================================================

#[test]
fn test_tilezero_basic() {
    // TILEZERO - Zero a tile
    // Opcode: VEX.128.F2.0F38.W0 49
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilezero_different_tiles() {
    // Zero different tiles
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x7A, 0x49, 0xC8, // TILEZERO tmm1
        0xC4, 0xE2, 0x7A, 0x49, 0xD0, // TILEZERO tmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilezero_all_tiles() {
    // Zero all tiles
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x7A, 0x49, 0xC8, // TILEZERO tmm1
        0xC4, 0xE2, 0x7A, 0x49, 0xD0, // TILEZERO tmm2
        0xC4, 0xE2, 0x7A, 0x49, 0xD8, // TILEZERO tmm3
        0xC4, 0xE2, 0x7A, 0x49, 0xE0, // TILEZERO tmm4
        0xC4, 0xE2, 0x7A, 0x49, 0xE8, // TILEZERO tmm5
        0xC4, 0xE2, 0x7A, 0x49, 0xF0, // TILEZERO tmm6
        0xC4, 0xE2, 0x7A, 0x49, 0xF8, // TILEZERO tmm7
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilezero_after_load() {
    // Load then zero a tile
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TILERELEASE Tests - Release Tile Resources
// ============================================================================

#[test]
fn test_tilerelease_basic() {
    // TILERELEASE - Release all tile resources
    // Opcode: VEX.128.66.0F38.W0 49 /r (reg=0, rm=0xC0)
    let code = [
        0xC4, 0xE2, 0x78, 0x49, 0xC0, // TILERELEASE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilerelease_after_operations() {
    // Release tiles after operations
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x00, // TILELOADD tmm0, [rax]
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x78, 0x49, 0xC0, // TILERELEASE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilerelease_no_operands() {
    // TILERELEASE takes no operands
    let code = [0xC4, 0xE2, 0x78, 0x49, 0xC0, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tilerelease_multiple() {
    // Multiple TILERELEASE operations (idempotent)
    let code = [
        0xC4, 0xE2, 0x78, 0x49, 0xC0, // TILERELEASE
        0xC4, 0xE2, 0x78, 0x49, 0xC0, // TILERELEASE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TDPBF16PS Tests - BF16 Dot Product
// ============================================================================

#[test]
fn test_tdpbf16ps_basic() {
    // TDPBF16PS - Dot product of BF16 pairs to FP32
    // Opcode: VEX.128.F3.0F38.W0 5C
    let code = [
        0xC4, 0xE2, 0x7B, 0x5C, 0xC8, // TDPBF16PS tmm0, tmm1, tmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tdpbf16ps_different_tiles() {
    // Use different tile combinations
    let code = [
        0xC4, 0xE2, 0x7B, 0x5C, 0xC8, // TDPBF16PS tmm0, tmm1, tmm2
        0xC4, 0xE2, 0x7B, 0x5C, 0xE0, // TDPBF16PS tmm3, tmm4, tmm5
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tdpbf16ps_with_zero() {
    // Dot product with zeroed accumulator
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x7B, 0x5C, 0xC8, // TDPBF16PS tmm0, tmm1, tmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TDPBSSD/TDPBSUD/TDPBUSD/TDPBUUD Tests - Integer Dot Products
// ============================================================================

#[test]
fn test_tdpbssd_basic() {
    // TDPBSSD - Dot product signed bytes to signed dwords
    // Opcode: VEX.128.F2.0F38.W0 5E
    let code = [
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tdpbsud_basic() {
    // TDPBSUD - Dot product signed/unsigned bytes to signed dwords
    // Opcode: VEX.128.F3.0F38.W0 5E
    let code = [
        0xC4, 0xE2, 0x7B, 0x5E, 0xD0, // TDPBSUD tmm1, tmm2, tmm3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tdpbusd_basic() {
    // TDPBUSD - Dot product unsigned/signed bytes to signed dwords
    // Opcode: VEX.128.66.0F38.W0 5E
    let code = [
        0xC4, 0xE2, 0x79, 0x5E, 0xE0, // TDPBUSD tmm2, tmm3, tmm4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_tdpbuud_basic() {
    // TDPBUUD - Dot product unsigned bytes to signed dwords
    // Opcode: VEX.128.0F38.W0 5E
    let code = [
        0xC4, 0xE2, 0x7A, 0x5E, 0xF0, // TDPBUUD tmm3, tmm4, tmm5
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_integer_dotprod_sequence() {
    // Sequence of different integer dot products
    let code = [
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        0xC4, 0xE2, 0x7B, 0x5E, 0xD0, // TDPBSUD tmm1, tmm2, tmm3
        0xC4, 0xE2, 0x79, 0x5E, 0xE0, // TDPBUSD tmm2, tmm3, tmm4
        0xC4, 0xE2, 0x7A, 0x5E, 0xF0, // TDPBUUD tmm3, tmm4, tmm5
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_dotprod_with_accumulation() {
    // Multiple dot products accumulating into same tile
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        0xC4, 0xE2, 0x7A, 0x5E, 0xD8, // TDPBSSD tmm0, tmm3, tmm4
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined AMX Operation Tests
// ============================================================================

#[test]
fn test_amx_complete_workflow() {
    // Complete AMX workflow: config, load, compute, store, release
    let code = [
        // Load configuration
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xC4, 0xE2, 0x78, 0x49, 0x00, // LDTILECFG [rax]
        // Load tiles
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x08, // TILELOADD tmm1, [rax]
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC4, 0xE2, 0x7A, 0x4B, 0x10, // TILELOADD tmm2, [rax]
        // Zero accumulator
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        // Compute dot product
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        // Store result
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x00, // TILESTORED [rax], tmm0
        // Release tiles
        0xC4, 0xE2, 0x78, 0x49, 0xC0, // TILERELEASE
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_amx_matrix_multiply_accumulate() {
    // Matrix multiply-accumulate pattern
    let code = [
        // Load matrices
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0xC4, 0xE2, 0x7A, 0x4B, 0x08, // TILELOADD tmm1, [rax] (A)
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0xC4, 0xE2, 0x7A, 0x4B, 0x10, // TILELOADD tmm2, [rax] (B)
        // Initialize accumulator
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        // C += A * B
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        // Store result
        0x48, 0xC7, 0xC0, 0x00, 0x60, 0x00, 0x00, // MOV RAX, 0x6000
        0xC4, 0xE2, 0x7B, 0x4B, 0x00, // TILESTORED [rax], tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_amx_multiple_accumulations() {
    // Multiple accumulations into same result
    let code = [
        // Initialize
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0x48, 0xC7, 0xC0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        // First product
        0xC4, 0xE2, 0x7A, 0x4B, 0x08, // TILELOADD tmm1, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x10, // TILELOADD tmm2, [rax]
        0xC4, 0xE2, 0x7A, 0x5E, 0xC8, // TDPBSSD tmm0, tmm1, tmm2
        // Second product
        0xC4, 0xE2, 0x7A, 0x4B, 0x18, // TILELOADD tmm3, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x20, // TILELOADD tmm4, [rax]
        0xC4, 0xE2, 0x7A, 0x5E, 0xD8, // TDPBSSD tmm0, tmm3, tmm4
        // Third product
        0xC4, 0xE2, 0x7A, 0x4B, 0x28, // TILELOADD tmm5, [rax]
        0xC4, 0xE2, 0x7A, 0x4B, 0x30, // TILELOADD tmm6, [rax]
        0xC4, 0xE2, 0x7A, 0x5E, 0xE8, // TDPBSSD tmm0, tmm5, tmm6
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_amx_config_reload() {
    // Reconfigure tiles mid-computation
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0xC4, 0xE2, 0x78, 0x49, 0x00, // LDTILECFG [rax]
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        // Reconfigure
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0xC4, 0xE2, 0x78, 0x49, 0x00, // LDTILECFG [rax]
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_amx_all_dotprod_types() {
    // Use all dot product instruction types
    let code = [
        0xC4, 0xE2, 0x7A, 0x49, 0xC0, // TILEZERO tmm0
        0xC4, 0xE2, 0x7B, 0x5C, 0xC8, // TDPBF16PS tmm0, tmm1, tmm2
        0xC4, 0xE2, 0x7A, 0x5E, 0xD0, // TDPBSSD tmm1, tmm2, tmm3
        0xC4, 0xE2, 0x7B, 0x5E, 0xE0, // TDPBSUD tmm2, tmm3, tmm4
        0xC4, 0xE2, 0x79, 0x5E, 0xF0, // TDPBUSD tmm3, tmm4, tmm5
        0xC4, 0xE2, 0x7A, 0x5E, 0xC0, // TDPBUUD tmm4, tmm5, tmm6
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
