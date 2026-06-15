use crate::common::*;

// XCHG Extended Tests - Comprehensive patterns for exchange instruction
// XCHG is implicitly atomic (LOCK prefix not needed for memory operands)

// ===== BASIC REGISTER-REGISTER EXCHANGE =====

#[test]
fn test_xchg_8bit_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0xbb, 0x00, 0x00, 0x00, // MOV RBX, 0xBB
        0x86, 0xd8, // XCHG AL, BL
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xBB, "AL should be 0xBB");
    assert_eq!(regs.rbx & 0xFF, 0xAA, "BL should be 0xAA");
}

#[test]
fn test_xchg_16bit_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x48, 0xc7, 0xc3, 0x78, 0x56, 0x00, 0x00, // MOV RBX, 0x5678
        0x66, 0x87, 0xd8, // XCHG AX, BX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x5678, "AX should be 0x5678");
    assert_eq!(regs.rbx & 0xFFFF, 0x1234, "BX should be 0x1234");
}

#[test]
fn test_xchg_32bit_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678
        0x48, 0xc7, 0xc3, 0xef, 0xcd, 0xab, 0x90, // MOV RBX, 0x90ABCDEF
        0x87, 0xd8, // XCHG EAX, EBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x90ABCDEF,
        "EAX should be 0x90ABCDEF"
    );
    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x12345678,
        "EBX should be 0x12345678"
    );
}

#[test]
fn test_xchg_64bit_registers() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0xbb, 0x21, 0x43, 0x65, 0x87, 0x09, 0xba, 0xdc,
        0xfe, // MOV RBX, 0xFEDCBA0987654321
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDCBA0987654321, "RAX should be swapped");
    assert_eq!(regs.rbx, 0x1234567890ABCDEF, "RBX should be swapped");
}

// ===== REGISTER-MEMORY EXCHANGE =====

#[test]
fn test_xchg_8bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x86, 0x03, // XCHG [RBX], AL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x99, "AL should be 0x99");
    assert_eq!(read_mem_u8(&mem), 0x42, "Memory should be 0x42");
}

#[test]
fn test_xchg_16bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x87, 0x03, // XCHG [RBX], AX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x5678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x5678, "AX should be 0x5678");
    assert_eq!(read_mem_u16(&mem), 0x1234, "Memory should be 0x1234");
}

#[test]
fn test_xchg_32bit_reg_mem() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x87, 0x03, // XCHG [RBX], EAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x90ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x90ABCDEF,
        "EAX should be 0x90ABCDEF"
    );
    assert_eq!(
        read_mem_u32(&mem),
        0x12345678,
        "Memory should be 0x12345678"
    );
}

#[test]
fn test_xchg_64bit_reg_mem() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x87, 0x03, // XCHG [RBX], RAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xFEDCBA0987654321);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDCBA0987654321, "RAX should be swapped");
    assert_eq!(
        read_mem_u64(&mem),
        0x1234567890ABCDEF,
        "Memory should be swapped"
    );
}

// ===== XCHG WITH RAX SPECIAL ENCODING =====

#[test]
fn test_xchg_rax_rbx_short_encoding() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV RAX, 0x11111111
        0x48, 0xc7, 0xc3, 0x22, 0x22, 0x22, 0x22, // MOV RBX, 0x22222222
        0x48, 0x93, // XCHG RAX, RBX (short form)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x22222222, "RAX should be 0x22222222");
    assert_eq!(regs.rbx, 0x11111111, "RBX should be 0x11111111");
}

#[test]
fn test_xchg_rax_rcx_short_encoding() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RAX, 0xAAAAAAAA
        0x48, 0xc7, 0xc1, 0xbb, 0xbb, 0xbb, 0xbb, // MOV RCX, 0xBBBBBBBB
        0x48, 0x91, // XCHG RAX, RCX (short form)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends, so 0xBBBBBBBB becomes 0xFFFFFFFFBBBBBBBB
    assert_eq!(
        regs.rax, 0xFFFFFFFFBBBBBBBBu64,
        "RAX should be 0xFFFFFFFFBBBBBBBB"
    );
    assert_eq!(
        regs.rcx, 0xFFFFFFFFAAAAAAAAu64,
        "RCX should be 0xFFFFFFFFAAAAAAAA"
    );
}

// ===== CHAIN EXCHANGES =====

#[test]
fn test_xchg_chain_three_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x87, 0xd8, // XCHG RAX, RBX (RAX=2, RBX=1)
        0x48, 0x87, 0xc8, // XCHG RAX, RCX (RAX=3, RCX=2)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "RAX should be 3");
    assert_eq!(regs.rbx, 1, "RBX should be 1");
    assert_eq!(regs.rcx, 2, "RCX should be 2");
}

#[test]
fn test_xchg_circular_rotation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x48, 0xc7, 0xc1, 0x1e, 0x00, 0x00, 0x00, // MOV RCX, 30
        0x48, 0xc7, 0xc2, 0x28, 0x00, 0x00, 0x00, // MOV RDX, 40
        // Rotate: A<->B, B<->C, C<->D
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0x48, 0x87, 0xd9, // XCHG RBX, RCX
        0x48, 0x87, 0xca, // XCHG RCX, RDX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 20, "RAX rotated");
    assert_eq!(regs.rbx, 30, "RBX rotated");
    assert_eq!(regs.rcx, 40, "RCX rotated");
    assert_eq!(regs.rdx, 10, "RDX rotated");
}

// ===== SELF-EXCHANGE (NOP) =====

#[test]
fn test_xchg_same_register_nop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x87, 0xc0, // XCHG RAX, RAX (NOP)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x42, "RAX should remain unchanged");
}

#[test]
fn test_xchg_rax_rax_is_nop() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RAX, 0xAAAAAAAA
        0x90, // NOP (actually XCHG EAX, EAX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xAAAAAAAA,
        "RAX should remain unchanged"
    );
}

// ===== ADDRESSING MODES =====

#[test]
fn test_xchg_with_displacement() {
    let code = [
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x87, 0x43, 0x08, // XCHG [RBX+8], EAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x99u32.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2008)).unwrap();
    assert_eq!(
        u32::from_le_bytes(buf),
        0x55,
        "Memory at offset should be 0x55"
    );
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x99, "EAX should be 0x99");
}

#[test]
fn test_xchg_with_sib() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0xc7, 0xc2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0x87, 0x04, 0x13, // XCHG [RBX+RDX], EAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0xBBu32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    assert_eq!(
        u32::from_le_bytes(buf),
        0xAA,
        "Memory at SIB address should be 0xAA"
    );
    assert_eq!(regs.rax & 0xFFFFFFFF, 0xBB, "EAX should be 0xBB");
}

// ===== EXTENDED REGISTERS =====

#[test]
fn test_xchg_r8_r9() {
    let code = [
        0x49, 0xc7, 0xc0, 0x11, 0x11, 0x11, 0x11, // MOV R8, 0x11111111
        0x49, 0xc7, 0xc1, 0x22, 0x22, 0x22, 0x22, // MOV R9, 0x22222222
        0x4d, 0x87, 0xc8, // XCHG R8, R9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x22222222, "R8 should be 0x22222222");
    assert_eq!(regs.r9, 0x11111111, "R9 should be 0x11111111");
}

#[test]
fn test_xchg_r14_r15() {
    let code = [
        0x49, 0xc7, 0xc6, 0xaa, 0xaa, 0xaa, 0xaa, // MOV R14, 0xAAAAAAAA
        0x49, 0xc7, 0xc7, 0xbb, 0xbb, 0xbb, 0xbb, // MOV R15, 0xBBBBBBBB
        0x4d, 0x87, 0xfe, // XCHG R14, R15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // MOV r64, imm32 sign-extends, so 0xBBBBBBBB becomes 0xFFFFFFFFBBBBBBBB
    assert_eq!(
        regs.r14, 0xFFFFFFFFBBBBBBBBu64,
        "R14 should be 0xFFFFFFFFBBBBBBBB"
    );
    assert_eq!(
        regs.r15, 0xFFFFFFFFAAAAAAAAu64,
        "R15 should be 0xFFFFFFFFAAAAAAAA"
    );
}

// ===== PRACTICAL PATTERNS =====

#[test]
fn test_xchg_swap_variables() {
    let code = [
        0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00, // MOV RAX, 42
        0x48, 0xc7, 0xc3, 0x45, 0x00, 0x00, 0x00, // MOV RBX, 69
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 69, "Variables swapped");
    assert_eq!(regs.rbx, 42, "Variables swapped");
}

#[test]
fn test_xchg_lock_free_update() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x87, 0x03, // XCHG [RBX], RAX (implicitly atomic)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 200);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 200, "RAX has old value");
    assert_eq!(read_mem_u64(&mem), 100, "Memory has new value");
}

#[test]
fn test_xchg_multiple_memory_locations() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x87, 0x03, // XCHG [RBX], EAX (mem1 and RAX)
        0x48, 0xc7, 0xc3, 0x10, 0x20, 0x00, 0x00, // MOV RBX, 0x2010
        0x87, 0x03, // XCHG [RBX], EAX (mem2 and RAX)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x22u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x33u32.to_le_bytes(), GuestAddress(0x2010))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf1 = [0u8; 4];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(
        u32::from_le_bytes(buf1),
        0x11,
        "First location gets RAX initial value"
    );

    let mut buf2 = [0u8; 4];
    mem.read_slice(&mut buf2, GuestAddress(0x2010)).unwrap();
    assert_eq!(
        u32::from_le_bytes(buf2),
        0x22,
        "Second location gets first location value"
    );

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x33,
        "RAX gets second location value"
    );
}

#[test]
fn test_xchg_boundary_values() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, max
        0x48, 0x89, 0xc3, // MOV RBX, RAX (RBX = max)
        0x48, 0x31, 0xc0, // XOR RAX, RAX (RAX = 0)
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should be max");
    assert_eq!(regs.rbx, 0, "RBX should be 0");
}

#[test]
fn test_xchg_preserves_upper_bits_32bit() {
    let code = [
        0x48, 0xb8, 0x78, 0x56, 0x34, 0x12, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFF12345678
        0x48, 0xbb, 0xef, 0xcd, 0xab, 0x90, 0xff, 0xff, 0xff,
        0xff, // MOV RBX, 0xFFFFFFFF90ABCDEF
        0x87, 0xd8, // XCHG EAX, EBX (32-bit)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 32-bit operations zero upper 32 bits
    assert_eq!(regs.rax, 0x90ABCDEF, "RAX upper bits should be zeroed");
    assert_eq!(regs.rbx, 0x12345678, "RBX upper bits should be zeroed");
}
