// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOVDIRI - Move Doubleword as Direct Store
// Moves the doubleword/quadword integer from a register to memory using a direct-store operation.
// Direct-stores use write combining (WC) memory type protocol and bypass the cache hierarchy.
//
// Opcodes:
// 0F 38 F9 /r             MOVDIRI m32, r32    - Move doubleword from r32 to m32 using direct store
// REX.W + 0F 38 F9 /r     MOVDIRI m64, r64    - Move quadword from r64 to m64 using direct store

// ===== 32-bit MOVDIRI Tests =====

#[test]
fn test_movdiri_m32_r32_basic() {
    // MOVDIRI [DATA_ADDR], EAX - basic 32-bit direct store
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25, // MOVDIRI [DATA_ADDR], EAX
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x12345678,
        "Memory should contain stored value"
    );
}

#[test]
fn test_movdiri_m32_r32_zero() {
    // MOVDIRI [DATA_ADDR], EAX - store zero
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_u32(&mem), 0x00000000, "Memory should contain zero");
}

#[test]
fn test_movdiri_m32_r32_all_ones() {
    // MOVDIRI [DATA_ADDR], EAX - store all ones
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xFFFFFFFF,
        "Memory should contain all ones"
    );
}

#[test]
fn test_movdiri_m32_r32_pattern() {
    // MOVDIRI [DATA_ADDR], EAX - store pattern
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAABBCCDD;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xAABBCCDD,
        "Memory should contain pattern"
    );
}

#[test]
fn test_movdiri_m32_r32_alternating() {
    // MOVDIRI [DATA_ADDR], EAX - alternating pattern
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x55AA55AA;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x55AA55AA,
        "Memory should contain alternating pattern"
    );
}

#[test]
fn test_movdiri_m32_ebx() {
    // MOVDIRI [DATA_ADDR], EBX - store from EBX
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x1c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x11223344;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x11223344,
        "Memory should contain EBX value"
    );
}

#[test]
fn test_movdiri_m32_ecx() {
    // MOVDIRI [DATA_ADDR], ECX - store from ECX
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x0c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xDEADBEEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xDEADBEEF,
        "Memory should contain ECX value"
    );
}

#[test]
fn test_movdiri_m32_edx() {
    // MOVDIRI [DATA_ADDR], EDX - store from EDX
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x14,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0xCAFEBABE;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xCAFEBABE,
        "Memory should contain EDX value"
    );
}

#[test]
fn test_movdiri_m32_esi() {
    // MOVDIRI [DATA_ADDR], ESI - store from ESI
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x34,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x01020304;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x01020304,
        "Memory should contain ESI value"
    );
}

#[test]
fn test_movdiri_m32_edi() {
    // MOVDIRI [DATA_ADDR], EDI - store from EDI
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x3c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0xFEDCBA98;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xFEDCBA98,
        "Memory should contain EDI value"
    );
}

#[test]
fn test_movdiri_m32_r8d() {
    // MOVDIRI [DATA_ADDR], R8D - store from extended register
    let code = [
        0x44,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x12345678,
        "Memory should contain R8D value"
    );
}

#[test]
fn test_movdiri_m32_r15d() {
    // MOVDIRI [DATA_ADDR], R15D - store from R15D
    let code = [
        0x44,
        0x0f,
        0x38,
        0xf9,
        0x3c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xAABBCCDD;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xAABBCCDD,
        "Memory should contain R15D value"
    );
}

// ===== 64-bit MOVDIRI Tests =====

#[test]
fn test_movdiri_m64_r64_basic() {
    // MOVDIRI [DATA_ADDR], RAX - basic 64-bit direct store
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x0123456789ABCDEF,
        "Memory should contain stored value"
    );
}

#[test]
fn test_movdiri_m64_r64_zero() {
    // MOVDIRI [DATA_ADDR], RAX - store zero
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0000000000000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x0000000000000000,
        "Memory should contain zero"
    );
}

#[test]
fn test_movdiri_m64_r64_all_ones() {
    // MOVDIRI [DATA_ADDR], RAX - store all ones
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0xFFFFFFFFFFFFFFFF,
        "Memory should contain all ones"
    );
}

#[test]
fn test_movdiri_m64_r64_pattern() {
    // MOVDIRI [DATA_ADDR], RAX - store pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFEDCBA9876543210;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0xFEDCBA9876543210,
        "Memory should contain pattern"
    );
}

#[test]
fn test_movdiri_m64_r64_alternating() {
    // MOVDIRI [DATA_ADDR], RAX - alternating pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x5555AAAA5555AAAA;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x5555AAAA5555AAAA,
        "Memory should contain alternating pattern"
    );
}

#[test]
fn test_movdiri_m64_rbx() {
    // MOVDIRI [DATA_ADDR], RBX - store from RBX
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x1c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1122334455667788;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x1122334455667788,
        "Memory should contain RBX value"
    );
}

#[test]
fn test_movdiri_m64_rcx() {
    // MOVDIRI [DATA_ADDR], RCX - store from RCX
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x0c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xDEADBEEFCAFEBABE;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0xDEADBEEFCAFEBABE,
        "Memory should contain RCX value"
    );
}

#[test]
fn test_movdiri_m64_r8() {
    // MOVDIRI [DATA_ADDR], R8 - store from extended register
    let code = [
        0x4c,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x0123456789ABCDEF,
        "Memory should contain R8 value"
    );
}

#[test]
fn test_movdiri_m64_r15() {
    // MOVDIRI [DATA_ADDR], R15 - store from R15
    let code = [
        0x4c,
        0x0f,
        0x38,
        0xf9,
        0x3c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xFEDCBA9876543210;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0xFEDCBA9876543210,
        "Memory should contain R15 value"
    );
}

// ===== Memory Alignment Tests =====

#[test]
fn test_movdiri_m32_aligned_4byte() {
    // MOVDIRI to 4-byte aligned address
    let addr = 0x2000u64; // 4-byte aligned
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (addr & 0xFF) as u8,
        ((addr >> 8) & 0xFF) as u8,
        ((addr >> 16) & 0xFF) as u8,
        ((addr >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, addr),
        0x12345678,
        "4-byte aligned store should work"
    );
}

#[test]
fn test_movdiri_m64_aligned_8byte() {
    // MOVDIRI to 8-byte aligned address
    let addr = 0x2000u64; // 8-byte aligned
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (addr & 0xFF) as u8,
        ((addr >> 8) & 0xFF) as u8,
        ((addr >> 16) & 0xFF) as u8,
        ((addr >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, addr),
        0x0123456789ABCDEF,
        "8-byte aligned store should work"
    );
}

#[test]
fn test_movdiri_m32_unaligned() {
    // MOVDIRI to unaligned address (will be split)
    let addr = 0x2001u64; // Not 4-byte aligned
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (addr & 0xFF) as u8,
        ((addr >> 8) & 0xFF) as u8,
        ((addr >> 16) & 0xFF) as u8,
        ((addr >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, addr),
        0x12345678,
        "Unaligned store should work (may be split)"
    );
}

#[test]
fn test_movdiri_m64_unaligned() {
    // MOVDIRI to unaligned address (will be split)
    let addr = 0x2001u64; // Not 8-byte aligned
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (addr & 0xFF) as u8,
        ((addr >> 8) & 0xFF) as u8,
        ((addr >> 16) & 0xFF) as u8,
        ((addr >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, addr),
        0x0123456789ABCDEF,
        "Unaligned store should work (may be split)"
    );
}

// ===== Special Value Tests =====

#[test]
fn test_movdiri_m32_max_signed() {
    // MOVDIRI with maximum signed 32-bit value
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x7FFFFFFF,
        "Memory should contain max signed 32-bit value"
    );
}

#[test]
fn test_movdiri_m32_min_signed() {
    // MOVDIRI with minimum signed 32-bit value
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x80000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x80000000,
        "Memory should contain min signed 32-bit value"
    );
}

#[test]
fn test_movdiri_m64_max_signed() {
    // MOVDIRI with maximum signed 64-bit value
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFFFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x7FFFFFFFFFFFFFFF,
        "Memory should contain max signed 64-bit value"
    );
}

#[test]
fn test_movdiri_m64_min_signed() {
    // MOVDIRI with minimum signed 64-bit value
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x8000000000000000,
        "Memory should contain min signed 64-bit value"
    );
}

#[test]
fn test_movdiri_preserves_source_register() {
    // MOVDIRI should not modify source register
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x12345678, "EAX should be unchanged");
}

#[test]
fn test_movdiri_preserves_other_registers() {
    // MOVDIRI should not affect other registers
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x2222222222222222, "RCX should be unchanged");
}

#[test]
fn test_movdiri_does_not_modify_flags() {
    // MOVDIRI does not modify any flags
    let code = [
        0x0f,
        0x38,
        0xf9,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // Only reserved bit 1
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}
