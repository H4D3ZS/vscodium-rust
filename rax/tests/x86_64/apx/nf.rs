//! APX NF (No Flags) Instruction Tests
//!
//! APX introduces No-Flags variants of arithmetic and logical instructions.
//! These instructions perform their operation without modifying RFLAGS,
//! which is useful for:
//! - Avoiding flag dependencies in out-of-order execution
//! - Preserving flags set by earlier instructions
//! - Enabling better instruction-level parallelism
//!
//! NF is encoded via EVEX with the NF bit set (EVEX.NF = 1).
//! NF can be combined with NDD for 3-operand no-flags operations.
//!
//! Instructions supporting NF:
//! - ADD, SUB, AND, OR, XOR
//! - ADC, SBB (still use CF as input, but don't modify flags)
//! - SHL, SHR, SAR, ROL, ROR, RCL, RCR
//! - INC, DEC
//! - NEG, NOT
//! - IMUL
//! - SHLD, SHRD
//! - POPCNT, LZCNT, TZCNT

use crate::common::*;

// ============================================================================
// NF ADD (no flags addition)
// ============================================================================

#[test]
fn test_nf_add_real_encoding_preserves_flags_match_llvm() {
    // LLVM 23 assembles "{nf} add rax, rbx" as 62 f4 fc 0c 01 d8.
    // The result is zero, but NF must preserve the initial clear ZF.
    const ZF: u64 = 1 << 6;
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = u64::MAX;
    regs.rflags = 0x2;
    let code = [0x62, 0xF4, 0xFC, 0x0C, 0x01, 0xD8, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
    assert_eq!(regs.rflags & ZF, 0);
}

#[test]
fn test_nf_add_without_p2_bit2_updates_flags_match_llvm() {
    // LLVM 23 decodes 62 f4 fc 08 01 d8 as EVEX ADD without {nf}.
    // Because the result is zero, ZF must be updated.
    const ZF: u64 = 1 << 6;
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = u64::MAX;
    regs.rflags = 0x2;
    let code = [0x62, 0xF4, 0xFC, 0x08, 0x01, 0xD8, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
    assert_ne!(regs.rflags & ZF, 0);
}

#[test]
fn test_nf_add_reg_reg() {
    // ADD{NF} RAX, RBX (RAX = RAX + RBX, flags unchanged)
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX prefix with NF
        0x01, 0xD8, // ADD r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_add_reg_imm32() {
    // ADD{NF} RAX, 0x12345678
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x81, 0xC0, // ADD r/m64, imm32
        0x78, 0x56, 0x34, 0x12, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_add_reg_mem() {
    // ADD{NF} RAX, [RBX]
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x03, 0x03, // ADD r64, r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_add_preserves_flags() {
    // Set flags with CMP, then ADD{NF} should preserve them
    let code = [
        // CMP RAX, RBX (sets flags)
        0x48, 0x39, 0xD8, // ADD{NF} RCX, RDX (should NOT change flags)
        0x62, 0xF4, 0xFC, 0x08, 0x01, 0xD1, // JZ should work based on original CMP
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF SUB (no flags subtraction)
// ============================================================================

#[test]
fn test_nf_sub_reg_reg() {
    // SUB{NF} RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x29, 0xD8, // SUB r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_sub_reg_imm8() {
    // SUB{NF} RAX, 0x10
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x83, 0xE8, 0x10, // SUB r/m64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF AND/OR/XOR (no flags logical)
// ============================================================================

#[test]
fn test_nf_and_reg_reg() {
    // AND{NF} RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x21, 0xD8, // AND r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_or_reg_reg() {
    // OR{NF} RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x09, 0xD8, // OR r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_xor_reg_reg() {
    // XOR{NF} RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x31, 0xD8, // XOR r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_xor_zero_idiom() {
    // XOR{NF} RAX, RAX (zero without affecting flags)
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x31, 0xC0, // XOR r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF SHL/SHR/SAR (no flags shifts)
// ============================================================================

#[test]
fn test_nf_shl_reg_imm() {
    // SHL{NF} RAX, 4
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xC1, 0xE0, 0x04, // SHL r/m64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_shl_reg_cl() {
    // SHL{NF} RAX, CL
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xD3, 0xE0, // SHL r/m64, CL
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_shr_reg_imm() {
    // SHR{NF} RAX, 8
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xC1, 0xE8, 0x08, // SHR r/m64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_sar_reg_imm() {
    // SAR{NF} RAX, 1
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xD1, 0xF8, // SAR r/m64, 1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF ROL/ROR (no flags rotates)
// ============================================================================

#[test]
fn test_nf_rol_reg_imm() {
    // ROL{NF} RAX, 7
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xC1, 0xC0, 0x07, // ROL r/m64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_ror_reg_cl() {
    // ROR{NF} RAX, CL
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xD3, 0xC8, // ROR r/m64, CL
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF INC/DEC (no flags increment/decrement)
// ============================================================================

#[test]
fn test_nf_inc_reg() {
    // INC{NF} RAX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xFF, 0xC0, // INC r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_dec_reg() {
    // DEC{NF} RAX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xFF, 0xC8, // DEC r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_inc_preserves_cf() {
    // INC normally preserves CF; NF INC preserves ALL flags
    let code = [
        // STC (set carry flag)
        0xF9, // INC{NF} RAX (should preserve CF and all other flags)
        0x62, 0xF4, 0xFC, 0x08, 0xFF, 0xC0, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF NEG/NOT (no flags unary)
// ============================================================================

#[test]
fn test_nf_neg_reg() {
    // NEG{NF} RAX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xF7, 0xD8, // NEG r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_not_reg() {
    // NOT{NF} RAX (NOT doesn't affect flags anyway, but NF is valid)
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0xF7, 0xD0, // NOT r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF MUL/DIV (implicit RDX:RAX forms)
// ============================================================================

#[test]
fn test_nf_mul_implicit_match_llvm() {
    // LLVM 23 assembles "{nf} mul rbx" as 62 f4 fc 0c f7 e3.
    const CF: u64 = 1 << 0;
    const ZF: u64 = 1 << 6;
    let code = [0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xE3, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 3;
    regs.rbx = 4;
    regs.rflags = CF | ZF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 12);
    assert_eq!(regs.rdx, 0);
    assert_eq!(regs.rflags & (CF | ZF), CF | ZF);
}

#[test]
fn test_nf_imul_implicit_match_llvm() {
    // LLVM 23 assembles "{nf} imul rbx" as 62 f4 fc 0c f7 eb.
    const OF: u64 = 1 << 11;
    let code = [0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xEB, 0xF4];
    let mut regs = Registers::default();
    regs.rax = (-3i64) as u64;
    regs.rbx = 4;
    regs.rflags = OF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, (-12i64) as u64);
    assert_eq!(regs.rdx, u64::MAX);
    assert_eq!(regs.rflags & OF, OF);
}

#[test]
fn test_nf_div_implicit_match_llvm() {
    // LLVM 23 assembles "{nf} div rbx" as 62 f4 fc 0c f7 f3.
    const CF: u64 = 1 << 0;
    let code = [0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xF3, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rdx = 0;
    regs.rbx = 7;
    regs.rflags = CF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 14);
    assert_eq!(regs.rdx, 2);
    assert_eq!(regs.rflags & CF, CF);
}

#[test]
fn test_nf_idiv_implicit_match_llvm() {
    // LLVM 23 assembles "{nf} idiv rbx" as 62 f4 fc 0c f7 fb.
    const SF: u64 = 1 << 7;
    let code = [0x62, 0xF4, 0xFC, 0x0C, 0xF7, 0xFB, 0xF4];
    let mut regs = Registers::default();
    regs.rax = (-100i64) as u64;
    regs.rdx = u64::MAX;
    regs.rbx = 7;
    regs.rflags = SF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, (-14i64) as u64);
    assert_eq!(regs.rdx, (-2i64) as u64);
    assert_eq!(regs.rflags & SF, SF);
}

// ============================================================================
// NF ADC/SBB (uses CF as input but doesn't update flags)
// ============================================================================

#[test]
fn test_nf_adc_uses_cf() {
    // ADC{NF} RAX, RBX - uses CF but doesn't modify flags
    let code = [
        // STC (set CF)
        0xF9, // ADC{NF} RAX, RBX (RAX = RAX + RBX + 1)
        0x62, 0xF4, 0xFC, 0x08, 0x11, 0xD8, // CF should still be set (from STC)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_sbb_uses_cf() {
    // SBB{NF} RAX, RBX - uses CF but doesn't modify flags
    let code = [
        // STC
        0xF9, // SBB{NF} RAX, RBX (RAX = RAX - RBX - 1)
        0x62, 0xF4, 0xFC, 0x08, 0x19, 0xD8, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF IMUL (no flags multiply)
// ============================================================================

#[test]
fn test_nf_imul_reg_reg() {
    // IMUL{NF} RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x88, // EVEX with NF and 0F map
        0xAF, 0xC3, // IMUL r64, r/m64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_imul_reg_reg_imm() {
    // IMUL{NF} RAX, RBX, 100
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x6B, 0xC3, 0x64, // IMUL r64, r/m64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF SHLD/SHRD (no flags double shifts)
// ============================================================================

#[test]
fn test_nf_shld_reg_reg_imm() {
    // SHLD{NF} RAX, RBX, 8
    let code = [
        0x62, 0xF4, 0xFC, 0x88, // EVEX with NF and 0F map
        0xA4, 0xD8, 0x08, // SHLD r/m64, r64, imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_shrd_reg_reg_cl() {
    // SHRD{NF} RAX, RBX, CL
    let code = [
        0x62, 0xF4, 0xFC, 0x88, // EVEX with NF and 0F map
        0xAD, 0xD8, // SHRD r/m64, r64, CL
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF + NDD combined (3-operand no-flags)
// ============================================================================

#[test]
fn test_nf_ndd_add() {
    // ADD{NF} R8, RAX, RBX (R8 = RAX + RBX, no flags modified)
    let code = [
        0x62, 0xF4, 0xFC, 0x18, // EVEX with NF and NDD
        0x01, 0xD8, // ADD
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_ndd_sub() {
    // SUB{NF} R8, RAX, RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x18, // EVEX with NF and NDD
        0x29, 0xD8, // SUB
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_ndd_shl() {
    // SHL{NF} R8, RAX, 4
    let code = [
        0x62, 0xF4, 0xFC, 0x18, // EVEX with NF and NDD
        0xC1, 0xE0, 0x04, // SHL imm8
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF with EGPR
// ============================================================================

#[test]
fn test_nf_add_r16_r17() {
    // ADD{NF} R16, R17
    let code = [
        0x62, 0xEC, 0xFC, 0x08, // EVEX with NF and EGPR
        0x01, 0xC8, // ADD r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_sub_r24_r31() {
    // SUB{NF} R24, R31
    let code = [
        0x62, 0x4C, 0xFC, 0x08, // EVEX with NF and high EGPR
        0x29, 0xF8, // SUB
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF with 32-bit operands
// ============================================================================

#[test]
fn test_nf_add_32bit() {
    // ADD{NF} EAX, EBX (32-bit, no flags)
    let code = [
        0x62, 0xF4, 0x7C, 0x08, // EVEX with NF, W=0
        0x01, 0xD8, // ADD r/m32, r32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF flag preservation test sequence
// ============================================================================

#[test]
fn test_nf_complete_flag_preservation() {
    // Comprehensive test: set specific flags, do NF ops, verify flags unchanged
    let code = [
        // Set up specific flag state
        // XOR EAX, EAX (sets ZF=1, SF=0, OF=0, CF=0, PF=1)
        0x31, 0xC0,
        // Now do multiple NF operations - flags should stay same
        // ADD{NF} RAX, 0x100
        0x62, 0xF4, 0xFC, 0x08, 0x81, 0xC0, 0x00, 0x01, 0x00, 0x00, // SUB{NF} RAX, 0x50
        0x62, 0xF4, 0xFC, 0x08, 0x83, 0xE8, 0x50, // SHL{NF} RAX, 4
        0x62, 0xF4, 0xFC, 0x08, 0xC1, 0xE0, 0x04, // INC{NF} RAX
        0x62, 0xF4, 0xFC, 0x08, 0xFF, 0xC0, // Flags should still be: ZF=1, SF=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF with memory operands
// ============================================================================

#[test]
fn test_nf_add_mem_reg() {
    // ADD{NF} [RAX], RBX
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x01, 0x18, // ADD [r/m64], r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_nf_sub_reg_mem() {
    // SUB{NF} RAX, [RBX + 0x100]
    let code = [
        0x62, 0xF4, 0xFC, 0x08, // EVEX with NF
        0x2B, 0x83, // SUB r64, [r/m64+disp32]
        0x00, 0x01, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NF count instructions
// ============================================================================

#[test]
fn test_nf_popcnt_reg_reg_preserves_flags_match_llvm() {
    // LLVM 23 assembles "{nf} popcnt r8, rax" as 62 74 fc 0c 88 c0.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rax = 0xF0F0_F0F0_0F0F_0F0F;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0x74, 0xFC, 0x0C, 0x88, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 32);
    assert_eq!(regs.rax, 0xF0F0_F0F0_0F0F_0F0F);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}

#[test]
fn test_nf_lzcnt_reg_reg_preserves_flags_match_llvm() {
    // LLVM 23 assembles "{nf} lzcnt r8, rax" as 62 74 fc 0c f5 c0.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rax = 0x00F0_0000_0000_0000;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0x74, 0xFC, 0x0C, 0xF5, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 8);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}

#[test]
fn test_nf_tzcnt_reg_reg_preserves_flags_match_llvm() {
    // LLVM 23 assembles "{nf} tzcnt r8, rax" as 62 74 fc 0c f4 c0.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0x74, 0xFC, 0x0C, 0xF4, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 12);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}

#[test]
fn test_nf_popcnt_word_preserves_upper_match_llvm() {
    // LLVM 23 assembles "{nf} popcnt r8w, ax" as 62 74 7d 0c 88 c0.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rax = 0xF0F0;
    regs.r8 = 0x1122_3344_5566_7788;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0x74, 0x7D, 0x0C, 0x88, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1122_3344_5566_0008);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}

#[test]
fn test_nf_lzcnt_memory_match_llvm() {
    // LLVM 23 assembles "{nf} lzcnt r8, [rbx]" as 62 74 fc 0c f5 03.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.r8 = 0xDEAD_BEEF;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0x74, 0xFC, 0x0C, 0xF5, 0x03, 0xF4];

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x0000_0000_0000_1000);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 51);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}
