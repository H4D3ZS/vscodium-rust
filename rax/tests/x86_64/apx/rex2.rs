//! APX REX2 Prefix Encoding Tests
//!
//! The REX2 prefix (0xD5) is a new prefix for APX that provides:
//! - Access to EGPR (R16-R31) in legacy instruction encodings
//! - Extended opcode map access (M bit)
//!
//! REX2 format: 0xD5 [M:R3:X3:B3:W:R4:X4:B4]
//!
//! Bit layout:
//! - M (bit 7): Map select - 0=legacy, 1=0F escape
//! - R3 (bit 6): REX.R equivalent
//! - X3 (bit 5): REX.X equivalent
//! - B3 (bit 4): REX.B equivalent
//! - W (bit 3): Operand size - 1=64-bit
//! - R4 (bit 2): Extended R bit for R16-R31
//! - X4 (bit 1): Extended X bit for R16-R31 (index)
//! - B4 (bit 0): Extended B bit for R16-R31 (base/rm)

use crate::common::*;

// ============================================================================
// REX2 with M=0 (Legacy opcode map)
// ============================================================================

#[test]
fn test_rex2_m0_b4_semantics_match_llvm() {
    // LLVM 23 decodes d5 18 89 c0 as: mov r16, rax.
    let mut regs = Registers::default();
    regs.rax = 0x1122_3344_5566_7788;
    let code = [
        0xD5, 0x18, 0x89, 0xC0, // MOV r16, rax
        0xF4,
    ];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r16, 0x1122_3344_5566_7788);
}

#[test]
fn test_rex2_prefix_survives_decode_cache_hit() {
    // Execute the same REX2 instruction twice at the same RIP. The second
    // execution uses the decode cache and must still target r16, not rax.
    let mut regs = Registers::default();
    regs.rax = 0xfeed_face_cafe_babe;
    regs.rcx = 2;
    let code = [
        0xD5, 0x18, 0x89, 0xC0, // MOV r16, rax
        0xE2, 0xFA, // LOOP back to the MOV once
        0xF4,
    ];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r16, 0xfeed_face_cafe_babe);
    assert_eq!(regs.rcx, 0);
}

#[test]
fn test_rex2_m1_dispatches_0f_map() {
    // LLVM 23 decodes d5 88 b6 c3 as: movzx rax, bl.
    let mut regs = Registers::default();
    regs.rbx = 0x1234_abcd;
    let code = [
        0xD5, 0x88, 0xB6, 0xC3, // MOVZX rax, bl via REX2.M 0F map
        0xF4,
    ];

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xcd);
}

#[test]
fn test_rex2_m0_basic() {
    // REX2 with M=0, all extension bits clear
    // MOV RAX, RBX (basic 64-bit move, W=1)
    let code = [
        0xD5, 0x08, // REX2: M=0 W=1 (just 64-bit operand size)
        0x89, 0xD8, // MOV r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m0_r3_set() {
    // REX2 with R3=1 for R8-R15 as reg operand
    // MOV RAX, R8
    let code = [
        0xD5, 0x48, // REX2: M=0 R3=1 W=1
        0x89, 0xC0, // MOV r/m64, r64: reg=0+R3*8=8 (R8)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m0_b3_set() {
    // REX2 with B3=1 for R8-R15 as rm operand
    // MOV R8, RAX
    let code = [
        0xD5, 0x18, // REX2: M=0 B3=1 W=1
        0x89, 0xC0, // MOV r/m64, r64: rm=0+B3*8=8 (R8)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m0_x3_set() {
    // REX2 with X3=1 for R8-R15 as SIB index
    // MOV RAX, [RBX + R8*4]
    let code = [
        0xD5, 0x28, // REX2: M=0 X3=1 W=1
        0x8B, 0x04, 0x83, // MOV r64, [SIB]: scale=4, idx=0+X3*8=8 (R8), base=3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with M=1 (0F opcode map - two-byte opcodes)
// ============================================================================

#[test]
fn test_rex2_m1_basic() {
    // REX2 with M=1 for 0F-prefixed instructions
    // MOVZX RAX, BL (0F B6)
    let code = [
        0xD5, 0x88, // REX2: M=1 W=1
        0xB6, 0xC3, // MOVZX r64, r/m8 (0F B6 with REX2)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m1_bswap() {
    // BSWAP R16 using REX2 with M=1
    let code = [
        0xD5, 0x89, // REX2: M=1 W=1 B4=1
        0xC8, // BSWAP r64 (0F C8+rd) - rd=0 with B4 -> R16
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m1_cmovcc() {
    // CMOVZ R16, R17 using REX2
    let code = [
        0xD5, 0x8D, // REX2: M=1 W=1 R4=1 B4=1
        0x44, 0xC1, // CMOVZ r64, r/m64 (0F 44): reg=0 (R16), rm=1 (R17)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_m1_setcc() {
    // SETZ R16L (8-bit register within R16)
    let code = [
        0xD5, 0x81, // REX2: M=1 B4=1 (no W for 8-bit)
        0x94, 0xC0, // SETZ r/m8 (0F 94): mod=11, rm=0 (R16B)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with R4 (EGPR destination)
// ============================================================================

#[test]
fn test_rex2_r4_alone() {
    // R4=1 alone (register 16-23 as reg field)
    // MOV R16, RAX (R16 as destination)
    let code = [
        0xD5, 0x0C, // REX2: W=1 R4=1
        0x8B, 0xC0, // MOV r64, r/m64: reg=0+R4*16=16 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_r3_r4_combined() {
    // R3=1, R4=1 for registers 24-31
    // MOV R24, RAX
    let code = [
        0xD5, 0x4C, // REX2: R3=1 W=1 R4=1
        0x8B, 0xC0, // MOV r64, r/m64: reg=0+R3*8+R4*16=24 (R24)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with B4 (EGPR base/rm)
// ============================================================================

#[test]
fn test_rex2_b4_alone() {
    // B4=1 alone (register 16-23 as rm/base field)
    // MOV RAX, R16
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x8B, 0xC0, // MOV r64, r/m64: rm=0+B4*16=16 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_b3_b4_combined() {
    // B3=1, B4=1 for registers 24-31 in rm field
    // MOV RAX, R24
    let code = [
        0xD5, 0x19, // REX2: B3=1 W=1 B4=1
        0x8B, 0xC0, // MOV r64, r/m64: rm=0+B3*8+B4*16=24 (R24)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with X4 (EGPR SIB index)
// ============================================================================

#[test]
fn test_rex2_x4_alone() {
    // X4=1 for EGPR as SIB index
    // MOV RAX, [RBX + R16*2]
    let code = [
        0xD5, 0x0A, // REX2: W=1 X4=1
        0x8B, 0x04, 0x43, // MOV r64, [SIB]: scale=2, idx=0+X4*16=16 (R16), base=3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_x3_x4_combined() {
    // X3=1, X4=1 for R24-R31 as SIB index
    // MOV RAX, [RBX + R24*8]
    let code = [
        0xD5, 0x2A, // REX2: X3=1 W=1 X4=1
        0x8B, 0x04, 0xC3, // MOV r64, [SIB]: scale=8, idx=0+X3*8+X4*16=24 (R24), base=3
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with all extension bits
// ============================================================================

#[test]
fn test_rex2_all_bits_set() {
    // All REX2 bits set: M=1 R3=1 X3=1 B3=1 W=1 R4=1 X4=1 B4=1
    // This encodes: 0F map, R31 as reg, R31 as index, R31 as base
    // IMUL R31, [R31 + R31*1]
    let code = [
        0xD5, 0xFF, // REX2: all bits set
        0xAF, 0x04, 0x3F, // IMUL r64, [SIB]: reg=7, scale=1, idx=7, base=7
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_no_w_32bit_operand() {
    // REX2 without W bit (32-bit operand size with EGPR)
    // MOV R16D, 0x12345678
    let code = [
        0xD5, 0x01, // REX2: B4=1 (no W)
        0xB8, 0x78, 0x56, 0x34, 0x12, // MOV r32, imm32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with memory operands
// ============================================================================

#[test]
fn test_rex2_mem_base_only() {
    // MOV RAX, [R16] - R16 as base
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x8B, 0x00, // MOV r64, [r/m64]: mod=00, rm=0 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_mem_base_disp8() {
    // MOV RAX, [R16 + 0x40]
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x8B, 0x40, 0x40, // MOV r64, [r/m64+disp8]: mod=01, rm=0, disp8=0x40
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_mem_base_disp32() {
    // MOV RAX, [R16 + 0x12345678]
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x8B, 0x80, // MOV r64, [r/m64+disp32]: mod=10, rm=0
        0x78, 0x56, 0x34, 0x12, // disp32
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_mem_sib_complex() {
    // MOV R16, [R24 + R31*4 + 0x100]
    // All EGPR in the encoding
    let code = [
        0xD5, 0x6D, // REX2: R3=0 X3=1 B3=1 W=1 R4=1 X4=1 B4=1
        0x8B, 0x84, 0xF8, // MOV r64, [SIB+disp32]: mod=10, reg=0, rm=100, SIB
        0x00, 0x01, 0x00, 0x00, // disp32 = 0x100
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with RIP-relative addressing
// ============================================================================

#[test]
fn test_rex2_rip_relative() {
    // MOV R16, [RIP + 0x100]
    let code = [
        0xD5, 0x0C, // REX2: W=1 R4=1
        0x8B, 0x05, // MOV r64, [RIP+disp32]: mod=00, reg=0 (R16), rm=101
        0x00, 0x01, 0x00, 0x00, // disp32 = 0x100
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 with various instruction forms
// ============================================================================

#[test]
fn test_rex2_push_pop() {
    // PUSH R16; POP R17
    let code = [
        0xD5, 0x01, 0x50, // REX2.B4 PUSH R16
        0xD5, 0x01, 0x59, // REX2.B4 POP R17 (58+1)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_xchg_rax() {
    // XCHG RAX, R16 (short form: 90+rd with REX2)
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x90, // XCHG RAX, r64 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_call_indirect() {
    // CALL R16 (indirect call through EGPR)
    let code = [
        0xD5, 0x01, // REX2: B4=1 (no W for CALL)
        0xFF, 0xD0, // CALL r/m64: mod=11, /2, rm=0 (R16)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_jmp_indirect() {
    // JMP R24 (indirect jump through EGPR)
    let code = [
        0xD5, 0x41, // REX2: B3=1 B4=1 (no W for JMP)
        0xFF, 0xE0, // JMP r/m64: mod=11, /4, rm=0 (R24)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_jmpabs_match_llvm() {
    // LLVM 23 assembles "jmpabs 0x1122334455667788" as d5 00 a1 imm64.
    let target = CODE_ADDR + 17;
    let mut code = vec![
        0xD5, 0x00, 0xA1, // JMPABS imm64
    ];
    code.extend_from_slice(&target.to_le_bytes());
    code.extend_from_slice(&[
        0xB8, 0x11, 0x11, 0x11, 0x11, // MOV eax, 0x11111111 (skipped)
        0xF4, 0xB8, 0x55, 0x66, 0x77, 0x88, // MOV eax, 0x88776655
        0xF4,
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8877_6655);
}

#[test]
fn test_rex2_jmpabs_ignores_w_bit() {
    // LLVM 23 disassembles d5 08 a1 imm64 as JMPABS too.
    let target = CODE_ADDR + 17;
    let mut code = vec![
        0xD5, 0x08, 0xA1, // JMPABS imm64 with REX2.W set
    ];
    code.extend_from_slice(&target.to_le_bytes());
    code.extend_from_slice(&[
        0xB8, 0x11, 0x11, 0x11, 0x11, // MOV eax, 0x11111111 (skipped)
        0xF4, 0xB8, 0x66, 0x77, 0x88, 0x99, // MOV eax, 0x99887766
        0xF4,
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x9988_7766);
}

#[test]
fn test_rex2_pushp_r16_match_llvm() {
    // LLVM 23 assembles "pushp r16" as d5 18 50.
    let code = [0xD5, 0x18, 0x50, 0xF4];
    let mut regs = Registers::default();
    regs.r16 = 0x1122_3344_5566_7788;

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR - 8);
    assert_eq!(read_mem_at_u64(&mem, STACK_ADDR - 8), 0x1122_3344_5566_7788);
}

#[test]
fn test_rex2_popp_r16_match_llvm() {
    // LLVM 23 assembles "popp r16" as d5 18 58.
    let code = [0xD5, 0x18, 0x58, 0xF4];
    let mut regs = Registers::default();
    regs.rsp = STACK_ADDR - 8;
    regs.r16 = 0xDEAD_BEEF;

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, STACK_ADDR - 8, 0x8877_6655_4433_2211);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsp, STACK_ADDR);
    assert_eq!(regs.r16, 0x8877_6655_4433_2211);
}

// ============================================================================
// REX2 cannot combine with REX/VEX/EVEX
// ============================================================================

#[test]
fn test_rex2_standalone() {
    // Verify REX2 works alone (these tests document expected behavior)
    // REX2 + instruction that would normally use REX
    let code = [
        0xD5, 0x09, // REX2: W=1 B4=1
        0x89, 0xC0, // MOV r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

// ============================================================================
// REX2 payload value tests
// ============================================================================

#[test]
fn test_rex2_payload_0x00() {
    // Minimal REX2 (all zeros) - should be valid but unusual
    let code = [
        0xD5, 0x00, // REX2: all bits zero
        0x89, 0xC0, // MOV r/m32, r32 (32-bit without W)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_payload_0x08() {
    // REX2 with just W=1 (64-bit mode)
    let code = [
        0xD5, 0x08, // REX2: W=1 only
        0x89, 0xC0, // MOV r/m64, r64
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}

#[test]
fn test_rex2_payload_0x80() {
    // REX2 with just M=1 (0F map)
    let code = [
        0xD5, 0x80, // REX2: M=1 only
        0xB6, 0xC0, // MOVZX r32, r/m8 (0F B6)
        0xF4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu);
}
