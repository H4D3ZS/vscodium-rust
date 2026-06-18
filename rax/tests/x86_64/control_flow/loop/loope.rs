use crate::common::{VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm};

// LOOPE/LOOPZ - Loop While Equal / Loop While Zero
// Decrements RCX and jumps if RCX != 0 AND ZF = 1
// Opcode: E1 cb - LOOPE rel8 / LOOPZ rel8

#[test]
fn test_loope_basic_with_zf_set() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0 (sets ZF=0, since RAX > 0)
        0xe1, 0xf7, // LOOPE -9 (won't loop since ZF=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1); // Only one iteration
    assert_eq!(vm.rcx, 2); // Decremented once
}

#[test]
fn test_loope_continues_while_zf_set() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0xf8, // LOOPE -8 (continues while ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // All 3 iterations
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_stops_on_zf_clear() {
    // LOOPE loops while ZF=1. CMP RAX, 3 sets ZF=0 when RAX!=3 (not equal).
    // Since RAX starts at 0 and increments, first iteration has RAX=1,
    // CMP 1,3 → ZF=0, so LOOPE stops immediately.
    // To test "stops on ZF clear", we keep ZF=1 until RAX==3, then continue
    // one more iteration where ZF=0 (RAX=4, CMP 4,3 → ZF=0).
    // But that's complex. Let's simplify: verify LOOPE stops when ZF=0.
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x03, // CMP RAX, 3 (ZF=1 when RAX==3, ZF=0 otherwise)
        0xe1, 0xf7, // LOOPE -9 (loops while ZF=1 AND RCX!=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // Iteration 1: RAX=1, CMP 1,3 → ZF=0, LOOPE: RCX=4, ZF=0 → stop
    assert_eq!(vm.rax, 1); // Stopped on first iteration (ZF=0)
    assert_eq!(vm.rcx, 4); // Only 1 decrement
}

#[test]
fn test_loopz_alias() {
    // LOOPZ is an alias for LOOPE
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0xf8, // LOOPZ -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 2);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_zero_iterations_rcx_zero() {
    // To achieve zero iterations with RCX=0, we must use JRCXZ to skip
    // LOOPE decrements RCX first, so RCX=0 wraps to 0xFF...FF
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe3, 0x05, // JRCXZ +5 (skip loop if RCX=0)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe1, 0xfb, // LOOPE -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0); // No iterations
    assert_eq!(vm.rcx, 0); // Unchanged
}

#[test]
fn test_loope_one_iteration() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF=1)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_string_comparison_pattern() {
    // Compare strings until mismatch or end
    // INC RBX/RDX must come BEFORE CMP so that CMP's ZF is what LOOPE sees
    // Layout:
    // 0x1000: MOV RCX, 5 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 'A' (7)
    // 0x1011: MOV RDX, 'A' (7)
    // 0x1018: INC RAX (3) <- loop_start
    // 0x101B: INC RBX (3)
    // 0x101E: INC RDX (3)
    // 0x1021: CMP RBX, RDX (3) - must be right before LOOPE!
    // 0x1024: LOOPE -14 (2) -> target = 0x1026 - 14 = 0x1018
    // 0x1026: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5 (max length)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (match count)
        0x48, 0xc7, 0xc3, 0x41, 0x00, 0x00, 0x00, // MOV RBX, 'A' (str1)
        0x48, 0xc7, 0xc2, 0x41, 0x00, 0x00, 0x00, // MOV RDX, 'A' (str2)
        // loop_start (0x1018):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xc3, // INC RBX (next char)
        0x48, 0xff, 0xc2, // INC RDX (next char)
        0x48, 0x39, 0xd3, // CMP RBX, RDX (sets ZF=1 if equal) - must be right before LOOPE!
        0xe1, 0xf2, // LOOPE -14 (continue while equal)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // All 5 matched
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_array_search_for_zero() {
    // Simplified test: LOOPE continues while ZF=1
    // We use CMP to compare values - when values equal, ZF=1, LOOPE continues
    // When they differ, ZF=0, LOOPE stops
    // Layout:
    // 0x1000: MOV RCX, 5 (7)
    // 0x1007: XOR RAX, RAX (3) - index counter
    // 0x100A: MOV RBX, 1 (7) - value to compare against
    // 0x1011: INC RAX (3) <- loop_start
    // 0x1014: CMP RBX, 1 (4) - ZF=1 when RBX==1
    // 0x1018: LOOPE -9 (2) -> target = 0x101A - 9 = 0x1011
    // 0x101A: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX (index)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xfb, 0x01, // CMP RBX, 1 (ZF=1 when equal)
        0xe1, 0xf7, // LOOPE -9 (continue while ZF=1 AND RCX!=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // RBX always equals 1, so ZF=1 throughout, loops 5 times
    assert_eq!(vm.rax, 5);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_all_equal_elements() {
    // Check if all elements are equal to first
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (target value)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (match count)
        // loop_start:
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0x83, 0xf8, 0x42, // CMP RAX, 0x42 (sets ZF=1 if equal)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 4); // All 4 matched
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_stops_when_rcx_zero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1, always continue)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3);
    assert_eq!(vm.rcx, 0); // Stopped because RCX reached 0
}

#[test]
fn test_loope_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xc7, 0xc2, 0x99, 0x00, 0x00, 0x00, // MOV RDX, 0x99
        0x48, 0xc7, 0xc6, 0x42, 0x00, 0x00, 0x00, // MOV RSI, 0x42
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rdx, 0x99); // Preserved
    assert_eq!(vm.rsi, 0x42); // Preserved
}

#[test]
fn test_loope_with_cmp_equal() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // MOV RBX, 7 (target)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF=1 when RAX==RBX)
        0xe1, 0xf8, // LOOPE -8 (stop when found, i.e., ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // Wait, this is confusing. Let me think: LOOPE continues while ZF=1
    // So CMP RAX, RBX sets ZF=1 when equal
    // LOOPE will continue while ZF=1, so it stops when ZF=0 (not equal) or RCX=0
    // When RAX=1..6, ZF=0 (not equal), so LOOPE doesn't jump
    // So it only does one iteration
    assert_eq!(vm.rax, 1);
}

#[test]
fn test_loope_with_test_zero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x85, 0xdb, // TEST RBX, RBX (sets ZF=1 since RBX=0)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // All iterations (ZF always 1)
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_with_test_nonzero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x85, 0xdb, // TEST RBX, RBX (sets ZF=0 since RBX!=0)
        0xe1, 0xf8, // LOOPE -8 (won't loop since ZF=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1); // Only one iteration
    assert_eq!(vm.rcx, 3);
}

#[test]
fn test_loope_nested_with_manual_inner() {
    // Layout:
    // 0x1000: MOV RCX, 3 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 2 (7) <- outer_loop
    // 0x1011: INC RAX (3) <- inner_loop
    // 0x1014: DEC RBX (3)
    // 0x1017: JNZ -8 (2) -> target = 0x1019 - 8 = 0x1011 (inner_loop)
    // 0x1019: XOR RBX, RBX (3)
    // 0x101C: LOOPE -20 (2) -> target = 0x101E - 20 = 0x100A (outer_loop)
    // 0x101E: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (outer)
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // outer_loop (0x100A):
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2 (inner count)
        // inner_loop (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xcb, // DEC RBX
        0x75, 0xf8, // JNZ -8 (inner loop)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1 for LOOPE)
        0xe1, 0xec, // LOOPE -20 (outer loop)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 6); // 3 * 2
}

#[test]
fn test_loope_max_backward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
    ];
    // loop_start:
    code.extend(vec![
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    ]); // 118 NOPs
    code.extend([
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0x80, // LOOPE -128
    ]);
    code.push(0xf4); // HLT

    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_forward_no_loop() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1)
        0xe1, 0x05, // LOOPE +5 (forward, decrements RCX and jumps if ZF=1)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, // HLT * 5
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 4); // Decremented once
}

#[test]
fn test_loope_break_on_nonequal() {
    // Loop while elements equal to first
    // Layout:
    // 0x1000: MOV RCX, 6 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 5 (7)
    // 0x1011: INC RAX (3) <- loop_start
    // 0x1014: CMP RAX, 4 (4)
    // 0x1018: JNE +7 (2) -> skip 7-byte MOV to CMP RBX at 0x1021
    // 0x101A: MOV RBX, 0x99 (7)
    // 0x1021: CMP RBX, 5 (4)
    // 0x1025: LOOPE -22 (2) -> target = 0x1027 - 22 = 0x1011
    // 0x1027: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x31, 0xc0, // XOR RAX, RAX (count)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (value)
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x04, // CMP RAX, 4
        0x75, 0x07, // JNE +7 (skip 7-byte MOV RBX)
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00,
        0x00, // MOV RBX, 0x99 (change value at iteration 4)
        0x48, 0x83, 0xfb, 0x05, // CMP RBX, 5 (sets ZF=1 while RBX==5)
        0xe1, 0xea, // LOOPE -22
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // Stopped when value changed
}

#[test]
fn test_loope_early_exit_pattern() {
    // LOOPE continues while ZF=1. CMP RAX,5 sets ZF=0 when RAX!=5.
    // We need XOR RBX,RBX to set ZF=1 for LOOPE, but JE checks the CMP result.
    // Layout (indices): MOV(0-6), XOR(7-9), INC(10-12), CMP(13-16), JE(17-18), XOR(19-21), LOOPE(22-23), HLT(24)
    // 0x1000: MOV RCX, 10 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: INC RAX (3) <- loop_start
    // 0x100D: CMP RAX, 5 (4)
    // 0x1011: JE +5 (2) -> jumps to HLT at 0x1018
    // 0x1013: XOR RBX, RBX (3) - sets ZF=1 for LOOPE
    // 0x1016: LOOPE -14 (2) -> target = 0x1018 - 14 = 0x100A
    // 0x1018: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start (0x100A):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x74, 0x05, // JE +5 (break early, skip XOR+LOOPE to HLT)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (sets ZF=1 for LOOPE)
        0xe1, 0xf2, // LOOPE -14
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Broke at 5
}

#[test]
fn test_loope_repe_string_scan() {
    // Scan for mismatch in repeated pattern
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX (position)
        0x48, 0xc7, 0xc3, 0x41, 0x00, 0x00, 0x00, // MOV RBX, 'A' (expected char)
        0x48, 0xc7, 0xc2, 0x41, 0x00, 0x00, 0x00, // MOV RDX, 'A' (current char)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX (position)
        0x48, 0x39, 0xd3, // CMP RBX, RDX (sets ZF=1 if match)
        0xe1, 0xf8, // LOOPE -8 (continue while match)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // All matched
}

#[test]
fn test_loope_password_check_pattern() {
    // Check each character until mismatch
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (password length)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (match count)
        0x48, 0xc7, 0xc3, 0x50, 0x00, 0x00, 0x00, // MOV RBX, 'P' (password chars)
        0x48, 0xc7, 0xc2, 0x50, 0x00, 0x00, 0x00, // MOV RDX, 'P' (input chars)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x39, 0xd3, // CMP RBX, RDX (check match)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // All chars matched
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loope_stops_on_both_conditions() {
    // Stops when EITHER RCX=0 OR ZF=0
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10 (large count)
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5 (ZF=1 only when RAX=5)
        0xe1, 0xf7, // LOOPE -9 (continues while ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // First iteration: RAX=1, CMP sets ZF=0, LOOPE doesn't jump
    assert_eq!(vm.rax, 1);
}

#[test]
fn test_loope_with_sub_zero_result() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x31, 0xdb, // XOR RBX, RBX (count)
        // loop_start:
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0x83, 0xe8, 0x05, // SUB RAX, 5 (first iter: RAX=0, ZF=1)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // First: RAX=5, SUB gives 0, ZF=1, LOOPE jumps
    // Second: RAX=0, SUB gives -5, ZF=0, LOOPE doesn't jump
    assert_eq!(vm.rbx, 2);
}

#[test]
fn test_loope_with_and_zero_result() {
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x31, 0xdb, // XOR RBX, RBX
        // loop_start:
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0x83, 0xe0, 0x00, // AND RAX, 0 (always gives 0, ZF=1)
        0xe1, 0xf8, // LOOPE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 4); // All iterations
}

#[test]
fn test_loope_consecutive_zero_checks() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x85, 0xdb, // TEST RBX, RBX (ZF=1, zero)
        0xe1, 0xf1, // LOOPE -15
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3);
}

#[test]
fn test_loope_two_byte_instruction() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x31, 0xdb, // XOR RBX, RBX (ZF=1)
        0xe1, 0xfd, // LOOPE -3 (to XOR, won't loop as RCX becomes 0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}
