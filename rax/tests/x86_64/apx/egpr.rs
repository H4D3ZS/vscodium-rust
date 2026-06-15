//! APX Extended General Purpose Registers (EGPR) Tests
//!
//! Tests for the 16 new general-purpose registers R16-R31.
//! These registers are accessed using the REX2 prefix or Extended EVEX encoding.
//!
//! REX2 prefix format: 0xD5 [M:R3:X3:B3:W:R4:X4:B4]
//! - M bit: 0 = legacy map, 1 = 0F escape (two-byte opcode map)
//! - R3/X3/B3: Standard REX bits for registers 8-15
//! - R4/X4/B4: Extension bits for registers 16-31
//!   - When R4=1, reg field encodes R16-R23 (with R3=0) or R24-R31 (with R3=1)
//!   - When B4=1, r/m field encodes R16-R23 (with B3=0) or R24-R31 (with B3=1)
//!   - When X4=1, index encodes R16-R23 (with X3=0) or R24-R31 (with X3=1)

use crate::common::*;

// ============================================================================
// Basic MOV with EGPR (R16-R31)
// ============================================================================

#[test]
fn test_mov_r16_imm64() {
    // MOV R16, imm64
    // REX2.W encoding: 0xD5 0x19 0xB8+rd imm64
    // REX2 payload: M=0 R3=0 X3=0 B3=0 W=1 R4=1 X4=0 B4=0 = 0x19
    // B8+0 = B8 for R16 (rd=0 with B4=1)
    let code = [
        0xD5, 0x19, // REX2.WB4
        0xB8, // MOV r64, imm64 (R16)
        0xEF, 0xBE, 0xAD, 0xDE, 0x78, 0x56, 0x34, 0x12, // imm64 = 0x12345678DEADBEEF
        0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_r17_imm64() {
    // MOV R17, imm64
    // REX2.W with B4=1, rd=1 for R17
    let code = [
        0xD5, 0x19, // REX2.WB4
        0xB9, // MOV r64, imm64 (R17 = B8+1)
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // imm64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_r24_imm64() {
    // MOV R24, imm64
    // REX2.W with B3=1 B4=1 for R24 (16+8=24)
    // REX2 payload: M=0 R3=0 X3=0 B3=1 W=1 R4=1 X4=0 B4=0 = 0x59
    // Wait, B4 should be 1 for extended. Let me recalculate:
    // For R24: B3=1, B4=1 -> register 24 = 8 + 16
    // Payload: M=0 R3=0 X3=0 B3=1 W=1 R4=0 X4=0 B4=1 = 0x51
    let code = [
        0xD5, 0x51, // REX2.WB3B4
        0xB8, // MOV r64, imm64 (R24 = base 0 + B3*8 + B4*16)
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_r31_imm64() {
    // MOV R31, imm64
    // R31 = 31 = 7 + 8 + 16, so rd=7, B3=1, B4=1
    // REX2 payload: M=0 R3=0 X3=0 B3=1 W=1 R4=0 X4=0 B4=1 = 0x51
    let code = [
        0xD5, 0x51, // REX2.WB3B4
        0xBF, // MOV r64, imm64 (rd=7 -> R31)
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F, // imm64 = INT64_MAX
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// MOV reg-to-reg with EGPR
// ============================================================================

#[test]
fn test_mov_r16_rax() {
    // MOV R16, RAX
    // REX2: 0xD5 with R4=1 for R16 as destination
    // 89 /r = MOV r/m64, r64 (r=RAX, r/m=R16)
    // ModRM: mod=11, reg=0 (RAX), rm=0 (R16 with B4)
    let code = [
        0xD5, 0x11, // REX2.WB4 (W=1 for 64-bit, B4=1 for R16)
        0x89, 0xC0, // MOV r/m64, r64: mod=11, reg=0, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_rax_r16() {
    // MOV RAX, R16
    // 8B /r = MOV r64, r/m64 (r=RAX, r/m=R16)
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x8B, 0xC0, // MOV r64, r/m64: mod=11, reg=0 (RAX), rm=0 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_r16_r17() {
    // MOV R16, R17
    // Both registers are EGPR
    let code = [
        0xD5, 0x15, // REX2.WR4B4 (R4=1 for R17 as source, B4=1 for R16 as dest)
        0x89, 0xC8, // MOV r/m64, r64: mod=11, reg=1 (R17), rm=0 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_r24_r31() {
    // MOV R24, R31
    // R24 needs B3=1, B4=1
    // R31 needs R3=1, R4=1, reg=7
    let code = [
        0xD5, 0x75, // REX2.WR3R4B3B4
        0x89, 0xF8, // MOV r/m64, r64: mod=11, reg=7 (R31), rm=0 (R24)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Memory operations with EGPR as base
// ============================================================================

#[test]
fn test_mov_mem_r16_base() {
    // MOV [R16], RAX
    // Using R16 as base register
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x89, 0x00, // MOV [r/m64], r64: mod=00, reg=0, rm=0 (R16 as base)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_mem_r16_disp8() {
    // MOV [R16+0x10], RAX
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x89, 0x40, 0x10, // MOV [r/m64+disp8], r64: mod=01, reg=0, rm=0, disp8=0x10
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_mem_r16_disp32() {
    // MOV [R16+0x12345678], RAX
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x89, 0x80, // MOV [r/m64+disp32], r64: mod=10, reg=0, rm=0
        0x78, 0x56, 0x34, 0x12, // disp32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Memory operations with EGPR as index (SIB)
// ============================================================================

#[test]
fn test_mov_mem_sib_r16_index() {
    // MOV [RAX + R16*4], RBX
    // SIB with R16 as index register
    let code = [
        0xD5, 0x12, // REX2.WX4 (X4=1 for R16 as index)
        0x89, 0x1C, 0x80, // MOV [SIB], r64: mod=00, reg=3, rm=100, SIB=scale*4 idx=0 base=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_mem_sib_r24_index() {
    // MOV [RAX + R24*8], RCX
    // R24 as index requires X3=1, X4=1
    let code = [
        0xD5, 0x52, // REX2.WX3X4
        0x89, 0x0C, 0xC0, // MOV [SIB], r64: mod=00, reg=1, rm=100, SIB=scale*8 idx=0 base=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_mov_mem_sib_r16_base_r24_index() {
    // MOV [R16 + R24*2], RDX
    // R16 as base (B4=1), R24 as index (X3=1, X4=1)
    let code = [
        0xD5, 0x53, // REX2.WX3X4B4
        0x89, 0x14, 0x40, // MOV [SIB], r64: mod=00, reg=2, rm=100, SIB=scale*2 idx=0 base=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Arithmetic with EGPR
// ============================================================================

#[test]
fn test_add_r16_rax() {
    // ADD R16, RAX
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x01, 0xC0, // ADD r/m64, r64: mod=11, reg=0, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_add_r16_imm32() {
    // ADD R16, imm32 (sign-extended)
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x81, 0xC0, // ADD r/m64, imm32: opcode + mod=11, rm=0
        0x78, 0x56, 0x34, 0x12, // imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_sub_r24_r31() {
    // SUB R24, R31
    let code = [
        0xD5, 0x75, // REX2.WR3R4B3B4
        0x29, 0xF8, // SUB r/m64, r64: mod=11, reg=7 (R31), rm=0 (R24)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_imul_r16_r17() {
    // IMUL R16, R17
    let code = [
        0xD5, 0x95, // REX2.WR4B4 with M=1 for 0F map
        0xAF, 0xC1, // IMUL r64, r/m64: mod=11, reg=0 (R16), rm=1 (R17)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_xor_r16_r16() {
    // XOR R16, R16 (common idiom to zero register)
    let code = [
        0xD5, 0x15, // REX2.WR4B4
        0x31, 0xC0, // XOR r/m64, r64: mod=11, reg=0, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Shift/Rotate with EGPR
// ============================================================================

#[test]
fn test_shl_r16_cl() {
    // SHL R16, CL
    let code = [
        0xD5, 0x11, // REX2.WB4
        0xD3, 0xE0, // SHL r/m64, CL: opcode + mod=11, /4, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_shr_r24_imm8() {
    // SHR R24, 4
    let code = [
        0xD5, 0x51, // REX2.WB3B4
        0xC1, 0xE8, 0x04, // SHR r/m64, imm8: mod=11, /5, rm=0, imm8=4
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_ror_r16_1() {
    // ROR R16, 1
    let code = [
        0xD5, 0x11, // REX2.WB4
        0xD1, 0xC8, // ROR r/m64, 1: mod=11, /1, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// PUSH/POP with EGPR
// ============================================================================

#[test]
fn test_push_r16() {
    // PUSH R16
    // REX2 + 50+rd
    let code = [
        0xD5, 0x01, // REX2.B4 (no W needed for PUSH)
        0x50, // PUSH r64 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_pop_r16() {
    // POP R16
    // REX2 + 58+rd
    let code = [
        0xD5, 0x01, // REX2.B4
        0x58, // POP r64 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_push_r31() {
    // PUSH R31
    let code = [
        0xD5, 0x41, // REX2.B3B4
        0x57, // PUSH r64 (rd=7 -> R31 with B3B4)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// LEA with EGPR
// ============================================================================

#[test]
fn test_lea_r16_mem() {
    // LEA R16, [RAX + RBX*4 + 0x100]
    let code = [
        0xD5, 0x14, // REX2.WR4 (R4 for R16 as destination)
        0x8D, 0x84, 0x98, // LEA r64, [SIB+disp32]: mod=10, reg=0, rm=100
        0x00, 0x01, 0x00, 0x00, // disp32 = 0x100
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_lea_rax_r16_base() {
    // LEA RAX, [R16 + 0x50]
    let code = [
        0xD5, 0x11, // REX2.WB4 (B4 for R16 as base)
        0x8D, 0x40, 0x50, // LEA r64, [r/m+disp8]: mod=01, reg=0, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// INC/DEC with EGPR
// ============================================================================

#[test]
fn test_inc_r16() {
    // INC R16
    let code = [
        0xD5, 0x11, // REX2.WB4
        0xFF, 0xC0, // INC r/m64: mod=11, /0, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_dec_r24() {
    // DEC R24
    let code = [
        0xD5, 0x51, // REX2.WB3B4
        0xFF, 0xC8, // DEC r/m64: mod=11, /1, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// TEST/CMP with EGPR
// ============================================================================

#[test]
fn test_test_r16_r17() {
    // TEST R16, R17
    let code = [
        0xD5, 0x15, // REX2.WR4B4
        0x85, 0xC8, // TEST r/m64, r64: mod=11, reg=1, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cmp_r16_imm32() {
    // CMP R16, 0x12345678
    let code = [
        0xD5, 0x11, // REX2.WB4
        0x81, 0xF8, // CMP r/m64, imm32: mod=11, /7, rm=0
        0x78, 0x56, 0x34, 0x12, // imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// Complex EGPR operations
// ============================================================================

#[test]
fn test_xchg_r16_r17() {
    // XCHG R16, R17
    let code = [
        0xD5, 0x15, // REX2.WR4B4
        0x87, 0xC8, // XCHG r/m64, r64: mod=11, reg=1, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_cmpxchg_r16_r17() {
    // CMPXCHG R16, R17
    let code = [
        0xD5, 0x95, // REX2.WR4B4 with M=1 for 0F map
        0xB1, 0xC8, // CMPXCHG r/m64, r64: 0F B1 mod=11, reg=1, rm=0
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bsf_r16_r17() {
    // BSF R16, R17
    let code = [
        0xD5, 0x95, // REX2.WR4B4 with M=1
        0xBC, 0xC1, // BSF r64, r/m64: 0F BC mod=11, reg=0, rm=1
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_bsr_r24_r31() {
    // BSR R24, R31
    let code = [
        0xD5, 0xD5, // REX2.WR3R4B3B4 with M=1
        0xBD, 0xC7, // BSR r64, r/m64: 0F BD mod=11, reg=0, rm=7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// All EGPR registers (R16-R31) basic test
// ============================================================================

#[test]
fn test_egpr_all_registers_mov() {
    // Test MOV immediate to each EGPR
    // This is a comprehensive test touching all new registers
    let code = [
        // R16 (B4=1, rd=0)
        0xD5, 0x19, 0xB8, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R17 (B4=1, rd=1)
        0xD5, 0x19, 0xB9, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R18 (B4=1, rd=2)
        0xD5, 0x19, 0xBA, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R19 (B4=1, rd=3)
        0xD5, 0x19, 0xBB, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R20 (B4=1, rd=4)
        0xD5, 0x19, 0xBC, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R21 (B4=1, rd=5)
        0xD5, 0x19, 0xBD, 0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R22 (B4=1, rd=6)
        0xD5, 0x19, 0xBE, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R23 (B4=1, rd=7)
        0xD5, 0x19, 0xBF, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R24 (B3=1, B4=1, rd=0)
        0xD5, 0x59, 0xB8, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R25 (B3=1, B4=1, rd=1)
        0xD5, 0x59, 0xB9, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R26 (B3=1, B4=1, rd=2)
        0xD5, 0x59, 0xBA, 0x1A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R27 (B3=1, B4=1, rd=3)
        0xD5, 0x59, 0xBB, 0x1B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R28 (B3=1, B4=1, rd=4)
        0xD5, 0x59, 0xBC, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R29 (B3=1, B4=1, rd=5)
        0xD5, 0x59, 0xBD, 0x1D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R30 (B3=1, B4=1, rd=6)
        0xD5, 0x59, 0xBE, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // R31 (B3=1, B4=1, rd=7)
        0xD5, 0x59, 0xBF, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// EVEX MAP4 scalar data movement
// ============================================================================

#[test]
fn test_apx_movbe_reg_reg_64_match_llvm() {
    // LLVM 23 assembles "movbe r8, rax" as 62 d4 fc 08 61 c0.
    const FLAG_MASK: u64 = 0x8D5;
    let mut regs = Registers::default();
    regs.rax = 0x1122_3344_5566_7788;
    regs.r8 = 0;
    regs.rflags = 0x2 | FLAG_MASK;
    let code = [0x62, 0xD4, 0xFC, 0x08, 0x61, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x8877_6655_4433_2211);
    assert_eq!(regs.rax, 0x1122_3344_5566_7788);
    assert_eq!(regs.rflags & FLAG_MASK, FLAG_MASK);
}

#[test]
fn test_apx_movbe_reg_reg_32_zero_extends_match_llvm() {
    // LLVM 23 assembles "movbe r8d, eax" as 62 d4 7c 08 61 c0.
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_1122_3344;
    regs.r8 = u64::MAX;
    let code = [0x62, 0xD4, 0x7C, 0x08, 0x61, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x4433_2211);
}

#[test]
fn test_apx_movbe_reg_reg_16_preserves_upper_match_llvm() {
    // LLVM 23 assembles "movbe r8w, ax" as 62 d4 7d 08 61 c0.
    let mut regs = Registers::default();
    regs.rax = 0x1122;
    regs.r8 = 0xAABB_CCDD_EEFF_7788;
    let code = [0x62, 0xD4, 0x7D, 0x08, 0x61, 0xC0, 0xF4];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0xAABB_CCDD_EEFF_2211);
}
