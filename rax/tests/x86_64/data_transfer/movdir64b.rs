// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOVDIR64B - Move 64 Bytes as Direct Store
// Moves 64 bytes from source memory address to destination memory address using direct-store.
// The destination address is specified in a register and must be 64-byte aligned.
// Direct-stores use write combining (WC) memory type protocol and bypass the cache hierarchy.
//
// Opcodes:
// 66 0F 38 F8 /r    MOVDIR64B r16/r32/r64, m512    - Move 64 bytes from m512 to address in register

// Source address for 64-byte data
const SRC_ADDR: u64 = 0x3000;
// Destination address (must be 64-byte aligned)
const DST_ADDR: u64 = 0x4000;

// ===== Basic MOVDIR64B Tests =====

#[test]
fn test_movdir64b_basic() {
    // MOVDIR64B RAX, [RSI] - move 64 bytes from [RSI] to address in RAX
    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, // MOVDIR64B RAX, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write 64 bytes of test data at source
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify all 64 bytes were copied
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_all_zeros() {
    // MOVDIR64B with all zeros
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Source is already zeros, just verify
    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            0,
            "Byte {} should be zero",
            i
        );
    }
}

#[test]
fn test_movdir64b_all_ones() {
    // MOVDIR64B with all ones
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write 64 bytes of 0xFF
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, 0xFF);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            0xFF,
            "Byte {} should be 0xFF",
            i
        );
    }
}

#[test]
fn test_movdir64b_alternating_pattern() {
    // MOVDIR64B with alternating byte pattern
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write alternating pattern
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, if i % 2 == 0 { 0xAA } else { 0x55 });
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        let expected = if i % 2 == 0 { 0xAA } else { 0x55 };
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            expected,
            "Byte {} should be {}",
            i,
            expected
        );
    }
}

#[test]
fn test_movdir64b_sequential_bytes() {
    // MOVDIR64B with sequential byte values
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write sequential values
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, (i * 4) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            (i * 4) as u8,
            "Byte {} should match",
            i
        );
    }
}

// ===== Different Register Tests =====

#[test]
fn test_movdir64b_with_rbx() {
    // MOVDIR64B RBX, [RSI] - destination in RBX
    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x1e, // MOVDIR64B RBX, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_with_rcx() {
    // MOVDIR64B RCX, [RSI] - destination in RCX
    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x0e, // MOVDIR64B RCX, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, (i ^ 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            (i ^ 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_with_rdx() {
    // MOVDIR64B RDX, [RSI] - destination in RDX
    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x16, // MOVDIR64B RDX, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, ((i * 3) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            ((i * 3) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_with_r8() {
    // MOVDIR64B R8, [RSI] - destination in extended register
    let code = [
        0x66, 0x4c, 0x0f, 0x38, 0xf8, 0x06, // MOVDIR64B R8, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_with_r15() {
    // MOVDIR64B R15, [RSI] - destination in R15
    let code = [
        0x66, 0x4c, 0x0f, 0x38, 0xf8, 0x3e, // MOVDIR64B R15, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, ((i * 2) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            ((i * 2) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_source_in_rbx() {
    // MOVDIR64B RAX, [RBX] - source in RBX
    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B RAX, [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rbx = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_source_in_r9() {
    // MOVDIR64B RAX, [R9] - source in extended register
    let code = [
        0x66, 0x49, 0x0f, 0x38, 0xf8, 0x01, // MOVDIR64B RAX, [R9]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.r9 = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, ((i + 10) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            ((i + 10) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

// ===== Alignment Tests =====

#[test]
fn test_movdir64b_aligned_64byte() {
    // MOVDIR64B with 64-byte aligned destination
    let aligned_dst = 0x4000u64; // 64-byte aligned
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = aligned_dst;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, aligned_dst + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_aligned_128byte() {
    // MOVDIR64B with 128-byte aligned destination (also 64-byte aligned)
    let aligned_dst = 0x4080u64; // 128-byte aligned
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = aligned_dst;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, ((i * 5) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, aligned_dst + i),
            ((i * 5) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_source_alignment_not_required() {
    // Source address does not need to be aligned
    let unaligned_src = 0x3001u64; // Not aligned
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = unaligned_src;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, unaligned_src + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be copied",
            i
        );
    }
}

// ===== Data Integrity Tests =====

#[test]
fn test_movdir64b_8byte_values() {
    // Test with 8 qwords (64 bytes)
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write 8 different 64-bit values
    for i in 0..8 {
        let value = 0x0011223344556677u64 + (i * 0x1111111111111111);
        write_mem_at_u64(&mem, SRC_ADDR + (i * 8), value);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        let expected = 0x0011223344556677u64 + (i * 0x1111111111111111);
        assert_eq!(
            read_mem_at_u64(&mem, DST_ADDR + (i * 8)),
            expected,
            "Qword {} should match",
            i
        );
    }
}

#[test]
fn test_movdir64b_16byte_values() {
    // Test with 4 128-bit values (represented as pairs of 64-bit)
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write 4 pairs of 64-bit values (simulating 128-bit values)
    for i in 0..4 {
        let value_low = 0xFEDCBA9876543210u64 + i;
        let value_high = 0x0123456789ABCDEFu64 + i;
        write_mem_at_u64(&mem, SRC_ADDR + (i * 16), value_low);
        write_mem_at_u64(&mem, SRC_ADDR + (i * 16) + 8, value_high);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        let expected_low = 0xFEDCBA9876543210u64 + i;
        let expected_high = 0x0123456789ABCDEFu64 + i;
        assert_eq!(
            read_mem_at_u64(&mem, DST_ADDR + (i * 16)),
            expected_low,
            "Low qword of pair {} should match",
            i
        );
        assert_eq!(
            read_mem_at_u64(&mem, DST_ADDR + (i * 16) + 8),
            expected_high,
            "High qword of pair {} should match",
            i
        );
    }
}

#[test]
fn test_movdir64b_mixed_data() {
    // Test with mixed data patterns
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write mixed patterns
    write_mem_at_u64(&mem, SRC_ADDR + 0, 0x0000000000000000);
    write_mem_at_u64(&mem, SRC_ADDR + 8, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, SRC_ADDR + 16, 0xAAAAAAAAAAAAAAAA);
    write_mem_at_u64(&mem, SRC_ADDR + 24, 0x5555555555555555);
    write_mem_at_u64(&mem, SRC_ADDR + 32, 0x0123456789ABCDEF);
    write_mem_at_u64(&mem, SRC_ADDR + 40, 0xFEDCBA9876543210);
    write_mem_at_u64(&mem, SRC_ADDR + 48, 0x1111111111111111);
    write_mem_at_u64(&mem, SRC_ADDR + 56, 0x8888888888888888);

    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 0), 0x0000000000000000);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 8), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 16), 0xAAAAAAAAAAAAAAAA);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 24), 0x5555555555555555);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 32), 0x0123456789ABCDEF);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 40), 0xFEDCBA9876543210);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 48), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 56), 0x8888888888888888);
}

#[test]
fn test_movdir64b_ascii_data() {
    // Test with ASCII-like data
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write ASCII string (64 characters)
    let test_string = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!?";
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, test_string[i as usize]);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            test_string[i as usize],
            "Byte {} should match",
            i
        );
    }
}

// ===== Edge Case Tests =====

#[test]
fn test_movdir64b_preserves_registers() {
    // MOVDIR64B should not modify registers
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, DST_ADDR, "RAX should be unchanged");
    assert_eq!(regs.rsi, SRC_ADDR, "RSI should be unchanged");
    assert_eq!(regs.rbx, 0x1111111111111111, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x2222222222222222, "RCX should be unchanged");
}

#[test]
fn test_movdir64b_does_not_modify_flags() {
    // MOVDIR64B does not modify any flags
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    regs.rflags = 0x2; // Only reserved bit 1
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

#[test]
fn test_movdir64b_same_cacheline() {
    // Test moving data within same cache line range
    let src = 0x3000u64;
    let dst = 0x3100u64; // Different but nearby
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = dst;
    regs.rsi = src;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, src + i, ((i * 7) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, dst + i),
            ((i * 7) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_high_memory() {
    // Test with higher memory addresses
    let src = 0x10000u64;
    let dst = 0x20000u64;
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = dst;
    regs.rsi = src;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, src + i, ((i + 100) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, dst + i),
            ((i + 100) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_sequential_operations() {
    // Multiple MOVDIR64B operations in sequence
    let src1 = 0x3000u64;
    let dst1 = 0x4000u64;
    let src2 = 0x3100u64;
    let dst2 = 0x4100u64;

    let code = [
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, // MOVDIR64B RAX, [RSI] - first operation
        0x66, 0x48, 0x0f, 0x38, 0xf8, 0x19, // MOVDIR64B RBX, [RCX] - second operation
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = dst1;
    regs.rsi = src1;
    regs.rbx = dst2;
    regs.rcx = src2;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Set up first source
    for i in 0..64 {
        write_mem_at_u8(&mem, src1 + i, i as u8);
    }

    // Set up second source
    for i in 0..64 {
        write_mem_at_u8(&mem, src2 + i, (63 - i) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify first destination
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, dst1 + i),
            i as u8,
            "First dst byte {} should be copied",
            i
        );
    }

    // Verify second destination
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, dst2 + i),
            (63 - i) as u8,
            "Second dst byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_overwrite_existing_data() {
    // Test that MOVDIR64B overwrites existing data
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Pre-fill destination with different data
    for i in 0..64 {
        write_mem_at_u8(&mem, DST_ADDR + i, 0xFF);
    }

    // Set source data
    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify source data overwrote destination
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            i as u8,
            "Byte {} should be overwritten",
            i
        );
    }
}

#[test]
fn test_movdir64b_byte_order_preserved() {
    // Verify that byte order is preserved (no endianness conversion)
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write specific 64-bit values to check byte order
    write_mem_at_u64(&mem, SRC_ADDR + 0, 0x0102030405060708);
    write_mem_at_u64(&mem, SRC_ADDR + 8, 0x090A0B0C0D0E0F10);
    write_mem_at_u64(&mem, SRC_ADDR + 16, 0x1112131415161718);
    write_mem_at_u64(&mem, SRC_ADDR + 24, 0x191A1B1C1D1E1F20);
    write_mem_at_u64(&mem, SRC_ADDR + 32, 0x2122232425262728);
    write_mem_at_u64(&mem, SRC_ADDR + 40, 0x292A2B2C2D2E2F30);
    write_mem_at_u64(&mem, SRC_ADDR + 48, 0x3132333435363738);
    write_mem_at_u64(&mem, SRC_ADDR + 56, 0x393A3B3C3D3E3F40);

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify exact byte order is preserved
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 0), 0x0102030405060708);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 8), 0x090A0B0C0D0E0F10);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 16), 0x1112131415161718);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 24), 0x191A1B1C1D1E1F20);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 32), 0x2122232425262728);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 40), 0x292A2B2C2D2E2F30);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 48), 0x3132333435363738);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 56), 0x393A3B3C3D3E3F40);
}

#[test]
fn test_movdir64b_32bit_register() {
    // MOVDIR64B EAX, [RSI] - using 32-bit register for destination
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x06, // MOVDIR64B EAX, [RSI]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, ((i * 11) & 0xFF) as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            ((i * 11) & 0xFF) as u8,
            "Byte {} should be copied",
            i
        );
    }
}

#[test]
fn test_movdir64b_power_of_two_pattern() {
    // Test with power of two pattern
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, SRC_ADDR + (i * 8), 1u64 << i);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, DST_ADDR + (i * 8)),
            1u64 << i,
            "Qword {} should match power of 2 pattern",
            i
        );
    }
}

#[test]
fn test_movdir64b_negative_signed_values() {
    // Test with negative signed values
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write negative signed values (high bit set)
    write_mem_at_u64(&mem, SRC_ADDR + 0, 0xFFFFFFFFFFFFFFFF);
    write_mem_at_u64(&mem, SRC_ADDR + 8, 0xFFFFFFFFFFFFFFFE);
    write_mem_at_u64(&mem, SRC_ADDR + 16, 0x8000000000000000);
    write_mem_at_u64(&mem, SRC_ADDR + 24, 0x8000000000000001);
    write_mem_at_u64(&mem, SRC_ADDR + 32, 0xDEADBEEFCAFEBABE);
    write_mem_at_u64(&mem, SRC_ADDR + 40, 0xBADC0FFEE0DDF00D);
    write_mem_at_u64(&mem, SRC_ADDR + 48, 0x9999999999999999);
    write_mem_at_u64(&mem, SRC_ADDR + 56, 0x8888888888888888);

    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 0), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 8), 0xFFFFFFFFFFFFFFFE);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 16), 0x8000000000000000);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 24), 0x8000000000000001);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 32), 0xDEADBEEFCAFEBABE);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 40), 0xBADC0FFEE0DDF00D);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 48), 0x9999999999999999);
    assert_eq!(read_mem_at_u64(&mem, DST_ADDR + 56), 0x8888888888888888);
}

#[test]
fn test_movdir64b_partial_overlap_prevention() {
    // Test that source and destination don't overlap
    let src = 0x3000u64;
    let dst = 0x3080u64; // 128 bytes apart, no overlap with 64-byte blocks
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = dst;
    regs.rsi = src;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..64 {
        write_mem_at_u8(&mem, src + i, i as u8);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify destination
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, dst + i),
            i as u8,
            "Dst byte {} should be copied",
            i
        );
    }

    // Verify source is unchanged
    for i in 0..64 {
        assert_eq!(
            read_mem_at_u8(&mem, src + i),
            i as u8,
            "Src byte {} should be unchanged",
            i
        );
    }
}

#[test]
fn test_movdir64b_cache_line_crossing() {
    // Test data that crosses cache line boundaries (64-byte blocks)
    let code = [0x66, 0x48, 0x0f, 0x38, 0xf8, 0x06, 0xf4];
    let mut regs = Registers::default();
    regs.rax = DST_ADDR;
    regs.rsi = SRC_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Create pattern that would be obvious if cache lines were mixed
    for i in 0..32 {
        write_mem_at_u8(&mem, SRC_ADDR + i, 0xAA);
    }
    for i in 32..64 {
        write_mem_at_u8(&mem, SRC_ADDR + i, 0x55);
    }

    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Verify first half
    for i in 0..32 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            0xAA,
            "First half byte {} should be 0xAA",
            i
        );
    }
    // Verify second half
    for i in 32..64 {
        assert_eq!(
            read_mem_at_u8(&mem, DST_ADDR + i),
            0x55,
            "Second half byte {} should be 0x55",
            i
        );
    }
}
