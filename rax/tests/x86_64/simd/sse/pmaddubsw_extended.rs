use crate::common::{run_until_hlt, setup_vm};

// PMADDUBSW Extended Tests - Additional comprehensive coverage
//
// Multiply and Add Packed Signed and Unsigned Bytes with Saturation
// Opcode: 66 0F 38 04 /r

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pmaddubsw_extended_xmm2_xmm4() {
    // PMADDUBSW XMM2, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xd4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm3_xmm5() {
    // PMADDUBSW XMM3, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xdd, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm4_xmm6() {
    // PMADDUBSW XMM4, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xe6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm5_xmm7() {
    // PMADDUBSW XMM5, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xef, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm6_xmm0() {
    // PMADDUBSW XMM6, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xf0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm7_xmm1() {
    // PMADDUBSW XMM7, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xf9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm0_xmm2() {
    // PMADDUBSW XMM0, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm1_xmm3() {
    // PMADDUBSW XMM1, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xcb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm8_xmm10() {
    // PMADDUBSW XMM8, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xc2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm9_xmm12() {
    // PMADDUBSW XMM9, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xcc, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm11_xmm14() {
    // PMADDUBSW XMM11, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xde, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm13_xmm15() {
    // PMADDUBSW XMM13, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xef, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm15_xmm8() {
    // PMADDUBSW XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xf8, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm0_xmm8() {
    // PMADDUBSW XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm1_xmm9() {
    // PMADDUBSW XMM1, XMM9
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm2_xmm10() {
    // PMADDUBSW XMM2, XMM10
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm3_xmm11() {
    // PMADDUBSW XMM3, XMM11
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm4_xmm12() {
    // PMADDUBSW XMM4, XMM12
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm5_xmm13() {
    // PMADDUBSW XMM5, XMM13
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm6_xmm14() {
    // PMADDUBSW XMM6, XMM14
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm7_xmm15() {
    // PMADDUBSW XMM7, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm8_xmm0() {
    // PMADDUBSW XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm9_xmm1() {
    // PMADDUBSW XMM9, XMM1
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm10_xmm2() {
    // PMADDUBSW XMM10, XMM2
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm11_xmm3() {
    // PMADDUBSW XMM11, XMM3
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm12_xmm4() {
    // PMADDUBSW XMM12, XMM4
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm13_xmm5() {
    // PMADDUBSW XMM13, XMM5
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm14_xmm6() {
    // PMADDUBSW XMM14, XMM6
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_xmm15_xmm7() {
    // PMADDUBSW XMM15, XMM7
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_chain_low() {
    // Chain pattern low regs
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x04, 0xd0, // PMADDUBSW XMM2, XMM0
        0x66, 0x0f, 0x38, 0x04, 0xda, // PMADDUBSW XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_chain_high() {
    // Chain pattern high regs
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM8, XMM9
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xd0, // PMADDUBSW XMM10, XMM8
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xda, // PMADDUBSW XMM11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_alternating() {
    // Alternating low/high regs
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0x66, 0x45, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM8, XMM9
        0x66, 0x0f, 0x38, 0x04, 0xd3, // PMADDUBSW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_cross_boundary_1() {
    // Cross boundary pattern 1
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xc7, // PMADDUBSW XMM8, XMM7
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xf8, // PMADDUBSW XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_cross_boundary_2() {
    // Cross boundary pattern 2
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x04, 0xce, // PMADDUBSW XMM9, XMM6
        0x66, 0x41, 0x0f, 0x38, 0x04, 0xf1, // PMADDUBSW XMM6, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmaddubsw_extended_all_to_xmm0() {
    // All to XMM0 pattern
    let code = [
        0x66, 0x0f, 0x38, 0x04, 0xc1, // PMADDUBSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x04, 0xc2, // PMADDUBSW XMM0, XMM2
        0x66, 0x0f, 0x38, 0x04, 0xc3, // PMADDUBSW XMM0, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
