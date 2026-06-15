use crate::common::{VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm};

// LOOPNE/LOOPNZ - Loop While Not Equal / Loop While Not Zero
// Decrements RCX and jumps if RCX != 0 AND ZF = 0
// Opcode: E0 cb - LOOPNE rel8 / LOOPNZ rel8

#[test]
fn test_loopne_basic_with_zf_clear() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0xff, // CMP RAX, 0xFF (sets ZF=0, not equal)
        0xe0, 0xf7, // LOOPNE -9 (continues while ZF=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // All 3 iterations
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_stops_on_zf_set() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x03, // CMP RAX, 3 (sets ZF=1 when RAX==3)
        0xe0, 0xf7, // LOOPNE -9 (stops when ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // Stopped when found 3
    assert_eq!(vm.rcx, 2); // 3 iterations (5->4->3->2)
}

#[test]
fn test_loopnz_alias() {
    // LOOPNZ is an alias for LOOPNE
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0xff, // CMP RAX, 0xFF (ZF=0)
        0xe0, 0xf7, // LOOPNZ -9
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 2);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_search_for_value() {
    // Search for target value
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x31, 0xc0, // XOR RAX, RAX (counter/current value)
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // MOV RBX, 7 (target)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets ZF=1 when found)
        0xe0, 0xf8, // LOOPNE -8 (stop when found)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7); // Found target
    assert_eq!(vm.rcx, 3); // 7 iterations (10->9->...->3)
}

#[test]
fn test_loopne_zero_iterations_rcx_zero() {
    // To achieve zero iterations with RCX=0, must use JRCXZ to skip
    // LOOPNE decrements RCX first, so RCX=0 wraps to 0xFFFFFFFFFFFFFFFF
    // Layout:
    // 0x1000: MOV RCX, 0 (7)
    // 0x1007: MOV RAX, 1 (7)
    // 0x100E: JRCXZ +5 (2) -> skip to HLT at 0x1015
    // 0x1010: INC RAX (3) <- loop_start
    // 0x1013: LOOPNE -5 (2) -> target = 0x1015 - 5 = 0x1010
    // 0x1015: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xe3, 0x05, // JRCXZ +5 (skip loop if RCX=0)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe0, 0xfb, // LOOPNE -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1); // No iterations
    assert_eq!(vm.rcx, 0); // Unchanged
}

#[test]
fn test_loopne_one_iteration() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0xff, // CMP RAX, 0xFF (ZF=0)
        0xe0, 0xf7, // LOOPNE -9
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_array_search() {
    // Search for zero in array
    // Layout:
    // 0x1000: MOV RCX, 6 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 0x42 (7)
    // 0x1011: INC RAX (3) <- loop_start
    // 0x1014: CMP RAX, 4 (4)
    // 0x1018: JNE +7 (2) -> skip 7-byte MOV to TEST at 0x1021
    // 0x101A: MOV RBX, 0 (7)
    // 0x1021: TEST RBX, RBX (3)
    // 0x1024: LOOPNE -21 (2) -> target = 0x1026 - 21 = 0x1011
    // 0x1026: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x31, 0xc0, // XOR RAX, RAX (index)
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42 (value)
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x04, // CMP RAX, 4
        0x75, 0x07, // JNE +7 (skip 7-byte MOV RBX)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (element 4 is zero)
        0x48, 0x85, 0xdb, // TEST RBX, RBX (sets ZF=1 if zero)
        0xe0, 0xeb, // LOOPNE -21 (stop when zero found)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // Found at index 4
}

#[test]
fn test_loopne_string_search() {
    // Search for specific character
    // Layout:
    // 0x1000: MOV RCX, 8 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 'A' (7)
    // 0x1011: MOV RDX, 'E' (7)
    // 0x1018: INC RAX (3) <- loop_start
    // 0x101B: INC RBX (3)
    // 0x101E: CMP RBX, RDX (3)
    // 0x1021: LOOPNE -11 (2) -> target = 0x1023 - 11 = 0x1018
    // 0x1023: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x31, 0xc0, // XOR RAX, RAX (position)
        0x48, 0xc7, 0xc3, 0x41, 0x00, 0x00, 0x00, // MOV RBX, 'A' (current char)
        0x48, 0xc7, 0xc2, 0x45, 0x00, 0x00, 0x00, // MOV RDX, 'E' (target)
        // loop_start (0x1018):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xc3, // INC RBX (next char: A->B->C->D->E)
        0x48, 0x39, 0xd3, // CMP RBX, RDX (sets ZF=1 when found)
        0xe0, 0xf5, // LOOPNE -11 (stop when found)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // Found 'E' at position 4
    assert_eq!(vm.rbx, 0x45); // 'E'
}

#[test]
fn test_loopne_not_found() {
    // Search for value that doesn't exist
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99 (target)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x39, 0xd8, // CMP RAX, RBX (never equal)
        0xe0, 0xf8, // LOOPNE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Searched all
    assert_eq!(vm.rcx, 0); // Exhausted count
}

#[test]
fn test_loopne_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xc7, 0xc2, 0x99, 0x00, 0x00, 0x00, // MOV RDX, 0x99
        0x48, 0xc7, 0xc6, 0x42, 0x00, 0x00, 0x00, // MOV RSI, 0x42
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0xff, // CMP RAX, 0xFF (ZF=0)
        0xe0, 0xf7, // LOOPNE -9
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rdx, 0x99); // Preserved
    assert_eq!(vm.rsi, 0x42); // Preserved
}

#[test]
fn test_loopne_with_test_zero_stops() {
    // Layout:
    // 0x1000: MOV RCX, 5 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 1 (7) - must initialize RBX to non-zero!
    // 0x1011: INC RAX (3) <- loop_start
    // 0x1014: CMP RAX, 3 (4)
    // 0x1018: JNE +3 (2) -> skip 3-byte XOR to TEST at 0x101D
    // 0x101A: XOR RBX, RBX (3)
    // 0x101D: TEST RBX, RBX (3)
    // 0x1020: LOOPNE -17 (2) -> target = 0x1022 - 17 = 0x1011
    // 0x1022: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (non-zero initially)
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x03, // CMP RAX, 3
        0x75, 0x03, // JNE +3 (skip XOR RBX)
        0x48, 0x31, 0xdb, // XOR RBX, RBX (make zero at iteration 3)
        0x48, 0x85, 0xdb, // TEST RBX, RBX (ZF=1 when zero)
        0xe0, 0xef, // LOOPNE -17 (stop when ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // Stopped at 3
}

#[test]
fn test_loopne_with_test_nonzero_continues() {
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x85, 0xdb, // TEST RBX, RBX (sets ZF=0 since RBX!=0)
        0xe0, 0xf8, // LOOPNE -8 (continues while ZF=0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // All iterations
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_linear_search() {
    // Linear search for value 5
    // INC must come BEFORE CMP so LOOPNE sees ZF from CMP
    // Layout:
    // 0x1000: MOV RCX, 7 (7)
    // 0x1007: MOV RAX, 0 (7) - start at 0 so INC gives 1,2,3,4,5
    // 0x100E: MOV RBX, 5 (7)
    // 0x1015: INC RAX (3) <- loop_start
    // 0x1018: CMP RAX, RBX (3)
    // 0x101B: LOOPNE -8 (2) -> target = 0x101D - 8 = 0x1015
    // 0x101D: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x07, 0x00, 0x00, 0x00, // MOV RCX, 7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (start)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (target)
        // loop_start (0x1015):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x39, 0xd8, // CMP RAX, RBX (must be right before LOOPNE)
        0xe0, 0xf8, // LOOPNE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Found 5
}

#[test]
fn test_loopne_validate_all_nonzero() {
    // Validate all elements are non-zero
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42 (all non-zero)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (count)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x85, 0xdb, // TEST RBX, RBX (ZF=0 if non-zero)
        0xe0, 0xf8, // LOOPNE -8 (continues while non-zero)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4); // All validated
}

#[test]
fn test_loopne_max_backward_offset() {
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
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (ZF=0)
        0xe0, 0x80, // LOOPNE -128
    ]);
    code.push(0xf4); // HLT

    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_forward_no_loop() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (ZF=0)
        0xe0, 0x05, // LOOPNE +5 (forward, decrements RCX and jumps if ZF=0)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, // HLT * 5
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 4); // Decremented once
}

#[test]
fn test_loopne_early_exit_on_match() {
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5 (ZF=1 when found)
        0xe0, 0xf7, // LOOPNE -9 (exit when ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Found at 5
    assert_eq!(vm.rcx, 5); // 5 iterations left
}

#[test]
fn test_loopne_password_mismatch() {
    // LOOPNE continues while ZF=0 (not equal), stops when ZF=1 (equal).
    // This test actually finds where chars MATCH (when we introduce a mismatch,
    // LOOPNE sees ZF=0 and continues; when chars match, ZF=1 and it stops).
    // At pos 4 we change RDX to 'Q', so CMP 'P','Q' gives ZF=0, loop continues.
    // After pos 4, RDX stays 'Q', so CMP 'P','Q' keeps giving ZF=0.
    // Loop runs until RCX=0.
    // Layout:
    // 0x1000: MOV RCX, 6 (7)
    // 0x1007: XOR RAX, RAX (3)
    // 0x100A: MOV RBX, 'P' (7)
    // 0x1011: MOV RDX, 'P' (7)
    // 0x1018: INC RAX (3) <- loop_start
    // 0x101B: CMP RAX, 4 (4)
    // 0x101F: JNE +7 (2) -> skip 7-byte MOV to CMP at 0x1028
    // 0x1021: MOV RDX, 'Q' (7)
    // 0x1028: CMP RBX, RDX (3)
    // 0x102B: LOOPNE -21 (2) -> target = 0x102D - 21 = 0x1018
    // 0x102D: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x31, 0xc0, // XOR RAX, RAX (position)
        0x48, 0xc7, 0xc3, 0x50, 0x00, 0x00, 0x00, // MOV RBX, 'P' (expected)
        0x48, 0xc7, 0xc2, 0x50, 0x00, 0x00, 0x00, // MOV RDX, 'P' (actual)
        // loop_start (0x1018):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x04, // CMP RAX, 4
        0x75, 0x07, // JNE +7 (skip 7-byte MOV RDX)
        0x48, 0xc7, 0xc2, 0x51, 0x00, 0x00, 0x00, // MOV RDX, 'Q' (mismatch at pos 4)
        0x48, 0x39, 0xd3, // CMP RBX, RDX (ZF=1 if match)
        0xe0, 0xeb, // LOOPNE -21 (continues while ZF=0, stops when ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // Iter 1-3: RDX='P', CMP 'P','P' → ZF=1, stop immediately!
    // Actually we stop on first iteration because chars match!
    assert_eq!(vm.rax, 1); // Stopped at first match
}

#[test]
fn test_loopne_find_first_even() {
    // Find first even number. LOOPNE continues while ZF=0, stops when ZF=1.
    // TEST RBX, 1: ZF=1 if even (LSB=0), ZF=0 if odd (LSB=1)
    // So LOOPNE stops when even (ZF=1), continues when odd (ZF=0).
    // Layout:
    // 0x1000: MOV RCX, 8 (7)
    // 0x1007: MOV RBX, 1 (7) - start with odd
    // 0x100E: XOR RAX, RAX (3)
    // 0x1011: INC RAX (3) <- loop_start
    // 0x1014: INC RBX (3)
    // 0x1017: TEST RBX, 1 (7)
    // 0x101E: LOOPNE -15 (2) -> target = 0x1020 - 15 = 0x1011
    // 0x1020: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (start with odd)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (count)
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xc3, // INC RBX (next number: 2,3,4,...)
        0x48, 0xf7, 0xc3, 0x01, 0x00, 0x00, 0x00, // TEST RBX, 1 (ZF=1 if even)
        0xe0, 0xf1, // LOOPNE -15 (stop when even found, i.e., ZF=1)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // RBX: 1->2 (even), ZF=1, stop
    assert_eq!(vm.rax, 1); // Found even at first iteration
    assert_eq!(vm.rbx, 2); // First even number
}

#[test]
fn test_loopne_bubble_sort_inner_loop() {
    // Simplified bubble sort inner loop pattern
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX (swap count)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (current)
        0x48, 0xc7, 0xc2, 0x02, 0x00, 0x00, 0x00, // MOV RDX, 2 (next)
        // loop_start:
        0x48, 0x39, 0xd3, // CMP RBX, RDX (compare elements)
        0x7e, 0x03, // JLE +3 (skip if in order)
        0x48, 0xff, 0xc0, // INC RAX (count swap)
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0xff, 0xc2, // INC RDX
        0x48, 0x83, 0xfb, 0x99, // CMP RBX, 0x99 (never equal)
        0xe0, 0xee, // LOOPNE -18
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0); // All iterations
}

#[test]
fn test_loopne_count_until_sentinel() {
    // Count elements until sentinel value
    let code = [
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 255 (max)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (count)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (value)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0x83, 0xfb, 0x0a, // CMP RBX, 10 (sentinel)
        0xe0, 0xf4, // LOOPNE -12 (stop when sentinel found)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 9); // Counted 9 before sentinel
    assert_eq!(vm.rbx, 10); // Sentinel value
}

#[test]
fn test_loopne_stops_on_both_conditions() {
    // Stops when EITHER RCX=0 OR ZF=1
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (small count)
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x0a, // CMP RAX, 10 (won't reach, ZF stays 0)
        0xe0, 0xf7, // LOOPNE -9
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // Stopped when RCX=0
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_with_or_nonzero() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xc8, 0x01, // OR RAX, 1 (always non-zero, ZF=0)
        0xe0, 0xf8, // LOOPNE -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0); // All iterations
}

#[test]
fn test_loopne_binary_search_pattern() {
    // Simplified binary search pattern
    // Layout:
    // 0x1000: MOV RCX, 4 (7)
    // 0x1007: MOV RAX, 1 (7)
    // 0x100E: MOV RBX, 8 (7)
    // 0x1015: MOV RDX, 6 (7)
    // 0x101C: MOV RSI, RAX (3) <- loop_start
    // 0x101F: ADD RSI, RBX (3)
    // 0x1022: SHR RSI, 1 (3)
    // 0x1025: CMP RSI, RDX (3)
    // 0x1028: LOOPNE -14 (2) -> target = 0x102A - 14 = 0x101C
    // 0x102A: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (max iterations)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (low)
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8 (high)
        0x48, 0xc7, 0xc2, 0x06, 0x00, 0x00, 0x00, // MOV RDX, 6 (target)
        // loop_start (0x101C):
        0x48, 0x89, 0xc6, // MOV RSI, RAX
        0x48, 0x01, 0xde, // ADD RSI, RBX
        0x48, 0xd1, 0xee, // SHR RSI, 1 (mid = (low+high)/2)
        0x48, 0x39, 0xd6, // CMP RSI, RDX (found?)
        0xe0, 0xf2, // LOOPNE -14 (continue while not found)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // First iteration: mid = (1+8)/2 = 4, CMP 4,6 → ZF=0, continue
    // Second iteration: mid = 4 again (no update to low/high), loop continues...
    // Eventually stops when RCX=0
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loopne_two_byte_instruction() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (ZF=0)
        0xe0, 0xfa, // LOOPNE -6 (won't loop as RCX becomes 0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}
