// Module path for tests run via x86_64.rs
use crate::common::{
    DATA_ADDR, read_mem_at_u64, run_until_hlt, setup_vm, setup_vm_no_idt, write_mem_at_u64,
};
use rax::cpu::{Registers, VCpu, VcpuExit};

// MOVDIR64B - Move 64 Bytes as Direct Store
//
// Moves 64 bytes from source memory to destination memory as a direct store
// The source operand is a memory location (specified by a general-purpose register)
// The destination is specified by ES:destination register
// Performs a 64-byte direct store operation (cache-line sized)
//
// Opcode:
// 66 0F 38 F8 /r         MOVDIR64B r16/r32/r64, m512    - Move 64 bytes from m512 to address in r16/r32/r64

#[test]
fn test_movdir64b_basic() {
    // Basic MOVDIR64B - move 64 bytes
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000; // Destination
    regs.rbx = 0x2000; // Source
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write 64 bytes of test data at source
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0x1111111111111111 * (i as u64 + 1));
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify all 64 bytes were copied
    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            0x1111111111111111 * (i as u64 + 1)
        );
    }
}

#[test]
fn test_movdir64b_aligned_addresses() {
    // Test with cache-line aligned addresses
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x06, // MOVDIR64B rax, [rsi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x4000; // 64-byte aligned destination
    regs.rsi = 0x2000; // 64-byte aligned source
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0xAAAAAAAAAAAAAAAA + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x4000 + i * 8),
            0xAAAAAAAAAAAAAAAA + i as u64
        );
    }
}

#[test]
fn test_movdir64b_different_registers() {
    // Test with different register combinations
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x11, // MOVDIR64B rdx, [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x5000;
    regs.rcx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        let value = 0xBBBBBBBBBBBBBBBBu64.wrapping_mul(i as u64 + 1);
        write_mem_at_u64(&mem, 0x2000 + i * 8, value);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        let value = 0xBBBBBBBBBBBBBBBBu64.wrapping_mul(i as u64 + 1);
        assert_eq!(read_mem_at_u64(&mem, 0x5000 + i * 8), value);
    }
}

#[test]
fn test_movdir64b_with_displacement() {
    // MOVDIR64B with displacement
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x43, 0x40, // MOVDIR64B rax, [rbx + 0x40]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Source at 0x2040
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2040 + i * 8, 0x1234567890ABCDEF - i as u64 * 0x100);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            0x1234567890ABCDEF - i as u64 * 0x100
        );
    }
}

#[test]
fn test_movdir64b_preserves_registers() {
    // MOVDIR64B preserves source and destination registers
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    regs.rcx = 0x1234567890ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3000);
    assert_eq!(regs.rbx, 0x2000);
    assert_eq!(regs.rcx, 0x1234567890ABCDEF);
}

#[test]
fn test_movdir64b_no_flags_modified() {
    // MOVDIR64B doesn't modify flags
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_movdir64b_sequential_operations() {
    // Multiple sequential MOVDIR64B operations
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0x66, 0x0f, 0x38, 0xf8, 0x0a, // MOVDIR64B rcx, [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    regs.rcx = 0x4000;
    regs.rdx = 0x2080;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Set up two different source blocks
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0xAAAAAAAAAAAAAAAA + i as u64);
        write_mem_at_u64(&mem, 0x2080 + i * 8, 0xBBBBBBBBBBBBBBBB + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify both blocks were copied correctly
    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            0xAAAAAAAAAAAAAAAA + i as u64
        );
        assert_eq!(
            read_mem_at_u64(&mem, 0x4000 + i * 8),
            0xBBBBBBBBBBBBBBBB + i as u64
        );
    }
}

#[test]
fn test_movdir64b_zero_data() {
    // MOVDIR64B with all zeros
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), 0);
    }
}

#[test]
fn test_movdir64b_all_ones() {
    // MOVDIR64B with all ones
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0xFFFFFFFFFFFFFFFF);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), 0xFFFFFFFFFFFFFFFF);
    }
}

#[test]
fn test_movdir64b_pattern_data() {
    // MOVDIR64B with various patterns
    let patterns = [
        0x0123456789ABCDEF,
        0xFEDCBA9876543210,
        0xAAAAAAAAAAAAAAAA,
        0x5555555555555555,
        0xF0F0F0F0F0F0F0F0,
        0x0F0F0F0F0F0F0F0F,
        0x00FF00FF00FF00FF,
        0xFF00FF00FF00FF00,
    ];

    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, patterns[i as usize]);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), patterns[i as usize]);
    }
}

#[test]
fn test_movdir64b_overlapping_check() {
    // Ensure source is not affected (non-overlapping regions)
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0x1000 + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify source unchanged
    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x2000 + i * 8), 0x1000 + i as u64);
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), 0x1000 + i as u64);
    }
}

#[test]
fn test_movdir64b_with_r8() {
    // MOVDIR64B with R8 extended register
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B r8, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x5000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0x8888888888888888 + i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x5000 + i * 8),
            0x8888888888888888 + i as u64
        );
    }
    assert_eq!(regs.r8, 0x5000);
}

#[test]
fn test_movdir64b_increment_pattern() {
    // MOVDIR64B with incrementing data pattern
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, i as u64 * 0x0101010101010101);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            i as u64 * 0x0101010101010101
        );
    }
}

#[test]
fn test_movdir64b_high_memory() {
    // MOVDIR64B with higher memory addresses
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x200000;
    regs.rbx = 0x100000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x100000 + i * 8, 0xCAFEBABE00000000 + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x200000 + i * 8),
            0xCAFEBABE00000000 + i as u64
        );
    }
}

#[test]
fn test_movdir64b_loop_simulation() {
    // Simulate copying multiple cache lines in a loop
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0x48, 0x83, 0xc0, 0x40, // ADD rax, 0x40
        0x48, 0x83, 0xc3, 0x40, // ADD rbx, 0x40
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Set up two source blocks
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0x1000 + i as u64);
        write_mem_at_u64(&mem, 0x2040 + i * 8, 0x2000 + i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify both blocks copied
    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), 0x1000 + i as u64);
        assert_eq!(read_mem_at_u64(&mem, 0x3040 + i * 8), 0x2000 + i as u64);
    }
    assert_eq!(regs.rax, 0x3040);
    assert_eq!(regs.rbx, 0x2040);
}

#[test]
fn test_movdir64b_alternating_pattern() {
    // MOVDIR64B with alternating bit pattern
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        let pattern = if i % 2 == 0 {
            0xAAAAAAAAAAAAAAAA
        } else {
            0x5555555555555555
        };
        write_mem_at_u64(&mem, 0x2000 + i * 8, pattern);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        let expected = if i % 2 == 0 {
            0xAAAAAAAAAAAAAAAA
        } else {
            0x5555555555555555
        };
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), expected);
    }
}

#[test]
fn test_movdir64b_boundary_crossing() {
    // MOVDIR64B crossing page boundaries
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2FC0; // Close to 4K boundary
    regs.rbx = 0x1FC0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x1FC0 + i * 8, 0xBEEFBEEFBEEFBEEF + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x2FC0 + i * 8),
            0xBEEFBEEFBEEFBEEF + i as u64
        );
    }
}

#[test]
fn test_movdir64b_preserves_all_registers() {
    // Verify MOVDIR64B doesn't modify unrelated registers
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    regs.rcx = 0x1111111111111111;
    regs.rdx = 0x2222222222222222;
    regs.rsi = 0x3333333333333333;
    regs.rdi = 0x4444444444444444;
    regs.r8 = 0x5555555555555555;
    regs.r9 = 0x6666666666666666;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, i as u64);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x3000);
    assert_eq!(regs.rbx, 0x2000);
    assert_eq!(regs.rcx, 0x1111111111111111);
    assert_eq!(regs.rdx, 0x2222222222222222);
    assert_eq!(regs.rsi, 0x3333333333333333);
    assert_eq!(regs.rdi, 0x4444444444444444);
    assert_eq!(regs.r8, 0x5555555555555555);
    assert_eq!(regs.r9, 0x6666666666666666);
}

#[test]
fn test_movdir64b_unique_values() {
    // Each qword has unique value
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let unique_values = [
        0x0011223344556677,
        0x8899AABBCCDDEEFF,
        0xFEDCBA9876543210,
        0x0123456789ABCDEF,
        0xDEADBEEFCAFEBABE,
        0x1234567890ABCDEF,
        0xFFEEDDCCBBAA9988,
        0x7766554433221100,
    ];

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, unique_values[i as usize]);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            unique_values[i as usize]
        );
    }
}

#[test]
fn test_movdir64b_sib_addressing() {
    // MOVDIR64B with SIB addressing
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x04, 0x19, // MOVDIR64B rax, [rcx + rbx*1]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rcx = 0x2000;
    regs.rbx = 0x40;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Source at 0x2040
    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2040 + i * 8, 0x9999999999999999 + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            0x9999999999999999 + i as u64
        );
    }
}

#[test]
fn test_movdir64b_first_last_qword() {
    // Ensure first and last qwords are copied correctly
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Use actual marker values
    write_mem_at_u64(&mem, 0x2000, 0xAAAAAAAAAAAAAAAA);
    write_mem_at_u64(&mem, 0x2038, 0xBBBBBBBBBBBBBBBB);
    for i in 1..7 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, i as u64);
    }
    write_mem_at_u64(&mem, 0x2038, 0xBBBBBBBBBBBBBBBB);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x3000), 0xAAAAAAAAAAAAAAAA);
    assert_eq!(read_mem_at_u64(&mem, 0x3038), 0xBBBBBBBBBBBBBBBB);
}

#[test]
fn test_movdir64b_sequential_bytes() {
    // Sequential byte pattern across 64 bytes
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        let val = ((i as u64) << 56)
            | ((i as u64) << 48)
            | ((i as u64) << 40)
            | ((i as u64) << 32)
            | ((i as u64) << 24)
            | ((i as u64) << 16)
            | ((i as u64) << 8)
            | (i as u64);
        write_mem_at_u64(&mem, 0x2000 + i * 8, val);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        let expected = ((i as u64) << 56)
            | ((i as u64) << 48)
            | ((i as u64) << 40)
            | ((i as u64) << 32)
            | ((i as u64) << 24)
            | ((i as u64) << 16)
            | ((i as u64) << 8)
            | (i as u64);
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), expected);
    }
}

#[test]
fn test_movdir64b_prime_numbers() {
    // Use prime numbers as test data
    let primes = [2u64, 3, 5, 7, 11, 13, 17, 19];

    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(
            &mem,
            0x2000 + i * 8,
            primes[i as usize] * 0x0101010101010101,
        );
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            primes[i as usize] * 0x0101010101010101
        );
    }
}

#[test]
fn test_movdir64b_power_of_two() {
    // Powers of 2 as test data
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 1u64 << (i * 8));
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), 1u64 << (i * 8));
    }
}

#[test]
fn test_movdir64b_negative_values() {
    // Test with values that would be negative if interpreted as signed
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, 0x8000000000000000 + i as u64);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(
            read_mem_at_u64(&mem, 0x3000 + i * 8),
            0x8000000000000000 + i as u64
        );
    }
}

#[test]
fn test_movdir64b_mixed_endianness_patterns() {
    // Various byte order patterns
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3000;
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let patterns = [
        0x0102030405060708,
        0x0807060504030201,
        0x1122334455667788,
        0x8877665544332211,
        0xAABBCCDDEEFF0011,
        0x1100FFEEDDCCBBAA,
        0x0F1E2D3C4B5A6978,
        0x78695A4B3C2D1E0F,
    ];

    for i in 0..8 {
        write_mem_at_u64(&mem, 0x2000 + i * 8, patterns[i as usize]);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..8 {
        assert_eq!(read_mem_at_u64(&mem, 0x3000 + i * 8), patterns[i as usize]);
    }
}

// ============================================================================
// MOVDIR64B - Architecturally invalid encodings (must fault, not abort the VM)
// ============================================================================

// MOVDIR64B requires the destination (memory) operand to be 64-byte aligned.
// A misaligned destination must raise #GP(0) instead of aborting the emulator.
// We detect the injected fault using the no-IDT harness (mirroring
// tests/x86_64/misc/ud.rs): with no IDT entries populated, exception delivery
// fails fast instead of reaching HLT.
#[test]
fn test_movdir64b_misaligned_dest_raises_gp() {
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0x03, // MOVDIR64B rax, [rbx]
        0xf4, // HLT (must not be reached)
    ];
    let mut regs = Registers::default();
    regs.rax = 0x3001; // Destination NOT 64-byte aligned -> #GP(0)
    regs.rbx = 0x2000; // Source
    let (mut vcpu, _mem) = setup_vm_no_idt(&code, Some(regs));

    // The guest must not be able to kill the emulator: a misaligned destination
    // should inject #GP(0) rather than reaching HLT.
    let result = vcpu.run();
    match result {
        Ok(VcpuExit::Hlt) => panic!("misaligned MOVDIR64B must raise #GP(0), not reach HLT"),
        Ok(VcpuExit::Shutdown) => {} // #GP injected (no handler) -> shutdown
        Err(_) => {}                 // #GP injected, IDT entry not present -> Err (no abort)
        _ => {}                      // other non-HLT exit is acceptable
    }
}

// MOVDIR64B with a register source (ModRM.mod = 11) is an invalid encoding and
// must raise #UD rather than aborting the emulator.
#[test]
fn test_movdir64b_register_source_raises_ud() {
    let code = [
        0x66, 0x0f, 0x38, 0xf8, 0xc3, // MOVDIR64B rax, rbx (ModRM.mod=11 -> #UD)
        0xf4, // HLT (must not be reached)
    ];
    let (mut vcpu, _mem) = setup_vm_no_idt(&code, None);

    let result = vcpu.run();
    match result {
        Ok(VcpuExit::Hlt) => panic!("MOVDIR64B with register source must raise #UD, not reach HLT"),
        Ok(VcpuExit::Shutdown) => {}
        Err(_) => {}
        _ => {}
    }
}
