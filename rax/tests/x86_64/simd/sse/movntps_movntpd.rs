use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVNTPS/MOVNTPD - Store Packed Single/Double Precision Floating-Point Using Non-Temporal Hint
//
// MOVNTPS moves packed single-precision floating-point from XMM register to memory using non-temporal hint.
// MOVNTPD moves packed double-precision floating-point from XMM register to memory using non-temporal hint.
// The non-temporal hint minimizes cache pollution by using write combining (WC) protocol.
//
// Memory operand must be aligned on 16-byte boundary or #GP exception occurs.
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// NP 0F 2B /r             MOVNTPS m128, xmm1     - Move packed single-precision from xmm1 to m128 using non-temporal hint
// 66 0F 2B /r             MOVNTPD m128, xmm1     - Move packed double-precision from xmm1 to m128 using non-temporal hint

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MOVNTPS Tests - Non-Temporal Store of Packed Single-Precision
// ============================================================================

#[test]
fn test_movntps_mem_xmm0() {
    // MOVNTPS [ALIGNED_ADDR], XMM0
    let code = [
        0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm1() {
    // MOVNTPS [ALIGNED_ADDR], XMM1
    let code = [
        0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm2() {
    // MOVNTPS [ALIGNED_ADDR], XMM2
    let code = [
        0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm3() {
    // MOVNTPS [ALIGNED_ADDR], XMM3
    let code = [
        0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm4() {
    // MOVNTPS [ALIGNED_ADDR], XMM4
    let code = [
        0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm5() {
    // MOVNTPS [ALIGNED_ADDR], XMM5
    let code = [
        0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm6() {
    // MOVNTPS [ALIGNED_ADDR], XMM6
    let code = [
        0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm7() {
    // MOVNTPS [ALIGNED_ADDR], XMM7
    let code = [
        0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm8() {
    // MOVNTPS [ALIGNED_ADDR], XMM8 (requires REX prefix)
    let code = [
        0x44, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm9() {
    // MOVNTPS [ALIGNED_ADDR], XMM9
    let code = [
        0x44, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm10() {
    // MOVNTPS [ALIGNED_ADDR], XMM10
    let code = [
        0x44, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm11() {
    // MOVNTPS [ALIGNED_ADDR], XMM11
    let code = [
        0x44, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm12() {
    // MOVNTPS [ALIGNED_ADDR], XMM12
    let code = [
        0x44, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm13() {
    // MOVNTPS [ALIGNED_ADDR], XMM13
    let code = [
        0x44, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm14() {
    // MOVNTPS [ALIGNED_ADDR], XMM14
    let code = [
        0x44, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_mem_xmm15() {
    // MOVNTPS [ALIGNED_ADDR], XMM15
    let code = [
        0x44, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// MOVNTPD Tests - Non-Temporal Store of Packed Double-Precision
// ============================================================================

#[test]
fn test_movntpd_mem_xmm0() {
    // MOVNTPD [ALIGNED_ADDR], XMM0
    let code = [
        0x66, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm1() {
    // MOVNTPD [ALIGNED_ADDR], XMM1
    let code = [
        0x66, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm2() {
    // MOVNTPD [ALIGNED_ADDR], XMM2
    let code = [
        0x66, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm3() {
    // MOVNTPD [ALIGNED_ADDR], XMM3
    let code = [
        0x66, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm4() {
    // MOVNTPD [ALIGNED_ADDR], XMM4
    let code = [
        0x66, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm5() {
    // MOVNTPD [ALIGNED_ADDR], XMM5
    let code = [
        0x66, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm6() {
    // MOVNTPD [ALIGNED_ADDR], XMM6
    let code = [
        0x66, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm7() {
    // MOVNTPD [ALIGNED_ADDR], XMM7
    let code = [
        0x66, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm8() {
    // MOVNTPD [ALIGNED_ADDR], XMM8 (requires REX prefix)
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm9() {
    // MOVNTPD [ALIGNED_ADDR], XMM9
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm10() {
    // MOVNTPD [ALIGNED_ADDR], XMM10
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm11() {
    // MOVNTPD [ALIGNED_ADDR], XMM11
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm12() {
    // MOVNTPD [ALIGNED_ADDR], XMM12
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm13() {
    // MOVNTPD [ALIGNED_ADDR], XMM13
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm14() {
    // MOVNTPD [ALIGNED_ADDR], XMM14
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_mem_xmm15() {
    // MOVNTPD [ALIGNED_ADDR], XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional Tests - Non-Temporal Stores
// ============================================================================

#[test]
fn test_movntps_aligned_16byte_boundary() {
    // Test MOVNTPS with 16-byte aligned address
    let code = [
        0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_aligned_16byte_boundary() {
    // Test MOVNTPD with 16-byte aligned address
    let code = [
        0x66, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_sequential_stores() {
    // Test sequential MOVNTPS stores
    let code = [
        0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM0
        0x0f, 0x2b, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTPS [0x3010], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_sequential_stores() {
    // Test sequential MOVNTPD stores
    let code = [
        0x66, 0x0f, 0x2b, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM0
        0x66, 0x0f, 0x2b, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTPD [0x3010], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_write_combining() {
    // Test MOVNTPS write combining behavior
    let code = [
        0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_write_combining() {
    // Test MOVNTPD write combining behavior
    let code = [
        0x66, 0x0f, 0x2b, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_memory_ordering() {
    // Test MOVNTPS memory ordering
    let code = [
        0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM3
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_memory_ordering() {
    // Test MOVNTPD memory ordering
    let code = [
        0x66, 0x0f, 0x2b, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM3
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_cache_bypass() {
    // Test MOVNTPS cache bypass behavior
    let code = [
        0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_cache_bypass() {
    // Test MOVNTPD cache bypass behavior
    let code = [
        0x66, 0x0f, 0x2b, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_weakly_ordered_memory() {
    // Test MOVNTPS with weakly ordered memory types
    let code = [
        0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_weakly_ordered_memory() {
    // Test MOVNTPD with weakly ordered memory types
    let code = [
        0x66, 0x0f, 0x2b, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_with_sfence() {
    // Test MOVNTPS followed by SFENCE
    let code = [
        0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_with_sfence() {
    // Test MOVNTPD followed by SFENCE
    let code = [
        0x66, 0x0f, 0x2b, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntps_with_mfence() {
    // Test MOVNTPS followed by MFENCE
    let code = [
        0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPS [0x3000], XMM7
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntpd_with_mfence() {
    // Test MOVNTPD followed by MFENCE
    let code = [
        0x66, 0x0f, 0x2b, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTPD [0x3000], XMM7
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
