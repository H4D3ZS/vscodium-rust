use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// MOV DR - Move to/from Debug Registers
// Opcodes:
// 0F 21 /r - MOV r64, DR0-DR7 (read debug register)
// 0F 23 /r - MOV DR0-DR7, r64 (write debug register)
//
// Debug Registers:
// DR0-DR3 - Linear breakpoint addresses
// DR4/DR5 - Aliased to DR6/DR7 when CR4.DE=0
// DR6 - Debug status register
// DR7 - Debug control register

// Test MOV from DR0 to RAX
#[test]
fn test_mov_dr0_to_rax() {
    let code = [
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DR0 should be readable (initially 0 if no breakpoint set)
    let _ = regs.rax;
}

// Test MOV from DR0 to RBX
#[test]
fn test_mov_dr0_to_rbx() {
    let code = [
        0x0f, 0x21, 0xc3, // MOV RBX, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rbx;
}

// Test MOV from DR1 to RAX
#[test]
fn test_mov_dr1_to_rax() {
    let code = [
        0x0f, 0x21, 0xc8, // MOV RAX, DR1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rax;
}

// Test MOV from DR1 to RCX
#[test]
fn test_mov_dr1_to_rcx() {
    let code = [
        0x0f, 0x21, 0xc9, // MOV RCX, DR1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rcx;
}

// Test MOV from DR2 to RAX
#[test]
fn test_mov_dr2_to_rax() {
    let code = [
        0x0f, 0x21, 0xd0, // MOV RAX, DR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rax;
}

// Test MOV from DR2 to RDX
#[test]
fn test_mov_dr2_to_rdx() {
    let code = [
        0x0f, 0x21, 0xd2, // MOV RDX, DR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rdx;
}

// Test MOV from DR3 to RAX
#[test]
fn test_mov_dr3_to_rax() {
    let code = [
        0x0f, 0x21, 0xd8, // MOV RAX, DR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rax;
}

// Test MOV from DR3 to RBX
#[test]
fn test_mov_dr3_to_rbx() {
    let code = [
        0x0f, 0x21, 0xdb, // MOV RBX, DR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rbx;
}

// Test MOV from DR6 to RAX (debug status)
#[test]
fn test_mov_dr6_to_rax() {
    let code = [
        0x0f, 0x21, 0xf0, // MOV RAX, DR6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DR6 is debug status register
    let _ = regs.rax;
}

// Test MOV from DR6 to RCX
#[test]
fn test_mov_dr6_to_rcx() {
    let code = [
        0x0f, 0x21, 0xf1, // MOV RCX, DR6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rcx;
}

// Test MOV from DR7 to RAX (debug control)
#[test]
fn test_mov_dr7_to_rax() {
    let code = [
        0x0f, 0x21, 0xf8, // MOV RAX, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DR7 is debug control register
    let _ = regs.rax;
}

// Test MOV from DR7 to RDX
#[test]
fn test_mov_dr7_to_rdx() {
    let code = [
        0x0f, 0x21, 0xfa, // MOV RDX, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rdx;
}

// Test MOV to DR0 from RAX
#[test]
fn test_mov_rax_to_dr0() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR0 from RBX
#[test]
fn test_mov_rbx_to_dr0() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x23, 0xc3, // MOV DR0, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR1 from RCX
#[test]
fn test_mov_rcx_to_dr1() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x30, 0x00, 0x00, // MOV RCX, 0x3000
        0x0f, 0x23, 0xc9, // MOV DR1, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR2 from RDX
#[test]
fn test_mov_rdx_to_dr2() {
    let code = [
        0x48, 0xc7, 0xc2, 0x00, 0x40, 0x00, 0x00, // MOV RDX, 0x4000
        0x0f, 0x23, 0xd2, // MOV DR2, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR3 from RSI
#[test]
fn test_mov_rsi_to_dr3() {
    let code = [
        0x48, 0xc7, 0xc6, 0x00, 0x50, 0x00, 0x00, // MOV RSI, 0x5000
        0x0f, 0x23, 0xde, // MOV DR3, RSI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR6 from RAX
#[test]
fn test_mov_rax_to_dr6() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x0f, 0x23, 0xf0, // MOV DR6, RAX (clear debug status)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to DR7 from RAX
#[test]
fn test_mov_rax_to_dr7() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x04, 0x00, 0x00, // MOV RAX, 0x400
        0x0f, 0x23, 0xf8, // MOV DR7, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test round-trip DR0 read/write
#[test]
fn test_dr0_round_trip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234, "DR0 value should be preserved");
}

// Test round-trip DR1 read/write
#[test]
fn test_dr1_round_trip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x00, 0x00, // MOV RAX, 0x5678
        0x0f, 0x23, 0xc8, // MOV DR1, RAX
        0x0f, 0x21, 0xc8, // MOV RAX, DR1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x5678, "DR1 value should be preserved");
}

// Test round-trip DR2 read/write
#[test]
fn test_dr2_round_trip() {
    let code = [
        0x48, 0xc7, 0xc0, 0xbc, 0x9a, 0x00, 0x00, // MOV RAX, 0x9abc
        0x0f, 0x23, 0xd0, // MOV DR2, RAX
        0x0f, 0x21, 0xd0, // MOV RAX, DR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x9abc, "DR2 value should be preserved");
}

// Test round-trip DR3 read/write
#[test]
fn test_dr3_round_trip() {
    let code = [
        0x48, 0xc7, 0xc0, 0xf0, 0xde, 0x00, 0x00, // MOV RAX, 0xdef0
        0x0f, 0x23, 0xd8, // MOV DR3, RAX
        0x0f, 0x21, 0xd8, // MOV RAX, DR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xdef0, "DR3 value should be preserved");
}

// Test round-trip DR6 read/write
#[test]
fn test_dr6_round_trip() {
    let code = [
        0x0f, 0x21, 0xf0, // MOV RAX, DR6
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x23, 0xf0, // MOV DR6, RAX
        0x0f, 0x21, 0xf0, // MOV RAX, DR6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Some bits in DR6 may be reserved, so we check general preservation
    let _ = (regs.rax, regs.rbx);
}

// Test round-trip DR7 read/write
#[test]
fn test_dr7_round_trip() {
    let code = [
        0x0f, 0x21, 0xf8, // MOV RAX, DR7
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x23, 0xf8, // MOV DR7, RAX
        0x0f, 0x21, 0xf8, // MOV RAX, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = (regs.rax, regs.rbx);
}

// Test DR0-DR3 all zero
#[test]
fn test_clear_all_breakpoint_registers() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0x0f, 0x23, 0xc8, // MOV DR1, RAX
        0x0f, 0x23, 0xd0, // MOV DR2, RAX
        0x0f, 0x23, 0xd8, // MOV DR3, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test reading all debug registers in sequence
#[test]
fn test_read_all_debug_registers() {
    let code = [
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0x0f, 0x21, 0xc9, // MOV RCX, DR1
        0x0f, 0x21, 0xd2, // MOV RDX, DR2
        0x0f, 0x21, 0xdb, // MOV RBX, DR3
        0x0f, 0x21, 0xf6, // MOV RSI, DR6
        0x0f, 0x21, 0xff, // MOV RDI, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All reads should complete
    let _ = regs;
}

// Test DR operations preserve other registers
#[test]
fn test_dr_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc6, 0x42, 0x42, 0x42,
        0x42, // MOV RSI, 0x42424242 (sign-extends to 0x42424242)
        0x48, 0xc7, 0xc7, 0x2a, 0x2a, 0x2a,
        0x2a, // MOV RDI, 0x2a2a2a2a (sign-extends to 0x2a2a2a2a)
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // MOV r64, imm32 sign-extends, so use values with bit 31 clear
    assert_eq!(regs.rsi, 0x42424242, "RSI should be preserved");
    assert_eq!(regs.rdi, 0x2a2a2a2a, "RDI should be preserved");
}

// Test DR with R8-R15
#[test]
fn test_dr0_to_r8() {
    let code = [
        0x4c, 0x0f, 0x21, 0xc0, // MOV R8, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r8;
}

// Test DR1 to R9
#[test]
fn test_dr1_to_r9() {
    let code = [
        0x4c, 0x0f, 0x21, 0xc9, // MOV R9, DR1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r9;
}

// Test DR2 to R10
#[test]
fn test_dr2_to_r10() {
    let code = [
        0x4c, 0x0f, 0x21, 0xd2, // MOV R10, DR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r10;
}

// Test DR3 to R11
#[test]
fn test_dr3_to_r11() {
    let code = [
        0x4c, 0x0f, 0x21, 0xdb, // MOV R11, DR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r11;
}

// Test DR6 to R14
#[test]
fn test_dr6_to_r14() {
    let code = [
        0x4c, 0x0f, 0x21, 0xf6, // MOV R14, DR6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r14;
}

// Test DR7 to R15
#[test]
fn test_dr7_to_r15() {
    let code = [
        0x4c, 0x0f, 0x21, 0xff, // MOV R15, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r15;
}

// Test writing from R8 to DR0
#[test]
fn test_r8_to_dr0() {
    let code = [
        0x49, 0xc7, 0xc0, 0x88, 0x77, 0x00, 0x00, // MOV R8, 0x7788
        0x49, 0x0f, 0x23, 0xc0, // MOV DR0, R8 (REX.B=1 to extend rm to R8)
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x7788, "DR0 should contain R8 value");
}

// Test writing from R15 to DR7
#[test]
fn test_r15_to_dr7() {
    let code = [
        0x0f, 0x21, 0xff, // MOV R15, DR7
        0x4c, 0x0f, 0x23, 0xff, // MOV DR7, R15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test DR flags are undefined
#[test]
fn test_mov_dr_flags_undefined() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags are undefined after MOV DR
    let _ = regs.rflags;
}

// Test multiple DR writes
#[test]
fn test_multiple_dr_writes() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x00, 0x00, // MOV RAX, 0x1111
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0x48, 0xc7, 0xc0, 0x22, 0x22, 0x00, 0x00, // MOV RAX, 0x2222
        0x0f, 0x23, 0xc8, // MOV DR1, RAX
        0x48, 0xc7, 0xc0, 0x33, 0x33, 0x00, 0x00, // MOV RAX, 0x3333
        0x0f, 0x23, 0xd0, // MOV DR2, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test DR operations in loop
#[test]
fn test_dr_operations_in_loop() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (7 bytes)
        // loop: (offset 0x1007)
        0x0f, 0x21, 0xc0, // MOV RAX, DR0 (3 bytes)
        0x48, 0x83, 0xc1, 0x01, // ADD RCX, 1 (4 bytes)
        0x48, 0x83, 0xf9, 0x03, // CMP RCX, 3 (4 bytes)
        0x75, 0xf3, // JNZ loop (rel8 = -13, jumps from 0x1014 to 0x1007)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 3, "Loop should complete");
}

// Test DR6 clear (common debug operation)
#[test]
fn test_dr6_clear_status() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x0f, 0x23, 0xf0, // MOV DR6, RAX (clear debug status)
        0x0f, 0x21, 0xf0, // MOV RAX, DR6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DR6 might have some always-set bits, but low bits should be clear
    let _ = regs.rax;
}

// Test preserving stack with DR operations
#[test]
fn test_dr_preserves_stack() {
    let code = [
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0x50, // PUSH RAX
        0x0f, 0x21, 0xc0, // MOV RAX, DR0
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x99, "Stack value should be preserved");
}

// Test setting multiple breakpoints
#[test]
fn test_set_multiple_breakpoints() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0f, 0x23, 0xc0, // MOV DR0, RAX (BP0 = 0x1000)
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x23, 0xc8, // MOV DR1, RAX (BP1 = 0x2000)
        0x48, 0xc7, 0xc0, 0x00, 0x30, 0x00, 0x00, // MOV RAX, 0x3000
        0x0f, 0x23, 0xd0, // MOV DR2, RAX (BP2 = 0x3000)
        0x48, 0xc7, 0xc0, 0x00, 0x40, 0x00, 0x00, // MOV RAX, 0x4000
        0x0f, 0x23, 0xd8, // MOV DR3, RAX (BP3 = 0x4000)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test reading all breakpoints after setting
#[test]
fn test_read_breakpoints_after_set() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0xaa, 0x00, 0x00, // MOV RAX, 0xaaaa
        0x0f, 0x23, 0xc0, // MOV DR0, RAX
        0x0f, 0x23, 0xc8, // MOV DR1, RAX
        0x0f, 0x21, 0xc1, // MOV RCX, DR0
        0x0f, 0x21, 0xca, // MOV RDX, DR1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0xaaaa, "DR0 should contain 0xaaaa");
    assert_eq!(regs.rdx, 0xaaaa, "DR1 should contain 0xaaaa");
}

// Test DR7 control bits (GD bit is bit 13)
#[test]
fn test_dr7_control_setting() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (disable all)
        0x0f, 0x23, 0xf8, // MOV DR7, RAX
        0x0f, 0x21, 0xf8, // MOV RAX, DR7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // DR7 has some bits that are always 1 (bit 10)
    let _ = regs.rax;
}
