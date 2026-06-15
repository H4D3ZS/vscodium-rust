// Module path for tests run via x86_64.rs
use crate::common::{
    read_mem_at_u8, read_mem_at_u16, read_mem_at_u32, read_mem_at_u64, run_until_hlt, setup_vm,
    write_mem_at_u8, write_mem_at_u16, write_mem_at_u32, write_mem_at_u64,
};
use rax::cpu::Registers;

// CLWB - Cache Line Write Back
//
// Writes back modified cache line to memory without invalidating the line
// Unlike CLFLUSH (which invalidates), CLWB keeps the line in cache after writeback
// Ensures data is persistent in memory (useful for persistent memory)
//
// Opcode:
// 66 0F AE /6            CLWB m8            - Write back cache line containing m8

#[test]
fn test_clwb_basic() {
    // Basic CLWB with memory operand [rax]
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x42);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_with_displacement() {
    // CLWB with displacement [rax + 0x10]
    let code = [
        0x66, 0x0f, 0xae, 0x70, 0x10, // CLWB [rax + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2010, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2010), 0xDEADBEEF);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_with_negative_displacement() {
    // CLWB with negative displacement [rax - 0x10]
    let code = [
        0x66, 0x0f, 0xae, 0x70, 0xf0, // CLWB [rax - 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2100;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x20f0, 0x1234567890ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x20f0), 0x1234567890ABCDEF);
    assert_eq!(regs.rax, 0x2100);
}

#[test]
fn test_clwb_different_base_registers() {
    // Test CLWB with different base registers
    let registers = [
        (0x30, 0x2000u64, "rax"),
        (0x33, 0x3000u64, "rbx"),
        (0x31, 0x4000u64, "rcx"),
        (0x32, 0x5000u64, "rdx"),
        (0x36, 0x6000u64, "rsi"),
        (0x37, 0x7000u64, "rdi"),
    ];

    for (modrm, addr, _name) in &registers {
        let code = [
            0x66, 0x0f, 0xae, *modrm, // CLWB [reg]
            0xf4,
        ];
        let mut regs = Registers::default();
        match modrm {
            0x30 => regs.rax = *addr,
            0x31 => regs.rcx = *addr,
            0x32 => regs.rdx = *addr,
            0x33 => regs.rbx = *addr,
            0x36 => regs.rsi = *addr,
            0x37 => regs.rdi = *addr,
            _ => {}
        }
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u8(&mem, *addr, 0xAB);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u8(&mem, *addr), 0xAB);
    }
}

#[test]
fn test_clwb_large_displacement() {
    // CLWB with large displacement [rax + 0x1000]
    let code = [
        0x66, 0x0f, 0xae, 0xb0, 0x00, 0x10, 0x00, 0x00, // CLWB [rax + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x3000, 0xFEDCBA9876543210);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x3000), 0xFEDCBA9876543210);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_sib_addressing() {
    // CLWB with SIB addressing [rax + rbx*4]
    let code = [
        0x66, 0x0f, 0xae, 0x34, 0x98, // CLWB [rax + rbx*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2040, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2040), 0x99);
    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x10);
}

#[test]
fn test_clwb_sib_scale_2() {
    // CLWB with SIB scale 2 [rax + rbx*2]
    let code = [
        0x66, 0x0f, 0xae, 0x34, 0x58, // CLWB [rax + rbx*2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x100;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2200, 0xAABBCCDD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2200), 0xAABBCCDD);
    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x100);
}

#[test]
fn test_clwb_sib_scale_8() {
    // CLWB with SIB scale 8 [rax + rbx*8]
    let code = [
        0x66, 0x0f, 0xae, 0x34, 0xd8, // CLWB [rax + rbx*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x20;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2100, 0x1122334455667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2100), 0x1122334455667788);
    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x20);
}

#[test]
fn test_clwb_no_flags_modified() {
    // CLWB doesn't modify flags
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rflags = 0x246; // CF, PF, ZF set
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_clwb_multiple_sequential() {
    // Multiple sequential CLWB instructions
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x33, // CLWB [rbx]
        0x66, 0x0f, 0xae, 0x31, // CLWB [rcx]
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

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x11);
    assert_eq!(read_mem_at_u8(&mem, 0x3000), 0x22);
    assert_eq!(read_mem_at_u8(&mem, 0x4000), 0x33);
    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x3000);
    assert_eq!(regs.rcx, 0x4000);
}

#[test]
fn test_clwb_aligned_address() {
    // CLWB on cache-line aligned address (64-byte aligned)
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000; // 64-byte aligned
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x2000, 0xABCDEF01);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0xABCDEF01);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_unaligned_address() {
    // CLWB on unaligned address
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2007; // Unaligned
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2007, 0x88);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2007), 0x88);
    assert_eq!(regs.rax, 0x2007);
}

#[test]
fn test_clwb_with_r8_base() {
    // CLWB using R8 as base [r8]
    let code = [
        0x66, 0x41, 0x0f, 0xae, 0x30, // CLWB [r8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x8000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x8000, 0x1111222233334444);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x8000), 0x1111222233334444);
    assert_eq!(regs.r8, 0x8000);
}

#[test]
fn test_clwb_with_r15_base() {
    // CLWB using R15 as base [r15]
    let code = [
        0x66, 0x41, 0x0f, 0xae, 0x37, // CLWB [r15]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xF000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0xF000, 0x55667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0xF000), 0x55667788);
    assert_eq!(regs.r15, 0xF000);
}

#[test]
fn test_clwb_write_then_writeback() {
    // Write to memory, then CLWB
    let code = [
        0x48, 0xc7, 0x00, 0x99, 0x00, 0x00, 0x00, // MOV qword [rax], 0x99
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0x99);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_writeback_then_read() {
    // CLWB then read from memory
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x48, 0x8b, 0x18, // MOV rbx, [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0xAABBCCDDEEFF0011);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xAABBCCDDEEFF0011);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_same_location_twice() {
    // CLWB on the same location twice
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xCC);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xCC);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_different_offsets_same_line() {
    // CLWB on different offsets in the same cache line
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x70, 0x08, // CLWB [rax + 8]
        0x66, 0x0f, 0xae, 0x70, 0x10, // CLWB [rax + 16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x11);
    write_mem_at_u8(&mem, 0x2008, 0x22);
    write_mem_at_u8(&mem, 0x2010, 0x33);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x11);
    assert_eq!(read_mem_at_u8(&mem, 0x2008), 0x22);
    assert_eq!(read_mem_at_u8(&mem, 0x2010), 0x33);
    assert_eq!(regs.rax, 0x2000);
}

#[test]
fn test_clwb_cache_line_boundaries() {
    // Test CLWB at cache line boundaries
    for offset in &[0x0, 0x40, 0x80, 0xC0] {
        let code = [
            0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = 0x2000 + offset;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u64(&mem, 0x2000 + offset, 0x123456789ABCDEF0);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u64(&mem, 0x2000 + offset), 0x123456789ABCDEF0);
    }
}

#[test]
fn test_clwb_preserves_all_registers() {
    // CLWB preserves all general-purpose registers
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
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

    assert_eq!(regs.rax, 0x1111);
    assert_eq!(regs.rbx, 0x2222);
    assert_eq!(regs.rcx, 0x3333);
    assert_eq!(regs.rdx, 0x4444);
    assert_eq!(regs.rsi, 0x5555);
    assert_eq!(regs.rdi, 0x6666);
    assert_eq!(regs.r8, 0x7777);
    assert_eq!(regs.r9, 0x8888);
}

#[test]
fn test_clwb_with_stack_memory() {
    // CLWB on stack memory
    let code = [
        0x66, 0x0f, 0xae, 0x34, 0x24, // CLWB [rsp]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x8000, 0xDEADBEEFCAFEBABE);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x8000), 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_clwb_near_boundary() {
    // CLWB near a page boundary
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2FFF; // Near a 4K boundary
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2FFF, 0x77);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2FFF), 0x77);
    assert_eq!(regs.rax, 0x2FFF);
}

#[test]
fn test_clwb_high_memory_address() {
    // CLWB on higher memory address
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x100000; // 1MB
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x100000, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x100000), 0x12345678);
    assert_eq!(regs.rax, 0x100000);
}

#[test]
fn test_clwb_with_sib_displacement() {
    // CLWB with SIB and displacement [rax + rbx*4 + 0x10]
    let code = [
        0x66, 0x0f, 0xae, 0x74, 0x98, 0x10, // CLWB [rax + rbx*4 + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2050, 0xAA);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2050), 0xAA);
    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x10);
}

#[test]
fn test_clwb_after_atomic_operation() {
    // CLWB after atomic operation
    let code = [
        0xf0, 0x48, 0xff, 0x00, // LOCK INC qword [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0x100);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x101);
}

#[test]
fn test_clwb_multiple_cache_lines() {
    // Write back multiple cache lines
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0xb0, 0x40, 0x00, 0x00, 0x00, // CLWB [rax + 0x40]
        0x66, 0x0f, 0xae, 0xb0, 0x80, 0x00, 0x00, 0x00, // CLWB [rax + 0x80]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);
    write_mem_at_u8(&mem, 0x2040, 0xBB);
    write_mem_at_u8(&mem, 0x2080, 0xCC);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xAA);
    assert_eq!(read_mem_at_u8(&mem, 0x2040), 0xBB);
    assert_eq!(read_mem_at_u8(&mem, 0x2080), 0xCC);
}

#[test]
fn test_clwb_interleaved_with_operations() {
    // CLWB interleaved with other operations
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV rax, 0x42
        0x66, 0x0f, 0xae, 0x33, // CLWB [rbx]
        0x48, 0xc7, 0xc1, 0x84, 0x00, 0x00, 0x00, // MOV rcx, 0x84
        0x66, 0x0f, 0xae, 0x32, // CLWB [rdx]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    regs.rdx = 0x5000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x3000, 0xAA);
    write_mem_at_u8(&mem, 0x5000, 0xBB);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42);
    assert_eq!(regs.rcx, 0x84);
    assert_eq!(read_mem_at_u8(&mem, 0x3000), 0xAA);
    assert_eq!(read_mem_at_u8(&mem, 0x5000), 0xBB);
}

#[test]
fn test_clwb_write_modify_writeback_read() {
    // Write, modify, CLWB, read sequence
    let code = [
        0x48, 0xc7, 0x00, 0x11, 0x00, 0x00, 0x00, // MOV qword [rax], 0x11
        0x48, 0xff, 0x00, // INC qword [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x48, 0x8b, 0x18, // MOV rbx, [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x12);
}

#[test]
fn test_clwb_with_different_data_sizes() {
    // CLWB after writing different data sizes
    let code = [
        0xc6, 0x00, 0xAA, // MOV byte [rax], 0xAA
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0xc7, 0x40, 0x08, 0xBB, 0xCC, // MOV word [rax+8], 0xCCBB
        0x66, 0x0f, 0xae, 0x70, 0x08, // CLWB [rax+8]
        0xc7, 0x40, 0x10, 0xDD, 0xEE, 0xFF, 0x00, // MOV dword [rax+16], 0x00FFEEDD
        0x66, 0x0f, 0xae, 0x70, 0x10, // CLWB [rax+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xAA);
    assert_eq!(read_mem_at_u16(&mem, 0x2008), 0xCCBB);
    assert_eq!(read_mem_at_u32(&mem, 0x2010), 0x00FFEEDD);
}

#[test]
fn test_clwb_loop_pattern() {
    // Simulate loop write-back pattern
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x48, 0x83, 0xc0, 0x40, // ADD rax, 0x40
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x48, 0x83, 0xc0, 0x40, // ADD rax, 0x40
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x11);
    write_mem_at_u8(&mem, 0x2040, 0x22);
    write_mem_at_u8(&mem, 0x2080, 0x33);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x11);
    assert_eq!(read_mem_at_u8(&mem, 0x2040), 0x22);
    assert_eq!(read_mem_at_u8(&mem, 0x2080), 0x33);
    assert_eq!(regs.rax, 0x2080);
}

#[test]
fn test_clwb_all_extended_registers() {
    // Test CLWB with all extended registers R8-R15
    for reg_offset in 0..8 {
        let code = match reg_offset {
            4 => vec![0x66, 0x41, 0x0f, 0xae, 0x34, 0x24, 0xf4], // CLWB [r12] (SIB required)
            5 => vec![0x66, 0x41, 0x0f, 0xae, 0x75, 0x00, 0xf4], // CLWB [r13+0] (disp8 required)
            _ => vec![0x66, 0x41, 0x0f, 0xae, 0x30 | reg_offset, 0xf4], // CLWB [r8-r15]
        };
        let mut regs = Registers::default();
        let addr = 0x8000 + (reg_offset as u64 * 0x100);
        match reg_offset {
            0 => regs.r8 = addr,
            1 => regs.r9 = addr,
            2 => regs.r10 = addr,
            3 => regs.r11 = addr,
            4 => regs.r12 = addr,
            5 => regs.r13 = addr,
            6 => regs.r14 = addr,
            7 => regs.r15 = addr,
            _ => {}
        }
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u8(&mem, addr, 0xCC);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u8(&mem, addr), 0xCC);
    }
}

#[test]
fn test_clwb_consecutive_addresses() {
    // CLWB on consecutive byte addresses
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x70, 0x01, // CLWB [rax + 1]
        0x66, 0x0f, 0xae, 0x70, 0x02, // CLWB [rax + 2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);
    write_mem_at_u8(&mem, 0x2001, 0xBB);
    write_mem_at_u8(&mem, 0x2002, 0xCC);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xAA);
    assert_eq!(read_mem_at_u8(&mem, 0x2001), 0xBB);
    assert_eq!(read_mem_at_u8(&mem, 0x2002), 0xCC);
}

#[test]
fn test_clwb_rapid_succession() {
    // Multiple CLWBs in rapid succession
    let code = [
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0xFEDCBA9876543210);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0xFEDCBA9876543210);
}

#[test]
fn test_clwb_within_cache_line() {
    // Test CLWB at different offsets within a 64-byte cache line
    for offset in 0..64 {
        let code = [
            0x66, 0x0f, 0xae, 0x30, // CLWB [rax]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = 0x2000 + offset;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u8(&mem, 0x2000 + offset, 0x42);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u8(&mem, 0x2000 + offset), 0x42);
    }
}

#[test]
fn test_clwb_array_writeback_pattern() {
    // Simulate writing back array elements
    let code = [
        0x66, 0x0f, 0xae, 0x34, 0xc8, // CLWB [rax + rcx*8]
        0x48, 0xff, 0xc1, // INC rcx
        0x66, 0x0f, 0xae, 0x34, 0xc8, // CLWB [rax + rcx*8]
        0x48, 0xff, 0xc1, // INC rcx
        0x66, 0x0f, 0xae, 0x34, 0xc8, // CLWB [rax + rcx*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rcx = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x2000, 0x1111111111111111);
    write_mem_at_u64(&mem, 0x2008, 0x2222222222222222);
    write_mem_at_u64(&mem, 0x2010, 0x3333333333333333);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, 0x2008), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&mem, 0x2010), 0x3333333333333333);
    assert_eq!(regs.rcx, 2);
}
