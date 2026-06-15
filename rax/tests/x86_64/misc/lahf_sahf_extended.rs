use crate::common::{af_set, cf_set, pf_set, run_until_hlt, setup_vm, sf_set, zf_set};
use rax::cpu::Registers;

// Comprehensive tests for LAHF and SAHF instructions
//
// LAHF - Load Status Flags into AH Register
// Loads SF, ZF, AF, PF, and CF into AH bits 7, 6, 4, 2, and 0 respectively
// Bit 1 is always set to 1
//
// SAHF - Store AH into Flags
// Loads SF, ZF, AF, PF, and CF from AH bits 7, 6, 4, 2, and 0 respectively
//
// These instructions provide compatibility with 8086 processors

// ============================================================================
// LAHF - Load AH with Flags
// Opcode: 9F
// ============================================================================

#[test]
fn test_lahf_basic() {
    let code = [
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH should contain flags, bit 1 should be set
    assert_ne!(regs.rax & 0xFF00, 0);
    // Bit 1 (0x02) should always be set
    assert_eq!((regs.rax >> 8) & 0x02, 0x02);
}

#[test]
fn test_lahf_all_flags_clear() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (clears most flags)
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF, clears CF, SF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // ZF should be set (bit 6)
    assert_ne!(ah & 0x40, 0);
    // CF should be clear (bit 0)
    assert_eq!(ah & 0x01, 0);
    // SF should be clear (bit 7)
    assert_eq!(ah & 0x80, 0);
}

#[test]
fn test_lahf_carry_flag_set() {
    let code = [
        0xf9, // STC (set carry flag)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // CF should be set (bit 0)
    assert_ne!(ah & 0x01, 0);
}

#[test]
fn test_lahf_carry_flag_clear() {
    let code = [
        0xf8, // CLC (clear carry flag)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // CF should be clear (bit 0)
    assert_eq!(ah & 0x01, 0);
}

#[test]
fn test_lahf_zero_flag_set() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // ZF should be set (bit 6)
    assert_ne!(ah & 0x40, 0);
}

#[test]
fn test_lahf_sign_flag_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // SF should be set (bit 7)
    assert_ne!(ah & 0x80, 0);
}

#[test]
fn test_lahf_parity_flag_even() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (0b11, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // PF should be set (bit 2)
    assert_ne!(ah & 0x04, 0);
}

#[test]
fn test_lahf_parity_flag_odd() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (0b111, odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears PF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // PF should be clear (bit 2)
    assert_eq!(ah & 0x04, 0);
}

#[test]
fn test_lahf_auxiliary_carry() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets AF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // AF should be set (bit 4)
    assert_ne!(ah & 0x10, 0);
}

#[test]
fn test_lahf_preserves_lower_byte() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL should still be 0x42
    assert_eq!(regs.rax & 0xFF, 0x42);
}

// ============================================================================
// SAHF - Store AH into Flags
// Opcode: 9E
// ============================================================================

#[test]
fn test_sahf_basic() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be affected by SAHF
    // With AH = 0, most flags should be clear
    assert!(!cf_set(regs.rflags));
    assert!(!sf_set(regs.rflags));
}

#[test]
fn test_sahf_set_carry() {
    let code = [
        0x9e, // SAHF (loads from AH)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0100; // AH = 0x01 (CF bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set
    assert!(cf_set(result.rflags));
}

#[test]
fn test_sahf_set_zero() {
    let code = [
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4000; // AH = 0x40 (ZF bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // ZF should be set
    assert!(zf_set(result.rflags));
}

#[test]
fn test_sahf_set_sign() {
    let code = [
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000; // AH = 0x80 (SF bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // SF should be set
    assert!(sf_set(result.rflags));
}

#[test]
fn test_sahf_set_parity() {
    let code = [
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0400; // AH = 0x04 (PF bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // PF should be set
    assert!(pf_set(result.rflags));
}

#[test]
fn test_sahf_set_auxiliary_carry() {
    let code = [
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1000; // AH = 0x10 (AF bit)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // AF should be set
    assert!(af_set(result.rflags));
}

#[test]
fn test_sahf_all_flags_set() {
    let code = [
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xD500; // AH = 0xD5 (SF, ZF, AF, PF, CF all set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // All 5 flags should be set
    assert!(sf_set(result.rflags));
    assert!(zf_set(result.rflags));
    assert!(af_set(result.rflags));
    assert!(pf_set(result.rflags));
    assert!(cf_set(result.rflags));
}

#[test]
fn test_sahf_clear_all_flags() {
    let code = [
        0xf9, // STC (set carry)
        0x9e, // SAHF (AH = 0, clears flags)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // AH = 0x00
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // All 5 flags should be clear
    assert!(!sf_set(result.rflags));
    assert!(!zf_set(result.rflags));
    assert!(!af_set(result.rflags));
    assert!(!pf_set(result.rflags));
    assert!(!cf_set(result.rflags));
}

// ============================================================================
// LAHF and SAHF together - Round-trip tests
// ============================================================================

#[test]
fn test_lahf_sahf_roundtrip() {
    let code = [
        0xf9, // STC (set carry)
        0x9f, // LAHF (save flags to AH)
        0xf8, // CLC (clear carry)
        0x9e, // SAHF (restore flags from AH)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set again (restored by SAHF)
    assert!(cf_set(regs.rflags));
}

#[test]
fn test_sahf_lahf_roundtrip() {
    let code = [
        0x9e, // SAHF (load flags from AH)
        0x9f, // LAHF (save flags to AH)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4500; // AH = 0x45 (ZF, PF, CF set)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    let ah = (result.rax >> 8) & 0xFF;
    // AH should contain the same flags (with bit 1 set)
    assert_ne!(ah & 0x40, 0); // ZF
    assert_ne!(ah & 0x04, 0); // PF
    assert_ne!(ah & 0x01, 0); // CF
}

#[test]
fn test_lahf_sahf_preserve_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x9f, // LAHF
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Other registers should be unchanged
    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rcx, 0x99);
}

#[test]
fn test_lahf_multiple_times() {
    let code = [
        0xf9, // STC
        0x9f, // LAHF
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save first LAHF)
        0xf8, // CLC
        0x9f, // LAHF
        0x48, 0x89, 0xc1, // MOV RCX, RAX (save second LAHF)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah1 = (regs.rbx >> 8) & 0xFF;
    let ah2 = (regs.rcx >> 8) & 0xFF;

    // First LAHF should have CF set
    assert_ne!(ah1 & 0x01, 0);
    // Second LAHF should have CF clear
    assert_eq!(ah2 & 0x01, 0);
}

#[test]
fn test_sahf_multiple_times() {
    let code = [
        0x9e, // SAHF (AH with CF set)
        0x9e, // SAHF again
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0100; // AH = 0x01 (CF)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // CF should still be set
    assert!(cf_set(result.rflags));
}

#[test]
fn test_lahf_sahf_complex_flags() {
    let code = [
        // Set complex flag state
        0x48, 0xc7, 0xc0, 0x5f, 0x00, 0x00, 0x00, // MOV RAX, 0x5F
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets PF and AF)
        0x9f, // LAHF
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save AH)
        // Clear flags
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0x89, 0xd8, // MOV RAX, RBX (restore AH)
        0x9e, // SAHF (restore flags)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be restored
    assert!(pf_set(regs.rflags));
}

#[test]
fn test_lahf_doesnt_modify_rflags() {
    let code = [
        0xf9, // STC
        0x48, 0x9c, // PUSHFQ (save flags)
        0x9f, // LAHF
        0x48, 0x9c, // PUSHFQ (save flags again)
        0x58, // POP RAX (second flags)
        0x5b, // POP RBX (first flags)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RFLAGS should be unchanged by LAHF
    assert_eq!(regs.rax & 0xD5, regs.rbx & 0xD5); // Check SF, ZF, AF, PF, CF
}

#[test]
fn test_sahf_only_affects_5_flags() {
    let code = [
        0x48, 0x9c, // PUSHFQ
        0x58, // POP RAX
        0x48, 0x0d, 0x00, 0x08, 0x00, 0x00, // OR RAX, 0x800 (set OF)
        0x50, // PUSH RAX
        0x48, 0x9d, // POPFQ (set OF)
        0x9e, // SAHF (should not affect OF)
        0x48, 0x9c, // PUSHFQ
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // AH = 0 (clear SF, ZF, AF, PF, CF)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let result = run_until_hlt(&mut vcpu).unwrap();

    // OF should still be set (bit 11)
    assert_ne!(result.rax & 0x800, 0);
}

#[test]
fn test_lahf_bit1_always_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clear most flags)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // Bit 1 should always be set
    assert_eq!(ah & 0x02, 0x02);
}

#[test]
fn test_lahf_sahf_with_arithmetic() {
    let code = [
        // Perform arithmetic to set flags
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF, CF, PF, AF)
        0x9f, // LAHF
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        // Clear RAX and restore flags
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be restored
    assert!(zf_set(regs.rflags));
    assert!(cf_set(regs.rflags));
}

#[test]
fn test_sahf_with_subtraction() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x29, 0xd8, // SUB RAX, RBX (5 - 10 = -5, sets SF, CF)
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = (regs.rax >> 8) & 0xFF;
    // SF should be set (bit 7)
    assert_ne!(ah & 0x80, 0);
    // CF should be set (bit 0)
    assert_ne!(ah & 0x01, 0);
}

#[test]
fn test_lahf_preserves_al_value() {
    let code = [
        0x48, 0xc7, 0xc0, 0xab, 0x00, 0x00, 0x00, // MOV RAX, 0xAB
        0xf9, // STC
        0x9f, // LAHF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AL should still be 0xAB
    assert_eq!(regs.rax & 0xFF, 0xAB);
    // AH should contain flags
    assert_ne!((regs.rax >> 8) & 0xFF, 0);
}
