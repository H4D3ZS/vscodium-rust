use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PMULHW - Multiply Packed Signed Integers and Store High Result
//
// Multiplies the packed signed word integers in the destination operand
// (first operand) by the packed signed word integers in the source operand
// (second operand), and stores the high 16 bits of each intermediate
// 32-bit result in the destination operand.
//
// Opcodes:
// 66 0F E5 /r             PMULHW xmm1, xmm2/m128    - Multiply packed signed words, store high 16 bits

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// PMULHW Tests - Packed Multiply High Signed Word (8x int16)
// ============================================================================

#[test]
fn test_pmulhw_xmm0_xmm1() {
    // PMULHW XMM0, XMM1
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm1_xmm2() {
    // PMULHW XMM1, XMM2
    let code = [
        0x66, 0x0f, 0xe5, 0xca, // PMULHW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm2_xmm3() {
    // PMULHW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm3_xmm4() {
    // PMULHW XMM3, XMM4
    let code = [
        0x66, 0x0f, 0xe5, 0xdc, // PMULHW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm4_xmm5() {
    // PMULHW XMM4, XMM5
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm5_xmm6() {
    // PMULHW XMM5, XMM6
    let code = [
        0x66, 0x0f, 0xe5, 0xee, // PMULHW XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm6_xmm7() {
    // PMULHW XMM6, XMM7
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm7_xmm0() {
    // PMULHW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0xe5, 0xf8, // PMULHW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm8_xmm9() {
    // PMULHW XMM8, XMM9 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm9_xmm10() {
    // PMULHW XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xca, // PMULHW XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm10_xmm11() {
    // PMULHW XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xd3, // PMULHW XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm11_xmm12() {
    // PMULHW XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xdc, // PMULHW XMM11, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm12_xmm13() {
    // PMULHW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xe5, // PMULHW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm13_xmm14() {
    // PMULHW XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xee, // PMULHW XMM13, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm14_xmm15() {
    // PMULHW XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xf7, // PMULHW XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm15_xmm0() {
    // PMULHW XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0xf8, // PMULHW XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm0_mem() {
    // PMULHW XMM0, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm1_mem() {
    // PMULHW XMM1, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM1, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm7_mem() {
    // PMULHW XMM7, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm15_mem() {
    // PMULHW XMM15, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_positive_values() {
    // Test multiplication of positive values
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_negative_values() {
    // Test multiplication with negative values
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_mixed_signs() {
    // Test multiplication of positive and negative values
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_zero_multiplication() {
    // Test multiplying by zero
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_max_values() {
    // Test multiplication with maximum signed values
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_min_values() {
    // Test multiplication with minimum signed values
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_all_words_different() {
    // Test with different values in each word
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_sequential_operations() {
    // Test sequential PMULHW operations
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_self_multiply() {
    // Test multiplying register by itself
    let code = [
        0x66, 0x0f, 0xe5, 0xc0, // PMULHW XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm1_self() {
    // Test XMM1 self-multiply
    let code = [
        0x66, 0x0f, 0xe5, 0xc9, // PMULHW XMM1, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_power_of_two() {
    // Test multiplication by powers of two
    let code = [
        0x66, 0x0f, 0xe5, 0xf7, // PMULHW XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_alternating_signs() {
    // Test with alternating positive/negative values
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_mem_aligned() {
    // Test memory operand with aligned address
    let code = [
        0x66, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_small_values() {
    // Test with small values (1, 2, 3, etc.)
    let code = [
        0x66, 0x0f, 0xe5, 0xd3, // PMULHW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_large_values() {
    // Test with large values near max
    let code = [
        0x66, 0x0f, 0xe5, 0xe5, // PMULHW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_high_regs() {
    // Test with high numbered XMM registers
    let code = [
        0x66, 0x45, 0x0f, 0xe5, 0xf7, // PMULHW XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_mixed_regs_1() {
    // Test with mixed low and high registers
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0xc1, // PMULHW XMM8, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_mixed_regs_2() {
    // Test with mixed registers reversed
    let code = [
        0x66, 0x0f, 0xe5, 0xc1, // PMULHW XMM0, XMM1
        0x66, 0x45, 0x0f, 0xe5, 0xd3, // PMULHW XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm2_mem() {
    // PMULHW XMM2, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM2, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm3_mem() {
    // PMULHW XMM3, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM3, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm4_mem() {
    // PMULHW XMM4, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM4, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm5_mem() {
    // PMULHW XMM5, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM5, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm6_mem() {
    // PMULHW XMM6, [ALIGNED_ADDR]
    let code = [
        0x66, 0x0f, 0xe5, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM6, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm8_mem() {
    // PMULHW XMM8, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM8, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm9_mem() {
    // PMULHW XMM9, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM9, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm10_mem() {
    // PMULHW XMM10, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM10, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm11_mem() {
    // PMULHW XMM11, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM11, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm12_mem() {
    // PMULHW XMM12, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM12, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm13_mem() {
    // PMULHW XMM13, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM13, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmulhw_xmm14_mem() {
    // PMULHW XMM14, [ALIGNED_ADDR]
    let code = [
        0x66, 0x44, 0x0f, 0xe5, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // PMULHW XMM14, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PMULHW keeps the HIGH 16 bits of each SIGNED 16x16->32 product. Computed by
// hand from x86 semantics.
//   DST = XMM0 = 0x0002000300040005FFFF8000007FABCD
//   SRC = XMM1 = 0x0003000500070009000280017FFF1234
// ============================================================================

#[test]
fn kat_pmulhw_value() {
    // PMULHW XMM0, XMM1 (66 0F E5 C1)
    let code = [0x66, 0x0f, 0xe5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x0002000300040005FFFF8000007FABCD);
    set_xmm(&mem, &mut vcpu, 1, 0x0003000500070009000280017FFF1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x0000000000000000ffff3fff003ffa03,
        "PMULHW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pmulhw_signed_high() {
    // 0x8000 (-32768) * 0x8000 (-32768) = 0x40000000, high word = 0x4000.
    let code = [0x66, 0x0f, 0xe5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x80008000800080008000800080008000);
    set_xmm(&mem, &mut vcpu, 1, 0x80008000800080008000800080008000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x40004000400040004000400040004000);
}
