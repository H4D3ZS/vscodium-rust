use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PREFETCH Instructions - Data Prefetch Hints
//
// PREFETCHNTA - Prefetch data into non-temporal cache structure (minimal cache pollution)
// PREFETCHT0  - Prefetch data into all cache levels (T0 = temporal locality, all levels)
// PREFETCHT1  - Prefetch data into L2 and higher (T1 = temporal locality, level 2 and up)
// PREFETCHT2  - Prefetch data into L3 and higher (T2 = temporal locality, level 3 and up)
//
// These are hint instructions - they do not cause exceptions and may be treated as NOPs.
// They provide cache locality hints to improve performance by preloading data.
//
// Opcodes:
// NP 0F 18 /0             PREFETCHNTA m8         - Prefetch data to non-temporal cache
// NP 0F 18 /1             PREFETCHT0 m8          - Prefetch data to all cache levels
// NP 0F 18 /2             PREFETCHT1 m8          - Prefetch data to L2 and higher
// NP 0F 18 /3             PREFETCHT2 m8          - Prefetch data to L3 and higher

const ADDR: u64 = 0x3000; // Address for testing

// ============================================================================
// PREFETCHNTA Tests - Non-Temporal Data Prefetch
// ============================================================================

#[test]
fn test_prefetchnta_basic() {
    // PREFETCHNTA [ADDR]
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_offset_1() {
    // PREFETCHNTA with offset
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x10, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_offset_2() {
    // PREFETCHNTA with different offset
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x20, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3020]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_sequential() {
    // Sequential PREFETCHNTA operations
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0x0f, 0x18, 0x04, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_cache_line() {
    // PREFETCHNTA aligned to cache line
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_multiple_lines() {
    // PREFETCHNTA multiple cache lines
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0x0f, 0x18, 0x04, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3040]
        0x0f, 0x18, 0x04, 0x25, 0x80, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3080]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_before_load() {
    // PREFETCHNTA before memory load
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_streaming() {
    // PREFETCHNTA for streaming data
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_non_temporal() {
    // PREFETCHNTA non-temporal access pattern
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetchnta_large_data() {
    // PREFETCHNTA for large data structure
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PREFETCHT0 Tests - Prefetch to All Cache Levels
// ============================================================================

#[test]
fn test_prefetcht0_basic() {
    // PREFETCHT0 [ADDR]
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_offset_1() {
    // PREFETCHT0 with offset
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x10, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_offset_2() {
    // PREFETCHT0 with different offset
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x20, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3020]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_sequential() {
    // Sequential PREFETCHT0 operations
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0x0f, 0x18, 0x0c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_all_levels() {
    // PREFETCHT0 to all cache levels
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_temporal_data() {
    // PREFETCHT0 for temporal data
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_hot_data() {
    // PREFETCHT0 for hot data
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_before_loop() {
    // PREFETCHT0 before loop access
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_multiple_lines() {
    // PREFETCHT0 multiple cache lines
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0x0f, 0x18, 0x0c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht0_frequently_accessed() {
    // PREFETCHT0 for frequently accessed data
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PREFETCHT1 Tests - Prefetch to L2 and Higher
// ============================================================================

#[test]
fn test_prefetcht1_basic() {
    // PREFETCHT1 [ADDR]
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_offset_1() {
    // PREFETCHT1 with offset
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_offset_2() {
    // PREFETCHT1 with different offset
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x20, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3020]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_sequential() {
    // Sequential PREFETCHT1 operations
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0x0f, 0x18, 0x14, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_l2_cache() {
    // PREFETCHT1 to L2 cache
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_moderate_locality() {
    // PREFETCHT1 for moderate temporal locality
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_warm_data() {
    // PREFETCHT1 for warm data
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_reuse_distance() {
    // PREFETCHT1 with moderate reuse distance
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_multiple_lines() {
    // PREFETCHT1 multiple cache lines
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0x0f, 0x18, 0x14, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht1_infrequent_access() {
    // PREFETCHT1 for infrequently accessed data
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PREFETCHT2 Tests - Prefetch to L3 and Higher
// ============================================================================

#[test]
fn test_prefetcht2_basic() {
    // PREFETCHT2 [ADDR]
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_offset_1() {
    // PREFETCHT2 with offset
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x10, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_offset_2() {
    // PREFETCHT2 with different offset
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x20, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3020]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_sequential() {
    // Sequential PREFETCHT2 operations
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0x0f, 0x18, 0x1c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_l3_cache() {
    // PREFETCHT2 to L3 cache
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_low_locality() {
    // PREFETCHT2 for low temporal locality
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_cold_data() {
    // PREFETCHT2 for cold data
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_large_reuse_distance() {
    // PREFETCHT2 with large reuse distance
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_multiple_lines() {
    // PREFETCHT2 multiple cache lines
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0x0f, 0x18, 0x1c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetcht2_rare_access() {
    // PREFETCHT2 for rarely accessed data
    let code = [
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Mixed PREFETCH Tests - Different Hint Combinations
// ============================================================================

#[test]
fn test_prefetch_mixed_nta_t0() {
    // Mix PREFETCHNTA and PREFETCHT0
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0x0f, 0x18, 0x0c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetch_mixed_t0_t1() {
    // Mix PREFETCHT0 and PREFETCHT1
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0x0f, 0x18, 0x14, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetch_mixed_t1_t2() {
    // Mix PREFETCHT1 and PREFETCHT2
    let code = [
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3000]
        0x0f, 0x18, 0x1c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT2 [0x3040]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetch_all_hints() {
    // All prefetch hints in sequence
    let code = [
        0x0f, 0x18, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHNTA [0x3000]
        0x0f, 0x18, 0x0c, 0x25, 0x40, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3040]
        0x0f, 0x18, 0x14, 0x25, 0x80, 0x30, 0x00, 0x00, // PREFETCHT1 [0x3080]
        0x0f, 0x18, 0x1c, 0x25, 0xc0, 0x30, 0x00, 0x00, // PREFETCHT2 [0x30c0]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetch_strided_pattern() {
    // Strided prefetch pattern
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0x0f, 0x18, 0x0c, 0x25, 0x80, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3080]
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x31, 0x00, 0x00, // PREFETCHT0 [0x3100]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_prefetch_hierarchical() {
    // Hierarchical prefetch pattern (T0 -> T1 -> T2)
    let code = [
        0x0f, 0x18, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PREFETCHT0 [0x3000]
        0x0f, 0x18, 0x14, 0x25, 0x00, 0x31, 0x00, 0x00, // PREFETCHT1 [0x3100]
        0x0f, 0x18, 0x1c, 0x25, 0x00, 0x32, 0x00, 0x00, // PREFETCHT2 [0x3200]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
