use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// MOVNTQ - Store Packed Integers Using Non-Temporal Hint (MMX)
//
// MOVNTQ moves packed integers from MMX register to memory using non-temporal hint.
// The non-temporal hint minimizes cache pollution by using write combining (WC) protocol.
//
// Memory operand must be aligned on 8-byte boundary or #GP exception may occur (model-specific).
// Use SFENCE or MFENCE for ordering with weakly-ordered memory types.
//
// Opcodes:
// NP 0F E7 /r             MOVNTQ m64, mm         - Move packed integers from mm to m64 using non-temporal hint

const ALIGNED_ADDR: u64 = 0x3000; // 8-byte aligned address for testing

// ============================================================================
// MOVNTQ Tests - Non-Temporal Store of Packed Integers (MMX)
// ============================================================================

#[test]
fn test_movntq_mem_mm0() {
    // MOVNTQ [ALIGNED_ADDR], MM0
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm1() {
    // MOVNTQ [ALIGNED_ADDR], MM1
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm2() {
    // MOVNTQ [ALIGNED_ADDR], MM2
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm3() {
    // MOVNTQ [ALIGNED_ADDR], MM3
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm4() {
    // MOVNTQ [ALIGNED_ADDR], MM4
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm5() {
    // MOVNTQ [ALIGNED_ADDR], MM5
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm6() {
    // MOVNTQ [ALIGNED_ADDR], MM6
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mem_mm7() {
    // MOVNTQ [ALIGNED_ADDR], MM7
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_aligned_8byte_boundary() {
    // Test MOVNTQ with 8-byte aligned address
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_aligned_16byte_boundary() {
    // Test MOVNTQ with 16-byte aligned address (also 8-byte aligned)
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_data_integrity_zeros() {
    // Test storing all zeros
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_data_integrity_ones() {
    // Test storing all ones
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_data_integrity_pattern() {
    // Test storing pattern data
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_non_temporal_hint() {
    // Test non-temporal hint behavior
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_sequential_stores() {
    // Test sequential non-temporal stores
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_write_combining() {
    // Test write combining behavior
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_1() {
    // Test with different aligned addresses
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_2() {
    // Test with another aligned address
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_with_different_addresses_3() {
    // Test with yet another aligned address
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_memory_ordering() {
    // Test memory ordering with non-temporal stores
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_cache_bypass() {
    // Test cache bypass behavior
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_weakly_ordered_memory() {
    // Test with weakly ordered memory types
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_full_64bit_data() {
    // Test storing full 64-bit data
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_with_sfence() {
    // Test MOVNTQ followed by SFENCE
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_with_mfence() {
    // Test MOVNTQ followed by MFENCE
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_multiple_sequential() {
    // Test multiple sequential MOVNTQ stores
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0x0f, 0xe7, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_multiple_with_sfence() {
    // Test multiple MOVNTQ stores with SFENCE
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0x0f, 0xe7, 0x24, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM4
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_packed_byte_data() {
    // Test storing packed byte data
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_packed_word_data() {
    // Test storing packed word data
    let code = [
        0x0f, 0xe7, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_packed_dword_data() {
    // Test storing packed dword data
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_streaming_store() {
    // Test streaming store pattern
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xe7, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM1
        0x0f, 0xe7, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVNTQ [0x3010], MM2
        0x0f, 0xe7, 0x1c, 0x25, 0x18, 0x30, 0x00, 0x00, // MOVNTQ [0x3018], MM3
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_high_throughput() {
    // Test high-throughput non-temporal stores
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVNTQ [0x3020], MM4
        0x0f, 0xe7, 0x2c, 0x25, 0x28, 0x30, 0x00, 0x00, // MOVNTQ [0x3028], MM5
        0x0f, 0xe7, 0x34, 0x25, 0x30, 0x30, 0x00, 0x00, // MOVNTQ [0x3030], MM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_large_buffer() {
    // Test storing to large buffer
    let code = [
        0x0f, 0xe7, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM7
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_buffer_fill() {
    // Test buffer fill pattern
    let code = [
        0x0f, 0xe7, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM0
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_write_bypass() {
    // Test write bypass behavior
    let code = [
        0x0f, 0xe7, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_mixed_with_regular_stores() {
    // Test MOVNTQ mixed with regular stores
    let code = [
        0x0f, 0xe7, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM2
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_temporal_locality() {
    // Test temporal locality behavior
    let code = [
        0x0f, 0xe7, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_write_through() {
    // Test write-through behavior
    let code = [
        0x0f, 0xe7, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movntq_performance_optimization() {
    // Test performance optimization pattern
    let code = [
        0x0f, 0xe7, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVNTQ [0x3000], MM5
        0x0f, 0xe7, 0x34, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVNTQ [0x3008], MM6
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
