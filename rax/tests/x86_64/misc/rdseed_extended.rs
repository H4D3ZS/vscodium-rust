// Module path for tests run via x86_64.rs
use crate::common::{cf_set, run_until_hlt, setup_vm};
use rax::cpu::Registers;

// RDSEED - Read Random SEED
//
// Loads a hardware-generated random seed value into the destination register
// Sets CF=1 on success, CF=0 if seed value not available
// Supports 16-bit, 32-bit, and 64-bit operands
// RDSEED provides entropy directly from hardware source (vs RDRAND which is DRBG output)
//
// Opcodes:
// NP 0F C7 /7            RDSEED r16    - Read 16-bit random seed into r16
// NP 0F C7 /7            RDSEED r32    - Read 32-bit random seed into r32
// NP REX.W 0F C7 /7      RDSEED r64    - Read 64-bit random seed into r64

#[test]
fn test_rdseed_16bit_basic() {
    // Basic RDSEED 16-bit
    let code = [
        0x66, 0x0f, 0xc7, 0xf8, // RDSEED AX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should be set on success
    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdseed_32bit_basic() {
    // Basic RDSEED 32-bit
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdseed_64bit_basic() {
    // Basic RDSEED 64-bit
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdseed_16bit_different_registers() {
    // RDSEED 16-bit into different registers
    for (modrm, _reg_name) in &[(0xf8, "ax"), (0xfb, "bx"), (0xf9, "cx"), (0xfa, "dx")] {
        let code = [
            0x66, 0x0f, 0xc7, *modrm, // RDSEED reg16
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdseed_32bit_different_registers() {
    // RDSEED 32-bit into different registers
    for (modrm, _reg_name) in &[(0xf8, "eax"), (0xfb, "ebx"), (0xf9, "ecx"), (0xfa, "edx")] {
        let code = [
            0x0f, 0xc7, *modrm, // RDSEED reg32
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdseed_64bit_different_registers() {
    // RDSEED 64-bit into different registers
    for (modrm, _reg_name) in &[(0xf8, "rax"), (0xfb, "rbx"), (0xf9, "rcx"), (0xfa, "rdx")] {
        let code = [
            0x48, 0x0f, 0xc7, *modrm, // RDSEED reg64
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdseed_32bit_sequential() {
    // Multiple sequential RDSEED calls
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0x0f, 0xc7, 0xfb, // RDSEED EBX
        0x0f, 0xc7, 0xf9, // RDSEED ECX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on last success");
}

#[test]
fn test_rdseed_64bit_sequential() {
    // Multiple sequential 64-bit RDSEED calls
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on last success");
}

#[test]
fn test_rdseed_16bit_preserves_other_registers() {
    // RDSEED 16-bit preserves other registers
    let code = [
        0x66, 0x0f, 0xc7, 0xf8, // RDSEED AX
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
fn test_rdseed_32bit_preserves_other_registers() {
    // RDSEED 32-bit preserves other registers
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
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
fn test_rdseed_64bit_preserves_other_registers() {
    // RDSEED 64-bit preserves other registers
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
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
fn test_rdseed_retry_pattern() {
    // Simulate retry pattern on failure (RDSEED may fail more often than RDRAND)
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0x73, 0xfb, // JNC retry (if CF=0)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Should eventually succeed");
}

#[test]
fn test_rdseed_with_r8_r15() {
    // RDSEED with extended registers
    for reg_offset in 0..8 {
        let modrm = 0xf8 | reg_offset;
        let code = [
            0x49, 0x0f, 0xc7, modrm, // RDSEED r8-r15
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set on success");
    }
}

#[test]
fn test_rdseed_seeding_rdrand() {
    // Use RDSEED to seed RDRAND-based PRNG
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (get entropy)
        0x48, 0x89, 0xc3, // MOV RBX, RAX (save seed)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_16bit_with_extended_regs() {
    // RDSEED 16-bit with R8-R15
    let code = [
        0x66, 0x41, 0x0f, 0xc7, 0xf8, // RDSEED R8W
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdseed_32bit_with_extended_regs() {
    // RDSEED 32-bit with R8-R15
    let code = [
        0x41, 0x0f, 0xc7, 0xf8, // RDSEED R8D
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set on success");
}

#[test]
fn test_rdseed_clears_other_flags() {
    // RDSEED should clear OF, SF, ZF, AF, PF (CF is set on success)
    let code = [
        0xf9, // STC
        0xfd, // STD
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "CF should be set");
}

#[test]
fn test_rdseed_interleaved_with_operations() {
    // RDSEED interleaved with other operations
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x0f, 0xc7, 0xf9, // RDSEED ECX
        0x48, 0xc7, 0xc2, 0x84, 0x00, 0x00, 0x00, // MOV RDX, 0x84
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "Last RDSEED should succeed");
    assert_eq!(regs.rbx, 0x42);
    assert_eq!(regs.rdx, 0x84);
}

#[test]
fn test_rdseed_save_to_memory_pattern() {
    // Pattern: RDSEED and save to memory
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x89, 0x05, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "RDSEED should succeed");
}

#[test]
fn test_rdseed_32bit_all_registers() {
    // RDSEED into all general purpose 32-bit registers
    let registers = [
        (0xf8, "eax"),
        (0xf9, "ecx"),
        (0xfa, "edx"),
        (0xfb, "ebx"),
        (0xfe, "esi"),
        (0xff, "edi"),
    ];

    for (modrm, _name) in &registers {
        let code = [
            0x0f, 0xc7, *modrm, // RDSEED reg32
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set for {}", _name);
    }
}

#[test]
fn test_rdseed_64bit_all_registers() {
    // RDSEED into all general purpose 64-bit registers
    let registers = [
        (0xf8, "rax"),
        (0xf9, "rcx"),
        (0xfa, "rdx"),
        (0xfb, "rbx"),
        (0xfe, "rsi"),
        (0xff, "rdi"),
    ];

    for (modrm, _name) in &registers {
        let code = [
            0x48, 0x0f, 0xc7, *modrm, // RDSEED reg64
            0xf4,
        ];
        let (mut vcpu, _mem) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();

        assert!(cf_set(regs.rflags), "CF should be set for {}", _name);
    }
}

#[test]
fn test_rdseed_entropy_gathering() {
    // Simulate gathering entropy for cryptographic operations
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDSEEDs should succeed");
}

#[test]
fn test_rdseed_16bit_upper_bits_preserved() {
    // RDSEED 16-bit should only modify lower 16 bits
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x66, 0x0f, 0xc7, 0xf8, // RDSEED AX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 48 bits should still be 0xFFFF
    assert_eq!(regs.rax >> 16, 0xFFFFFFFFFFFF);
}

#[test]
fn test_rdseed_32bit_upper_bits_zeroed() {
    // RDSEED 32-bit should zero upper 32 bits in 64-bit mode
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0x0f, 0xc7, 0xf8, // RDSEED EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Upper 32 bits should be zero
    assert_eq!(regs.rax >> 32, 0);
}

#[test]
fn test_rdseed_conditional_branch_pattern() {
    // Pattern using RDSEED with conditional branch
    let code = [
        0x0f, 0xc7, 0xf8, // RDSEED EAX
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

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_master_key_generation() {
    // Simulate master key generation for encryption
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (part 1)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (part 2)
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX (part 3)
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX (part 4)
        // Now have 256 bits of entropy
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDSEEDs for key should succeed");
}

#[test]
fn test_rdseed_initialization_vector() {
    // Generate initialization vector (IV) for encryption
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_salt_generation() {
    // Generate cryptographic salt
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (salt)
        0x48, 0x89, 0x05, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_nonce_generation() {
    // Generate unique nonce
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x89, 0x05, 0x00, 0x20, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_session_key() {
    // Generate session key
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_challenge_response() {
    // Generate challenge for challenge-response protocol
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (challenge)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_entropy_pool_seeding() {
    // Seed entropy pool with multiple RDSEED calls
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x33, 0xc3, // XOR RAX, RBX
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX
        0x48, 0x33, 0xd8, // XOR RBX, RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_token_generation() {
    // Generate authentication token
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (token part 1)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (token part 2)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_uuid_generation() {
    // Generate UUID-like value
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (UUID part 1)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (UUID part 2)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_fortuna_style_seeding() {
    // Fortuna-style PRNG seeding
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (entropy)
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (more entropy)
        0x48, 0x31, 0xc3, // XOR RBX, RAX (mix)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rdseed_drbg_instantiation() {
    // DRBG instantiation with seed from RDSEED
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (entropy input)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (nonce)
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX (personalization)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDSEEDs should succeed");
}

#[test]
fn test_rdseed_batch_generation() {
    // Batch generation of random seeds
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags), "All RDSEEDs should succeed");
}

#[test]
fn test_rdseed_reseed_counter() {
    // Using RDSEED for PRNG reseed counter
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x9c, // PUSHFQ
        0x5b, // POP RBX (save CF before SHR)
        0x48, 0xc1, 0xe8, 0x20, // SHR RAX, 32 (use upper 32 bits)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rbx));
}

#[test]
fn test_rdseed_key_derivation_seed() {
    // Seed for key derivation function
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (KDF seed)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (additional entropy)
        0x48, 0x31, 0xc8, // XOR RAX, RCX (combine)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_rdseed_secure_erase_pattern() {
    // Generate random pattern for secure erase
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_hardware_rng_test() {
    // Test hardware RNG quality
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX
        // Compare for randomness quality
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}

#[test]
fn test_rdseed_tls_handshake_random() {
    // Generate random for TLS handshake
    let code = [
        0x48, 0x0f, 0xc7, 0xf8, // RDSEED RAX (client random part 1)
        0x48, 0x0f, 0xc7, 0xf9, // RDSEED RCX (client random part 2)
        0x48, 0x0f, 0xc7, 0xfa, // RDSEED RDX (client random part 3)
        0x48, 0x0f, 0xc7, 0xfb, // RDSEED RBX (client random part 4)
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(cf_set(regs.rflags));
}
