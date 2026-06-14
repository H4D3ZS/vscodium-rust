use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::Registers;

// NOT — One's Complement Negation
//
// Opcodes:
// - F6 /2       NOT r/m8      Reverse each bit of r/m8
// - REX+F6 /2   NOT r/m8*     (with REX for extended regs)
// - F7 /2       NOT r/m16     Reverse each bit of r/m16
// - F7 /2       NOT r/m32     Reverse each bit of r/m32
// - REX.W+F7 /2 NOT r/m64     Reverse each bit of r/m64
//
// Operation: DEST := NOT DEST (bitwise inversion)
//
// Flags: NONE - NOT does not affect any flags!
//
// CRITICAL: NOT is one's complement (bitwise inversion). Each 0 becomes 1,
// each 1 becomes 0. Equivalent to XOR with all 1s (-1).

// ============================================================================
// 8-bit NOT
// ============================================================================

#[test]
fn test_not_al_basic() {
    let code = [
        0xf6, 0xd0, // NOT AL (F6 /2, ModRM=11_010_000)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA; // 10101010
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 10101010 inverted = 01010101 = 0x55
    assert_eq!(regs.rax & 0xFF, 0x55, "AL: NOT 0xAA = 0x55");
}

#[test]
fn test_not_al_all_zeros() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xFF, "AL: NOT 0x00 = 0xFF");
}

#[test]
fn test_not_al_all_ones() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x00, "AL: NOT 0xFF = 0x00");
}

#[test]
fn test_not_al_partial_bits() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0x0F; // 00001111
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xF0, "AL: NOT 0x0F = 0xF0");
}

#[test]
fn test_not_bl_register() {
    let code = [
        0xf6, 0xd3, // NOT BL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3C; // 00111100
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFF, 0xC3, "BL: NOT 0x3C = 0xC3");
}

#[test]
fn test_not_preserves_high_bytes_8bit() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x87, "AL: NOT 0x78 = 0x87");
    assert_eq!(
        regs.rax & !0xFF,
        0xDEADBEEF_12345600,
        "Upper bytes preserved"
    );
}

// ============================================================================
// 16-bit NOT
// ============================================================================

#[test]
fn test_not_ax_basic() {
    let code = [
        0x66, 0xf7, 0xd0, // NOT AX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xEDCB, "AX: NOT 0x1234 = 0xEDCB");
}

#[test]
fn test_not_ax_all_zeros() {
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    let mut regs = Registers::default();
    regs.rax = 0x0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX: NOT 0 = 0xFFFF");
}

#[test]
fn test_not_ax_pattern() {
    let code = [0x66, 0xf7, 0xd0, 0xf4]; // NOT AX
    let mut regs = Registers::default();
    regs.rax = 0x00FF; // 0000000011111111
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFF00, "AX: NOT 0x00FF = 0xFF00");
}

// ============================================================================
// 32-bit NOT
// ============================================================================

#[test]
fn test_not_eax_basic() {
    let code = [
        0xf7, 0xd0, // NOT EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xEDCBA987, "EAX: NOT 0x12345678 = 0xEDCBA987");
}

#[test]
fn test_not_eax_all_zeros() {
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFF, "EAX: NOT 0 = 0xFFFFFFFF");
}

#[test]
fn test_not_eax_pattern() {
    let code = [0xf7, 0xd0, 0xf4]; // NOT EAX
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00FF00FF, "EAX: NOT 0xFF00FF00 = 0x00FF00FF");
}

#[test]
fn test_not_ebx_register() {
    let code = [0xf7, 0xd3, 0xf4]; // NOT EBX
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x55555555, "EBX: NOT 0xAAAAAAAA = 0x55555555");
}

// ============================================================================
// 64-bit NOT
// ============================================================================

#[test]
fn test_not_rax_basic() {
    let code = [
        0x48, 0xf7, 0xd0, // NOT RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xEDCBA9876543210F, "RAX: NOT 0x123456789ABCDEF0");
}

#[test]
fn test_not_rax_all_zeros() {
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX: NOT 0 = 0xFFFF...FFFF");
}

#[test]
fn test_not_rax_all_ones() {
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX: NOT 0xFFFF...FFFF = 0");
}

#[test]
fn test_not_rax_pattern() {
    let code = [0x48, 0xf7, 0xd0, 0xf4]; // NOT RAX
    let mut regs = Registers::default();
    regs.rax = 0xFF00FF00FF00FF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x00FF00FF00FF00FF,
        "RAX: invert alternating bytes"
    );
}

#[test]
fn test_not_rbx_register() {
    let code = [0x48, 0xf7, 0xd3, 0xf4]; // NOT RBX
    let mut regs = Registers::default();
    regs.rbx = 0xF0F0F0F0F0F0F0F0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x0F0F0F0F0F0F0F0F, "RBX: NOT pattern");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_not_r8b_basic() {
    let code = [
        0x41, 0xf6, 0xd0, // NOT R8B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r8 & 0xFF, 0x55, "R8B: NOT 0xAA = 0x55");
}

#[test]
fn test_not_r9w_basic() {
    let code = [
        0x66, 0x41, 0xf7, 0xd1, // NOT R9W
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r9 & 0xFFFF, 0xEDCB, "R9W: NOT 0x1234");
}

#[test]
fn test_not_r10d_basic() {
    let code = [
        0x41, 0xf7, 0xd2, // NOT R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r10 = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r10, 0xEDCBA987, "R10D: NOT 0x12345678");
}

#[test]
fn test_not_r11_basic() {
    let code = [
        0x49, 0xf7, 0xd3, // NOT R11
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r11 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r11, 0xEDCBA9876543210F, "R11: NOT works");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_not_byte_ptr_mem() {
    let code = [
        0xf6, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0xAA);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u8(&mem);

    assert_eq!(result, 0x55, "Memory: NOT 0xAA = 0x55");
}

#[test]
fn test_not_word_ptr_mem() {
    let code = [
        0x66, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1234);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u16(&mem);

    assert_eq!(result, 0xEDCB, "Memory: NOT 0x1234 = 0xEDCB");
}

#[test]
fn test_not_dword_ptr_mem() {
    let code = [
        0xf7, 0x15, 0xfa, 0x0f, 0x00, 0x00, // NOT DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u32(&mem);

    assert_eq!(result, 0xEDCBA987, "Memory: NOT 0x12345678");
}

#[test]
fn test_not_qword_ptr_mem() {
    let code = [
        0x48, 0xf7, 0x15, 0xf9, 0x0f, 0x00, 0x00, // NOT QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);

    let _ = run_until_hlt(&mut vcpu).unwrap();
    let result = read_mem_u64(&mem);

    assert_eq!(result, 0xEDCBA9876543210F, "Memory: NOT works");
}

// ============================================================================
// Flags not affected
// ============================================================================

#[test]
fn test_not_preserves_flags() {
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0x00;
    // Set all flags
    regs.rflags = 0x2
        | flags::bits::CF
        | flags::bits::PF
        | flags::bits::AF
        | flags::bits::ZF
        | flags::bits::SF
        | flags::bits::OF;
    let initial_flags = regs.rflags;

    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rflags, initial_flags,
        "NOT should not affect any flags"
    );
}

// ============================================================================
// Double NOT is identity
// ============================================================================

#[test]
fn test_not_twice_is_identity() {
    let code = [
        0xf6, 0xd0, // NOT AL (first time)
        0xf6, 0xd0, // NOT AL (second time)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0x42, "NOT(NOT(x)) = x (identity)");
}

// ============================================================================
// Practical use cases
// ============================================================================

#[test]
fn test_not_create_mask() {
    // Create inverted mask
    let code = [0xf6, 0xd0, 0xf4]; // NOT AL
    let mut regs = Registers::default();
    regs.rax = 0x0F; // Mask for low nibble
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFF, 0xF0, "Inverted mask for high nibble");
}

#[test]
fn test_not_equivalent_to_xor_minus_one() {
    // NOT x is equivalent to x XOR -1
    let value = 0x42;

    // NOT approach
    let code_not = [0xf6, 0xd0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = value;
    let (mut vcpu, _) = setup_vm(&code_not, Some(regs));
    let regs_not = run_until_hlt(&mut vcpu).unwrap();

    // XOR approach
    let code_xor = [0x34, 0xFF, 0xf4]; // XOR AL, 0xFF
    let mut regs = Registers::default();
    regs.rax = value;
    let (mut vcpu, _) = setup_vm(&code_xor, Some(regs));
    let regs_xor = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs_not.rax & 0xFF,
        regs_xor.rax & 0xFF,
        "NOT x = x XOR 0xFF"
    );
}
