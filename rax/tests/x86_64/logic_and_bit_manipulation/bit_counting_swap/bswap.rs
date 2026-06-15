use crate::common::*;
use rax::cpu::Registers;

// BSWAP - Byte Swap
// Reverses the byte order of a 32-bit or 64-bit register.
// For 32-bit: bytes 0,1,2,3 become 3,2,1,0
// For 64-bit: bytes 0-7 become 7-0
// 16-bit operands are undefined (typically zero the register or leave it unchanged).
//
// Opcodes:
// 0F C8+rd    BSWAP r32    - Reverse byte order of r32
// REX.W 0F C8+rd BSWAP r64 - Reverse byte order of r64

#[test]
fn test_bswap_eax() {
    // BSWAP EAX
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78563412,
        "EAX bytes should be reversed"
    );
}

#[test]
fn test_bswap_ebx() {
    // BSWAP EBX
    let code = [
        0x0f, 0xcb, // BSWAP EBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAABBCCDD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0xDDCCBBAA,
        "EBX bytes should be reversed"
    );
}

#[test]
fn test_bswap_ecx() {
    // BSWAP ECX
    let code = [
        0x0f, 0xc9, // BSWAP ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x01020304;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0x04030201,
        "ECX bytes should be reversed"
    );
}

#[test]
fn test_bswap_edx() {
    // BSWAP EDX
    let code = [
        0x0f, 0xca, // BSWAP EDX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rdx & 0xFFFFFFFF,
        0xEFBEADDE,
        "EDX bytes should be reversed"
    );
}

#[test]
fn test_bswap_rax_64bit() {
    // BSWAP RAX (64-bit)
    let code = [
        0x48, 0x0f, 0xc8, // BSWAP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xEFCDAB8967452301, "RAX bytes should be reversed");
}

#[test]
fn test_bswap_rbx_64bit() {
    // BSWAP RBX (64-bit)
    let code = [
        0x48, 0x0f, 0xcb, // BSWAP RBX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1032547698BADCFE, "RBX bytes should be reversed");
}

#[test]
fn test_bswap_all_zeros() {
    // BSWAP with all zeros
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should remain zero");
}

#[test]
fn test_bswap_all_ones() {
    // BSWAP with all ones
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "EAX should remain all ones"
    );
}

#[test]
fn test_bswap_alternating_bytes() {
    // BSWAP with alternating byte pattern
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA55AA55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x55AA55AA,
        "EAX bytes should be reversed"
    );
}

#[test]
fn test_bswap_sequential_bytes() {
    // BSWAP with sequential byte values
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00010203;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x03020100,
        "EAX bytes should be reversed"
    );
}

#[test]
fn test_bswap_with_extended_registers() {
    // BSWAP R8D
    let code = [
        0x41, 0x0f, 0xc8, // BSWAP R8D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x11223344;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0x44332211,
        "R8D bytes should be reversed"
    );
}

#[test]
fn test_bswap_r9d() {
    // BSWAP R9D
    let code = [
        0x41, 0x0f, 0xc9, // BSWAP R9D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xABCDEF01;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r9 & 0xFFFFFFFF,
        0x01EFCDAB,
        "R9D bytes should be reversed"
    );
}

#[test]
fn test_bswap_r15_64bit() {
    // BSWAP R15 (64-bit)
    let code = [
        0x49, 0x0f, 0xcf, // BSWAP R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x0011223344556677;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.r15, 0x7766554433221100, "R15 bytes should be reversed");
}

#[test]
fn test_bswap_idempotent() {
    // Double BSWAP returns to original
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0x0f, 0xc8, // BSWAP EAX again
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should return to original after double BSWAP"
    );
}

#[test]
fn test_bswap_endianness_conversion() {
    // BSWAP for big-endian to little-endian conversion
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000; // Big-endian representation
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000080,
        "Endianness should be converted"
    );
}

#[test]
fn test_bswap_preserves_other_registers() {
    // BSWAP should not affect other registers
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0xAABBCCDD;
    regs.rcx = 0x11111111;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0xAABBCCDD, "EBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x11111111, "ECX should be unchanged");
}

#[test]
fn test_bswap_single_byte_values() {
    // Test with only one byte set
    let test_cases = vec![
        (0x000000FF, 0xFF000000),
        (0x0000FF00, 0x00FF0000),
        (0x00FF0000, 0x0000FF00),
        (0xFF000000, 0x000000FF),
    ];

    for (input, expected) in test_cases {
        let code = [
            0x0f, 0xc8, // BSWAP EAX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BSWAP(0x{:08X}) should be 0x{:08X}",
            input,
            expected
        );
    }
}

#[test]
fn test_bswap_network_byte_order() {
    // Network byte order (big-endian) to host byte order (little-endian)
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000100; // Network byte order for 256
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00010000,
        "Should convert network to host byte order"
    );
}

#[test]
fn test_bswap_64bit_symmetric() {
    // Symmetric 64-bit value
    let code = [
        0x48, 0x0f, 0xc8, // BSWAP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0102030404030201;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0102030404030201,
        "Symmetric pattern should equal itself when swapped"
    );
}

#[test]
fn test_bswap_64bit_asymmetric() {
    // Asymmetric 64-bit value
    let code = [
        0x48, 0x0f, 0xc8, // BSWAP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0102030405060708;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0807060504030201, "RAX bytes should be reversed");
}

#[test]
fn test_bswap_high_low_word_swap() {
    // BSWAP effectively swaps high and low words with byte reversal
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12340000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00003412,
        "High word moves to low with byte swap"
    );
}

#[test]
fn test_bswap_ascii_to_reversed() {
    // BSWAP with ASCII-like values
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x41424344; // "ABCD" in ASCII
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x44434241,
        "ASCII bytes should be reversed"
    );
}

#[test]
fn test_bswap_powers_of_256() {
    // Test with powers of 256
    let test_cases = vec![
        (0x00000001, 0x01000000), // 256^0
        (0x00000100, 0x00010000), // 256^1
        (0x00010000, 0x00000100), // 256^2
        (0x01000000, 0x00000001), // 256^3
    ];

    for (input, expected) in test_cases {
        let code = [
            0x0f, 0xc8, // BSWAP EAX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = input;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BSWAP(0x{:08X}) should be 0x{:08X}",
            input,
            expected
        );
    }
}

#[test]
fn test_bswap_sequential_registers() {
    // BSWAP multiple registers in sequence
    let code = [
        0x0f, 0xc8, // BSWAP EAX
        0x0f, 0xcb, // BSWAP EBX
        0x0f, 0xc9, // BSWAP ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x11111111;
    regs.rbx = 0x22222222;
    regs.rcx = 0x33333333;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x11111111, "EAX should be swapped");
    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x22222222, "EBX should be swapped");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 0x33333333, "ECX should be swapped");
}

#[test]
fn test_bswap_64bit_upper_lower_independence() {
    // Upper and lower 32 bits swap independently
    let code = [
        0x48, 0x0f, 0xc8, // BSWAP RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678_9ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xF0DEBC9A_78563412,
        "Both halves should be byte-swapped and position-swapped"
    );
}
