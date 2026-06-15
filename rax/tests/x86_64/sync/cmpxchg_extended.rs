use crate::common::*;

// CMPXCHG Extended Tests - Comprehensive patterns for atomic compare-and-exchange
// This extends the basic CMPXCHG tests with more complex scenarios

// ===== MEMORY OPERAND TESTS =====

#[test]
fn test_cmpxchg_8bit_mem_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (AL = 0x42)
        0x48, 0xc7, 0xc1, 0x55, 0x00, 0x00, 0x00, // MOV RCX, 0x55 (new value)
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb0, 0x0b, // CMPXCHG [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x55, "Memory should be updated to 0x55");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_8bit_mem_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc1, 0x55, 0x00, 0x00, 0x00, // MOV RCX, 0x55
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb0, 0x0b, // CMPXCHG [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u8(&mem), 0x99, "Memory should remain 0x99");
    assert_eq!(regs.rax & 0xFF, 0x99, "AL should be loaded from memory");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_16bit_mem_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x48, 0xc7, 0xc1, 0x78, 0x56, 0x00, 0x00, // MOV RCX, 0x5678
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u16(&mem), 0x5678, "Memory should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_16bit_mem_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x48, 0xc7, 0xc1, 0x78, 0x56, 0x00, 0x00, // MOV RCX, 0x5678
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xABCD);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u16(&mem), 0xABCD, "Memory should remain unchanged");
    assert_eq!(regs.rax & 0xFFFF, 0xABCD, "AX should be loaded from memory");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_32bit_mem_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678
        0x48, 0xc7, 0xc1, 0xef, 0xcd, 0xab, 0x90, // MOV RCX, 0x90ABCDEF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x90ABCDEF, "Memory should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_32bit_mem_failure() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678
        0x48, 0xc7, 0xc1, 0xef, 0xcd, 0xab, 0x90, // MOV RCX, 0x90ABCDEF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xDEADBEEF,
        "Memory should remain unchanged"
    );
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDEADBEEF,
        "EAX should be loaded from memory"
    );
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_64bit_mem_success() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0xb9, 0x21, 0x43, 0x65, 0x87, 0x09, 0xba, 0xdc,
        0xfe, // MOV RCX, 0xFEDCBA0987654321
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1234567890ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xFEDCBA0987654321,
        "Memory should be updated"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_64bit_mem_failure() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0xb9, 0x21, 0x43, 0x65, 0x87, 0x09, 0xba, 0xdc,
        0xfe, // MOV RCX, 0xFEDCBA0987654321
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xCAFEBABEDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xCAFEBABEDEADBEEF,
        "Memory should remain unchanged"
    );
    assert_eq!(
        regs.rax, 0xCAFEBABEDEADBEEF,
        "RAX should be loaded from memory"
    );
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

// ===== LOCK PREFIX TESTS =====

#[test]
fn test_cmpxchg_lock_8bit_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x0f, 0xb0, 0x0b, // LOCK CMPXCHG [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u8(&mem),
        0x99,
        "Memory should be atomically updated"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_lock_16bit_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x34, 0x12, 0x00, 0x00, // MOV RAX, 0x1234
        0x48, 0xc7, 0xc1, 0xcd, 0xab, 0x00, 0x00, // MOV RCX, 0xABCD
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x66, 0x0f, 0xb1, 0x0b, // LOCK CMPXCHG [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u16(&mem),
        0xABCD,
        "Memory should be atomically updated"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_lock_32bit_success() {
    let code = [
        0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12, // MOV RAX, 0x12345678
        0x48, 0xc7, 0xc1, 0xef, 0xbe, 0xad, 0xde, // MOV RCX, 0xDEADBEEF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x0f, 0xb1, 0x0b, // LOCK CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xDEADBEEF,
        "Memory should be atomically updated"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_lock_64bit_success() {
    let code = [
        0x48, 0xb8, 0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34,
        0x12, // MOV RAX, 0x1234567890ABCDEF
        0x48, 0xb9, 0xef, 0xbe, 0xad, 0xde, 0xbe, 0xba, 0xfe,
        0xca, // MOV RCX, 0xCAFEBABEDEADBEEF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x0f, 0xb1, 0x0b, // LOCK CMPXCHG [RBX], RCX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1234567890ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xCAFEBABEDEADBEEF,
        "Memory should be atomically updated"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== BOUNDARY AND EDGE CASE TESTS =====

#[test]
fn test_cmpxchg_crossing_boundary_8bit() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0xff, 0x00, 0x00, 0x00, // MOV RCX, 0xFF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb0, 0x0b, // CMPXCHG [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u8(&mem),
        0xFF,
        "Memory should be updated to max value"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_crossing_boundary_16bit() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0x00, 0x00, // MOV RCX, 0xFFFF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], CX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u16(&mem),
        0xFFFF,
        "Memory should be updated to max value"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_signed_negative_mem() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (0xFFFFFFFF)
        0x48, 0xc7, 0xc1, 0xfe, 0xff, 0xff, 0xff, // MOV RCX, -2 (0xFFFFFFFE)
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0xFFFFFFFE, "Memory should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_max_to_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x48, 0x31, 0xc9, // XOR RCX, RCX
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0, "Memory should be updated to zero");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_zero_to_max() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFF
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xFFFFFFFF,
        "Memory should be updated to max"
    );
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== RETRY LOOP PATTERNS =====

#[test]
fn test_cmpxchg_retry_pattern_first_attempt() {
    let code = [
        // Load expected value
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x8b, 0x03, // MOV EAX, [RBX]
        // Try to update
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 50);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 100, "Memory should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set on success");
}

#[test]
fn test_cmpxchg_retry_pattern_failure() {
    let code = [
        // Load expected value (will be stale)
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Try to update (will fail)
        0x48, 0xc7, 0xc1, 0x64, 0x00, 0x00, 0x00, // MOV RCX, 100
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 75); // Different value
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 75, "Memory should remain unchanged");
    assert_eq!(regs.rax & 0xFFFFFFFF, 75, "EAX should have current value");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear on failure");
}

// ===== SEQUENTIAL CAS OPERATIONS =====

#[test]
fn test_cmpxchg_sequential_updates() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // First CAS: 0 -> 1
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        // Second CAS: 1 -> 2
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        // Third CAS: 2 -> 3
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 3, "Memory should be updated to 3");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== DIFFERENT ADDRESSING MODES =====

#[test]
fn test_cmpxchg_with_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc1, 0xbb, 0x00, 0x00, 0x00, // MOV RCX, 0xBB
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb0, 0x4b, 0x04, // CMPXCHG [RBX+4], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&[0xAA], GuestAddress(0x2004)).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 1];
    mem.read_slice(&mut buf, GuestAddress(0x2004)).unwrap();
    assert_eq!(buf[0], 0xBB, "Memory at offset should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_cmpxchg_with_sib() {
    let code = [
        0x48, 0xc7, 0xc0, 0x55, 0x55, 0x55, 0x55, // MOV RAX, 0x55555555
        0x48, 0xc7, 0xc1, 0xaa, 0xaa, 0xaa, 0xaa, // MOV RCX, 0xAAAAAAAA
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x48, 0xc7, 0xc2, 0x00, 0x10, 0x00, 0x00, // MOV RDX, 0x1000
        0x0f, 0xb1, 0x0c, 0x13, // CMPXCHG [RBX+RDX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x55555555u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    let val = u32::from_le_bytes(buf);
    assert_eq!(val, 0xAAAAAAAA, "Memory at SIB address should be updated");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}

// ===== FLAG COMBINATIONS =====

#[test]
fn test_cmpxchg_cf_set_unsigned_compare() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 10);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Comparison is 5 - 10 (borrow needed)
    assert_ne!(regs.rflags & 0x01, 0, "CF should be set (5 < 10)");
    assert_eq!(regs.rflags & 0x40, 0, "ZF should be clear");
}

#[test]
fn test_cmpxchg_sf_set_on_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Comparison is 5 - 100 (negative result)
    assert_ne!(regs.rflags & 0x80, 0, "SF should be set (negative)");
}

#[test]
fn test_cmpxchg_pf_even_parity() {
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Result is 0 (even parity)
    assert_ne!(regs.rflags & 0x04, 0, "PF should be set (even parity)");
}

#[test]
fn test_cmpxchg_af_auxiliary_carry() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Comparison 0x10 - 0x01 = 0x0F causes AF (borrow from bit 4 to bit 3)
    assert_ne!(regs.rflags & 0x10, 0, "AF should be set");
}

// ===== STRESS PATTERNS =====

#[test]
fn test_cmpxchg_alternating_success_failure() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Success
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        // Failure
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        // Success again
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 4, "Final value should be 4");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set (last was success)");
}

// ===== REGISTER PRESERVATION TESTS =====

#[test]
fn test_cmpxchg_preserves_other_registers() {
    let code = [
        // Set up non-participating registers
        0x48, 0xc7, 0xc2, 0x11, 0x11, 0x11, 0x11, // MOV RDX, 0x11111111
        0x48, 0xc7, 0xc6, 0x22, 0x22, 0x22, 0x22, // MOV RSI, 0x22222222
        0x48, 0xc7, 0xc7, 0x33, 0x33, 0x33, 0x33, // MOV RDI, 0x33333333
        // CMPXCHG operation
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0x11111111, "RDX should be preserved");
    assert_eq!(regs.rsi, 0x22222222, "RSI should be preserved");
    assert_eq!(regs.rdi, 0x33333333, "RDI should be preserved");
}

#[test]
fn test_cmpxchg_8bit_preserves_upper_bits() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xb0, 0x0b, // CMPXCHG [RBX], CL
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 0x42);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x42, "AL should be loaded from memory");
    assert_eq!(
        (regs.rax >> 8) & 0xFF,
        0xFF,
        "Upper bits should be preserved"
    );
}

// ===== MULTIPLE MEMORY LOCATIONS =====

#[test]
fn test_cmpxchg_different_addresses() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // First location
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        // Second location
        0x48, 0xc7, 0xc3, 0x08, 0x20, 0x00, 0x00, // MOV RBX, 0x2008
        0x48, 0xc7, 0xc0, 0x33, 0x00, 0x00, 0x00, // MOV RAX, 0x33
        0x48, 0xc7, 0xc1, 0x44, 0x00, 0x00, 0x00, // MOV RCX, 0x44
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x11u32.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x33u32.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf1 = [0u8; 4];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(u32::from_le_bytes(buf1), 0x22, "First location updated");

    let mut buf2 = [0u8; 4];
    mem.read_slice(&mut buf2, GuestAddress(0x2008)).unwrap();
    assert_eq!(u32::from_le_bytes(buf2), 0x44, "Second location updated");
}

// ===== ABA PROBLEM SIMULATION =====

#[test]
fn test_cmpxchg_aba_scenario() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // A: Value is 10
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc1, 0x14, 0x00, 0x00, 0x00, // MOV RCX, 20
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX (10 -> 20)
        // B: Change to different value
        0x48, 0xc7, 0xc0, 0x14, 0x00, 0x00, 0x00, // MOV RAX, 20
        0x48, 0xc7, 0xc1, 0x1e, 0x00, 0x00, 0x00, // MOV RCX, 30
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX (20 -> 30)
        // A: Back to original (would succeed incorrectly if not careful)
        0x48, 0xc7, 0xc0, 0x1e, 0x00, 0x00, 0x00, // MOV RAX, 30
        0x48, 0xc7, 0xc1, 0x0a, 0x00, 0x00, 0x00, // MOV RCX, 10
        0x0f, 0xb1, 0x0b, // CMPXCHG [RBX], ECX (30 -> 10)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 10);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 10, "Memory back to original value");
    assert_ne!(regs.rflags & 0x40, 0, "ZF should be set");
}
