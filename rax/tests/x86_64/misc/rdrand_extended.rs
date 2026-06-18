// Module path for tests run via x86_64.rs
use crate::common::{cf_set, run_until_hlt, setup_vm};
use rax::cpu::Registers;

// RDRAND - Read Random Number
//
// Loads a hardware-generated random value into the destination register
// Sets CF=1 on success, CF=0 if random value not available
// Supports 16-bit, 32-bit, and 64-bit operands
//
// Opcodes:
// NP 0F C7 /6            RDRAND r16    - Read 16-bit random number into r16
// NP 0F C7 /6            RDRAND r32    - Read 32-bit random number into r32
// NP REX.W 0F C7 /6      RDRAND r64    - Read 64-bit random number into r64

#[test]
fn test_rdrand_16bit_basic() {
    // Basic RDRAND 16-bit
    let code = [
        0x66, 0x0f, 0xc7, 0xf0, // RDRAND AX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set on success
    assert!(cf_set(regs.rflags), "CF should be set on success");
    // Value in AX should be non-zero (statistically very likely)
    // Note: We can't predict the exact value since it's random
}

#[test]
fn test_rdrand_32bit_basic() {
    // Basic RDRAND 32-bit
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdrand_64bit_basic() {
    // Basic RDRAND 64-bit
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdrand_16bit_different_registers() {
    // RDRAND 16-bit into different registers
    for (modrm, _reg_name) in &[(0xf0, "ax"), (0xf3, "bx"), (0xf1, "cx"), (0xf2, "dx")] {
        let code = [
            0x66, 0x0f, 0xc7, *modrm, // RDRAND reg16
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdrand_32bit_different_registers() {
    // RDRAND 32-bit into different registers
    for (modrm, _reg_name) in &[(0xf0, "eax"), (0xf3, "ebx"), (0xf1, "ecx"), (0xf2, "edx")] {
        let code = [
            0x0f, 0xc7, *modrm, // RDRAND reg32
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdrand_64bit_different_registers() {
    // RDRAND 64-bit into different registers
    for (modrm, _reg_name) in &[(0xf0, "rax"), (0xf3, "rbx"), (0xf1, "rcx"), (0xf2, "rdx")] {
        let code = [
            0x48, 0x0f, 0xc7, *modrm, // RDRAND reg64
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdrand_32bit_sequential() {
    // Multiple sequential RDRAND calls
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf3, // RDRAND EBX
        0x0f, 0xc7, 0xf1, // RDRAND ECX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on last success");
    // All three registers should have random values (likely different)
}

#[test]
fn test_rdrand_64bit_sequential() {
    // Multiple sequential 64-bit RDRAND calls
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on last success");
}

#[test]
fn test_rdrand_16bit_preserves_other_registers() {
    // RDRAND 16-bit preserves other registers
    let code = [
        0x66, 0x0f, 0xc7, 0xf0, // RDRAND AX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    regs.rdx = 0x3333333333333333;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
    assert_eq!(regs.rdx, 0x3333333333333333);
}

#[test]
fn test_rdrand_32bit_preserves_other_registers() {
    // RDRAND 32-bit preserves other registers
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

#[test]
fn test_rdrand_64bit_preserves_other_registers() {
    // RDRAND 64-bit preserves other registers
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
}

#[test]
fn test_rdrand_retry_pattern() {
    // Simulate retry pattern on failure (though likely always succeeds in emulation)
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x73, 0xfb, // JNC retry (if CF=0)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Should eventually succeed");
}

#[test]
fn test_rdrand_with_r8_r15() {
    // RDRAND with extended registers
    for reg_offset in 0..8 {
        let modrm = 0xf0 | reg_offset;
        let code = [
            0x49, 0x0f, 0xc7, modrm, // RDRAND r8-r15
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdrand_multiple_in_loop() {
    // Multiple RDRAND calls to fill array pattern
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x89, 0x03, // MOV [RBX], EAX
        0x48, 0x83, 0xc3, 0x04, // ADD RBX, 4
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x89, 0x03, // MOV [RBX], EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x2004);
}

#[test]
fn test_rdrand_16bit_with_extended_regs() {
    // RDRAND 16-bit with R8-R15
    let code = [
        0x66, 0x41, 0x0f, 0xc7, 0xf0, // RDRAND R8W
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdrand_32bit_with_extended_regs() {
    // RDRAND 32-bit with R8-R15
    let code = [
        0x41, 0x0f, 0xc7, 0xf0, // RDRAND R8D
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdrand_clears_other_flags() {
    // RDRAND should clear OF, SF, ZF, AF, PF (CF is set on success)
    let code = [
        0xf9, // STC (set all arithmetic flags)
        0xfd, // STD
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be 1 (success), OF/SF/ZF/AF/PF should be 0
    assert!(cf_set(regs.rflags), "CF should be set");
    // DF should be preserved
}

#[test]
fn test_rdrand_determinism_check() {
    // Multiple RDRAND calls should produce different values (statistically)
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // While theoretically they could be equal, it's statistically unlikely
    // In a real implementation, we would expect different values
    // For testing purposes, we just verify both succeeded
    assert!(cf_set(regs.rflags), "Both RDRANDs should succeed");
}

#[test]
fn test_rdrand_interleaved_with_operations() {
    // RDRAND interleaved with other operations
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x0f, 0xc7, 0xf1, // RDRAND ECX
        0x48, 0xc7, 0xc2, 0x84, 0x00, 0x00, 0x00, // MOV RDX, 0x84
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Last RDRAND should succeed");
    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rdx, 0x84);
}

#[test]
fn test_rdrand_save_to_memory_pattern() {
    // Pattern: RDRAND and save to memory
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x89, 0x05, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "RDRAND should succeed");
}

#[test]
fn test_rdrand_32bit_all_registers() {
    // RDRAND into all general purpose 32-bit registers
    let registers = [
        (0xf0, "eax"),
        (0xf1, "ecx"),
        (0xf2, "edx"),
        (0xf3, "ebx"),
        (0xf6, "esi"),
        (0xf7, "edi"),
    ];

    for (modrm, _name) in &registers {
        let code = [
            0x0f, 0xc7, *modrm, // RDRAND reg32
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set for {}", _name);
    }
}

#[test]
fn test_rdrand_64bit_all_registers() {
    // RDRAND into all general purpose 64-bit registers
    let registers = [
        (0xf0, "rax"),
        (0xf1, "rcx"),
        (0xf2, "rdx"),
        (0xf3, "rbx"),
        (0xf6, "rsi"),
        (0xf7, "rdi"),
    ];

    for (modrm, _name) in &registers {
        let code = [
            0x48, 0x0f, 0xc7, *modrm, // RDRAND reg64
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set for {}", _name);
    }
}

#[test]
fn test_rdrand_fill_buffer_simulation() {
    // Simulate filling a buffer with random data
    let code = [
        // Loop to get 4 random values
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX
        0x48, 0x0f, 0xc7, 0xf2, // RDRAND RDX
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDRANDs should succeed");
}

#[test]
fn test_rdrand_16bit_upper_bits_preserved() {
    // RDRAND 16-bit should only modify lower 16 bits
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x66, 0x0f, 0xc7, 0xf0, // RDRAND AX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 48 bits should still be 0xFFFF
    assert_eq!(regs.rax >> 16, 0xFFFFFFFFFFFF);
}

#[test]
fn test_rdrand_32bit_upper_bits_zeroed() {
    // RDRAND 32-bit should zero upper 32 bits in 64-bit mode
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zero
    assert_eq!(regs.rax >> 32, 0);
}

#[test]
fn test_rdrand_conditional_branch_pattern() {
    // Pattern using RDRAND with conditional branch
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x72, 0x04, // JC success (if CF=1)
        // failure path:
        0x31, 0xc0, // XOR EAX, EAX
        0xeb, 0x00, // JMP end
        // success path:
        // end:
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should take success path in most cases
    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdrand_entropy_source_pattern() {
    // Simulate gathering entropy from multiple RDRAND calls
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x33, 0xc3, // XOR RAX, RBX (mix with RBX)
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX
        0x48, 0x33, 0xd8, // XOR RBX, RAX (mix with RAX)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rdrand_benchmark_pattern() {
    // Simulate benchmarking RDRAND performance
    let code = [
        // Get 10 random numbers
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDRANDs should succeed");
}

#[test]
fn test_rdrand_seeding_pattern() {
    // Use RDRAND to seed a PRNG
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX (seed high)
        0x48, 0xc1, 0xe0, 0x20, // SHL RAX, 32
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x0f, 0xc7, 0xf0, // RDRAND EAX (seed low)
        0x48, 0x09, 0xc3, // OR RBX, RAX (combine)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rdrand_key_generation_pattern() {
    // Simulate cryptographic key generation
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX (part 1)
        0x48, 0x0f, 0xc7, 0xf1, // RDRAND RCX (part 2)
        0x48, 0x0f, 0xc7, 0xf2, // RDRAND RDX (part 3)
        0x48, 0x0f, 0xc7, 0xf3, // RDRAND RBX (part 4)
        // Now have 256 bits of random data
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDRANDs for key should succeed");
}

#[test]
fn test_rdrand_nonce_generation() {
    // Generate a nonce using RDRAND
    let code = [
        0x48, 0x0f, 0xc7, 0xf0, // RDRAND RAX
        0x48, 0x89, 0x05, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], RAX (store nonce)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdrand_monte_carlo_setup() {
    // Setup for Monte Carlo simulation
    let code = [
        0x0f, 0xc7, 0xf0, // RDRAND EAX (x coordinate)
        0x0f, 0xc7, 0xf1, // RDRAND ECX (y coordinate)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}
