use rax::cpu::Registers;

use crate::common::{
    DATA_ADDR, read_mem_at_u16, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u16,
    write_mem_at_u64,
};

// SGDT - Store Global Descriptor Table Register
// Opcode: 0F 01 /0
// Stores the GDTR to memory
// Format in memory: 2-byte limit, 8-byte base (in 64-bit mode)

// SIDT - Store Interrupt Descriptor Table Register
// Opcode: 0F 01 /1
// Stores the IDTR to memory
// Format in memory: 2-byte limit, 8-byte base (in 64-bit mode)

// SGDT m80 - Store GDTR to memory (basic)
#[test]
fn test_sgdt_basic() {
    let code = [
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Clear the memory area to verify SGDT writes
    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify instruction completed
    assert_eq!(regs.rip, 0x1000 + 9, "RIP should point past HLT");

    // Memory should now contain GDTR data
    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);

    // Values depend on VM initialization, but should be valid
    // Just verify it wrote something
    assert!(limit > 0 || base > 0, "SGDT should write GDTR data");
}

// SIDT m80 - Store IDTR to memory (basic)
#[test]
fn test_sidt_basic() {
    let code = [
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Clear the memory area
    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rip, 0x1000 + 9, "RIP should point past HLT");

    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);

    assert!(limit > 0 || base > 0, "SIDT should write IDTR data");
}

// SGDT using RAX register indirect
#[test]
fn test_sgdt_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x01, 0x00, // SGDT [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);

    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);
    assert!(limit > 0 || base > 0);
}

// SIDT using RBX register indirect
#[test]
fn test_sidt_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x01, 0x0b, // SIDT [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);

    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);
    assert!(limit > 0 || base > 0);
}

// SGDT using RCX register indirect
#[test]
fn test_sgdt_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x01, 0x01, // SGDT [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// SIDT using RDX register indirect
#[test]
fn test_sidt_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x01, 0x0a, // SIDT [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// SGDT using RSI register indirect
#[test]
fn test_sgdt_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x01, 0x06, // SGDT [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// SIDT using RDI register indirect
#[test]
fn test_sidt_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x01, 0x0f, // SIDT [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// SGDT using R8 register indirect
#[test]
fn test_sgdt_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x01, 0x00, // SGDT [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SIDT using R9 register indirect
#[test]
fn test_sidt_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x01, 0x09, // SIDT [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SGDT using R10 register indirect
#[test]
fn test_sgdt_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x01, 0x02, // SGDT [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SIDT using R11 register indirect
#[test]
fn test_sidt_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x01, 0x0b, // SIDT [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SGDT with displacement
#[test]
fn test_sgdt_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x01, 0x80, 0x00, 0x01, 0x00, 0x00, // SGDT [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// SIDT with displacement
#[test]
fn test_sidt_displacement() {
    let code = [
        0x48, 0xbb, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x1F00
        0x0f, 0x01, 0x8b, 0x00, 0x01, 0x00, 0x00, // SIDT [RBX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1F00);
}

// SGDT with negative displacement
#[test]
fn test_sgdt_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x01, 0x80, 0x00, 0xFF, 0xFF, 0xFF, // SGDT [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// SIDT with negative displacement
#[test]
fn test_sidt_negative_displacement() {
    let code = [
        0x48, 0xbb, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2100
        0x0f, 0x01, 0x8b, 0x00, 0xFF, 0xFF, 0xFF, // SIDT [RBX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x2100);
}

// SGDT preserves all registers
#[test]
fn test_sgdt_preserves_registers() {
    let code = [
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RAX, 0x1111111111111111
        0x48, 0xbb, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RBX, 0x2222222222222222
        0x48, 0xb9, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, // MOV RCX, 0x3333333333333333
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
}

// SIDT preserves all registers
#[test]
fn test_sidt_preserves_registers() {
    let code = [
        0x48, 0xb8, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, // MOV RAX, 0x4444444444444444
        0x48, 0xbb, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, // MOV RBX, 0x5555555555555555
        0x48, 0xb9, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, // MOV RCX, 0x6666666666666666
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x4444444444444444);
    assert_eq!(regs.rbx, 0x5555555555555555);
    assert_eq!(regs.rcx, 0x6666666666666666);
}

// SGDT followed by SIDT
#[test]
fn test_sgdt_then_sidt() {
    let code = [
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0x0f, 0x01, 0x0c, 0x25, 0x10, 0x20, 0x00, 0x00, // SIDT [0x2010]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Clear both memory areas
    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Both should have written data
    let gdt_limit = read_mem_at_u16(&mem, DATA_ADDR);
    let gdt_base = read_mem_at_u64(&mem, DATA_ADDR + 2);
    let idt_limit = read_mem_at_u16(&mem, DATA_ADDR + 0x10);
    let idt_base = read_mem_at_u64(&mem, DATA_ADDR + 0x12);

    assert!(gdt_limit > 0 || gdt_base > 0);
    assert!(idt_limit > 0 || idt_base > 0);
}

// SGDT using RBP register indirect
#[test]
fn test_sgdt_rbp_indirect() {
    let code = [
        0x48, 0xbd, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x2000
        0x0f, 0x01, 0x45, 0x00, // SGDT [RBP]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbp, DATA_ADDR);
}

// SIDT using R12 register indirect
#[test]
fn test_sidt_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x01, 0x0c, 0x24, // SIDT [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SGDT using R13 register indirect with displacement
#[test]
fn test_sgdt_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x01, 0x85, 0x00, 0x01, 0x00, 0x00, // SGDT [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SIDT using R14 register indirect
#[test]
fn test_sidt_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x01, 0x0e, // SIDT [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SGDT using R15 register indirect
#[test]
fn test_sgdt_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x01, 0x07, // SGDT [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SGDT multiple times to same location
#[test]
fn test_sgdt_multiple_times() {
    let code = [
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);
    assert!(limit > 0 || base > 0);
}

// SIDT multiple times to same location
#[test]
fn test_sidt_multiple_times() {
    let code = [
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    let limit = read_mem_at_u16(&mem, DATA_ADDR);
    let base = read_mem_at_u64(&mem, DATA_ADDR + 2);
    assert!(limit > 0 || base > 0);
}

// SGDT to different locations
#[test]
fn test_sgdt_different_locations() {
    let code = [
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0x0f, 0x01, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // SGDT [0x2010]
        0x0f, 0x01, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00, // SGDT [0x2020]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000000000);
    write_mem_at_u16(&mem, DATA_ADDR + 0x20, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x22, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // All three locations should have data
    let limit1 = read_mem_at_u16(&mem, DATA_ADDR);
    let limit2 = read_mem_at_u16(&mem, DATA_ADDR + 0x10);
    let limit3 = read_mem_at_u16(&mem, DATA_ADDR + 0x20);

    assert!(limit1 > 0 || limit2 > 0 || limit3 > 0);
}

// SIDT to different locations
#[test]
fn test_sidt_different_locations() {
    let code = [
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0x0f, 0x01, 0x0c, 0x25, 0x10, 0x20, 0x00, 0x00, // SIDT [0x2010]
        0x0f, 0x01, 0x0c, 0x25, 0x20, 0x20, 0x00, 0x00, // SIDT [0x2020]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000000000);
    write_mem_at_u16(&mem, DATA_ADDR + 0x20, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x22, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    let limit1 = read_mem_at_u16(&mem, DATA_ADDR);
    let limit2 = read_mem_at_u16(&mem, DATA_ADDR + 0x10);
    let limit3 = read_mem_at_u16(&mem, DATA_ADDR + 0x20);

    assert!(limit1 > 0 || limit2 > 0 || limit3 > 0);
}

// LGDT then SGDT to verify round-trip
#[test]
fn test_lgdt_sgdt_roundtrip() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0x0f, 0x01, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // SGDT [0x2010]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up known GDTR values
    write_mem_at_u16(&mem, DATA_ADDR, 0xABCD);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000012345678);

    // Clear destination
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify round-trip (values should match)
    let stored_limit = read_mem_at_u16(&mem, DATA_ADDR + 0x10);
    let stored_base = read_mem_at_u64(&mem, DATA_ADDR + 0x12);

    assert_eq!(stored_limit, 0xABCD, "Limit should match after LGDT/SGDT");
    assert_eq!(
        stored_base, 0x0000000012345678,
        "Base should match after LGDT/SGDT"
    );
}

// LIDT then SIDT to verify round-trip
#[test]
fn test_lidt_sidt_roundtrip() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0x0f, 0x01, 0x0c, 0x25, 0x10, 0x20, 0x00, 0x00, // SIDT [0x2010]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up known IDTR values
    write_mem_at_u16(&mem, DATA_ADDR, 0x1234);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x00000000ABCDEF00);

    // Clear destination
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify round-trip
    let stored_limit = read_mem_at_u16(&mem, DATA_ADDR + 0x10);
    let stored_base = read_mem_at_u64(&mem, DATA_ADDR + 0x12);

    assert_eq!(stored_limit, 0x1234, "Limit should match after LIDT/SIDT");
    assert_eq!(
        stored_base, 0x00000000ABCDEF00,
        "Base should match after LIDT/SIDT"
    );
}

// SGDT doesn't affect flags
#[test]
fn test_sgdt_no_flags_change() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets flags)
        0x9c, // PUSHFQ
        0x48, 0x8f, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00, // POP [0x2030]
        0x0f, 0x01, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SGDT [0x2000]
        0x9c, // PUSHFQ
        0x48, 0x8f, 0x04, 0x25, 0x38, 0x20, 0x00, 0x00, // POP [0x2038]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Flags before and after SGDT should be the same
    let flags_before = read_mem_at_u64(&mem, DATA_ADDR + 0x30);
    let flags_after = read_mem_at_u64(&mem, DATA_ADDR + 0x38);

    assert_eq!(flags_before, flags_after, "SGDT should not modify flags");
}

// SIDT doesn't affect flags
#[test]
fn test_sidt_no_flags_change() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets flags)
        0x9c, // PUSHFQ
        0x48, 0x8f, 0x04, 0x25, 0x30, 0x20, 0x00, 0x00, // POP [0x2030]
        0x0f, 0x01, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // SIDT [0x2000]
        0x9c, // PUSHFQ
        0x48, 0x8f, 0x04, 0x25, 0x38, 0x20, 0x00, 0x00, // POP [0x2038]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000000000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    let flags_before = read_mem_at_u64(&mem, DATA_ADDR + 0x30);
    let flags_after = read_mem_at_u64(&mem, DATA_ADDR + 0x38);

    assert_eq!(flags_before, flags_after, "SIDT should not modify flags");
}
