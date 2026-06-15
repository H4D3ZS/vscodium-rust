// Module path for tests run via x86_64.rs
use crate::common::{
    read_mem_at_u32, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u32, write_mem_at_u64,
};
use rax::cpu::Registers;

// MOVDIRI - Move Doubleword/Quadword as Direct Store
//
// Moves a 32-bit or 64-bit value from a register to memory using a direct store
// The store is performed using write-combining memory type semantics
// Useful for persistent memory and non-temporal stores
//
// Opcodes:
// NP 0F 38 F9 /r         MOVDIRI m32, r32    - Move r32 to m32 using direct store
// NP REX.W 0F 38 F9 /r   MOVDIRI m64, r64    - Move r64 to m64 using direct store

#[test]
fn test_movdiri_32bit_basic() {
    // Basic MOVDIRI 32-bit
    let code = [
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0x12345678);
}

#[test]
fn test_movdiri_64bit_basic() {
    // Basic MOVDIRI 64-bit
    let code = [
        0x48, 0xb8, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x77, // MOV RAX, 0x7766554433221100
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x7766554433221100);
}

#[test]
fn test_movdiri_32bit_with_register_addressing() {
    // MOVDIRI 32-bit with register addressing [rax]
    let code = [
        0xbb, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EBX, 0xDEADBEEF
        0x0f, 0x38, 0xf9, 0x18, // MOVDIRI [rax], ebx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0xDEADBEEF);
}

#[test]
fn test_movdiri_64bit_with_register_addressing() {
    // MOVDIRI 64-bit with register addressing [rbx]
    let code = [
        0x48, 0xb8, 0xEF, 0xBE, 0xAD, 0xDE, 0xFE, 0xCA, 0xBE,
        0xBA, // MOV RAX, 0xBABECAFEDEADBEEF
        0x48, 0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], rax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x3000), 0xBABECAFEDEADBEEF);
}

#[test]
fn test_movdiri_32bit_preserves_register() {
    // MOVDIRI 32-bit preserves source register
    let code = [
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x78563412);
}

#[test]
fn test_movdiri_64bit_preserves_register() {
    // MOVDIRI 64-bit preserves source register
    let code = [
        0x48, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // MOV RAX, value
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x8877665544332211);
}

#[test]
fn test_movdiri_32bit_with_displacement() {
    // MOVDIRI 32-bit with displacement [rax + 0x10]
    let code = [
        0xb8, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EAX, 0xDDCCBBAA
        0x0f, 0x38, 0xf9, 0x43, 0x10, // MOVDIRI [rbx + 0x10], eax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2010), 0xDDCCBBAA);
}

#[test]
fn test_movdiri_64bit_with_displacement() {
    // MOVDIRI 64-bit with displacement [rcx + 0x20]
    let code = [
        0x48, 0xb8, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, // MOV RAX, value
        0x48, 0x0f, 0x38, 0xf9, 0x41, 0x20, // MOVDIRI [rcx + 0x20], rax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x3000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x3020), 0x2233445566778899);
}

#[test]
fn test_movdiri_32bit_no_flags_modified() {
    // MOVDIRI 32-bit doesn't modify flags
    let code = [
        0xf9, // STC
        0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags & 0x1, 1); // CF still set
}

#[test]
fn test_movdiri_64bit_no_flags_modified() {
    // MOVDIRI 64-bit doesn't modify flags
    let code = [
        0xf9, // STC
        0x48, 0xb8, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, // MOV RAX, value
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags & 0x1, 1); // CF still set
}

#[test]
fn test_movdiri_32bit_different_registers() {
    // MOVDIRI 32-bit with different registers
    for (reg, reg_name) in &[(0, "eax"), (3, "ebx"), (1, "ecx"), (2, "edx")] {
        let modrm = 0x07 | (reg << 3);
        let code = [
            0xb8 | reg,
            0x10,
            0x20,
            0x30,
            0x40, // MOV reg, value
            0xbf,
            0x00,
            0x20,
            0x00,
            0x00, // MOV EDI, 0x2000
            0x0f,
            0x38,
            0xf9,
            modrm, // MOVDIRI [rdi], reg
            0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(
            read_mem_at_u32(&mem, 0x2000),
            0x40302010,
            "Failed for {}",
            reg_name
        );
    }
}

#[test]
fn test_movdiri_32bit_zero_value() {
    // MOVDIRI 32-bit with zero
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0);
}

#[test]
fn test_movdiri_64bit_zero_value() {
    // MOVDIRI 64-bit with zero
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0);
}

#[test]
fn test_movdiri_32bit_all_ones() {
    // MOVDIRI 32-bit with all ones
    let code = [
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0xFFFFFFFF);
}

#[test]
fn test_movdiri_64bit_all_ones() {
    // MOVDIRI 64-bit with all ones
    let code = [
        0x48, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // MOV RAX, -1
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_movdiri_32bit_sequential_stores() {
    // Multiple sequential 32-bit MOVDIRI stores
    let code = [
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xb8, 0x22, 0x22, 0x22, 0x22, // MOV EAX, 0x22222222
        0x0f, 0x38, 0xf9, 0x47, 0x04, // MOVDIRI [rdi + 0x04], EAX
        0xb8, 0x33, 0x33, 0x33, 0x33, // MOV EAX, 0x33333333
        0x0f, 0x38, 0xf9, 0x47, 0x08, // MOVDIRI [rdi + 0x08], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0x11111111);
    assert_eq!(read_mem_at_u32(&mem, 0x2004), 0x22222222);
    assert_eq!(read_mem_at_u32(&mem, 0x2008), 0x33333333);
}

#[test]
fn test_movdiri_64bit_sequential_stores() {
    // Multiple sequential 64-bit MOVDIRI stores
    let code = [
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RAX, value1
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0x48, 0xb8, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RAX, value2
        0x48, 0x0f, 0x38, 0xf9, 0x47, 0x08, // MOVDIRI [rdi + 0x08], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, 0x2008), 0x2222222222222222);
}

#[test]
fn test_movdiri_32bit_sib_addressing() {
    // MOVDIRI 32-bit with SIB addressing [rax + rbx*4]
    let code = [
        0xb9, 0xAA, 0xBB, 0xCC, 0xDD, // MOV ECX, 0xDDCCBBAA
        0x0f, 0x38, 0xf9, 0x0c, 0x98, // MOVDIRI [rax + rbx*4], ecx
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    regs.rbx = 0x4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Address = 0x2000 + 0x4*4 = 0x2010
    assert_eq!(read_mem_at_u32(&mem, 0x2010), 0xDDCCBBAA);
}

#[test]
fn test_movdiri_64bit_sib_addressing() {
    // MOVDIRI 64-bit with SIB addressing [rcx + rdx*8]
    let code = [
        0x48, 0xb8, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, // MOV RAX, value
        0x48, 0x0f, 0x38, 0xf9, 0x04, 0xd1, // MOVDIRI [rcx + rdx*8], rax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x2000;
    regs.rdx = 0x4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Address = 0x2000 + 0x4*8 = 0x2020
    assert_eq!(read_mem_at_u64(&mem, 0x2020), 0x2233445566778899);
}

#[test]
fn test_movdiri_32bit_aligned_unaligned() {
    // MOVDIRI 32-bit to both aligned and unaligned addresses
    for offset in &[0, 1, 2, 3] {
        let code = [
            0xb8, 0xEE, 0xDD, 0xCC, 0xBB, // MOV EAX, 0xBBCCDDEE
            0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], eax
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x2000 + offset;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u32(&mem, 0x2000 + offset), 0xBBCCDDEE);
    }
}

#[test]
fn test_movdiri_64bit_aligned_unaligned() {
    // MOVDIRI 64-bit to both aligned and unaligned addresses
    for offset in &[0, 4] {
        let code = [
            0x48, 0xb8, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, // MOV RAX, value
            0x48, 0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], rax
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x2000 + offset;
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u64(&mem, 0x2000 + offset), 0x1100FFEEDDCCBBAA);
    }
}

#[test]
fn test_movdiri_32bit_high_memory() {
    // MOVDIRI 32-bit to higher memory address
    let code = [
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], eax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x100000; // 1MB
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x100000), 0x78563412);
}

#[test]
fn test_movdiri_64bit_high_memory() {
    // MOVDIRI 64-bit to higher memory address
    let code = [
        0x48, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // MOV RAX, value
        0x48, 0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], rax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x200000; // 2MB
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x200000), 0x8877665544332211);
}

#[test]
fn test_movdiri_32bit_with_r8_r15() {
    // MOVDIRI 32-bit with extended registers
    let code = [
        0x41, 0xb8, 0xAA, 0xBB, 0xCC, 0xDD, // MOV R8D, 0xDDCCBBAA
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x44, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], r8d
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0xDDCCBBAA);
}

#[test]
fn test_movdiri_64bit_with_r8_r15() {
    // MOVDIRI 64-bit with extended registers
    let code = [
        0x49, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // MOV R8, value
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x4c, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], r8
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x8877665544332211);
}

#[test]
fn test_movdiri_32bit_patterns() {
    // MOVDIRI 32-bit with various patterns
    let patterns = [
        0x00000000, 0xFFFFFFFF, 0xAAAAAAAA, 0x55555555, 0x12345678, 0x87654321, 0xDEADBEEF,
        0xCAFEBABE,
    ];

    for (i, &pattern) in patterns.iter().enumerate() {
        let code = [
            0xb8,
            (pattern & 0xFF) as u8,
            ((pattern >> 8) & 0xFF) as u8,
            ((pattern >> 16) & 0xFF) as u8,
            ((pattern >> 24) & 0xFF) as u8, // MOV EAX, pattern
            0x0f,
            0x38,
            0xf9,
            0x03, // MOVDIRI [rbx], eax
            0xf4,
        ];
        let mut regs = Registers::default();
        regs.rbx = 0x2000 + (i as u64 * 4);
        let (mut vcpu, mem) = setup_vm(&code, Some(regs));
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u32(&mem, 0x2000 + (i as u64 * 4)), pattern);
    }
}

#[test]
fn test_movdiri_32bit_overwrites_memory() {
    // MOVDIRI 32-bit overwrites existing memory
    let code = [
        0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pre-fill with different value
    write_mem_at_u32(&mem, 0x2000, 0xFFFFFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0x44332211);
}

#[test]
fn test_movdiri_64bit_overwrites_memory() {
    // MOVDIRI 64-bit overwrites existing memory
    let code = [
        0x48, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // MOV RAX, value
        0xbf, 0x00, 0x20, 0x00, 0x00, // MOV EDI, 0x2000
        0x48, 0x0f, 0x38, 0xf9, 0x07, // MOVDIRI [rdi], RAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Pre-fill with different value
    write_mem_at_u64(&mem, 0x2000, 0xFFFFFFFFFFFFFFFF);

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x8877665544332211);
}

#[test]
fn test_movdiri_32bit_loop_pattern() {
    // Simulate array write pattern
    let code = [
        0xb8, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EAX, 0xDDCCBBAA
        0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], eax
        0x48, 0x83, 0xc3, 0x04, // ADD RBX, 4
        0xb8, 0xEE, 0xFF, 0x00, 0x11, // MOV EAX, 0x1100FFEE
        0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], eax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u32(&mem, 0x2000), 0xDDCCBBAA);
    assert_eq!(read_mem_at_u32(&mem, 0x2004), 0x1100FFEE);
    assert_eq!(regs.rbx, 0x2004);
}

#[test]
fn test_movdiri_64bit_loop_pattern() {
    // Simulate array write pattern with 64-bit
    let code = [
        0x48, 0xb8, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RAX, value1
        0x48, 0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], rax
        0x48, 0x83, 0xc3, 0x08, // ADD RBX, 8
        0x48, 0xb8, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RAX, value2
        0x48, 0x0f, 0x38, 0xf9, 0x03, // MOVDIRI [rbx], rax
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&mem, 0x2000), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&mem, 0x2008), 0x2222222222222222);
    assert_eq!(regs.rbx, 0x2008);
}

#[test]
fn test_movdiri_preserves_all_registers() {
    // Verify MOVDIRI preserves all unrelated registers
    let code = [
        0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, value
        0x41, 0xb8, 0x00, 0x20, 0x00, 0x00, // MOV R8D, 0x2000
        0x41, 0x0f, 0x38, 0xf9, 0x00, // MOVDIRI [r8], EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    regs.rdx = 0x3333333333333333;
    regs.rsi = 0x4444444444444444;
    regs.rdi = 0x5555555555555555;
    let (mut vcpu, _mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x44332211);
    assert_eq!(regs.rbx, 0x1111111111111111);
    assert_eq!(regs.rcx, 0x2222222222222222);
    assert_eq!(regs.rdx, 0x3333333333333333);
    assert_eq!(regs.rsi, 0x4444444444444444);
    assert_eq!(regs.rdi, 0x5555555555555555);
}

#[test]
fn test_movdiri_32bit_power_of_two() {
    // Powers of 2 as test values
    for i in 0..31 {
        let value = 1u32 << i;
        let code = [
            0xb8,
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            0xbf,
            0x00,
            0x20,
            0x00,
            0x00,
            0x0f,
            0x38,
            0xf9,
            0x07,
            0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u32(&mem, 0x2000), value);
    }
}

#[test]
fn test_movdiri_64bit_power_of_two() {
    // Powers of 2 as 64-bit test values
    for i in &[0, 8, 16, 32, 48, 56, 63] {
        let value = 1u64 << i;
        let bytes: [u8; 8] = value.to_le_bytes();
        let code = [
            0x48, 0xb8, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
            bytes[7], 0xbf, 0x00, 0x20, 0x00, 0x00, 0x48, 0x0f, 0x38, 0xf9, 0x07, 0xf4,
        ];
        let (mut vcpu, mem) = setup_vm(&code, None);
        let _regs = run_until_hlt(&mut vcpu).unwrap();

        assert_eq!(read_mem_at_u64(&mem, 0x2000), value);
    }
}
