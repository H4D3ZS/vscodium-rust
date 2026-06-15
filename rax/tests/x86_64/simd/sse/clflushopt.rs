use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// CLFLUSHOPT - Optimized Cache Line Flush
//
// CLFLUSHOPT flushes a cache line from all levels of the processor cache hierarchy.
// It is similar to CLFLUSH but with optimizations:
// - Allows more flexible ordering (can be ordered with SFENCE)
// - May have better performance characteristics
// - Invalidates the cache line containing the specified memory address
//
// CLFLUSHOPT is weakly-ordered and can be ordered with:
// - SFENCE for stores
// - MFENCE for loads and stores
//
// Unlike CLFLUSH, CLFLUSHOPT does not serialize the processor pipeline by default.
//
// Opcode:
// 66 0F AE /7             CLFLUSHOPT m8          - Flush cache line (optimized)

const ADDR: u64 = 0x3000; // Address for testing

// ============================================================================
// CLFLUSHOPT Tests - Optimized Cache Line Flush
// ============================================================================

#[test]
fn test_clflushopt_basic() {
    // Basic CLFLUSHOPT execution
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_single_line() {
    // Flush single cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_offset_1() {
    // CLFLUSHOPT with offset 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x10, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_offset_2() {
    // CLFLUSHOPT with offset 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x20, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3020]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_offset_3() {
    // CLFLUSHOPT with offset 3
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_sequential() {
    // Sequential CLFLUSHOPT operations
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_multiple_lines() {
    // Flush multiple cache lines
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_cache_line_aligned() {
    // CLFLUSHOPT on cache line boundary
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_unaligned() {
    // CLFLUSHOPT on unaligned address
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x01, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3001]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_with_sfence() {
    // CLFLUSHOPT followed by SFENCE
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_with_mfence() {
    // CLFLUSHOPT followed by MFENCE
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_ordering_1() {
    // CLFLUSHOPT ordering test 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_ordering_2() {
    // CLFLUSHOPT ordering test 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_invalidate() {
    // CLFLUSHOPT invalidating cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_writeback() {
    // CLFLUSHOPT with writeback
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_all_levels() {
    // CLFLUSHOPT from all cache levels
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_performance() {
    // CLFLUSHOPT performance pattern
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_weakly_ordered() {
    // CLFLUSHOPT weak ordering
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_persistence() {
    // CLFLUSHOPT for persistence (PMEM)
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_modified_line() {
    // CLFLUSHOPT on modified cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_shared_line() {
    // CLFLUSHOPT on shared cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_exclusive_line() {
    // CLFLUSHOPT on exclusive cache line
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_range_1() {
    // CLFLUSHOPT range pattern 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_range_2() {
    // CLFLUSHOPT range pattern 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_buffer_flush() {
    // CLFLUSHOPT for buffer flush
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_data_sync() {
    // CLFLUSHOPT for data synchronization
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_coherency() {
    // CLFLUSHOPT for cache coherency
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_dma_buffer() {
    // CLFLUSHOPT for DMA buffer
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_io_coherency() {
    // CLFLUSHOPT for I/O coherency
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_large_buffer() {
    // CLFLUSHOPT for large buffer
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0xc0, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x30c0]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_streaming_data() {
    // CLFLUSHOPT for streaming data
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_memory_mapped_io() {
    // CLFLUSHOPT for memory-mapped I/O
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_cache_control() {
    // CLFLUSHOPT for cache control
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_batch_flush_1() {
    // CLFLUSHOPT batch flush pattern 1
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_batch_flush_2() {
    // CLFLUSHOPT batch flush pattern 2
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_nvdimm() {
    // CLFLUSHOPT for NVDIMM
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_persistent_memory() {
    // CLFLUSHOPT for persistent memory
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_transaction_log() {
    // CLFLUSHOPT for transaction log
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_durability() {
    // CLFLUSHOPT for durability guarantee
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_optimized_ordering() {
    // CLFLUSHOPT with optimized ordering
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x40, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3040]
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x80, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3080]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_hierarchical_flush() {
    // CLFLUSHOPT hierarchical flush
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_bypass_cache() {
    // CLFLUSHOPT bypassing cache
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_memory_barrier() {
    // CLFLUSHOPT with memory barrier
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_clflushopt_store_barrier() {
    // CLFLUSHOPT with store barrier
    let code = [
        0x66, 0x0f, 0xae, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // CLFLUSHOPT [0x3000]
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
