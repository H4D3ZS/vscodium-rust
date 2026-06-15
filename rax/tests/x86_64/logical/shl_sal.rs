use crate::common::*;
use crate::common::{run_until_hlt, setup_vm};
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// SHL/SAL — Shift Left (Logical/Arithmetic)
// SHL and SAL are the same instruction (same opcodes)
//
// Opcodes:
// - D0 /4       SHL r/m8, 1
// - D2 /4       SHL r/m8, CL
// - C0 /4 ib    SHL r/m8, imm8
// - D1 /4       SHL r/m16/32/64, 1
// - D3 /4       SHL r/m16/32/64, CL
// - C1 /4 ib    SHL r/m16/32/64, imm8
//
// Flags:
// - CF: Last bit shifted out
// - OF: Only for 1-bit shifts (MSB of result XOR CF)
// - SF, ZF, PF: Set according to result
// - Count is 0: No flags affected
// - Count is masked to 5 bits (0x1F) for 8/16/32-bit, 6 bits (0x3F) for 64-bit

// ============================================================================
// 8-bit SHL tests
// ============================================================================

#[test]
fn test_shl_al_1_basic() {
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x42; // 0100_0010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x84, "AL: 0x42 << 1 = 0x84");
    assert!(!cf_set(regs.rflags), "CF clear (MSB was 0)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 1 XOR 0 = 1");
    assert!(sf_set(regs.rflags), "SF set (bit 7 = 1)");
}

#[test]
fn test_shl_al_1_with_carry() {
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x81; // 1000_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x02, "AL: 0x81 << 1 = 0x02");
    assert!(cf_set(regs.rflags), "CF set (MSB was 1)");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF = 0 XOR 1 = 1");
}

#[test]
fn test_shl_al_1_no_overflow() {
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    let mut regs = Registers::default();
    regs.rax = 0xC0; // 1100_0000
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x80, "AL: 0xC0 << 1 = 0x80");
    assert!(cf_set(regs.rflags), "CF set");
    assert!(!of_set(regs.rflags), "OF clear: MSB XOR CF = 1 XOR 1 = 0");
}

#[test]
fn test_shl_bl_cl() {
    let code = [0xd2, 0xe3, 0xf4]; // SHL BL, CL
    let mut regs = Registers::default();
    regs.rbx = 0x01;
    regs.rcx = 0x07;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0x80, "BL: 0x01 << 7 = 0x80");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_cl_imm8() {
    let code = [0xc0, 0xe1, 0x03, 0xf4]; // SHL CL, 3
    let mut regs = Registers::default();
    regs.rcx = 0x11; // 0001_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFF, 0x88, "CL: 0x11 << 3 = 0x88");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_al_to_zero() {
    let code = [0xc0, 0xe0, 0x08, 0xf4]; // SHL AL, 8
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL: all bits shifted out");
    assert!(zf_set(regs.rflags), "ZF set (result is zero)");
    assert!(!sf_set(regs.rflags), "SF clear");
}

#[test]
fn test_shl_count_masked_8bit() {
    let code = [0xd2, 0xe0, 0xf4]; // SHL AL, CL
    let mut regs = Registers::default();
    regs.rax = 0x11;
    regs.rcx = 0x23; // 35 & 0x1F = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x88, "AL: 0x11 << 3 = 0x88 (count masked)");
}

#[test]
fn test_shl_count_zero_preserves_flags() {
    let code = [0xc0, 0xe0, 0x00, 0xf4]; // SHL AL, 0
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rflags = 0x2 | flags::bits::CF | flags::bits::ZF | flags::bits::OF;
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "AL unchanged");
    assert_eq!(
        regs.rflags, initial_flags,
        "Flags unchanged when count is 0"
    );
}

#[test]
fn test_shl_dh_1() {
    let code = [0xd0, 0xe6, 0xf4]; // SHL DH, 1
    let mut regs = Registers::default();
    regs.rdx = 0x4200; // DH = 0x42
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!((regs.rdx >> 8) & 0xFF, 0x84, "DH: 0x42 << 1 = 0x84");
}

// ============================================================================
// 16-bit SHL tests
// ============================================================================

#[test]
fn test_shl_ax_1() {
    let code = [0x66, 0xd1, 0xe0, 0xf4]; // SHL AX, 1
    let mut regs = Registers::default();
    regs.rax = 0x4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8642, "AX: 0x4321 << 1 = 0x8642");
    assert!(!cf_set(regs.rflags), "CF clear");
    assert!(of_set(regs.rflags), "OF: MSB XOR CF");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_ax_cl() {
    let code = [0x66, 0xd3, 0xe0, 0xf4]; // SHL AX, CL
    let mut regs = Registers::default();
    regs.rax = 0x0001;
    regs.rcx = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x8000, "AX: 0x0001 << 15 = 0x8000");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_bx_imm8() {
    let code = [0x66, 0xc1, 0xe3, 0x04, 0xf4]; // SHL BX, 4
    let mut regs = Registers::default();
    regs.rbx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFF, 0x2340, "BX: 0x1234 << 4 = 0x2340");
}

#[test]
fn test_shl_cx_to_zero() {
    let code = [0x66, 0xc1, 0xe1, 0x10, 0xf4]; // SHL CX, 16
    let mut regs = Registers::default();
    regs.rcx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx & 0xFFFF, 0x0000, "CX: all bits shifted out");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_shl_si_1_with_carry() {
    let code = [0x66, 0xd1, 0xe6, 0xf4]; // SHL SI, 1
    let mut regs = Registers::default();
    regs.rsi = 0x8001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi & 0xFFFF, 0x0002, "SI: 0x8001 << 1 = 0x0002");
    assert!(cf_set(regs.rflags), "CF set (MSB was 1)");
}

// ============================================================================
// 32-bit SHL tests
// ============================================================================

#[test]
fn test_shl_eax_1() {
    let code = [0xd1, 0xe0, 0xf4]; // SHL EAX, 1
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2468ACF0, "EAX: 0x12345678 << 1 = 0x2468ACF0");
    assert!(!cf_set(regs.rflags), "CF clear");
    assert!(!of_set(regs.rflags), "OF clear");
}

#[test]
fn test_shl_ebx_cl() {
    let code = [0xd3, 0xe3, 0xf4]; // SHL EBX, CL
    let mut regs = Registers::default();
    regs.rbx = 0x00000001;
    regs.rcx = 0x1F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x80000000, "EBX: 0x00000001 << 31 = 0x80000000");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_ecx_imm8() {
    let code = [0xc1, 0xe1, 0x08, 0xf4]; // SHL ECX, 8
    let mut regs = Registers::default();
    regs.rcx = 0x00123456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x12345600, "ECX: 0x00123456 << 8 = 0x12345600");
}

#[test]
fn test_shl_esi_with_carry() {
    let code = [0xc1, 0xe6, 0x10, 0xf4]; // SHL ESI, 16
    let mut regs = Registers::default();
    regs.rsi = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x56780000, "ESI: 0x12345678 << 16 = 0x56780000");
}

#[test]
fn test_shl_edi_to_zero() {
    // Note: SHL EDI, 32 would mask to 0 (no shift), so we use 31 which leaves only MSB
    // Then shift once more to get zero
    let code = [
        0xc1, 0xe7, 0x1f, // SHL EDI, 31
        0xd1, 0xe7, // SHL EDI, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x00000000, "EDI: all bits shifted out");
    assert!(zf_set(regs.rflags), "ZF set");
}

// ============================================================================
// 64-bit SHL tests
// ============================================================================

#[test]
fn test_shl_rax_1() {
    let code = [0x48, 0xd1, 0xe0, 0xf4]; // SHL RAX, 1
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2468ACF13579BDE0, "RAX: << 1");
    assert!(!cf_set(regs.rflags), "CF clear");
    assert!(!of_set(regs.rflags), "OF clear");
}

#[test]
fn test_shl_rbx_cl() {
    let code = [0x48, 0xd3, 0xe3, 0xf4]; // SHL RBX, CL
    let mut regs = Registers::default();
    regs.rbx = 0x0000000000000001;
    regs.rcx = 0x3F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x8000000000000000, "RBX: 0x01 << 63 = 0x8000...0");
    assert!(!cf_set(regs.rflags), "CF: last bit shifted out was 0");
    assert!(sf_set(regs.rflags), "SF set");
}

#[test]
fn test_shl_rcx_imm8() {
    let code = [0x48, 0xc1, 0xe1, 0x10, 0xf4]; // SHL RCX, 16
    let mut regs = Registers::default();
    regs.rcx = 0x0000123456789ABC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x123456789ABC0000, "RCX: << 16");
}

#[test]
fn test_shl_rsi_with_carry() {
    let code = [0x48, 0xc1, 0xe6, 0x20, 0xf4]; // SHL RSI, 32
    let mut regs = Registers::default();
    regs.rsi = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsi, 0x9ABCDEF000000000, "RSI: << 32");
}

#[test]
fn test_shl_rdi_to_zero() {
    // Note: SHL RDI, 64 would mask to 0 (no shift), so we use 63 + 1
    let code = [
        0x48, 0xc1, 0xe7, 0x3f, // SHL RDI, 63
        0x48, 0xd1, 0xe7, // SHL RDI, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rdi, 0x0000000000000000, "RDI: all bits shifted out");
    assert!(zf_set(regs.rflags), "ZF set");
}

#[test]
fn test_shl_count_masked_64bit() {
    let code = [0x48, 0xd3, 0xe0, 0xf4]; // SHL RAX, CL
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000001;
    regs.rcx = 0x43; // 67 & 0x3F = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000008,
        "RAX: 0x01 << 3 = 0x08 (count masked)"
    );
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_shl_r8b_1() {
    let code = [0x41, 0xd0, 0xe0, 0xf4]; // SHL R8B, 1
    let mut regs = Registers::default();
    regs.r8 = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x84, "R8B: 0x42 << 1 = 0x84");
}

#[test]
fn test_shl_r9w_cl() {
    let code = [0x66, 0x41, 0xd3, 0xe1, 0xf4]; // SHL R9W, CL
    let mut regs = Registers::default();
    regs.r9 = 0x0001;
    regs.rcx = 0x0F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0x8000, "R9W: 0x0001 << 15 = 0x8000");
}

#[test]
fn test_shl_r10d_imm8() {
    let code = [0x41, 0xc1, 0xe2, 0x08, 0xf4]; // SHL R10D, 8
    let mut regs = Registers::default();
    regs.r10 = 0x00123456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0x12345600, "R10D: << 8");
}

#[test]
fn test_shl_r11_1() {
    let code = [0x49, 0xd1, 0xe3, 0xf4]; // SHL R11, 1
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0x2468ACF13579BDE0, "R11: << 1");
}

#[test]
fn test_shl_r12_cl() {
    let code = [0x49, 0xd3, 0xe4, 0xf4]; // SHL R12, CL
    let mut regs = Registers::default();
    regs.r12 = 0x0000000000000001;
    regs.rcx = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r12, 0x0000000100000000, "R12: << 32");
}

#[test]
fn test_shl_r15_imm8() {
    let code = [0x49, 0xc1, 0xe7, 0x10, 0xf4]; // SHL R15, 16
    let mut regs = Registers::default();
    regs.r15 = 0x0000123456789ABC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x123456789ABC0000, "R15: << 16");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_shl_byte_ptr_1() {
    let code = [
        0xd0, 0x25, 0xfa, 0x0f, 0x00, 0x00, // SHL BYTE PTR [rip+0x0FFA], 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x84, "Memory: 0x42 << 1 = 0x84");
}

#[test]
fn test_shl_word_ptr_cl() {
    let code = [
        0x66, 0xd3, 0x25, 0xf9, 0x0f, 0x00, 0x00, // SHL WORD PTR [rip+0x0FF9], CL
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x0001);
    let mut regs = Registers::default();
    regs.rcx = 0x0F;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x0001);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0x8000, "Memory: 0x0001 << 15 = 0x8000");
}

#[test]
fn test_shl_dword_ptr_imm8() {
    let code = [
        0xc1, 0x25, 0xf9, 0x0f, 0x00, 0x00, 0x08, // SHL DWORD PTR [rip+0x0FF9], 8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00123456);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0x12345600, "Memory: << 8");
}

#[test]
fn test_shl_qword_ptr_imm8() {
    let code = [
        0x48, 0xc1, 0x25, 0xf8, 0x0f, 0x00, 0x00, 0x10, // SHL QWORD PTR [rip+0x0FF8], 16
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0000123456789ABC);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0x123456789ABC0000, "Memory: << 16");
}

// ============================================================================
// Parity flag tests
// ============================================================================

#[test]
fn test_shl_parity_even() {
    let code = [0xc0, 0xe0, 0x02, 0xf4]; // SHL AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x01; // Shift to 0x04 (one 1-bit = odd, but PF checks low byte)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x04 (one 1-bit = odd parity)
    assert_eq!(regs.rax & 0xFF, 0x04);
    assert!(!pf_set(regs.rflags), "PF clear (odd parity)");
}

#[test]
fn test_shl_parity_odd() {
    let code = [0xc0, 0xe0, 0x02, 0xf4]; // SHL AL, 2
    let mut regs = Registers::default();
    regs.rax = 0x03; // Shift to 0x0C (two 1-bits = even parity)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Result = 0x0C (two 1-bits = even parity)
    assert_eq!(regs.rax & 0xFF, 0x0C);
    assert!(pf_set(regs.rflags), "PF set (even parity)");
}

// ============================================================================
// Edge cases and special tests
// ============================================================================

#[test]
fn test_shl_multiple_operations() {
    let code = [
        0xd0, 0xe0, // SHL AL, 1
        0xd0, 0xe0, // SHL AL, 1
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x21; // 0010_0001
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x84, "AL: 0x21 << 2 = 0x84");
}

// ============================================================================
// Strengthened SHL tests (appended): exact result + CF (last bit shifted out)
// and OF (defined only for count == 1: OF = MSB(result) XOR CF). Plus SHLD.
// ============================================================================

#[test]
fn test_strict_shl_by1_cf_and_of() {
    // SHL AL,1 with AL=0xC0 (1100_0000): result 0x80, bit shifted out = 1 -> CF=1.
    // OF (count==1) = MSB(result) XOR CF = 1 XOR 1 = 0.
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    let mut regs = Registers::default();
    regs.rax = 0xC0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80, "0xC0 << 1 = 0x80");
    assert!(cf_set(regs.rflags), "CF = bit shifted out (1)");
    assert!(!of_set(regs.rflags), "OF = MSB^CF = 0");
    assert!(sf_set(regs.rflags), "SF set (bit 7)");
}

#[test]
fn test_strict_shl_by1_of_set() {
    // SHL AL,1 with AL=0x40 (0100_0000): result 0x80, CF=0, OF = 1 XOR 0 = 1.
    let code = [0xd0, 0xe0, 0xf4]; // SHL AL, 1
    let mut regs = Registers::default();
    regs.rax = 0x40;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80, "0x40 << 1 = 0x80");
    assert!(!cf_set(regs.rflags), "CF = 0 (bit 6 shifted into bit 7)");
    assert!(of_set(regs.rflags), "OF set (sign changed)");
}

#[test]
fn test_strict_shl_r32_by_cl_cf() {
    // SHL EAX, CL with EAX=0x8000_0001, CL=1: result 0x0000_0002, CF=1 (bit31 out).
    let code = [0xd3, 0xe0, 0xf4]; // SHL EAX, CL
    let mut regs = Registers::default();
    regs.rax = 0x8000_0001;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0002,
        "0x80000001 << 1 = 0x2 (32-bit, zero-extended)"
    );
    assert!(cf_set(regs.rflags), "CF = bit 31 shifted out");
}

#[test]
fn test_strict_shl_r64_imm_cf() {
    // SHL RAX, 4 with RAX=0x1000_0000_0000_000F -> 0x0000_0000_0000_00F0, CF = bit60 = 0.
    let code = [0x48, 0xc1, 0xe0, 0x04, 0xf4]; // SHL RAX, 4
    let mut regs = Registers::default();
    regs.rax = 0x1000_0000_0000_000F;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_0000_0000_00F0, "RAX << 4 drops top nibble");
    // Last bit shifted out for SHL by N is original bit (width-N) = bit 60.
    // 0x1000_0000_0000_000F has bit 60 set, so CF = 1.
    assert!(
        cf_set(regs.rflags),
        "CF = last bit shifted out (bit 60 = 1)"
    );
}

#[test]
fn test_strict_shl_count_masked_64() {
    // SHL RAX, 65: count masked to 65 & 0x3F = 1.
    let code = [0x48, 0xc1, 0xe0, 0x41, 0xf4]; // SHL RAX, 0x41
    let mut regs = Registers::default();
    regs.rax = 0x1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2, "count masked to 1 -> shift by 1");
}

#[test]
fn test_strict_shl_zero_count_preserves_flags() {
    // SHL EAX, CL with CL=0 must NOT modify flags. Seed CF and verify it survives.
    let code = [0xd3, 0xe0, 0xf4]; // SHL EAX, CL
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rcx = 0;
    regs.rflags = 0x2 | 0x1; // CF set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1234, "value unchanged for count 0");
    assert!(cf_set(regs.rflags), "flags unchanged when count == 0");
}

#[test]
fn test_strict_shld_r32() {
    // SHLD EAX, EDX, 8 (0F A4): shift EAX left 8, filling from high bits of EDX.
    // EAX=0x12345678, EDX=0xAABBCCDD, count=8:
    //   result = (0x12345678 << 8) | (0xAABBCCDD >> 24) = 0x345678AA.
    let code = [0x0f, 0xa4, 0xd0, 0x08, 0xf4]; // SHLD EAX, EDX, 8
    let mut regs = Registers::default();
    regs.rax = 0x1234_5678;
    regs.rdx = 0xAABB_CCDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x3456_78AA,
        "SHLD EAX,EDX,8 brings in EDX high byte"
    );
}

#[test]
fn test_strict_shld_r64() {
    // SHLD RAX, RDX, 16 (REX.W 0F A4):
    //   (0x1122334455667788 << 16) | (0xAABBCCDDEEFF0011 >> 48) = 0x334455667788AABB.
    let code = [0x48, 0x0f, 0xa4, 0xd0, 0x10, 0xf4]; // SHLD RAX, RDX, 16
    let mut regs = Registers::default();
    regs.rax = 0x1122_3344_5566_7788;
    regs.rdx = 0xAABB_CCDD_EEFF_0011;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x3344_5566_7788_AABB, "SHLD RAX,RDX,16");
}
