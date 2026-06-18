use rax::cpu::Registers;

use crate::common::{
    DATA_ADDR, read_mem_at_u16, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u16,
    write_mem_at_u64,
};

// LGDT - Load Global Descriptor Table Register
// Opcode: 0F 01 /2
// Loads the GDTR from memory
// Format in memory: 2-byte limit, 8-byte base (in 64-bit mode)

// LIDT - Load Interrupt Descriptor Table Register
// Opcode: 0F 01 /3
// Loads the IDTR from memory
// Format in memory: 2-byte limit, 8-byte base (in 64-bit mode)

// LGDT m80 - Load GDTR from memory (basic)
#[test]
fn test_lgdt_basic() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up descriptor table pointer: limit=0x00FF, base=0x0000000000003000
    write_mem_at_u16(&mem, DATA_ADDR, 0x00FF);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000003000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify instruction completed
    assert_eq!(regs.rip, 0x1000 + 9, "RIP should point past HLT");
}

// LIDT m80 - Load IDTR from memory (basic)
#[test]
fn test_lidt_basic() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Set up descriptor table pointer: limit=0x00FF, base=0x0000000000004000
    write_mem_at_u16(&mem, DATA_ADDR, 0x00FF);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000004000);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify instruction completed
    assert_eq!(regs.rip, 0x1000 + 9, "RIP should point past HLT");
}

// LGDT with maximum limit value
#[test]
fn test_lgdt_max_limit() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Maximum limit value
    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000005000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LIDT with maximum limit value
#[test]
fn test_lidt_max_limit() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000006000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LGDT with zero limit
#[test]
fn test_lgdt_zero_limit() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000007000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LIDT with zero limit
#[test]
fn test_lidt_zero_limit() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000008000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LGDT with high base address
#[test]
fn test_lgdt_high_base() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0800);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x00000000FFFF0000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LIDT with high base address
#[test]
fn test_lidt_high_base() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0800);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x00000000FFFF8000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LGDT using RAX register indirect
#[test]
fn test_lgdt_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x01, 0x10, // LGDT [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0100);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000009000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// LIDT using RBX register indirect
#[test]
fn test_lidt_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x01, 0x1b, // LIDT [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0100);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000A000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// LGDT using RCX register indirect
#[test]
fn test_lgdt_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x01, 0x11, // LGDT [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0200);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000B000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// LIDT using RDX register indirect
#[test]
fn test_lidt_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x01, 0x1a, // LIDT [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0200);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000C000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// LGDT using RSI register indirect
#[test]
fn test_lgdt_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x01, 0x16, // LGDT [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0300);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000D000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// LIDT using RDI register indirect
#[test]
fn test_lidt_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x01, 0x1f, // LIDT [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0300);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000E000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// LGDT using R8 register indirect
#[test]
fn test_lgdt_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x01, 0x10, // LGDT [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0400);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000000F000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT using R9 register indirect
#[test]
fn test_lidt_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x01, 0x19, // LIDT [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0400);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000010000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT using R10 register indirect
#[test]
fn test_lgdt_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x01, 0x12, // LGDT [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0500);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000011000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT using R11 register indirect
#[test]
fn test_lidt_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x01, 0x1b, // LIDT [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0500);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000012000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT with displacement
#[test]
fn test_lgdt_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x01, 0x90, 0x00, 0x01, 0x00, 0x00, // LGDT [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0600);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000013000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// LIDT with displacement
#[test]
fn test_lidt_displacement() {
    let code = [
        0x48, 0xbb, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x1F00
        0x0f, 0x01, 0x9b, 0x00, 0x01, 0x00, 0x00, // LIDT [RBX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0600);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000014000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1F00);
}

// LGDT with negative displacement
#[test]
fn test_lgdt_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x01, 0x90, 0x00, 0xFF, 0xFF, 0xFF, // LGDT [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0700);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000015000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// LIDT with negative displacement
#[test]
fn test_lidt_negative_displacement() {
    let code = [
        0x48, 0xbb, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2100
        0x0f, 0x01, 0x9b, 0x00, 0xFF, 0xFF, 0xFF, // LIDT [RBX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0700);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000016000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x2100);
}

// LGDT preserves all registers
#[test]
fn test_lgdt_preserves_registers() {
    let code = [
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RAX, 0x1111111111111111
        0x48, 0xbb, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RBX, 0x2222222222222222
        0x48, 0xb9, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, // MOV RCX, 0x3333333333333333
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0800);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000017000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111111111111111);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
}

// LIDT preserves all registers
#[test]
fn test_lidt_preserves_registers() {
    let code = [
        0x48, 0xb8, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, // MOV RAX, 0x4444444444444444
        0x48, 0xbb, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55, // MOV RBX, 0x5555555555555555
        0x48, 0xb9, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, // MOV RCX, 0x6666666666666666
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0800);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000018000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x4444444444444444);
    assert_eq!(regs.rbx, 0x5555555555555555);
    assert_eq!(regs.rcx, 0x6666666666666666);
}

// LGDT followed by LIDT
#[test]
fn test_lgdt_then_lidt() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0x0f, 0x01, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, // LIDT [0x2010]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // GDTR data
    write_mem_at_u16(&mem, DATA_ADDR, 0x0900);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000019000);

    // IDTR data
    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x0A00);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x000000000001A000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT using RBP register indirect
#[test]
fn test_lgdt_rbp_indirect() {
    let code = [
        0x48, 0xbd, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x2000
        0x0f, 0x01, 0x55, 0x00, // LGDT [RBP]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0B00);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000001B000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbp, DATA_ADDR);
}

// LIDT using R12 register indirect
#[test]
fn test_lidt_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x01, 0x1c, 0x24, // LIDT [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0C00);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000001C000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT using R13 register indirect with displacement
#[test]
fn test_lgdt_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x01, 0x95, 0x00, 0x01, 0x00, 0x00, // LGDT [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0D00);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000001D000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT using R14 register indirect
#[test]
fn test_lidt_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x01, 0x1e, // LIDT [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0E00);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000001E000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT using R15 register indirect
#[test]
fn test_lgdt_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x01, 0x17, // LGDT [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0F00);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x000000000001F000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT with various limit values
#[test]
fn test_lgdt_limit_0x0001() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0001);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000020000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT with various limit values
#[test]
fn test_lidt_limit_0x07FF() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x07FF);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000021000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT with aligned base address
#[test]
fn test_lgdt_aligned_base() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x1000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000100000); // 1MB aligned

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT with page-aligned base address
#[test]
fn test_lidt_page_aligned_base() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x1000);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000200000); // 2MB aligned

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LGDT multiple times
#[test]
fn test_lgdt_multiple_times() {
    let code = [
        0x0f, 0x01, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LGDT [0x2000]
        0x0f, 0x01, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // LGDT [0x2010]
        0x0f, 0x01, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00, // LGDT [0x2020]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x1100);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000022000);

    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x1200);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000023000);

    write_mem_at_u16(&mem, DATA_ADDR + 0x20, 0x1300);
    write_mem_at_u64(&mem, DATA_ADDR + 0x22, 0x0000000000024000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LIDT multiple times
#[test]
fn test_lidt_multiple_times() {
    let code = [
        0x0f, 0x01, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LIDT [0x2000]
        0x0f, 0x01, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, // LIDT [0x2010]
        0x0f, 0x01, 0x1c, 0x25, 0x20, 0x20, 0x00, 0x00, // LIDT [0x2020]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x1400);
    write_mem_at_u64(&mem, DATA_ADDR + 2, 0x0000000000025000);

    write_mem_at_u16(&mem, DATA_ADDR + 0x10, 0x1500);
    write_mem_at_u64(&mem, DATA_ADDR + 0x12, 0x0000000000026000);

    write_mem_at_u16(&mem, DATA_ADDR + 0x20, 0x1600);
    write_mem_at_u64(&mem, DATA_ADDR + 0x22, 0x0000000000027000);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
