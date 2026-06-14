//! Comprehensive tests for AAM and AAD BCD adjustment instructions
//!
//! Tests AAM (ASCII Adjust after Multiply) and AAD (ASCII Adjust before Division)
//! with default base 10 and custom bases.

use crate::common::*;

fn assert_invalid_bcd_opcode_raises_ud(opcode: &[u8]) {
    let mut code = Vec::from(opcode);
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0xf4, // HLT
    ]);
    let (mut vcpu, _) = setup_vm_no_idt(&code, None);

    match vcpu.run() {
        Ok(VcpuExit::Hlt) => panic!("BCD opcode {opcode:x?} must raise #UD in 64-bit mode"),
        Ok(VcpuExit::Shutdown) | Err(_) => {}
        _ => {}
    }
}

// ============================================================================
// AAM - ASCII Adjust AX After Multiply
// ============================================================================

#[test]
fn test_aam_basic_base_10() {
    // AAM with default base 10
    let code = &[
        0xB0, 0x05, // MOV AL, 5
        0xB4, 0x03, // MOV AH, 3
        0xF6, 0xE4, // MUL AH (AL = 5 * 3 = 15)
        0xD4, 0x0A, // AAM (AH = 15 / 10 = 1, AL = 15 % 10 = 5)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 5, "AAM: AL = 15 % 10 = 5");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 1, "AAM: AH = 15 / 10 = 1");
}

#[test]
fn test_aam_value_99() {
    // AAM with AL=99 (maximum BCD value)
    let code = &[
        0xB0, 0x63, // MOV AL, 99
        0xD4, 0x0A, // AAM
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 9, "AAM: AL = 99 % 10 = 9");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 9, "AAM: AH = 99 / 10 = 9");
}

#[test]
fn test_aam_base_8_octal() {
    // AAM with base 8 (octal)
    let code = &[
        0xB0, 0x3F, // MOV AL, 63
        0xD4, 0x08, // AAM 8 (AH = 63 / 8 = 7, AL = 63 % 8 = 7)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 7, "AAM base 8: AL = 63 % 8 = 7");
    assert_eq!(
        (cpu.get_rax() >> 8) & 0xFF,
        7,
        "AAM base 8: AH = 63 / 8 = 7"
    );
}

#[test]
fn test_aam_base_16_hex() {
    // AAM with base 16 (hexadecimal)
    let code = &[
        0xB0, 0xFF, // MOV AL, 255
        0xD4, 0x10, // AAM 16 (AH = 255 / 16 = 15, AL = 255 % 16 = 15)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 15, "AAM base 16: AL = 255 % 16 = 15");
    assert_eq!(
        (cpu.get_rax() >> 8) & 0xFF,
        15,
        "AAM base 16: AH = 255 / 16 = 15"
    );
}

#[test]
fn test_aam_base_2_binary() {
    // AAM with base 2 (binary)
    let code = &[
        0xB0, 0x05, // MOV AL, 5
        0xD4, 0x02, // AAM 2 (AH = 5 / 2 = 2, AL = 5 % 2 = 1)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 1, "AAM base 2: AL = 5 % 2 = 1");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 2, "AAM base 2: AH = 5 / 2 = 2");
}

// ============================================================================
// AAD - ASCII Adjust AX Before Division
// ============================================================================

#[test]
fn test_aad_basic_base_10() {
    // AAD with default base 10
    let code = &[
        0xB0, 0x05, // MOV AL, 5
        0xB4, 0x03, // MOV AH, 3 (represents 35 in BCD)
        0xD5, 0x0A, // AAD (AL = 3 * 10 + 5 = 35, AH = 0)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 35, "AAD: AL = 3 * 10 + 5 = 35");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 0, "AAD: AH = 0");
}

#[test]
fn test_aad_max_bcd() {
    // AAD with maximum BCD value (9,9)
    let code = &[
        0xB0, 0x09, // MOV AL, 9
        0xB4, 0x09, // MOV AH, 9 (represents 99)
        0xD5, 0x0A, // AAD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 99, "AAD: AL = 9 * 10 + 9 = 99");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 0, "AAD: AH = 0");
}

#[test]
fn test_aad_base_8_octal() {
    // AAD with base 8 (octal)
    let code = &[
        0xB0, 0x07, // MOV AL, 7
        0xB4, 0x06, // MOV AH, 6 (represents 67 octal = 55 decimal)
        0xD5, 0x08, // AAD 8 (AL = 6 * 8 + 7 = 55)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 55, "AAD base 8: AL = 6 * 8 + 7 = 55");
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 0, "AAD base 8: AH = 0");
}

#[test]
fn test_aad_base_16_hex() {
    // AAD with base 16 (hexadecimal)
    let code = &[
        0xB0, 0x0F, // MOV AL, 15
        0xB4, 0x0A, // MOV AH, 10 (represents AF hex = 175 decimal)
        0xD5, 0x10, // AAD 16 (AL = 10 * 16 + 15 = 175)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rax() & 0xFF,
        175,
        "AAD base 16: AL = 10 * 16 + 15 = 175"
    );
    assert_eq!((cpu.get_rax() >> 8) & 0xFF, 0, "AAD base 16: AH = 0");
}

#[test]
fn test_aam_aad_roundtrip() {
    // AAM followed by AAD should give original value
    let code = &[
        0xB0, 0x42, // MOV AL, 66
        0xD4, 0x0A, // AAM (AH=6, AL=6)
        0xD5, 0x0A, // AAD (AL=66)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 66, "AAM/AAD roundtrip");
}

#[test]
fn test_aam_aad_custom_base_roundtrip() {
    // AAM/AAD roundtrip with custom base
    let code = &[
        0xB0, 0x37, // MOV AL, 55
        0xD4, 0x07, // AAM 7 (AH=7, AL=6 since 55=7*7+6)
        0xD5, 0x07, // AAD 7 (AL=55)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu_compat(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 55, "AAM/AAD roundtrip base 7");
}

#[test]
fn test_aam_invalid_in_64bit_mode_raises_ud() {
    assert_invalid_bcd_opcode_raises_ud(&[0xd4, 0x0a]);
}
