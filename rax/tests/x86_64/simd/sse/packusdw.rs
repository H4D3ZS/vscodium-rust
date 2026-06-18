use crate::common::{run_until_hlt, setup_vm};

// PACKUSDW - Pack with Unsigned Saturation (Dwords to Words)
//
// Converts 4 signed dword integers (32-bit) from dest and 4 from src
// into 8 unsigned word integers (16-bit) with unsigned saturation.
// Range: 0 to 65535 (0xFFFF)
// Negative values -> 0, values > 65535 -> 65535
//
// Opcode:
// 66 0F 38 2B /r    PACKUSDW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_packusdw_xmm0_xmm1_basic() {
    // PACKUSDW XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm2_basic() {
    // PACKUSDW XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xca, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm2_xmm3_basic() {
    // PACKUSDW XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xd3, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm3_xmm4_basic() {
    // PACKUSDW XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xdc, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm5_basic() {
    // PACKUSDW XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xe5, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm5_xmm6_basic() {
    // PACKUSDW XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xee, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm6_xmm7_basic() {
    // PACKUSDW XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xf7, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm7_xmm0_basic() {
    // PACKUSDW XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xf8, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm9() {
    // PACKUSDW XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm9_xmm10() {
    // PACKUSDW XMM9, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xca, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm11() {
    // PACKUSDW XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm11_xmm12() {
    // PACKUSDW XMM11, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xdc, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm12_xmm13() {
    // PACKUSDW XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xe5, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm13_xmm14() {
    // PACKUSDW XMM13, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xee, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm15() {
    // PACKUSDW XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf7, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm15_xmm8() {
    // PACKUSDW XMM15, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf8, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm0_xmm8() {
    // PACKUSDW XMM0, XMM8
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm9() {
    // PACKUSDW XMM1, XMM9
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm2_xmm10() {
    // PACKUSDW XMM2, XMM10
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm3_xmm11() {
    // PACKUSDW XMM3, XMM11
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm12() {
    // PACKUSDW XMM4, XMM12
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm5_xmm13() {
    // PACKUSDW XMM5, XMM13
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm6_xmm14() {
    // PACKUSDW XMM6, XMM14
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm7_xmm15() {
    // PACKUSDW XMM7, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm0() {
    // PACKUSDW XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm9_xmm1() {
    // PACKUSDW XMM9, XMM1
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc9, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm2() {
    // PACKUSDW XMM10, XMM2
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xd2, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm11_xmm3() {
    // PACKUSDW XMM11, XMM3
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xdb, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm12_xmm4() {
    // PACKUSDW XMM12, XMM4
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xe4, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm13_xmm5() {
    // PACKUSDW XMM13, XMM5
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xed, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm6() {
    // PACKUSDW XMM14, XMM6
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xf6, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm15_xmm7() {
    // PACKUSDW XMM15, XMM7
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xff, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm0_xmm0_same() {
    // PACKUSDW XMM0, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm8_same() {
    // PACKUSDW XMM8, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc0, 0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_sequential() {
    // Sequential pack operations
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM4, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_chain() {
    // Chained pack operations
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd0, // PACKUSDW XMM2, XMM0
        0x66, 0x0f, 0x38, 0x2b, 0xda, // PACKUSDW XMM3, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_all_high_regs() {
    // All high registers
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM8, XMM9
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM10, XMM11
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM12, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_cross_boundary() {
    // Cross boundary operations
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc7, // PACKUSDW XMM8, XMM7
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xf8, // PACKUSDW XMM7, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_alternating() {
    // Alternating low/high registers
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM8, XMM9
        0x66, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM2, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_packusdw_bidirectional() {
    // Bidirectional pattern
    let code = [
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xc8, // PACKUSDW XMM1, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register via set_xmm/get_xmm)
//
// PACKUSDW packs 4 signed dwords from DST (low result) and 4 from SRC (high
// result) into 8 UNSIGNED-saturated words. Negative -> 0, >0xFFFF -> 0xFFFF.
// ============================================================================

#[test]
fn kat_packusdw_value() {
    // PACKUSDW XMM0, XMM1 (66 0F 38 2B C1)
    // DST dwords: 0x12, -1, 0x10000, 5  -> 0x12, 0, 0xFFFF, 5
    // SRC dwords: 0, 0x7FFFFFFF, 0xFFFF, 0x8000 -> 0, 0xFFFF, 0xFFFF, 0x8000
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00000005_00010000_FFFFFFFF_00000012);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00008000_0000FFFF_7FFFFFFF_00000000);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x8000FFFFFFFF0000_0005FFFF00000012,
        "PACKUSDW got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
