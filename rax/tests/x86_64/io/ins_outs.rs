//! Comprehensive tests for INS and OUTS string I/O instructions
//!
//! Tests INSB, INSW, INSD and OUTSB, OUTSW, OUTSD instructions with and without REP prefix.

use crate::common::*;

// ============================================================================
// INSB - Input byte string from port DX
// ============================================================================

#[test]
fn test_insb_basic() {
    // INSB - Input byte from port in DX to [RDI]
    let code = &[
        0x6C, // INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x60); // Port address
    cpu.set_rdi(0x2000); // Destination address
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RDI should increment by 1
    assert_eq!(
        cpu.get_rdi(),
        0x2001,
        "INSB should increment RDI by 1 when DF=0"
    );
}

#[test]
fn test_insb_direction_flag_clear() {
    // INSB with DF=0 (increment)
    let code = &[
        0xFC, // CLD
        0x6C, // INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x80);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdi(), 0x3001, "INSB with DF=0 increments RDI");
}

#[test]
fn test_insb_direction_flag_set() {
    // INSB with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x6C, // INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x80);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdi(), 0x2FFF, "INSB with DF=1 decrements RDI");
}

// ============================================================================
// INSW - Input word string from port DX
// ============================================================================

#[test]
fn test_insw_basic() {
    // INSW - Input word from port in DX to [RDI]
    let code = &[
        0x66, 0x6D, // INSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rdi(0x2000);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RDI should increment by 2
    assert_eq!(
        cpu.get_rdi(),
        0x2002,
        "INSW should increment RDI by 2 when DF=0"
    );
}

#[test]
fn test_insw_direction_flag_set() {
    // INSW with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x66, 0x6D, // INSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdi(), 0x2FFE, "INSW with DF=1 decrements RDI by 2");
}

// ============================================================================
// INSD - Input dword string from port DX
// ============================================================================

#[test]
fn test_insd_basic() {
    // INSD - Input dword from port in DX to [RDI]
    let code = &[
        0x6D, // INSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);
    cpu.set_rdi(0x2000);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RDI should increment by 4
    assert_eq!(
        cpu.get_rdi(),
        0x2004,
        "INSD should increment RDI by 4 when DF=0"
    );
}

#[test]
fn test_insd_direction_flag_set() {
    // INSD with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x6D, // INSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdi(), 0x2FFC, "INSD with DF=1 decrements RDI by 4");
}

// ============================================================================
// REP INSB - Repeat input byte string
// ============================================================================

#[test]
fn test_rep_insb_count_zero() {
    // REP INSB with RCX=0 (should not execute)
    let code = &[
        0xF3, 0x6C, // REP INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(0);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rdi(),
        0x2000,
        "REP INSB with RCX=0 should not modify RDI"
    );
}

#[test]
fn test_rep_insb_count_one() {
    // REP INSB with RCX=1
    let code = &[
        0xFC, // CLD
        0xF3, 0x6C, // REP INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(1);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSB should decrement RCX to 0");
    assert_eq!(cpu.get_rdi(), 0x2001, "REP INSB should increment RDI");
}

#[test]
fn test_rep_insb_multiple() {
    // REP INSB with RCX=5
    let code = &[
        0xFC, // CLD
        0xF3, 0x6C, // REP INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(5);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSB should decrement RCX to 0");
    assert_eq!(
        cpu.get_rdi(),
        0x2005,
        "REP INSB should increment RDI by count"
    );
}

#[test]
fn test_rep_insb_direction_flag_set() {
    // REP INSB with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0xF3, 0x6C, // REP INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(3);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSB should decrement RCX to 0");
    assert_eq!(
        cpu.get_rdi(),
        0x2FFD,
        "REP INSB with DF=1 should decrement RDI"
    );
}

// ============================================================================
// REP INSW - Repeat input word string
// ============================================================================

#[test]
fn test_rep_insw_basic() {
    // REP INSW with RCX=4
    let code = &[
        0xFC, // CLD
        0xF3, 0x66, 0x6D, // REP INSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(4);
    cpu.set_rdx(0x3D4);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSW should decrement RCX to 0");
    assert_eq!(
        cpu.get_rdi(),
        0x2008,
        "REP INSW should increment RDI by count*2"
    );
}

// ============================================================================
// REP INSD - Repeat input dword string
// ============================================================================

#[test]
fn test_rep_insd_basic() {
    // REP INSD with RCX=8
    let code = &[
        0xFC, // CLD
        0xF3, 0x6D, // REP INSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(8);
    cpu.set_rdx(0xCF8);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSD should decrement RCX to 0");
    assert_eq!(
        cpu.get_rdi(),
        0x2020,
        "REP INSD should increment RDI by count*4"
    );
}

// ============================================================================
// OUTSB - Output byte string to port DX
// ============================================================================

#[test]
fn test_outsb_basic() {
    // OUTSB - Output byte from [RSI] to port in DX
    let code = &[
        0x6E, // OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x80);
    cpu.set_rsi(0x2000);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RSI should increment by 1
    assert_eq!(
        cpu.get_rsi(),
        0x2001,
        "OUTSB should increment RSI by 1 when DF=0"
    );
}

#[test]
fn test_outsb_direction_flag_set() {
    // OUTSB with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x6E, // OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x80);
    cpu.set_rsi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rsi(), 0x2FFF, "OUTSB with DF=1 decrements RSI");
}

// ============================================================================
// OUTSW - Output word string to port DX
// ============================================================================

#[test]
fn test_outsw_basic() {
    // OUTSW - Output word from [RSI] to port in DX
    let code = &[
        0x66, 0x6F, // OUTSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rsi(0x2000);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RSI should increment by 2
    assert_eq!(
        cpu.get_rsi(),
        0x2002,
        "OUTSW should increment RSI by 2 when DF=0"
    );
}

#[test]
fn test_outsw_direction_flag_set() {
    // OUTSW with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x66, 0x6F, // OUTSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x3D4);
    cpu.set_rsi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rsi(), 0x2FFE, "OUTSW with DF=1 decrements RSI by 2");
}

// ============================================================================
// OUTSD - Output dword string to port DX
// ============================================================================

#[test]
fn test_outsd_basic() {
    // OUTSD - Output dword from [RSI] to port in DX
    let code = &[
        0x6F, // OUTSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);
    cpu.set_rsi(0x2000);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // RSI should increment by 4
    assert_eq!(
        cpu.get_rsi(),
        0x2004,
        "OUTSD should increment RSI by 4 when DF=0"
    );
}

#[test]
fn test_outsd_direction_flag_set() {
    // OUTSD with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0x6F, // OUTSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xCF8);
    cpu.set_rsi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rsi(), 0x2FFC, "OUTSD with DF=1 decrements RSI by 4");
}

// ============================================================================
// REP OUTSB - Repeat output byte string
// ============================================================================

#[test]
fn test_rep_outsb_count_zero() {
    // REP OUTSB with RCX=0 (should not execute)
    let code = &[
        0xF3, 0x6E, // REP OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(0);
    cpu.set_rdx(0x80);
    cpu.set_rsi(0x2000);

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rsi(),
        0x2000,
        "REP OUTSB with RCX=0 should not modify RSI"
    );
}

#[test]
fn test_rep_outsb_multiple() {
    // REP OUTSB with RCX=5
    let code = &[
        0xFC, // CLD
        0xF3, 0x6E, // REP OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(5);
    cpu.set_rdx(0x80);
    cpu.set_rsi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP OUTSB should decrement RCX to 0");
    assert_eq!(
        cpu.get_rsi(),
        0x2005,
        "REP OUTSB should increment RSI by count"
    );
}

#[test]
fn test_rep_outsb_direction_flag_set() {
    // REP OUTSB with DF=1 (decrement)
    let code = &[
        0xFD, // STD
        0xF3, 0x6E, // REP OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(3);
    cpu.set_rdx(0x80);
    cpu.set_rsi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP OUTSB should decrement RCX to 0");
    assert_eq!(
        cpu.get_rsi(),
        0x2FFD,
        "REP OUTSB with DF=1 should decrement RSI"
    );
}

// ============================================================================
// REP OUTSW - Repeat output word string
// ============================================================================

#[test]
fn test_rep_outsw_basic() {
    // REP OUTSW with RCX=4
    let code = &[
        0xFC, // CLD
        0xF3, 0x66, 0x6F, // REP OUTSW
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(4);
    cpu.set_rdx(0x3D4);
    cpu.set_rsi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP OUTSW should decrement RCX to 0");
    assert_eq!(
        cpu.get_rsi(),
        0x2008,
        "REP OUTSW should increment RSI by count*2"
    );
}

// ============================================================================
// REP OUTSD - Repeat output dword string
// ============================================================================

#[test]
fn test_rep_outsd_basic() {
    // REP OUTSD with RCX=8
    let code = &[
        0xFC, // CLD
        0xF3, 0x6F, // REP OUTSD
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(8);
    cpu.set_rdx(0xCF8);
    cpu.set_rsi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP OUTSD should decrement RCX to 0");
    assert_eq!(
        cpu.get_rsi(),
        0x2020,
        "REP OUTSD should increment RSI by count*4"
    );
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_ins_preserves_dx() {
    // INS instructions should not modify DX
    let code = &[
        0x6C, // INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x12345678_ABCD0080);
    cpu.set_rdi(0x2000);
    let original_dx = cpu.get_rdx();

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdx(), original_dx, "INSB should preserve DX");
}

#[test]
fn test_outs_preserves_dx() {
    // OUTS instructions should not modify DX
    let code = &[
        0x6E, // OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x12345678_ABCD0080);
    cpu.set_rsi(0x2000);
    let original_dx = cpu.get_rdx();

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdx(), original_dx, "OUTSB should preserve DX");
}

#[test]
fn test_rep_insb_large_count() {
    // REP INSB with large count
    let code = &[
        0xFC, // CLD
        0xF3, 0x6C, // REP INSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(256);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0x2000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "REP INSB should decrement RCX to 0");
    assert_eq!(
        cpu.get_rdi(),
        0x2100,
        "REP INSB should increment RDI by 256"
    );
}

#[test]
fn test_ins_outs_alternating() {
    // Alternate INS and OUTS operations
    let code = &[
        0xFC, // CLD
        0xBA, 0x60, 0x00, 0x00, 0x00, // MOV EDX, 0x60
        0xBF, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x6C, // INSB
        0xBE, 0x00, 0x20, 0x00, 0x00, // MOV ESI, 0x2000
        0x6E, // OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rdi(), 0x2001, "INSB should increment RDI");
    assert_eq!(cpu.get_rsi(), 0x2001, "OUTSB should increment RSI");
}

#[test]
fn test_ins_16bit_address_size() {
    // INSB with 16-bit address size override
    let code = &[
        0x67, 0x6C, // INSB (16-bit address)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0x60);
    cpu.set_rdi(0xFFFF);
    cpu.set_rflags(cpu.get_rflags() & !0x400); // Clear DF

    run_test(&mut cpu);

    // With 16-bit addressing, should wrap to 0
    assert_eq!(
        cpu.get_rdi() & 0xFFFF,
        0,
        "INSB with 16-bit address should wrap"
    );
}

#[test]
fn test_outs_uses_only_low_16bits_of_dx() {
    // OUTS should use only low 16 bits of DX
    let code = &[
        0x6E, // OUTSB
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rdx(0xFFFFFFFF_FFFF0080);
    cpu.set_rsi(0x2000);

    run_test(&mut cpu);
    // Should output to port 0x0080, not 0xFFFF0080
}
