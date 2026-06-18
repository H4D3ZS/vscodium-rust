use crate::common::{run_until_hlt, setup_vm};

// PEXTRB/PEXTRD/PEXTRQ - Extract Byte/Dword/Qword
// Opcode: 66 0F 3A 14 /r ib       PEXTRB r32/m8, xmm2, imm8
//         66 0F 3A 16 /r ib       PEXTRD r/m32, xmm2, imm8
//         66 REX.W 0F 3A 16 /r ib PEXTRQ r/m64, xmm2, imm8

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// PEXTRB - Extract Byte to Register (16 positions: 0-15)
// ============================================================================

#[test]
fn test_pextrb_eax_xmm0_pos0() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x00, 0xf4]; // PEXTRB EAX, XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos1() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x01, 0xf4]; // PEXTRB EAX, XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos2() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x02, 0xf4]; // PEXTRB EAX, XMM0, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos3() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x03, 0xf4]; // PEXTRB EAX, XMM0, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos4() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x04, 0xf4]; // PEXTRB EAX, XMM0, 4
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos5() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x05, 0xf4]; // PEXTRB EAX, XMM0, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos6() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x06, 0xf4]; // PEXTRB EAX, XMM0, 6
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos7() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x07, 0xf4]; // PEXTRB EAX, XMM0, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos8() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x08, 0xf4]; // PEXTRB EAX, XMM0, 8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos9() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x09, 0xf4]; // PEXTRB EAX, XMM0, 9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos10() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0a, 0xf4]; // PEXTRB EAX, XMM0, 10
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos11() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0b, 0xf4]; // PEXTRB EAX, XMM0, 11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos12() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0c, 0xf4]; // PEXTRB EAX, XMM0, 12
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos13() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0d, 0xf4]; // PEXTRB EAX, XMM0, 13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos14() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0e, 0xf4]; // PEXTRB EAX, XMM0, 14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm0_pos15() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x0f, 0xf4]; // PEXTRB EAX, XMM0, 15
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRB with different registers
#[test]
fn test_pextrb_ebx_xmm1_pos5() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xcb, 0x05, 0xf4]; // PEXTRB EBX, XMM1, 5
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_ecx_xmm2_pos7() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xd1, 0x07, 0xf4]; // PEXTRB ECX, XMM2, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_edx_xmm3_pos9() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xda, 0x09, 0xf4]; // PEXTRB EDX, XMM3, 9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_esi_xmm4_pos11() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xe6, 0x0b, 0xf4]; // PEXTRB ESI, XMM4, 11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_edi_xmm5_pos13() {
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xef, 0x0d, 0xf4]; // PEXTRB EDI, XMM5, 13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRB with extended XMM registers
#[test]
fn test_pextrb_eax_xmm8_pos3() {
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x14, 0xc0, 0x03, 0xf4]; // PEXTRB EAX, XMM8, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_eax_xmm15_pos7() {
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x14, 0xf8, 0x07, 0xf4]; // PEXTRB EAX, XMM15, 7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRB to memory
#[test]
fn test_pextrb_mem_xmm0_pos0() {
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4,
    ]; // PEXTRB [0x3000], XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_mem_xmm1_pos8() {
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x08, 0xf4,
    ]; // PEXTRB [0x3000], XMM1, 8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrb_mem_xmm7_pos15() {
    let code = [
        0x66, 0x0f, 0x3a, 0x14, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x0f, 0xf4,
    ]; // PEXTRB [0x3000], XMM7, 15
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PEXTRD - Extract Dword to Register (4 positions: 0-3)
// ============================================================================

#[test]
fn test_pextrd_eax_xmm0_pos0() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRD EAX, XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos1() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x01, 0xf4]; // PEXTRD EAX, XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos2() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x02, 0xf4]; // PEXTRD EAX, XMM0, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_eax_xmm0_pos3() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x03, 0xf4]; // PEXTRD EAX, XMM0, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRD with different registers
#[test]
fn test_pextrd_ebx_xmm1_pos0() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xcb, 0x00, 0xf4]; // PEXTRD EBX, XMM1, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_ecx_xmm2_pos1() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xd1, 0x01, 0xf4]; // PEXTRD ECX, XMM2, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_edx_xmm3_pos2() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xda, 0x02, 0xf4]; // PEXTRD EDX, XMM3, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_esi_xmm4_pos3() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xe6, 0x03, 0xf4]; // PEXTRD ESI, XMM4, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_edi_xmm5_pos1() {
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xef, 0x01, 0xf4]; // PEXTRD EDI, XMM5, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_r8d_xmm6_pos2() {
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x16, 0xf0, 0x02, 0xf4]; // PEXTRD R8D, XMM6, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_r15d_xmm7_pos3() {
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x16, 0xff, 0x03, 0xf4]; // PEXTRD R15D, XMM7, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRD with extended XMM registers
#[test]
fn test_pextrd_eax_xmm8_pos0() {
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRD EAX, XMM8, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_eax_xmm15_pos2() {
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x16, 0xf8, 0x02, 0xf4]; // PEXTRD EAX, XMM15, 2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRD to memory
#[test]
fn test_pextrd_mem_xmm0_pos0() {
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4,
    ]; // PEXTRD [0x3000], XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_mem_xmm1_pos1() {
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4,
    ]; // PEXTRD [0x3000], XMM1, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrd_mem_xmm7_pos3() {
    let code = [
        0x66, 0x0f, 0x3a, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, 0xf4,
    ]; // PEXTRD [0x3000], XMM7, 3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// PEXTRQ - Extract Qword to Register (2 positions: 0-1)
// ============================================================================

#[test]
fn test_pextrq_rax_xmm0_pos0() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRQ RAX, XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rax_xmm0_pos1() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xc0, 0x01, 0xf4]; // PEXTRQ RAX, XMM0, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRQ with different registers
#[test]
fn test_pextrq_rbx_xmm1_pos0() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xcb, 0x00, 0xf4]; // PEXTRQ RBX, XMM1, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rcx_xmm2_pos1() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xd1, 0x01, 0xf4]; // PEXTRQ RCX, XMM2, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rdx_xmm3_pos0() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xda, 0x00, 0xf4]; // PEXTRQ RDX, XMM3, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rsi_xmm4_pos1() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xe6, 0x01, 0xf4]; // PEXTRQ RSI, XMM4, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rdi_xmm5_pos0() {
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xef, 0x00, 0xf4]; // PEXTRQ RDI, XMM5, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_r8_xmm6_pos1() {
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x16, 0xf0, 0x01, 0xf4]; // PEXTRQ R8, XMM6, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_r15_xmm7_pos0() {
    let code = [0x66, 0x49, 0x0f, 0x3a, 0x16, 0xff, 0x00, 0xf4]; // PEXTRQ R15, XMM7, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRQ with extended XMM registers
#[test]
fn test_pextrq_rax_xmm8_pos0() {
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x16, 0xc0, 0x00, 0xf4]; // PEXTRQ RAX, XMM8, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_rax_xmm15_pos1() {
    let code = [0x66, 0x4c, 0x0f, 0x3a, 0x16, 0xf8, 0x01, 0xf4]; // PEXTRQ RAX, XMM15, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_r8_xmm9_pos0() {
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x16, 0xc8, 0x00, 0xf4]; // PEXTRQ R8, XMM9, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_r15_xmm15_pos1() {
    let code = [0x66, 0x4d, 0x0f, 0x3a, 0x16, 0xff, 0x01, 0xf4]; // PEXTRQ R15, XMM15, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// PEXTRQ to memory
#[test]
fn test_pextrq_mem_xmm0_pos0() {
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4,
    ]; // PEXTRQ [0x3000], XMM0, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_mem_xmm1_pos1() {
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4,
    ]; // PEXTRQ [0x3000], XMM1, 1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pextrq_mem_xmm7_pos0() {
    let code = [
        0x66, 0x48, 0x0f, 0x3a, 0x16, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4,
    ]; // PEXTRQ [0x3000], XMM7, 0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (XMM source -> GP register via get_regs)
// ============================================================================

#[test]
fn kat_pextrb_value() {
    // PEXTRB EAX, XMM0, 5 (66 0F 3A 14 C0 05): extract byte 5, zero-extended.
    // XMM0 byte i = 0x10 + i, so byte5 = 0x15.
    let code = [0x66, 0x0f, 0x3a, 0x14, 0xc0, 0x05, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x1F1E1D1C1B1A19181716151413121110);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0x15, "PEXTRB EAX = {:#x}", regs.rax);
}

#[test]
fn kat_pextrd_value() {
    // PEXTRD EAX, XMM0, 3 (66 0F 3A 16 C0 03): extract dword 3.
    let code = [0x66, 0x0f, 0x3a, 0x16, 0xc0, 0x03, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x44444444_33333333_22222222_11111111);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        0x44444444,
        "PEXTRD EAX = {:#x}",
        regs.rax
    );
}

#[test]
fn kat_pextrq_value() {
    // PEXTRQ RAX, XMM0, 1 (66 REX.W 0F 3A 16 C0 01): extract qword 1.
    let code = [0x66, 0x48, 0x0f, 0x3a, 0x16, 0xc0, 0x01, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x1122334455667788_99AABBCCDDEEFF00);
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1122334455667788, "PEXTRQ RAX = {:#x}", regs.rax);
}
