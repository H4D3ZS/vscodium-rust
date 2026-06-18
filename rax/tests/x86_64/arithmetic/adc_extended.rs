use crate::common::*;
use rax::cpu::Registers;

// ADC — Add with Carry
//
// Opcodes:
// - 14 ib           ADC AL, imm8      Add with carry imm8 to AL
// - 15 iw           ADC AX, imm16     Add with carry imm16 to AX
// - 15 id           ADC EAX, imm32    Add with carry imm32 to EAX
// - REX.W + 15 id   ADC RAX, imm32    Add with carry imm32 to RAX (sign-extended)
// - 80 /2 ib        ADC r/m8, imm8    Add with carry imm8 to r/m8
// - 81 /2 iw        ADC r/m16, imm16  Add with carry imm16 to r/m16
// - 81 /2 id        ADC r/m32, imm32  Add with carry imm32 to r/m32
// - REX.W + 81 /2 id ADC r/m64, imm32 Add with carry imm32 (sign-extended) to r/m64
// - 83 /2 ib        ADC r/m16, imm8   Add with carry sign-extended imm8 to r/m16
// - 83 /2 ib        ADC r/m32, imm8   Add with carry sign-extended imm8 to r/m32
// - REX.W + 83 /2 ib ADC r/m64, imm8  Add with carry sign-extended imm8 to r/m64
// - 10 /r           ADC r/m8, r8      Add with carry r8 to r/m8
// - 11 /r           ADC r/m16, r16    Add with carry r16 to r/m16
// - 11 /r           ADC r/m32, r32    Add with carry r32 to r/m32
// - REX.W + 11 /r   ADC r/m64, r64    Add with carry r64 to r/m64
// - 12 /r           ADC r8, r/m8      Add with carry r/m8 to r8
// - 13 /r           ADC r16, r/m16    Add with carry r/m16 to r16
// - 13 /r           ADC r32, r/m32    Add with carry r/m32 to r32
// - REX.W + 13 /r   ADC r64, r/m64    Add with carry r/m64 to r64
//
// Operation: DEST := DEST + SRC + CF
//
// Flags: CF, OF, SF, ZF, AF, PF are set according to result

// ============================================================================
// 8-bit ADC Tests
// ============================================================================

#[test]
fn test_adc_al_imm8_no_carry() {
    // ADC AL, 5 with CF=0
    let code = [
        0x14, 0x05, // ADC AL, 5
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x0F, "AL should be 15 (10 + 5)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_al_imm8_with_carry() {
    // ADC AL, 5 with CF=1
    let code = [0x14, 0x05, 0xf4]; // ADC AL, 5; HLT
    let mut regs = Registers::default();
    regs.rax = 0x0A;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x10, "AL should be 16 (10 + 5 + 1)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_al_overflow() {
    // ADC AL, 0xFF with CF=1 (causes overflow)
    let code = [0x14, 0xFF, 0xf4]; // ADC AL, 0xFF; HLT
    let mut regs = Registers::default();
    regs.rax = 0x02;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x02 + 0xFF + 1 = 0x102, AL = 0x02
    assert_eq!(regs.rax & 0xFF, 0x02, "AL should wrap to 0x02");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_adc_r8_r8_no_carry() {
    // ADC AL, BL with CF=0
    let code = [
        0x10, 0xd8, // ADC AL, BL (ModRM: 11_011_000)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x20;
    regs.rbx = 0x15;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x35, "AL should be 0x35 (32 + 21)");
}

#[test]
fn test_adc_r8_r8_with_carry() {
    // ADC AL, CL with CF=1
    let code = [0x10, 0xc8, 0xf4]; // ADC AL, CL; HLT
    let mut regs = Registers::default();
    regs.rax = 0x7F;
    regs.rcx = 0x01;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x81, "AL should be 0x81 (127 + 1 + 1)");
    assert!(sf_set(regs.rflags), "SF should be set");
    assert!(of_set(regs.rflags), "OF should be set (signed overflow)");
}

#[test]
fn test_adc_all_8bit_registers() {
    // Test ADC with different 8-bit registers
    let test_cases = vec![
        (0xd8, "AL"), // ADC AL, BL
        (0xc8, "AL"), // ADC AL, CL
        (0xd0, "AL"), // ADC AL, DL
    ];

    for (modrm, _name) in test_cases {
        let code = [0x10, modrm, 0xf4]; // ADC
        let mut regs = Registers::default();
        regs.rax = 0x10;
        regs.rbx = 0x05;
        regs.rcx = 0x06;
        regs.rdx = 0x07;
        regs.rflags = 0x01; // Set CF
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        // Result should include carry
        assert!((regs.rax & 0xFF) > 0x10, "ADC should add carry");
    }
}

#[test]
fn test_adc_extended_r8_registers() {
    // ADC R8B, R9B with CF=1
    let code = [
        0x45, 0x10, 0xc8, // ADC R8B, R9B (REX.RB + 10 /r)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x40;
    regs.r9 = 0x30;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x71, "R8B should be 0x71 (64 + 48 + 1)");
}

// ============================================================================
// 16-bit ADC Tests
// ============================================================================

#[test]
fn test_adc_ax_imm16_no_carry() {
    // ADC AX, 0x1234 with CF=0
    let code = [
        0x66, 0x15, 0x34, 0x12, // ADC AX, 0x1234
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x5678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x68AC, "AX should be 0x68AC");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_ax_imm16_with_carry() {
    // ADC AX, 0xFFFF with CF=1 (causes overflow)
    let code = [0x66, 0x15, 0xFF, 0xFF, 0xf4]; // ADC AX, 0xFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0002;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x0002 + 0xFFFF + 1 = 0x10002, AX = 0x0002
    assert_eq!(regs.rax & 0xFFFF, 0x0002, "AX should wrap to 0x0002");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_r16_r16() {
    // ADC AX, BX with CF=1
    let code = [
        0x66, 0x11, 0xd8, // ADC AX, BX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    regs.rbx = 0x2000;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x3001, "AX should be 0x3001");
}

#[test]
fn test_adc_r16_imm8_sign_extended() {
    // ADC AX, -1 (sign-extended from imm8)
    let code = [
        0x66, 0x83, 0xd0, 0xFF, // ADC AX, -1 (sign-extended)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x1000 + 0xFFFF + 0 = 0x0FFF
    assert_eq!(regs.rax & 0xFFFF, 0x0FFF, "AX should be 0x0FFF");
}

#[test]
fn test_adc_extended_r16_registers() {
    // ADC R10W, R11W with CF=1
    let code = [
        0x66, 0x45, 0x11, 0xda, // ADC R10W, R11W
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x8000;
    regs.r11 = 0x7FFF;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10 & 0xFFFF, 0x0000, "R10W should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// 32-bit ADC Tests
// ============================================================================

#[test]
fn test_adc_eax_imm32_no_carry() {
    // ADC EAX, 0x12345678 with CF=0
    let code = [
        0x15, 0x78, 0x56, 0x34, 0x12, // ADC EAX, 0x12345678
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x23456789, "EAX should be 0x23456789");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_eax_imm32_with_carry() {
    // ADC EAX, 0xFFFFFFFF with CF=1
    let code = [0x15, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // ADC EAX, 0xFFFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x00000001;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x00000001 + 0xFFFFFFFF + 1 = 0x100000001, EAX = 0x00000001
    assert_eq!(regs.rax, 0x00000001, "EAX should wrap to 0x00000001");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_r32_r32() {
    // ADC EAX, EBX with CF=1
    let code = [
        0x11, 0xd8, // ADC EAX, EBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    regs.rbx = 0x7FFFFFFF;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000000, "EAX should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_r32_imm8_sign_extended() {
    // ADC EAX, -1 (sign-extended from imm8)
    let code = [
        0x83, 0xd0, 0xFF, // ADC EAX, -1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x10000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0FFFFFFF, "EAX should be 0x0FFFFFFF");
}

#[test]
fn test_adc_extended_r32_registers() {
    // ADC R12D, R13D with CF=1
    let code = [
        0x45, 0x11, 0xec, // ADC R12D, R13D
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r12 = 0xFFFFFFFF;
    regs.r13 = 0x00000001;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x00000001, "R12D should wrap to 1");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// 64-bit ADC Tests
// ============================================================================

#[test]
fn test_adc_rax_imm32_no_carry() {
    // ADC RAX, 0x12345678 (sign-extended to 64-bit)
    let code = [
        0x48, 0x15, 0x78, 0x56, 0x34, 0x12, // ADC RAX, 0x12345678
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111111123456789, "RAX should be correct");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_rax_imm32_with_carry() {
    // ADC RAX, 0x7FFFFFFF with CF=1
    let code = [0x48, 0x15, 0xFF, 0xFF, 0xFF, 0x7F, 0xf4]; // ADC RAX, 0x7FFFFFFF
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000080000001, "RAX should be 0x80000001");
}

#[test]
fn test_adc_r64_r64() {
    // ADC RAX, RBX with CF=1
    let code = [
        0x48, 0x11, 0xd8, // ADC RAX, RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0x0000000000000001;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "RAX should wrap to 1");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_r64_imm8_sign_extended() {
    // ADC RAX, -1 (sign-extended from imm8)
    let code = [
        0x48, 0x83, 0xd0, 0xFF, // ADC RAX, -1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0FFFFFFFFFFFFFFF, "RAX should be decremented");
}

#[test]
fn test_adc_extended_r64_registers() {
    // ADC R14, R15 with CF=1
    let code = [
        0x4d, 0x11, 0xfe, // ADC R14, R15
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.r14 = 0x8000000000000000;
    regs.r15 = 0x7FFFFFFFFFFFFFFF;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r14, 0x0000000000000000, "R14 should wrap to 0");
    assert!(cf_set(regs.rflags), "CF should be set");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_adc_byte_ptr_imm8() {
    // ADC BYTE PTR [mem], 0x10 with CF=1
    let code = [
        0x80, 0x15, 0xF9, 0x0F, 0x00, 0x00, 0x10, // ADC BYTE PTR [rip+0x0FF9], 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x20);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= 0x01; // Set CF
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x31, "Memory should be 0x31 (32 + 16 + 1)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_adc_word_ptr_imm16() {
    // ADC WORD PTR [mem], 0x1000 with CF=1
    let code = [
        0x66, 0x81, 0x15, 0xF7, 0x0F, 0x00, 0x00, 0x00,
        0x10, // ADC WORD PTR [rip+0x0FF7], 0x1000
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x2000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rflags |= 0x01; // Set CF
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x3001, "Memory should be 0x3001");
}

#[test]
fn test_adc_dword_ptr_r32() {
    // ADC DWORD PTR [mem], EBX with CF=1
    let code = [
        0x11, 0x1d, 0xFA, 0x0F, 0x00,
        0x00, // ADC DWORD PTR [rip+0x0FFA], EBX (target: 0x2000)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x30000000;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x40000000);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x70000001, "Memory should be 0x70000001");
}

#[test]
fn test_adc_qword_ptr_r64() {
    // ADC QWORD PTR [mem], RBX with CF=1
    let code = [
        0x48, 0x11, 0x1d, 0xF9, 0x0F, 0x00,
        0x00, // ADC QWORD PTR [rip+0x0FF9], RBX (target: 0x2000)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1000000000000000);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rbx = 0x2000000000000000;
    regs.rflags |= 0x01; // Set CF
    vcpu.set_regs(&regs).unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x3000000000000001, "Memory should include carry");
}

#[test]
fn test_adc_r64_from_memory() {
    // ADC RAX, QWORD PTR [mem] with CF=1
    let code = [
        0x48, 0x13, 0x05, 0xF9, 0x0F, 0x00, 0x00, // ADC RAX, QWORD PTR [rip+0x0FF6]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0FFFFFFFFFFFFFFF);
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 0x1000000000000000;
    regs.rflags |= 0x01; // Set CF
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000000000000000, "RAX should be correct sum");
}

// ============================================================================
// Flag Tests
// ============================================================================

#[test]
fn test_adc_zero_flag() {
    // ADC resulting in zero should set ZF
    let code = [0x14, 0xFF, 0xf4]; // ADC AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x00 + 0xFF + 1 = 0x100, AL = 0x00
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be 0");
    assert!(zf_set(regs.rflags), "ZF should be set");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_sign_flag() {
    // ADC resulting in negative (bit 7 set for 8-bit)
    let code = [0x14, 0x7F, 0xf4]; // ADC AL, 0x7F
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL should be 0x80");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_adc_overflow_flag() {
    // ADC causing signed overflow
    let code = [0x14, 0x01, 0xf4]; // ADC AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x7F; // Max positive i8
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x7F + 1 + 1 = 0x81 (overflow from positive to negative)
    assert_eq!(regs.rax & 0xFF, 0x81, "AL should be 0x81");
    assert!(of_set(regs.rflags), "OF should be set");
    assert!(sf_set(regs.rflags), "SF should be set");
}

#[test]
fn test_adc_parity_flag() {
    // Test parity flag (even number of 1-bits in low byte)
    let code = [0x14, 0x02, 0xf4]; // ADC AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result is 3 (0b00000011, even parity)
    assert_eq!(regs.rax & 0xFF, 0x03, "AL should be 3");
    assert!(pf_set(regs.rflags), "PF should be set (even parity)");
}

#[test]
fn test_adc_auxiliary_carry_flag() {
    // ADC with auxiliary carry (carry from bit 3 to bit 4)
    let code = [0x14, 0x0A, 0xf4]; // ADC AL, 0x0A
    let mut regs = Registers::default();
    regs.rax = 0x08;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0x08 + 0x0A = 0x12, AF should be set
    assert_eq!(regs.rax & 0xFF, 0x12, "AL should be 0x12");
    assert!(af_set(regs.rflags), "AF should be set");
}

// ============================================================================
// Multi-precision Arithmetic Tests
// ============================================================================

#[test]
fn test_adc_chain_64bit_addition() {
    // Simulate 128-bit addition using two ADC operations
    // Add 0x0000000000000001_FFFFFFFFFFFFFFFF + 0x0000000000000000_0000000000000002
    // Result: 0x0000000000000002_0000000000000001
    let code = [
        0x48, 0x01, 0xd8, // ADD RAX, RBX (low 64 bits)
        0x49, 0x11, 0xc8, // ADC R8, RCX (high 64 bits, with carry)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF; // Low 64 bits of first number
    regs.r8 = 0x0000000000000001; // High 64 bits of first number
    regs.rbx = 0x0000000000000002; // Low 64 bits of second number
    regs.rcx = 0x0000000000000000; // High 64 bits of second number
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "Low 64 bits should be 1");
    assert_eq!(
        regs.r8, 0x0000000000000002,
        "High 64 bits should be 2 (with carry)"
    );
}

#[test]
fn test_adc_preserves_high_bits_8bit() {
    let code = [0x14, 0x05, 0xf4]; // ADC AL, 5
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only AL should change
    assert_eq!(
        regs.rax >> 8,
        0xDEADBEEF123456,
        "High bits should be preserved"
    );
}

#[test]
fn test_adc_preserves_high_bits_16bit() {
    let code = [0x66, 0x15, 0x00, 0x10, 0xf4]; // ADC AX, 0x1000
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Only AX should change
    assert_eq!(
        regs.rax >> 16,
        0xDEADBEEF1234,
        "High bits should be preserved"
    );
}

#[test]
fn test_adc_preserves_high_bits_32bit() {
    let code = [0x15, 0x00, 0x00, 0x00, 0x10, 0xf4]; // ADC EAX, 0x10000000
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX operation zeros high 32 bits
    assert_eq!(
        regs.rax >> 32,
        0,
        "High 32 bits should be zeroed for 32-bit op"
    );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_adc_all_ones() {
    // ADC with all bits set
    let code = [0x48, 0x15, 0xFF, 0xFF, 0xFF, 0xFF, 0xf4]; // ADC RAX, 0xFFFFFFFF (sign-extended)
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 0xFFFFFFFFFFFFFFFF + 0xFFFFFFFFFFFFFFFF + 1 = 0x1FFFFFFFFFFFFFFFF
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should be all ones");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_adc_zero_plus_zero_plus_carry() {
    // 0 + 0 + 1 = 1
    let code = [0x14, 0x00, 0xf4]; // ADC AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x00;
    regs.rflags = 0x01; // Set CF
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x01, "AL should be 1");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}
