use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PUNPCKLBW/PUNPCKLWD/PUNPCKLDQ/PUNPCKLQDQ - Unpack Low Data
//
// These instructions unpack and interleave the low-order data elements of the
// destination and source operands into the destination operand.
//
// PUNPCKLBW - Unpack low bytes
// PUNPCKLWD - Unpack low words
// PUNPCKLDQ - Unpack low doublewords
// PUNPCKLQDQ - Unpack low quadwords
//
// Opcodes:
// 66 0F 60 /r             PUNPCKLBW xmm1, xmm2/m128     - Unpack and interleave low bytes
// 66 0F 61 /r             PUNPCKLWD xmm1, xmm2/m128     - Unpack and interleave low words
// 66 0F 62 /r             PUNPCKLDQ xmm1, xmm2/m128     - Unpack and interleave low doublewords
// 66 0F 6C /r             PUNPCKLQDQ xmm1, xmm2/m128    - Unpack and interleave low quadwords

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PUNPCKLBW Tests - Unpack Low Bytes
// ============================================================================

#[test]
fn test_punpcklbw_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x60, 0xca, // PUNPCKLBW XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x60, 0xd3, // PUNPCKLBW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x60, 0xdc, // PUNPCKLBW XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x60, 0xf8, // PUNPCKLBW XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x60, 0xf8, // PUNPCKLBW XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklbw_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x60, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLBW XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKLWD Tests - Unpack Low Words
// ============================================================================

#[test]
fn test_punpcklwd_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x61, 0xc1, // PUNPCKLWD XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x61, 0xca, // PUNPCKLWD XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x61, 0xd3, // PUNPCKLWD XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x61, 0xdc, // PUNPCKLWD XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x61, 0xf8, // PUNPCKLWD XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x61, 0xc1, // PUNPCKLWD XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x61, 0xf8, // PUNPCKLWD XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklwd_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x61, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLWD XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKLDQ Tests - Unpack Low Doublewords
// ============================================================================

#[test]
fn test_punpckldq_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x62, 0xc1, // PUNPCKLDQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x62, 0xca, // PUNPCKLDQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x62, 0xd3, // PUNPCKLDQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x62, 0xdc, // PUNPCKLDQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x62, 0xe5, // PUNPCKLDQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x62, 0xf8, // PUNPCKLDQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x62, 0xc1, // PUNPCKLDQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x62, 0xf8, // PUNPCKLDQ XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpckldq_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x62, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PUNPCKLQDQ Tests - Unpack Low Quadwords
// ============================================================================

#[test]
fn test_punpcklqdq_xmm0_xmm1() {
    let code = [
        0x66, 0x0f, 0x6c, 0xc1, // PUNPCKLQDQ XMM0, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm1_xmm2() {
    let code = [
        0x66, 0x0f, 0x6c, 0xca, // PUNPCKLQDQ XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm2_xmm3() {
    let code = [
        0x66, 0x0f, 0x6c, 0xd3, // PUNPCKLQDQ XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm3_xmm4() {
    let code = [
        0x66, 0x0f, 0x6c, 0xdc, // PUNPCKLQDQ XMM3, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm4_xmm5() {
    let code = [
        0x66, 0x0f, 0x6c, 0xe5, // PUNPCKLQDQ XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm5_xmm6() {
    let code = [
        0x66, 0x0f, 0x6c, 0xee, // PUNPCKLQDQ XMM5, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm6_xmm7() {
    let code = [
        0x66, 0x0f, 0x6c, 0xf7, // PUNPCKLQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm7_xmm0() {
    let code = [
        0x66, 0x0f, 0x6c, 0xf8, // PUNPCKLQDQ XMM7, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm8_xmm9() {
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xc1, // PUNPCKLQDQ XMM8, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm9_xmm10() {
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xca, // PUNPCKLQDQ XMM9, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm10_xmm11() {
    let code = [
        0x66, 0x45, 0x0f, 0x6c, 0xd3, // PUNPCKLQDQ XMM10, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm15_xmm0() {
    let code = [
        0x66, 0x44, 0x0f, 0x6c, 0xf8, // PUNPCKLQDQ XMM15, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm0_mem() {
    let code = [
        0x66, 0x0f, 0x6c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLQDQ XMM0, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm7_mem() {
    let code = [
        0x66, 0x0f, 0x6c, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // PUNPCKLQDQ XMM7, [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_punpcklqdq_xmm15_mem() {
    let code = [
        0x66, 0x44, 0x0f, 0x6c, 0x3c, 0x25, 0x00, 0x30, 0x00,
        0x00, // PUNPCKLQDQ XMM15, [0x3000]
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
        0x66, 0x0f, 0x60, 0xc1, // PUNPCKLBW XMM0, XMM1
        0x66, 0x0f, 0x61, 0xd3, // PUNPCKLWD XMM2, XMM3
        0x66, 0x0f, 0x62, 0xe5, // PUNPCKLDQ XMM4, XMM5
        0x66, 0x0f, 0x6c, 0xf7, // PUNPCKLQDQ XMM6, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PUNPCKL* interleaves the LOW halves of DST and SRC. Output element 0 comes
// from DST, element 1 from SRC, alternating.
//   DST = XMM0 = 0x0102030405060708090A0B0C0D0E0F10
//   SRC = XMM1 = 0x1112131415161718191A1B1C1D1E1F20
// Computed by hand from x86 unpack-low semantics.
// ============================================================================

const KAT_UNP_DST: u128 = 0x0102030405060708090A0B0C0D0E0F10;
const KAT_UNP_SRC: u128 = 0x1112131415161718191A1B1C1D1E1F20;

#[test]
fn kat_punpcklbw_value() {
    // PUNPCKLBW XMM0, XMM1 (66 0F 60 C1)
    let code = [0x66, 0x0f, 0x60, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNP_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNP_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x19091a0a1b0b1c0c1d0d1e0e1f0f2010,
        "PUNPCKLBW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpcklwd_value() {
    // PUNPCKLWD XMM0, XMM1 (66 0F 61 C1)
    let code = [0x66, 0x0f, 0x61, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNP_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNP_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x191a090a1b1c0b0c1d1e0d0e1f200f10,
        "PUNPCKLWD got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpckldq_value() {
    // PUNPCKLDQ XMM0, XMM1 (66 0F 62 C1)
    let code = [0x66, 0x0f, 0x62, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNP_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNP_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x191a1b1c090a0b0c1d1e1f200d0e0f10,
        "PUNPCKLDQ got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_punpcklqdq_value() {
    // PUNPCKLQDQ XMM0, XMM1 (66 0F 6C C1): low qword of each operand.
    let code = [0x66, 0x0f, 0x6c, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, KAT_UNP_DST);
    set_xmm(&mem, &mut vcpu, 1, KAT_UNP_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x191a1b1c1d1e1f20090a0b0c0d0e0f10,
        "PUNPCKLQDQ got {:032x}",
        get_xmm(&regs, 0)
    );
}
