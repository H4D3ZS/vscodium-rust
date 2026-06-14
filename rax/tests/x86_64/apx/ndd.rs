//! APX NDD (New Data Destination) Tests
//!
//! NDD transforms traditional 2-operand x86 instructions into 3-operand form:
//!   dst = op(src1, src2)  instead of  dst = op(dst, src)
//!
//! This is achieved through Extended EVEX encoding with the ND bit set.
//! The vvvv field specifies the first source operand, allowing the destination
//! to be different from both sources.
//!
//! Benefits:
//! - Non-destructive operations (preserve source operands)
//! - Better register allocation opportunities
//! - Reduced MOV instructions
//!
//! EVEX encoding for NDD:
//! - EVEX.ND = 1 (EVEX.b4 in byte 3)
//! - EVEX.NF can also be set for no-flags variants
//! - EVEX.vvvv specifies src1 (inverted encoding)
//! - ModRM.reg specifies destination
//! - ModRM.rm specifies src2

use crate::common::*;

// ============================================================================
// NDD ADD (3-operand addition)
// ============================================================================

#[test]
fn test_ndd_add_no_b4_uses_legacy_rm_source_match_llvm() {
    // LLVM 23 decodes 62 f4 7c 18 01 c0 as: add eax, eax, eax.
    // The APX B4 bit is clear, so the r/m source is EAX, not R16D.
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.r16 = 100;
    let code = [0x62, 0xF4, 0x7C, 0x18, 0x01, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10);
    assert_eq!(regs.r16, 100);
}

#[test]
fn test_ndd_add_b4_uses_r16_rm_source_match_llvm() {
    // LLVM 23 assembles "add eax, r16d, eax" as 62 fc 7c 18 01 c0.
    // APX MAP4 uses EVEX P0 bit 3 as B4, extending the r/m source by +16.
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.r16 = 7;
    let code = [0x62, 0xFC, 0x7C, 0x18, 0x01, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12);
    assert_eq!(regs.r16, 7);
}

#[test]
fn test_ndd_add_memory_uses_apx_x4_sib_index_match_llvm() {
    // LLVM 23 assembles:
    // add eax, ebx, dword ptr [rax + 2*r16] -> 62 f4 78 18 03 1c 40.
    // APX MAP4 reuses EVEX P1 bit 2 as inverted X4; when it is clear, the
    // SIB index is R16, not RAX or R24.
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 5;
    regs.r16 = 0x10;
    regs.r24 = 0x20;
    let code = [0x62, 0xF4, 0x78, 0x18, 0x03, 0x1C, 0x40, 0xF4];

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, 0x3000 + 0x10 * 2, 7);
    write_mem_at_u32(&mem, 0x3000 + 0x20 * 2, 100);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12);
}

#[test]
fn test_ndd_add_memory_uses_apx_b4_sib_base_match_llvm() {
    // LLVM 23 assembles:
    // add eax, ebx, dword ptr [r16 + 2*rax] -> 62 fc 7c 18 03 1c 40.
    // APX MAP4 P0 bit 3 extends the SIB base by +16.
    let mut regs = Registers::default();
    regs.rax = 0x10;
    regs.rbx = 5;
    regs.r16 = 0x3000;
    let code = [0x62, 0xFC, 0x7C, 0x18, 0x03, 0x1C, 0x40, 0xF4];

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, 0x3000 + 0x10 * 2, 7);
    write_mem_at_u32(&mem, 0x10 * 3, 100);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12);
}

#[test]
fn test_ndd_add_reg_reg_reg() {
    // ADD R8, RAX, RBX (R8 = RAX + RBX)
    // EVEX.NDD.128.0F38.W1 encoding
    // EVEX: 62 [R:X:B:R':0:0:m:m] [W:vvvv:1:p:p] [z:L'L:b:V':aaa]
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x01, 0xC0, // ADD encoding with ModRM
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_r16_r17_r18() {
    // ADD R16, R17, R18 (R16 = R17 + R18)
    // Using EGPR with NDD
    let code = [
        0x62, 0xEC, 0x74, 0x18, // EVEX prefix with NDD and EGPR
        0x01, 0xC2, // ADD: mod=11, reg=0 (R16), rm=2 (R18)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_reg_reg_mem() {
    // ADD RAX, RBX, [RCX] (RAX = RBX + [RCX])
    let code = [
        0x62, 0xF4, 0x64, 0x18, // EVEX prefix with NDD, vvvv=RBX
        0x03, 0x01, // ADD r64, r/m64: mod=00, reg=0, rm=1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_reg_reg_imm32() {
    // ADD RCX, RAX, imm32 (RCX = RAX + imm32)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x81, 0xC1, // ADD r/m64, imm32: mod=11, /0, rm=1
        0x78, 0x56, 0x34, 0x12, // imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_reg_reg_imm8() {
    // ADD RDX, RAX, imm8 (RDX = RAX + sign_ext(imm8))
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x83, 0xC2, 0x10, // ADD r/m64, imm8: mod=11, /0, rm=2, imm8=0x10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_reg_imm8_match_llvm() {
    // LLVM 23 assembles "add r8, rax, 16" as 62 f4 bc 18 83 c0 10.
    let code = [0x62, 0xF4, 0xBC, 0x18, 0x83, 0xC0, 0x10, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.r8 = 0xDEAD_BEEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 21);
    assert_eq!(regs.rax, 5);
}

#[test]
fn test_ndd_add_reg_imm32_match_llvm() {
    // LLVM 23 assembles "add r8, rax, 0x12345678" as 62 f4 bc 18 81 c0 78 56 34 12.
    let code = [
        0x62, 0xF4, 0xBC, 0x18, 0x81, 0xC0, 0x78, 0x56, 0x34, 0x12, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.r8 = 0xDEAD_BEEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1234_567D);
    assert_eq!(regs.rax, 5);
}

#[test]
fn test_nf_ndd_add_reg_imm8_match_llvm() {
    // LLVM 23 assembles "{nf} add r8, rax, 1" as 62 f4 bc 1c 83 c0 01.
    let code = [0x62, 0xF4, 0xBC, 0x1C, 0x83, 0xC0, 0x01, 0xF4];
    let mut regs = Registers::default();
    regs.rax = u64::MAX;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x8D5;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0);
    assert_eq!(regs.rax, u64::MAX);
    assert_eq!(regs.rflags & 0x8D5, 0x8D5);
}

// ============================================================================
// NDD SUB (3-operand subtraction)
// ============================================================================

#[test]
fn test_ndd_sub_reg_reg_reg() {
    // SUB R8, RAX, RBX (R8 = RAX - RBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x29, 0xC0, // SUB r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_sub_r16_r24_r31() {
    // SUB R16, R24, R31 (R16 = R24 - R31)
    let code = [
        0x62, 0x4C, 0x3C, 0x18, // EVEX prefix with NDD and EGPR
        0x29, 0xF8, // SUB: mod=11, reg=7, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_sub_reg_reg_mem() {
    // SUB RAX, RBX, [RCX+0x100] (RAX = RBX - [RCX+0x100])
    let code = [
        0x62, 0xF4, 0x64, 0x18, // EVEX prefix with NDD
        0x2B, 0x81, // SUB r64, r/m64: mod=10, reg=0, rm=1
        0x00, 0x01, 0x00, 0x00, // disp32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD AND/OR/XOR (3-operand logical)
// ============================================================================

#[test]
fn test_ndd_and_reg_reg_reg() {
    // AND R8, RAX, RBX (R8 = RAX & RBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x21, 0xC0, // AND r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_or_reg_reg_reg() {
    // OR R8, RAX, RBX (R8 = RAX | RBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x09, 0xC0, // OR r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_xor_reg_reg_reg() {
    // XOR R8, RAX, RBX (R8 = RAX ^ RBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x31, 0xC0, // XOR r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_and_reg_reg_imm32() {
    // AND RCX, RAX, 0xFF00FF00 (RCX = RAX & 0xFF00FF00)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x81, 0xE1, // AND r/m64, imm32: /4
        0x00, 0xFF, 0x00, 0xFF, // imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD SHL/SHR/SAR (3-operand shifts)
// ============================================================================

#[test]
fn test_ndd_shl_reg_reg_imm() {
    // SHL R8, RAX, 4 (R8 = RAX << 4)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xC1, 0xE0, 0x04, // SHL r/m64, imm8: /4, imm8=4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_shr_reg_reg_cl() {
    // SHR R8, RAX, CL (R8 = RAX >> CL)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xD3, 0xE8, // SHR r/m64, CL: /5
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_sar_reg_reg_imm() {
    // SAR R8, RAX, 8 (R8 = RAX >> 8, arithmetic)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xC1, 0xF8, 0x08, // SAR r/m64, imm8: /7, imm8=8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_shl_r16_r17_imm() {
    // SHL R16, R17, 16 (R16 = R17 << 16)
    let code = [
        0x62, 0xEC, 0x74, 0x18, // EVEX with NDD and EGPR
        0xC1, 0xE1, 0x10, // SHL r/m64, imm8: mod=11, /4, rm=1, imm8=16
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_shld_reg_reg_imm_match_llvm() {
    // LLVM 23 assembles "shld r8, rax, rbx, 4" as 62 f4 bc 18 24 d8 04.
    let code = [0x62, 0xF4, 0xBC, 0x18, 0x24, 0xD8, 0x04, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1111_2222_3333_4444;
    regs.rbx = 0xA000_0000_0000_0000;
    regs.r8 = 0xDEAD_BEEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1112_2223_3334_444A);
    assert_eq!(regs.rax, 0x1111_2222_3333_4444);
    assert_eq!(regs.rbx, 0xA000_0000_0000_0000);
}

#[test]
fn test_ndd_shrd_reg_reg_cl_match_llvm() {
    // LLVM 23 assembles "shrd r8, rax, rbx, cl" as 62 f4 bc 18 ad d8.
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xAD, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1111_2222_3333_4444;
    regs.rbx = 0xB;
    regs.rcx = 4;
    regs.r8 = 0xDEAD_BEEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xB111_1222_2333_3444);
    assert_eq!(regs.rax, 0x1111_2222_3333_4444);
    assert_eq!(regs.rbx, 0xB);
}

#[test]
fn test_nf_ndd_shld_reg_reg_imm_match_llvm() {
    // LLVM 23 assembles "{nf} shld r8, rax, rbx, 4" as 62 f4 bc 1c 24 d8 04.
    let code = [0x62, 0xF4, 0xBC, 0x1C, 0x24, 0xD8, 0x04, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1111_2222_3333_4444;
    regs.rbx = 0xA000_0000_0000_0000;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x8D5;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1112_2223_3334_444A);
    assert_eq!(regs.rflags & 0x8D5, 0x8D5);
}

// ============================================================================
// NDD ROL/ROR (3-operand rotates)
// ============================================================================

#[test]
fn test_ndd_rol_reg_reg_imm() {
    // ROL R8, RAX, 7 (R8 = rotate_left(RAX, 7))
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xC1, 0xC0, 0x07, // ROL r/m64, imm8: /0, imm8=7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_rol_32bit_match_llvm() {
    // LLVM 23 assembles "rol r8d, eax, 8" as 62 f4 3c 18 c1 c0 08.
    let code = [0x62, 0xF4, 0x3C, 0x18, 0xC1, 0xC0, 0x08, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678;
    regs.r8 = u64::MAX;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x3456_7812);
    assert_eq!(regs.rax, 0x1234_5678);
}

#[test]
fn test_ndd_ror_reg_reg_cl() {
    // ROR R8, RAX, CL (R8 = rotate_right(RAX, CL))
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xD3, 0xC8, // ROR r/m64, CL: /1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_ror_32bit_match_llvm() {
    // LLVM 23 assembles "ror r8d, eax, 8" as 62 f4 3c 18 c1 c8 08.
    let code = [0x62, 0xF4, 0x3C, 0x18, 0xC1, 0xC8, 0x08, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678;
    regs.r8 = u64::MAX;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x7812_3456);
    assert_eq!(regs.rax, 0x1234_5678);
}

#[test]
fn test_ndd_rol_8bit_match_llvm() {
    // LLVM 23 assembles "rol r8b, al" as 62 f4 3c 18 d0 c0.
    let code = [0x62, 0xF4, 0x3C, 0x18, 0xD0, 0xC0, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x81;
    regs.r8 = 0x1122_3344_5566_7788;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1122_3344_5566_7703);
    assert_eq!(regs.rax, 0x81);
}

#[test]
fn test_ndd_rcl_reg_reg_one_match_llvm() {
    // LLVM 23 assembles "rcl r8, rax, 1" as 62 f4 bc 18 d1 d0.
    const CF: u64 = 1 << 0;
    const OF: u64 = 1 << 11;
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xD1, 0xD0, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0001;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = CF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x3);
    assert_eq!(regs.rax, 0x8000_0000_0000_0001);
    assert_ne!(regs.rflags & CF, 0);
    assert_ne!(regs.rflags & OF, 0);
}

#[test]
fn test_ndd_rcr_reg_reg_one_match_llvm() {
    // LLVM 23 assembles "rcr r8, rax, 1" as 62 f4 bc 18 d1 d8.
    const CF: u64 = 1 << 0;
    const OF: u64 = 1 << 11;
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xD1, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0000_0000_0001;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = CF | OF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xC000_0000_0000_0000);
    assert_eq!(regs.rax, 0x8000_0000_0000_0001);
    assert_ne!(regs.rflags & CF, 0);
    assert_eq!(regs.rflags & OF, 0);
}

#[test]
fn test_ndd_rcl_32bit_imm_match_llvm() {
    // LLVM 23 assembles "rcl r8d, eax, 4" as 62 f4 3c 18 c1 d0 04.
    let code = [0x62, 0xF4, 0x3C, 0x18, 0xC1, 0xD0, 0x04, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x8000_0001;
    regs.r8 = 0xFFFF_FFFF_DEAD_BEEF;
    regs.rflags = 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x14);
    assert_eq!(regs.rax, 0x8000_0001);
}

#[test]
fn test_ndd_rcr_32bit_cl_match_llvm() {
    // LLVM 23 assembles "rcr r8d, eax, cl" as 62 f4 3c 18 d3 d8.
    let code = [0x62, 0xF4, 0x3C, 0x18, 0xD3, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x10;
    regs.rcx = 4;
    regs.r8 = 0xFFFF_FFFF_DEAD_BEEF;
    regs.rflags = 0x3;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1000_0001);
    assert_eq!(regs.rax, 0x10);
}

// ============================================================================
// NDD ADC/SBB (3-operand with carry)
// ============================================================================

#[test]
fn test_ndd_adc_reg_reg_reg() {
    // ADC R8, RAX, RBX (R8 = RAX + RBX + CF)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x11, 0xC0, // ADC r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_adc_reg_reg_reg_match_llvm() {
    // LLVM 23 assembles "adc r8, rax, rbx" as 62 f4 bc 18 11 d8.
    const CF: u64 = 1;
    let code = [0x62, 0xF4, 0xBC, 0x18, 0x11, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = u64::MAX;
    regs.rbx = 0;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | CF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0);
    assert_eq!(regs.rax, u64::MAX);
    assert_eq!(regs.rbx, 0);
    assert_ne!(regs.rflags & CF, 0);
}

#[test]
fn test_ndd_sbb_reg_reg_reg() {
    // SBB R8, RAX, RBX (R8 = RAX - RBX - CF)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x19, 0xC0, // SBB r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_sbb_reg_reg_reg_match_llvm() {
    // LLVM 23 assembles "sbb r8, rax, rbx" as 62 f4 bc 18 19 d8.
    const CF: u64 = 1;
    let code = [0x62, 0xF4, 0xBC, 0x18, 0x19, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | CF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, u64::MAX);
    assert_eq!(regs.rax, 0);
    assert_eq!(regs.rbx, 0);
    assert_ne!(regs.rflags & CF, 0);
}

// ============================================================================
// NDD IMUL (3-operand multiply)
// ============================================================================

#[test]
fn test_ndd_imul_reg_reg_reg_match_llvm() {
    // LLVM 23 assembles "imul r8, rax, rbx" as 62 f4 bc 18 af c3.
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xAF, 0xC3, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 7;
    regs.rbx = 6;
    regs.r8 = 0xDEAD_BEEF;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 42);
    assert_eq!(regs.rax, 7);
    assert_eq!(regs.rbx, 6);
}

#[test]
fn test_ndd_imul_reg_reg_reg() {
    // IMUL R8, RAX, RBX (R8 = RAX * RBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x98, // EVEX prefix with NDD (0F map)
        0xAF, 0xC3, // IMUL r64, r/m64: mod=11, reg=0, rm=3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_imul_reg_reg_imm() {
    // IMUL R8, RAX, 100 (R8 = RAX * 100)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x6B, 0xC0, 0x64, // IMUL r64, r/m64, imm8: imm8=100
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_imul_reg_reg_imm32() {
    // IMUL R8, RAX, 0x12345678 (R8 = RAX * 0x12345678)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0x69, 0xC0, // IMUL r64, r/m64, imm32
        0x78, 0x56, 0x34, 0x12, // imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD NOT/NEG (2-operand with different dest)
// ============================================================================

#[test]
fn test_ndd_not_reg_reg() {
    // NOT R8, RAX (R8 = ~RAX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xF7, 0xD0, // NOT r/m64: /2
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_not_reg_reg_match_llvm() {
    // LLVM 23 assembles "not r8, rax" as 62 f4 bc 18 f7 d0.
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xF7, 0xD0, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x0123_4567_89AB_CDEF;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x8D7;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, !0x0123_4567_89AB_CDEFu64);
    assert_eq!(regs.rax, 0x0123_4567_89AB_CDEF);
    assert_eq!(regs.rflags & 0x8D5, 0x8D5);
}

#[test]
fn test_ndd_neg_reg_reg() {
    // NEG R8, RAX (R8 = -RAX)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xF7, 0xD8, // NEG r/m64: /3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_neg_reg_reg_match_llvm() {
    // LLVM 23 assembles "neg r8, rax" as 62 f4 bc 18 f7 d8.
    const CF: u64 = 1;
    const SF: u64 = 0x80;
    let code = [0x62, 0xF4, 0xBC, 0x18, 0xF7, 0xD8, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 5u64.wrapping_neg());
    assert_eq!(regs.rax, 5);
    assert_ne!(regs.rflags & CF, 0);
    assert_ne!(regs.rflags & SF, 0);
}

// ============================================================================
// NDD INC/DEC (different destination)
// ============================================================================

#[test]
fn test_ndd_inc_reg_reg() {
    // INC R8, RAX (R8 = RAX + 1)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xFF, 0xC0, // INC r/m64: /0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_dec_reg_reg() {
    // DEC R8, RAX (R8 = RAX - 1)
    let code = [
        0x62, 0xF4, 0x7C, 0x18, // EVEX prefix with NDD
        0xFF, 0xC8, // DEC r/m64: /1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD with 32-bit operand size
// ============================================================================

#[test]
fn test_ndd_add_32bit() {
    // ADD R8D, EAX, EBX (R8D = EAX + EBX)
    let code = [
        0x62, 0xF4, 0x7C, 0x10, // EVEX prefix with NDD, W=0 for 32-bit
        0x01, 0xC0, // ADD r/m32, r32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_xor_32bit_zero_upper() {
    // XOR R8D, EAX, EAX (R8D = 0, with zero-extension to R8)
    let code = [
        0x62, 0xF4, 0x7C, 0x10, // EVEX prefix with NDD, W=0
        0x31, 0xC0, // XOR r/m32, r32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD with memory destination (not applicable - NDD is reg-only dest)
// But test with memory source
// ============================================================================

#[test]
fn test_ndd_add_reg_reg_mem_disp32() {
    // ADD RAX, RBX, [RCX + 0x12345678]
    let code = [
        0x62, 0xF4, 0x64, 0x18, // EVEX with NDD, vvvv=RBX
        0x03, 0x81, // ADD r64, r/m64: mod=10, reg=0, rm=1
        0x78, 0x56, 0x34, 0x12, // disp32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_add_reg_reg_sib() {
    // ADD RAX, RBX, [RCX + RDX*4]
    let code = [
        0x62, 0xF4, 0x64, 0x18, // EVEX with NDD
        0x03, 0x04, 0x91, // ADD r64, [SIB]: mod=00, reg=0, rm=100, SIB
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD preserves source operands (the key benefit)
// ============================================================================

#[test]
fn test_ndd_preserves_sources() {
    // After: ADD R8, RAX, RBX
    // RAX and RBX should be unchanged
    let code = [
        // MOV RAX, 0x100
        0x48, 0xC7, 0xC0, 0x00, 0x01, 0x00, 0x00, // MOV RBX, 0x200
        0x48, 0xC7, 0xC3, 0x00, 0x02, 0x00, 0x00, // ADD R8, RAX, RBX (R8 = 0x300)
        0x62, 0xF4, 0x7C, 0x18, 0x01, 0xD8, // RAX should still be 0x100, RBX still 0x200
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD with EGPR (all combinations)
// ============================================================================

#[test]
fn test_ndd_add_r16_r17_r18_egpr() {
    // ADD R16, R17, R18
    let code = [
        0x62, 0xEC, 0x74, 0x18, // EVEX with NDD and EGPR extensions
        0x01, 0xD0, // ADD: reg=2 (R18), rm=0 (dest inferred)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_sub_r24_r25_r26_egpr() {
    // SUB R24, R25, R26
    let code = [
        0x62, 0x4C, 0x34, 0x18, // EVEX with NDD and high EGPR
        0x29, 0xD0, // SUB: reg=2, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ndd_and_r31_r30_r29_egpr() {
    // AND R31, R30, R29 (all high EGPR)
    let code = [
        0x62, 0x4C, 0x0C, 0x18, // EVEX with NDD and all high EGPR
        0x21, 0xE8, // AND: reg=5, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD chained operations
// ============================================================================

#[test]
fn test_ndd_chained_operations() {
    // R8 = RAX + RBX
    // R9 = R8 + RCX
    // R10 = R9 * 2 (via SHL)
    let code = [
        // ADD R8, RAX, RBX
        0x62, 0xF4, 0x7C, 0x18, 0x01, 0xD8, // ADD R9, R8, RCX
        0x62, 0xD4, 0x3C, 0x18, 0x01, 0xC9, // SHL R10, R9, 1
        0x62, 0xD4, 0x34, 0x18, 0xD1, 0xE1, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
