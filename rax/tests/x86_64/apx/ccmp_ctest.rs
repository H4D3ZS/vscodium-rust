//! Intel APX Conditional Compare/Test (CCMP/CTEST) Tests
//!
//! CCMP and CTEST are new APX instructions that perform conditional
//! compare/test operations based on the current flag state.
//!
//! Instruction format:
//! - CCMP{cond} dst, src, dfv  - Conditional compare
//! - CTEST{cond} dst, src, dfv - Conditional test (AND)
//!
//! If the condition is true, the instruction performs CMP/TEST normally.
//! If the condition is false, flags are set to the default flag values (dfv).
//!
//! DFV (Default Flag Values) encoding in immediate:
//! - Bit 0: CF default
//! - Bit 1: ZF default
//! - Bit 2: SF default
//! - Bit 3: OF default
//!
//! EVEX encoding:
//! - EVEX.NF must be 0 (these instructions always modify flags)
//! - Condition code in EVEX.SC field (bits [3:0] of payload2)
//! - DFV in immediate byte

use crate::common::*;

// ============================================================================
// CCMP Instruction Tests
// ============================================================================

#[test]
fn test_ccmp_false_condition_uses_encoded_default_flags_match_llvm() {
    // LLVM 23 assembles "ccmpo {dfv=cf,zf} rax, rbx" without a trailing dfv byte:
    // 62 f4 9c 00 39 d8. OF is initially clear, so CCMPO is false and the
    // encoded default flags must be applied.
    const CF: u64 = 1;
    const ZF: u64 = 1 << 6;
    const SF: u64 = 1 << 7;
    const OF: u64 = 1 << 11;
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 1;
    regs.rflags = 0x2;
    let code = [0x62, 0xF4, 0x9C, 0x00, 0x39, 0xD8, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_ne!(regs.rflags & CF, 0);
    assert_ne!(regs.rflags & ZF, 0);
    assert_eq!(regs.rflags & SF, 0);
    assert_eq!(regs.rflags & OF, 0);
}

#[test]
fn test_ccmp_true_condition_performs_compare_match_llvm() {
    // LLVM 23 assembles "ccmpno {dfv=cf,zf} rax, rbx" as 62 f4 9c 01 39 d8.
    // OF is initially clear, so CCMPNO is true and the CMP result wins over dfv.
    const CF: u64 = 1;
    const ZF: u64 = 1 << 6;
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 1;
    regs.rflags = 0x2;
    let code = [0x62, 0xF4, 0x9C, 0x01, 0x39, 0xD8, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & CF, 0);
    assert_ne!(regs.rflags & ZF, 0);
}

#[test]
fn test_ctest_false_condition_uses_encoded_default_flags_match_llvm() {
    // LLVM 23 assembles "ctesto {dfv=sf,of} rax, rbx" as 62 f4 e4 00 85 d8.
    // OF is initially clear, so CTESTO is false and default SF/OF are applied.
    const CF: u64 = 1;
    const ZF: u64 = 1 << 6;
    const SF: u64 = 1 << 7;
    const OF: u64 = 1 << 11;
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0;
    regs.rflags = 0x2;
    let code = [0x62, 0xF4, 0xE4, 0x00, 0x85, 0xD8, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags & CF, 0);
    assert_eq!(regs.rflags & ZF, 0);
    assert_ne!(regs.rflags & SF, 0);
    assert_ne!(regs.rflags & OF, 0);
}

/// CCMPO (overflow) - Compare if OF=1
#[test]
fn test_ccmpo_r64_r64() {
    // CCMPO rcx, rdx, dfv=0
    // EVEX.NDS CCMPO encoding with condition=0
    let code = [
        0x62, 0xF4, 0xE4, 0x00, // EVEX prefix (CCMPO condition=0)
        0x39, 0xD1, // CMP rcx, rdx
        0x00, // dfv = 0
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNO (not overflow) - Compare if OF=0
#[test]
fn test_ccmpno_r64_r64() {
    // CCMPNO rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x01, // EVEX prefix (CCMPNO condition=1)
        0x39, 0xD8, // CMP rax, rbx
        0x00, // dfv = 0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPB (below/carry) - Compare if CF=1
#[test]
fn test_ccmpb_r32_r32() {
    // CCMPB ecx, edx, dfv=0
    let code = [
        0x62, 0xF4, 0x64, 0x02, // EVEX prefix (CCMPB condition=2)
        0x39, 0xD1, // CMP ecx, edx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNB (not below) - Compare if CF=0
#[test]
fn test_ccmpnb_r64_imm8() {
    // CCMPNB rax, 100, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x03, // EVEX prefix (CCMPNB condition=3)
        0x83, 0xF8, 0x64, // CMP rax, 100
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPZ (zero) - Compare if ZF=1
#[test]
fn test_ccmpz_r64_r64() {
    // CCMPZ r8, r9, dfv=0
    let code = [
        0x62, 0xD4, 0xE4, 0x04, // EVEX prefix (CCMPZ condition=4)
        0x4D, 0x39, 0xC8, // CMP r8, r9
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNZ (not zero) - Compare if ZF=0
#[test]
fn test_ccmpnz_r64_mem() {
    // CCMPNZ rax, [rbx], dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x05, // EVEX prefix (CCMPNZ condition=5)
        0x3B, 0x03, // CMP rax, [rbx]
        0x00, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x1234);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPBE (below or equal) - Compare if CF=1 or ZF=1
#[test]
fn test_ccmpbe_r64_r64() {
    // CCMPBE rcx, rdx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x06, // EVEX prefix (CCMPBE condition=6)
        0x39, 0xD1, // CMP rcx, rdx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNBE (above) - Compare if CF=0 and ZF=0
#[test]
fn test_ccmpnbe_r64_r64() {
    // CCMPNBE rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x07, // EVEX prefix (CCMPNBE condition=7)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPS (sign) - Compare if SF=1
#[test]
fn test_ccmps_r64_r64() {
    // CCMPS rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x08, // EVEX prefix (CCMPS condition=8)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNS (not sign) - Compare if SF=0
#[test]
fn test_ccmpns_r64_r64() {
    // CCMPNS rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x09, // EVEX prefix (CCMPNS condition=9)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPP (parity) - Compare if PF=1
#[test]
fn test_ccmpp_r64_r64() {
    // CCMPP rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0A, // EVEX prefix (CCMPP condition=10)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPNP (not parity) - Compare if PF=0
#[test]
fn test_ccmpnp_r64_r64() {
    // CCMPNP rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0B, // EVEX prefix (CCMPNP condition=11)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPL (less) - Compare if SF!=OF
#[test]
fn test_ccmpl_r64_r64() {
    // CCMPL rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0C, // EVEX prefix (CCMPL condition=12)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPGE (greater or equal) - Compare if SF==OF
#[test]
fn test_ccmpge_r64_r64() {
    // CCMPGE rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0D, // EVEX prefix (CCMPGE condition=13)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPLE (less or equal) - Compare if ZF=1 or SF!=OF
#[test]
fn test_ccmple_r64_r64() {
    // CCMPLE rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0E, // EVEX prefix (CCMPLE condition=14)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMPG (greater) - Compare if ZF=0 and SF==OF
#[test]
fn test_ccmpg_r64_r64() {
    // CCMPG rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x0F, // EVEX prefix (CCMPG condition=15)
        0x39, 0xD8, // CMP rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CTEST Instruction Tests
// ============================================================================

/// CTESTO - Test if OF=1
#[test]
fn test_ctesto_r64_r64() {
    // CTESTO rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x40, // EVEX prefix (CTESTO condition=0, test=1)
        0x85, 0xD8, // TEST rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTESTZ - Test if ZF=1
#[test]
fn test_ctestz_r32_r32() {
    // CTESTZ ecx, edx, dfv=0
    let code = [
        0x62, 0xF4, 0x64, 0x44, // EVEX prefix (CTESTZ condition=4)
        0x85, 0xD1, // TEST ecx, edx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTESTNZ - Test if ZF=0
#[test]
fn test_ctestnz_r64_imm32() {
    // CTESTNZ rax, 0x0F, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x45, // EVEX prefix (CTESTNZ condition=5)
        0xF7, 0xC0, // TEST rax, imm32
        0x0F, 0x00, 0x00, 0x00, // imm32 = 0x0F
        0x00, // dfv
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTESTB - Test if CF=1
#[test]
fn test_ctestb_r64_r64() {
    // CTESTB rax, rbx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x42, // EVEX prefix (CTESTB condition=2)
        0x85, 0xD8, // TEST rax, rbx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTESTS - Test if SF=1
#[test]
fn test_ctests_mem_imm() {
    // CTESTS [rbx], 0xF0, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x48, // EVEX prefix (CTESTS condition=8)
        0xF7, 0x03, // TEST [rbx], imm32
        0xF0, 0x00, 0x00, 0x00, // imm32 = 0xF0
        0x00, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x00F0);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// DFV (Default Flag Values) Tests
// ============================================================================

/// CCMP with all flags set in dfv
#[test]
fn test_ccmp_dfv_all_set() {
    // CCMPO rax, rbx, dfv=0x0F (CF|ZF|SF|OF)
    let code = [
        0x62, 0xF4, 0xE4, 0x00, 0x39, 0xD8, 0x0F, // dfv = all 4 flags set
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMP with selective flags in dfv
#[test]
fn test_ccmp_dfv_selective() {
    // CCMPO rax, rbx, dfv=0x05 (CF|SF)
    let code = [
        0x62, 0xF4, 0xE4, 0x00, 0x39, 0xD8, 0x05, // dfv = CF + SF
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// CCMP/CTEST with EGPR (R16-R31)
// ============================================================================

/// CCMP with R16, R17
#[test]
fn test_ccmpz_r16_r17() {
    // CCMPZ r16, r17, dfv=0 - using extended EVEX for EGPR
    let code = [
        0x62, 0xEC, 0xE4, 0x04, // EVEX with EGPR bits
        0x39, 0xC8, // CMP r16, r17 (modrm)
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTEST with R20
#[test]
fn test_ctestnz_r20_imm() {
    // CTESTNZ r20, 0x0F, dfv=0
    let code = [
        0x62, 0xEC, 0x64, 0x45, // EVEX with EGPR
        0xF7, 0xC4, // TEST r20, imm32
        0x0F, 0x00, 0x00, 0x00, 0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Size Variants
// ============================================================================

/// CCMP with 8-bit operands
#[test]
fn test_ccmpz_r8_r8() {
    // CCMPZ al, bl, dfv=0
    let code = [
        0x62, 0xF4, 0x24, 0x04, // EVEX 8-bit
        0x38, 0xD8, // CMP al, bl
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CCMP with 16-bit operands
#[test]
fn test_ccmpnz_r16_r16_operands() {
    // CCMPNZ ax, bx, dfv=0
    let code = [
        0x62, 0xF4, 0x25, 0x05, // EVEX 16-bit (OSO)
        0x39, 0xD8, // CMP ax, bx
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTEST with 8-bit operands
#[test]
fn test_ctestb_r8_imm8() {
    // CTESTB al, 0x0F, dfv=0
    let code = [
        0x62, 0xF4, 0x24, 0x42, 0xF6, 0xC0, 0x0F, // TEST al, 0x0F
        0x00, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Chained CCMP (Multi-condition Evaluation)
// ============================================================================

/// Chained CCMP pattern
#[test]
fn test_ccmp_chain() {
    // CMP rax, 10; CCMPG rbx, 20, dfv=0x02
    let code = [
        0x48, 0x83, 0xF8, 0x0A, // CMP rax, 10
        0x62, 0xF4, 0xE4, 0x0F, // CCMPG
        0x83, 0xFB, 0x14, // CMP rbx, 20
        0x02, // dfv = ZF set
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 15; // > 10
    regs.rbx = 12; // < 20
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

/// CCMP with memory source
#[test]
fn test_ccmpnz_r64_mem64() {
    // CCMPNZ rax, [rbx], dfv=0
    let code = [0x62, 0xF4, 0xE4, 0x05, 0x3B, 0x03, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 500);
    let _ = run_until_hlt(&mut vcpu);
}

/// CTEST with memory operand
#[test]
fn test_ctestb_mem_r64() {
    // CTESTB [rbx], rcx, dfv=0
    let code = [
        0x62, 0xF4, 0xE4, 0x42, 0x85, 0x0B, // TEST [rbx], rcx
        0x00, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 0x00FF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0xFF00);
    let _ = run_until_hlt(&mut vcpu);
}
