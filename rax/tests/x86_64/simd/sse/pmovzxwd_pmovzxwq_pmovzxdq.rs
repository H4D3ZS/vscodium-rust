use crate::common::*;
use vm_memory::{Bytes, GuestAddress};

// PMOVZXWD/PMOVZXWQ/PMOVZXDQ - Zero Extend Packed Integer Types
//
// These instructions zero-extend packed unsigned integers to larger integer types.
//
// PMOVZXWD: Zero extend 4 packed unsigned words to 4 packed unsigned dwords
// PMOVZXWQ: Zero extend 2 packed unsigned words to 2 packed unsigned qwords
// PMOVZXDQ: Zero extend 2 packed unsigned dwords to 2 packed unsigned qwords
//
// Opcodes:
// 66 0F 38 33 /r      PMOVZXWD xmm1, xmm2/m64   - Zero extend 4 words to 4 dwords
// 66 0F 38 34 /r      PMOVZXWQ xmm1, xmm2/m32   - Zero extend 2 words to 2 qwords
// 66 0F 38 35 /r      PMOVZXDQ xmm1, xmm2/m64   - Zero extend 2 dwords to 2 qwords

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PMOVZXWD Tests - 4 Words to 4 Dwords
// ============================================================================

#[test]
fn test_pmovzxwd_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_small_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x80, 0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0x7F, 0x00, 0x80, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x33, 0x00, // PMOVZXWD XMM0, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x11, 0x11, 0x22, 0x22, 0x33, 0x33, 0x44, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_xmm2_xmm3() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xda, // PMOVZXWD XMM3, XMM2
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_all_ones() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PMOVZXWQ Tests - 2 Words to 2 Qwords
// ============================================================================

#[test]
fn test_pmovzxwq_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, // PMOVZXWQ XMM1, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_small_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x80, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0x7F, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x34, 0x00, // PMOVZXWQ XMM0, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x11, 0x11, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_xmm4_xmm5() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x38, 0x34, 0xec, // PMOVZXWQ XMM5, XMM4
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_all_ones() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PMOVZXDQ Tests - 2 Dwords to 2 Qwords
// ============================================================================

#[test]
fn test_pmovzxdq_all_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, // PMOVZXDQ XMM1, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_small_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x35, 0x00, // PMOVZXDQ XMM0, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_xmm6_xmm7() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x38, 0x35, 0xfe, // PMOVZXDQ XMM7, XMM6
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_all_ones() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_sequential() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pmovzxwd_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM9, XMM8
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x34, 0xda, // PMOVZXWQ XMM11, XMM10
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_xmm12_xmm13() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x35, 0xec, // PMOVZXDQ XMM13, XMM12
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_xmm14_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x33, 0x30, // PMOVZXWD XMM14, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0x7F, 0x00, 0x80, 0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_xmm15_from_memory() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x34, 0x38, // PMOVZXWQ XMM15, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0xFF, 0x7F, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Combined/Sequence Tests
// ============================================================================

#[test]
fn test_pmovzxwd_pmovzxwq_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x34, 0xd0, // PMOVZXWQ XMM2, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_all_pmovzxw_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x34, 0xd0, // PMOVZXWQ XMM2, XMM0
        0x66, 0x0f, 0x38, 0x35, 0xd8, // PMOVZXDQ XMM3, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_chain() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x35, 0xd1, // PMOVZXDQ XMM2, XMM1
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_double_memory_load() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x33, 0x00, // PMOVZXWD XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0x0b, // PMOVZXWD XMM1, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_double_memory_load() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x34, 0x00, // PMOVZXWQ XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x34, 0x0b, // PMOVZXWQ XMM1, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0xFF, 0xFF, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_double_memory_load() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x35, 0x00, // PMOVZXDQ XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x35, 0x0b, // PMOVZXDQ XMM1, [RBX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1 = [
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let data2 = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, GuestAddress(ALIGNED_ADDR)).unwrap();
    mem.write_slice(&data2, GuestAddress(ALIGNED_ADDR2))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwd_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x33, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxwq_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x34, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovzxdq_alternating() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x38, 0x35, 0xc8, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data = [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data, GuestAddress(ALIGNED_ADDR)).unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
// ============================================================================

#[test]
fn kat_pmovzxwd_value() {
    // PMOVZXWD XMM0, XMM1 (66 0F 38 33 C1): low 4 words -> 4 zero-ext dwords.
    // words 0x0001, 0xFFFF, 0x7FFF, 0x8000
    let code = [0x66, 0x0f, 0x38, 0x33, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000000_8000_7FFF_FFFF_0001);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x00008000_00007FFF_0000FFFF_00000001,
        "PMOVZXWD got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pmovzxwq_value() {
    // PMOVZXWQ XMM0, XMM1 (66 0F 38 34 C1): low 2 words -> 2 zero-ext qwords.
    let code = [0x66, 0x0f, 0x38, 0x34, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x000000000000000000000000_FFFF_7FFF);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x000000000000FFFF_0000000000007FFF,
        "PMOVZXWQ got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pmovzxdq_value() {
    // PMOVZXDQ XMM0, XMM1 (66 0F 38 35 C1): low 2 dwords -> 2 zero-ext qwords.
    let code = [0x66, 0x0f, 0x38, 0x35, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000000_FFFFFFFF_7FFFFFFF);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x00000000FFFFFFFF_000000007FFFFFFF,
        "PMOVZXDQ got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
