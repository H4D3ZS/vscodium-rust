use crate::common::*;
use rax::cpu::Registers;

// BZHI - Zero High Bits Starting with Specified Bit Position (BMI2)
// Zeroes all bits in the source operand starting from the bit position specified
// in the second source operand. Bits below the index are copied to the destination.
// The index is taken from bits [7:0] of the second source (modulo operand size).
// Sets CF if the index is greater than or equal to the operand size, clears ZF if result is non-zero.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F5 /r   BZHI r32, r/m32, r32   - Zero high bits starting with bit index
// VEX.NDS.LZ.0F38.W1 F5 /r   BZHI r64, r/m64, r64   - Zero high bits starting with bit index

#[test]
fn test_bzhi_eax_ebx_ecx_index_0() {
    // BZHI EAX, EBX, ECX - zero all bits (index 0)
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0; // index 0 - zero all bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be zero (all bits masked)"
    );
    assert!(zf_set(regs.rflags), "ZF should be set (result is zero)");
    assert!(!cf_set(regs.rflags), "CF should be clear (index < 32)");
}

#[test]
fn test_bzhi_eax_ebx_ecx_index_8() {
    // BZHI EAX, EBX, ECX - keep lower 8 bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 8; // index 8 - keep bits 0-7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78,
        "EAX should contain lower 8 bits"
    );
    assert!(
        !zf_set(regs.rflags),
        "ZF should be clear (result is non-zero)"
    );
    assert!(!cf_set(regs.rflags), "CF should be clear");
}

#[test]
fn test_bzhi_eax_ebx_ecx_index_16() {
    // BZHI EAX, EBX, ECX - keep lower 16 bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 16; // index 16 - keep bits 0-15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x5678,
        "EAX should contain lower 16 bits"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bzhi_eax_ebx_ecx_index_32() {
    // BZHI EAX, EBX, ECX - index 32 keeps all bits
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 32; // index 32 - keep all 32 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should contain all bits"
    );
    assert!(!zf_set(regs.rflags), "ZF should be clear");
    assert!(cf_set(regs.rflags), "CF should be set (index >= 32)");
}

#[test]
fn test_bzhi_eax_ebx_ecx_index_beyond() {
    // BZHI EAX, EBX, ECX - index beyond operand size
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 64; // index 64 (only bits [7:0] used, so 64 % 256 = 64)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should contain all bits"
    );
    assert!(cf_set(regs.rflags), "CF should be set (index >= 32)");
}

#[test]
fn test_bzhi_rax_rbx_rcx_index_8() {
    // BZHI RAX, RBX, RCX - 64-bit version, keep lower 8 bits
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX (W1)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 8; // index 8 - keep bits 0-7
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xF0, "RAX should contain lower 8 bits");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bzhi_rax_rbx_rcx_index_32() {
    // BZHI RAX, RBX, RCX - 64-bit, keep lower 32 bits
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 32; // index 32 - keep bits 0-31
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x9ABCDEF0, "RAX should contain lower 32 bits");
    assert!(!zf_set(regs.rflags), "ZF should be clear");
}

#[test]
fn test_bzhi_rax_rbx_rcx_index_64() {
    // BZHI RAX, RBX, RCX - index 64 keeps all bits
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    regs.rcx = 64; // index 64 - keep all 64 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x123456789ABCDEF0, "RAX should contain all bits");
    assert!(cf_set(regs.rflags), "CF should be set (index >= 64)");
}

#[test]
fn test_bzhi_zero_result() {
    // BZHI that produces zero result
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 0; // index 0 - zero everything
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "EAX should be zero");
    assert!(zf_set(regs.rflags), "ZF should be set");
}

#[test]
fn test_bzhi_with_extended_registers() {
    // BZHI R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x28, 0xf5, 0xc1, // BZHI R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0xABCDEF01;
    regs.r10 = 12; // index 12 - keep bits 0-11
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = 0xABCDEF01 & ((1u32 << 12) - 1);
    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        expected as u64,
        "R8D should contain lower 12 bits"
    );
}

#[test]
fn test_bzhi_mem32() {
    // BZHI EAX, [mem], ECX
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BZHI EAX, [DATA_ADDR], ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 8; // index 8
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0xAABBCCDD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDD,
        "EAX should contain lower 8 bits from memory"
    );
}

#[test]
fn test_bzhi_mem64() {
    // BZHI RAX, [mem], RCX
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0x04, 0x25, 0x00, 0x20, 0x00,
        0x00, // BZHI RAX, [DATA_ADDR], RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 16; // index 16
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = 0x0123456789ABCDEF & ((1u64 << 16) - 1);
    assert_eq!(
        regs.rax, expected,
        "RAX should contain lower 16 bits from memory"
    );
}

#[test]
fn test_bzhi_mask_creation() {
    // BZHI can create bit masks
    for index in 0..=32 {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        let expected = if index >= 32 {
            0xFFFFFFFF
        } else {
            ((1u64 << index) - 1) & 0xFFFFFFFF
        };
        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "Should create {}-bit mask",
            index
        );
    }
}

#[test]
fn test_bzhi_nibble_mask() {
    // Create 4-bit mask (nibble)
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 4; // keep lower nibble
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x8,
        "EAX should contain lower nibble"
    );
}

#[test]
fn test_bzhi_preserves_source() {
    // BZHI should not modify source operand
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678;
    regs.rcx = 16;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx & 0xFFFFFFFF, 0x12345678, "EBX should be unchanged");
    assert_eq!(regs.rcx & 0xFFFFFFFF, 16, "ECX should be unchanged");
}

#[test]
fn test_bzhi_index_modulo_behavior() {
    // Index uses only bits [7:0], so 256 wraps to 0
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 256; // wraps to 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0,
        "EAX should be zero (index wraps to 0)"
    );
}

#[test]
fn test_bzhi_alternating_pattern() {
    // BZHI with alternating pattern
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xAAAAAAAA; // 1010...1010
    regs.rcx = 16; // keep lower 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAA,
        "EAX should contain lower 16 bits of pattern"
    );
}

#[test]
fn test_bzhi_single_bit() {
    // Keep only single bit
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFF;
    regs.rcx = 1; // keep only bit 0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 1, "EAX should contain only bit 0");
}

#[test]
fn test_bzhi_sparse_bits() {
    // BZHI with sparse bit pattern
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x80001001; // bits 0, 12, 31 set
    regs.rcx = 16; // keep bits 0-15
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1001,
        "EAX should contain bits 0 and 12 only"
    );
}

#[test]
fn test_bzhi_field_extraction() {
    // Extract various bit fields
    let test_cases = vec![
        (0x12345678, 4, 0x8),       // lower nibble
        (0x12345678, 8, 0x78),      // lower byte
        (0x12345678, 12, 0x678),    // 12 bits
        (0x12345678, 20, 0x45678),  // 20 bits
        (0x12345678, 24, 0x345678), // 24 bits
    ];

    for (value, index, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = value;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            regs.rax & 0xFFFFFFFF,
            expected,
            "BZHI(0x{:X}, {}) should be 0x{:X}",
            value,
            index,
            expected
        );
    }
}

#[test]
fn test_bzhi_64bit_high_index() {
    // BZHI with high index in 64-bit mode
    let code = [
        0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    regs.rcx = 48; // keep lower 48 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (1u64 << 48) - 1;
    assert_eq!(regs.rax, expected, "RAX should contain lower 48 bits");
}

#[test]
fn test_bzhi_cf_boundary_32bit() {
    // Test CF flag at boundary (index = operand size)
    let test_cases = vec![
        (31, false), // index 31 < 32, CF clear
        (32, true),  // index 32 >= 32, CF set
        (33, true),  // index 33 >= 32, CF set
    ];

    for (index, expect_cf) in test_cases {
        let code = [
            0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        if expect_cf {
            assert!(cf_set(regs.rflags), "CF should be set for index {}", index);
        } else {
            assert!(
                !cf_set(regs.rflags),
                "CF should be clear for index {}",
                index
            );
        }
    }
}

#[test]
fn test_bzhi_cf_boundary_64bit() {
    // Test CF flag at boundary in 64-bit mode
    let test_cases = vec![
        (63, false), // index 63 < 64, CF clear
        (64, true),  // index 64 >= 64, CF set
        (65, true),  // index 65 >= 64, CF set
    ];

    for (index, expect_cf) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf0, 0xf5, 0xc3, // BZHI RAX, RBX, RCX
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0xFFFFFFFFFFFFFFFF;
        regs.rcx = index;
        let (mut vcpu, _) = setup_vm(&code, Some(regs));
        let regs = run_until_hlt(&mut vcpu).unwrap();

        if expect_cf {
            assert!(cf_set(regs.rflags), "CF should be set for index {}", index);
        } else {
            assert!(
                !cf_set(regs.rflags),
                "CF should be clear for index {}",
                index
            );
        }
    }
}

#[test]
fn test_bzhi_combines_with_shifts() {
    // BZHI can extract middle bits when combined with shifts
    let code = [
        0xc4, 0xe2, 0x70, 0xf5, 0xc3, // BZHI EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x12345678 >> 8; // shift right by 8 first
    regs.rcx = 16; // then keep 16 bits
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let expected = (0x12345678 >> 8) & 0xFFFF;
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        expected as u64,
        "Should extract bits 8-23"
    );
}
