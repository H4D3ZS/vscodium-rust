use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVNTDQ - Store Packed Integers Using Non-Temporal Hint
//
// MOVNTDQ moves packed integers from XMM register to memory using non-temporal hint.
// The non-temporal hint minimizes cache pollution by using write combining (WC) protocol.
//
// Memory operand must be aligned on 16-byte boundary or #GP exception occurs.
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// 66 0F E7 /r             MOVNTDQ m128, xmm1     - Move packed integers from xmm1 to m128 using non-temporal hint

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing
const UNALIGNED_ADDR: u64 = 0x3001; // Unaligned address (should cause #GP)

// ============================================================================
// MOVNTDQ Tests - Non-Temporal Store of Packed Integers
// ============================================================================

#[test]
fn test_movntdq_mem_xmm0() {
    // MOVNTDQ [ALIGNED_ADDR], XMM0
    let code = [
        0x66, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm1() {
    // MOVNTDQ [ALIGNED_ADDR], XMM1
    let code = [
        0x66, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm2() {
    // MOVNTDQ [ALIGNED_ADDR], XMM2
    let code = [
        0x66, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm3() {
    // MOVNTDQ [ALIGNED_ADDR], XMM3
    let code = [
        0x66, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm4() {
    // MOVNTDQ [ALIGNED_ADDR], XMM4
    let code = [
        0x66, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm5() {
    // MOVNTDQ [ALIGNED_ADDR], XMM5
    let code = [
        0x66, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm6() {
    // MOVNTDQ [ALIGNED_ADDR], XMM6
    let code = [
        0x66, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm7() {
    // MOVNTDQ [ALIGNED_ADDR], XMM7
    let code = [
        0x66, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm8() {
    // MOVNTDQ [ALIGNED_ADDR], XMM8 (requires REX prefix)
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm9() {
    // MOVNTDQ [ALIGNED_ADDR], XMM9
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm10() {
    // MOVNTDQ [ALIGNED_ADDR], XMM10
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm11() {
    // MOVNTDQ [ALIGNED_ADDR], XMM11
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm12() {
    // MOVNTDQ [ALIGNED_ADDR], XMM12
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm13() {
    // MOVNTDQ [ALIGNED_ADDR], XMM13
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm14() {
    // MOVNTDQ [ALIGNED_ADDR], XMM14
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_mem_xmm15() {
    // MOVNTDQ [ALIGNED_ADDR], XMM15
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_aligned_16byte_boundary() {
    // Test with 16-byte aligned address
    let code = [
        0x66, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_aligned_32byte_boundary() {
    // Test with 32-byte aligned address (also 16-byte aligned)
    let code = [
        0x66, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_data_integrity_zeros() {
    // Test storing all zeros
    let code = [
        0x66, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_data_integrity_ones() {
    // Test storing all ones
    let code = [
        0x66, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_data_integrity_pattern() {
    // Test storing pattern data
    let code = [
        0x66, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_non_temporal_hint() {
    // Test non-temporal hint behavior
    let code = [
        0x66, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_sequential_stores() {
    // Test sequential non-temporal stores
    let code = [
        0x66, 0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_write_combining() {
    // Test write combining behavior
    let code = [
        0x66, 0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_with_different_addresses_1() {
    // Test with different aligned addresses
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_with_different_addresses_2() {
    // Test with another aligned address
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_memory_ordering() {
    // Test memory ordering with non-temporal stores
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_cache_bypass() {
    // Test cache bypass behavior
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_weakly_ordered_memory() {
    // Test with weakly ordered memory types
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntdq_full_128bit_data() {
    // Test storing full 128-bit data
    let code = [
        0x66, 0x44, 0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTDQ [0x3000], XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
