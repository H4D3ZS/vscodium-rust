use crate::common::*;
use rax::cpu::{Registers, VCpu};

// MUL — Unsigned Multiply
//
// Opcodes:
// - F6 /4       MUL r/m8      AX := AL * r/m8
// - F7 /4       MUL r/m16     DX:AX := AX * r/m16
// - F7 /4       MUL r/m32     EDX:EAX := EAX * r/m32
// - REX.W+F7 /4 MUL r/m64     RDX:RAX := RAX * r/m64
//
// Operation: For 8-bit:  AX := AL * r/m8
//            For 16-bit: DX:AX := AX * r/m16
//            For 32-bit: EDX:EAX := EAX * r/m32
//            For 64-bit: RDX:RAX := RAX * r/m64
//
// Flags: CF and OF are set if the result is nonzero in the upper half of the destination
//        SF, ZF, AF, PF are undefined
//
// CRITICAL: MUL works with UNSIGNED integers, unlike IMUL which is signed

// ============================================================================
// 8-bit MUL (opcode F6 /4)
// ============================================================================

#[test]
fn test_mul_al_small() {
    let code = [
        0xf6, 0xe3, // MUL BL (F6 /4, ModRM=11_100_011)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 5; // AL = 5
    regs.rbx = 3; // BL = 3
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 15, "5 * 3 = 15");
    assert!(!cf_set(regs.rflags), "CF should be clear (fits in AL)");
    assert!(!of_set(regs.rflags), "OF should be clear");
}

#[test]
fn test_mul_al_max() {
    // 255 * 255 = 65025 (0xFE01) - upper byte not zero
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 255;
    regs.rbx = 255;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFE01, "255 * 255 = 65025 (0xFE01)");
    assert!(cf_set(regs.rflags), "CF should be set (result in AH)");
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_mul_al_fits_in_byte() {
    // 10 * 15 = 150 (0x96), fits in AX with AH=0
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rbx = 15;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 150, "10 * 15 = 150");
    assert!(
        !cf_set(regs.rflags),
        "CF should be clear (result fits in AL)"
    );
}

#[test]
fn test_mul_al_zero() {
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "0 * 100 = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_al_one() {
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 100, "1 * 100 = 100");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// 16-bit MUL (opcode F7 /4 with 0x66 prefix)
// ============================================================================

#[test]
fn test_mul_ax_small() {
    let code = [
        0x66, 0xf7, 0xe3, // MUL BX (66 F7 /4)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 5000, "AX: 100 * 50 = 5000");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0 (upper is zero)");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_ax_overflow() {
    // 65535 * 65535 = 0xFFFE0001
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    regs.rbx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0001, "AX (low word)");
    assert_eq!(regs.rdx & 0xFFFF, 0xFFFE, "DX (high word, non-zero)");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
    assert!(of_set(regs.rflags), "OF should be set");
}

#[test]
fn test_mul_ax_fits() {
    // 1000 * 50 = 50000 (fits in 16 bits)
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 50;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 50000, "1000 * 50 = 50000");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_ax_zero() {
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0, "0 * 100 = 0");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// 32-bit MUL (opcode F7 /4)
// ============================================================================

#[test]
fn test_mul_eax_small() {
    let code = [
        0xf7, 0xe3, // MUL EBX (F7 /4)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000, "EAX: 1000 * 2000 = 2000000");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_eax_overflow() {
    // 0xFFFFFFFF * 0xFFFFFFFF
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rbx = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x00000001, "EAX (low)");
    assert_eq!(regs.rdx, 0xFFFFFFFE, "EDX (high)");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_mul_eax_fits() {
    // 100000 * 50000 = 5000000000 (fits in 32 bits? No, exceeds)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 100000;
    regs.rbx = 50000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 5000000000 = 0x12A05F200, so EDX has upper bits
    assert!(cf_set(regs.rflags), "CF should be set (result > 32-bit)");
}

#[test]
fn test_mul_eax_small_product() {
    // 100 * 200 = 20000 (fits in 32 bits)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 200;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 20000, "100 * 200 = 20000");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_eax_zero() {
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 1000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 * 1000000 = 0");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// 64-bit MUL (opcode REX.W + F7 /4)
// ============================================================================

#[test]
fn test_mul_rax_small() {
    let code = [
        0x48, 0xf7, 0xe3, // MUL RBX (REX.W F7 /4)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 1000000;
    regs.rbx = 2000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000000000, "RAX: 1M * 2M");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_rax_overflow() {
    // 0xFFFFFFFFFFFFFFFF * 0xFFFFFFFFFFFFFFFF
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000001, "RAX (low)");
    assert_eq!(regs.rdx, 0xFFFFFFFFFFFFFFFE, "RDX (high)");
    assert!(cf_set(regs.rflags), "CF should be set (overflow)");
}

#[test]
fn test_mul_rax_large_product() {
    // 2^32 * 2^32 = 2^64 = 0x1_00000000_00000000 as 128-bit
    // Split into RDX:RAX -> RDX = 1, RAX = 0
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 0x0000000100000000; // 2^32
    regs.rbx = 0x0000000100000000; // 2^32
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX (low 64 bits = 0)");
    assert_eq!(regs.rdx, 0x0000000000000001, "RDX (high 64 bits = 1)");
    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_mul_rax_fits() {
    // 1000 * 2000 = 2000000 (fits in 64 bits)
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000, "1000 * 2000 = 2000000");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_rax_zero() {
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "0 * max = 0");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_rax_one() {
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    let mut regs = Registers::default();
    regs.rax = 1;
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "1 * x = x");
    assert_eq!(regs.rdx, 0, "RDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_mul_cl_register() {
    // MUL CL (8-bit)
    let code = [0xf6, 0xe1, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 20;
    regs.rcx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 200, "20 * 10 = 200");
}

#[test]
fn test_mul_dx_16bit() {
    // MUL DX (16-bit): DX:AX = AX * DX
    // 1000 * 100 = 100000 = 0x000186A0
    // DX = 0x0001, AX = 0x86A0
    let code = [0x66, 0xf7, 0xe2, 0xf4]; // MUL DX
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rdx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let result = ((regs.rdx & 0xFFFF) << 16) | (regs.rax & 0xFFFF);
    assert_eq!(result, 100000, "1000 * 100 = 100000 (in DX:AX)");
}

#[test]
fn test_mul_ecx_32bit() {
    // MUL ECX (32-bit)
    let code = [0xf7, 0xe1, 0xf4]; // MUL ECX
    let mut regs = Registers::default();
    regs.rax = 100000;
    regs.rcx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10000000, "100000 * 100 = 10000000");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_mul_r8b() {
    let code = [0x41, 0xf6, 0xe0, 0xf4]; // MUL R8B
    let mut regs = Registers::default();
    regs.rax = 25;
    regs.r8 = 4;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 100, "25 * 4 = 100");
}

#[test]
fn test_mul_r10d() {
    let code = [0x41, 0xf7, 0xe2, 0xf4]; // MUL R10D
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.r10 = 5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5000000, "1000 * 5000 = 5000000");
}

#[test]
fn test_mul_r15() {
    let code = [0x49, 0xf7, 0xe7, 0xf4]; // MUL R15
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.r15 = 200;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 20000, "100 * 200 = 20000");
    assert_eq!(regs.rdx, 0, "RDX = 0");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_mul_byte_ptr_mem() {
    let code = [
        0xf6, 0x25, 0xfa, 0x0f, 0x00, 0x00, // MUL BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 25);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 4;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 100, "4 * 25 = 100");
}

#[test]
fn test_mul_word_ptr_mem() {
    let code = [
        0x66, 0xf7, 0x25, 0xf9, 0x0f, 0x00, 0x00, // MUL WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 1000);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 50;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 50000, "50 * 1000 = 50000");
    assert_eq!(regs.rdx & 0xFFFF, 0, "DX = 0");
}

#[test]
fn test_mul_dword_ptr_mem() {
    let code = [
        0xf7, 0x25, 0xfa, 0x0f, 0x00, 0x00, // MUL DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100000);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 100;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10000000, "100 * 100000 = 10000000");
}

#[test]
fn test_mul_qword_ptr_mem() {
    let code = [
        0x48, 0xf7, 0x25, 0xf9, 0x0f, 0x00, 0x00, // MUL QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 1000000);

    // Get current regs to preserve RIP, then modify
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 2000000;
    vcpu.set_regs(&regs).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 2000000000000, "2000000 * 1000000");
}

// ============================================================================
// Comparison: MUL vs IMUL with unsigned values
// ============================================================================

#[test]
fn test_mul_vs_imul_unsigned() {
    // For unsigned values, MUL and IMUL should produce the same lower 32 bits
    // but CF/OF flags may differ

    // MUL: 100 * 200 = 20000
    let code_mul = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 200;
    let (mut vcpu, _) = setup_vm(&code_mul, Some(regs));
    let regs_mul = run_until_hlt(&mut vcpu).unwrap();

    // IMUL (two-operand): EBX = EBX * EAX
    let code_imul = [0x0f, 0xaf, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rbx = 200;
    let (mut vcpu, _) = setup_vm(&code_imul, Some(regs));
    let regs_imul = run_until_hlt(&mut vcpu).unwrap();

    // Both should have same product in lower part
    assert_eq!(regs_mul.rax, regs_imul.rbx, "Products should match");
    assert_eq!(regs_mul.rax, 20000, "Product is 20000");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_mul_powers_of_two() {
    // MUL EAX by powers of 2
    // 1000 * 256 = 256000
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 1000;
    regs.rbx = 256;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 256000, "1000 * 256 = 256000");
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_mul_boundary_values() {
    // Test boundary: result just fits in 32 bits
    // 65536 * 65535 = 4294836224 (just under 2^32)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut regs = Registers::default();
    regs.rax = 65536;
    regs.rbx = 65535;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // 65536 * 65535 = 0xFFFF0000
    assert_eq!(regs.rax, 0xFFFF0000, "Result");
    // Upper part is zero
    assert_eq!(regs.rdx, 0, "EDX = 0");
    assert!(!cf_set(regs.rflags), "CF should be clear");
}
