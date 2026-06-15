use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};

// PMULLW - Multiply Packed Signed Integers and Store Low Result
//
// Performs a SIMD signed multiply of the packed signed word integers from both
// source operands and stores the low 16 bits of each 32-bit result in the destination.
//
// Opcodes:
// NP 0F D5 /r         PMULLW mm, mm/m64        - Multiply packed signed words, store low
// 66 0F D5 /r         PMULLW xmm1, xmm2/m128   - Multiply packed signed words, store low

const DATA_ADDR: u64 = 0x3000;

// MMX Tests
#[test]
fn test_pmullw_mm0_mm1() {
    let code = [0x0f, 0xd5, 0xc1, 0xf4]; // PMULLW MM0, MM1; HLT
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mm2_mm3() {
    let code = [0x0f, 0xd5, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mm4_mm5() {
    let code = [0x0f, 0xd5, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mm6_mm7() {
    let code = [0x0f, 0xd5, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mm0_mem() {
    let code = [0x0f, 0xd5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mm7_mem() {
    let code = [0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// SSE2 Tests
#[test]
fn test_pmullw_xmm0_xmm1() {
    let code = [0x66, 0x0f, 0xd5, 0xc1, 0xf4]; // PMULLW XMM0, XMM1; HLT
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm2_xmm3() {
    let code = [0x66, 0x0f, 0xd5, 0xd3, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm4_xmm5() {
    let code = [0x66, 0x0f, 0xd5, 0xe5, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm6_xmm7() {
    let code = [0x66, 0x0f, 0xd5, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm8_xmm9() {
    let code = [0x66, 0x45, 0x0f, 0xd5, 0xc1, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm14_xmm15() {
    let code = [0x66, 0x45, 0x0f, 0xd5, 0xf7, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm0_mem() {
    let code = [0x66, 0x0f, 0xd5, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm7_mem() {
    let code = [0x66, 0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_xmm15_mem() {
    let code = [
        0x66, 0x44, 0x0f, 0xd5, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_multiple_mmx() {
    let code = [
        0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0xd5, 0xd3, // PMULLW MM2, MM3
        0x0f, 0xd5, 0xe5, // PMULLW MM4, MM5
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_multiple_xmm() {
    let code = [
        0x66, 0x0f, 0xd5, 0xc1, // PMULLW XMM0, XMM1
        0x66, 0x0f, 0xd5, 0xd3, // PMULLW XMM2, XMM3
        0x66, 0x0f, 0xd5, 0xe5, // PMULLW XMM4, XMM5
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_chain() {
    let code = [
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_all_xmm_pairs() {
    let code = [
        0x66, 0x0f, 0xd5, 0xc1, // PMULLW XMM0, XMM1
        0x66, 0x0f, 0xd5, 0xda, // PMULLW XMM3, XMM2
        0x66, 0x0f, 0xd5, 0xe5, // PMULLW XMM4, XMM5
        0x66, 0x0f, 0xd5, 0xfe, // PMULLW XMM7, XMM6
        0x66, 0x45, 0x0f, 0xd5, 0xc1, // PMULLW XMM8, XMM9
        0x66, 0x45, 0x0f, 0xd5, 0xda, // PMULLW XMM11, XMM10
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_self_multiply() {
    let code = [
        0x66, 0x0f, 0xd5, 0xc0, // PMULLW XMM0, XMM0
        0x66, 0x0f, 0xd5, 0xc9, // PMULLW XMM1, XMM1
        0x66, 0x0f, 0xd5, 0xd2, // PMULLW XMM2, XMM2
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_extended_regs() {
    let code = [
        0x66, 0x45, 0x0f, 0xd5, 0xc8, // PMULLW XMM9, XMM8
        0x66, 0x45, 0x0f, 0xd5, 0xda, // PMULLW XMM11, XMM10
        0x66, 0x45, 0x0f, 0xd5, 0xec, // PMULLW XMM13, XMM12
        0x66, 0x45, 0x0f, 0xd5, 0xfe, // PMULLW XMM15, XMM14
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_cross_registers() {
    let code = [
        0x66, 0x0f, 0xd5, 0xc7, // PMULLW XMM0, XMM7
        0x66, 0x44, 0x0f, 0xd5, 0xc7, // PMULLW XMM8, XMM7
        0x66, 0x41, 0x0f, 0xd5, 0xc7, // PMULLW XMM0, XMM15
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmullw_mmx_all_regs() {
    let code = [
        0x0f, 0xd5, 0xc1, // PMULLW MM0, MM1
        0x0f, 0xd5, 0xca, // PMULLW MM1, MM2
        0x0f, 0xd5, 0xd3, // PMULLW MM2, MM3
        0x0f, 0xd5, 0xdc, // PMULLW MM3, MM4
        0x0f, 0xd5, 0xe5, // PMULLW MM4, MM5
        0x0f, 0xd5, 0xee, // PMULLW MM5, MM6
        0x0f, 0xd5, 0xf7, // PMULLW MM6, MM7
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PMULLW multiplies 8 packed words and keeps the LOW 16 bits of each product
// (identical for signed/unsigned). Computed by hand.
//   DST = XMM0 = 0x0002000300040005FFFF8000007FABCD
//   SRC = XMM1 = 0x0003000500070009000280017FFF1234
// ============================================================================

#[test]
fn kat_pmullw_value() {
    // PMULLW XMM0, XMM1 (66 0F D5 C1)
    let code = [0x66, 0x0f, 0xd5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x0002000300040005FFFF8000007FABCD);
    set_xmm(&mem, &mut vcpu, 1, 0x0003000500070009000280017FFF1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x0006000f001c002dfffe80007f814fa4,
        "PMULLW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pmullw_simple() {
    // Each word of DST multiplied by 2: word0=8*2=0x10, word1=7*2=0xE, ...
    let code = [0x66, 0x0f, 0xd5, 0xc1, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x00010002000300040005000600070008);
    set_xmm(&mem, &mut vcpu, 1, 0x00020002000200020002000200020002);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x0002000400060008000a000c000e0010);
}
