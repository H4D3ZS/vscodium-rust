use rax::cpu::Registers;

use crate::common::{DATA_ADDR, run_until_hlt, setup_vm, write_mem_at_u16};

// LLDT - Load Local Descriptor Table Register
// Opcode: 0F 00 /2
// Loads the LDTR from a 16-bit segment selector
// The selector is loaded from a register or memory operand

// LLDT r16 - Load LDTR from AX
#[test]
fn test_lldt_ax() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0008, "AX should be preserved");
    assert_eq!(regs.rip, 0x1000 + 8, "RIP should point past HLT");
}

// LLDT r16 - Load LDTR from BX
#[test]
fn test_lldt_bx() {
    let code = [
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xd3, // LLDT BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x0010);
}

// LLDT r16 - Load LDTR from CX
#[test]
fn test_lldt_cx() {
    let code = [
        0x66, 0xb9, 0x18, 0x00, // MOV CX, 0x0018
        0x0f, 0x00, 0xd1, // LLDT CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x0018);
}

// LLDT r16 - Load LDTR from DX
#[test]
fn test_lldt_dx() {
    let code = [
        0x66, 0xba, 0x20, 0x00, // MOV DX, 0x0020
        0x0f, 0x00, 0xd2, // LLDT DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x0020);
}

// LLDT r16 - Load LDTR from SI
#[test]
fn test_lldt_si() {
    let code = [
        0x66, 0xbe, 0x28, 0x00, // MOV SI, 0x0028
        0x0f, 0x00, 0xd6, // LLDT SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0x0028);
}

// LLDT r16 - Load LDTR from DI
#[test]
fn test_lldt_di() {
    let code = [
        0x66, 0xbf, 0x30, 0x00, // MOV DI, 0x0030
        0x0f, 0x00, 0xd7, // LLDT DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi & 0xFFFF, 0x0030);
}

// LLDT r16 - Load LDTR from BP
#[test]
fn test_lldt_bp() {
    let code = [
        0x66, 0xbd, 0x38, 0x00, // MOV BP, 0x0038
        0x0f, 0x00, 0xd5, // LLDT BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp & 0xFFFF, 0x0038);
}

// LLDT r16 - Load LDTR from SP
#[test]
fn test_lldt_sp() {
    let code = [
        0x66, 0xbc, 0x40, 0x00, // MOV SP, 0x0040
        0x0f, 0x00, 0xd4, // LLDT SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R8W
#[test]
fn test_lldt_r8w() {
    let code = [
        0x66, 0x41, 0xb8, 0x48, 0x00, // MOV R8W, 0x0048
        0x41, 0x0f, 0x00, 0xd0, // LLDT R8W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R9W
#[test]
fn test_lldt_r9w() {
    let code = [
        0x66, 0x41, 0xb9, 0x50, 0x00, // MOV R9W, 0x0050
        0x41, 0x0f, 0x00, 0xd1, // LLDT R9W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R10W
#[test]
fn test_lldt_r10w() {
    let code = [
        0x66, 0x41, 0xba, 0x58, 0x00, // MOV R10W, 0x0058
        0x41, 0x0f, 0x00, 0xd2, // LLDT R10W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R11W
#[test]
fn test_lldt_r11w() {
    let code = [
        0x66, 0x41, 0xbb, 0x60, 0x00, // MOV R11W, 0x0060
        0x41, 0x0f, 0x00, 0xd3, // LLDT R11W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R12W
#[test]
fn test_lldt_r12w() {
    let code = [
        0x66, 0x41, 0xbc, 0x68, 0x00, // MOV R12W, 0x0068
        0x41, 0x0f, 0x00, 0xd4, // LLDT R12W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R13W
#[test]
fn test_lldt_r13w() {
    let code = [
        0x66, 0x41, 0xbd, 0x70, 0x00, // MOV R13W, 0x0070
        0x41, 0x0f, 0x00, 0xd5, // LLDT R13W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R14W
#[test]
fn test_lldt_r14w() {
    let code = [
        0x66, 0x41, 0xbe, 0x78, 0x00, // MOV R14W, 0x0078
        0x41, 0x0f, 0x00, 0xd6, // LLDT R14W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT r16 - Load LDTR from R15W
#[test]
fn test_lldt_r15w() {
    let code = [
        0x66, 0x41, 0xbf, 0x80, 0x00, // MOV R15W, 0x0080
        0x41, 0x0f, 0x00, 0xd7, // LLDT R15W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 - Load LDTR from memory
#[test]
fn test_lldt_memory() {
    let code = [
        0x0f, 0x00, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // LLDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0008);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LLDT m16 - Load LDTR from memory via RAX
#[test]
fn test_lldt_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x00, 0x10, // LLDT [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0010);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via RBX
#[test]
fn test_lldt_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x00, 0x13, // LLDT [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0018);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via RCX
#[test]
fn test_lldt_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x00, 0x11, // LLDT [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0020);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via RDX
#[test]
fn test_lldt_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x00, 0x12, // LLDT [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0028);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via RSI
#[test]
fn test_lldt_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x00, 0x16, // LLDT [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0030);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via RDI
#[test]
fn test_lldt_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x00, 0x17, // LLDT [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0038);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// LLDT m16 - Load LDTR from memory via R8
#[test]
fn test_lldt_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x00, 0x10, // LLDT [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0040);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 - Load LDTR from memory via R9
#[test]
fn test_lldt_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x00, 0x11, // LLDT [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0048);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 - Load LDTR from memory via R10
#[test]
fn test_lldt_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x00, 0x12, // LLDT [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0050);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 - Load LDTR from memory via R11
#[test]
fn test_lldt_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x00, 0x13, // LLDT [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0058);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with displacement
#[test]
fn test_lldt_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x00, 0x90, 0x00, 0x01, 0x00, 0x00, // LLDT [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0060);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// LLDT with negative displacement
#[test]
fn test_lldt_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x00, 0x90, 0x00, 0xFF, 0xFF, 0xFF, // LLDT [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0068);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// LLDT with zero selector (null selector)
#[test]
fn test_lldt_null_selector() {
    let code = [
        0x66, 0xb8, 0x00, 0x00, // MOV AX, 0x0000
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000);
}

// LLDT preserves other registers
#[test]
fn test_lldt_preserves_registers() {
    let code = [
        0x48, 0xbb, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RBX, 0x1111111111111111
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RCX, 0x2222222222222222
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

// LLDT multiple times
#[test]
fn test_lldt_multiple_times() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xd0, // LLDT AX
        0x66, 0xb8, 0x10, 0x00, // MOV AX, 0x0010
        0x0f, 0x00, 0xd0, // LLDT AX
        0x66, 0xb8, 0x18, 0x00, // MOV AX, 0x0018
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0018);
}

// LLDT with various selector values
#[test]
fn test_lldt_selector_0x0004() {
    let code = [
        0x66, 0xb8, 0x04, 0x00, // MOV AX, 0x0004
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with selector 0x000C
#[test]
fn test_lldt_selector_0x000c() {
    let code = [
        0x66, 0xb8, 0x0C, 0x00, // MOV AX, 0x000C
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with selector 0x0014
#[test]
fn test_lldt_selector_0x0014() {
    let code = [
        0x66, 0xb8, 0x14, 0x00, // MOV AX, 0x0014
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with selector 0x001C
#[test]
fn test_lldt_selector_0x001c() {
    let code = [
        0x66, 0xb8, 0x1C, 0x00, // MOV AX, 0x001C
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with selector having RPL bits
#[test]
fn test_lldt_selector_with_rpl() {
    let code = [
        0x66, 0xb8, 0x0B, 0x00, // MOV AX, 0x000B (selector 8 with RPL=3)
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT with selector having TI bit set
#[test]
fn test_lldt_selector_ti_bit() {
    let code = [
        0x66, 0xb8, 0x0C, 0x00, // MOV AX, 0x000C (TI bit set)
        0x0f, 0x00, 0xd0, // LLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 via RBP with displacement
#[test]
fn test_lldt_rbp_displacement() {
    let code = [
        0x48, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x1F00
        0x0f, 0x00, 0x95, 0x00, 0x01, 0x00, 0x00, // LLDT [RBP + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0070);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 via R12
#[test]
fn test_lldt_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x00, 0x14, 0x24, // LLDT [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0078);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 via R13 with displacement
#[test]
fn test_lldt_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x00, 0x95, 0x00, 0x01, 0x00, 0x00, // LLDT [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0080);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 via R14
#[test]
fn test_lldt_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x00, 0x16, // LLDT [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0088);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LLDT m16 via R15
#[test]
fn test_lldt_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x00, 0x17, // LLDT [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0090);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
