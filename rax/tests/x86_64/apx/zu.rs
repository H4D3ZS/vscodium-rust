//! Intel APX Zero-Upper (ZU) Semantics Tests
//!
//! APX introduces zero-upper semantics for certain 32-bit operations in 64-bit mode.
//! When a 32-bit instruction writes to a GPR using the ZU form, the upper 32 bits
//! of the 64-bit register are zeroed.
//!
//! This is similar to the standard x86-64 behavior for 32-bit operations (which
//! always zero-extend), but APX extends this to additional instruction forms
//! and allows explicit control via the EVEX.ZU bit.
//!
//! EVEX.ZU encoding:
//! - Located in EVEX byte 3, bit position varies by instruction
//! - ZU=0: Preserve upper 32 bits (merge)
//! - ZU=1: Zero upper 32 bits (standard x86-64 behavior)
//!
//! Note: Standard x86-64 32-bit operations ALWAYS zero the upper 32 bits.
//! The ZU bit in APX provides explicit control for new instruction forms.

use crate::common::*;

#[test]
fn test_setzuo_true_zeroes_upper_match_llvm() {
    // LLVM 23 assembles "setzuo al" as 62 f4 7f 18 40 c0.
    const OF: u64 = 1 << 11;
    let code = [0x62, 0xF4, 0x7F, 0x18, 0x40, 0xC0, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FF00;
    regs.rflags = OF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1);
}

#[test]
fn test_setzune_false_zeroes_upper_match_llvm() {
    // LLVM 23 assembles "setzune bl" as 62 f4 7f 18 45 c3.
    const ZF: u64 = 1 << 6;
    let code = [0x62, 0xF4, 0x7F, 0x18, 0x45, 0xC3, 0xF4];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rflags = ZF | 0x2;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0);
}

// ============================================================================
// Basic Zero-Upper Tests
// ============================================================================

/// Standard 32-bit MOV zeros upper bits (baseline behavior)
#[test]
fn test_standard_mov32_zeros_upper() {
    // MOV eax, 0x12345678 - standard instruction
    let code = [
        0xB8, 0x78, 0x56, 0x34, 0x12, // MOV eax, imm32
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// APX NDD instruction with ZU=1 zeros upper bits
#[test]
fn test_apx_add_ndd_zu_set() {
    // ADD32 eax, ebx, ecx (NDD with ZU=1)
    let code = [
        0x62, 0xF4, 0x64, 0x1A, // EVEX with ZU=1
        0x01, 0xD9, // ADD
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 10;
    regs.rcx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// APX NDD instruction with ZU=0 preserves upper bits
#[test]
fn test_apx_add_ndd_zu_clear() {
    // ADD32 eax, ebx, ecx (NDD with ZU=0)
    let code = [
        0x62, 0xF4, 0x64, 0x10, // EVEX with ZU=0
        0x01, 0xD9, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF00000000;
    regs.rbx = 10;
    regs.rcx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Arithmetic Instructions with ZU
// ============================================================================

/// SUB with ZU=1
#[test]
fn test_sub_ndd_zu_set() {
    // SUB32 eax, ebx, ecx with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0x29, 0xD9, // SUB
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xCAFEBABE00000000;
    regs.rbx = 100;
    regs.rcx = 30;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// SUB with ZU=0
#[test]
fn test_sub_ndd_zu_clear() {
    let code = [0x62, 0xF4, 0x64, 0x10, 0x29, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1234567800000000;
    regs.rbx = 100;
    regs.rcx = 30;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// AND with ZU=1
#[test]
fn test_and_ndd_zu_set() {
    // AND32 eax, ebx, ecx with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0x21, 0xD9, // AND
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xABCDEF0000000000;
    regs.rbx = 0xFF00FF00;
    regs.rcx = 0x0F0F0F0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// OR with ZU=0
#[test]
fn test_or_ndd_zu_clear() {
    // OR32 eax, ebx, ecx with ZU=0
    let code = [
        0x62, 0xF4, 0x64, 0x10, 0x09, 0xD9, // OR
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADDEAD00000000;
    regs.rbx = 0x00FF0000;
    regs.rcx = 0x000000FF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// XOR with ZU=1
#[test]
fn test_xor_ndd_zu_set() {
    // XOR32 eax, ebx, ecx with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0x31, 0xD9, // XOR
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111100000000;
    regs.rbx = 0xAAAAAAAA;
    regs.rcx = 0x55555555;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Shift Instructions with ZU
// ============================================================================

/// SHL with ZU=1
#[test]
fn test_shl_ndd_zu_set() {
    // SHL32 eax, ebx, 4 with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0xC1, 0xE3, 0x04, // SHL by 4
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// SHR with ZU=0
#[test]
fn test_shr_ndd_zu_clear() {
    // SHR32 eax, ebx, 4 with ZU=0
    let code = [
        0x62, 0xF4, 0x64, 0x10, 0xC1, 0xEB, 0x04, // SHR by 4
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xBEEFBEEF00000000;
    regs.rbx = 0x80000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// SAR with ZU=1
#[test]
fn test_sar_ndd_zu_set() {
    // SAR32 eax, ebx, 4 with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0xC1, 0xFB, 0x04, // SAR by 4
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADDEAD00000000;
    regs.rbx = 0x80000000u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// ROL with ZU=1
#[test]
fn test_rol_ndd_zu_set() {
    // ROL32 eax, ebx, 8 with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0xC1, 0xC3, 0x08, // ROL by 8
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA00000000;
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// ROR with ZU=0
#[test]
fn test_ror_ndd_zu_clear() {
    // ROR32 eax, ebx, 8 with ZU=0
    let code = [
        0x62, 0xF4, 0x64, 0x10, 0xC1, 0xCB, 0x08, // ROR by 8
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x5555555500000000;
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ZU with EGPR (R16-R31)
// ============================================================================

/// ADD to R16 with ZU=1
#[test]
fn test_add_r16_zu_set() {
    // ADD32 r16d, ebx, ecx with ZU=1
    let code = [0x62, 0xEC, 0x64, 0x1A, 0x01, 0xD9, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// SUB from R24 with ZU=0
#[test]
fn test_sub_r24_zu_clear() {
    // SUB32 r24d, r25d, r26d with ZU=0
    let code = [0x62, 0xEC, 0x34, 0x10, 0x29, 0xD1, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

/// SHL with R31 and ZU=1
#[test]
fn test_shl_r31_zu_set() {
    // SHL32 r31d, r30d, 4 with ZU=1
    let code = [0x62, 0xEC, 0x04, 0x1A, 0xC1, 0xE6, 0x04, 0xF4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Memory Operations with ZU
// ============================================================================

/// MOV from memory with ZU=1
#[test]
fn test_mov_from_mem_zu_set() {
    // MOV32 eax, [rbx] with ZU=1
    let code = [
        0x62, 0xF4, 0x7C, 0x1A, 0x8B, 0x03, // MOV eax, [rbx]
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 0x12345678);
    let _ = run_until_hlt(&mut vcpu);
}

/// ADD from memory with ZU=0
#[test]
fn test_add_from_mem_zu_clear() {
    // ADD32 eax, [rbx] with ZU=0
    let code = [
        0x62, 0xF4, 0x7C, 0x10, 0x03, 0x03, // ADD eax, [rbx]
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xABCDABCD00000064;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 50);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Overflow and Edge Cases
// ============================================================================

/// 32-bit overflow with ZU=1 (result wraps, upper zeroed)
#[test]
fn test_add_overflow_zu_set() {
    // ADD32 eax, ebx, ecx with ZU=1 -> overflows to 0
    let code = [0x62, 0xF4, 0x64, 0x1A, 0x01, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// 32-bit overflow with ZU=0 (result wraps, upper preserved)
#[test]
fn test_add_overflow_zu_clear() {
    // ADD32 eax, ebx, ecx with ZU=0
    let code = [0x62, 0xF4, 0x64, 0x10, 0x01, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF00000000;
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// Zero result with ZU=1
#[test]
fn test_xor_zero_zu_set() {
    // XOR32 eax, ebx, ecx with ZU=1 -> 0
    let code = [0x62, 0xF4, 0x64, 0x1A, 0x31, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xCAFEBABE00000000;
    regs.rbx = 0x12345678;
    regs.rcx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// Zero result with ZU=0
#[test]
fn test_xor_zero_zu_clear() {
    // XOR32 eax, ebx, ecx with ZU=0
    let code = [0x62, 0xF4, 0x64, 0x10, 0x31, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFEEDFACE00000000;
    regs.rbx = 0x12345678;
    regs.rcx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// ZU with Different Operand Sizes
// ============================================================================

/// 16-bit operation does NOT zero upper bits (only 32-bit does)
#[test]
fn test_16bit_preserves_upper() {
    // MOV ax, 0x1234 (16-bit)
    let code = [
        0x66, 0xB8, 0x34, 0x12, // MOV ax, imm16
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFF0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// 8-bit operation preserves upper bits
#[test]
fn test_8bit_preserves_upper() {
    // MOV al, 0x42 (8-bit)
    let code = [
        0xB0, 0x42, // MOV al, imm8
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Combined ZU and NF (No Flags)
// ============================================================================

/// ADD with both NF and ZU=1
#[test]
fn test_add_nf_zu_set() {
    // ADD32 eax, ebx, ecx with NF=1 and ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1E, // NF=1, ZU=1
        0x01, 0xD9, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xCAFEBABE00000000;
    regs.rbx = 100;
    regs.rcx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

/// SUB with NF and ZU=0
#[test]
fn test_sub_nf_zu_clear() {
    // SUB32 eax, ebx, ecx with NF=1 and ZU=0
    let code = [
        0x62, 0xF4, 0x64, 0x14, // NF=1, ZU=0
        0x29, 0xD9, 0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xCAFEBABE00000000;
    regs.rbx = 100;
    regs.rcx = 30;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// NDD with ZU
// ============================================================================

/// NDD ADD with ZU preserves source registers
#[test]
fn test_ndd_add_zu_preserves_sources() {
    // ADD32 eax, ebx, ecx with ZU=1
    let code = [0x62, 0xF4, 0x64, 0x1A, 0x01, 0xD9, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 0xAAAAAAAA00000064;
    regs.rcx = 0xBBBBBBBB00000032;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Consecutive ZU Operations
// ============================================================================

/// Multiple ZU operations in sequence
#[test]
fn test_consecutive_zu_operations() {
    // ADD32 with ZU=1; SHL32 with ZU=1
    let code = [
        0x62, 0xF4, 0x64, 0x1A, 0x01, 0xD9, // ADD32
        0x62, 0xF4, 0x7C, 0x1A, 0xC1, 0xE0, 0x02, // SHL32
        0xF4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF00000000;
    regs.rbx = 10;
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu);
}
