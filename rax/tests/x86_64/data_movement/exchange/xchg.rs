use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// XCHG - Exchange Register/Memory with Register
// Swaps the values of two operands

// Basic register-to-register exchange (32-bit)
#[test]
fn test_xchg_eax_ebx() {
    let code = [0x87, 0xd8, 0xf4]; // XCHG EAX, EBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x22222222,
        "EAX should have EBX's value"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x11111111,
        "EBX should have EAX's value"
    );
}

// Short form XCHG with RAX/EAX
#[test]
fn test_xchg_eax_ecx_short_form() {
    let code = [0x91, 0xf4]; // XCHG EAX, ECX (short form), HLT
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAA;
    regs.rcx = 0xBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xBBBBBBBB,
        "EAX should have ECX's value"
    );
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xAAAAAAAA,
        "ECX should have EAX's value"
    );
}

// 16-bit exchange
#[test]
fn test_xchg_ax_bx() {
    let code = [0x66, 0x93, 0xf4]; // XCHG AX, BX, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF0000AAAA;
    regs.rbx = 0xFFFFFFFF0000BBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xBBBB, "AX should have BX's value");
    assert_eq!(regs.rbx & 0xFFFF, 0xAAAA, "BX should have AX's value");
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFF0000,
        0xFFFFFFFF00000000,
        "Upper bits of RAX preserved"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFFFFFF0000,
        0xFFFFFFFF00000000,
        "Upper bits of RBX preserved"
    );
}

// 64-bit exchange
#[test]
fn test_xchg_rax_rbx() {
    let code = [0x48, 0x87, 0xd8, 0xf4]; // XCHG RAX, RBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should have RBX's value");
    assert_eq!(regs.rbx, 0x1111111111111111, "RBX should have RAX's value");
}

// Test that 32-bit XCHG zeros upper 32 bits
#[test]
fn test_xchg_eax_ebx_zeros_upper() {
    let code = [0x87, 0xd8, 0xf4]; // XCHG EAX, EBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF11111111;
    regs.rbx = 0xCAFEBABE22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000022222222,
        "RAX upper bits should be zeroed"
    );
    assert_eq!(
        regs.rbx, 0x0000000011111111,
        "RBX upper bits should be zeroed"
    );
}

// Exchange with different registers
#[test]
fn test_xchg_ecx_edx() {
    let code = [0x87, 0xd1, 0xf4]; // XCHG ECX, EDX, HLT
    let mut regs = Registers::default();
    regs.rcx = 0x33333333;
    regs.rdx = 0x44444444;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x44444444,
        "ECX should have EDX's value"
    );
    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0x33333333,
        "EDX should have ECX's value"
    );
}

#[test]
fn test_xchg_esi_edi() {
    let code = [0x87, 0xfe, 0xf4]; // XCHG ESI, EDI, HLT
    let mut regs = Registers::default();
    regs.rsi = 0x55555555;
    regs.rdi = 0x66666666;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rsi & 0xFFFFFFFF,
        0x66666666,
        "ESI should have EDI's value"
    );
    assert_eq!(
        regs.rdi & 0xFFFFFFFF,
        0x55555555,
        "EDI should have ESI's value"
    );
}

// Test short form with all general purpose registers
#[test]
fn test_xchg_rax_rcx_short() {
    let code = [0x48, 0x91, 0xf4]; // XCHG RAX, RCX, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should have RCX's value");
    assert_eq!(regs.rcx, 0x1111111111111111, "RCX should have RAX's value");
}

#[test]
fn test_xchg_rax_rdx_short() {
    let code = [0x48, 0x92, 0xf4]; // XCHG RAX, RDX, HLT
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.rdx = 0xBBBBBBBBBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xBBBBBBBBBBBBBBBB, "RAX should have RDX's value");
    assert_eq!(regs.rdx, 0xAAAAAAAAAAAAAAAA, "RDX should have RAX's value");
}

#[test]
fn test_xchg_rax_rsi_short() {
    let code = [0x48, 0x96, 0xf4]; // XCHG RAX, RSI, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    regs.rsi = 0xFEDCBA0987654321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDCBA0987654321, "RAX should have RSI's value");
    assert_eq!(regs.rsi, 0x1234567890ABCDEF, "RSI should have RAX's value");
}

#[test]
fn test_xchg_rax_rdi_short() {
    let code = [0x48, 0x97, 0xf4]; // XCHG RAX, RDI, HLT
    let mut regs = Registers::default();
    regs.rax = 0x0011223344556677;
    regs.rdi = 0x8899AABBCCDDEEFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8899AABBCCDDEEFF, "RAX should have RDI's value");
    assert_eq!(regs.rdi, 0x0011223344556677, "RDI should have RAX's value");
}

// Test with extended registers (R8-R15)
#[test]
fn test_xchg_r8_r9() {
    let code = [0x4d, 0x87, 0xc8, 0xf4]; // XCHG R8, R9, HLT
    let mut regs = Registers::default();
    regs.r8 = 0x1111111111111111;
    regs.r9 = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x2222222222222222, "R8 should have R9's value");
    assert_eq!(regs.r9, 0x1111111111111111, "R9 should have R8's value");
}

#[test]
fn test_xchg_rax_r10() {
    let code = [0x49, 0x87, 0xc2, 0xf4]; // XCHG RAX, R10, HLT
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.r10 = 0xBBBBBBBBBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xBBBBBBBBBBBBBBBB, "RAX should have R10's value");
    assert_eq!(regs.r10, 0xAAAAAAAAAAAAAAAA, "R10 should have RAX's value");
}

#[test]
fn test_xchg_r11_r12() {
    let code = [0x4d, 0x87, 0xe3, 0xf4]; // XCHG R11, R12, HLT
    let mut regs = Registers::default();
    regs.r11 = 0x1234567890ABCDEF;
    regs.r12 = 0xFEDCBA0987654321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r11, 0xFEDCBA0987654321, "R11 should have R12's value");
    assert_eq!(regs.r12, 0x1234567890ABCDEF, "R12 should have R11's value");
}

// Test exchanging zero values
#[test]
fn test_xchg_with_zero() {
    let code = [0x87, 0xd8, 0xf4]; // XCHG EAX, EBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should have EBX's value"
    );
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0, "EBX should be 0");
}

// Test exchanging same values
#[test]
fn test_xchg_same_value() {
    let code = [0x87, 0xd8, 0xf4]; // XCHG EAX, EBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should remain 0xFFFFFFFF"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EBX should remain 0xFFFFFFFF"
    );
}

// Test XCHG with same register (NOP-like behavior)
#[test]
fn test_xchg_eax_eax() {
    let code = [0x87, 0xc0, 0xf4]; // XCHG EAX, EAX, HLT
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000012345678,
        "RAX upper bits zeroed, value unchanged"
    );
}

// Test that flags are not affected
#[test]
fn test_xchg_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x87, 0xd8, // XCHG EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set");
}

// Test byte register exchange (8-bit)
#[test]
fn test_xchg_al_bl() {
    let code = [0x86, 0xd8, 0xf4]; // XCHG AL, BL, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFF11;
    regs.rbx = 0xFFFFFFFFFFFFFF22;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x22, "AL should have BL's value");
    assert_eq!(regs.rbx & 0xFF, 0x11, "BL should have AL's value");
    assert_eq!(
        regs.rax & 0xFFFFFFFFFFFFFF00,
        0xFFFFFFFFFFFFFF00,
        "Upper bits of RAX preserved"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFFFFFFFF00,
        0xFFFFFFFFFFFFFF00,
        "Upper bits of RBX preserved"
    );
}

#[test]
fn test_xchg_cl_dl() {
    let code = [0x86, 0xd1, 0xf4]; // XCHG CL, DL, HLT
    let mut regs = Registers::default();
    regs.rcx = 0xFFFFFFFFFFFFFFAA;
    regs.rdx = 0xFFFFFFFFFFFFFFBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFF, 0xBB, "CL should have DL's value");
    assert_eq!(regs.rdx & 0xFF, 0xAA, "DL should have CL's value");
}

// Test practical use case: swapping two values
#[test]
fn test_xchg_practical_swap() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0xc8, 0x00, 0x00, 0x00, // MOV RBX, 200
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 200, "RAX should be 200");
    assert_eq!(regs.rbx, 100, "RBX should be 100");
}

// Test chaining multiple exchanges
#[test]
fn test_xchg_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x87, 0xd8, // XCHG RAX, RBX (RAX=2, RBX=1)
        0x48, 0x87, 0xc8, // XCHG RAX, RCX (RAX=3, RCX=2)
        0x48, 0x87, 0xd9, // XCHG RBX, RCX (RBX=2, RCX=1)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "RAX should be 3");
    assert_eq!(regs.rbx, 2, "RBX should be 2");
    assert_eq!(regs.rcx, 1, "RCX should be 1");
}

// Test with maximum values
#[test]
fn test_xchg_max_values() {
    let code = [0x48, 0x87, 0xd8, 0xf4]; // XCHG RAX, RBX, HLT
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be 0");
    assert_eq!(regs.rbx, 0xFFFFFFFFFFFFFFFF, "RBX should be all 1s");
}

// Test short form XCHG RAX, RAX (canonical NOP on x86-64)
#[test]
fn test_xchg_rax_rax_nop() {
    let code = [0x48, 0x90, 0xf4]; // XCHG RAX, RAX (NOP), HLT
    let mut regs = Registers::default();
    regs.rax = 0x1234567890ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1234567890ABCDEF, "RAX should remain unchanged");
}

// Test XCHG with SP/RSP (stack pointer)
#[test]
fn test_xchg_rax_rsp() {
    let code = [0x48, 0x94, 0xf4]; // XCHG RAX, RSP, HLT
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rsp = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222, "RAX should have RSP's value");
    assert_eq!(regs.rsp, 0x1111111111111111, "RSP should have RAX's value");
}

// Test XCHG with BP/RBP (base pointer)
#[test]
fn test_xchg_rax_rbp() {
    let code = [0x48, 0x95, 0xf4]; // XCHG RAX, RBP, HLT
    let mut regs = Registers::default();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.rbp = 0xBBBBBBBBBBBBBBBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xBBBBBBBBBBBBBBBB, "RAX should have RBP's value");
    assert_eq!(regs.rbp, 0xAAAAAAAAAAAAAAAA, "RBP should have RAX's value");
}

// Test 32-bit XCHG with SP (ESP)
#[test]
fn test_xchg_eax_esp() {
    let code = [0x94, 0xf4]; // XCHG EAX, ESP, HLT
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF11111111;
    regs.rsp = 0xCAFEBABE22222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000022222222,
        "RAX should have ESP's value (upper bits zeroed)"
    );
    assert_eq!(
        regs.rsp, 0x0000000011111111,
        "RSP should have EAX's value (upper bits zeroed)"
    );
}

// ============================================================================
// Strengthened XCHG tests (appended): exact swapped values across all operand
// sizes, register<->memory exchange (implicitly atomic per the ISA), and the
// guarantee that XCHG does not affect flags.
// ============================================================================

#[test]
fn test_strict_xchg_r64_r64_exact() {
    // XCHG RAX, RBX swaps full 64-bit values.
    let code = [0x48, 0x93, 0xf4]; // XCHG RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 0x0123_4567_89AB_CDEF;
    regs.rbx = 0xFEDC_BA98_7654_3210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDC_BA98_7654_3210);
    assert_eq!(regs.rbx, 0x0123_4567_89AB_CDEF);
}

#[test]
fn test_strict_xchg_r8_exact() {
    // XCHG AL, BL swaps only the low byte, preserving the rest.
    let code = [0x86, 0xd8, 0xf4]; // XCHG AL, BL
    let mut regs = Registers::default();
    regs.rax = 0x1111_1111_1111_1111;
    regs.rbx = 0x2222_2222_2222_2222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1111_1111_1111_1122, "AL got BL");
    assert_eq!(regs.rbx, 0x2222_2222_2222_2211, "BL got AL");
}

#[test]
fn test_strict_xchg_r16_exact() {
    // XCHG AX, BX swaps low 16 bits, preserving the upper 48.
    let code = [0x66, 0x87, 0xd8, 0xf4]; // XCHG AX, BX
    let mut regs = Registers::default();
    regs.rax = 0xAAAA_AAAA_AAAA_1234;
    regs.rbx = 0xBBBB_BBBB_BBBB_5678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xAAAA_AAAA_AAAA_5678);
    assert_eq!(regs.rbx, 0xBBBB_BBBB_BBBB_1234);
}

#[test]
fn test_strict_xchg_r64_mem_atomic_swap() {
    // XCHG RAX, [RBX]: exchange register with memory. XCHG with a memory operand
    // is implicitly locked (atomic) per the ISA; observable effect is a full swap.
    let code = [0x48, 0x87, 0x03, 0xf4]; // XCHG RAX, [RBX]
    let mut regs = Registers::default();
    regs.rax = 0xCAFE_BABE_DEAD_BEEF;
    regs.rbx = crate::common::DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    crate::common::write_mem_at_u64(&mem, crate::common::DATA_ADDR, 0x1122_3344_5566_7788);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1122_3344_5566_7788, "RAX got memory value");
    assert_eq!(
        crate::common::read_mem_at_u64(&mem, crate::common::DATA_ADDR),
        0xCAFE_BABE_DEAD_BEEF,
        "memory got RAX"
    );
}

#[test]
fn test_strict_xchg_r32_mem_zero_extends() {
    // XCHG EAX, [RBX]: 32-bit memory swap; RAX upper 32 bits cleared.
    let code = [0x87, 0x03, 0xf4]; // XCHG EAX, [RBX]
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_AABB_CCDD;
    regs.rbx = crate::common::DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    crate::common::write_mem_at_u32(&mem, crate::common::DATA_ADDR, 0x1234_5678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_1234_5678,
        "EAX got mem value, upper cleared"
    );
    assert_eq!(
        crate::common::read_mem_at_u32(&mem, crate::common::DATA_ADDR),
        0xAABB_CCDD,
        "mem got EAX"
    );
}

#[test]
fn test_strict_xchg_lock_prefix_mem() {
    // LOCK XCHG [RBX], RAX — explicit LOCK prefix, same observable swap.
    let code = [0xf0, 0x48, 0x87, 0x03, 0xf4]; // LOCK XCHG [RBX], RAX
    let mut regs = Registers::default();
    regs.rax = 0x00FF_00FF_00FF_00FF;
    regs.rbx = crate::common::DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    crate::common::write_mem_at_u64(&mem, crate::common::DATA_ADDR, 0xFF00_FF00_FF00_FF00);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFF00_FF00_FF00_FF00);
    assert_eq!(
        crate::common::read_mem_at_u64(&mem, crate::common::DATA_ADDR),
        0x00FF_00FF_00FF_00FF
    );
}

#[test]
fn test_strict_xchg_does_not_touch_flags() {
    // XCHG must not affect flags.
    let code = [0x48, 0x93, 0xf4]; // XCHG RAX, RBX
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 2;
    regs.rflags = 0x2 | 0x1 | 0x40 | 0x80 | 0x800;
    let before = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2);
    assert_eq!(regs.rbx, 1);
    assert_eq!(
        regs.rflags & 0x8D5,
        before & 0x8D5,
        "XCHG must not alter status flags"
    );
}
