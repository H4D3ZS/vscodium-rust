use crate::common::{run_until_hlt, setup_vm};

// PHMINPOSUW Extended Tests - Additional comprehensive coverage
//
// Packed Horizontal Word Minimum with Index
// Opcode: 66 0F 38 41 /r

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_phminposuw_extended_xmm2_xmm4() {
    // PHMINPOSUW XMM2, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xd4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm3_xmm5() {
    // PHMINPOSUW XMM3, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xdd, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm4_xmm6() {
    // PHMINPOSUW XMM4, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xe6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm5_xmm7() {
    // PHMINPOSUW XMM5, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xef, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm6_xmm0() {
    // PHMINPOSUW XMM6, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xf0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm7_xmm1() {
    // PHMINPOSUW XMM7, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xf9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm0_xmm2() {
    // PHMINPOSUW XMM0, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm1_xmm3() {
    // PHMINPOSUW XMM1, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xcb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm8_xmm10() {
    // PHMINPOSUW XMM8, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xc2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm9_xmm12() {
    // PHMINPOSUW XMM9, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xcc, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm11_xmm14() {
    // PHMINPOSUW XMM11, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xde, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm13_xmm15() {
    // PHMINPOSUW XMM13, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xef, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm15_xmm8() {
    // PHMINPOSUW XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xf8, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm0_xmm8() {
    // PHMINPOSUW XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm1_xmm9() {
    // PHMINPOSUW XMM1, XMM9
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm2_xmm10() {
    // PHMINPOSUW XMM2, XMM10
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm3_xmm11() {
    // PHMINPOSUW XMM3, XMM11
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm4_xmm12() {
    // PHMINPOSUW XMM4, XMM12
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm5_xmm13() {
    // PHMINPOSUW XMM5, XMM13
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm6_xmm14() {
    // PHMINPOSUW XMM6, XMM14
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm7_xmm15() {
    // PHMINPOSUW XMM7, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm8_xmm0() {
    // PHMINPOSUW XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm9_xmm1() {
    // PHMINPOSUW XMM9, XMM1
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm10_xmm2() {
    // PHMINPOSUW XMM10, XMM2
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm11_xmm3() {
    // PHMINPOSUW XMM11, XMM3
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm12_xmm4() {
    // PHMINPOSUW XMM12, XMM4
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm13_xmm5() {
    // PHMINPOSUW XMM13, XMM5
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm14_xmm6() {
    // PHMINPOSUW XMM14, XMM6
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_xmm15_xmm7() {
    // PHMINPOSUW XMM15, XMM7
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_chain_pattern() {
    // Chain pattern
    let code = [
        0x66, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x41, 0xd0, // PHMINPOSUW XMM2, XMM0
        0x66, 0x0f, 0x38, 0x41, 0xda, // PHMINPOSUW XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_all_high_regs() {
    // All high regs
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xc1, // PHMINPOSUW XMM8, XMM9
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xd3, // PHMINPOSUW XMM10, XMM11
        0x66, 0x45, 0x0f, 0x38, 0x41, 0xe5, // PHMINPOSUW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_phminposuw_extended_bidirectional() {
    // Bidirectional pattern
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x41, 0xc7, // PHMINPOSUW XMM8, XMM7
        0x66, 0x41, 0x0f, 0x38, 0x41, 0xf8, // PHMINPOSUW XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
