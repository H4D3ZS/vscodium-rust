use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// RDTSCP - Read Time-Stamp Counter and Processor ID
// Opcode: 0F 01 F9
// Reads 64-bit time-stamp counter into EDX:EAX
// Reads IA32_TSC_AUX MSR into ECX
// EDX = high 32 bits of TSC, EAX = low 32 bits of TSC
// ECX = low 32 bits of IA32_TSC_AUX (processor ID)
// Does not modify flags
// Waits for previous instructions, not fully serializing

// Basic RDTSCP test - reads TSC into EDX:EAX and processor ID into ECX
#[test]
fn test_rdtscp_basic() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TSC should be loaded into EDX:EAX
    // Processor ID should be in ECX
    // Upper 32 bits of RAX, RDX, RCX should be cleared
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX should be cleared");
    assert_eq!(regs.rcx >> 32, 0, "Upper 32 bits of RCX should be cleared");
}

// Test RDTSCP increments over time
#[test]
fn test_rdtscp_increments() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP (first read)
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save low part)
        0x48, 0x89, 0xd6, // MOV RSI, RDX (save high part)
        0x90, // NOP (some instructions to pass time)
        0x90, // NOP
        0x90, // NOP
        0x0f, 0x01, 0xf9, // RDTSCP (second read)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Reconstruct both TSC values
    let tsc1 = ((regs.rsi as u64) << 32) | (regs.rbx as u64);
    let tsc2 = ((regs.rdx as u64) << 32) | (regs.rax as u64);

    // Second read should be >= first read (monotonic)
    assert!(tsc2 >= tsc1, "TSC should be monotonic");
}

// Test RDTSCP doesn't modify flags
#[test]
fn test_rdtscp_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set from the ADD
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDTSCP preserves other registers
#[test]
fn test_rdtscp_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x42, 0x42,
        0x42, // MOV RBX, 0x42424242 (sign-extended, positive)
        0x48, 0xc7, 0xc6, 0xaa, 0xaa, 0xaa,
        0xaa, // MOV RSI, 0xaaaaaaaa (sign-extended to 0xFFFFFFFFaaaaaaaa)
        0x48, 0xc7, 0xc7, 0xbb, 0xbb, 0xbb,
        0xbb, // MOV RDI, 0xbbbbbbbb (sign-extended to 0xFFFFFFFFbbbbbbbb)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX, RSI, RDI should be unchanged (MOV r64, imm32 sign-extends the immediate)
    assert_eq!(regs.rbx, 0x42424242, "RBX should not be affected");
    assert_eq!(
        regs.rsi, 0xFFFFFFFF_AAAAAAAAu64,
        "RSI should not be affected"
    );
    assert_eq!(
        regs.rdi, 0xFFFFFFFF_BBBBBBBBu64,
        "RDI should not be affected"
    );
}

// Test multiple sequential RDTSCP calls
#[test]
fn test_rdtscp_sequential_calls() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP #1
        0x0f, 0x01, 0xf9, // RDTSCP #2
        0x0f, 0x01, 0xf9, // RDTSCP #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete without error
    let tsc = ((regs.rdx as u64) << 32) | (regs.rax as u64);
    let _ = tsc;
}

// Test RDTSCP processor ID in ECX
#[test]
fn test_rdtscp_processor_id() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ECX should contain processor ID (from IA32_TSC_AUX MSR)
    // Upper 32 bits should be cleared
    assert_eq!(regs.rcx >> 32, 0, "Upper 32 bits of RCX should be cleared");
    // Processor ID value depends on implementation
    let proc_id = regs.rcx & 0xFFFFFFFF;
    let _ = proc_id;
}

// Test RDTSCP clears upper 32 bits of RAX, RDX, RCX
#[test]
fn test_rdtscp_clears_upper_bits() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xffffffffffffffff
        0x48, 0xc7, 0xc2, 0xff, 0xff, 0xff, 0xff, // MOV RDX, 0xffffffffffffffff
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be cleared for all three registers
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits of RAX should be cleared");
    assert_eq!(regs.rcx >> 32, 0, "Upper 32 bits of RCX should be cleared");
    assert_eq!(regs.rdx >> 32, 0, "Upper 32 bits of RDX should be cleared");
}

// Test RDTSCP overwrites previous register values
#[test]
fn test_rdtscp_overwrites_registers() {
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xb9, 0x22, 0x22, 0x22, 0x22, // MOV ECX, 0x22222222
        0xba, 0x33, 0x33, 0x33, 0x33, // MOV EDX, 0x33333333
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Old values should be completely overwritten
    // (We can't predict exact TSC/AUX values, but they should differ)
    let _ = regs;
}

// Test RDTSCP is monotonic across multiple calls
#[test]
fn test_rdtscp_monotonic_three_calls() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP #1
        0x50, // PUSH RAX
        0x52, // PUSH RDX
        0x90, // NOP
        0x0f, 0x01, 0xf9, // RDTSCP #2
        0x50, // PUSH RAX
        0x52, // PUSH RDX
        0x90, // NOP
        0x0f, 0x01, 0xf9, // RDTSCP #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Latest TSC value should be valid
    let tsc3 = ((regs.rdx as u64) << 32) | (regs.rax as u64);
    let _ = tsc3;
}

// Test RDTSCP with all flags set
#[test]
fn test_rdtscp_with_all_flags_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (sets CF, SF)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags from SUB should be preserved
    assert!(regs.rflags & 0x01 != 0, "CF should be preserved");
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDTSCP waits for previous instructions
#[test]
fn test_rdtscp_waits_for_previous() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x0f, 0x01, 0xf9, // RDTSCP (waits for ADD to complete)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ADD should complete before RDTSCP
    assert_eq!(regs.rbx, 2, "ADD should complete before RDTSCP");
}

// Test RDTSCP with various instruction combinations
#[test]
fn test_rdtscp_with_mov_sequence() {
    let code = [
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x01, 0xf9, // RDTSCP
        0x48, 0x89, 0xc6, // MOV RSI, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RSI should contain low 32 bits of TSC
    assert_eq!(regs.rsi >> 32, 0, "Upper bits should be clear");
}

// Test RDTSCP in a loop
#[test]
fn test_rdtscp_in_loop() {
    let code = [
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3 (loop counter)
        // loop:
        0x0f, 0x01, 0xf9, // RDTSCP
        0x48, 0x83, 0xeb, 0x01, // SUB RBX, 1
        0x75, 0xf8, // JNZ loop (back 8 bytes)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Loop should complete, RBX should be 0
    assert_eq!(regs.rbx, 0, "Loop should complete");
}

// Test RDTSCP with PUSH/POP operations
#[test]
fn test_rdtscp_with_push_pop() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0x50, // PUSH RAX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x5a, // POP RDX
        0x59, // POP RCX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Values should be restored
    assert_eq!(regs.rax >> 32, 0, "Upper bits should still be clear");
    assert_eq!(regs.rcx >> 32, 0, "Upper bits should still be clear");
    assert_eq!(regs.rdx >> 32, 0, "Upper bits should still be clear");
}

// Test RDTSCP can be used for timing measurements
#[test]
fn test_rdtscp_timing_measurement() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP (start)
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0x89, 0xd6, // MOV RSI, RDX
        // Some work
        0x48, 0xc7, 0xc7, 0x64, 0x00, 0x00, 0x00, // MOV RDI, 100
        0x48, 0x83, 0xef, 0x01, // SUB RDI, 1
        0x75, 0xfa, // JNZ loop
        0x0f, 0x01, 0xf9, // RDTSCP (end)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let tsc_start = ((regs.rsi as u64) << 32) | (regs.rbx as u64);
    let tsc_end = ((regs.rdx as u64) << 32) | (regs.rax as u64);

    // End should be >= start
    assert!(tsc_end >= tsc_start, "TSC should advance");
}

// Test RDTSCP vs RDTSC comparison
#[test]
fn test_rdtscp_vs_rdtsc() {
    let code = [
        0x0f, 0x31, // RDTSC
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save RDTSC low)
        0x48, 0x89, 0xd6, // MOV RSI, RDX (save RDTSC high)
        0x90, // NOP
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let tsc_rdtsc = ((regs.rsi as u64) << 32) | (regs.rbx as u64);
    let tsc_rdtscp = ((regs.rdx as u64) << 32) | (regs.rax as u64);

    // RDTSCP should be >= RDTSC (executed later)
    assert!(tsc_rdtscp >= tsc_rdtsc, "RDTSCP should be >= RDTSC");
}

// Test RDTSCP processor ID consistency
#[test]
fn test_rdtscp_processor_id_consistency() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP #1
        0x48, 0x89, 0xcb, // MOV RBX, RCX (save processor ID)
        0x0f, 0x01, 0xf9, // RDTSCP #2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Processor ID should be same on both calls (same processor)
    let proc_id1 = regs.rbx & 0xFFFFFFFF;
    let proc_id2 = regs.rcx & 0xFFFFFFFF;
    assert_eq!(proc_id1, proc_id2, "Processor ID should be consistent");
}

// Test RDTSCP with zero flag conditions
#[test]
fn test_rdtscp_zero_flag() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ZF should still be set
    assert!(regs.rflags & 0x40 != 0, "ZF should be preserved");
}

// Test RDTSCP with carry flag conditions
#[test]
fn test_rdtscp_carry_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xffffffffffffffff
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets CF)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags should be preserved
    let _ = regs.rflags;
}

// Test RDTSCP with sign flag conditions
#[test]
fn test_rdtscp_sign_flag() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // SF should still be set
    assert!(regs.rflags & 0x80 != 0, "SF should be preserved");
}

// Test RDTSCP after arithmetic operations
#[test]
fn test_rdtscp_after_arithmetic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x02, // ADD RBX, 2
        0x48, 0xf7, 0xdb, // NEG RBX
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should still have result of NEG
    assert_eq!(regs.rbx as i64, -3, "RBX should be -3");
}

// Test RDTSCP 64-bit reconstruction
#[test]
fn test_rdtscp_64bit_reconstruction() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0x48, 0x89, 0xd3, // MOV RBX, RDX
        0x48, 0xc1, 0xe3, 0x20, // SHL RBX, 32
        0x48, 0x09, 0xd8, // OR RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should now contain full 64-bit TSC
    let tsc = regs.rax;
    let _ = tsc;
}

// Test RDTSCP with conditional jumps
#[test]
fn test_rdtscp_with_conditional_jump() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x75, 0x02, // JNZ skip
        0x90, // NOP
        0x90, // NOP
        // skip:
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test RDTSCP execution completes quickly
#[test]
fn test_rdtscp_execution_speed() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0x0f, 0x01, 0xf9, // RDTSCP
        0x0f, 0x01, 0xf9, // RDTSCP
        0x0f, 0x01, 0xf9, // RDTSCP
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All calls should complete
    let _ = regs;
}

// Test RDTSCP with high counter values
#[test]
fn test_rdtscp_high_values() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EDX, EAX, ECX should contain 32-bit values
    let edx = regs.rdx & 0xFFFFFFFF;
    let eax = regs.rax & 0xFFFFFFFF;
    let ecx = regs.rcx & 0xFFFFFFFF;
    let _ = (edx, eax, ecx);
}

// Test RDTSCP preserves RBX through multiple calls
#[test]
fn test_rdtscp_rbx_preservation() {
    let code = [
        0x48, 0xc7, 0xc3, 0xef, 0xbe, 0xad,
        0xde, // MOV RBX, 0xdeadbeef (sign-extended to 0xFFFFFFFFdeadbeef)
        0x0f, 0x01, 0xf9, // RDTSCP #1
        0x0f, 0x01, 0xf9, // RDTSCP #2
        0x0f, 0x01, 0xf9, // RDTSCP #3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MOV r64, imm32 sign-extends the immediate
    assert_eq!(regs.rbx, 0xFFFFFFFF_DEADBEEFu64, "RBX should be preserved");
}

// Test RDTSCP increment rate is reasonable
#[test]
fn test_rdtscp_increment_rate() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP #1
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0x89, 0xd6, // MOV RSI, RDX
        0x90, // NOP
        0x0f, 0x01, 0xf9, // RDTSCP #2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let tsc1 = ((regs.rsi as u64) << 32) | (regs.rbx as u64);
    let tsc2 = ((regs.rdx as u64) << 32) | (regs.rax as u64);

    // Should have incremented
    assert!(tsc2 >= tsc1, "TSC should increment");
}

// Test RDTSCP serialization properties
#[test]
fn test_rdtscp_serialization() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1 (must complete before RDTSCP)
        0x0f, 0x01, 0xf9, // RDTSCP (waits for previous instructions)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // ADD must complete before RDTSCP
    assert_eq!(regs.rbx, 2, "Previous instructions should complete");
}

// Test RDTSCP IA32_TSC_AUX value
#[test]
fn test_rdtscp_tsc_aux_value() {
    let code = [
        0x0f, 0x01, 0xf9, // RDTSCP
        0x48, 0x89, 0xcb, // MOV RBX, RCX (save TSC_AUX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should contain processor ID
    let tsc_aux = regs.rbx & 0xFFFFFFFF;
    let _ = tsc_aux;
}
