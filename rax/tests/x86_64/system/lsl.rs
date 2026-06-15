use rax::cpu::Registers;

use crate::common::{DATA_ADDR, run_until_hlt, setup_vm, write_mem_at_u16, zf_set};

// LSL - Load Segment Limit
// Opcode: 0F 03 /r
// Loads the segment limit of a segment selector into a register
// Sets ZF=1 if successful, ZF=0 if the selector is invalid
// The segment limit is loaded into the destination register

// LSL r16, r16 - Load segment limit from AX to BX
#[test]
fn test_lsl_bx_ax() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x66, 0x0f, 0x03, 0xd8, // LSL BX, AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0008, "AX should be preserved");
    assert_eq!(regs.rip, 0x1000 + 9, "RIP should point past HLT");
}

// LSL r16, r16 - Load segment limit from CX to DX
#[test]
fn test_lsl_dx_cx() {
    let code = [
        0x66, 0xb9, 0x10, 0x00, // MOV CX, 0x0010
        0x66, 0x0f, 0x03, 0xd1, // LSL DX, CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x0010);
}

// LSL r32, r32 - Load segment limit from EAX to EBX
#[test]
fn test_lsl_ebx_eax() {
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 0x00000008
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000008);
}

// LSL r32, r32 - Load segment limit from ECX to EDX
#[test]
fn test_lsl_edx_ecx() {
    let code = [
        0xb9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 0x00000010
        0x0f, 0x03, 0xd1, // LSL EDX, ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x00000010);
}

// LSL r64, r64 - Load segment limit from RAX to RBX
#[test]
fn test_lsl_rbx_rax() {
    let code = [
        0x48, 0xb8, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x0008
        0x48, 0x0f, 0x03, 0xd8, // LSL RBX, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0008);
}

// LSL r64, r64 - Load segment limit from RCX to RDX
#[test]
fn test_lsl_rdx_rcx() {
    let code = [
        0x48, 0xb9, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x0010
        0x48, 0x0f, 0x03, 0xd1, // LSL RDX, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x0010);
}

// LSL r32, m16 - Load segment limit from memory to EAX
#[test]
fn test_lsl_eax_memory() {
    let code = [
        0x0f, 0x03, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // LSL EAX, [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0008);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);
}

// LSL r32, m16 - Load segment limit via RAX
#[test]
fn test_lsl_ebx_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x03, 0x18, // LSL EBX, [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0010);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// LSL r32, r32 - All GP registers as destination
#[test]
fn test_lsl_eax_ebx() {
    let code = [
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 0x0008
        0x0f, 0x03, 0xc3, // LSL EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r32, r32 - ECX to EAX
#[test]
fn test_lsl_ecx_ebx() {
    let code = [
        0xbb, 0x10, 0x00, 0x00, 0x00, // MOV EBX, 0x0010
        0x0f, 0x03, 0xcb, // LSL ECX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r32, r32 - ESI to EDX
#[test]
fn test_lsl_esi_edx() {
    let code = [
        0xba, 0x18, 0x00, 0x00, 0x00, // MOV EDX, 0x0018
        0x0f, 0x03, 0xf2, // LSL ESI, EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r32, r32 - EDI to ESI
#[test]
fn test_lsl_edi_esi() {
    let code = [
        0xbe, 0x20, 0x00, 0x00, 0x00, // MOV ESI, 0x0020
        0x0f, 0x03, 0xfe, // LSL EDI, ESI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - RSI to RDI
#[test]
fn test_lsl_rsi_rdi() {
    let code = [
        0x48, 0xbf, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x0028
        0x48, 0x0f, 0x03, 0xf7, // LSL RSI, RDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - RBP to RBX
#[test]
fn test_lsl_rbp_rbx() {
    let code = [
        0x48, 0xbb, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x0030
        0x48, 0x0f, 0x03, 0xeb, // LSL RBP, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R8 to RAX
#[test]
fn test_lsl_r8_rax() {
    let code = [
        0x48, 0xb8, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x0038
        0x4c, 0x0f, 0x03, 0xc0, // LSL R8, RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R9 to RBX
#[test]
fn test_lsl_r9_rbx() {
    let code = [
        0x48, 0xbb, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x0040
        0x4c, 0x0f, 0x03, 0xcb, // LSL R9, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R10 to RCX
#[test]
fn test_lsl_r10_rcx() {
    let code = [
        0x48, 0xb9, 0x48, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x0048
        0x4c, 0x0f, 0x03, 0xd1, // LSL R10, RCX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R11 to RDX
#[test]
fn test_lsl_r11_rdx() {
    let code = [
        0x48, 0xba, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x0050
        0x4c, 0x0f, 0x03, 0xda, // LSL R11, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R12 to RSI
#[test]
fn test_lsl_r12_rsi() {
    let code = [
        0x48, 0xbe, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x0058
        0x4c, 0x0f, 0x03, 0xe6, // LSL R12, RSI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R13 to RDI
#[test]
fn test_lsl_r13_rdi() {
    let code = [
        0x48, 0xbf, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x0060
        0x4c, 0x0f, 0x03, 0xef, // LSL R13, RDI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R14 to R8
#[test]
fn test_lsl_r14_r8() {
    let code = [
        0x49, 0xb8, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x0068
        0x4d, 0x0f, 0x03, 0xf0, // LSL R14, R8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL r64, r64 - R15 to R9
#[test]
fn test_lsl_r15_r9() {
    let code = [
        0x49, 0xb9, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x0070
        0x4d, 0x0f, 0x03, 0xf9, // LSL R15, R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL from memory via RBX
#[test]
fn test_lsl_eax_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x03, 0x03, // LSL EAX, [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0018);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// LSL from memory via RCX
#[test]
fn test_lsl_edx_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x03, 0x11, // LSL EDX, [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0020);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// LSL from memory via RDX
#[test]
fn test_lsl_esi_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x03, 0x32, // LSL ESI, [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0028);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// LSL from memory via RSI
#[test]
fn test_lsl_edi_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x03, 0x3e, // LSL EDI, [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0030);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// LSL from memory via RDI
#[test]
fn test_lsl_ebx_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x03, 0x1f, // LSL EBX, [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0038);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// LSL from memory with displacement
#[test]
fn test_lsl_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x03, 0x98, 0x00, 0x01, 0x00, 0x00, // LSL EBX, [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0040);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// LSL preserves source register
#[test]
fn test_lsl_preserves_source() {
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 0x00000008
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000008,
        "Source should be preserved"
    );
}

// LSL preserves other registers
#[test]
fn test_lsl_preserves_registers() {
    let code = [
        0x48, 0xb9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RCX, 0x1111111111111111
        0x48, 0xba, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RDX, 0x2222222222222222
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 0x0008
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x1111111111111111);
    assert_eq!(regs.rdx, 0x2222222222222222);
}

// LSL with null selector
#[test]
fn test_lsl_null_selector() {
    let code = [
        0x66, 0xb8, 0x00, 0x00, // MOV AX, 0x0000
        0x66, 0x0f, 0x03, 0xd8, // LSL BX, AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Null selector should fail (ZF=0)
    assert!(!zf_set(regs.rflags), "Null selector should fail LSL");
}

// LSL multiple times
#[test]
fn test_lsl_multiple_times() {
    let code = [
        0xb8, 0x08, 0x00, 0x00, 0x00, // MOV EAX, 0x0008
        0x0f, 0x03, 0xd8, // LSL EBX, EAX
        0x0f, 0x03, 0xc8, // LSL ECX, EAX
        0x0f, 0x03, 0xd0, // LSL EDX, EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R8
#[test]
fn test_lsl_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x03, 0x00, // LSL EAX, [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0048);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R9
#[test]
fn test_lsl_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x03, 0x19, // LSL EBX, [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0050);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R10
#[test]
fn test_lsl_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x03, 0x0a, // LSL ECX, [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0058);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R11
#[test]
fn test_lsl_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x03, 0x13, // LSL EDX, [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0060);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R12
#[test]
fn test_lsl_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x03, 0x04, 0x24, // LSL EAX, [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0068);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R13 with displacement
#[test]
fn test_lsl_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x03, 0x85, 0x00, 0x01, 0x00, 0x00, // LSL EAX, [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0070);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R14
#[test]
fn test_lsl_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x03, 0x1e, // LSL EBX, [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0078);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// LSL via R15
#[test]
fn test_lsl_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x03, 0x0f, // LSL ECX, [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0x0080);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}
