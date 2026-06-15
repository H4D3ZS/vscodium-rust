use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// Comprehensive tests for all conditional jump instructions
// Tests all 30+ conditional jump mnemonics including aliases
// Based on documentation from /Users/int/dev/rax/docs/jcc.txt

// ============================================================================
// JA / JNBE - Jump if Above / Not Below or Equal
// Condition: CF=0 AND ZF=0 (unsigned: a > b)
// ============================================================================

#[test]
fn test_ja_short_taken() {
    // JA target sets RCX=0xA1; the skipped path sets RCX=0xBAD.
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 8: CF=0, ZF=0)
        0x77, 0x07, // JA +7 (should jump, skips MOV RCX,0xBAD)
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD (not executed)
        0x48, 0xc7, 0xc1, 0xa1, 0x00, 0x00, 0x00, // MOV RCX, 0xA1 (target)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xA1, "JA should be taken (CF=0,ZF=0)");
}

#[test]
fn test_ja_short_not_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (8 == 8: ZF=1)
        0x77, 0x05, // JA +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x42);
}

#[test]
fn test_jnbe_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (32 > 16)
        0x77, 0x02, // JNBE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JAE / JNB / JNC - Jump if Above or Equal / Not Below / Not Carry
// Condition: CF=0 (unsigned: a >= b)
// ============================================================================

#[test]
fn test_jae_short_taken_above() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 8: CF=0)
        0x73, 0x02, // JAE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jae_short_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (8 == 8: CF=0)
        0x73, 0x02, // JAE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jae_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16: CF=1)
        0x73, 0x05, // JAE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x99);
}

#[test]
fn test_jnb_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x73, 0x02, // JNB +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jnc_short_taken() {
    let code = [
        0xf8, // CLC (clear carry flag)
        0x73, 0x02, // JNC +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JB / JNAE / JC - Jump if Below / Not Above or Equal / Carry
// Condition: CF=1 (unsigned: a < b)
// ============================================================================

#[test]
fn test_jb_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16: CF=1)
        0x72, 0x02, // JB +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jb_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 5: CF=0)
        0x72, 0x05, // JB +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x33);
}

#[test]
fn test_jnae_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x72, 0x02, // JNAE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jc_short_taken() {
    let code = [
        0xf9, // STC (set carry flag)
        0x72, 0x02, // JC +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JBE / JNA - Jump if Below or Equal / Not Above
// Condition: CF=1 OR ZF=1 (unsigned: a <= b)
// ============================================================================

#[test]
fn test_jbe_short_taken_below() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16: CF=1)
        0x76, 0x02, // JBE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jbe_short_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: ZF=1)
        0x76, 0x02, // JBE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jbe_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (32 > 16: CF=0, ZF=0)
        0x76, 0x05, // JBE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x77, 0x00, 0x00, 0x00, // MOV RCX, 0x77
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x77);
}

#[test]
fn test_jna_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x76, 0x02, // JNA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JE / JZ - Jump if Equal / Zero
// Condition: ZF=1
// ============================================================================

#[test]
fn test_je_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: ZF=1)
        0x74, 0x02, // JE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_je_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 != 8: ZF=0)
        0x74, 0x05, // JE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x55, 0x00, 0x00, 0x00, // MOV RCX, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x55);
}

#[test]
fn test_jz_short_taken() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF=1)
        0x74, 0x02, // JZ +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JG / JNLE - Jump if Greater / Not Less or Equal
// Condition: ZF=0 AND SF=OF (signed: a > b)
// ============================================================================

#[test]
fn test_jg_short_taken_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 5: ZF=0, SF=OF=0)
        0x7f, 0x02, // JG +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jg_short_not_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: ZF=1)
        0x7f, 0x05, // JG +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x88, 0x00, 0x00, 0x00, // MOV RCX, 0x88
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x88);
}

#[test]
fn test_jnle_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7f, 0x02, // JNLE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JGE / JNL - Jump if Greater or Equal / Not Less
// Condition: SF=OF (signed: a >= b)
// ============================================================================

#[test]
fn test_jge_short_taken_greater() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 5: SF=OF=0)
        0x7d, 0x02, // JGE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jge_short_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: SF=OF=0)
        0x7d, 0x02, // JGE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jnl_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x02, // JNL +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JL / JNGE - Jump if Less / Not Greater or Equal
// Condition: SF!=OF (signed: a < b)
// ============================================================================

#[test]
fn test_jl_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16: SF!=OF)
        0x7c, 0x02, // JL +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jl_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 5: SF=OF)
        0x7c, 0x05, // JL +5 (should not jump)
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xaa
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xaa);
}

#[test]
fn test_jnge_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x0c, 0x00, 0x00, 0x00, // MOV RBX, 12
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7c, 0x02, // JNGE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JLE / JNG - Jump if Less or Equal / Not Greater
// Condition: ZF=1 OR SF!=OF (signed: a <= b)
// ============================================================================

#[test]
fn test_jle_short_taken_less() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16: SF!=OF)
        0x7e, 0x02, // JLE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jle_short_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: ZF=1)
        0x7e, 0x02, // JLE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jle_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (32 > 16: ZF=0, SF=OF)
        0x7e, 0x05, // JLE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xbb
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xbb);
}

#[test]
fn test_jng_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7e, 0x02, // JNG +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JNE / JNZ - Jump if Not Equal / Not Zero
// Condition: ZF=0
// ============================================================================

#[test]
fn test_jne_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 != 8: ZF=0)
        0x75, 0x02, // JNE +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jne_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16: ZF=1)
        0x75, 0x05, // JNE +5 (should not jump)
        0x48, 0xc7, 0xc1, 0xcc, 0x00, 0x00, 0x00, // MOV RCX, 0xcc
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xcc);
}

#[test]
fn test_jnz_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (ZF=0)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x02, // JNZ +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JO - Jump if Overflow
// Condition: OF=1
// ============================================================================

#[test]
fn test_jo_short_taken() {
    // Must use 64-bit max signed value for 64-bit overflow with REX.W ADD
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (64-bit overflow, sets OF=1)
        0x70, 0x07, // JO +7 (should jump over the MOV RCX, 0)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (should not execute)
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xaa (jump target)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xaa); // Verify jump was taken
}

#[test]
fn test_jo_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5 (no overflow)
        0x70, 0x05, // JO +5 (should not jump)
        0x48, 0xc7, 0xc1, 0xdd, 0x00, 0x00, 0x00, // MOV RCX, 0xdd
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xdd);
}

// ============================================================================
// JNO - Jump if Not Overflow
// Condition: OF=0
// ============================================================================

#[test]
fn test_jno_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5 (no overflow)
        0x71, 0x02, // JNO +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jno_short_not_taken() {
    // Must use 64-bit max signed value (0x7FFFFFFFFFFFFFFF) for 64-bit overflow
    // 0x7FFFFFFF only causes 32-bit overflow, but with REX.W the ADD is 64-bit
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (64-bit overflow, sets OF=1)
        0x71, 0x05, // JNO +5 (should not jump because OF=1)
        0x48, 0xc7, 0xc1, 0xee, 0x00, 0x00, 0x00, // MOV RCX, 0xee
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xee);
}

// ============================================================================
// JS - Jump if Sign
// Condition: SF=1
// ============================================================================

#[test]
fn test_js_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (signed)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1)
        0x78, 0x02, // JS +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_js_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16 (positive)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x78, 0x05, // JS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xff
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xff);
}

// ============================================================================
// JNS - Jump if Not Sign
// Condition: SF=0
// ============================================================================

#[test]
fn test_jns_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16 (positive)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x79, 0x02, // JNS +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jns_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=1)
        0x79, 0x05, // JNS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x11);
}

// ============================================================================
// JP / JPE - Jump if Parity / Parity Even
// Condition: PF=1 (even number of 1 bits in low byte)
// ============================================================================

#[test]
fn test_jp_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (0b11, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7a, 0x02, // JP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jp_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (0b111, odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=0)
        0x7a, 0x05, // JP +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x22);
}

#[test]
fn test_jpe_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15 (0b1111, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x02, // JPE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// JNP / JPO - Jump if Not Parity / Parity Odd
// Condition: PF=0 (odd number of 1 bits in low byte)
// ============================================================================

#[test]
fn test_jnp_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (0b111, odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jnp_short_not_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=1)
        0x7b, 0x05, // JNP +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x33);
}

#[test]
fn test_jpo_short_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x02, // JPO +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Near jumps (32-bit displacement) - using 0x0F prefix
// ============================================================================

#[test]
fn test_ja_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (32 > 16)
        0x0f, 0x87, 0x02, 0x00, 0x00, 0x00, // JA +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jae_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 >= 16)
        0x0f, 0x83, 0x02, 0x00, 0x00, 0x00, // JAE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jb_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16)
        0x0f, 0x82, 0x02, 0x00, 0x00, 0x00, // JB +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jbe_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 <= 16)
        0x0f, 0x86, 0x02, 0x00, 0x00, 0x00, // JBE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_je_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 == 16)
        0x0f, 0x84, 0x02, 0x00, 0x00, 0x00, // JE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jg_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x20, 0x00, 0x00, 0x00, // MOV RAX, 32
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (32 > 16)
        0x0f, 0x8f, 0x02, 0x00, 0x00, 0x00, // JG +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jge_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 >= 16)
        0x0f, 0x8d, 0x02, 0x00, 0x00, 0x00, // JGE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jl_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 < 16)
        0x0f, 0x8c, 0x02, 0x00, 0x00, 0x00, // JL +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jle_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 <= 16)
        0x0f, 0x8e, 0x02, 0x00, 0x00, 0x00, // JLE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jne_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 != 8)
        0x0f, 0x85, 0x02, 0x00, 0x00, 0x00, // JNE +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jo_near_taken() {
    // Must use 64-bit max signed value for 64-bit overflow with REX.W ADD
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (64-bit overflow, sets OF=1)
        0x0f, 0x80, 0x07, 0x00, 0x00, 0x00, // JO +7 (near, should jump over MOV RCX, 0)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (should not execute)
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xbb (jump target)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xbb); // Verify jump was taken
}

#[test]
fn test_jno_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x83, 0xc0, 0x05, // ADD RAX, 5 (no overflow)
        0x0f, 0x81, 0x02, 0x00, 0x00, 0x00, // JNO +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_js_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=1)
        0x0f, 0x88, 0x02, 0x00, 0x00, 0x00, // JS +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jns_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x0f, 0x89, 0x02, 0x00, 0x00, 0x00, // JNS +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jp_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=1)
        0x0f, 0x8a, 0x02, 0x00, 0x00, 0x00, // JP +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_jnp_near_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=0)
        0x0f, 0x8b, 0x02, 0x00, 0x00, 0x00, // JNP +2 (near)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional edge case tests
// ============================================================================

#[test]
fn test_backward_jump_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        // loop start (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xc3, // CMP RBX, RAX
        0x72, 0xf7, // JB -9 (loop while RBX < RAX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 5);
}

#[test]
fn test_multiple_conditions_in_sequence() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (16 > 8)
        0x72, 0x05, // JB +5 (should not jump)
        0x76, 0x03, // JBE +3 (should not jump)
        0x77, 0x01, // JA +1 (should jump)
        0xf4, // HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_zero_offset_jump() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x39, 0xd8, // CMP RAX, RBX (equal)
        0x74, 0x00, // JE +0 (should jump to next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Strengthened: deterministic taken / not-taken assertions for every Jcc.
//
// Each helper preloads RFLAGS directly (so we control CF/ZF/SF/OF/PF exactly),
// then executes a single short Jcc whose target sets RCX=0xACED and whose
// fall-through path sets RCX=0xFA11. We assert the exact RCX sentinel AND the
// final RIP, proving both the branch decision and the displacement arithmetic.
// ============================================================================

const FLAG_CF: u64 = 0x0001;
const FLAG_PF: u64 = 0x0004;
const FLAG_ZF: u64 = 0x0040;
const FLAG_SF: u64 = 0x0080;
const FLAG_OF: u64 = 0x0800;

const TAKEN_SENTINEL: u64 = 0xACED;
const FALL_SENTINEL: u64 = 0xFA11;

/// Run a single 1-byte-opcode short Jcc (opcode `op`) with the given initial
/// RFLAGS, returning (final RCX, final RIP). Layout:
///   0x1000: Jcc +8                    (2 bytes)  -> target = 0x1002 + 8 = 0x100A
///   0x1002: MOV RCX, 0xFA11           (7 bytes)  fall-through sentinel
///   0x1009: HLT                       fall-through stop -> RIP = 0x100A
///   0x100A: MOV RCX, 0xACED           (7 bytes)  taken sentinel
///   0x1011: HLT                       taken stop      -> RIP = 0x1012
fn run_jcc_short(op: u8, flags: u64) -> (u64, u64) {
    let code = [
        op, 0x08, // Jcc +8
        0x48, 0xc7, 0xc1, 0x11, 0xfa, 0x00, 0x00, // MOV RCX, 0xFA11
        0xf4, // HLT (fall-through stop)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT (taken stop)
    ];
    let mut regs = Registers::default();
    regs.rflags = flags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let out = run_until_hlt(&mut vcpu).unwrap();
    (out.rcx, out.rip)
}

fn assert_taken(op: u8, flags: u64, msg: &str) {
    let (rcx, rip) = run_jcc_short(op, flags);
    assert_eq!(
        rcx, TAKEN_SENTINEL,
        "Jcc {:#04x} should be TAKEN: {}",
        op, msg
    );
    assert_eq!(rip, 0x1012, "taken RIP for {:#04x}: {}", op, msg);
}

fn assert_not_taken(op: u8, flags: u64, msg: &str) {
    let (rcx, rip) = run_jcc_short(op, flags);
    assert_eq!(
        rcx, FALL_SENTINEL,
        "Jcc {:#04x} should NOT be taken: {}",
        op, msg
    );
    assert_eq!(rip, 0x100A, "not-taken RIP for {:#04x}: {}", op, msg);
}

#[test]
fn test_jcc_jo_taken_and_not() {
    assert_taken(0x70, FLAG_OF, "OF=1");
    assert_not_taken(0x70, 0, "OF=0");
}

#[test]
fn test_jcc_jno_taken_and_not() {
    assert_taken(0x71, 0, "OF=0");
    assert_not_taken(0x71, FLAG_OF, "OF=1");
}

#[test]
fn test_jcc_jb_jc_taken_and_not() {
    // JB/JC/JNAE = 0x72, condition CF=1
    assert_taken(0x72, FLAG_CF, "CF=1");
    assert_not_taken(0x72, 0, "CF=0");
}

#[test]
fn test_jcc_jae_jnc_taken_and_not() {
    // JAE/JNB/JNC = 0x73, condition CF=0
    assert_taken(0x73, 0, "CF=0");
    assert_not_taken(0x73, FLAG_CF, "CF=1");
}

#[test]
fn test_jcc_je_jz_taken_and_not() {
    // JE/JZ = 0x74, condition ZF=1
    assert_taken(0x74, FLAG_ZF, "ZF=1");
    assert_not_taken(0x74, 0, "ZF=0");
}

#[test]
fn test_jcc_jne_jnz_taken_and_not() {
    // JNE/JNZ = 0x75, condition ZF=0
    assert_taken(0x75, 0, "ZF=0");
    assert_not_taken(0x75, FLAG_ZF, "ZF=1");
}

#[test]
fn test_jcc_jbe_jna_taken_and_not() {
    // JBE/JNA = 0x76, condition CF=1 OR ZF=1
    assert_taken(0x76, FLAG_CF, "CF=1");
    assert_taken(0x76, FLAG_ZF, "ZF=1");
    assert_taken(0x76, FLAG_CF | FLAG_ZF, "CF=1,ZF=1");
    assert_not_taken(0x76, 0, "CF=0,ZF=0");
}

#[test]
fn test_jcc_ja_jnbe_taken_and_not() {
    // JA/JNBE = 0x77, condition CF=0 AND ZF=0
    assert_taken(0x77, 0, "CF=0,ZF=0");
    assert_not_taken(0x77, FLAG_CF, "CF=1");
    assert_not_taken(0x77, FLAG_ZF, "ZF=1");
    assert_not_taken(0x77, FLAG_CF | FLAG_ZF, "CF=1,ZF=1");
}

#[test]
fn test_jcc_js_taken_and_not() {
    // JS = 0x78, condition SF=1
    assert_taken(0x78, FLAG_SF, "SF=1");
    assert_not_taken(0x78, 0, "SF=0");
}

#[test]
fn test_jcc_jns_taken_and_not() {
    // JNS = 0x79, condition SF=0
    assert_taken(0x79, 0, "SF=0");
    assert_not_taken(0x79, FLAG_SF, "SF=1");
}

#[test]
fn test_jcc_jp_jpe_taken_and_not() {
    // JP/JPE = 0x7A, condition PF=1
    assert_taken(0x7A, FLAG_PF, "PF=1");
    assert_not_taken(0x7A, 0, "PF=0");
}

#[test]
fn test_jcc_jnp_jpo_taken_and_not() {
    // JNP/JPO = 0x7B, condition PF=0
    assert_taken(0x7B, 0, "PF=0");
    assert_not_taken(0x7B, FLAG_PF, "PF=1");
}

#[test]
fn test_jcc_jl_jnge_taken_and_not() {
    // JL/JNGE = 0x7C, condition SF != OF
    assert_taken(0x7C, FLAG_SF, "SF=1,OF=0");
    assert_taken(0x7C, FLAG_OF, "SF=0,OF=1");
    assert_not_taken(0x7C, 0, "SF=0,OF=0");
    assert_not_taken(0x7C, FLAG_SF | FLAG_OF, "SF=1,OF=1");
}

#[test]
fn test_jcc_jge_jnl_taken_and_not() {
    // JGE/JNL = 0x7D, condition SF == OF
    assert_taken(0x7D, 0, "SF=0,OF=0");
    assert_taken(0x7D, FLAG_SF | FLAG_OF, "SF=1,OF=1");
    assert_not_taken(0x7D, FLAG_SF, "SF=1,OF=0");
    assert_not_taken(0x7D, FLAG_OF, "SF=0,OF=1");
}

#[test]
fn test_jcc_jle_jng_taken_and_not() {
    // JLE/JNG = 0x7E, condition ZF=1 OR SF != OF
    assert_taken(0x7E, FLAG_ZF, "ZF=1");
    assert_taken(0x7E, FLAG_SF, "SF != OF");
    assert_taken(0x7E, FLAG_OF, "SF != OF");
    assert_not_taken(0x7E, 0, "ZF=0,SF=OF");
    assert_not_taken(0x7E, FLAG_SF | FLAG_OF, "ZF=0,SF=OF");
}

#[test]
fn test_jcc_jg_jnle_taken_and_not() {
    // JG/JNLE = 0x7F, condition ZF=0 AND SF == OF
    assert_taken(0x7F, 0, "ZF=0,SF=OF");
    assert_taken(0x7F, FLAG_SF | FLAG_OF, "ZF=0,SF=OF=1");
    assert_not_taken(0x7F, FLAG_ZF, "ZF=1");
    assert_not_taken(0x7F, FLAG_SF, "SF != OF");
}

/// Near (0F 8x, rel32) variant: target at +8, fall-through sentinel + HLT between.
///   0x1000: 0F 8x rel32(+8)           (6 bytes) -> target = 0x1006 + 8 = 0x100E
///   0x1006: MOV RCX, 0xFA11           (7 bytes)
///   0x100D: HLT                       fall-through stop -> RIP = 0x100E
///   0x100E: MOV RCX, 0xACED           (7 bytes)
///   0x1015: HLT                       taken stop -> RIP = 0x1016
fn run_jcc_near(op2: u8, flags: u64) -> (u64, u64) {
    let code = [
        0x0f, op2, 0x08, 0x00, 0x00, 0x00, // Jcc near +8
        0x48, 0xc7, 0xc1, 0x11, 0xfa, 0x00, 0x00, // MOV RCX, 0xFA11
        0xf4, // HLT (fall-through stop)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT (taken stop)
    ];
    let mut regs = Registers::default();
    regs.rflags = flags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let out = run_until_hlt(&mut vcpu).unwrap();
    (out.rcx, out.rip)
}

#[test]
fn test_jcc_near_displacement_taken_and_not() {
    // 0F 84 = JE near, 0F 85 = JNE near. Verify rel32 displacement + RIP.
    let (rcx_t, rip_t) = run_jcc_near(0x84, FLAG_ZF);
    assert_eq!(rcx_t, TAKEN_SENTINEL, "JE near taken (ZF=1)");
    assert_eq!(rip_t, 0x1016, "JE near taken RIP past HLT");

    let (rcx_n, rip_n) = run_jcc_near(0x84, 0);
    assert_eq!(rcx_n, FALL_SENTINEL, "JE near not taken (ZF=0)");
    assert_eq!(rip_n, 0x100E, "JE near not-taken RIP at taken target start");

    let (rcx_t2, _) = run_jcc_near(0x85, 0);
    assert_eq!(rcx_t2, TAKEN_SENTINEL, "JNE near taken (ZF=0)");
    let (rcx_n2, _) = run_jcc_near(0x85, FLAG_ZF);
    assert_eq!(rcx_n2, FALL_SENTINEL, "JNE near not taken (ZF=1)");
}

#[test]
fn test_jmp_forward_rel8_lands_exactly() {
    // Validate that a forward JMP rel8 lands exactly on the HLT, skipping the MOV.
    //   0x1000: XOR RCX,RCX           (3)
    //   0x1003: JMP +7                (2) -> 0x1005 + 7 = 0x100C
    //   0x1005: MOV RCX,0xACED        (7) ends 0x100C  (skipped)
    //   0x100C: HLT                   -> RIP past HLT = 0x100D
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX,RCX
        0xeb, 0x07, // JMP +7
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX,0xACED (skipped)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "forward JMP skipped the MOV, RCX stays 0");
    assert_eq!(regs.rip, 0x100D, "RIP is past the HLT at 0x100C");
}
