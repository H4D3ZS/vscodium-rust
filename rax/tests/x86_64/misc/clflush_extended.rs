// Module path for tests run via x86_64.rs
use crate::common::{
    read_mem_at_u8, read_mem_at_u16, read_mem_at_u32, read_mem_at_u64, run_until_hlt, setup_vm,
    write_mem_at_u8, write_mem_at_u16, write_mem_at_u32, write_mem_at_u64,
};
use rax::cpu::Registers;

// CLFLUSH Extended Tests - Cache Line Flush
//
// Extended and comprehensive testing of CLFLUSH instruction patterns
// CLFLUSH invalidates the cache line containing the specified memory address
// from all levels of the cache hierarchy.
//
// Opcode:
// NP 0F AE /7            CLFLUSH m8            - Flush cache line containing m8

#[test]
fn test_clflush_all_base_registers() {
    // Test CLFLUSH with all general purpose base registers
    for (reg_idx, addr) in [
        (0, 0x2000u64),
        (3, 0x3000),
        (1, 0x4000),
        (2, 0x5000),
        (6, 0x6000),
        (7, 0x7000),
    ]
    .iter()
    {
        let modrm = 0x38 | reg_idx;
        let code = [
            0x0f, 0xae, modrm, // CLFLUSH [reg]
            0xf4,
        ];
        let mut regs = Registers::default();
        match reg_idx {
            0 => regs.rax = *addr,
            1 => regs.rcx = *addr,
            2 => regs.rdx = *addr,
            3 => regs.rbx = *addr,
            6 => regs.rsi = *addr,
            7 => regs.rdi = *addr,
            _ => {}
        }
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        write_mem_at_u8(&mem, *addr, 0xAB);

        let _regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_mem_at_u8(&mem, *addr), 0xAB);
    }
}

#[test]
fn test_clflush_cache_line_boundaries() {
    // Test CLFLUSH at various cache line boundaries (64-byte aligned)
    for offset in &[0x0, 0x40, 0x80, 0xC0, 0x100, 0x140] {
        let code = [
            0x0f, 0xae, 0x38, // CLFLUSH [rax]
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
fn test_clflush_within_cache_line() {
    // Test CLFLUSH at different offsets within a single 64-byte cache line
    let base_addr = 0x2000u64;
    for offset in 0..64 {
        let code = [
            0x0f, 0xae, 0x38, // CLFLUSH [rax]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = base_addr + offset;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u8(&mem, base_addr + offset, 0x42);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u8(&mem, base_addr + offset), 0x42);
    }
}

#[test]
fn test_clflush_crossing_page_boundary() {
    // CLFLUSH near 4K page boundary
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2FFE; // 2 bytes before 0x3000
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u16(&mem, 0x2FFE, 0xABCD);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u16(&mem, 0x2FFE), 0xABCD);
}

#[test]
fn test_clflush_with_all_sib_scales() {
    // Test CLFLUSH with SIB addressing using all scale factors
    for (scale, scale_val) in &[(0x40, 1), (0x80, 2), (0xC0, 4), (0x00, 8)] {
        let code = [
            0x0f, 0xae, 0x3c, *scale, // CLFLUSH [rax + rcx*scale]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = 0x2000;
        regs.rcx = 0x10;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        let addr = 0x2000 + (0x10 * scale_val);
        write_mem_at_u8(&mem, addr, 0x99);

        let _regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_mem_at_u8(&mem, addr), 0x99);
    }
}

#[test]
fn test_clflush_complex_sib_with_displacement() {
    // Test CLFLUSH with complex SIB: [base + index*scale + disp32]
    let code = [
        0x0f, 0xae, 0xbc, 0x98, 0x20, 0x00, 0x00, 0x00, // CLFLUSH [rax + rbx*4 + 0x20]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x8;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = 0x2000 + 0x8*4 + 0x20 = 0x2040
    write_mem_at_u32(&mem, 0x2040, 0xDEADBEEF);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2040), 0xDEADBEEF);
}

#[test]
fn test_clflush_with_r8_r15_bases() {
    // Test CLFLUSH with extended registers R8-R15 as base
    for reg_offset in 0..8 {
        let code = match reg_offset {
            4 => vec![0x41, 0x0f, 0xae, 0x3c, 0x24, 0xf4], // CLFLUSH [r12] (SIB required)
            5 => vec![0x41, 0x0f, 0xae, 0x7d, 0x00, 0xf4], // CLFLUSH [r13+0] (disp8 required)
            _ => vec![0x41, 0x0f, 0xae, 0x38 | reg_offset, 0xf4], // CLFLUSH [r8-r15]
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
fn test_clflush_write_modify_flush_read() {
    // Write, modify, flush, then read sequence
    let code = [
        0x48, 0xc7, 0x00, 0x11, 0x00, 0x00, 0x00, // MOV qword [rax], 0x11
        0x48, 0xff, 0x00, // INC qword [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
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
fn test_clflush_multiple_lines_sequential() {
    // Flush multiple sequential cache lines
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0xb8, 0x40, 0x00, 0x00, 0x00, // CLFLUSH [rax + 0x40]
        0x0f, 0xae, 0xb8, 0x80, 0x00, 0x00, 0x00, // CLFLUSH [rax + 0x80]
        0x0f, 0xae, 0xb8, 0xC0, 0x00, 0x00, 0x00, // CLFLUSH [rax + 0xC0]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);
    write_mem_at_u8(&mem, 0x2040, 0xBB);
    write_mem_at_u8(&mem, 0x2080, 0xCC);
    write_mem_at_u8(&mem, 0x20C0, 0xDD);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xAA);
    assert_eq!(read_mem_at_u8(&mem, 0x2040), 0xBB);
    assert_eq!(read_mem_at_u8(&mem, 0x2080), 0xCC);
    assert_eq!(read_mem_at_u8(&mem, 0x20C0), 0xDD);
}

#[test]
fn test_clflush_overlapping_addresses() {
    // Multiple flushes of overlapping addresses in same cache line
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x78, 0x10, // CLFLUSH [rax + 0x10]
        0x0f, 0xae, 0x78, 0x20, // CLFLUSH [rax + 0x20]
        0x0f, 0xae, 0x78, 0x30, // CLFLUSH [rax + 0x30]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x2000 + i * 0x10, 0x10 + i as u8);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u8(&mem, 0x2000 + i * 0x10), 0x10 + i as u8);
    }
}

#[test]
fn test_clflush_after_atomic_operation() {
    // CLFLUSH after atomic operation
    let code = [
        0xf0, 0x48, 0xff, 0x00, // LOCK INC qword [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
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
fn test_clflush_different_data_sizes() {
    // CLFLUSH after writing different data sizes
    let code = [
        0xc6, 0x00, 0xAA, // MOV byte [rax], 0xAA
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x66, 0xc7, 0x40, 0x08, 0xBB, 0xCC, // MOV word [rax+8], 0xCCBB
        0x0f, 0xae, 0x78, 0x08, // CLFLUSH [rax+8]
        0xc7, 0x40, 0x10, 0xDD, 0xEE, 0xFF, 0x00, // MOV dword [rax+16], 0x00FFEEDD
        0x0f, 0xae, 0x78, 0x10, // CLFLUSH [rax+16]
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
fn test_clflush_with_negative_displacements() {
    // Test various negative displacement values
    for disp in &[0xF0, 0xE0, 0xD0, 0xC0] {
        let code = [
            0x0f, 0xae, 0x78, *disp, // CLFLUSH [rax + disp8]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = 0x2100;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        let offset = (*disp as i8) as i64;
        let addr = (0x2100i64 + offset) as u64;
        write_mem_at_u8(&mem, addr, 0x77);

        let _regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(read_mem_at_u8(&mem, addr), 0x77);
    }
}

#[test]
fn test_clflush_large_positive_displacement() {
    // Test with maximum positive 32-bit displacement
    let code = [
        0x0f, 0xae, 0xb8, 0x00, 0x10, 0x00, 0x00, // CLFLUSH [rax + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x3000, 0xFEDCBA9876543210);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x3000), 0xFEDCBA9876543210);
}

#[test]
fn test_clflush_in_loop_pattern() {
    // Simulate loop flushing pattern
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x48, 0x83, 0xc0, 0x40, // ADD rax, 0x40
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x48, 0x83, 0xc0, 0x40, // ADD rax, 0x40
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
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
fn test_clflush_preserves_rflags() {
    // Verify CLFLUSH preserves all RFLAGS
    let code = [
        0xf8, // CLC (clear carry)
        0xf9, // STC (set carry)
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // CF should still be set from STC
    assert_eq!(regs.rflags & 0x1, 1);
}

#[test]
fn test_clflush_with_stack_pointer() {
    // CLFLUSH using RSP as base
    let code = [
        0x0f, 0xae, 0x3c, 0x24, // CLFLUSH [rsp]
        0xf4,
    ];
    let mut regs = Registers::default();
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&mem, 0x8000, 0x1234567890ABCDEF);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x8000), 0x1234567890ABCDEF);
}

#[test]
fn test_clflush_sib_with_rbp_base() {
    // CLFLUSH with RBP as base in SIB
    let code = [
        0x0f, 0xae, 0x7c, 0x2d, 0x10, // CLFLUSH [rbp + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = 0x5000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x5010, 0xAABBCCDD);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x5010), 0xAABBCCDD);
}

#[test]
fn test_clflush_rapid_succession() {
    // Multiple CLFLUSHes in rapid succession
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
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
fn test_clflush_all_address_patterns() {
    // Test various address patterns
    let patterns = [
        0x2000, // Cache line aligned
        0x2001, // +1
        0x2007, // +7
        0x2010, // +16
        0x201F, // +31
        0x2020, // +32
        0x203F, // +63 (last byte in 64-byte line)
    ];

    for addr in &patterns {
        let code = [
            0x0f, 0xae, 0x38, // CLFLUSH [rax]
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rax = *addr;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));

        write_mem_at_u8(&mem, *addr, 0x88);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u8(&mem, *addr), 0x88);
    }
}

#[test]
fn test_clflush_interleaved_reads_writes() {
    // CLFLUSH interleaved with reads and writes
    let code = [
        0x48, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV qword [rax], 0x1
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x48, 0x8b, 0x18, // MOV rbx, [rax]
        0x48, 0xc7, 0x00, 0x02, 0x00, 0x00, 0x00, // MOV qword [rax], 0x2
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x48, 0x8b, 0x08, // MOV rcx, [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1);
    assert_eq!(regs.rcx, 0x2);
}

#[test]
fn test_clflush_with_rip_relative() {
    // CLFLUSH with RIP-relative addressing [rip + disp32]
    let code = [
        0x0f, 0xae, 0x3d, 0x00, 0x10, 0x00, 0x00, // CLFLUSH [rip + 0x1000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // RIP will be at 0x1000 (CODE_ADDR), so rip+0x1000 after instruction = 0x2007
    write_mem_at_u8(&mem, 0x2007, 0x99);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2007), 0x99);
}

#[test]
fn test_clflush_zero_displacement() {
    // CLFLUSH with explicit zero displacement
    let code = [
        0x0f, 0xae, 0x78, 0x00, // CLFLUSH [rax + 0]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xEE);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xEE);
}

#[test]
fn test_clflush_high_memory() {
    // CLFLUSH on higher memory addresses
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x500000; // 5MB
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x500000, 0x12345678);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x500000), 0x12345678);
}

#[test]
fn test_clflush_mixed_with_prefetch() {
    // CLFLUSH mixed with PREFETCH operations
    let code = [
        0x0f, 0x18, 0x08, // PREFETCH0 [rax]
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0x18, 0x48, 0x40, // PREFETCH1 [rax + 0x40]
        0x0f, 0xae, 0x78, 0x40, // CLFLUSH [rax + 0x40]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x2000, 0xAA);
    write_mem_at_u8(&mem, 0x2040, 0xBB);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0xAA);
    assert_eq!(read_mem_at_u8(&mem, 0x2040), 0xBB);
}

#[test]
fn test_clflush_preserves_all_gpr() {
    // Verify CLFLUSH doesn't modify any general purpose registers
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;
    regs.rdx = 0x4444444444444444;
    regs.rsi = 0x5555555555555555;
    regs.rdi = 0x6666666666666666;
    regs.rbp = 0x7777777777777777;
    regs.r8 = 0x8888888888888888;
    regs.r9 = 0x9999999999999999;
    regs.r10 = 0xAAAAAAAAAAAAAAAA;
    regs.r11 = 0xBBBBBBBBBBBBBBBB;
    regs.r12 = 0xCCCCCCCCCCCCCCCC;
    regs.r13 = 0xDDDDDDDDDDDDDDDD;
    regs.r14 = 0xEEEEEEEEEEEEEEEE;
    regs.r15 = 0xFFFFFFFFFFFFFFFF;

    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u8(&mem, 0x2000, 0xFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x2000);
    assert_eq!(regs.rbx, 0x2222222222222222);
    assert_eq!(regs.rcx, 0x3333333333333333);
    assert_eq!(regs.rdx, 0x4444444444444444);
    assert_eq!(regs.rsi, 0x5555555555555555);
    assert_eq!(regs.rdi, 0x6666666666666666);
    assert_eq!(regs.rbp, 0x7777777777777777);
    assert_eq!(regs.r8, 0x8888888888888888);
    assert_eq!(regs.r9, 0x9999999999999999);
    assert_eq!(regs.r10, 0xAAAAAAAAAAAAAAAA);
    assert_eq!(regs.r11, 0xBBBBBBBBBBBBBBBB);
    assert_eq!(regs.r12, 0xCCCCCCCCCCCCCCCC);
    assert_eq!(regs.r13, 0xDDDDDDDDDDDDDDDD);
    assert_eq!(regs.r14, 0xEEEEEEEEEEEEEEEE);
    assert_eq!(regs.r15, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_clflush_sib_index_only() {
    // CLFLUSH with SIB using index register only (no base)
    let code = [
        0x0f, 0xae, 0x3c, 0x8d, 0x00, 0x00, 0x00, 0x00, // CLFLUSH [rcx*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x800;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Address = rcx*4 = 0x800*4 = 0x2000
    write_mem_at_u8(&mem, 0x2000, 0x77);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x2000), 0x77);
}

#[test]
fn test_clflush_array_pattern() {
    // Simulate flushing array elements
    let code = [
        0x0f, 0xae, 0x3c, 0xc8, // CLFLUSH [rax + rcx*8] - element 0
        0x48, 0xff, 0xc1, // INC rcx
        0x0f, 0xae, 0x3c, 0xc8, // CLFLUSH [rax + rcx*8] - element 1
        0x48, 0xff, 0xc1, // INC rcx
        0x0f, 0xae, 0x3c, 0xc8, // CLFLUSH [rax + rcx*8] - element 2
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

#[test]
fn test_clflush_cross_cache_line_boundary() {
    // Test addresses that would span cache line boundaries if multi-byte
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x203C; // 4 bytes before next cache line (0x2040)
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u32(&mem, 0x203C, 0xAABBCCDD); // Spans to 0x2040
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x203C), 0xAABBCCDD);
}

#[test]
fn test_clflush_alternating_addresses() {
    // Flush alternating cache lines
    let code = [
        0x0f, 0xae, 0x38, // CLFLUSH [rax]
        0x0f, 0xae, 0xb8, 0x80, 0x00, 0x00, 0x00, // CLFLUSH [rax + 0x80]
        0x0f, 0xae, 0x78, 0x40, // CLFLUSH [rax + 0x40]
        0x0f, 0xae, 0xb8, 0xC0, 0x00, 0x00, 0x00, // CLFLUSH [rax + 0xC0]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    for i in 0..4 {
        write_mem_at_u8(&mem, 0x2000 + i * 0x40, 0xA0 + i as u8);
    }

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..4 {
        assert_eq!(read_mem_at_u8(&mem, 0x2000 + i * 0x40), 0xA0 + i as u8);
    }
}

#[test]
fn test_clflush_maximum_cache_line_offset() {
    // Test at maximum offset within a cache line (63 bytes)
    let code = [
        0x0f, 0xae, 0x78, 0x3F, // CLFLUSH [rax + 63]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_at_u8(&mem, 0x203F, 0xFE);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u8(&mem, 0x203F), 0xFE);
}
