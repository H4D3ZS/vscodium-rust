// Module path for tests run via x86_64.rs
use crate::common::{
    read_mem_at_u8, read_mem_at_u32, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u8,
    write_mem_at_u32, write_mem_at_u64,
};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// CLFLUSH - Flush Cache Line
//
// Invalidates the cache line containing the specified memory address
// from all levels of the cache hierarchy.
// If the cache line contains modified data, it is written back to memory.
// Ordered with respect to other CLFLUSH, writes, locked operations, and fences.
//
// Opcode:
// NP 0F AE /7            CLFLUSH m8            - Flush cache line containing m8

#[test]
fn test_clflush_basic() {
    // Basic CLFLUSH with memory operand [rax]
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory
    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Memory should still contain the data (just flushed from cache)
    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x42,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_with_displacement() {
    // CLFLUSH with displacement [rax + 0x10]
    let code = [
        0x0f, 0xae, 0x78, 0x10, // CLFLUSH [rax + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory at rax + 0x10
    write_mem_at_u32(&mem, 0x2010, 0xDEADBEEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2010),
        0xDEADBEEF,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_with_negative_displacement() {
    // CLFLUSH with negative displacement [rax - 0x10]
    let code = [
        0x0f, 0xae, 0x78, 0xf0, // CLFLUSH [rax - 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2100;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory at rax - 0x10
    write_mem_at_u64(&mem, 0x20f0, 0x1234567890ABCDEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x20f0),
        0x1234567890ABCDEF,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2100, "RAX should be unchanged");
}

#[test]
fn test_clflush_rbx_base() {
    // CLFLUSH using RBX as base [rbx]
    let code = [
        0x0f, 0xae, 0x3b, // CLFLUSH [rbx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x3000, 0xAB);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x3000),
        0xAB,
        "Memory should still contain data"
    );
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
}

#[test]
fn test_clflush_rcx_base() {
    // CLFLUSH using RCX as base [rcx]
    let code = [
        0x0f, 0xae, 0x39, // CLFLUSH [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x4000, 0xCD);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x4000),
        0xCD,
        "Memory should still contain data"
    );
    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_clflush_rdx_base() {
    // CLFLUSH using RDX as base [rdx]
    let code = [
        0x0f, 0xae, 0x3a, // CLFLUSH [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x5000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x5000, 0xEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x5000),
        0xEF,
        "Memory should still contain data"
    );
    assert_eq!(regs.rdx, 0x5000, "RDX should be unchanged");
}

#[test]
fn test_clflush_rsi_base() {
    // CLFLUSH using RSI as base [rsi]
    let code = [
        0x0f, 0xae, 0x3e, // CLFLUSH [rsi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rsi = 0x6000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x6000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x6000),
        0x12345678,
        "Memory should still contain data"
    );
    assert_eq!(regs.rsi, 0x6000, "RSI should be unchanged");
}

#[test]
fn test_clflush_rdi_base() {
    // CLFLUSH using RDI as base [rdi]
    let code = [
        0x0f, 0xae, 0x3f, // CLFLUSH [rdi]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rdi = 0x7000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x7000, 0x87654321);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x7000),
        0x87654321,
        "Memory should still contain data"
    );
    assert_eq!(regs.rdi, 0x7000, "RDI should be unchanged");
}

#[test]
fn test_clflush_large_displacement() {
    // CLFLUSH with large displacement [rax + 0x1000]
    let code = [
        0x0f, 0xae, 0xb8, 0x00, 0x10, 0x00, 0x00, // CLFLUSH [rax + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x3000, 0xFEDCBA9876543210);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x3000),
        0xFEDCBA9876543210,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_sib_addressing() {
    // CLFLUSH with SIB addressing [rax + rbx*4]
    let code = [
        0x0f, 0xae, 0x3c, 0x98, // CLFLUSH [rax + rbx*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x10*4 = 0x2040
    write_mem_at_u8(&mem, 0x2040, 0x99);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2040),
        0x99,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x10, "RBX should be unchanged");
}

#[test]
fn test_clflush_sib_scale_2() {
    // CLFLUSH with SIB scale 2 [rax + rbx*2]
    let code = [
        0x0f, 0xae, 0x3c, 0x58, // CLFLUSH [rax + rbx*2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x100;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x100*2 = 0x2200
    write_mem_at_u32(&mem, 0x2200, 0xAABBCCDD);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2200),
        0xAABBCCDD,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x100, "RBX should be unchanged");
}

#[test]
fn test_clflush_sib_scale_8() {
    // CLFLUSH with SIB scale 8 [rax + rbx*8]
    let code = [
        0x0f, 0xae, 0x3c, 0xd8, // CLFLUSH [rax + rbx*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x20*8 = 0x2100
    write_mem_at_u64(&mem, 0x2100, 0x1122334455667788);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x2100),
        0x1122334455667788,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x20, "RBX should be unchanged");
}

#[test]
fn test_clflush_no_flags_modified() {
    // CLFLUSH doesn't modify flags
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should be unchanged");
}

#[test]
fn test_clflush_multiple_sequential() {
    // Multiple sequential CLFLUSH instructions
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x3b, // CLFLUSH [rbx]
        0x0f, 0xae, 0x39, // CLFLUSH [rcx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x3000;
    regs.rcx = 0x4000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x11);
    write_mem_at_u8(&mem, 0x3000, 0x22);
    write_mem_at_u8(&mem, 0x4000, 0x33);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x11,
        "Memory at RAX should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x3000),
        0x22,
        "Memory at RBX should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x4000),
        0x33,
        "Memory at RCX should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x3000, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x4000, "RCX should be unchanged");
}

#[test]
fn test_clflush_aligned_address() {
    // CLFLUSH on cache-line aligned address (64-byte aligned)
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // 64-byte aligned
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0xABCDEF01);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xABCDEF01,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_unaligned_address() {
    // CLFLUSH on unaligned address
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2007; // Unaligned
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2007, 0x88);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2007),
        0x88,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2007, "RAX should be unchanged");
}

#[test]
fn test_clflush_with_r8_base() {
    // CLFLUSH using R8 as base [r8]
    let code = [
        0x41, 0x0f, 0xae, 0x38, // CLFLUSH [r8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x8000, 0x1111222233334444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x8000),
        0x1111222233334444,
        "Memory should still contain data"
    );
    assert_eq!(regs.r8, 0x8000, "R8 should be unchanged");
}

#[test]
fn test_clflush_with_r15_base() {
    // CLFLUSH using R15 as base [r15]
    let code = [
        0x41, 0x0f, 0xae, 0x3f, // CLFLUSH [r15]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xF000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0xF000, 0x55667788);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0xF000),
        0x55667788,
        "Memory should still contain data"
    );
    assert_eq!(regs.r15, 0xF000, "R15 should be unchanged");
}

#[test]
fn test_clflush_write_then_flush() {
    // Write to memory, then CLFLUSH
    let code = [
        0x48, 0xc7, 0x00, 0x99, 0x00, 0x00, 0x00, // MOV qword [rax], 0x99
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Memory should contain the written value
    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0x99,
        "Memory should contain written value"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_flush_then_read() {
    // CLFLUSH then read from memory
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x48, 0x8b, 0x18, // MOV rbx, [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to memory
    write_mem_at_u64(&mem, 0x2000, 0xAABBCCDDEEFF0011);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RBX should contain the value from memory
    assert_eq!(
        regs.rbx, 0xAABBCCDDEEFF0011,
        "RBX should contain value from memory"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_same_location_twice() {
    // CLFLUSH on the same location twice
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xCC);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xCC,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_different_offsets_same_line() {
    // CLFLUSH on different offsets that might be in the same cache line
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x78, 0x08, // CLFLUSH [rax + 8]
        0x0f, 0xae, 0x78, 0x10, // CLFLUSH [rax + 16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x11);
    write_mem_at_u8(&mem, 0x2008, 0x22);
    write_mem_at_u8(&mem, 0x2010, 0x33);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0x11,
        "Memory at offset 0 should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2008),
        0x22,
        "Memory at offset 8 should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2010),
        0x33,
        "Memory at offset 16 should still contain data"
    );
    assert_eq!(regs.rax, 0x2000, "RAX should be unchanged");
}

#[test]
fn test_clflush_interleaved_with_operations() {
    // CLFLUSH interleaved with other operations
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV rax, 0x42
        0x0f, 0xae, 0x3b, // CLFLUSH [rbx]
        0x48, 0xc7, 0xc1, 0x84, 0x00, 0x00, 0x00, // MOV rcx, 0x84
        0x0f, 0xae, 0x3a, // CLFLUSH [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    regs.rdx = 0x5000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x3000, 0xAA);
    write_mem_at_u8(&mem, 0x5000, 0xBB);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should be 0x42");
    assert_eq!(regs.rcx, 0x84, "RCX should be 0x84");
    assert_eq!(
        read_mem_at_u8(&mem, 0x3000),
        0xAA,
        "Memory at RBX should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x5000),
        0xBB,
        "Memory at RDX should still contain data"
    );
}

#[test]
fn test_clflush_preserves_all_registers() {
    // CLFLUSH preserves all general-purpose registers
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111;
    regs.rbx = 0x2222;
    regs.rcx = 0x3333;
    regs.rdx = 0x4444;
    regs.rsi = 0x5555;
    regs.rdi = 0x6666;
    regs.r8 = 0x7777;
    regs.r9 = 0x8888;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x1111, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1111, "RAX should be unchanged");
    assert_eq!(regs.rbx, 0x2222, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x3333, "RCX should be unchanged");
    assert_eq!(regs.rdx, 0x4444, "RDX should be unchanged");
    assert_eq!(regs.rsi, 0x5555, "RSI should be unchanged");
    assert_eq!(regs.rdi, 0x6666, "RDI should be unchanged");
    assert_eq!(regs.r8, 0x7777, "R8 should be unchanged");
    assert_eq!(regs.r9, 0x8888, "R9 should be unchanged");
}

#[test]
fn test_clflush_with_stack_memory() {
    // CLFLUSH on stack memory
    let code = [
        0x0f, 0xae, 0x3c, 0x24, // CLFLUSH [rsp]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write data to stack
    write_mem_at_u64(&mem, 0x8000, 0xDEADBEEFCAFEBABE);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u64(&mem, 0x8000),
        0xDEADBEEFCAFEBABE,
        "Stack memory should still contain data"
    );
}

#[test]
fn test_clflush_near_boundary() {
    // CLFLUSH near a page boundary
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2FFF; // Near a 4K boundary
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2FFF, 0x77);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2FFF),
        0x77,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x2FFF, "RAX should be unchanged");
}

#[test]
fn test_clflush_high_memory_address() {
    // CLFLUSH on higher memory address
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x100000; // 1MB
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x100000, 0x12345678);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x100000),
        0x12345678,
        "Memory should still contain data"
    );
    assert_eq!(regs.rax, 0x100000, "RAX should be unchanged");
}

#[test]
fn test_clflush_with_sib_displacement() {
    // CLFLUSH with SIB and displacement [rax + rbx*4 + 0x10]
    let code = [
        0x0f, 0xae, 0x7c, 0x98, 0x10, // CLFLUSH [rax + rbx*4 + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x10*4 + 0x10 = 0x2050
    write_mem_at_u8(&mem, 0x2050, 0xAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2050),
        0xAA,
        "Memory should still contain data"
    );
}

#[test]
fn test_clflush_r9_base() {
    // CLFLUSH using R9 as base [r9]
    let code = [
        0x41, 0x0f, 0xae, 0x39, // CLFLUSH [r9]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0x9000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x9000, 0x99887766);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u32(&mem, 0x9000),
        0x99887766,
        "Memory should still contain data"
    );
    assert_eq!(regs.r9, 0x9000, "R9 should be unchanged");
}

#[test]
fn test_clflush_consecutive_bytes() {
    // CLFLUSH on consecutive byte addresses
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x78, 0x01, // CLFLUSH [rax + 1]
        0x0f, 0xae, 0x78, 0x02, // CLFLUSH [rax + 2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);
    write_mem_at_u8(&mem, 0x2001, 0xBB);
    write_mem_at_u8(&mem, 0x2002, 0xCC);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_at_u8(&mem, 0x2000),
        0xAA,
        "Byte 0 should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2001),
        0xBB,
        "Byte 1 should still contain data"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2002),
        0xCC,
        "Byte 2 should still contain data"
    );
}
