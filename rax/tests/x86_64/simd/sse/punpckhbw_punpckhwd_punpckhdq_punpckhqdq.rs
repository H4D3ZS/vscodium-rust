use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PUNPCKHBW/PUNPCKHWD/PUNPCKHDQ/PUNPCKHQDQ - Unpack High Data
//
// These instructions unpack and interleave the high-order data elements of the
// destination and source operands into the destination operand.
//
// PUNPCKHBW - Unpack high bytes
// PUNPCKHWD - Unpack high words
// PUNPCKHDQ - Unpack high doublewords
// PUNPCKHQDQ - Unpack high quadwords
//
// Opcodes:
// 66 0F 68 /r             PUNPCKHBW xmm1, xmm2/m128     - Unpack and interleave high bytes
// 66 0F 69 /r             PUNPCKHWD xmm1, xmm2/m128     - Unpack and interleave high words
// 66 0F 6A /r             PUNPCKHDQ xmm1, xmm2/m128     - Unpack and interleave high doublewords
// 66 0F 6D /r             PUNPCKHQDQ xmm1, xmm2/m128    - Unpack and interleave high quadwords

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PUNPCKHBW Tests - Unpack High Bytes
// ============================================================================

#[test]
fn test_punpckhbw_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x68, 0xca, // PUNPCKHBW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x68, 0xd3, // PUNPCKHBW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x68, 0xf8, // PUNPCKHBW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x68, 0xf8, // PUNPCKHBW XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x68, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHBW XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhbw_xmm7_mem() {
    let code = [
        0x66, 0x0f, 0x68, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHBW XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKHWD Tests - Unpack High Words
// ============================================================================

#[test]
fn test_punpckhwd_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x69, 0xc1, // PUNPCKHWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x69, 0xca, // PUNPCKHWD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x69, 0xd3, // PUNPCKHWD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x69, 0xdc, // PUNPCKHWD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x69, 0xe5, // PUNPCKHWD XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x69, 0xf8, // PUNPCKHWD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x69, 0xc1, // PUNPCKHWD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x69, 0xf8, // PUNPCKHWD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x69, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHWD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhwd_xmm7_mem() {
    let code = [
        0x66, 0x0f, 0x69, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHWD XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKHDQ Tests - Unpack High Doublewords
// ============================================================================

#[test]
fn test_punpckhdq_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x6a, 0xc1, // PUNPCKHDQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x6a, 0xca, // PUNPCKHDQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x6a, 0xd3, // PUNPCKHDQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x6a, 0xdc, // PUNPCKHDQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x6a, 0xe5, // PUNPCKHDQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm5_xmm6() {
    let code = [
        0x66, 0x0f, 0x6a, 0xee, // PUNPCKHDQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm6_xmm7() {
    let code = [
        0x66, 0x0f, 0x6a, 0xf7, // PUNPCKHDQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x6a, 0xf8, // PUNPCKHDQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x6a, 0xc1, // PUNPCKHDQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x6a, 0xf8, // PUNPCKHDQ XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x6a, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhdq_xmm7_mem() {
    let code = [
        0x66, 0x0f, 0x6a, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKHQDQ Tests - Unpack High Quadwords
// ============================================================================

#[test]
fn test_punpckhqdq_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x6d, 0xc1, // PUNPCKHQDQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x6d, 0xca, // PUNPCKHQDQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x6d, 0xd3, // PUNPCKHQDQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x6d, 0xdc, // PUNPCKHQDQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x6d, 0xe5, // PUNPCKHQDQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm5_xmm6() {
    let code = [
        0x66, 0x0f, 0x6d, 0xee, // PUNPCKHQDQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm6_xmm7() {
    let code = [
        0x66, 0x0f, 0x6d, 0xf7, // PUNPCKHQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x6d, 0xf8, // PUNPCKHQDQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xc1, // PUNPCKHQDQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm9_xmm10() {
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xca, // PUNPCKHQDQ XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x6d, 0xd3, // PUNPCKHQDQ XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x6d, 0xf8, // PUNPCKHQDQ XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x6d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHQDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm7_mem() {
    let code = [
        0x66, 0x0f, 0x6d, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKHQDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckhqdq_xmm15_mem() {
    let code = [
        0x66, 0x44, 0x0f, 0x6d, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // PUNPCKHQDQ XMM15, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Mixed Tests - Various Unpack Operations
// ============================================================================

#[test]
fn test_unpack_sequential_operations() {
    // Test sequential unpack operations
    let code = [
        0x66, 0x0f, 0x68, 0xc1, // PUNPCKHBW XMM0, XMM1
        0x66, 0x0f, 0x69, 0xd3, // PUNPCKHWD XMM2, XMM3
        0x66, 0x0f, 0x6a, 0xe5, // PUNPCKHDQ XMM4, XMM5
        0x66, 0x0f, 0x6d, 0xf7, // PUNPCKHQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PUNPCKH* interleaves the HIGH halves of DST and SRC. Output element 0 comes
// from DST's high half, element 1 from SRC's high half, alternating.
//   DST = XMM0 = 0x0102030405060708090A0B0C0D0E0F10
//   SRC = XMM1 = 0x1112131415161718191A1B1C1D1E1F20
// Computed by hand from x86 unpack-high semantics.
// ============================================================================

const KAT_UNPH_DST: u128 = 0x0102030405060708090A0B0C0D0E0F10;
const KAT_UNPH_SRC: u128 = 0x1112131415161718191A1B1C1D1E1F20;

#[test]
fn kat_punpckhbw_value() {
    // PUNPCKHBW XMM0, XMM1 (66 0F 68 C1)
    let code = [0x66, 0x0f, 0x68, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNPH_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNPH_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x11011202130314041505160617071808,
        "PUNPCKHBW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpckhwd_value() {
    // PUNPCKHWD XMM0, XMM1 (66 0F 69 C1)
    let code = [0x66, 0x0f, 0x69, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNPH_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNPH_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x11120102131403041516050617180708,
        "PUNPCKHWD got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpckhdq_value() {
    // PUNPCKHDQ XMM0, XMM1 (66 0F 6A C1)
    let code = [0x66, 0x0f, 0x6a, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNPH_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNPH_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x11121314010203041516171805060708,
        "PUNPCKHDQ got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpckhqdq_value() {
    // PUNPCKHQDQ XMM0, XMM1 (66 0F 6D C1): high qword of each operand.
    let code = [0x66, 0x0f, 0x6d, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNPH_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNPH_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x11121314151617180102030405060708,
        "PUNPCKHQDQ got {:032x}",
        get_xmm(&regs, 0)
    );
}
