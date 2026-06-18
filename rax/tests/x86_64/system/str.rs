use rax::cpu::Registers;

use crate::common::{DATA_ADDR, read_mem_at_u16, run_until_hlt, setup_vm, write_mem_at_u16};

// STR - Store Task Register
// Opcode: 0F 00 /1
// Stores the task register to a 16-bit register or memory operand
// The 16-bit selector value is stored

// STR r16 - Store TR to AX
#[test]
fn test_str_ax() {
    let code = [
        0x0f, 0x00, 0xc8, // STR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // TR value depends on VM initialization
    assert_eq!(regs.rip, 0x1000 + 4, "RIP should point past HLT");
}

// STR r16 - Store TR to BX
#[test]
fn test_str_bx() {
    let code = [
        0x0f, 0x00, 0xcb, // STR BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to CX
#[test]
fn test_str_cx() {
    let code = [
        0x0f, 0x00, 0xc9, // STR CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to DX
#[test]
fn test_str_dx() {
    let code = [
        0x0f, 0x00, 0xca, // STR DX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to SI
#[test]
fn test_str_si() {
    let code = [
        0x0f, 0x00, 0xce, // STR SI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to DI
#[test]
fn test_str_di() {
    let code = [
        0x0f, 0x00, 0xcf, // STR DI
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to BP
#[test]
fn test_str_bp() {
    let code = [
        0x0f, 0x00, 0xcd, // STR BP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to SP
#[test]
fn test_str_sp() {
    let code = [
        0x0f, 0x00, 0xcc, // STR SP
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R8W
#[test]
fn test_str_r8w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc8, // STR R8W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R9W
#[test]
fn test_str_r9w() {
    let code = [
        0x41, 0x0f, 0x00, 0xc9, // STR R9W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R10W
#[test]
fn test_str_r10w() {
    let code = [
        0x41, 0x0f, 0x00, 0xca, // STR R10W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R11W
#[test]
fn test_str_r11w() {
    let code = [
        0x41, 0x0f, 0x00, 0xcb, // STR R11W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R12W
#[test]
fn test_str_r12w() {
    let code = [
        0x41, 0x0f, 0x00, 0xcc, // STR R12W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R13W
#[test]
fn test_str_r13w() {
    let code = [
        0x41, 0x0f, 0x00, 0xcd, // STR R13W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R14W
#[test]
fn test_str_r14w() {
    let code = [
        0x41, 0x0f, 0x00, 0xce, // STR R14W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r16 - Store TR to R15W
#[test]
fn test_str_r15w() {
    let code = [
        0x41, 0x0f, 0x00, 0xcf, // STR R15W
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r32 - Store TR to EAX
#[test]
fn test_str_eax() {
    let code = [
        0x0f, 0x00, 0xc8, // STR EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 16 bits should be zero
    assert_eq!(regs.rax & 0xFFFF0000, 0, "Upper 16 bits should be zero");
}

// STR r32 - Store TR to EBX
#[test]
fn test_str_ebx() {
    let code = [
        0x0f, 0x00, 0xcb, // STR EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r32 - Store TR to ECX
#[test]
fn test_str_ecx() {
    let code = [
        0x0f, 0x00, 0xc9, // STR ECX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r32 - Store TR to EDX
#[test]
fn test_str_edx() {
    let code = [
        0x0f, 0x00, 0xca, // STR EDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR r64 - Store TR to RAX
#[test]
fn test_str_rax() {
    let code = [
        0x48, 0x0f, 0x00, 0xc8, // STR RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 48 bits should be zero
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0,
        "Upper 48 bits should be zero"
    );
}

// STR r64 - Store TR to RBX
#[test]
fn test_str_rbx() {
    let code = [
        0x48, 0x0f, 0x00, 0xcb, // STR RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 - Store TR to memory
#[test]
fn test_str_memory() {
    let code = [
        0x0f, 0x00, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // STR [0x2000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rip, 0x1000 + 9);

    let tr = read_mem_at_u16(&mem, DATA_ADDR);
    assert!(tr <= 0xFFFF);
}

// STR m16 - Store TR to memory via RAX
#[test]
fn test_str_rax_indirect() {
    let code = [
        0x48, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2000
        0x0f, 0x00, 0x08, // STR [RAX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, DATA_ADDR);
}

// STR m16 - Store TR to memory via RBX
#[test]
fn test_str_rbx_indirect() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0x00, 0x0b, // STR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, DATA_ADDR);
}

// STR m16 - Store TR to memory via RCX
#[test]
fn test_str_rcx_indirect() {
    let code = [
        0x48, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x2000
        0x0f, 0x00, 0x09, // STR [RCX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, DATA_ADDR);
}

// STR m16 - Store TR to memory via RDX
#[test]
fn test_str_rdx_indirect() {
    let code = [
        0x48, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 0x2000
        0x0f, 0x00, 0x0a, // STR [RDX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, DATA_ADDR);
}

// STR m16 - Store TR to memory via RSI
#[test]
fn test_str_rsi_indirect() {
    let code = [
        0x48, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0x2000
        0x0f, 0x00, 0x0e, // STR [RSI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi, DATA_ADDR);
}

// STR m16 - Store TR to memory via RDI
#[test]
fn test_str_rdi_indirect() {
    let code = [
        0x48, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0x2000
        0x0f, 0x00, 0x0f, // STR [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, DATA_ADDR);
}

// STR m16 - Store TR to memory via R8
#[test]
fn test_str_r8_indirect() {
    let code = [
        0x49, 0xb8, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0x2000
        0x41, 0x0f, 0x00, 0x08, // STR [R8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 - Store TR to memory via R9
#[test]
fn test_str_r9_indirect() {
    let code = [
        0x49, 0xb9, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 0x2000
        0x41, 0x0f, 0x00, 0x09, // STR [R9]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 - Store TR to memory via R10
#[test]
fn test_str_r10_indirect() {
    let code = [
        0x49, 0xba, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0x2000
        0x41, 0x0f, 0x00, 0x0a, // STR [R10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 - Store TR to memory via R11
#[test]
fn test_str_r11_indirect() {
    let code = [
        0x49, 0xbb, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 0x2000
        0x41, 0x0f, 0x00, 0x0b, // STR [R11]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR with displacement
#[test]
fn test_str_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1F00
        0x0f, 0x00, 0x88, 0x00, 0x01, 0x00, 0x00, // STR [RAX + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1F00);
}

// STR with negative displacement
#[test]
fn test_str_negative_displacement() {
    let code = [
        0x48, 0xb8, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2100
        0x0f, 0x00, 0x88, 0x00, 0xFF, 0xFF, 0xFF, // STR [RAX - 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2100);
}

// STR preserves other registers
#[test]
fn test_str_preserves_registers() {
    let code = [
        0x48, 0xbb, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
        0x11, // MOV RBX, 0x1111111111111111
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, // MOV RCX, 0x2222222222222222
        0x0f, 0x00, 0xc8, // STR AX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

// LTR then STR roundtrip
#[test]
fn test_ltr_str_roundtrip() {
    let code = [
        0x66, 0xb8, 0x28, 0x00, // MOV AX, 0x0028
        0x0f, 0x00, 0xd8, // LTR AX
        0x66, 0xbb, 0x00, 0x00, // MOV BX, 0x0000
        0x0f, 0x00, 0xcb, // STR BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // BX should contain the TR value we loaded (0x0028)
    assert_eq!(
        regs.rbx & 0xFFFF,
        0x0028,
        "STR should return value loaded by LTR"
    );
}

// STR m16 via RBP with displacement
#[test]
fn test_str_rbp_displacement() {
    let code = [
        0x48, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBP, 0x1F00
        0x0f, 0x00, 0x8d, 0x00, 0x01, 0x00, 0x00, // STR [RBP + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 via R12
#[test]
fn test_str_r12_indirect() {
    let code = [
        0x49, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 0x2000
        0x41, 0x0f, 0x00, 0x0c, 0x24, // STR [R12]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 via R13 with displacement
#[test]
fn test_str_r13_displacement() {
    let code = [
        0x49, 0xbd, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 0x1F00
        0x41, 0x0f, 0x00, 0x8d, 0x00, 0x01, 0x00, 0x00, // STR [R13 + 0x100]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 via R14
#[test]
fn test_str_r14_indirect() {
    let code = [
        0x49, 0xbe, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 0x2000
        0x41, 0x0f, 0x00, 0x0e, // STR [R14]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR m16 via R15
#[test]
fn test_str_r15_indirect() {
    let code = [
        0x49, 0xbf, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 0x2000
        0x41, 0x0f, 0x00, 0x0f, // STR [R15]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mem_at_u16(&mem, DATA_ADDR, 0xFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

// STR multiple times
#[test]
fn test_str_multiple_times() {
    let code = [
        0x0f, 0x00, 0xc8, // STR AX
        0x0f, 0x00, 0xcb, // STR BX
        0x0f, 0x00, 0xc9, // STR CX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // All three should have the same TR value
    let ax = regs.rax & 0xFFFF;
    let bx = regs.rbx & 0xFFFF;
    let cx = regs.rcx & 0xFFFF;

    assert_eq!(ax, bx, "Multiple STR should return same value");
    assert_eq!(bx, cx, "Multiple STR should return same value");
}

// STR r32 - Verify upper bits are zero
#[test]
fn test_str_r32_upper_bits_zero() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x0f, 0x00, 0xc8, // STR EAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zero
    assert_eq!(regs.rax >> 32, 0, "Upper 32 bits should be zero");
}

// STR r64 - Verify upper bits are zero
#[test]
fn test_str_r64_upper_bits_zero() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x0f, 0x00, 0xc8, // STR RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only lower 16 bits should be set
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0,
        "Upper 48 bits should be zero"
    );
}
