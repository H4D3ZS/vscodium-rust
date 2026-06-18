// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// LAHF/SAHF - Load/Store AH from/to Flags (Comprehensive Extended Tests)
// These instructions provide a quick way to save and restore the low byte of RFLAGS
// using the AH register.
//
// LAHF copies SF:ZF:0:AF:0:PF:1:CF into AH
// SAHF copies AH into SF:ZF:0:AF:0:PF:1:CF
//
// Opcodes:
// 9F           LAHF   - Load AH from flags (SF:ZF:0:AF:0:PF:1:CF)
// 9E           SAHF   - Store AH to flags (SF:ZF:0:AF:0:PF:1:CF)
//
// Note: In 64-bit mode, LAHF/SAHF may not be available on all CPUs by default,
// but modern CPUs support them. Bit pattern in AH: SF ZF 0 AF 0 PF 1 CF
// Positions:                                         7  6  5  4  3  2  1  0

#[test]
fn test_lahf_basic() {
    // LAHF - Load flags into AH
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    // Set some flags: CF=1, PF=1, AF=0, ZF=0, SF=1
    regs.rflags = 0x87; // 10000111 = SF, PF, CF set, bit 1 always 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // AH should contain SF:ZF:0:AF:0:PF:1:CF
    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    assert_eq!(
        ah, 0x87,
        "AH should contain flags: SF=1, ZF=0, AF=0, PF=1, CF=1"
    );
}

#[test]
fn test_sahf_basic() {
    // SAHF - Store AH to flags
    let code = [
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    // Set AH to a specific pattern: SF=1, ZF=0, AF=0, PF=1, CF=1
    regs.rax = 0x8500; // AH = 0x85
    regs.rflags = 0x02; // Clear all relevant flags except bit 1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Check that flags were set from AH
    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(pf_set(regs.rflags), "PF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
    assert!(!zf_set(regs.rflags), "ZF should not be set");
    assert!(!af_set(regs.rflags), "AF should not be set");
}

#[test]
fn test_lahf_sahf_round_trip() {
    // LAHF followed by SAHF should preserve flags
    let code = [
        0x9f, // LAHF
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0xC7; // SF=1, ZF=1, AF=1, PF=1, CF=1, bit 1=1
    let expected_flags = regs.rflags & 0xD5; // Mask for LAHF/SAHF relevant bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be preserved (for the relevant bits)
    assert_eq!(regs.rflags & 0xD5, expected_flags);
}

#[test]
fn test_lahf_all_flags_set() {
    // LAHF with all relevant flags set
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    // Set all LAHF-relevant flags: SF, ZF, AF, PF, CF
    regs.rflags = 0xD7; // 11010111 = SF, ZF, AF, PF, bit 1, CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    assert_eq!(ah, 0xD7, "AH should have all relevant flags set");
}

#[test]
fn test_lahf_all_flags_clear() {
    // LAHF with all relevant flags clear (except bit 1 which is always 1)
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x02; // Only bit 1 set (reserved, always 1)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    assert_eq!(ah, 0x02, "AH should be 0x02 (only bit 1 set)");
}

#[test]
fn test_sahf_all_flags_set() {
    // SAHF with all flags set in AH
    let code = [
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xD700; // AH = 0xD7 (all relevant flags)
    regs.rflags = 0x02;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(af_set(regs.rflags), "AF should be set");
    assert!(pf_set(regs.rflags), "PF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_sahf_all_flags_clear() {
    // SAHF with all flags clear in AH (except bit 1)
    let code = [
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0200; // AH = 0x02 (only bit 1)
    regs.rflags = 0xFF; // All flags initially set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(!sf_set(regs.rflags), "SF should not be set");
    assert!(!zf_set(regs.rflags), "ZF should not be set");
    assert!(!af_set(regs.rflags), "AF should not be set");
    assert!(!pf_set(regs.rflags), "PF should not be set");
    assert!(!cf_set(regs.rflags), "CF should not be set");
}

#[test]
fn test_lahf_preserves_rax_lower_byte() {
    // LAHF only affects AH, not AL
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678ABCDEF42; // AL = 0x42
    regs.rflags = 0x87;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL should be preserved");
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0x12345678ABCD0000,
        "Upper bytes preserved"
    );
}

#[test]
fn test_sahf_preserves_rax() {
    // SAHF doesn't modify RAX
    let code = [
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDE85;
    let initial_rax = regs.rax;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, initial_rax, "RAX should be unchanged");
}

#[test]
fn test_lahf_only_affects_status_flags() {
    // LAHF should only copy status flags, not other RFLAGS bits
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    // Set some non-status flags (OF, DF, IF, etc.)
    regs.rflags = 0x8CD7; // Include overflow, direction, interrupt flags
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    // AH should only contain the low 8 bits of flags
    assert_eq!(ah, 0xD7, "AH should only contain lower byte of flags");
}

#[test]
fn test_sahf_only_affects_status_flags() {
    // SAHF should only modify status flags, not other RFLAGS bits
    let code = [
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFF00; // AH = 0xFF (all bits set)
    // Set some upper flags that should be preserved
    regs.rflags = 0x0800; // Overflow flag (bit 11)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Overflow flag should still be set
    assert!(of_set(regs.rflags), "OF should be preserved");
    // Status flags should be set
    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_lahf_sahf_save_restore_pattern() {
    // Common pattern: save flags, do operation, restore flags
    let code = [
        0x9f, // LAHF (save flags)
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1 (modifies flags)
        0x9e, // SAHF (restore flags)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000; // AH will receive flags
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    regs.rflags = 0x42; // Specific flag pattern
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be restored to original (within LAHF/SAHF bits)
    assert_eq!(regs.rflags & 0xD5, 0x42 & 0xD5);
}

#[test]
fn test_lahf_individual_flags() {
    // Test individual flag patterns
    let test_cases = vec![
        (0x01, "CF only"), // 00000001
        (0x04, "PF only"), // 00000100
        (0x10, "AF only"), // 00010000
        (0x40, "ZF only"), // 01000000
        (0x80, "SF only"), // 10000000
    ];

    for (flags, description) in test_cases {
        let code = [
            0x9f, // LAHF
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rflags = flags | 0x02; // Add bit 1 (always set)
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let ah = ((regs.rax >> 8) & 0xFF) as u8;
        assert_eq!(ah, (flags | 0x02) as u8, "LAHF failed for {}", description);
    }
}

#[test]
fn test_sahf_individual_flags() {
    // Test setting individual flags via SAHF
    let test_cases = vec![
        (0x01, "CF only"),
        (0x04, "PF only"),
        (0x10, "AF only"),
        (0x40, "ZF only"),
        (0x80, "SF only"),
    ];

    for (ah_value, description) in test_cases {
        let code = [
            0x9e, // SAHF
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = (ah_value as u64) << 8;
        regs.rflags = 0x02; // Clear all
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rflags & 0xD5,
            ah_value as u64,
            "SAHF failed for {}",
            description
        );
    }
}

#[test]
fn test_lahf_after_arithmetic() {
    // LAHF after ADD that sets carry
    let code = [
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (will set carry if RAX = -1)
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // -1, ADD will cause carry
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    // After ADD, flags should be set, LAHF captures them
    assert_ne!(ah, 0x02, "AH should reflect flags set by ADD");
}

#[test]
fn test_sahf_before_conditional() {
    // SAHF to set up flags for conditional jump
    let code = [
        0x9e, // SAHF (set ZF from AH)
        0x74, 0x02, // JZ +2 (jump if zero flag set)
        0xb3, 0x00, // MOV BL, 0 (shouldn't execute)
        0xb3, 0x01, // MOV BL, 1 (should execute)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4200; // AH = 0x42 (ZF set, bit 6)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x01, "Jump should have been taken");
}

#[test]
fn test_lahf_bit_pattern() {
    // Verify exact bit pattern: SF ZF 0 AF 0 PF 1 CF
    //                     Bits: 7  6  5  4  3  2  1  0
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    // Set SF=1, ZF=0, AF=1, PF=0, CF=1 (with bit 1 always 1)
    // Binary: 10010011 = 0x93
    regs.rflags = 0x93;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    assert_eq!(ah, 0x93, "AH bit pattern should be 10010011");
}

#[test]
fn test_lahf_sahf_multiple_times() {
    // Multiple LAHF/SAHF in sequence
    let code = [
        0x9f, // LAHF
        0x9e, // SAHF
        0x9f, // LAHF
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0xC7;
    let expected = regs.rflags & 0xD5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags & 0xD5,
        expected,
        "Flags should be preserved through multiple LAHF/SAHF"
    );
}

#[test]
fn test_sahf_with_different_ah_values() {
    // Test various AH values
    let test_cases = vec![0x00, 0x02, 0xFF, 0xAA, 0x55, 0xC7, 0x85, 0x47];

    for ah_val in test_cases {
        let code = [
            0x9e, // SAHF
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = (ah_val as u64) << 8;
        regs.rflags = 0x02;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rflags & 0xD5,
            (ah_val as u64) & 0xD5,
            "SAHF with AH={:#x}",
            ah_val
        );
    }
}

#[test]
fn test_lahf_preserves_upper_rax() {
    // LAHF shouldn't modify upper 56 bits of RAX
    let code = [
        0x9f, // LAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCD000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF00FF,
        0x123456789ABC0000,
        "Upper and lower bytes preserved"
    );
}

#[test]
fn test_lahf_sahf_compatibility() {
    // Ensure LAHF and SAHF are compatible
    let code = [
        0x9f, // LAHF
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (clear)
        0x48, 0x89, 0xd8, // MOV RAX, RBX (restore)
        0x9e, // SAHF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x87;
    let expected = regs.rflags & 0xD5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags & 0xD5,
        expected,
        "Flags preserved through save/restore"
    );
}
