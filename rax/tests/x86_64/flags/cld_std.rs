// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// CLD - Clear Direction Flag
// STD - Set Direction Flag
//
// These instructions manipulate the direction flag in RFLAGS.
// The DF flag controls the direction of string operations.
// CLD: clears DF (DF = 0) - string operations increment (forward)
// STD: sets DF (DF = 1) - string operations decrement (backward)
//
// Opcodes:
// FC           CLD           - Clear direction flag
// FD           STD           - Set direction flag

#[test]
fn test_cld_clears_direction_flag() {
    // CLD - Clear direction flag
    let code = [
        0xfc, // CLD - Clear direction flag
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 10); // Set DF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear after CLD");
}

#[test]
fn test_cld_on_already_clear_flag() {
    // CLD when DF is already clear
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // DF not set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should remain clear");
}

#[test]
fn test_std_sets_direction_flag() {
    // STD - Set direction flag
    let code = [
        0xfd, // STD - Set direction flag
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // DF clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set after STD");
}

#[test]
fn test_std_on_already_set_flag() {
    // STD when DF is already set
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 10); // DF already set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should remain set");
}

#[test]
fn test_cld_preserves_other_flags() {
    // CLD preserves other flags (CF, ZF, SF, PF, OF, AF)
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1 | (1 << 6) | (1 << 7) | (1 << 2) | (1 << 11); // CF, ZF, SF, PF, OF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
    assert_eq!(cf_set(regs.rflags), true, "CF should be preserved");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
    assert_eq!(sf_set(regs.rflags), true, "SF should be preserved");
    assert_eq!(pf_set(regs.rflags), true, "PF should be preserved");
    assert_eq!(of_set(regs.rflags), true, "OF should be preserved");
    assert_eq!(
        regs.rflags & !(1 << 10),
        initial_flags & !(1 << 10),
        "All flags except DF should match"
    );
}

#[test]
fn test_std_preserves_other_flags() {
    // STD preserves other flags
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6) | (1 << 7); // ZF, SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
    assert_eq!(sf_set(regs.rflags), true, "SF should be preserved");
}

#[test]
fn test_cld_does_not_modify_registers() {
    // CLD doesn't modify any general-purpose registers
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    regs.rbx = 0xFEDCBA9876543210;
    regs.rcx = 0x0011223344556677;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234567890ABCDEF, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0xFEDCBA9876543210, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x0011223344556677, "RCX should be unchanged");
}

#[test]
fn test_std_does_not_modify_registers() {
    // STD doesn't modify any general-purpose registers
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111111111111, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x2222222222222222, "RBX should be unchanged");
}

#[test]
fn test_cld_std_sequence() {
    // Sequence of CLD and STD
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        df_set(regs.rflags),
        true,
        "DF should be set after final STD"
    );
}

#[test]
fn test_cld_in_loop() {
    // CLD repeatedly in sequence
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_in_loop() {
    // STD repeatedly in sequence
    let code = [
        0xfc, // CLD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_with_all_flags_set() {
    // CLD when all flags are set
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 0xFFF; // Many flags set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
    // Other flags should not be affected
    let other_flags = regs.rflags & !(1 << 10); // Mask out DF
    let expected_flags = (0x2 | 0xFFF) & !(1 << 10);
    assert_eq!(
        other_flags, expected_flags,
        "Other flags should be preserved"
    );
}

#[test]
fn test_std_with_no_flags_set() {
    // STD when no flags are set
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
    assert_eq!(
        regs.rflags & !(1 << 10),
        0x2,
        "Other flags should remain unchanged"
    );
}

#[test]
fn test_cld_std_before_string_ops() {
    // CLD/STD are typically used before string operations
    // This tests they work in sequence
    let code = [
        0xfc, // CLD - forward direction
        0xb8, 0x00, 0x10, 0x00, 0x00, // MOV EAX, 0x1000
        0xfd, // STD - reverse direction
        0xb8, 0x00, 0x20, 0x00, 0x00, // MOV EAX, 0x2000
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set (reverse)");
}

#[test]
fn test_cld_multiple_times() {
    // Multiple CLDs
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_multiple_times() {
    // Multiple STDs
    let code = [
        0xfc, // CLD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_does_not_affect_carry_flag() {
    // CLD doesn't affect carry flag
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be preserved");
    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_does_not_affect_carry_flag() {
    // STD doesn't affect carry flag
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be preserved");
    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_isolation() {
    // CLD affects only DF
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 0xFFF; // Many flags set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All flags except DF should be the same
    let before = (0x2 | 0xFFF);
    let after = regs.rflags;
    let mask = !(1 << 10); // DF mask

    assert_eq!(
        after & mask,
        before & mask,
        "Non-DF flags should be unchanged"
    );
    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_isolation() {
    // STD affects only DF
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // No flags set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mask = !(1 << 10); // DF mask
    assert_eq!(regs.rflags & mask, 0x2, "Non-DF flags should not change");
    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_with_zero_flags() {
    // CLD with zero flags
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, 0x2, "Only reserved bit should remain");
}

#[test]
fn test_std_with_zero_flags() {
    // STD with zero flags
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags,
        0x2 | (1 << 10),
        "Only reserved bit and DF should be set"
    );
}

#[test]
fn test_cld_std_alternating() {
    // Alternating CLD and STD
    let code = [
        0xfc, // CLD
        0xfd, // STD
        0xfc, // CLD
        0xfd, // STD
        0xfc, // CLD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        df_set(regs.rflags),
        false,
        "DF should be clear after final CLD"
    );
}

#[test]
fn test_std_cld_alternating() {
    // Alternating STD and CLD, starting with STD
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xfd, // STD
        0xfc, // CLD
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        df_set(regs.rflags),
        true,
        "DF should be set after final STD"
    );
}

#[test]
fn test_cld_with_carry_flag_set() {
    // CLD with CF set
    let code = [
        0xfc, // CLD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 10) | 1; // DF and CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
    assert_eq!(cf_set(regs.rflags), true, "CF should be preserved");
}

#[test]
fn test_std_with_zero_flag_set() {
    // STD with ZF set
    let code = [
        0xfd, // STD
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6); // ZF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
}

#[test]
fn test_cld_rapid_sequence() {
    // Rapid sequence of CLD
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xfc, // CLD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_rapid_sequence() {
    // Rapid sequence of STD
    let code = [
        0xfc, // CLD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_std_with_add_instruction() {
    // CLD/STD don't interfere with other instructions
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xfc, // CLD
        0x83, 0xc0, 0x03, // ADD EAX, 3
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 8, "EAX should be 8");
    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_cld_std_does_not_modify_rsi_rdi() {
    // CLD/STD don't modify RSI or RDI
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x10, 0x00, 0x00, // MOV RSI, 0x1000
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000
        0xfc, // CLD
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x1000, "RSI should be unchanged");
    assert_eq!(regs.rdi, 0x2000, "RDI should be unchanged");
    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}

#[test]
fn test_direction_flag_default_state() {
    // Direction flag starts clear in default registers
    let code = [
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear by default");
}

#[test]
fn test_cld_after_std() {
    // CLD clears what STD set
    let code = [
        0xfd, // STD
        0xfc, // CLD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), false, "DF should be clear");
}

#[test]
fn test_std_after_cld() {
    // STD sets what CLD cleared
    let code = [
        0xfc, // CLD
        0xfd, // STD
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(df_set(regs.rflags), true, "DF should be set");
}
