//! Tests for AVX-512 Extended Instructions.
//!
//! This module covers advanced AVX-512 instructions including scatter/gather,
//! compress/expand, and conflict detection operations.
//!
//! Instructions covered:
//! - VSCATTER* - Scatter operations (various types)
//! - VPSCATTER* - Packed scatter operations
//! - VPCONFLICT* - Conflict detection
//! - VPCOMPRESS* - Compress operations
//! - VPEXPAND* - Expand operations
//! - VP2INTERSECT* - Intersection detection
//! - VPOPCNT* - Population count
//! - VRANGEPS/VRANGEPD - Range operations
//! - VREDUCE* - Reduce operations
//! - VFIXUPIMM* - Fixup operations
//!
//! References: AVX-512 instruction set documentation

use crate::common::*;
use rax::cpu::Registers;

// ============================================================================
// VPSCATTER Tests - Packed Scatter Operations
// ============================================================================

#[test]
fn test_vpscatterdd_basic() {
    // VPSCATTERDD - Scatter packed dwords with dword indices
    // Uses mask register and VSIB addressing
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x62, 0xF2, 0x7D, 0x48, 0xA0, 0x04, 0x08, // VPSCATTERDD [rax+ymm1*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpscatterdq_basic() {
    // VPSCATTERDQ - Scatter packed qwords with dword indices
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x62, 0xF2, 0xFD, 0x48, 0xA0, 0x04, 0x1B, // VPSCATTERDQ [rbx+ymm3*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpscatterqd_basic() {
    // VPSCATTERQD - Scatter packed dwords with qword indices
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x62, 0xF2, 0x7D, 0x48, 0xA1, 0x04, 0x09, // VPSCATTERQD [rcx+zmm1*1]{k0}, ymm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpscatterqq_basic() {
    // VPSCATTERQQ - Scatter packed qwords with qword indices
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0x62, 0xF2, 0xFD, 0x48, 0xA1, 0x04, 0x0A, // VPSCATTERQQ [rdx+zmm1*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VSCATTER Tests - Float Scatter Operations
// ============================================================================

#[test]
fn test_vscatterdps_basic() {
    // VSCATTERDPS - Scatter packed single-precision floats with dword indices
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x50, 0x00, 0x00, // MOV RAX, 0x5000
        0x62, 0xF2, 0x7D, 0x48, 0xA2, 0x04, 0x08, // VSCATTERDPS [rax+ymm1*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vscatterdpd_basic() {
    // VSCATTERDPD - Scatter packed double-precision floats with dword indices
    let code = [
        0x48, 0xC7, 0xC3, 0x00, 0x60, 0x00, 0x00, // MOV RBX, 0x6000
        0x62, 0xF2, 0xFD, 0x48, 0xA2, 0x04, 0x1B, // VSCATTERDPD [rbx+ymm3*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vscatterqps_basic() {
    // VSCATTERQPS - Scatter packed single-precision floats with qword indices
    let code = [
        0x48, 0xC7, 0xC1, 0x00, 0x70, 0x00, 0x00, // MOV RCX, 0x7000
        0x62, 0xF2, 0x7D, 0x48, 0xA3, 0x04, 0x09, // VSCATTERQPS [rcx+zmm1*1]{k0}, ymm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vscatterqpd_basic() {
    // VSCATTERQPD - Scatter packed double-precision floats with qword indices
    let code = [
        0x48, 0xC7, 0xC2, 0x00, 0x80, 0x00, 0x00, // MOV RDX, 0x8000
        0x62, 0xF2, 0xFD, 0x48, 0xA3, 0x04, 0x0A, // VSCATTERQPD [rdx+zmm1*1]{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPCONFLICT Tests - Conflict Detection
// ============================================================================

#[test]
fn test_vpconflictd_basic() {
    // VPCONFLICTD - Detect conflicts in dword elements
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0xC4, 0xC1, // VPCONFLICTD zmm0, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpconflictq_basic() {
    // VPCONFLICTQ - Detect conflicts in qword elements
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0xC4, 0xC2, // VPCONFLICTQ zmm0, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpconflictd_memory() {
    // VPCONFLICTD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x62, 0xF2, 0x7D, 0x48, 0xC4, 0x00, // VPCONFLICTD zmm0, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPCOMPRESS Tests - Compress Operations
// ============================================================================

#[test]
fn test_vpcompressd_basic() {
    // VPCOMPRESSD - Compress dword elements
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x8B, 0xC1, // VPCOMPRESSD zmm0{k0}, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpcompressq_basic() {
    // VPCOMPRESSQ - Compress qword elements
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x8B, 0xC2, // VPCOMPRESSQ zmm0{k0}, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpcompressd_to_memory() {
    // VPCOMPRESSD to memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x62, 0xF2, 0x7D, 0x48, 0x8B, 0x08, // VPCOMPRESSD [rax]{k0}, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcompressps_basic() {
    // VCOMPRESSPS - Compress single-precision floats
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x8A, 0xC1, // VCOMPRESSPS zmm0{k0}, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vcompresspd_basic() {
    // VCOMPRESSPD - Compress double-precision floats
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x8A, 0xC2, // VCOMPRESSPD zmm0{k0}, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPEXPAND Tests - Expand Operations
// ============================================================================

#[test]
fn test_vpexpandd_basic() {
    // VPEXPANDD - Expand dword elements
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x89, 0xC1, // VPEXPANDD zmm0{k0}, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpexpandq_basic() {
    // VPEXPANDQ - Expand qword elements
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x89, 0xC2, // VPEXPANDQ zmm0{k0}, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpexpandd_from_memory() {
    // VPEXPANDD from memory
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x62, 0xF2, 0x7D, 0x48, 0x89, 0x00, // VPEXPANDD zmm0{k0}, [rax]
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vexpandps_basic() {
    // VEXPANDPS - Expand single-precision floats
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x88, 0xC1, // VEXPANDPS zmm0{k0}, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vexpandpd_basic() {
    // VEXPANDPD - Expand double-precision floats
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x88, 0xC2, // VEXPANDPD zmm0{k0}, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VPOPCNT Tests - Population Count
// ============================================================================

#[test]
fn test_vpopcntd_basic() {
    // VPOPCNTD - Count set bits in dwords
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xC1, // VPOPCNTD zmm0, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vpopcntq_basic() {
    // VPOPCNTQ - Count set bits in qwords
    let code = [
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xC2, // VPOPCNTQ zmm0, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VRANGE Tests - Range Operations
// ============================================================================

#[test]
fn test_vrangeps_basic() {
    // VRANGEPS - Range operation on single-precision floats
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x50, 0xC1, 0x00, // VRANGEPS zmm0, zmm1, zmm2, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vrangepd_basic() {
    // VRANGEPD - Range operation on double-precision floats
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x50, 0xC2, 0x01, // VRANGEPD zmm0, zmm1, zmm2, 1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vrangess_basic() {
    // VRANGESS - Range operation on scalar single-precision
    let code = [
        0x62, 0xF3, 0x7D, 0x08, 0x51, 0xC1, 0x02, // VRANGESS xmm0, xmm1, xmm2, 2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vrangesd_basic() {
    // VRANGESD - Range operation on scalar double-precision
    let code = [
        0x62, 0xF3, 0xFD, 0x08, 0x51, 0xC2, 0x03, // VRANGESD xmm0, xmm1, xmm2, 3
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VREDUCE Tests - Reduce Operations
// ============================================================================

#[test]
fn test_vreduceps_basic() {
    // VREDUCEPS - Reduce single-precision floats
    let code = [
        0x62, 0xF3, 0x7D, 0x48, 0x56, 0xC1, 0x00, // VREDUCEPS zmm0, zmm1, 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vreducepd_basic() {
    // VREDUCEPD - Reduce double-precision floats
    let code = [
        0x62, 0xF3, 0xFD, 0x48, 0x56, 0xC2, 0x01, // VREDUCEPD zmm0, zmm2, 1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// VP2INTERSECT Tests - Intersection Detection
// ============================================================================

#[test]
fn test_vp2intersectd_basic() {
    // VP2INTERSECTD - Find intersecting dwords
    let code = [
        0x62, 0xF2, 0x7F, 0x48, 0x68, 0xC1, // VP2INTERSECTD k0, zmm0, zmm1
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_vp2intersectq_basic() {
    // VP2INTERSECTQ - Find intersecting qwords
    let code = [
        0x62, 0xF2, 0xFF, 0x48, 0x68, 0xC2, // VP2INTERSECTQ k0, zmm0, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined AVX-512 Extended Operations
// ============================================================================

#[test]
fn test_compress_expand_roundtrip() {
    // Compress then expand data
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x8B, 0xC1, // VPCOMPRESSD zmm0{k0}, zmm1
        0x62, 0xF2, 0x7D, 0x48, 0x89, 0xD0, // VPEXPANDD zmm2{k0}, zmm0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_scatter_gather_pattern() {
    // Scatter data then gather it back
    let code = [
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x62, 0xF2, 0x7D, 0x48, 0xA0, 0x04, 0x08, // VPSCATTERDD [rax+ymm1*1]{k0}, zmm0
        // Would need gather here in real code
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_conflict_detection_workflow() {
    // Detect conflicts in data
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0xC4, 0xC1, // VPCONFLICTD zmm0, zmm1
        0x62, 0xF2, 0xFD, 0x48, 0xC4, 0xD2, // VPCONFLICTQ zmm2, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_population_count_analysis() {
    // Count set bits in multiple registers
    let code = [
        0x62, 0xF2, 0x7D, 0x48, 0x55, 0xC1, // VPOPCNTD zmm0, zmm1
        0x62, 0xF2, 0xFD, 0x48, 0x55, 0xD2, // VPOPCNTQ zmm2, zmm2
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
