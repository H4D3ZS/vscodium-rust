//! Comprehensive tests for LOOP, LOOPE, LOOPNE, JCXZ/JECXZ/JRCXZ instructions
//!
//! Tests all variants of loop instructions including different counter sizes and conditions.

use crate::common::*;

// ============================================================================
// LOOP - Loop with RCX/ECX/CX counter
// ============================================================================

#[test]
fn test_loop_basic_count_1() {
    // LOOP with RCX=1 (executes once, then falls through)
    let code = &[
        0xB9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x48, 0xFF, 0xC0, // INC RAX (loop body)
        0xE2, 0xFB, // LOOP -5 (back to INC)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 1, "LOOP with RCX=1 should execute once");
    assert_eq!(cpu.get_rcx(), 0, "RCX should be decremented to 0");
}

// NOTE: test_loop_count_zero is removed because LOOP with ECX=0 is untestable.
// LOOP decrements ECX first, then checks if non-zero. When ECX=0, it wraps to
// 0xFFFFFFFF and loops ~4 billion times, which is not practical to test.
// The x86 way to handle "skip if counter is zero" is JECXZ, not LOOP.

#[test]
fn test_jecxz_skips_when_zero() {
    // JECXZ jumps if ECX is zero - the proper way to skip a loop body
    let code = &[
        0xB9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x67, 0xE3, 0x03, // JECXZ +3 (skip INC RAX)
        0x48, 0xFF, 0xC0, // INC RAX (skipped)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 0, "JECXZ should skip body when ECX=0");
    assert_eq!(cpu.get_rcx() & 0xFFFFFFFF, 0, "ECX should remain 0");
}

#[test]
fn test_loop_count_5() {
    // LOOP executes 5 times
    let code = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0x48, 0xFF, 0xC0, // INC RAX
        0xE2, 0xFB, // LOOP -5
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 5, "LOOP should execute 5 times");
    assert_eq!(cpu.get_rcx(), 0, "RCX should be 0 after loop");
}

#[test]
fn test_loop_count_100() {
    // LOOP with larger count
    let code = &[
        0xB9, 0x64, 0x00, 0x00, 0x00, // MOV ECX, 100
        0x48, 0xFF, 0xC0, // INC RAX
        0xE2, 0xFB, // LOOP -5
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 100, "LOOP should execute 100 times");
    assert_eq!(cpu.get_rcx(), 0, "RCX should be 0 after loop");
}

#[test]
fn test_loop_nested() {
    // Nested LOOP
    // Positions: 0-4 MOV ECX, 5 PUSH RBX, 6-10 MOV EBX, 11-13 INC RAX,
    //            14-15 DEC EBX, 16-17 JNZ, 18-19 LOOP, 20 POP, 21 HLT
    let code = &[
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3 (outer) [0-4]
        0x53, // PUSH RBX [5]
        0xBB, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2 (inner) [6-10]
        0x48, 0xFF, 0xC0, // INC RAX (inner body) [11-13]
        0xFF, 0xCB, // DEC EBX [14-15]
        0x75, 0xF9, // JNZ -7 (inner loop: 18 - 7 = 11) [16-17]
        0xE2, 0xF1, // LOOP outer (20 - 15 = 5) [18-19]
        0x5B, // POP RBX [20]
        0xF4, // HLT [21]
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 6, "Nested LOOP 3*2 should execute 6 times");
}

// ============================================================================
// LOOPE/LOOPZ - Loop while ZF=1
// ============================================================================

#[test]
fn test_loope_zf_stays_set() {
    // LOOPE continues while ZF=1
    let code = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0x31, 0xC0, // XOR EAX, EAX (sets ZF=1)
        0xE1, 0xFC, // LOOPE -4 (back to XOR)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(1); // Start with non-zero

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rcx(),
        0,
        "LOOPE should decrement RCX to 0 when ZF=1"
    );
}

#[test]
fn test_loope_zf_becomes_clear() {
    // LOOPE exits early when ZF becomes 0
    let code = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0x48, 0xFF, 0xC0, // INC RAX (clears ZF after RAX becomes non-zero)
        0xE1, 0xFB, // LOOPE -5
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    // Should exit after first iteration when ZF becomes 0
    assert_eq!(cpu.get_rax(), 1, "LOOPE should exit when ZF=0");
    assert_eq!(cpu.get_rcx(), 4, "RCX should be 4 after early exit");
}

#[test]
fn test_loopne_with_cmp() {
    // LOOPNE loops while ZF=0 (not equal), stops when ZF=1 (equal) or ECX=0
    // This loops until CMP RBX, 5 sets ZF=1 (when RBX equals 5)
    // Positions: 0-4 MOV ECX, 5-9 MOV EBX, 10-12 INC, 13-16 CMP, 17-18 LOOPNE, 19 HLT
    let code = &[
        0xB9, 0x0A, 0x00, 0x00, 0x00, // MOV ECX, 10 [0-4]
        0xBB, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0 [5-9]
        0x48, 0xFF, 0xC3, // INC RBX (loop body) [10-12]
        0x48, 0x83, 0xFB, 0x05, // CMP RBX, 5 [13-16]
        0xE0, 0xF7, // LOOPNE -9 (19 - 9 = 10) [17-18]
        0xF4, // HLT [19]
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rbx(), 5, "LOOPNE should stop when RBX=5 (ZF=1)");
    assert_eq!(cpu.get_rcx(), 5, "RCX should be 5 (10-5 iterations)");
}

// ============================================================================
// LOOPNE/LOOPNZ - Loop while ZF=0
// ============================================================================

#[test]
fn test_loopne_zf_stays_clear() {
    // LOOPNE continues while ZF=0
    let code = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0x48, 0xFF, 0xC0, // INC RAX (keeps ZF=0 as long as RAX != 0)
        0xE0, 0xFB, // LOOPNE -5
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);
    cpu.set_rflags(cpu.get_rflags() & !0x40); // Clear ZF

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rcx(),
        0,
        "LOOPNE should decrement RCX to 0 when ZF=0"
    );
    assert_eq!(cpu.get_rax(), 5, "Loop body should execute 5 times");
}

#[test]
fn test_loopne_zf_becomes_set() {
    // LOOPNE exits early when ZF becomes 1
    let code = &[
        0xB9, 0x05, 0x00, 0x00, 0x00, // MOV ECX, 5
        0xFF, 0xC8, // DEC EAX (sets ZF when EAX=0)
        0xE0, 0xFC, // LOOPNE -4
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(2); // Will decrement to 0

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 0, "LOOPNE should stop when RAX becomes 0");
    assert_eq!(cpu.get_rcx(), 3, "RCX should be 3 after 2 iterations");
}

// ============================================================================
// JCXZ/JECXZ/JRCXZ - Jump if CX/ECX/RCX is zero
// ============================================================================

#[test]
fn test_jrcxz_rcx_zero() {
    // JRCXZ when RCX=0 (should jump)
    let code = &[
        0xE3, 0x03, // JRCXZ +3
        0x48, 0xFF, 0xC0, // INC RAX (skipped)
        0xF4, // HLT (jumped to)
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(0);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 0, "JRCXZ should skip INC when RCX=0");
}

#[test]
fn test_jrcxz_rcx_nonzero() {
    // JRCXZ when RCX!=0 (should not jump)
    let code = &[
        0xE3, 0x03, // JRCXZ +3
        0x48, 0xFF, 0xC0, // INC RAX (not skipped)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(1);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 1, "JRCXZ should not skip when RCX!=0");
}

#[test]
fn test_jrcxz_high_bits_matter() {
    // JRCXZ checks all 64 bits of RCX
    let code = &[
        0xE3, 0x03, // JRCXZ +3
        0x48, 0xFF, 0xC0, // INC RAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(0x100000000); // High bit set, low bits 0
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rax(),
        1,
        "JRCXZ should not jump when high bits are set"
    );
}

#[test]
fn test_jecxz_32bit() {
    // JECXZ with 32-bit address size (checks ECX only)
    let code = &[
        0x67, 0xE3, 0x03, // JECXZ +3 (32-bit)
        0x48, 0xFF, 0xC0, // INC RAX
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(0x100000000); // High bits set, ECX=0
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(
        cpu.get_rax(),
        0,
        "JECXZ should jump when ECX=0 (ignoring high bits)"
    );
}

#[test]
fn test_jrcxz_backward_jump() {
    // JRCXZ with backward jump (create simple loop)
    let code = &[
        0x48, 0xFF, 0xC0, // INC RAX (start)
        0x48, 0xFF, 0xC9, // DEC RCX
        0xE3, 0x02, // JRCXZ +2 (forward to HLT)
        0xEB, 0xF7, // JMP -9 (back to INC RAX)
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(3);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 3, "Should loop 3 times");
    assert_eq!(cpu.get_rcx(), 0, "RCX should be 0");
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_loop_with_string_operations() {
    // LOOP combined with string operations
    let code = &[
        0xB9, 0x04, 0x00, 0x00, 0x00, // MOV ECX, 4
        0xFC, // CLD
        0xAC, // LODSB (AL = [RSI++])
        0xAA, // STOSB ([RDI++] = AL)
        0xE2, 0xFA, // LOOP -6
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(4);
    cpu.set_rsi(0x2000);
    cpu.set_rdi(0x3000);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rcx(), 0, "Loop should complete");
    assert_eq!(cpu.get_rsi(), 0x2004, "RSI should advance by 4");
    assert_eq!(cpu.get_rdi(), 0x3004, "RDI should advance by 4");
}

#[test]
fn test_loopne_search_pattern() {
    // Use LOOPNE to search for a value
    // Positions: 0-4 MOV ECX, 5-9 MOV EBX, 10-12 INC, 13-16 CMP, 17-18 LOOPNE, 19 HLT
    let code = &[
        0xB9, 0x0A, 0x00, 0x00, 0x00, // MOV ECX, 10 [0-4]
        0xBB, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0 [5-9]
        0x48, 0xFF, 0xC3, // INC RBX [10-12]
        0x48, 0x83, 0xFB, 0x07, // CMP RBX, 7 [13-16]
        0xE0, 0xF7, // LOOPNE -9 (19 - 9 = 10) [17-18]
        0xF4, // HLT [19]
    ];
    let mut cpu = create_test_cpu(code);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rbx(), 7, "Should find value 7");
    assert_eq!(cpu.get_rcx(), 3, "Should have 3 iterations left");
}

#[test]
fn test_all_loop_variants_in_sequence() {
    // Test LOOP, LOOPE, LOOPNE in one test
    let code = &[
        // LOOP test
        0xB9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x48, 0xFF, 0xC0, // INC RAX
        0xE2, 0xFB, // LOOP -5
        // LOOPE test (ZF=1)
        0xB9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x09, 0xDB, // OR EBX, EBX (sets ZF if EBX=0)
        0xE1, 0xFC, // LOOPE -4
        // LOOPNE test (ZF=0)
        0xB9, 0x02, 0x00, 0x00, 0x00, // MOV ECX, 2
        0x48, 0xFF, 0xC2, // INC RDX (clears ZF)
        0xE0, 0xFB, // LOOPNE -5
        0xF4, // HLT
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);
    cpu.set_rbx(0);
    cpu.set_rdx(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 2, "LOOP iterations");
    assert_eq!(cpu.get_rdx(), 2, "LOOPNE iterations");
}

#[test]
fn test_loop_forward_backward_combinations() {
    // Test forward and backward LOOP jumps
    // Positions: 0-4 MOV ECX, 5-7 INC, 8-11 CMP, 12-13 JZ, 14-15 LOOP, 16-20 MOV ECX, 21-22 JMP, 23 HLT
    let code = &[
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3 [0-4]
        0x48, 0xFF, 0xC0, // INC RAX (label: start) [5-7]
        0x48, 0x83, 0xF8, 0x10, // CMP RAX, 16 [8-11]
        0x74, 0x09, // JZ +9 (14+9=23, to HLT) [12-13]
        0xE2, 0xF5, // LOOP -11 (16-11=5, back to INC) [14-15]
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3 (reload) [16-20]
        0xEB, 0xEE, // JMP -18 (23-18=5, back to INC) [21-22]
        0xF4, // HLT (label: end) [23]
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 16, "Should reach 16");
}

#[test]
fn test_jrcxz_multiple_checks() {
    // Multiple JRCXZ checks
    // Positions: 0-1 JRCXZ, 2-4 INC, 5-7 DEC, 8-9 JMP, 10 HLT
    let code = &[
        0xE3, 0x08, // JRCXZ +8 (2+8=10, to HLT if RCX=0) [0-1]
        0x48, 0xFF, 0xC0, // INC RAX [2-4]
        0x48, 0xFF, 0xC9, // DEC RCX [5-7]
        0xEB, 0xF6, // JMP -10 (10-10=0, back to JRCXZ) [8-9]
        0xF4, // HLT [10]
    ];
    let mut cpu = create_test_cpu(code);
    cpu.set_rcx(5);
    cpu.set_rax(0);

    run_test(&mut cpu);

    assert_eq!(cpu.get_rax(), 5, "Should increment 5 times");
    assert_eq!(cpu.get_rcx(), 0, "RCX should be 0");
}
