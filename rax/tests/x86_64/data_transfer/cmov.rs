use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// CMOVcc - Conditional Move Instructions (0F 4x family)
// Moves source to destination if condition code is true, preserving destination if false
// Conditions based on RFLAGS bits:
// - CF (Carry Flag) - bit 0
// - ZF (Zero Flag) - bit 6
// - SF (Sign Flag) - bit 7
// - OF (Overflow Flag) - bit 11
// - PF (Parity Flag) - bit 2

// ============================================================================
// CMOVO/CMOVNO - Overflow Flag Tests
// ============================================================================

#[test]
fn test_cmovo_eax_ebx_overflow_set() {
    // CMOVO moves if OF=1 (overflow occurred)
    // Use 32-bit operations for signed overflow detection
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF (MAX_INT in 32-bit)
        0x83, 0xc0, 0x01, // ADD EAX, 1 (overflows in 32-bit, sets OF)
        0x0f, 0x40, 0xc3, // CMOVO EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when OF=1"
    );
}

#[test]
fn test_cmovo_eax_ebx_overflow_clear() {
    // CMOVO should not move if OF=0 (no overflow)
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x83, 0xc0, 0x01, // ADD EAX, 1 (no overflow)
        0x0f, 0x40, 0xc3, // CMOVO EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000002,
        "EAX should not be moved when OF=0"
    );
}

#[test]
fn test_cmovno_eax_ebx_no_overflow() {
    // CMOVNO moves if OF=0 (no overflow)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (no overflow)
        0x0f, 0x41, 0xc3, // CMOVNO EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when OF=0"
    );
}

// ============================================================================
// CMOVB/CMOVC/CMOVNAE - Below/Carry Tests (unsigned <)
// ============================================================================

#[test]
fn test_cmovb_eax_ebx_below() {
    // CMOVB moves if CF=1 (carry, unsigned below)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10, sets CF)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 5 < 10"
    );
}

#[test]
fn test_cmovb_eax_ebx_not_below() {
    // CMOVB should not move if CF=0 (no carry)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 >= 5)
        0x0f, 0x42, 0xc3, // CMOVB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000a,
        "EAX should not be moved when 10 >= 5"
    );
}

#[test]
fn test_cmovc_eax_ebx_carry() {
    // CMOVC is alias for CMOVB (moves if CF=1)
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears CF)
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (sets CF)
        0x0f, 0x42, 0xc3, // CMOVC EAX, EBX (same as CMOVB)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when CF=1"
    );
}

#[test]
fn test_cmovnae_eax_ebx_below() {
    // CMOVNAE is alias for CMOVB (not above or equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x42, 0xc3, // CMOVNAE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when below"
    );
}

// ============================================================================
// CMOVAE/CMOVNB/CMOVNC - Above or Equal/Not Below Tests (unsigned >=)
// ============================================================================

#[test]
fn test_cmovae_eax_ebx_above_or_equal() {
    // CMOVAE moves if CF=0 (no carry, unsigned above or equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 >= 5)
        0x0f, 0x43, 0xc3, // CMOVAE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 10 >= 5"
    );
}

#[test]
fn test_cmovnb_eax_ebx_not_below() {
    // CMOVNB is alias for CMOVAE
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x43, 0xc3, // CMOVNB EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not below"
    );
}

#[test]
fn test_cmovnc_eax_ebx_no_carry() {
    // CMOVNC is alias for CMOVAE
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x43, 0xc3, // CMOVNC EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when no carry"
    );
}

// ============================================================================
// CMOVE/CMOVZ - Equal/Zero Tests (ZF=1)
// ============================================================================

#[test]
fn test_cmove_eax_ebx_equal() {
    // CMOVE moves if ZF=1 (values equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when equal"
    );
}

#[test]
fn test_cmove_eax_ebx_not_equal() {
    // CMOVE should not move if ZF=0
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (clears ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000005,
        "EAX should not be moved when not equal"
    );
}

#[test]
fn test_cmovz_eax_ebx_zero() {
    // CMOVZ is alias for CMOVE (move if zero)
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVZ EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when zero"
    );
}

// ============================================================================
// CMOVNE/CMOVNZ - Not Equal/Not Zero Tests (ZF=0)
// ============================================================================

#[test]
fn test_cmovne_eax_ebx_not_equal() {
    // CMOVNE moves if ZF=0 (not equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not equal"
    );
}

#[test]
fn test_cmovnz_eax_ebx_not_zero() {
    // CMOVNZ is alias for CMOVNE
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x45, 0xc3, // CMOVNZ EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not zero"
    );
}

// ============================================================================
// CMOVBE/CMOVNA - Below or Equal/Not Above Tests (unsigned <=)
// ============================================================================

#[test]
fn test_cmovbe_eax_ebx_below_equal() {
    // CMOVBE moves if CF=1 OR ZF=1
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 <= 10, sets CF)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 5 <= 10"
    );
}

#[test]
fn test_cmovbe_eax_ebx_equal() {
    // CMOVBE when values are equal
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (sets ZF)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when equal"
    );
}

#[test]
fn test_cmovbe_eax_ebx_above() {
    // CMOVBE should not move if above
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x46, 0xc3, // CMOVBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000a,
        "EAX should not be moved when above"
    );
}

#[test]
fn test_cmovna_eax_ebx_not_above() {
    // CMOVNA is alias for CMOVBE
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x46, 0xc3, // CMOVNA EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not above"
    );
}

// ============================================================================
// CMOVA/CMOVNBE - Above/Not Below or Equal Tests (unsigned >)
// ============================================================================

#[test]
fn test_cmova_eax_ebx_above() {
    // CMOVA moves if CF=0 AND ZF=0 (unsigned above)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x47, 0xc3, // CMOVA EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 10 > 5"
    );
}

#[test]
fn test_cmovnbe_eax_ebx_not_below_equal() {
    // CMOVNBE is alias for CMOVA
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x47, 0xc3, // CMOVNBE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when above"
    );
}

// ============================================================================
// CMOVS/CMOVNS - Sign Flag Tests (SF for negative/positive)
// ============================================================================

#[test]
fn test_cmovs_eax_ebx_sign_set() {
    // CMOVS moves if SF=1 (result is negative)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x29, 0xc8, // SUB RAX, RCX (5-10=-5, sets SF)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when result negative"
    );
}

#[test]
fn test_cmovs_eax_ebx_sign_clear() {
    // CMOVS should not move if SF=0
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (10-5=5, clears SF)
        0x0f, 0x48, 0xc3, // CMOVS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000005,
        "EAX should not be moved when result positive"
    );
}

#[test]
fn test_cmovns_eax_ebx_no_sign() {
    // CMOVNS moves if SF=0 (result is positive or zero)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x29, 0xc8, // SUB RAX, RCX (clears SF)
        0x0f, 0x49, 0xc3, // CMOVNS EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when result positive"
    );
}

// ============================================================================
// CMOVP/CMOVPE - Parity/Parity Even Tests (PF for even parity)
// ============================================================================

#[test]
fn test_cmovp_eax_ebx_parity_even() {
    // CMOVP moves if PF=1 (even parity, even number of set bits)
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 0x03 (2 bits set, even)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF)
        0x0f, 0x4a, 0xc3, // CMOVP EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when parity even"
    );
}

#[test]
fn test_cmovpe_eax_ebx_parity_even() {
    // CMOVPE is alias for CMOVP
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 0x03 (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x0f, 0x4a, 0xc3, // CMOVPE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when parity even"
    );
}

#[test]
fn test_cmovp_eax_ebx_parity_odd() {
    // CMOVP should not move if PF=0 (odd parity)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x01 (1 bit set, odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears PF)
        0x0f, 0x4a, 0xc3, // CMOVP EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should not be moved when parity odd"
    );
}

// ============================================================================
// CMOVNP/CMOVPO - Not Parity/Parity Odd Tests (PF=0)
// ============================================================================

#[test]
fn test_cmovnp_eax_ebx_parity_odd() {
    // CMOVNP moves if PF=0 (odd parity)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x01 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears PF)
        0x0f, 0x4b, 0xc3, // CMOVNP EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when parity odd"
    );
}

#[test]
fn test_cmovpo_eax_ebx_parity_odd() {
    // CMOVPO is alias for CMOVNP
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x01
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x0f, 0x4b, 0xc3, // CMOVPO EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when parity odd"
    );
}

// ============================================================================
// CMOVL/CMOVNGE - Less/Not Greater or Equal Tests (signed <)
// ============================================================================

#[test]
fn test_cmovl_eax_ebx_less() {
    // CMOVL moves if SF!=OF (signed less)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10 signed)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 5 < 10"
    );
}

#[test]
fn test_cmovl_negative_vs_positive() {
    // CMOVL with negative value
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x39, 0xc8, // CMP RAX, RCX (-1 < 1 signed)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when -1 < 1"
    );
}

#[test]
fn test_cmovnge_eax_ebx_not_greater_equal() {
    // CMOVNGE is alias for CMOVL
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4c, 0xc3, // CMOVNGE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when less"
    );
}

// ============================================================================
// CMOVGE/CMOVNL - Greater or Equal/Not Less Tests (signed >=)
// ============================================================================

#[test]
fn test_cmovge_eax_ebx_greater_equal() {
    // CMOVGE moves if SF==OF (signed greater or equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 >= 5 signed)
        0x0f, 0x4d, 0xc3, // CMOVGE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 10 >= 5"
    );
}

#[test]
fn test_cmovnl_eax_ebx_not_less() {
    // CMOVNL is alias for CMOVGE
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4d, 0xc3, // CMOVNL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not less"
    );
}

// ============================================================================
// CMOVLE/CMOVNG - Less or Equal/Not Greater Tests (signed <=)
// ============================================================================

#[test]
fn test_cmovle_eax_ebx_less_equal() {
    // CMOVLE moves if ZF==1 OR SF!=OF (signed less or equal)
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 <= 10)
        0x0f, 0x4e, 0xc3, // CMOVLE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 5 <= 10"
    );
}

#[test]
fn test_cmovle_eax_ebx_equal() {
    // CMOVLE when equal
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4e, 0xc3, // CMOVLE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when equal"
    );
}

#[test]
fn test_cmovng_eax_ebx_not_greater() {
    // CMOVNG is alias for CMOVLE
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4e, 0xc3, // CMOVNG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when not greater"
    );
}

// ============================================================================
// CMOVG/CMOVNLE - Greater/Not Less or Equal Tests (signed >)
// ============================================================================

#[test]
fn test_cmovg_eax_ebx_greater() {
    // CMOVG moves if ZF==0 AND SF==OF (signed greater)
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x4f, 0xc3, // CMOVG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when 10 > 5"
    );
}

#[test]
fn test_cmovnle_eax_ebx_not_less_equal() {
    // CMOVNLE is alias for CMOVG
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4f, 0xc3, // CMOVNLE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should be moved when greater"
    );
}

// ============================================================================
// 16-bit and 64-bit Operand Variants
// ============================================================================

#[test]
fn test_cmove_ax_bx_16bit() {
    // Test 16-bit CMOVE with 0x66 prefix
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x66, 0x0f, 0x44, 0xc3, // CMOVE AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x2222, "AX should be moved");
}

#[test]
fn test_cmove_rax_rbx_64bit() {
    // Test 64-bit CMOVE with REX.W prefix
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (sets ZF)
        0x48, 0x0f, 0x44, 0xc3, // CMOVE RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should be moved");
}

#[test]
fn test_cmovl_ax_bx_16bit() {
    // Test 16-bit CMOVL
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8
        0x48, 0xc7, 0xc1, 0x10, 0x00, 0x00, 0x00, // MOV RCX, 16
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x66, 0x0f, 0x4c, 0xc3, // CMOVL AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x2222, "AX should be moved");
}

#[test]
fn test_cmovl_rax_rbx_64bit() {
    // Test 64-bit CMOVL
    let code = [
        0x48, 0xc7, 0xc2, 0x32, 0x00, 0x00, 0x00, // MOV RDX, 50
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0x39, 0xca, // CMP RDX, RCX
        0x48, 0x0f, 0x4c, 0xc3, // CMOVL RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should be moved");
}

// ============================================================================
// Extended Register Variants (R8-R15)
// ============================================================================

#[test]
fn test_cmove_r8d_r9d() {
    // Test with extended registers
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x45, 0x0f, 0x44, 0xc1, // CMOVE R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFFFFFFFF, 0x22222222, "R8D should be moved");
}

#[test]
fn test_cmovl_r8d_r9d() {
    // Test CMOVL with extended registers
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x45, 0x0f, 0x4c, 0xc1, // CMOVL R8D, R9D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11111111;
    regs.r9 = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFFFFFFFF, 0x22222222, "R8D should be moved");
}

#[test]
fn test_cmove_r10_r11_64bit() {
    // Test 64-bit CMOVE with extended registers
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x4d, 0x0f, 0x44, 0xd3, // CMOVE R10, R11
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x1111111111111111;
    regs.r11 = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r10, 0x2222222222222222, "R10 should be moved");
}

// ============================================================================
// Flag Preservation Tests
// ============================================================================

#[test]
fn test_cmove_preserves_flags() {
    // CMOVE should not modify flags
    // Note: ADD -1, 1 = 0 with carry (CF=1, ZF=1)
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF and CF - wraps to 0 with carry)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set");
    assert!(
        regs.rflags & 0x01 != 0,
        "CF should still be set (from wrap-around)"
    );
}

// ============================================================================
// Practical Use Cases - Min/Max Patterns
// ============================================================================

#[test]
fn test_cmovl_unsigned_min() {
    // Min pattern: if RAX >= RBX, replace RAX with RBX (the smaller value)
    // Uses CMOVGE (move if greater or equal, signed) which works for positive values
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39,
        0xd8, // CMP RAX, RBX (20 - 10 = 10, positive, so SF=0, OF=0 => SF==OF => GE)
        0x48, 0x0f, 0x4d, 0xc3, // CMOVGE RAX, RBX (move if SF==OF)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10, "RAX should be minimum (10)");
}

#[test]
fn test_cmovg_unsigned_max() {
    // Unsigned max: result = (a < b) ? b : a - use CMOVL to move larger value
    let code = [
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (20 - 10 = 10, positive)
        0x48, 0x0f, 0x4c, 0xc3, // CMOVL RAX, RBX (if RAX < RBX, move RBX to RAX - not taken)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 20, "RAX should be maximum (20)");
}

#[test]
fn test_cmovl_signed_min() {
    // Signed min with negative numbers: if RAX > RBX, move RBX to RAX
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0xfe, 0xff, 0xff, 0xff, // MOV RBX, -2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (-1 - (-2) = 1, positive, no overflow)
        0x48, 0x0f, 0x4f, 0xc3, // CMOVG RAX, RBX (if RAX > RBX, move - ZF=0 AND SF==OF)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFE, "RAX should be -2 (minimum)");
}

// ============================================================================
// Conditional Assignment Patterns
// ============================================================================

#[test]
fn test_cmove_conditional_assignment_taken() {
    // if (x == 0) result = value;
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets ZF if RAX==0)
        0x0f, 0x44, 0xd3, // CMOVE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000; // x == 0
    regs.rdx = 0x11111111; // result (old value)
    regs.rbx = 0x22222222; // value (new value)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x22222222,
        "result should be updated"
    );
}

#[test]
fn test_cmove_conditional_assignment_not_taken() {
    // if (x == 0) result = value; (but x != 0)
    let code = [
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x0f, 0x44, 0xd3, // CMOVE EDX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000001; // x != 0
    regs.rdx = 0x11111111; // result (old value)
    regs.rbx = 0x22222222; // value (new value)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x11111111,
        "result should stay the same"
    );
}

// ============================================================================
// Source Unchanged When Condition Not Met
// ============================================================================

#[test]
fn test_cmove_source_unchanged_on_no_move() {
    // Destination should be unchanged if condition is false
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // When condition is false and RAX holds 1, only lower 32 bits are present after TEST
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000001,
        "EAX should retain its value"
    );
}

#[test]
fn test_cmovl_source_unchanged_on_no_move() {
    // Destination unchanged when not less
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xc8, // CMP RAX, RCX (10 > 5)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000000a,
        "EAX should retain its value (10)"
    );
}

// ============================================================================
// Operand Size Behavior
// ============================================================================

#[test]
fn test_cmove_32bit_zeros_upper_64() {
    // 32-bit CMOVE should zero upper 32 bits of 64-bit register
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF;
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000012345678,
        "Upper 32 bits should be zeroed"
    );
}

#[test]
fn test_cmove_16bit_preserves_upper() {
    // 16-bit CMOVE should preserve upper 48 bits
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX (sets ZF without touching RAX)
        0x66, 0x0f, 0x44, 0xc3, // CMOVE AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF;
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xDEADBEEFDEAD1234,
        "Upper 48 bits should be preserved"
    );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_cmove_same_register() {
    // Source and destination are same register (mov reg, reg)
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc0, // CMOVE EAX, EAX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 32-bit move should zero upper 32 bits
    assert_eq!(
        regs.rax, 0x0000000000000000,
        "RAX should be zero (moved to itself after XOR)"
    );
}

#[test]
fn test_cmove_with_zero_source() {
    // Moving zero value
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX should be set to zero"
    );
}

#[test]
fn test_cmove_with_all_ones_source() {
    // Moving all 1s value
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be all ones");
}

// ============================================================================
// Chaining Multiple Conditional Moves
// ============================================================================

#[test]
fn test_cmove_chain() {
    // Chain multiple conditional moves with flag changes
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX (should move, RAX = 0x22222222)
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF, RAX = 0x22222223)
        0x0f, 0x44, 0xd1, // CMOVE EDX, ECX (should not move, ZF=0)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rdx = 0x33333333;
    regs.rcx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222223,
        "EAX should be 0x22222222 after move, then +1"
    );
    assert_eq!(regs.rdx & 0xFFFFFFFF, 0x33333333, "EDX should not change");
}

#[test]
fn test_cmovl_chain() {
    // Chain multiple CMOVL with different comparisons
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x39, 0xc8, // CMP RAX, RCX (5 < 10)
        0x0f, 0x4c, 0xc3, // CMOVL EAX, EBX (should move)
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x39, 0xca, // CMP RDX, RCX (10 > 5)
        0x0f, 0x4c, 0xf7, // CMOVL ESI, EDI (should not move)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rsi = 0x33333333;
    regs.rdi = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x22222222, "EAX should be moved");
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x33333333, "ESI should not be moved");
}

// ============================================================================
// Multiple Register Combinations
// ============================================================================

#[test]
fn test_cmove_esi_edi() {
    // Test different register combination
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x44, 0xf7, // CMOVE ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x66666666,
        "ESI should be moved from EDI"
    );
}

#[test]
fn test_cmovl_esi_edi() {
    // CMOVL with different registers
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x48, 0x39, 0xc8, // CMP RAX, RCX
        0x0f, 0x4c, 0xf7, // CMOVL ESI, EDI
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 0x66666666, "ESI should be moved");
}

// ============================================================================
// Strengthened CMOVcc tests (appended): exact taken/not-taken destination
// values, the 32-bit zero-extension quirk when taken, preservation of the
// destination when not taken, memory source, and flag-neutrality.
// ============================================================================

#[test]
fn test_strict_cmove_taken_r64_exact() {
    // ZF set => CMOVE RAX, RBX moves; RAX becomes RBX exactly.
    // XOR RCX,RCX sets ZF=1; CMOVE RAX,RBX.
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX -> ZF=1
        0x48, 0x0f, 0x44, 0xc3, // CMOVE RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEAD_DEAD_DEAD_DEAD;
    regs.rbx = 0x0123_4567_89AB_CDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0123_4567_89AB_CDEF,
        "CMOVE taken copies full RBX"
    );
}

#[test]
fn test_strict_cmovne_not_taken_preserves_dest() {
    // ZF set => CMOVNE not taken; RAX must be unchanged (full 64-bit).
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX -> ZF=1
        0x48, 0x0f, 0x45, 0xc3, // CMOVNE RAX, RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1122_3344_5566_7788;
    regs.rbx = 0xFFFF_FFFF_FFFF_FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1122_3344_5566_7788,
        "CMOVNE not taken preserves dest"
    );
}

#[test]
fn test_strict_cmov_taken_r32_zero_extends() {
    // 32-bit CMOVE EAX, EBX when taken zero-extends EAX into RAX.
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX -> ZF=1
        0x0f, 0x44, 0xc3, // CMOVE EAX, EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = 0x0000_0000_AABB_CCDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_AABB_CCDD,
        "32-bit CMOV taken zero-extends RAX"
    );
}

#[test]
fn test_strict_cmov_taken_from_memory() {
    // CMOVE RAX, [RBX] when ZF set loads memory operand.
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX -> ZF=1
        0x48, 0x0f, 0x44, 0x03, // CMOVE RAX, [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = crate::common::DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    crate::common::write_mem_at_u64(&mem, crate::common::DATA_ADDR, 0xCAFE_F00D_1234_5678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xCAFE_F00D_1234_5678,
        "CMOVE taken loads memory operand"
    );
}

#[test]
fn test_strict_cmov_does_not_change_flags() {
    // CMOVcc must not modify flags; ZF set going in must remain set.
    let code = [
        0x48, 0x31, 0xc9, // XOR RCX, RCX -> ZF=1, CF/OF=0
        0x48, 0x0f, 0x44, 0xc3, // CMOVE RAX, RBX (taken)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5);
    assert!(
        crate::common::zf_set(regs.rflags),
        "ZF from XOR must survive CMOV"
    );
    assert!(!crate::common::cf_set(regs.rflags), "CF must remain clear");
    assert!(!crate::common::of_set(regs.rflags), "OF must remain clear");
}
