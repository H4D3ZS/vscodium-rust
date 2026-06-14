use rax::cpu::Registers;

use crate::common::{DATA_ADDR, run_until_hlt, setup_vm, write_mem_at_u16};

// LTR - Load Task Register
// Opcode: 0F 00 /3
// Loads the task register from a 16-bit segment selector
// The selector is loaded from a register or memory operand

// LTR r16 - Load TR from AX
#[test]
fn test_ltr_ax() {
    let code = [
        0x66, 0xb8, 0x28, 0x00, // MOV AX, 0x0028
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0028, "AX should be preserved");
    assert_eq!(regs.rip, 0x1000 + 8, "RIP should point past HLT");
}

// LTR r16 - Load TR from BX
#[test]
fn test_ltr_bx() {
    let code = [
        0x66, 0xbb, 0x30, 0x00, // MOV BX, 0x0030
        0x0f, 0x00, 0xdb, // LTR BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x0030);
}

// LTR r16 - Load TR from CX
#[test]
fn test_ltr_cx() {
    let code = [
        0x66, 0xb9, 0x38, 0x00, // MOV CX, 0x0038
        0x0f, 0x00, 0xd9, // LTR CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x0038);
}

// LTR r16 - Load TR from DX
#[test]
fn test_ltr_dx() {
    let code = [
        0x66, 0xba, 0x40, 0x00, // MOV DX, 0x0040
        0x0f, 0x00, 0xda, // LTR DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdx & 0xFFFF, 0x0040);
}

// LTR r16 - Load TR from SI
#[test]
fn test_ltr_si() {
    let code = [
        0x66, 0xbe, 0x48, 0x00, // MOV SI, 0x0048
        0x0f, 0x00, 0xde, // LTR SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0x0048);
}

// LTR r16 - Load TR from DI
#[test]
fn test_ltr_di() {
    let code = [
        0x66, 0xbf, 0x50, 0x00, // MOV DI, 0x0050
        0x0f, 0x00, 0xdf, // LTR DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi & 0xFFFF, 0x0050);
}

// LTR r16 - Load TR from BP
#[test]
fn test_ltr_bp() {
    let code = [
        0x66, 0xbd, 0x58, 0x00, // MOV BP, 0x0058
        0x0f, 0x00, 0xdd, // LTR BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbp & 0xFFFF, 0x0058);
}

// LTR r16 - Load TR from SP
#[test]
fn test_ltr_sp() {
    let code = [
        0x66, 0xbc, 0x60, 0x00, // MOV SP, 0x0060
        0x0f, 0x00, 0xdc, // LTR SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R8W
#[test]
fn test_ltr_r8w() {
    let code = [
        0x66, 0x41, 0xb8, 0x68, 0x00, // MOV R8W, 0x0068
        0x41, 0x0f, 0x00, 0xd8, // LTR R8W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R9W
#[test]
fn test_ltr_r9w() {
    let code = [
        0x66, 0x41, 0xb9, 0x70, 0x00, // MOV R9W, 0x0070
        0x41, 0x0f, 0x00, 0xd9, // LTR R9W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R10W
#[test]
fn test_ltr_r10w() {
    let code = [
        0x66, 0x41, 0xba, 0x78, 0x00, // MOV R10W, 0x0078
        0x41, 0x0f, 0x00, 0xda, // LTR R10W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R11W
#[test]
fn test_ltr_r11w() {
    let code = [
        0x66, 0x41, 0xbb, 0x80, 0x00, // MOV R11W, 0x0080
        0x41, 0x0f, 0x00, 0xdb, // LTR R11W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R12W
#[test]
fn test_ltr_r12w() {
    let code = [
        0x66, 0x41, 0xbc, 0x88, 0x00, // MOV R12W, 0x0088
        0x41, 0x0f, 0x00, 0xdc, // LTR R12W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R13W
#[test]
fn test_ltr_r13w() {
    let code = [
        0x66, 0x41, 0xbd, 0x90, 0x00, // MOV R13W, 0x0090
        0x41, 0x0f, 0x00, 0xdd, // LTR R13W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R14W
#[test]
fn test_ltr_r14w() {
    let code = [
        0x66, 0x41, 0xbe, 0x98, 0x00, // MOV R14W, 0x0098
        0x41, 0x0f, 0x00, 0xde, // LTR R14W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR r16 - Load TR from R15W
#[test]
fn test_ltr_r15w() {
    let code = [
        0x66, 0x41, 0xbf, 0xA0, 0x00, // MOV R15W, 0x00A0
        0x41, 0x0f, 0x00, 0xdf, // LTR R15W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 - Load TR from memory
#[test]
fn test_ltr_memory() {
    let code = [
        0x0f, 0x00, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // LTR [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0028);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LTR m16 - Load TR from memory via RAX
#[test]
fn test_ltr_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x00, 0x18, // LTR [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0030);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// LTR m16 - Load TR from memory via RBX
#[test]
fn test_ltr_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x00, 0x1b, // LTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0038);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// LTR m16 - Load TR from memory via RCX
#[test]
fn test_ltr_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x00, 0x19, // LTR [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0040);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// LTR m16 - Load TR from memory via RDX
#[test]
fn test_ltr_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x00, 0x1a, // LTR [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0048);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// LTR m16 - Load TR from memory via RSI
#[test]
fn test_ltr_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x00, 0x1e, // LTR [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0050);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// LTR m16 - Load TR from memory via RDI
#[test]
fn test_ltr_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x00, 0x1f, // LTR [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0058);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// LTR m16 - Load TR from memory via R8
#[test]
fn test_ltr_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x00, 0x18, // LTR [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0060);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 - Load TR from memory via R9
#[test]
fn test_ltr_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x00, 0x19, // LTR [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0068);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 - Load TR from memory via R10
#[test]
fn test_ltr_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x00, 0x1a, // LTR [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0070);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 - Load TR from memory via R11
#[test]
fn test_ltr_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x00, 0x1b, // LTR [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0078);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR with displacement
#[test]
fn test_ltr_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x00, 0x98, 0x00, 0x01, 0x00, 0x00, // LTR [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0080);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// LTR with negative displacement
#[test]
fn test_ltr_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x00, 0x98, 0x00, 0xFF, 0xFF, 0xFF, // LTR [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0088);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// LTR preserves other registers
#[test]
fn test_ltr_preserves_registers() {
    let code = [
        0x48, 0xbb, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RBX, 0x1111111111111111
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RCX, 0x2222222222222222
        0x66, 0xb8, 0x28, 0x00, // MOV AX, 0x0028
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

// LTR multiple times
#[test]
fn test_ltr_multiple_times() {
    let code = [
        0x66, 0xb8, 0x28, 0x00, // MOV AX, 0x0028
        0x0f, 0x00, 0xd8, // LTR AX
        0x66, 0xb8, 0x30, 0x00, // MOV AX, 0x0030
        0x0f, 0x00, 0xd8, // LTR AX
        0x66, 0xb8, 0x38, 0x00, // MOV AX, 0x0038
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0038);
}

// LTR with various selector values
#[test]
fn test_ltr_selector_0x0020() {
    let code = [
        0x66, 0xb8, 0x20, 0x00, // MOV AX, 0x0020
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR with selector 0x0028
#[test]
fn test_ltr_selector_0x0028() {
    let code = [
        0x66, 0xb8, 0x28, 0x00, // MOV AX, 0x0028
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR with selector 0x0030
#[test]
fn test_ltr_selector_0x0030() {
    let code = [
        0x66, 0xb8, 0x30, 0x00, // MOV AX, 0x0030
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR with selector 0x0038
#[test]
fn test_ltr_selector_0x0038() {
    let code = [
        0x66, 0xb8, 0x38, 0x00, // MOV AX, 0x0038
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR with selector having RPL bits
#[test]
fn test_ltr_selector_with_rpl() {
    let code = [
        0x66, 0xb8, 0x2B, 0x00, // MOV AX, 0x002B (selector 0x28 with RPL=3)
        0x0f, 0x00, 0xd8, // LTR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 via RBP with displacement
#[test]
fn test_ltr_rbp_displacement() {
    let code = [
        0x48, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x1F00
        0x0f, 0x00, 0x9d, 0x00, 0x01, 0x00, 0x00, // LTR [RBP + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0090);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 via R12
#[test]
fn test_ltr_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x00, 0x1c, 0x24, // LTR [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0098);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 via R13 with displacement
#[test]
fn test_ltr_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x00, 0x9d, 0x00, 0x01, 0x00, 0x00, // LTR [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x00A0);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 via R14
#[test]
fn test_ltr_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x00, 0x1e, // LTR [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x00A8);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LTR m16 via R15
#[test]
fn test_ltr_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x00, 0x1f, // LTR [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x00B0);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
