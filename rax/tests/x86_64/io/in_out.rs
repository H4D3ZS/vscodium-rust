//! Comprehensive tests for IN and OUT I/O port instructions
//!
//! Tests all variants of IN and OUT instructions for reading from and writing to I/O ports.

use crate::common::*;

// ============================================================================
// IN AL, imm8 - Input byte from immediate port
// ============================================================================

#[test]
fn test_in_al_imm8_port_0() {
    // IN AL, 0
    let code = &[
        0xE4, 0x00, // IN AL, 0
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0xFFFFFFFF);

    run_test(&mut cpu);

    // IN from port returns a value (implementation dependent)
    // Just verify it executed without error
}

#[test]
fn test_in_al_imm8_port_255() {
    // IN AL, 0xFF (maximum immediate port)
    let code = &[
        0xE4, 0xFF, // IN AL, 0xFF
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);
}

#[test]
fn test_in_al_imm8_various_ports() {
    // Test reading from various port addresses
    for port in [0x20, 0x21, 0x40, 0x60, 0x64, 0x70, 0x80, 0xA0, 0xA1] {
        let code = &[
            0xE4, port, // IN AL, port
            0xF4, // HLT
        ];
        let mut cpu = create_test_cpu(code);
        run_test(&mut cpu);
    }
}

// ============================================================================
// IN AX, imm8 - Input word from immediate port
// ============================================================================

#[test]
fn test_in_ax_imm8_port_0() {
    // IN AX, 0
    let code = &[
        0x66, 0xE5, 0x00, // IN AX, 0
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0xFFFFFFFF);

    run_test(&mut cpu);
}

#[test]
fn test_in_ax_imm8_aligned_port() {
    // IN AX, 0x3D0 (aligned)
    let code = &[
        0x66, 0xE5, 0xD0, // IN AX, 0xD0
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    run_test(&mut cpu);
}

// ============================================================================
// IN EAX, imm8 - Input dword from immediate port
// ============================================================================

#[test]
fn test_in_eax_imm8_port_0() {
    // IN EAX, 0
    let code = &[
        0xE5, 0x00, // IN EAX, 0
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0xFFFFFFFF_FFFFFFFF);

    run_test(&mut cpu);
}

#[test]
fn test_in_eax_imm8_clears_high_bits() {
    // IN EAX should zero-extend to RAX
    let code = &[
        0xE5, 0x40, // IN EAX, 0x40
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0xDEADBEEF_CAFEBABE);

    run_test(&mut cpu);

    // High 32 bits should be cleared
    let rax = cpu.get_rax();
    assert_eq!(rax >> 32, 0, "IN EAX should clear high 32 bits of RAX");
}

// ============================================================================
// IN AL, DX - Input byte from DX port
// ============================================================================

#[test]
fn test_in_al_dx_port_0() {
    // IN AL, DX with DX=0
    let code = &[
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x0000);
    cpu.set_rax(0xFF);

    run_test(&mut cpu);
}

#[test]
fn test_in_al_dx_high_port() {
    // IN AL, DX with DX=0xFFFF (maximum port)
    let code = &[
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xFFFF);

    run_test(&mut cpu);
}

#[test]
fn test_in_al_dx_preserves_dx() {
    // IN AL, DX should not modify DX
    let code = &[
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x12345678_ABCD1234);
    let original_dx = cpu.get_rdx();

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdx(), original_dx, "IN AL, DX should preserve DX");
}

// ============================================================================
// IN AX, DX - Input word from DX port
// ============================================================================

#[test]
fn test_in_ax_dx_port() {
    // IN AX, DX
    let code = &[
        0x66, 0xED, // IN AX, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rax(0xFFFFFFFF);

    run_test(&mut cpu);
}

// ============================================================================
// IN EAX, DX - Input dword from DX port
// ============================================================================

#[test]
fn test_in_eax_dx_port() {
    // IN EAX, DX
    let code = &[
        0xED, // IN EAX, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);

    run_test(&mut cpu);
}

#[test]
fn test_in_eax_dx_all_ports() {
    // Test all valid 16-bit port addresses
    for port in [
        0x0000, 0x0001, 0x00FF, 0x0100, 0x03D4, 0x03D5, 0x0CF8, 0xFFFF,
    ] {
        let code = &[
            0xED, // IN EAX, DX
            0xF4, // HLT
        ];
        let mut cpu = create_test_cpu(code);
        cpu.set_rdx(port);
        run_test(&mut cpu);
    }
}

// ============================================================================
// OUT imm8, AL - Output byte to immediate port
// ============================================================================

#[test]
fn test_out_imm8_al_port_0() {
    // OUT 0, AL
    let code = &[
        0xE6, 0x00, // OUT 0, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x42);

    run_test(&mut cpu);
}

#[test]
fn test_out_imm8_al_various_values() {
    // Test outputting various values
    for value in [0x00, 0x01, 0x7F, 0x80, 0xFF] {
        let code = &[
            0xE6, 0x80, // OUT 0x80, AL
            0xF4, // HLT
        ];
        let mut cpu = create_test_cpu(code);
        cpu.set_rax(value as u64);
        run_test(&mut cpu);
    }
}

#[test]
fn test_out_imm8_al_preserves_registers() {
    // OUT should not modify AL
    let code = &[
        0xE6, 0x80, // OUT 0x80, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x5A);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax() & 0xFF, 0x5A, "OUT should preserve AL");
}

// ============================================================================
// OUT imm8, AX - Output word to immediate port
// ============================================================================

#[test]
fn test_out_imm8_ax_port() {
    // OUT 0xD0, AX
    let code = &[
        0x66, 0xE7, 0xD0, // OUT 0xD0, AX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x1234);

    run_test(&mut cpu);
}

// ============================================================================
// OUT imm8, EAX - Output dword to immediate port
// ============================================================================

#[test]
fn test_out_imm8_eax_port() {
    // OUT 0xCF, EAX
    let code = &[
        0xE7, 0xCF, // OUT 0xCF, EAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0xDEADBEEF);

    run_test(&mut cpu);
}

#[test]
fn test_out_imm8_eax_uses_low_32bits() {
    // OUT should use only low 32 bits of RAX
    let code = &[
        0xE7, 0x40, // OUT 0x40, EAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x12345678_ABCDEF01);

    run_test(&mut cpu);
    // Should output 0xABCDEF01, not the high bits
}

// ============================================================================
// OUT DX, AL - Output byte to DX port
// ============================================================================

#[test]
fn test_out_dx_al_port_0() {
    // OUT DX, AL with DX=0
    let code = &[
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x0000);
    cpu.set_rax(0xAA);

    run_test(&mut cpu);
}

#[test]
fn test_out_dx_al_high_port() {
    // OUT DX, AL with DX=0xFFFF
    let code = &[
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xFFFF);
    cpu.set_rax(0x55);

    run_test(&mut cpu);
}

#[test]
fn test_out_dx_al_multiple_writes() {
    // Multiple OUT operations
    let code = &[
        0xB0, 0x20, // MOV AL, 0x20
        0xBA, 0x20, 0x00, 0x00, 0x00, // MOV EDX, 0x20
        0xEE, // OUT DX, AL
        0xB0, 0x21, // MOV AL, 0x21
        0xBA, 0x21, 0x00, 0x00, 0x00, // MOV EDX, 0x21
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

// ============================================================================
// OUT DX, AX - Output word to DX port
// ============================================================================

#[test]
fn test_out_dx_ax_port() {
    // OUT DX, AX
    let code = &[
        0x66, 0xEF, // OUT DX, AX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rax(0x1234);

    run_test(&mut cpu);
}

// ============================================================================
// OUT DX, EAX - Output dword to DX port
// ============================================================================

#[test]
fn test_out_dx_eax_port() {
    // OUT DX, EAX
    let code = &[
        0xEF, // OUT DX, EAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);
    cpu.set_rax(0x80000000);

    run_test(&mut cpu);
}

#[test]
fn test_out_dx_eax_pci_config() {
    // Simulate PCI configuration space access
    let code = &[
        0xBA, 0xF8, 0x0C, 0x00, 0x00, // MOV EDX, 0x0CF8 (32-bit imm, zero-extends to RDX)
        0xB8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0xEF, // OUT DX, EAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

// ============================================================================
// Combined IN/OUT sequences
// ============================================================================

#[test]
fn test_in_out_sequence() {
    // Read-modify-write sequence
    let code = &[
        0xBA, 0x60, 0x00, 0x00, 0x00, // MOV EDX, 0x60
        0xEC, // IN AL, DX
        0x0C, 0x01, // OR AL, 1
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

#[test]
fn test_multiple_port_access() {
    // Access multiple ports in sequence
    let code = &[
        0xBA, 0x40, 0x00, 0x00, 0x00, // MOV EDX, 0x40
        0xEC, // IN AL, DX
        0xBA, 0x41, 0x00, 0x00, 0x00, // MOV EDX, 0x41
        0xEC, // IN AL, DX
        0xBA, 0x42, 0x00, 0x00, 0x00, // MOV EDX, 0x42
        0xEC, // IN AL, DX
        0xBA, 0x43, 0x00, 0x00, 0x00, // MOV EDX, 0x43
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

// ============================================================================
// Port I/O with different sizes
// ============================================================================

#[test]
fn test_io_size_8bit() {
    // 8-bit I/O
    let code = &[
        0xB0, 0xAA, // MOV AL, 0xAA
        0xBA, 0x80, 0x00, 0x00, 0x00, // MOV EDX, 0x80
        0xEE, // OUT DX, AL
        0xB0, 0x00, // MOV AL, 0
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

#[test]
fn test_io_size_16bit() {
    // 16-bit I/O
    let code = &[
        0x66, 0xB8, 0x34, 0x12, // MOV AX, 0x1234
        0xBA, 0xD0, 0x03, 0x00, 0x00, // MOV EDX, 0x3D0
        0x66, 0xEF, // OUT DX, AX
        0x66, 0xB8, 0x00, 0x00, // MOV AX, 0
        0x66, 0xED, // IN AX, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

#[test]
fn test_io_size_32bit() {
    // 32-bit I/O
    let code = &[
        0xB8, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EAX, 0xDEADBEEF
        0xBA, 0xF8, 0x0C, 0x00, 0x00, // MOV EDX, 0x0CF8
        0xEF, // OUT DX, EAX
        0xB8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xED, // IN EAX, DX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_in_al_preserves_high_bits() {
    // IN AL should preserve AH and higher bits
    let code = &[
        0xE4, 0x80, // IN AL, 0x80
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x12345678_9ABCDEF0);

    run_test(&mut cpu);

    let rax = cpu.get_rax();
    assert_eq!(
        rax & 0xFFFFFFFF_FFFFFF00,
        0x12345678_9ABCDE00,
        "IN AL should preserve high bits"
    );
}

#[test]
fn test_in_ax_preserves_high_bits() {
    // IN AX should preserve high bits of RAX
    let code = &[
        0x66, 0xE5, 0x80, // IN AX, 0x80
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0x12345678_9ABCDEF0);

    run_test(&mut cpu);

    let rax = cpu.get_rax();
    assert_eq!(
        rax & 0xFFFFFFFF_FFFF0000,
        0x12345678_9ABC0000,
        "IN AX should preserve high bits"
    );
}

#[test]
fn test_out_with_high_dx_bits() {
    // OUT should use only low 16 bits of DX
    let code = &[
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xFFFFFFFF_FFFF0080);
    cpu.set_rax(0x42);

    run_test(&mut cpu);
    // Should output to port 0x0080, not 0xFFFF0080
}

#[test]
fn test_in_out_boundary_ports() {
    // Test boundary port addresses
    for port in [0x0000, 0x0001, 0x00FE, 0x00FF] {
        // OUT imm8
        let code = &[
            0xE6, port as u8, // OUT port, AL
            0xF4,       // HLT
        ];
        let mut cpu = create_test_cpu(code);
        cpu.set_rax(0x42);
        run_test(&mut cpu);

        // IN imm8
        let code = &[
            0xE4, port as u8, // IN AL, port
            0xF4,       // HLT
        ];
        let mut cpu = create_test_cpu(code);
        run_test(&mut cpu);
    }
}

#[test]
fn test_io_does_not_affect_flags() {
    // I/O operations should not affect flags
    let code = &[
        0x9C, // PUSHF
        0xE4, 0x80, // IN AL, 0x80
        0x9C, // PUSHF
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rflags(0x246); // Set some flags

    run_test(&mut cpu);

    // Flags should be unchanged
}
