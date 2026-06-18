use crate::common::{run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// PMOVMSKB - Move Byte Mask
//
// Creates a mask made up of the most significant bit of each byte in the
// source operand and stores the result in the low byte or word of the
// destination operand (depending on operand size).
//
// Opcodes:
// 66 0F D7 /r             PMOVMSKB r32, xmm1    - Move byte mask from XMM to r32
// 66 REX.W 0F D7 /r       PMOVMSKB r64, xmm1    - Move byte mask from XMM to r64

// ============================================================================
// PMOVMSKB Tests - Move Byte Mask (XMM -> GPR)
// ============================================================================

#[test]
fn test_pmovmskb_eax_xmm0() {
    // PMOVMSKB EAX, XMM0
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_ebx_xmm1() {
    // PMOVMSKB EBX, XMM1
    let code = [
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_ecx_xmm2() {
    // PMOVMSKB ECX, XMM2
    let code = [
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_edx_xmm3() {
    // PMOVMSKB EDX, XMM3
    let code = [
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_esi_xmm4() {
    // PMOVMSKB ESI, XMM4
    let code = [
        0x66, 0x0f, 0xd7, 0xf4, // PMOVMSKB ESI, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_edi_xmm5() {
    // PMOVMSKB EDI, XMM5
    let code = [
        0x66, 0x0f, 0xd7, 0xfd, // PMOVMSKB EDI, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_ebp_xmm6() {
    // PMOVMSKB EBP, XMM6
    let code = [
        0x66, 0x0f, 0xd7, 0xee, // PMOVMSKB EBP, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_eax_xmm7() {
    // PMOVMSKB EAX, XMM7
    let code = [
        0x66, 0x0f, 0xd7, 0xc7, // PMOVMSKB EAX, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r8d_xmm8() {
    // PMOVMSKB R8D, XMM8 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xc0, // PMOVMSKB R8D, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r9d_xmm9() {
    // PMOVMSKB R9D, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xc9, // PMOVMSKB R9D, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r10d_xmm10() {
    // PMOVMSKB R10D, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xd2, // PMOVMSKB R10D, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r11d_xmm11() {
    // PMOVMSKB R11D, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xdb, // PMOVMSKB R11D, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r12d_xmm12() {
    // PMOVMSKB R12D, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xe4, // PMOVMSKB R12D, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r13d_xmm13() {
    // PMOVMSKB R13D, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xed, // PMOVMSKB R13D, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r14d_xmm14() {
    // PMOVMSKB R14D, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xf6, // PMOVMSKB R14D, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r15d_xmm15() {
    // PMOVMSKB R15D, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xff, // PMOVMSKB R15D, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rax_xmm0() {
    // PMOVMSKB RAX, XMM0 (64-bit mode with REX.W)
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xc0, // PMOVMSKB RAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rbx_xmm1() {
    // PMOVMSKB RBX, XMM1
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xd9, // PMOVMSKB RBX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rcx_xmm2() {
    // PMOVMSKB RCX, XMM2
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xca, // PMOVMSKB RCX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rdx_xmm3() {
    // PMOVMSKB RDX, XMM3
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xd3, // PMOVMSKB RDX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rsi_xmm4() {
    // PMOVMSKB RSI, XMM4
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xf4, // PMOVMSKB RSI, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rdi_xmm5() {
    // PMOVMSKB RDI, XMM5
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xfd, // PMOVMSKB RDI, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_rbp_xmm6() {
    // PMOVMSKB RBP, XMM6
    let code = [
        0x66, 0x48, 0x0f, 0xd7, 0xee, // PMOVMSKB RBP, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r8_xmm7() {
    // PMOVMSKB R8, XMM7
    let code = [
        0x66, 0x4c, 0x0f, 0xd7, 0xc7, // PMOVMSKB R8, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r9_xmm8() {
    // PMOVMSKB R9, XMM8
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xc8, // PMOVMSKB R9, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r10_xmm9() {
    // PMOVMSKB R10, XMM9
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xd1, // PMOVMSKB R10, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r11_xmm10() {
    // PMOVMSKB R11, XMM10
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xda, // PMOVMSKB R11, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r12_xmm11() {
    // PMOVMSKB R12, XMM11
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xe3, // PMOVMSKB R12, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r13_xmm12() {
    // PMOVMSKB R13, XMM12
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xec, // PMOVMSKB R13, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r14_xmm13() {
    // PMOVMSKB R14, XMM13
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xf5, // PMOVMSKB R14, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_r15_xmm14() {
    // PMOVMSKB R15, XMM14
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xfe, // PMOVMSKB R15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_all_bits_set() {
    // Test with all MSBs set
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_all_bits_clear() {
    // Test with all MSBs clear
    let code = [
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_alternating_bits() {
    // Test with alternating MSBs
    let code = [
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_first_half_set() {
    // Test with first half of MSBs set
    let code = [
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_second_half_set() {
    // Test with second half of MSBs set
    let code = [
        0x66, 0x0f, 0xd7, 0xf4, // PMOVMSKB ESI, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_single_bit() {
    // Test with single MSB set
    let code = [
        0x66, 0x0f, 0xd7, 0xfd, // PMOVMSKB EDI, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_sequential_ops() {
    // Test sequential PMOVMSKB operations
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_mixed_registers_1() {
    // Test with mixed register combinations
    let code = [
        0x66, 0x44, 0x0f, 0xd7, 0xc1, // PMOVMSKB R8D, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_mixed_registers_2() {
    // Test with different mixed combinations
    let code = [
        0x66, 0x0f, 0xd7, 0xc7, // PMOVMSKB EAX, XMM7
        0x66, 0x45, 0x0f, 0xd7, 0xc8, // PMOVMSKB R9D, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_pattern_1() {
    // Test specific bit pattern
    let code = [
        0x66, 0x0f, 0xd7, 0xee, // PMOVMSKB EBP, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_pattern_2() {
    // Test another bit pattern
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xdb, // PMOVMSKB R11D, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_pattern_3() {
    // Test yet another bit pattern
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xe4, // PMOVMSKB R12D, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_64bit_mode_1() {
    // Test 64-bit destination register
    let code = [
        0x66, 0x4c, 0x0f, 0xd7, 0xc7, // PMOVMSKB R8, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_64bit_mode_2() {
    // Test another 64-bit combination
    let code = [
        0x66, 0x4d, 0x0f, 0xd7, 0xfe, // PMOVMSKB R15, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_all_xmm_regs() {
    // Cycle through different XMM registers
    let code = [
        0x66, 0x0f, 0xd7, 0xc0, // PMOVMSKB EAX, XMM0
        0x66, 0x0f, 0xd7, 0xd9, // PMOVMSKB EBX, XMM1
        0x66, 0x0f, 0xd7, 0xca, // PMOVMSKB ECX, XMM2
        0x66, 0x0f, 0xd7, 0xd3, // PMOVMSKB EDX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pmovmskb_high_xmm_regs() {
    // Test with high numbered XMM registers
    let code = [
        0x66, 0x45, 0x0f, 0xd7, 0xff, // PMOVMSKB R15D, XMM15
        0x66, 0x45, 0x0f, 0xd7, 0xf6, // PMOVMSKB R14D, XMM14
        0x66, 0x45, 0x0f, 0xd7, 0xed, // PMOVMSKB R13D, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (XMM input via set_xmm, GPR result via regs.rax)
//
// PMOVMSKB collects the most-significant bit of each of the 16 source bytes
// into the low 16 bits of a GPR (byte i -> result bit i), zero-extended.
// Computed by hand.
// ============================================================================

#[test]
fn kat_pmovmskb_value() {
    // PMOVMSKB EAX, XMM0 (66 0F D7 C0)
    //   XMM0 = 0x8001FF00807FFF0180010204080F1011
    //   MSB-set bytes are at positions 7,9,11,13,15 => mask 0xAA80.
    let code = [0x66, 0x0f, 0xd7, 0xc0, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x8001FF00807FFF0180010204080F1011);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xaa80,
        "PMOVMSKB got {:#x}",
        regs.rax & 0xFFFF
    );
}

#[test]
fn kat_pmovmskb_all_msb_set() {
    // Every byte has its MSB set => all 16 mask bits set => 0xFFFF.
    let code = [0x66, 0x0f, 0xd7, 0xc0, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x80808080808080808080808080808080);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xffff);
}

#[test]
fn kat_pmovmskb_none_set() {
    // No byte has its MSB set (all <= 0x7F) => mask is 0.
    let code = [0x66, 0x0f, 0xd7, 0xc0, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 0, 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0);
}
