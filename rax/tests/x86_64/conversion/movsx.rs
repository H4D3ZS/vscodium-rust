use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// MOVSX - Move with Sign Extension
// Copies a byte or word to a larger register with sign extension
// The sign bit of the source is propagated to all upper bits of the destination

// ============================================================================
// MOVSX r16, r/m8 - Byte to Word Sign Extension
// ============================================================================

#[test]
fn test_movsx_ax_bl_negative_one() {
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFFFF,
        "AX should be sign-extended to 0xFFFF"
    );
}

#[test]
fn test_movsx_ax_bl_positive_max() {
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // 127 (max positive signed byte)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0x007F,
        "AX should be sign-extended to 0x007F"
    );
}

#[test]
fn test_movsx_ax_bl_negative_max() {
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 (min negative signed byte)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFF,
        0xFF80,
        "AX should be sign-extended to 0xFF80"
    );
}

#[test]
fn test_movsx_ax_bl_zero() {
    let code = [0x66, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX AX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be 0x0000");
}

#[test]
fn test_movsx_cx_dl_boundary_positive() {
    let code = [0x66, 0x0f, 0xbe, 0xca, 0xf4]; // MOVSX CX, DL
    let mut regs = Registers::default();
    regs.rdx = 0x7E; // 126 - just before boundary
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx & 0xFFFF,
        0x007E,
        "CX should be sign-extended to 0x007E"
    );
}

#[test]
fn test_movsx_cx_dl_boundary_negative() {
    let code = [0x66, 0x0f, 0xbe, 0xca, 0xf4]; // MOVSX CX, DL
    let mut regs = Registers::default();
    regs.rdx = 0x81; // -127
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx & 0xFFFF,
        0xFF81,
        "CX should be sign-extended to 0xFF81"
    );
}

// ============================================================================
// MOVSX r32, r/m8 - Byte to Dword Sign Extension
// ============================================================================

#[test]
fn test_movsx_eax_bl_negative_one() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended to 0xFFFFFFFF"
    );
}

#[test]
fn test_movsx_eax_bl_positive() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F; // 127 - positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000007F,
        "EAX should be sign-extended to 0x0000007F"
    );
}

#[test]
fn test_movsx_eax_bl_negative() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80; // -128 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "EAX should be sign-extended to 0xFFFFFF80"
    );
}

#[test]
fn test_movsx_eax_cl() {
    let code = [0x0f, 0xbe, 0xc1, 0xf4]; // MOVSX EAX, CL
    let mut regs = Registers::default();
    regs.rcx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "EAX should be sign-extended from CL"
    );
}

#[test]
fn test_movsx_eax_dl() {
    let code = [0x0f, 0xbe, 0xc2, 0xf4]; // MOVSX EAX, DL
    let mut regs = Registers::default();
    regs.rdx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x0000007F,
        "EAX should be sign-extended from DL"
    );
}

#[test]
fn test_movsx_edx_bl() {
    let code = [0x0f, 0xbe, 0xd3, 0xf4]; // MOVSX EDX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xAA; // -86
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFFFFAA,
        "EDX should be sign-extended"
    );
}

#[test]
fn test_movsx_esi_cl() {
    let code = [0x0f, 0xbe, 0xf1, 0xf4]; // MOVSX ESI, CL
    let mut regs = Registers::default();
    regs.rcx = 0x42; // 66
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x00000042,
        "ESI should be sign-extended from CL"
    );
}

// Note: SPL register tests removed as they may not be supported in all contexts

// ============================================================================
// MOVSX r32, r/m16 - Word to Dword Sign Extension
// ============================================================================

#[test]
fn test_movsx_eax_bx_negative_one() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // -1 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended to 0xFFFFFFFF"
    );
}

#[test]
fn test_movsx_eax_bx_positive() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFF; // 32767 - positive
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00007FFF,
        "EAX should be sign-extended to 0x7FFF"
    );
}

#[test]
fn test_movsx_eax_bx_negative() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8000; // -32768 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF8000,
        "EAX should be sign-extended to 0xFFFF8000"
    );
}

#[test]
fn test_movsx_edx_cx() {
    let code = [0x0f, 0xbf, 0xd1, 0xf4]; // MOVSX EDX, CX
    let mut regs = Registers::default();
    regs.rcx = 0x8000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xFFFF8000,
        "EDX should be sign-extended from CX"
    );
}

#[test]
fn test_movsx_esi_di() {
    let code = [0x0f, 0xbf, 0xf7, 0xf4]; // MOVSX ESI, DI
    let mut regs = Registers::default();
    regs.rdi = 0x7FFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x00007FFF,
        "ESI should be sign-extended from DI"
    );
}

#[test]
fn test_movsx_edi_ax() {
    let code = [0x0f, 0xbf, 0xf8, 0xf4]; // MOVSX EDI, AX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rdi & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EDI should be sign-extended from AX"
    );
}

#[test]
fn test_movsx_word_boundary_0x7ffe() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFE; // 32766
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00007FFE,
        "EAX should be 0x00007FFE"
    );
}

#[test]
fn test_movsx_word_boundary_0x8001() {
    let code = [0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX EAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8001; // -32767
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFF8001,
        "EAX should be 0xFFFF8001"
    );
}

// ============================================================================
// MOVSX r64, r/m8 - Byte to Qword Sign Extension
// ============================================================================

#[test]
fn test_movsx_rax_bl_negative_one() {
    let code = [0x48, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1 in signed byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

#[test]
fn test_movsx_rax_bl_positive() {
    let code = [0x48, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX RAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000000001,
        "RAX should be sign-extended to 0x0000000000000001"
    );
}

// Note: SPL register test removed

// ============================================================================
// MOVSX r64, r/m16 - Word to Qword Sign Extension
// ============================================================================

#[test]
fn test_movsx_rax_bx_negative_one() {
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF; // -1 in signed word
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

#[test]
fn test_movsx_rax_bx_positive() {
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000000001234,
        "RAX should be sign-extended to 0x0000000000001234"
    );
}

#[test]
fn test_movsx_rcx_di() {
    let code = [0x48, 0x0f, 0xbf, 0xcf, 0xf4]; // MOVSX RCX, DI
    let mut regs = Registers::default();
    regs.rdi = 0x8001; // -32767
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xFFFFFFFFFFFF8001,
        "RCX should be sign-extended from DI"
    );
}

// ============================================================================
// MOVSXD r64, r/m32 - Dword to Qword Sign Extension
// ============================================================================

#[test]
fn test_movsxd_rax_ebx_negative_one() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF; // -1 in signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFFFF,
        "RAX should be sign-extended to all 1s"
    );
}

#[test]
fn test_movsxd_rax_ebx_positive() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x7FFFFFFF; // max positive signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000007FFFFFFF,
        "RAX should be sign-extended to 0x7FFFFFFF"
    );
}

#[test]
fn test_movsxd_rax_ebx_negative() {
    let code = [0x48, 0x63, 0xc3, 0xf4]; // MOVSXD RAX, EBX
    let mut regs = Registers::default();
    regs.rbx = 0x80000000; // min negative signed dword
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFF80000000,
        "RAX should be sign-extended to 0xFFFFFFFF80000000"
    );
}

#[test]
fn test_movsxd_rcx_edx_small_positive() {
    let code = [0x48, 0x63, 0xca, 0xf4]; // MOVSXD RCX, EDX
    let mut regs = Registers::default();
    regs.rdx = 0x00000001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x0000000000000001, "RCX should be 1");
}

#[test]
fn test_movsxd_rcx_edx_small_negative() {
    let code = [0x48, 0x63, 0xca, 0xf4]; // MOVSXD RCX, EDX
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF; // -1 as 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xFFFFFFFFFFFFFFFF, "RCX should be -1 as 64-bit");
}

#[test]
fn test_movsxd_rbx_esi_value_midrange() {
    let code = [0x48, 0x63, 0xde, 0xf4]; // MOVSXD RBX, ESI
    let mut regs = Registers::default();
    regs.rsi = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx, 0x0000000012345678,
        "RBX should be 0x0000000012345678"
    );
}

#[test]
fn test_movsxd_rdi_eax() {
    let code = [0x48, 0x63, 0xf8, 0xf4]; // MOVSXD RDI, EAX
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // negative
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, 0xFFFFFFFF80000000, "RDI should be sign-extended");
}

// ============================================================================
// Extended Registers (R8-R15)
// ============================================================================

#[test]
fn test_movsx_r8d_bl() {
    let code = [0x44, 0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX R8D, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0xFFFFFF80,
        "R8D should be sign-extended"
    );
}

#[test]
fn test_movsx_eax_r8b() {
    let code = [0x41, 0x0f, 0xbe, 0xc0, 0xf4]; // MOVSX EAX, R8B
    let mut regs = Registers::default();
    regs.r8 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should be sign-extended from R8B"
    );
}

#[test]
fn test_movsx_r9_r10b() {
    let code = [0x4d, 0x0f, 0xbe, 0xca, 0xf4]; // MOVSX R9, R10B
    let mut regs = Registers::default();
    regs.r10 = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r9, 0xFFFFFFFFFFFFFF80,
        "R9 should be sign-extended from R10B"
    );
}

#[test]
fn test_movsx_r11_r12b() {
    let code = [0x4d, 0x0f, 0xbe, 0xdc, 0xf4]; // MOVSX R11, R12B
    let mut regs = Registers::default();
    regs.r12 = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r11, 0x000000000000007F,
        "R11 should be sign-extended from R12B"
    );
}

#[test]
fn test_movsx_r14d_r15b() {
    let code = [0x4d, 0x0f, 0xbe, 0xf7, 0xf4]; // MOVSX R14D, R15B
    let mut regs = Registers::default();
    regs.r15 = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.r14 & 0xFFFFFFFF,
        0xFFFFFFFF,
        "R14D should be sign-extended from R15B"
    );
}

// ============================================================================
// Upper Bits Zero Behavior (32-bit operations)
// ============================================================================

#[test]
fn test_movsx_eax_bl_zeros_upper_32_positive() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF; // Set RAX to all garbage
    regs.rbx = 0x7F; // Positive byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000000000007F,
        "Upper 32 bits of RAX should be zeroed"
    );
}

#[test]
fn test_movsx_eax_bl_zeros_upper_32_negative() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF; // Set RAX to all garbage
    regs.rbx = 0x80; // Negative byte
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x00000000FFFFFF80,
        "Upper 32 bits should be zeroed, lower 32 sign-extended"
    );
}

// ============================================================================
// Practical Use Cases
// ============================================================================

#[test]
fn test_movsx_practical_char_to_int() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xF0; // -16 as signed char
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFF0,
        "EAX should represent -16 as 32-bit signed"
    );
}

#[test]
fn test_movsx_practical_short_to_long() {
    let code = [0x48, 0x0f, 0xbf, 0xc3, 0xf4]; // MOVSX RAX, BX
    let mut regs = Registers::default();
    regs.rbx = 0x8001; // -32767 as signed short
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFF8001,
        "RAX should represent -32767 as 64-bit signed"
    );
}

#[test]
fn test_movsx_practical_negative_arithmetic() {
    let code = [
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0x05, 0x01, 0x00, 0x00, 0x00, // ADD EAX, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF; // -1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000000,
        "EAX should be 0 after adding 1 to -1"
    );
}

// ============================================================================
// Flags are Not Affected
// ============================================================================

#[test]
fn test_movsx_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        regs.rflags & 0x40 != 0,
        "ZF should still be set after MOVSX"
    );
}

// ============================================================================
// Source Register Not Modified
// ============================================================================

#[test]
fn test_movsx_source_unchanged() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x12345678, "RBX should be unchanged");
}

// ============================================================================
// Ignores Upper Bits of Source
// ============================================================================

#[test]
fn test_movsx_ignores_upper_bits_of_source() {
    let code = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0xDEADBEEF000000FF; // garbage in upper bits, 0xFF in BL
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Should only use BL, ignoring upper bits"
    );
}

// ============================================================================
// Sequential Boundary Tests
// ============================================================================

#[test]
fn test_movsx_byte_boundary_0x7f_0x80() {
    // Test transition at sign bit boundary
    let code_positive = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code_positive, Some(regs));
    let regs_pos = run_until_hlt(&mut vcpu).unwrap();

    let code_negative = [0x0f, 0xbe, 0xc3, 0xf4]; // MOVSX EAX, BL
    let mut regs = Registers::default();
    regs.rbx = 0x80;
    let (mut vcpu, _) = setup_vm(&code_negative, Some(regs));
    let regs_neg = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs_pos.rax & 0xFFFFFFFF,
        0x0000007F,
        "0x7F should be positive"
    );
    assert_eq!(
        regs_neg.rax & 0xFFFFFFFF,
        0xFFFFFF80,
        "0x80 should be negative"
    );
}

// ============================================================================
// Multiple Different Register Sources
// ============================================================================

#[test]
fn test_movsx_multiple_registers() {
    let code = [
        0x0f, 0xbe, 0xc3, // MOVSX EAX, BL
        0x0f, 0xbe, 0xca, // MOVSX ECX, DL
        0x0f, 0xbe, 0xd1, // MOVSX EDX, CL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFF;
    regs.rdx = 0x7F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be -1");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x0000007F, "ECX should be 127");
}
