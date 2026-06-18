use crate::common::{DATA_ADDR, read_mem_u8, run_until_hlt, setup_vm, write_mem_u8};
use rax::cpu::Registers;

// SETcc - Conditional Set Instructions (0F 9x family)
// Sets destination byte to 1 if condition is true, 0 if false
// Conditions based on RFLAGS bits:
// - CF (Carry Flag) - bit 0
// - ZF (Zero Flag) - bit 6
// - SF (Sign Flag) - bit 7
// - OF (Overflow Flag) - bit 11
// - PF (Parity Flag) - bit 2

// ============================================================================
// SETO/SETNO - Overflow Flag Tests (OF)
// ============================================================================

#[test]
fn test_seto_al_overflow_set() {
    // SETO sets byte to 1 if OF=1
    let code = [
        0xb0, 0x7f, // MOV AL, 0x7F (max positive i8)
        0x04, 0x01, // ADD AL, 1 (overflow to -128)
        0x0f, 0x90, 0xc0, // SETO AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when OF=1");
}

#[test]
fn test_seto_al_overflow_clear() {
    // SETO sets byte to 0 if OF=0
    let code = [
        0xb0, 0x01, // MOV AL, 1
        0x04, 0x01, // ADD AL, 1 (no overflow)
        0x0f, 0x90, 0xc0, // SETO AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when OF=0");
}

#[test]
fn test_setno_bl_no_overflow() {
    // SETNO sets byte to 1 if OF=0
    let code = [
        0xb0, 0x01, // MOV AL, 1
        0x04, 0x01, // ADD AL, 1 (no overflow)
        0x0f, 0x91, 0xc3, // SETNO BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when OF=0");
}

#[test]
fn test_setno_bl_overflow() {
    // SETNO sets byte to 0 if OF=1
    let code = [
        0xb0, 0x7f, // MOV AL, 0x7F
        0x04, 0x01, // ADD AL, 1 (overflow)
        0x0f, 0x91, 0xc3, // SETNO BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x00, "BL should be 0 when OF=1");
}

// ============================================================================
// SETB/SETC/SETNAE - Below/Carry Tests (CF=1)
// ============================================================================

#[test]
fn test_setb_al_below() {
    // SETB sets byte to 1 if CF=1 (unsigned below)
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (5 < 10, sets CF)
        0x0f, 0x92, 0xc0, // SETB AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when CF=1");
}

#[test]
fn test_setb_al_not_below() {
    // SETB sets byte to 0 if CF=0
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 >= 5)
        0x0f, 0x92, 0xc0, // SETB AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when CF=0");
}

#[test]
fn test_setc_cl_carry() {
    // SETC is alias for SETB (CF=1)
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x92, 0xc1, // SETC CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 when CF=1");
}

#[test]
fn test_setnae_dl_not_above_equal() {
    // SETNAE is alias for SETB
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xbb, 0x07, 0x00, 0x00, 0x00, // MOV EBX, 7
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x92, 0xc2, // SETNAE DL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFF, 0x01, "DL should be 1 when below");
}

// ============================================================================
// SETAE/SETNB/SETNC - Above or Equal/Not Below Tests (CF=0)
// ============================================================================

#[test]
fn test_setae_al_above_or_equal() {
    // SETAE sets byte to 1 if CF=0
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 >= 5)
        0x0f, 0x93, 0xc0, // SETAE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when CF=0");
}

#[test]
fn test_setae_al_below() {
    // SETAE sets byte to 0 if CF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x93, 0xc0, // SETAE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when CF=1");
}

#[test]
fn test_setnb_bl_not_below() {
    // SETNB is alias for SETAE
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 8
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0x39, 0xd8, // CMP EAX, EBX (equal)
        0x0f, 0x93, 0xc3, // SETNB BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when not below");
}

#[test]
fn test_setnc_cl_no_carry() {
    // SETNC is alias for SETAE
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x93, 0xc1, // SETNC CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 when no carry");
}

// ============================================================================
// SETE/SETZ - Equal/Zero Tests (ZF=1)
// ============================================================================

#[test]
fn test_sete_al_equal() {
    // SETE sets byte to 1 if ZF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (sets ZF)
        0x0f, 0x94, 0xc0, // SETE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when ZF=1");
}

#[test]
fn test_sete_al_not_equal() {
    // SETE sets byte to 0 if ZF=0
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x94, 0xc0, // SETE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when ZF=0");
}

#[test]
fn test_setz_bl_zero() {
    // SETZ is alias for SETE
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (sets ZF)
        0x0f, 0x94, 0xc3, // SETZ BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when zero");
}

// ============================================================================
// SETNE/SETNZ - Not Equal/Not Zero Tests (ZF=0)
// ============================================================================

#[test]
fn test_setne_al_not_equal() {
    // SETNE sets byte to 1 if ZF=0
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x95, 0xc0, // SETNE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when ZF=0");
}

#[test]
fn test_setne_al_equal() {
    // SETNE sets byte to 0 if ZF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x95, 0xc0, // SETNE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when ZF=1");
}

#[test]
fn test_setnz_cl_not_zero() {
    // SETNZ is alias for SETNE
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0x85, 0xc0, // TEST EAX, EAX (clears ZF)
        0x0f, 0x95, 0xc1, // SETNZ CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 when not zero");
}

// ============================================================================
// SETBE/SETNA - Below or Equal/Not Above Tests (CF=1 or ZF=1)
// ============================================================================

#[test]
fn test_setbe_al_below() {
    // SETBE sets byte to 1 if CF=1 or ZF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (5 < 10, sets CF)
        0x0f, 0x96, 0xc0, // SETBE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when below");
}

#[test]
fn test_setbe_al_equal() {
    // SETBE when equal (ZF=1)
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (sets ZF)
        0x0f, 0x96, 0xc0, // SETBE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when equal");
}

#[test]
fn test_setbe_al_above() {
    // SETBE sets byte to 0 if CF=0 and ZF=0
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 > 5)
        0x0f, 0x96, 0xc0, // SETBE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when above");
}

#[test]
fn test_setna_bl_not_above() {
    // SETNA is alias for SETBE
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x96, 0xc3, // SETNA BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when not above");
}

// ============================================================================
// SETA/SETNBE - Above/Not Below or Equal Tests (CF=0 and ZF=0)
// ============================================================================

#[test]
fn test_seta_al_above() {
    // SETA sets byte to 1 if CF=0 and ZF=0
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 > 5)
        0x0f, 0x97, 0xc0, // SETA AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when above");
}

#[test]
fn test_seta_al_equal() {
    // SETA sets byte to 0 if ZF=1
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x97, 0xc0, // SETA AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when equal");
}

#[test]
fn test_seta_al_below() {
    // SETA sets byte to 0 if CF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x97, 0xc0, // SETA AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when below");
}

#[test]
fn test_setnbe_bl_not_below_equal() {
    // SETNBE is alias for SETA
    let code = [
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x97, 0xc3, // SETNBE BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when above");
}

// ============================================================================
// SETS/SETNS - Sign Flag Tests (SF)
// ============================================================================

#[test]
fn test_sets_al_sign_set() {
    // SETS sets byte to 1 if SF=1 (negative)
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x29, 0xd8, // SUB EAX, EBX (5-10=-5, sets SF)
        0x0f, 0x98, 0xc0, // SETS AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when SF=1");
}

#[test]
fn test_sets_al_sign_clear() {
    // SETS sets byte to 0 if SF=0 (positive)
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x29, 0xd8, // SUB EAX, EBX (10-5=5, clears SF)
        0x0f, 0x98, 0xc0, // SETS AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when SF=0");
}

#[test]
fn test_setns_bl_no_sign() {
    // SETNS sets byte to 1 if SF=0
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x29, 0xd8, // SUB EAX, EBX (clears SF)
        0x0f, 0x99, 0xc3, // SETNS BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when SF=0");
}

#[test]
fn test_setns_bl_sign_set() {
    // SETNS sets byte to 0 if SF=1
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x29, 0xd8, // SUB EAX, EBX (sets SF)
        0x0f, 0x99, 0xc3, // SETNS BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x00, "BL should be 0 when SF=1");
}

// ============================================================================
// SETP/SETPE - Parity/Parity Even Tests (PF=1)
// ============================================================================

#[test]
fn test_setp_al_parity_even() {
    // SETP sets byte to 1 if PF=1 (even number of set bits in low byte)
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03 (2 bits set, even parity)
        0x85, 0xc0, // TEST EAX, EAX (sets PF)
        0x0f, 0x9a, 0xc0, // SETP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when PF=1");
}

#[test]
fn test_setp_al_parity_odd() {
    // SETP sets byte to 0 if PF=0 (odd number of set bits)
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01 (1 bit set, odd parity)
        0x85, 0xc0, // TEST EAX, EAX (clears PF)
        0x0f, 0x9a, 0xc0, // SETP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when PF=0");
}

#[test]
fn test_setpe_bl_parity_even() {
    // SETPE is alias for SETP
    let code = [
        0xb8, 0x07, 0x00, 0x00,
        0x00, // MOV EAX, 0x07 (3 bits, odd count but only low byte matters)
        0x85, 0xc0, // TEST EAX, EAX
        0x0f, 0x9a, 0xc3, // SETPE BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x00, "BL should match parity flag");
}

// ============================================================================
// SETNP/SETPO - Not Parity/Parity Odd Tests (PF=0)
// ============================================================================

#[test]
fn test_setnp_al_parity_odd() {
    // SETNP sets byte to 1 if PF=0
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x01 (odd parity)
        0x85, 0xc0, // TEST EAX, EAX
        0x0f, 0x9b, 0xc0, // SETNP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when PF=0");
}

#[test]
fn test_setnp_al_parity_even() {
    // SETNP sets byte to 0 if PF=1
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 0x03 (even parity)
        0x85, 0xc0, // TEST EAX, EAX
        0x0f, 0x9b, 0xc0, // SETNP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when PF=1");
}

#[test]
fn test_setpo_cl_parity_odd() {
    // SETPO is alias for SETNP
    let code = [
        0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 0x07 (odd parity)
        0x85, 0xc0, // TEST EAX, EAX
        0x0f, 0x9b, 0xc1, // SETPO CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 when parity odd");
}

// ============================================================================
// SETL/SETNGE - Less/Not Greater or Equal Tests (SF != OF)
// ============================================================================

#[test]
fn test_setl_al_less() {
    // SETL sets byte to 1 if SF != OF (signed less)
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (5 < 10 signed)
        0x0f, 0x9c, 0xc0, // SETL AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when less");
}

#[test]
fn test_setl_al_not_less() {
    // SETL sets byte to 0 if SF == OF
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 >= 5)
        0x0f, 0x9c, 0xc0, // SETL AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when not less");
}

#[test]
fn test_setl_negative_vs_positive() {
    // SETL with negative < positive
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, -1
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x39, 0xd8, // CMP EAX, EBX (-1 < 1 signed)
        0x0f, 0x9c, 0xc0, // SETL AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when -1 < 1");
}

#[test]
fn test_setnge_bl_not_greater_equal() {
    // SETNGE is alias for SETL
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9c, 0xc3, // SETNGE BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when less");
}

// ============================================================================
// SETGE/SETNL - Greater or Equal/Not Less Tests (SF == OF)
// ============================================================================

#[test]
fn test_setge_al_greater_equal() {
    // SETGE sets byte to 1 if SF == OF
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 >= 5)
        0x0f, 0x9d, 0xc0, // SETGE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0x01,
        "AL should be 1 when greater or equal"
    );
}

#[test]
fn test_setge_al_less() {
    // SETGE sets byte to 0 if SF != OF
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9d, 0xc0, // SETGE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when less");
}

#[test]
fn test_setnl_bl_not_less() {
    // SETNL is alias for SETGE
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 8
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0x39, 0xd8, // CMP EAX, EBX (equal)
        0x0f, 0x9d, 0xc3, // SETNL BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when equal");
}

// ============================================================================
// SETLE/SETNG - Less or Equal/Not Greater Tests (ZF=1 or SF != OF)
// ============================================================================

#[test]
fn test_setle_al_less() {
    // SETLE sets byte to 1 if ZF=1 or SF != OF
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (5 <= 10)
        0x0f, 0x9e, 0xc0, // SETLE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when less");
}

#[test]
fn test_setle_al_equal() {
    // SETLE when equal
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9e, 0xc0, // SETLE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when equal");
}

#[test]
fn test_setle_al_greater() {
    // SETLE sets byte to 0 if ZF=0 and SF == OF
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 > 5)
        0x0f, 0x9e, 0xc0, // SETLE AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when greater");
}

#[test]
fn test_setng_bl_not_greater() {
    // SETNG is alias for SETLE
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9e, 0xc3, // SETNG BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when not greater");
}

// ============================================================================
// SETG/SETNLE - Greater/Not Less or Equal Tests (ZF=0 and SF == OF)
// ============================================================================

#[test]
fn test_setg_al_greater() {
    // SETG sets byte to 1 if ZF=0 and SF == OF
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (10 > 5)
        0x0f, 0x9f, 0xc0, // SETG AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 when greater");
}

#[test]
fn test_setg_al_equal() {
    // SETG sets byte to 0 if ZF=1
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9f, 0xc0, // SETG AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when equal");
}

#[test]
fn test_setg_al_less() {
    // SETG sets byte to 0 if SF != OF
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9f, 0xc0, // SETG AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when less");
}

#[test]
fn test_setnle_bl_not_less_equal() {
    // SETNLE is alias for SETG
    let code = [
        0xb8, 0x0f, 0x00, 0x00, 0x00, // MOV EAX, 15
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9f, 0xc3, // SETNLE BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x01, "BL should be 1 when greater");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_sete_memory_true() {
    // SETE with memory operand, condition true
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (sets ZF)
        0x0f, 0x94, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SETE [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xFF); // Pre-fill with non-zero
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x01, "Memory should be 1 when ZF=1");
}

#[test]
fn test_sete_memory_false() {
    // SETE with memory operand, condition false
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (clears ZF)
        0x0f, 0x94, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SETE [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xFF); // Pre-fill with non-zero
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x00, "Memory should be 0 when ZF=0");
}

#[test]
fn test_setl_memory() {
    // SETL with memory operand
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x9c, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SETL [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x01, "Memory should be 1 when less");
}

#[test]
fn test_seta_memory() {
    // SETA with memory operand
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x97, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SETA [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x01, "Memory should be 1 when above");
}

// ============================================================================
// Extended Register Tests (R8-R15 low bytes)
// ============================================================================

#[test]
fn test_sete_r8b() {
    // SETE with R8B (requires REX prefix)
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x41, 0x0f, 0x94, 0xc0, // SETE R8B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8 & 0xFF, 0x01, "R8B should be 1 when equal");
}

#[test]
fn test_setne_r9b() {
    // SETNE with R9B
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x41, 0x0f, 0x95, 0xc1, // SETNE R9B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r9 & 0xFF, 0x01, "R9B should be 1 when not equal");
}

#[test]
fn test_setl_r10b() {
    // SETL with R10B
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX
        0x41, 0x0f, 0x9c, 0xc2, // SETL R10B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r10 & 0xFF, 0x01, "R10B should be 1 when less");
}

#[test]
fn test_setg_r11b() {
    // SETG with R11B
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x41, 0x0f, 0x9f, 0xc3, // SETG R11B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r11 & 0xFF, 0x01, "R11B should be 1 when greater");
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_sete_preserves_upper_bits() {
    // SETcc only modifies the target byte
    let code = [
        0x31, 0xc9, // XOR ECX, ECX (sets ZF without touching RAX)
        0x0f, 0x94, 0xc0, // SETE AL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFFFF00,
        0xDEADBEEFDEADBE00,
        "Upper 56 bits should be preserved"
    );
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
}

#[test]
fn test_setcc_does_not_modify_flags() {
    // SETcc instructions should not modify flags
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX (sets ZF)
        0x0f, 0x94, 0xc0, // SETE AL
        0x0f, 0x94, 0xc1, // SETE CL (should still work, ZF unchanged)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 (flags preserved)");
}

#[test]
fn test_setb_unsigned_boundary() {
    // Test SETB at unsigned boundary (0 < 1)
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (EAX = 0)
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x39, 0xd8, // CMP EAX, EBX (0 < 1)
        0x0f, 0x92, 0xc0, // SETB AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 for 0 < 1");
}

#[test]
fn test_setl_signed_boundary() {
    // Test SETL with signed overflow boundary
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000 (INT_MIN)
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x39, 0xd8, // CMP EAX, EBX (INT_MIN < 1 signed)
        0x0f, 0x9c, 0xc0, // SETL AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 for INT_MIN < 1");
}

#[test]
fn test_seta_max_unsigned() {
    // Test SETA with maximum unsigned value
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0xfe, 0xff, 0xff, 0xff, // MOV EBX, 0xFFFFFFFE
        0x39, 0xd8, // CMP EAX, EBX
        0x0f, 0x97, 0xc0, // SETA AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 for MAX > MAX-1");
}

#[test]
fn test_setcc_chain_multiple_conditions() {
    // Chain multiple SETcc operations
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x0a, 0x00, 0x00, 0x00, // MOV EBX, 10
        0x39, 0xd8, // CMP EAX, EBX (5 < 10)
        0x0f, 0x92, 0xc0, // SETB AL (CF=1, should set AL=1)
        0x0f, 0x94, 0xc1, // SETE CL (ZF=0, should set CL=0)
        0x0f, 0x9c, 0xc2, // SETL DL (SF!=OF, should set DL=1)
        0x0f, 0x97, 0xc3, // SETA BL (CF=1, should set BL=0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1 (below)");
    assert_eq!(regs.rcx & 0xFF, 0x00, "CL should be 0 (not equal)");
    assert_eq!(regs.rdx & 0xFF, 0x01, "DL should be 1 (less)");
    assert_eq!(regs.rbx & 0xFF, 0x00, "BL should be 0 (not above)");
}

#[test]
fn test_seto_signed_overflow_add() {
    // Test overflow with addition of two positive numbers
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF (INT_MAX)
        0x83, 0xc0, 0x01, // ADD EAX, 1 (overflow)
        0x0f, 0x90, 0xc1, // SETO CL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0x01, "CL should be 1 on overflow");
}

#[test]
fn test_sets_zero_result() {
    // SETS should be clear when result is exactly zero
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x29, 0xd8, // SUB EAX, EBX (5-5=0, clears SF)
        0x0f, 0x98, 0xc0, // SETS AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0 when result is 0");
}

#[test]
fn test_setp_zero_byte() {
    // Parity flag with zero (even parity - all bits clear)
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (sets PF)
        0x0f, 0x9a, 0xc0, // SETP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0x01,
        "AL should be 1 (zero has even parity)"
    );
}

#[test]
fn test_setp_all_bits_set() {
    // Parity flag with 0xFF (8 bits set, even parity)
    let code = [
        0xb8, 0xff, 0x00, 0x00, 0x00, // MOV EAX, 0xFF
        0x85, 0xc0, // TEST EAX, EAX
        0x0f, 0x9a, 0xc0, // SETP AL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0x01,
        "AL should be 1 (0xFF has even parity)"
    );
}
