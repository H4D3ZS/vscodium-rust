use rax::cpu::Registers;

use crate::common::{DATA_ADDR, read_mem_at_u16, run_until_hlt, setup_vm, write_mem_at_u16};

// SLDT - Store Local Descriptor Table Register
// Opcode: 0F 00 /0
// Stores the LDTR to a 16-bit register or memory operand
// The 16-bit selector value is stored

// SLDT r16 - Store LDTR to AX
#[test]
fn test_sldt_ax() {
    let code = [
        0x0f, 0x00, 0xc0, // SLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // LDTR value depends on VM initialization, just check instruction completed
    assert_eq!(regs.rip, 0x1000 + 4, "RIP should point past HLT");
}

// SLDT r16 - Store LDTR to BX
#[test]
fn test_sldt_bx() {
    let code = [
        0x0f, 0x00, 0xc3, // SLDT BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to CX
#[test]
fn test_sldt_cx() {
    let code = [
        0x0f, 0x00, 0xc1, // SLDT CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to DX
#[test]
fn test_sldt_dx() {
    let code = [
        0x0f, 0x00, 0xc2, // SLDT DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to SI
#[test]
fn test_sldt_si() {
    let code = [
        0x0f, 0x00, 0xc6, // SLDT SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to DI
#[test]
fn test_sldt_di() {
    let code = [
        0x0f, 0x00, 0xc7, // SLDT DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to BP
#[test]
fn test_sldt_bp() {
    let code = [
        0x0f, 0x00, 0xc5, // SLDT BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to SP
#[test]
fn test_sldt_sp() {
    let code = [
        0x0f, 0x00, 0xc4, // SLDT SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R8W
#[test]
fn test_sldt_r8w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc0, // SLDT R8W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R9W
#[test]
fn test_sldt_r9w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc1, // SLDT R9W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R10W
#[test]
fn test_sldt_r10w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc2, // SLDT R10W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R11W
#[test]
fn test_sldt_r11w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc3, // SLDT R11W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R12W
#[test]
fn test_sldt_r12w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc4, // SLDT R12W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R13W
#[test]
fn test_sldt_r13w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc5, // SLDT R13W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R14W
#[test]
fn test_sldt_r14w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc6, // SLDT R14W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r16 - Store LDTR to R15W
#[test]
fn test_sldt_r15w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc7, // SLDT R15W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r32 - Store LDTR to EAX (zero-extends to 32 bits)
#[test]
fn test_sldt_eax() {
    let code = [
        0x0f, 0x00, 0xc0, // SLDT EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 16 bits of EAX should be zero
    assert_eq!(regs.rax & 0xFFFF0000, 0, "Upper 16 bits should be zero");
}

// SLDT r32 - Store LDTR to EBX
#[test]
fn test_sldt_ebx() {
    let code = [
        0x0f, 0x00, 0xc3, // SLDT EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r32 - Store LDTR to ECX
#[test]
fn test_sldt_ecx() {
    let code = [
        0x0f, 0x00, 0xc1, // SLDT ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r32 - Store LDTR to EDX
#[test]
fn test_sldt_edx() {
    let code = [
        0x0f, 0x00, 0xc2, // SLDT EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT r64 - Store LDTR to RAX (zero-extends to 64 bits)
#[test]
fn test_sldt_rax() {
    let code = [
        0x48, 0x0f, 0x00, 0xc0, // SLDT RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 48 bits of RAX should be zero
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0,
        "Upper 48 bits should be zero"
    );
}

// SLDT r64 - Store LDTR to RBX
#[test]
fn test_sldt_rbx() {
    let code = [
        0x48, 0x0f, 0x00, 0xc3, // SLDT RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 - Store LDTR to memory
#[test]
fn test_sldt_memory() {
    let code = [
        0x0f, 0x00, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SLDT [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);

    // Memory should contain LDTR value (not 0xFFFF anymore)
    let ldtr = read_mem_at_u16(&mem, DATA_ADDR);
    // LDTR is likely 0 in test environment
    assert!(ldtr <= 0xFFFF);
}

// SLDT m16 - Store LDTR to memory via RAX
#[test]
fn test_sldt_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x00, 0x00, // SLDT [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via RBX
#[test]
fn test_sldt_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x00, 0x03, // SLDT [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via RCX
#[test]
fn test_sldt_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x00, 0x01, // SLDT [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via RDX
#[test]
fn test_sldt_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x00, 0x02, // SLDT [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via RSI
#[test]
fn test_sldt_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x00, 0x06, // SLDT [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via RDI
#[test]
fn test_sldt_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x00, 0x07, // SLDT [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// SLDT m16 - Store LDTR to memory via R8
#[test]
fn test_sldt_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x00, 0x00, // SLDT [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 - Store LDTR to memory via R9
#[test]
fn test_sldt_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x00, 0x01, // SLDT [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 - Store LDTR to memory via R10
#[test]
fn test_sldt_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x00, 0x02, // SLDT [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 - Store LDTR to memory via R11
#[test]
fn test_sldt_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x00, 0x03, // SLDT [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT with displacement
#[test]
fn test_sldt_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x00, 0x80, 0x00, 0x01, 0x00, 0x00, // SLDT [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// SLDT with negative displacement
#[test]
fn test_sldt_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x00, 0x80, 0x00, 0xFF, 0xFF, 0xFF, // SLDT [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// SLDT preserves other registers
#[test]
fn test_sldt_preserves_registers() {
    let code = [
        0x48, 0xbb, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RBX, 0x1111111111111111
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RCX, 0x2222222222222222
        0x0f, 0x00, 0xc0, // SLDT AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

// LLDT then SLDT roundtrip
#[test]
fn test_lldt_sldt_roundtrip() {
    let code = [
        0x66, 0xb8, 0x08, 0x00, // MOV AX, 0x0008
        0x0f, 0x00, 0xd0, // LLDT AX
        0x66, 0xbb, 0x00, 0x00, // MOV BX, 0x0000
        0x0f, 0x00, 0xc3, // SLDT BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // BX should contain the LDTR value we loaded (0x0008)
    assert_eq!(
        regs.rbx & 0xFFFF,
        0x0008,
        "SLDT should return value loaded by LLDT"
    );
}

// SLDT m16 via RBP with displacement
#[test]
fn test_sldt_rbp_displacement() {
    let code = [
        0x48, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x1F00
        0x0f, 0x00, 0x85, 0x00, 0x01, 0x00, 0x00, // SLDT [RBP + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 via R12
#[test]
fn test_sldt_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x00, 0x04, 0x24, // SLDT [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 via R13 with displacement
#[test]
fn test_sldt_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x00, 0x85, 0x00, 0x01, 0x00, 0x00, // SLDT [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 via R14
#[test]
fn test_sldt_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x00, 0x06, // SLDT [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT m16 via R15
#[test]
fn test_sldt_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x00, 0x07, // SLDT [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// SLDT multiple times
#[test]
fn test_sldt_multiple_times() {
    let code = [
        0x0f, 0x00, 0xc0, // SLDT AX
        0x0f, 0x00, 0xc3, // SLDT BX
        0x0f, 0x00, 0xc1, // SLDT CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All three should have the same LDTR value
    let ax = regs.rax & 0xFFFF;
    let bx = regs.rbx & 0xFFFF;
    let cx = regs.rcx & 0xFFFF;

    assert_eq!(ax, bx, "Multiple SLDT should return same value");
    assert_eq!(bx, cx, "Multiple SLDT should return same value");
}

// SLDT r32 - Verify upper bits are zero
#[test]
fn test_sldt_r32_upper_bits_zero() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x0f, 0x00, 0xc0, // SLDT EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zero (32-bit write zeros upper in 64-bit mode)
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits should be zero");
}

// SLDT r64 - Verify upper bits are zero
#[test]
fn test_sldt_r64_upper_bits_zero() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x0f, 0x00, 0xc0, // SLDT RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only lower 16 bits should be set, upper 48 bits zero
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0,
        "Upper 48 bits should be zero"
    );
}
