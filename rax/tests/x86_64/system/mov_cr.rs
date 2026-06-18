use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;

// MOV CR - Move to/from Control Registers
// Opcodes:
// 0F 20 /r - MOV r64, CR0-CR7 (read control register)
// 0F 22 /r - MOV CR0-CR7, r64 (write control register)
// REX.R + 0F 20/0 - MOV r64, CR8
// REX.R + 0F 22/0 - MOV CR8, r64
//
// Control Registers:
// CR0 - System control flags (PE, MP, EM, TS, ET, NE, WP, AM, NW, CD, PG)
// CR2 - Page fault linear address
// CR3 - Page directory base register (PDBR)
// CR4 - Extended control flags (VME, PVI, TSD, DE, PSE, PAE, MCE, PGE, etc.)
// CR8 - Task priority register (TPR) - 64-bit mode only

// Test MOV from CR0 to RAX
#[test]
fn test_mov_cr0_to_rax() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // setup_vm initializes CR0 = 0x00050033 (PE|MP|ET|NE|WP|AM, PG clear).
    assert_eq!(regs.rax, 0x00050033, "CR0 exact default");
    assert!(regs.rax & 1 != 0, "PE set");
    assert_eq!(regs.rax >> 31 & 1, 0, "PG clear (no paging)");
}

// Test MOV from CR0 to RBX
#[test]
fn test_mov_cr0_to_rbx() {
    let code = [
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rbx != 0, "CR0 loaded into RBX");
}

// Test MOV from CR0 to RCX
#[test]
fn test_mov_cr0_to_rcx() {
    let code = [
        0x0f, 0x20, 0xc1, // MOV RCX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rcx != 0, "CR0 loaded into RCX");
}

// Test MOV from CR0 to RDX
#[test]
fn test_mov_cr0_to_rdx() {
    let code = [
        0x0f, 0x20, 0xc2, // MOV RDX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rdx != 0, "CR0 loaded into RDX");
}

// Test MOV from CR2 to RAX
#[test]
fn test_mov_cr2_to_rax() {
    let code = [
        0x0f, 0x20, 0xd0, // MOV RAX, CR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CR2 contains page fault address, should be 0 if no page fault occurred
    let _ = regs.rax;
}

// Test MOV from CR2 to RBX
#[test]
fn test_mov_cr2_to_rbx() {
    let code = [
        0x0f, 0x20, 0xd3, // MOV RBX, CR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rbx;
}

// Test MOV from CR3 to RAX
#[test]
fn test_mov_cr3_to_rax() {
    let code = [
        0x0f, 0x20, 0xd8, // MOV RAX, CR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CR3 should contain page directory base
    let _ = regs.rax;
}

// Test MOV from CR3 to RDX
#[test]
fn test_mov_cr3_to_rdx() {
    let code = [
        0x0f, 0x20, 0xda, // MOV RDX, CR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rdx;
}

// Test MOV from CR4 to RAX
#[test]
fn test_mov_cr4_to_rax() {
    let code = [
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // setup_vm initializes CR4 = 0x20 (PAE, bit 5).
    assert_eq!(regs.rax, 0x20, "CR4 exact default (PAE)");
    assert!(regs.rax & (1 << 5) != 0, "PAE set");
}

// Test MOV from CR4 to RBX
#[test]
fn test_mov_cr4_to_rbx() {
    let code = [
        0x0f, 0x20, 0xe3, // MOV RBX, CR4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rbx;
}

// Test MOV from CR8 to RAX (requires REX.R prefix)
#[test]
fn test_mov_cr8_to_rax() {
    let code = [
        0x44, 0x0f, 0x20, 0xc0, // MOV RAX, CR8 (REX.R + 0F 20 /0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CR8 is task priority register, value depends on system state
    let _ = regs.rax;
}

// Test MOV from CR8 to RBX
#[test]
fn test_mov_cr8_to_rbx() {
    let code = [
        0x44, 0x0f, 0x20, 0xc3, // MOV RBX, CR8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.rbx;
}

// Test MOV to CR0 from RAX (write back same value)
#[test]
fn test_mov_rax_to_cr0() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should complete successfully
    let _ = regs;
}

// Test MOV to CR0 from RBX
#[test]
fn test_mov_rbx_to_cr0() {
    let code = [
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0x0f, 0x22, 0xc3, // MOV CR0, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR2 from RAX
#[test]
fn test_mov_rax_to_cr2() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x10, 0x00, 0x00, // MOV RAX, 0x1000
        0x0f, 0x22, 0xd0, // MOV CR2, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR2 from RDX
#[test]
fn test_mov_rdx_to_cr2() {
    let code = [
        0x48, 0xc7, 0xc2, 0x00, 0x20, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x22, 0xd2, // MOV CR2, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR3 from RAX (write back same value)
#[test]
fn test_mov_rax_to_cr3() {
    let code = [
        0x0f, 0x20, 0xd8, // MOV RAX, CR3
        0x0f, 0x22, 0xd8, // MOV CR3, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR3 from RBX
#[test]
fn test_mov_rbx_to_cr3() {
    let code = [
        0x0f, 0x20, 0xdb, // MOV RBX, CR3
        0x0f, 0x22, 0xdb, // MOV CR3, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR4 from RAX (write back same value)
#[test]
fn test_mov_rax_to_cr4() {
    let code = [
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR4 from RCX
#[test]
fn test_mov_rcx_to_cr4() {
    let code = [
        0x0f, 0x20, 0xe1, // MOV RCX, CR4
        0x0f, 0x22, 0xe1, // MOV CR4, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR8 from RAX
#[test]
fn test_mov_rax_to_cr8() {
    let code = [
        0x44, 0x0f, 0x20, 0xc0, // MOV RAX, CR8
        0x44, 0x0f, 0x22, 0xc0, // MOV CR8, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test MOV to CR8 from RDX
#[test]
fn test_mov_rdx_to_cr8() {
    let code = [
        0x48, 0x31, 0xd2, // XOR RDX, RDX
        0x44, 0x0f, 0x22, 0xc2, // MOV CR8, RDX (set TPR to 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test round-trip CR0 read/write
#[test]
fn test_cr0_round_trip() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should match RBX (original CR0 value)
    assert_eq!(regs.rax, regs.rbx, "CR0 value should be preserved");
}

// Test round-trip CR3 read/write
#[test]
fn test_cr3_round_trip() {
    let code = [
        0x0f, 0x20, 0xd8, // MOV RAX, CR3
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x22, 0xd8, // MOV CR3, RAX
        0x0f, 0x20, 0xd8, // MOV RAX, CR3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should match RBX (original CR3 value)
    assert_eq!(regs.rax, regs.rbx, "CR3 value should be preserved");
}

// Test round-trip CR4 read/write
#[test]
fn test_cr4_round_trip() {
    let code = [
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0x0f, 0x20, 0xe0, // MOV RAX, CR4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, regs.rbx, "CR4 value should be preserved");
}

// Test CR2 write/read
#[test]
fn test_cr2_write_read() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x0f, 0x22, 0xd0, // MOV CR2, RAX
        0x0f, 0x20, 0xd0, // MOV RAX, CR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1234, "CR2 should contain written value");
}

// Test CR8 write/read with value 0
#[test]
fn test_cr8_write_read_zero() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x44, 0x0f, 0x22, 0xc0, // MOV CR8, RAX
        0x44, 0x0f, 0x20, 0xc0, // MOV RAX, CR8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "CR8 should be 0");
}

// Test CR8 with different values
#[test]
fn test_cr8_different_values() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x44, 0x0f, 0x22, 0xc0, // MOV CR8, RAX
        0x44, 0x0f, 0x20, 0xc1, // MOV RCX, CR8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "CR8 should contain 1");
}

// Test preserving other registers during CR0 operations
#[test]
fn test_cr0_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc6, 0x42, 0x42, 0x42, 0x42, // MOV RSI, 0x42424242
        0x48, 0xc7, 0xc7, 0x2a, 0x2a, 0x2a,
        0x2a, // MOV RDI, 0x2a2a2a2a (bit 31 clear to avoid sign-extension)
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x42424242, "RSI should be preserved");
    assert_eq!(regs.rdi, 0x2a2a2a2a, "RDI should be preserved");
}

// Test multiple CR reads in sequence
#[test]
fn test_multiple_cr_reads() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x20, 0xd3, // MOV RBX, CR2
        0x0f, 0x20, 0xd9, // MOV RCX, CR3
        0x0f, 0x20, 0xe2, // MOV RDX, CR4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should complete successfully
    assert!(regs.rax != 0, "CR0 should be non-zero");
}

// Test CR operations with R8-R15
#[test]
fn test_cr0_to_r8() {
    let code = [
        0x49, 0x0f, 0x20, 0xc0, // MOV R8, CR0 (REX.W + REX.B extends rm field)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.r8 != 0, "CR0 loaded into R8");
}

// Test CR0 to R15
#[test]
fn test_cr0_to_r15() {
    let code = [
        0x49, 0x0f, 0x20, 0xc7, // MOV R15, CR0 (REX.B extends rm field)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.r15 != 0, "CR0 loaded into R15");
}

// Test CR3 to R8
#[test]
fn test_cr3_to_r8() {
    let code = [
        0x49, 0x0f, 0x20, 0xd8, // MOV R8, CR3 (REX.B extends rm field)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r8;
}

// Test CR4 to R9
#[test]
fn test_cr4_to_r9() {
    let code = [
        0x49, 0x0f, 0x20, 0xe1, // MOV R9, CR4 (REX.B extends rm field)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs.r9;
}

// Test writing from R8 to CR0
#[test]
fn test_r8_to_cr0() {
    let code = [
        0x49, 0x0f, 0x20, 0xc0, // MOV R8, CR0 (REX.B extends rm field)
        0x49, 0x0f, 0x22, 0xc0, // MOV CR0, R8 (REX.B extends rm field)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test CR flags are undefined (MOV CR affects flags)
#[test]
fn test_mov_cr_flags_undefined() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags are undefined after MOV CR
    let _ = regs.rflags;
}

// Test sequential CR writes
#[test]
fn test_sequential_cr_writes() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let _ = regs;
}

// Test CR2 with different addresses
#[test]
fn test_cr2_various_addresses() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x01, 0x00, // MOV RAX, 0x10000
        0x0f, 0x22, 0xd0, // MOV CR2, RAX
        0x0f, 0x20, 0xd1, // MOV RCX, CR2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x10000, "CR2 should contain 0x10000");
}

// Test CR operations preserve stack
#[test]
fn test_cr_preserves_stack() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "Stack value should be preserved");
}

// Test CR0 read preserves instruction pointer advancement
#[test]
fn test_cr0_read_advances_rip() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x99, "RIP should advance correctly");
}

// Test CR8 priority levels (0-15)
#[test]
fn test_cr8_priority_level_15() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15
        0x44, 0x0f, 0x22, 0xc0, // MOV CR8, RAX
        0x44, 0x0f, 0x20, 0xc1, // MOV RCX, CR8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 15, "CR8 should contain priority 15");
}

// Test CR operations in loop
#[test]
fn test_cr_operations_in_loop() {
    let code = [
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (offset 0, 7 bytes)
        // loop: (offset 7)
        0x0f, 0x20, 0xc0, // MOV RAX, CR0 (offset 7, 3 bytes)
        0x48, 0x83, 0xc1, 0x01, // ADD RCX, 1 (offset 10, 4 bytes)
        0x48, 0x83, 0xf9, 0x03, // CMP RCX, 3 (offset 14, 4 bytes)
        0x75, 0xf3, // JNZ loop (offset 18, disp = 7 - 20 = -13 = 0xf3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 3, "Loop should complete");
}

// Test CR0 multiple register destinations
#[test]
fn test_cr0_to_multiple_registers() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0x0f, 0x20, 0xc1, // MOV RCX, CR0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All should have same CR0 value
    assert_eq!(regs.rax, regs.rbx, "CR0 values should match");
    assert_eq!(regs.rax, regs.rcx, "CR0 values should match");
}

// ============================================================================
// Strengthened: CR0/CR4 specific bit read/write assertions.
// ============================================================================

// Set CR0.WP (bit 16) on top of the default, read it back exactly.
#[test]
fn test_cr0_set_wp_bit_roundtrip() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0  (= 0x00050033)
        0x48, 0x0d, 0x00, 0x00, 0x01, 0x00, // OR RAX, 0x10000 (WP, bit 16)
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x0f, 0x20, 0xc3, // MOV RBX, CR0 (read back)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // WP was already set in 0x00050033, OR is idempotent; value unchanged.
    assert_eq!(regs.rbx, 0x00050033, "CR0 retains WP after OR");
    assert!(regs.rbx & (1 << 16) != 0, "WP set");
}

// Clearing CR0.MP (bit 1) and reading it back. Avoids touching PG/PE so the
// no-paging instruction-fetch path stays valid.
#[test]
fn test_cr0_clear_mp_bit_roundtrip() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0 (= 0x00050033, MP set)
        0x48, 0x83, 0xe0, 0xfd, // AND RAX, ~2 (clear MP, bit 1)
        0x0f, 0x22, 0xc0, // MOV CR0, RAX
        0x0f, 0x20, 0xc1, // MOV RCX, CR0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x00050031, "CR0 with MP cleared");
    assert_eq!(regs.rcx & 2, 0, "MP cleared");
    assert!(regs.rcx & 1 != 0, "PE preserved");
}

// Write a fresh CR4 value (PAE|PSE|OSFXSR|OSXSAVE) and read it back exactly.
#[test]
fn test_cr4_write_read_exact() {
    // 0x30 = PAE|PSE; 0x200 = OSFXSR (bit 9); 0x40000 = OSXSAVE (bit 18).
    let code = [
        0x48, 0xc7, 0xc0, 0x30, 0x02, 0x04, 0x00, // MOV RAX, 0x40230
        0x0f, 0x22, 0xe0, // MOV CR4, RAX
        0x0f, 0x20, 0xe3, // MOV RBX, CR4
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x40230, "CR4 stores written value exactly");
    assert!(regs.rbx & (1 << 5) != 0, "PAE");
    assert!(regs.rbx & (1 << 4) != 0, "PSE");
    assert!(regs.rbx & (1 << 9) != 0, "OSFXSR");
    assert!(regs.rbx & (1 << 18) != 0, "OSXSAVE");
}

// CLTS clears CR0.TS (bit 3). Set TS, then CLTS, verify it is cleared.
#[test]
fn test_clts_clears_cr0_ts() {
    let code = [
        0x0f, 0x20, 0xc0, // MOV RAX, CR0
        0x48, 0x83, 0xc8, 0x08, // OR RAX, 8 (TS, bit 3)
        0x0f, 0x22, 0xc0, // MOV CR0, RAX (TS now set)
        0x0f, 0x06, // CLTS
        0x0f, 0x20, 0xc3, // MOV RBX, CR0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 8, 0, "CLTS cleared CR0.TS");
    // The rest of CR0 (PE etc.) is preserved.
    assert!(regs.rbx & 1 != 0, "PE preserved by CLTS");
}
