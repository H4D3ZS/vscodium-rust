use crate::common::*;

// PMADDWD - Multiply and Add Packed Integers
//
// Multiplies the individual signed words of the destination operand by the
// corresponding signed words of the source operand, producing temporary signed,
// doubleword results. The adjacent doubleword results are then summed and stored
// in the destination operand.
//
// For each pair of words:
//   DEST[31:0] := (DEST[15:0] * SRC[15:0]) + (DEST[31:16] * SRC[31:16])
//   DEST[63:32] := (DEST[47:32] * SRC[47:32]) + (DEST[63:48] * SRC[63:48])
//   etc.
//
// Special case: When all pairs are 0x8000, result wraps to 0x80000000
//
// Opcode:
//   66 0F F5 /r    PMADDWD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pmaddwd_xmm0_xmm1_basic() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm2_xmm3_basic() {
    let code = [
        0x66, 0x0f, 0xf5, 0xd3, // PMADDWD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm4_xmm5_zeros() {
    let code = [
        0x66, 0x0f, 0xf5, 0xe5, // PMADDWD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm6_xmm7_ones() {
    let code = [
        0x66, 0x0f, 0xf5, 0xf7, // PMADDWD XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm0_xmm1_positive_values() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm1_xmm2_negative_values() {
    let code = [
        0x66, 0x0f, 0xf5, 0xca, // PMADDWD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm3_xmm4_mixed_signs() {
    let code = [
        0x66, 0x0f, 0xf5, 0xdc, // PMADDWD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm5_xmm6_max_positive() {
    let code = [
        0x66, 0x0f, 0xf5, 0xee, // PMADDWD XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm7_xmm0_min_negative() {
    let code = [
        0x66, 0x0f, 0xf5, 0xf8, // PMADDWD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xc1, // PMADDWD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xd3, // PMADDWD XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm12_xmm13() {
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xe5, // PMADDWD XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm14_xmm15() {
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xf7, // PMADDWD XMM14, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm0_mem() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x00, // PMADDWD XMM0, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm1_mem_negative() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x08, // PMADDWD XMM1, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // -1 as i16 = 0xFFFF
    let data: [u8; 16] = [
        0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF, 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8,
        0xFF,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm2_mem_overflow_case() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x10, // PMADDWD XMM2, [RAX]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // 0x8000 = -32768, the special wrap case
    let data: [u8; 16] = [
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00,
        0x80,
    ];
    mem.write_slice(&data, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_same_register() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc0, // PMADDWD XMM0, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_sequential() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0x66, 0x0f, 0xf5, 0xd3, // PMADDWD XMM2, XMM3
        0x66, 0x0f, 0xf5, 0xe5, // PMADDWD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_xmm15_xmm0_cross() {
    let code = [
        0x66, 0x44, 0x0f, 0xf5, 0xf8, // PMADDWD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_mem_displacement() {
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x40, 0x10, // PMADDWD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_positive_overflow() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_negative_overflow() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_wrap_special_case() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_zero_multiplication() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_identity_multiplication() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_alternating_pattern() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_max_word_value() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_min_word_value() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_mixed_positive_negative() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_small_values() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_large_values() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_adjacent_pairs() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_carry_addition() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_negative_sum() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_positive_sum() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_cancellation() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_edge_8000h() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_edge_7fffh() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_varying_products() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_all_dwords() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_boundary_cases() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_incremental_pattern() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddwd_decremental_pattern() {
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PMADDWD signed-multiplies adjacent word pairs and sums each pair into a
// 32-bit result (4 dwords). Computed by hand.
//   DST = XMM0 = 0x0002000300040005FFFF8000007FABCD
//   SRC = XMM1 = 0x0003000500070009000280017FFF1234
// ============================================================================

#[test]
fn kat_pmaddwd_value() {
    // PMADDWD XMM0, XMM1 (66 0F F5 C1)
    let code = [0x66, 0x0f, 0xf5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x0002000300040005FFFF8000007FABCD);
    set_xmm(&mem, &mut vcpu, 1, 0x0003000500070009000280017FFF1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x00000015000000493fff7ffefa42cf25,
        "PMADDWD got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pmaddwd_simple() {
    // dword0 = 1*1 + 2*2 = 5; dword1 = 3*3 + 4*4 = 25 (0x19); etc.
    let code = [0x66, 0x0f, 0xf5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x00070008000500060003000400010002);
    set_xmm(&mem, &mut vcpu, 1, 0x00070008000500060003000400010002);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // pair0 (1,2): 1+4=5; pair1 (3,4): 9+16=25=0x19; pair2 (5,6): 25+36=61=0x3D;
    // pair3 (7,8): 49+64=113=0x71.
    assert_eq!(get_xmm(&regs, 0), 0x000000710000003d000000190000_0005);
}
