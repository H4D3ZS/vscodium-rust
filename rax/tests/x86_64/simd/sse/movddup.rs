use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVDDUP - Replicate Double Precision Floating-Point Values
//
// For 128-bit: Duplicates the low double precision FP value to high qword
// Result pattern: low qword duplicated to high qword
//
// Opcode:
// F2 0F 12 /r             MOVDDUP xmm1, xmm2/m64    - Duplicate low DP FP value

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// MOVDDUP Tests - Duplicate Low Double Precision Value
// ============================================================================

#[test]
fn test_movddup_xmm0_xmm1() {
    // MOVDDUP XMM0, XMM1
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm1_xmm2() {
    // MOVDDUP XMM1, XMM2
    let code = [
        0xf2, 0x0f, 0x12, 0xca, // MOVDDUP XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm2_xmm3() {
    // MOVDDUP XMM2, XMM3
    let code = [
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm3_xmm4() {
    // MOVDDUP XMM3, XMM4
    let code = [
        0xf2, 0x0f, 0x12, 0xdc, // MOVDDUP XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm4_xmm5() {
    // MOVDDUP XMM4, XMM5
    let code = [
        0xf2, 0x0f, 0x12, 0xe5, // MOVDDUP XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm5_xmm6() {
    // MOVDDUP XMM5, XMM6
    let code = [
        0xf2, 0x0f, 0x12, 0xee, // MOVDDUP XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm6_xmm7() {
    // MOVDDUP XMM6, XMM7
    let code = [
        0xf2, 0x0f, 0x12, 0xf7, // MOVDDUP XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm7_xmm0() {
    // MOVDDUP XMM7, XMM0
    let code = [
        0xf2, 0x0f, 0x12, 0xf8, // MOVDDUP XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm8_xmm9() {
    // MOVDDUP XMM8, XMM9 (requires REX prefix)
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xc1, // MOVDDUP XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm9_xmm10() {
    // MOVDDUP XMM9, XMM10
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xca, // MOVDDUP XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm10_xmm11() {
    // MOVDDUP XMM10, XMM11
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xd3, // MOVDDUP XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm11_xmm12() {
    // MOVDDUP XMM11, XMM12
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xdc, // MOVDDUP XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm12_xmm13() {
    // MOVDDUP XMM12, XMM13
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xe5, // MOVDDUP XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm13_xmm14() {
    // MOVDDUP XMM13, XMM14
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xee, // MOVDDUP XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm14_xmm15() {
    // MOVDDUP XMM14, XMM15
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xf7, // MOVDDUP XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm15_xmm0() {
    // MOVDDUP XMM15, XMM0
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xf8, // MOVDDUP XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm0_mem() {
    // MOVDDUP XMM0, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm1_mem() {
    // MOVDDUP XMM1, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x12, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm7_mem() {
    // MOVDDUP XMM7, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_xmm15_mem() {
    // MOVDDUP XMM15, [ALIGNED_ADDR]
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_movddup_duplicate_pattern() {
    // Test MOVDDUP duplication pattern
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_self() {
    // Test MOVDDUP with same register
    let code = [
        0xf2, 0x0f, 0x12, 0xc0, // MOVDDUP XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_chain() {
    // Test chaining MOVDDUP operations
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xd0, // MOVDDUP XMM2, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_multiple_registers() {
    // Test MOVDDUP on multiple registers
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xe5, // MOVDDUP XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_high_registers() {
    // Test MOVDDUP with high XMM registers
    let code = [
        0xf2, 0x45, 0x0f, 0x12, 0xc1, // MOVDDUP XMM8, XMM9
        0xf2, 0x45, 0x0f, 0x12, 0xd3, // MOVDDUP XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_mixed_low_high() {
    // Test MOVDDUP with mixed low and high registers
    let code = [
        0xf2, 0x44, 0x0f, 0x12, 0xc1, // MOVDDUP XMM8, XMM1
        0xf2, 0x41, 0x0f, 0x12, 0xc8, // MOVDDUP XMM1, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_mem_various_regs() {
    // Test MOVDDUP from memory to various registers
    let code = [
        0xf2, 0x0f, 0x12, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM0, [0x3000]
        0xf2, 0x0f, 0x12, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVDDUP XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_sequential() {
    // Test sequential MOVDDUP operations
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xca, // MOVDDUP XMM1, XMM2
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xdc, // MOVDDUP XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_all_low_regs() {
    // Test MOVDDUP on all low XMM registers
    let code = [
        0xf2, 0x0f, 0x12, 0xc1, // MOVDDUP XMM0, XMM1
        0xf2, 0x0f, 0x12, 0xd3, // MOVDDUP XMM2, XMM3
        0xf2, 0x0f, 0x12, 0xe5, // MOVDDUP XMM4, XMM5
        0xf2, 0x0f, 0x12, 0xf7, // MOVDDUP XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movddup_reverse() {
    // Test MOVDDUP in reverse register order
    let code = [
        0xf2, 0x0f, 0x12, 0xf8, // MOVDDUP XMM7, XMM0
        0xf2, 0x0f, 0x12, 0xf7, // MOVDDUP XMM6, XMM7
        0xf2, 0x0f, 0x12, 0xee, // MOVDDUP XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
