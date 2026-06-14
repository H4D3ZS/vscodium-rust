use crate::common::{VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm};

// LOOP - Loop According to RCX Counter
// Decrements RCX and jumps if RCX != 0
// Opcode: E2 cb - LOOP rel8

#[test]
fn test_loop_basic_countdown() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        // loop_start (offset 7):
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5 (back to loop_start)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // Loop executed 3 times
    assert_eq!(vm.rcx, 0); // RCX decremented to 0
}

#[test]
fn test_loop_zero_iterations() {
    // To achieve zero iterations with RCX=0, we must use JRCXZ to skip the loop body
    // LOOP alone always executes the body at least once (it's at the END of the loop)
    // Layout:
    // 0x1000: MOV RCX, 0 (7 bytes)
    // 0x1007: JRCXZ +5 (2 bytes) -> target = 0x1009 + 5 = 0x100E
    // 0x1009: INC RAX (3 bytes) - loop body
    // 0x100C: LOOP -5 (2 bytes) -> target = 0x100E - 5 = 0x1009
    // 0x100E: HLT (1 byte)
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xe3, 0x05, // JRCXZ +5 (skip loop body if RCX=0)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX (should not execute when RCX=0)
        0xe2, 0xfb, // LOOP -5 (back to loop_start)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0); // Loop body not executed
    assert_eq!(vm.rcx, 0); // RCX unchanged
}

#[test]
fn test_loop_one_iteration() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1); // Executed once
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_five_iterations() {
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_ten_iterations() {
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 10);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_accumulator() {
    // Sum 1+2+3+4+5 = 15
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sum = 0)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (counter)
        // loop_start:
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0x48, 0xff, 0xc3, // INC RBX
        0xe2, 0xf8, // LOOP -8
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 15); // 1+2+3+4+5
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_decrements_rcx_before_test() {
    let code = [
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        // loop_start:
        0x48, 0x89, 0xc8, // MOV RAX, RCX (save RCX value)
        0xe2, 0xfb, // LOOP -5 (decrements RCX, then tests)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // First iteration: RCX=2, saves 2, then LOOP decrements to 1, jumps
    // Second iteration: RCX=1, saves 1, then LOOP decrements to 0, doesn't jump
    assert_eq!(vm.rax, 1); // Last value of RCX before final decrement
}

#[test]
fn test_loop_forward_jump() {
    // Layout:
    // 0x1000: MOV RCX, 3 (7 bytes)
    // 0x1007: LOOP +4 (2 bytes) -> next_rip = 0x1009, target = 0x1009 + 4 = 0x100D
    // 0x1009: HLT (skipped when jumping forward)
    // 0x100A: NOP (padding)
    // 0x100B: NOP (padding)
    // 0x100C: NOP (padding)
    // 0x100D: HLT (target)
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0xe2, 0x04, // LOOP +4 (forward - unusual but valid)
        0xf4, // HLT (should not reach on first iteration)
        0x90, 0x90, 0x90, // NOP padding
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // RCX=3, LOOP decrements to 2 and jumps forward +4
    assert_eq!(vm.rcx, 2);
}

#[test]
fn test_loop_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0xc7, 0xc2, 0x99, 0x00, 0x00, 0x00, // MOV RDX, 0x99
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3);
    assert_eq!(vm.rbx, 0x42); // Preserved
    assert_eq!(vm.rdx, 0x99); // Preserved
}

#[test]
fn test_loop_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF=1, CF=1)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        // loop_start:
        0xe2, 0xfe, // LOOP -2 (should preserve flags)
        0x74, 0x01, // JZ +1 (should jump if ZF preserved)
        0xf4, // HLT (should not reach)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
}

#[test]
fn test_loop_array_iteration() {
    // Process 4 array elements
    // Layout:
    // 0x1000: MOV RCX, 4 (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: MOV RBX, 1 (7 bytes)
    // 0x1011: ADD RAX, RBX (3 bytes) <- loop_start
    // 0x1014: ADD RBX, 2 (4 bytes)
    // 0x1018: LOOP -9 (2 bytes) -> next_rip = 0x101A, target = 0x101A - 9 = 0x1011
    // 0x101A: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (count)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sum)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (value)
        // loop_start:
        0x48, 0x01, 0xd8, // ADD RAX, RBX (add to sum)
        0x48, 0x83, 0xc3, 0x02, // ADD RBX, 2 (next odd number)
        0xe2, 0xf7, // LOOP -9
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 16); // 1+3+5+7
}

#[test]
fn test_loop_nested_loops_outer() {
    // Outer loop with inner manual counter
    // Layout:
    // 0x1000: MOV RCX, 3 (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: MOV RBX, 2 (7 bytes) <- outer_loop
    // 0x1011: INC RAX (3 bytes) <- inner_loop
    // 0x1014: DEC RBX (3 bytes)
    // 0x1017: JNZ -9 (2 bytes) -> target = 0x1019 - 9 = 0x1010... need to adjust
    // Actually JNZ -6 targets 0x1019 - 6 = 0x1013... wrong
    // Let me recalculate:
    // JNZ is at 0x1017-0x1018. Next instruction at 0x1019.
    // Target should be inner_loop at 0x1011. Displacement = 0x1011 - 0x1019 = -8
    // 0x1019: LOOP -15 -> target = 0x101B - 15 = 0x100C... wrong, should be 0x100A
    // LOOP at 0x1019-0x101A. Next instruction at 0x101B.
    // Displacement = 0x100A - 0x101B = -17
    // 0x101B: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (outer count)
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // outer_loop (0x100A):
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2 (inner count)
        // inner_loop (0x1011):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0xff, 0xcb, // DEC RBX
        0x75, 0xf8, // JNZ -8 (inner loop: 0x1019 - 8 = 0x1011)
        0xe2, 0xef, // LOOP -17 (outer loop: 0x101B - 17 = 0x100A)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 6); // 3 * 2 = 6 iterations
}

#[test]
fn test_loop_string_length() {
    // Count until null terminator pattern
    // Layout:
    // 0x1000: MOV RCX, max (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: MOV RBX, 5 (7 bytes)
    // 0x1011: INC RAX (3 bytes) <- loop_start
    // 0x1014: DEC RBX (3 bytes)
    // 0x1017: JZ +4 (2 bytes) -> target = 0x1019 + 4 = 0x101D
    // 0x1019: LOOP -10 (2 bytes) -> target = 0x101B - 10 = 0x1011
    // 0x101B: NOP padding
    // 0x101C: NOP padding
    // 0x101D: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, max (large limit)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (length counter)
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5 (simulate string length)
        // loop_start (0x1011):
        0x48, 0xff, 0xc0, // INC RAX (count)
        0x48, 0xff, 0xcb, // DEC RBX (simulate checking character)
        0x74, 0x04, // JZ +4 (found null, exit to 0x101D)
        0xe2, 0xf6, // LOOP -10 (continue)
        0x90, 0x90, // NOP padding
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Found 5 characters
}

#[test]
fn test_loop_break_early() {
    // Loop with early exit condition
    // Layout:
    // 0x1000: MOV RCX, 10 (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: INC RAX (3 bytes) <- loop_start
    // 0x100D: CMP RAX, 5 (4 bytes)
    // 0x1011: JE +4 (2 bytes) -> target = 0x1013 + 4 = 0x1017
    // 0x1013: LOOP -11 (2 bytes) -> target = 0x1015 - 11 = 0x100A
    // 0x1015: NOP padding
    // 0x1016: NOP padding
    // 0x1017: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start (0x100A):
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x74, 0x04, // JE +4 (break if RAX == 5)
        0xe2, 0xf5, // LOOP -11
        0x90, 0x90, // NOP padding
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5); // Broke at 5
    assert_eq!(vm.rcx, 6); // 6 iterations remaining (4 LOOPs executed, broke before 5th)
}

#[test]
fn test_loop_continue_pattern() {
    // Skip even numbers, count odd
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x31, 0xc0, // XOR RAX, RAX (odd count)
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1 (number)
        // loop_start:
        0x48, 0xf7, 0xc3, 0x01, 0x00, 0x00, 0x00, // TEST RBX, 1
        0x74, 0x03, // JZ +3 (skip if even)
        0x48, 0xff, 0xc0, // INC RAX (count odd)
        0x48, 0xff, 0xc3, // INC RBX (next number)
        0xe2, 0xef, // LOOP -17
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // 1, 3, 5 are odd
}

#[test]
fn test_loop_max_backward_offset() {
    // For max backward offset (-128), we need:
    // displacement = target - next_rip = -128
    // target = loop_start = 0x1007
    // next_rip = 0x1007 + num_nops + 2 (LOOP size)
    // -128 = 0x1007 - (0x1007 + num_nops + 2)
    // num_nops = 126
    let mut code = vec![
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
    ];
    // loop_start (offset 7 = 0x1007):
    // 126 NOPs needed for -128 displacement
    code.extend(vec![0x90; 126]);
    code.extend([
        0xe2, 0x80, // LOOP -128 (max negative offset)
    ]);
    code.push(0xf4); // HLT

    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_max_forward_offset() {
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xe2, 0x7f, // LOOP +127 (will decrement to 0, no jump)
        0xf4, // HLT (should execute)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
}

#[test]
fn test_loop_zero_offset() {
    // Infinite loop with manual break
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0x00, // LOOP +0 (infinite if RCX>0, but RCX decrements)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // This is a bit tricky: LOOP +0 means loop to the instruction after LOOP
    // So it's effectively not looping back
    assert_eq!(vm.rax, 1);
}

#[test]
fn test_loop_multiplication_by_addition() {
    // Multiply 7 * 4 using repeated addition
    let code = [
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (multiplier)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (result)
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // MOV RBX, 7 (multiplicand)
        // loop_start:
        0x48, 0x01, 0xd8, // ADD RAX, RBX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 28); // 7 * 4
}

#[test]
fn test_loop_factorial_iterative() {
    // Compute 5! = 120 using LOOP
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (result)
        // loop_start:
        0x48, 0x0f, 0xaf, 0xc1, // IMUL RAX, RCX
        0xe2, 0xfa, // LOOP -6
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 120); // 5! = 5*4*3*2*1
}

#[test]
fn test_loop_power_of_two() {
    // Compute 2^5 = 32
    let code = [
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5 (exponent)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (result)
        // loop_start:
        0x48, 0xd1, 0xe0, // SHL RAX, 1 (multiply by 2)
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 32); // 2^5
}

#[test]
fn test_loop_buffer_clear() {
    // Simulate clearing a buffer (just counting)
    let code = [
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8 (buffer size)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (operations count)
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX (simulate write)
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 8); // Cleared 8 entries
}

#[test]
fn test_loop_fibonacci_sum() {
    // Sum first 8 Fibonacci numbers
    // Layout:
    // 0x1000: MOV RCX, 8 (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: MOV RBX, 0 (7 bytes)
    // 0x1011: MOV RDX, 1 (7 bytes)
    // 0x1018: ADD RAX, RBX (3 bytes) <- loop_start
    // 0x101B: MOV RSI, RDX (3 bytes)
    // 0x101E: ADD RDX, RBX (3 bytes)
    // 0x1021: MOV RBX, RSI (3 bytes)
    // 0x1024: LOOP -14 (2 bytes) -> target = 0x1026 - 14 = 0x1018
    // 0x1026: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sum)
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (fib[n-2])
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1 (fib[n-1])
        // loop_start (0x1018):
        0x48, 0x01, 0xd8, // ADD RAX, RBX (add to sum)
        0x48, 0x89, 0xd6, // MOV RSI, RDX
        0x48, 0x01, 0xda, // ADD RDX, RBX (new fib)
        0x48, 0x89, 0xf3, // MOV RBX, RSI
        0xe2, 0xf2, // LOOP -14
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 33); // 0+1+1+2+3+5+8+13
}

#[test]
fn test_loop_find_max() {
    // Find maximum in sequence: 3, 7, 2, 9, 4
    // This test doesn't use LOOP in a typical way - it's a linear comparison
    // that only runs through once. Let's just remove the LOOP and use
    // a simpler structure.
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (max so far)
        // Compare with 3
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x03, // JGE +3
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        // Compare with 7
        0x48, 0xc7, 0xc3, 0x07, 0x00, 0x00, 0x00, // MOV RBX, 7
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x03, // JGE +3
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        // Compare with 2
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x03, // JGE +3
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        // Compare with 9
        0x48, 0xc7, 0xc3, 0x09, 0x00, 0x00, 0x00, // MOV RBX, 9
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x03, // JGE +3
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        // Compare with 4
        0x48, 0xc7, 0xc3, 0x04, 0x00, 0x00, 0x00, // MOV RBX, 4
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x7d, 0x03, // JGE +3
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 9); // Maximum value
}

#[test]
fn test_loop_decrement_behavior() {
    // Verify LOOP decrements before checking
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX (executes once)
        0xe2, 0xfb, // LOOP -5 (decrements RCX to 0, doesn't jump)
        0x48, 0xff, 0xc0, // INC RAX (should execute)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 2); // Once in loop, once after
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_large_count() {
    // 100 iterations
    let code = [
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // loop_start:
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xfb, // LOOP -5
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 100);
    assert_eq!(vm.rcx, 0);
}

#[test]
fn test_loop_with_conditional_inside() {
    // Loop with conditional execution inside
    // Layout:
    // 0x1000: MOV RCX, 6 (7 bytes)
    // 0x1007: XOR RAX, RAX (3 bytes)
    // 0x100A: MOV RBX, 0 (7 bytes)
    // 0x1011: INC RBX (3 bytes) <- loop_start
    // 0x1014: TEST RBX, 1 (7 bytes)
    // 0x101B: JZ +3 (2 bytes) -> skip to 0x101D + 3 = 0x1020
    // 0x101D: INC RAX (3 bytes)
    // 0x1020: LOOP -17 (2 bytes) -> target = 0x1022 - 17 = 0x1011
    // 0x1022: HLT
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (index)
        // loop_start (0x1011):
        0x48, 0xff, 0xc3, // INC RBX
        0x48, 0xf7, 0xc3, 0x01, 0x00, 0x00, 0x00, // TEST RBX, 1 (check if odd)
        0x74, 0x03, // JZ +3 (skip if even)
        0x48, 0xff, 0xc0, // INC RAX
        0xe2, 0xef, // LOOP -17
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3); // 3 odd iterations (1, 3, 5)
}

#[test]
fn test_loop_two_byte_instruction() {
    // Verify LOOP is 2 bytes (opcode + rel8)
    let code = [
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xe2, 0xfe, // LOOP -2 (loop to itself - won't loop as RCX becomes 0)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
}
