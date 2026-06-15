// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// CLC - Clear Carry Flag
// STC - Set Carry Flag
// CMC - Complement Carry Flag
//
// These instructions manipulate the carry flag in RFLAGS.
// CLC: clears CF (CF = 0)
// STC: sets CF (CF = 1)
// CMC: toggles CF (CF = NOT CF)
//
// Opcodes:
// F8           CLC           - Clear carry flag
// F9           STC           - Set carry flag
// F5           CMC           - Complement carry flag

#[test]
fn test_clc_clears_carry_flag() {
    // CLC - Clear carry flag
    let code = [
        0xf8, // CLC - Clear carry flag
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear after CLC");
}

#[test]
fn test_clc_on_already_clear_flag() {
    // CLC when CF is already clear
    let code = [
        0xf8, // CLC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF not set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should remain clear");
}

#[test]
fn test_stc_sets_carry_flag() {
    // STC - Set carry flag
    let code = [
        0xf9, // STC - Set carry flag
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set after STC");
}

#[test]
fn test_stc_on_already_set_flag() {
    // STC when CF is already set
    let code = [
        0xf9, // STC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF already set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should remain set");
}

#[test]
fn test_cmc_complements_carry_flag_from_set() {
    // CMC when CF is set
    let code = [
        0xf5, // CMC - Complement carry flag
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        false,
        "CF should be clear after CMC from set"
    );
}

#[test]
fn test_cmc_complements_carry_flag_from_clear() {
    // CMC when CF is clear
    let code = [
        0xf5, // CMC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        true,
        "CF should be set after CMC from clear"
    );
}

#[test]
fn test_clc_preserves_other_flags() {
    // CLC preserves other flags (ZF, SF, PF, OF, AF)
    let code = [
        0xf8, // CLC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6) | (1 << 7) | (1 << 2) | (1 << 11); // ZF, SF, PF, OF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
    assert_eq!(sf_set(regs.rflags), true, "SF should be preserved");
    assert_eq!(pf_set(regs.rflags), true, "PF should be preserved");
    assert_eq!(of_set(regs.rflags), true, "OF should be preserved");
    assert_eq!(
        regs.rflags & !(1),
        initial_flags & !(1),
        "All flags except CF should match"
    );
}

#[test]
fn test_stc_preserves_other_flags() {
    // STC preserves other flags
    let code = [
        0xf9, // STC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6) | (1 << 7); // ZF, SF set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
    assert_eq!(sf_set(regs.rflags), true, "SF should be preserved");
}

#[test]
fn test_cmc_preserves_other_flags() {
    // CMC preserves other flags
    let code = [
        0xf5, // CMC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 6) | (1 << 7) | 1; // ZF, SF set, CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be toggled to clear");
    assert_eq!(zf_set(regs.rflags), true, "ZF should be preserved");
    assert_eq!(sf_set(regs.rflags), true, "SF should be preserved");
}

#[test]
fn test_clc_does_not_modify_registers() {
    // CLC doesn't modify any general-purpose registers
    let code = [
        0xf8, // CLC
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
fn test_stc_does_not_modify_registers() {
    // STC doesn't modify any general-purpose registers
    let code = [
        0xf9, // STC
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
fn test_cmc_does_not_modify_registers() {
    // CMC doesn't modify any general-purpose registers
    let code = [
        0xf5, // CMC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.rbx = 0xBBBBBBBBBBBBBBBB;
    regs.rcx = 0xCCCCCCCCCCCCCCCC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xAAAAAAAAAAAAAAAA, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0xBBBBBBBBBBBBBBBB, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0xCCCCCCCCCCCCCCCC, "RCX should be unchanged");
}

#[test]
fn test_clc_after_add_with_carry() {
    // CLC clears CF set by ADD
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets CF)
        0xf8, // CLC
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        false,
        "CF should be clear after CLC following ADD"
    );
}

#[test]
fn test_stc_before_adc() {
    // STC sets CF for ADC instruction
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0xf9, // STC - set carry for next ADC
        0x11, 0xd8, // ADC EAX, EBX (EAX = 1 + 2 + 1 = 4)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        4,
        "EAX should be 4 (1 + 2 + 1 from carry)"
    );
}

#[test]
fn test_cmc_double_toggle() {
    // CMC applied twice returns to original
    let code = [
        0xf5, // CMC - set CF
        0xf5, // CMC - clear CF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF clear
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        false,
        "CF should be clear after double CMC"
    );
}

#[test]
fn test_clc_stc_sequence() {
    // Sequence of CLC and STC
    let code = [
        0xf8, // CLC
        0xf9, // STC
        0xf8, // CLC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 1; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        false,
        "CF should be clear after final CLC"
    );
}

#[test]
fn test_clc_in_loop() {
    // CLC repeatedly in sequence
    let code = [
        0xf9, // STC
        0xf8, // CLC
        0xf8, // CLC
        0xf8, // CLC
        0xf8, // CLC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear");
}

#[test]
fn test_stc_in_loop() {
    // STC repeatedly in sequence
    let code = [
        0xf8, // CLC
        0xf9, // STC
        0xf9, // STC
        0xf9, // STC
        0xf9, // STC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set");
}

#[test]
fn test_cmc_alternating() {
    // CMC alternating (set, clear, set)
    let code = [
        0xf8, // CLC
        0xf5, // CMC - set
        0xf5, // CMC - clear
        0xf5, // CMC - set
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        true,
        "CF should be set after odd number of CMCs"
    );
}

#[test]
fn test_clc_with_all_flags_set() {
    // CLC when all flags are set
    let code = [
        0xf8, // CLC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | 0xFFF; // Many flags set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear");
    // Other flags should not be affected
    let other_flags = regs.rflags & !1; // Mask out CF
    let expected_flags = (0x2 | 0xFFF) & !1;
    assert_eq!(
        other_flags, expected_flags,
        "Other flags should be preserved"
    );
}

#[test]
fn test_stc_with_no_flags_set() {
    // STC when no flags are set
    let code = [
        0xf9, // STC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set");
    assert_eq!(regs.rflags & !1, 0x2, "Other flags should remain unchanged");
}

#[test]
fn test_cmc_with_intermediate_flags() {
    // CMC with other flags changed between instructions
    let code = [
        0xf9, // STC - set CF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (may modify flags but CF set by STC)
        0xf5, // CMC - toggle CF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // After ADD without overflow, CF should be clear, then CMC sets it
    // (This depends on the implementation of ADD)
}

#[test]
fn test_clc_stc_cmc_before_sub() {
    // Flag manipulation before SUB instruction
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0xf9, // STC
        0x19, 0xd8, // SBB EAX, EBX (5 - 3 - 1 = 1)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should be 1 (5 - 3 - 1)");
}

#[test]
fn test_clc_before_sbb_no_borrow() {
    // CLC ensures no borrow in SBB
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0xf8, // CLC - no borrow
        0x19, 0xd8, // SBB EAX, EBX (5 - 3 - 0 = 2)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 2, "EAX should be 2 (5 - 3 - 0)");
}

#[test]
fn test_cmc_behavior_with_adc() {
    // CMC used to invert ADC behavior
    let code = [
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16
        0xbb, 0x0f, 0x00, 0x00, 0x00, // MOV EBX, 15
        0xf8, // CLC - no carry
        0x11, 0xd8, // ADC EAX, EBX (16 + 15 + 0 = 31)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 31, "EAX should be 31 with no carry");
}

#[test]
fn test_clc_stc_cmc_rapid_sequence() {
    // Rapid sequence of all three instructions
    let code = [
        0xf8, // CLC
        0xf9, // STC
        0xf5, // CMC
        0xf5, // CMC
        0xf9, // STC
        0xf8, // CLC
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear at end");
}

#[test]
fn test_clc_after_comparison() {
    // CLC clears any CF set by comparison
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0x39, 0xd8, // CMP EAX, EBX
        0xf8, // CLC
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), false, "CF should be clear");
}

#[test]
fn test_stc_with_af_flag() {
    // STC doesn't affect auxiliary carry flag
    let code = [
        0xf9, // STC
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | (1 << 4); // AF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set");
    assert_eq!(af_set(regs.rflags), true, "AF should be preserved");
}

#[test]
fn test_clc_cmc_stc_pattern() {
    // Pattern test: clear, toggle (sets), set (no change)
    let code = [
        0xf8, // CLC - clear
        0xf5, // CMC - toggle to set
        0xf9, // STC - set (no change, already set)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set");
}

#[test]
fn test_cmc_idempotent_twice() {
    // CMC twice should be idempotent
    let code = [
        0xf5, // CMC - set CF (assuming it starts clear from 0x2)
        0xf5, // CMC - clear CF
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Start with CF clear
    let initial_cf = cf_set(regs.rflags);
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        cf_set(regs.rflags),
        initial_cf,
        "CF should be back to initial state after two CMCs"
    );
}

#[test]
fn test_carry_flag_instructions_chain() {
    // Chain: set, clear, toggle, set
    let code = [
        0xf9, // STC
        0xf8, // CLC
        0xf5, // CMC
        0xf9, // STC
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(cf_set(regs.rflags), true, "CF should be set at end");
}
