use crate::common::{run_until_hlt, setup_vm};

// PEXTRW - Extract Word
// Opcode: 66 0F C5 /r ib          PEXTRW reg, xmm, imm8
//         66 0F 3A 15 /r ib       PEXTRW reg/m16, xmm, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// PEXTRW - Extract Word to Register (8 positions: 0-7)
// ============================================================================

#[test]
fn test_pextrw_eax_xmm0_pos0() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x00, 0xf4]; // PEXTRW EAX, XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos1() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x01, 0xf4]; // PEXTRW EAX, XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos2() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x02, 0xf4]; // PEXTRW EAX, XMM0, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos3() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x03, 0xf4]; // PEXTRW EAX, XMM0, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos4() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x04, 0xf4]; // PEXTRW EAX, XMM0, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos5() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x05, 0xf4]; // PEXTRW EAX, XMM0, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos6() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x06, 0xf4]; // PEXTRW EAX, XMM0, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm0_pos7() {
    let code = [0x66, 0x0f, 0xc5, 0xc0, 0x07, 0xf4]; // PEXTRW EAX, XMM0, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRW with different XMM registers
#[test]
fn test_pextrw_eax_xmm1_pos0() {
    let code = [0x66, 0x0f, 0xc5, 0xc1, 0x00, 0xf4]; // PEXTRW EAX, XMM1, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm1_pos3() {
    let code = [0x66, 0x0f, 0xc5, 0xc1, 0x03, 0xf4]; // PEXTRW EAX, XMM1, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm2_pos5() {
    let code = [0x66, 0x0f, 0xc5, 0xc2, 0x05, 0xf4]; // PEXTRW EAX, XMM2, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm3_pos7() {
    let code = [0x66, 0x0f, 0xc5, 0xc3, 0x07, 0xf4]; // PEXTRW EAX, XMM3, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm4_pos2() {
    let code = [0x66, 0x0f, 0xc5, 0xc4, 0x02, 0xf4]; // PEXTRW EAX, XMM4, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm5_pos4() {
    let code = [0x66, 0x0f, 0xc5, 0xc5, 0x04, 0xf4]; // PEXTRW EAX, XMM5, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm6_pos6() {
    let code = [0x66, 0x0f, 0xc5, 0xc6, 0x06, 0xf4]; // PEXTRW EAX, XMM6, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm7_pos1() {
    let code = [0x66, 0x0f, 0xc5, 0xc7, 0x01, 0xf4]; // PEXTRW EAX, XMM7, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRW with different GPRs
#[test]
fn test_pextrw_ebx_xmm0_pos3() {
    let code = [0x66, 0x0f, 0xc5, 0xd8, 0x03, 0xf4]; // PEXTRW EBX, XMM0, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_ecx_xmm1_pos5() {
    let code = [0x66, 0x0f, 0xc5, 0xc9, 0x05, 0xf4]; // PEXTRW ECX, XMM1, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_edx_xmm2_pos2() {
    let code = [0x66, 0x0f, 0xc5, 0xd2, 0x02, 0xf4]; // PEXTRW EDX, XMM2, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_esi_xmm3_pos7() {
    let code = [0x66, 0x0f, 0xc5, 0xf3, 0x07, 0xf4]; // PEXTRW ESI, XMM3, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_edi_xmm4_pos4() {
    let code = [0x66, 0x0f, 0xc5, 0xfc, 0x04, 0xf4]; // PEXTRW EDI, XMM4, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRW with extended XMM registers
#[test]
fn test_pextrw_eax_xmm8_pos0() {
    let code = [0x66, 0x44, 0x0f, 0xc5, 0xc0, 0x00, 0xf4]; // PEXTRW EAX, XMM8, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm9_pos3() {
    let code = [0x66, 0x44, 0x0f, 0xc5, 0xc8, 0x03, 0xf4]; // PEXTRW EAX, XMM9, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm10_pos5() {
    let code = [0x66, 0x44, 0x0f, 0xc5, 0xd0, 0x05, 0xf4]; // PEXTRW EAX, XMM10, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_eax_xmm15_pos7() {
    let code = [0x66, 0x44, 0x0f, 0xc5, 0xf8, 0x07, 0xf4]; // PEXTRW EAX, XMM15, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRW with extended GPRs
#[test]
fn test_pextrw_r8d_xmm0_pos1() {
    let code = [0x66, 0x41, 0x0f, 0xc5, 0xc0, 0x01, 0xf4]; // PEXTRW R8D, XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_r15d_xmm7_pos6() {
    let code = [0x66, 0x41, 0x0f, 0xc5, 0xff, 0x06, 0xf4]; // PEXTRW R15D, XMM7, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PEXTRW to memory - SSE4.1 variant (66 0F 3A 15)
// ============================================================================

#[test]
fn test_pextrw_mem_xmm0_pos0() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos1() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos2() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos3() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos4() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x04, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos5() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x05, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos6() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x06, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm0_pos7() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x07, 0xf4,
    ]; // PEXTRW [0x3000], XMM0, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm1_pos3() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4,
    ]; // PEXTRW [0x3000], XMM1, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm7_pos5() {
    let code = [
        0x66, 0x0f, 0x3a, 0x15, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x05, 0xf4,
    ]; // PEXTRW [0x3000], XMM7, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_mem_xmm15_pos7() {
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x15, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x07, 0xf4,
    ]; // PEXTRW [0x3000], XMM15, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Additional combinations with different registers
#[test]
fn test_pextrw_ebx_xmm7_pos6() {
    let code = [0x66, 0x0f, 0xc5, 0xdf, 0x06, 0xf4]; // PEXTRW EBX, XMM7, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_ecx_xmm6_pos4() {
    let code = [0x66, 0x0f, 0xc5, 0xce, 0x04, 0xf4]; // PEXTRW ECX, XMM6, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrw_edx_xmm5_pos2() {
    let code = [0x66, 0x0f, 0xc5, 0xd5, 0x02, 0xf4]; // PEXTRW EDX, XMM5, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (XMM source -> GP register via get_regs)
//
// PEXTRW (0F 3A 15 form) extracts the word at imm8[2:0], zero-extended.
// ============================================================================

#[test]
fn kat_pextrw_value() {
    // PEXTRW EAX, XMM0, 5 (66 0F 3A 15 C0 05): extract word 5.
    // words lane0..7 = 0x1100,0x3322,0x5544,0x7766,0x9988,0xBBAA,0xDDCC,0xFFEE
    let code = [0x66, 0x0f, 0x3a, 0x15, 0xc0, 0x05, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0xFFEEDDCCBBAA99887766554433221100);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        0xBBAA,
        "PEXTRW EAX = {:#x}",
        regs.rax
    );
}
