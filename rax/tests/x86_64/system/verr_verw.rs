use rax::cpu::Registers;

use crate::common::{DATA_ADDR, run_until_hlt, setup_vm, write_mem_at_u16, zf_set};

// VERR - Verify a Segment for Reading
// Opcode: 0F 00 /4
// Verifies that a segment selector is readable at the current privilege level
// Sets ZF=1 if readable, ZF=0 if not

// VERW - Verify a Segment for Writing
// Opcode: 0F 00 /5
// Verifies that a segment selector is writable at the current privilege level
// Sets ZF=1 if writable, ZF=0 if not

// VERR r16 - Verify AX for reading
#[test]
fn test_verr_ax() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0008, "AX should be preserved");
    assert_eq!(regs.rip, 0x1000 + 8, "RIP should point past HLT");
}

// VERW r16 - Verify BX for writing
#[test]
fn test_verw_bx() {
    let code = [
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x0010);
}

// VERR r16 - Verify CX for reading
#[test]
fn test_verr_cx() {
    let code = [
        0x66, 0xb9, 0x18, 0x00, // MOV CX, 0x0018
        0x0f, 0x00, 0xe1, // VERR CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify DX for writing
#[test]
fn test_verw_dx() {
    let code = [
        0x66, 0xba, 0x20, 0x00, // MOV DX, 0x0020
        0x0f, 0x00, 0xea, // VERW DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify SI for reading
#[test]
fn test_verr_si() {
    let code = [
        0x66, 0xbe, 0x28, 0x00, // MOV SI, 0x0028
        0x0f, 0x00, 0xe6, // VERR SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify DI for writing
#[test]
fn test_verw_di() {
    let code = [
        0x66, 0xbf, 0x30, 0x00, // MOV DI, 0x0030
        0x0f, 0x00, 0xef, // VERW DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify BP for reading
#[test]
fn test_verr_bp() {
    let code = [
        0x66, 0xbd, 0x38, 0x00, // MOV BP, 0x0038
        0x0f, 0x00, 0xe5, // VERR BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify SP for writing
#[test]
fn test_verw_sp() {
    let code = [
        0x66, 0xbc, 0x40, 0x00, // MOV SP, 0x0040
        0x0f, 0x00, 0xec, // VERW SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify R8W for reading
#[test]
fn test_verr_r8w() {
    let code = [
        0x66, 0x41, 0xb8, 0x48, 0x00, // MOV R8W, 0x0048
        0x41, 0x0f, 0x00, 0xe0, // VERR R8W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify R9W for writing
#[test]
fn test_verw_r9w() {
    let code = [
        0x66, 0x41, 0xb9, 0x50, 0x00, // MOV R9W, 0x0050
        0x41, 0x0f, 0x00, 0xe9, // VERW R9W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify R10W for reading
#[test]
fn test_verr_r10w() {
    let code = [
        0x66, 0x41, 0xba, 0x58, 0x00, // MOV R10W, 0x0058
        0x41, 0x0f, 0x00, 0xe2, // VERR R10W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify R11W for writing
#[test]
fn test_verw_r11w() {
    let code = [
        0x66, 0x41, 0xbb, 0x60, 0x00, // MOV R11W, 0x0060
        0x41, 0x0f, 0x00, 0xeb, // VERW R11W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify R12W for reading
#[test]
fn test_verr_r12w() {
    let code = [
        0x66, 0x41, 0xbc, 0x68, 0x00, // MOV R12W, 0x0068
        0x41, 0x0f, 0x00, 0xe4, // VERR R12W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify R13W for writing
#[test]
fn test_verw_r13w() {
    let code = [
        0x66, 0x41, 0xbd, 0x70, 0x00, // MOV R13W, 0x0070
        0x41, 0x0f, 0x00, 0xed, // VERW R13W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR r16 - Verify R14W for reading
#[test]
fn test_verr_r14w() {
    let code = [
        0x66, 0x41, 0xbe, 0x78, 0x00, // MOV R14W, 0x0078
        0x41, 0x0f, 0x00, 0xe6, // VERR R14W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW r16 - Verify R15W for writing
#[test]
fn test_verw_r15w() {
    let code = [
        0x66, 0x41, 0xbf, 0x80, 0x00, // MOV R15W, 0x0080
        0x41, 0x0f, 0x00, 0xef, // VERW R15W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR m16 - Verify from memory
#[test]
fn test_verr_memory() {
    let code = [
        0x0f, 0x00, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00, // VERR [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0008);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// VERW m16 - Verify from memory
#[test]
fn test_verw_memory() {
    let code = [
        0x0f, 0x00, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // VERW [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0010);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// VERR m16 - Via RAX
#[test]
fn test_verr_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x00, 0x20, // VERR [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0018);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// VERW m16 - Via RBX
#[test]
fn test_verw_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x00, 0x2b, // VERW [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0020);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// VERR m16 - Via RCX
#[test]
fn test_verr_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x00, 0x21, // VERR [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0028);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// VERW m16 - Via RDX
#[test]
fn test_verw_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x00, 0x2a, // VERW [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0030);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// VERR m16 - Via RSI
#[test]
fn test_verr_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x00, 0x26, // VERR [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0038);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// VERW m16 - Via RDI
#[test]
fn test_verw_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x00, 0x2f, // VERW [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0040);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// VERR with displacement
#[test]
fn test_verr_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x00, 0xa0, 0x00, 0x01, 0x00, 0x00, // VERR [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0048);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// VERW with displacement
#[test]
fn test_verw_displacement() {
    let code = [
        0x48, 0xbb, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x1F00
        0x0f, 0x00, 0xab, 0x00, 0x01, 0x00, 0x00, // VERW [RBX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0050);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x1F00);
}

// VERR preserves source register
#[test]
fn test_verr_preserves_source() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x0008,
        "Source register should be preserved"
    );
}

// VERW preserves source register
#[test]
fn test_verw_preserves_source() {
    let code = [
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFF,
        0x0010,
        "Source register should be preserved"
    );
}

// VERR preserves other registers
#[test]
fn test_verr_preserves_registers() {
    let code = [
        0x48, 0xbb, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RBX, 0x1111111111111111
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RCX, 0x2222222222222222
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

// VERW preserves other registers
#[test]
fn test_verw_preserves_registers() {
    let code = [
        0x48, 0xb8, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, // MOV RAX, 0x3333333333333333
        0x48, 0xb9, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, // MOV RCX, 0x4444444444444444
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3333333333333333);
    assert_eq!(regs.rcx, 0x4444444444444444);
}

// VERR with null selector
#[test]
fn test_verr_null_selector() {
    let code = [
        0x66, 0xb8, 0x00, 0x00, // MOV AX, 0x0000
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Null selector should fail verification (ZF=0)
    assert!(!zf_set(regs.rflags), "Null selector should fail VERR");
}

// VERW with null selector
#[test]
fn test_verw_null_selector() {
    let code = [
        0x66, 0xbb, 0x00, 0x00, // MOV BX, 0x0000
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Null selector should fail verification (ZF=0)
    assert!(!zf_set(regs.rflags), "Null selector should fail VERW");
}

// VERR multiple times
#[test]
fn test_verr_multiple_times() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xe0, // VERR AX
        0x0f, 0x00, 0xe0, // VERR AX
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW multiple times
#[test]
fn test_verw_multiple_times() {
    let code = [
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xeb, // VERW BX
        0x0f, 0x00, 0xeb, // VERW BX
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR with different selectors
#[test]
fn test_verr_various_selectors() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xe0, // VERR AX
        0x66, 0xb8, 0x10, 0x00, // MOV AX, 0x0010
        0x0f, 0x00, 0xe0, // VERR AX
        0x66, 0xb8, 0x18, 0x00, // MOV AX, 0x0018
        0x0f, 0x00, 0xe0, // VERR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW with different selectors
#[test]
fn test_verw_various_selectors() {
    let code = [
        0x66, 0xbb, 0x10, 0x00, // MOV BX, 0x0010
        0x0f, 0x00, 0xeb, // VERW BX
        0x66, 0xbb, 0x18, 0x00, // MOV BX, 0x0018
        0x0f, 0x00, 0xeb, // VERW BX
        0x66, 0xbb, 0x20, 0x00, // MOV BX, 0x0020
        0x0f, 0x00, 0xeb, // VERW BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR via R8
#[test]
fn test_verr_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x00, 0x20, // VERR [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0058);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW via R9
#[test]
fn test_verw_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x00, 0x29, // VERW [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0060);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR via R10
#[test]
fn test_verr_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x00, 0x22, // VERR [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0068);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW via R11
#[test]
fn test_verw_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x00, 0x2b, // VERW [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0070);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR via R12
#[test]
fn test_verr_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x00, 0x24, 0x24, // VERR [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0078);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW via R13
#[test]
fn test_verw_r13_indirect() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x00, 0xad, 0x00, 0x01, 0x00, 0x00, // VERW [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0080);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERR via R14
#[test]
fn test_verr_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x00, 0x26, // VERR [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0088);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// VERW via R15
#[test]
fn test_verw_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x00, 0x2f, // VERW [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0090);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
