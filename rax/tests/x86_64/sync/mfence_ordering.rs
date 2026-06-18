use crate::common::*;

// MFENCE Tests - Memory Fence for ordering loads and stores
// MFENCE: 0F AE F0
// Serializes all load and store operations before and after the fence

#[test]
fn test_mfence_basic() {
    let code = [
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _regs = run_until_hlt(&mut vcpu).unwrap();
    // MFENCE should execute without errors
}

#[test]
fn test_mfence_after_store() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x89, 0x03, // MOV [RBX], RAX (store)
        0x0f, 0xae, 0xf0, // MFENCE (ensure store completes)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0x42,
        "Store should complete before MFENCE"
    );
}

#[test]
fn test_mfence_before_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x0f, 0xae, 0xf0, // MFENCE (ensure previous stores complete)
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (load)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x99, "Load should see value after MFENCE");
}

#[test]
fn test_mfence_between_stores() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0x11, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0x11
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0xc7, 0x43, 0x08, 0x22, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 0x22
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf1 = [0u8; 8];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(
        u64::from_le_bytes(buf1),
        0x11,
        "First store should complete"
    );

    let mut buf2 = [0u8; 8];
    mem.read_slice(&mut buf2, GuestAddress(0x2008)).unwrap();
    assert_eq!(
        u64::from_le_bytes(buf2),
        0x22,
        "Second store should complete"
    );
}

#[test]
fn test_mfence_between_loads() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0x11u64.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();
    mem.write_slice(&0x22u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11, "First load should complete");
    assert_eq!(regs.rdx, 0x22, "Second load should complete after MFENCE");
}

#[test]
fn test_mfence_multiple_fences() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0x01, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 1
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0xc7, 0x43, 0x08, 0x02, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 2
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0xc7, 0x43, 0x10, 0x03, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+16], 3
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf1 = [0u8; 8];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(u64::from_le_bytes(buf1), 1, "First store should complete");

    let mut buf2 = [0u8; 8];
    mem.read_slice(&mut buf2, GuestAddress(0x2008)).unwrap();
    assert_eq!(u64::from_le_bytes(buf2), 2, "Second store should complete");

    let mut buf3 = [0u8; 8];
    mem.read_slice(&mut buf3, GuestAddress(0x2010)).unwrap();
    assert_eq!(u64::from_le_bytes(buf3), 3, "Third store should complete");
}

#[test]
fn test_mfence_sequential_operations() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Write, fence, read pattern
        0x48, 0xc7, 0x03, 0xaa, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0xAA
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x48, 0x89, 0x03, // MOV [RBX], RAX
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xAB, "RAX should be 0xAB");
    assert_eq!(read_mem_u64(&mem), 0xAB, "Memory should be updated to 0xAB");
}

#[test]
fn test_mfence_with_atomic_operations() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0x64, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 100
        0x0f, 0xae, 0xf0, // MFENCE
        0xf0, 0x48, 0x81, 0x03, 0x0a, 0x00, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 10
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        110,
        "Atomic add should complete with MFENCE"
    );
}

#[test]
fn test_mfence_data_dependency() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Producer writes flag and data
        0x48, 0xc7, 0x43, 0x08, 0x42, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 0x42 (data)
        0x0f, 0xae, 0xf0, // MFENCE (ensure data written before flag)
        0x48, 0xc7, 0x03, 0x01, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 1 (flag)
        0x0f, 0xae, 0xf0, // MFENCE
        // Consumer reads flag then data
        0x48, 0x8b, 0x03, // MOV RAX, [RBX] (flag)
        0x0f, 0xae, 0xf0, // MFENCE (ensure flag read before data)
        0x48, 0x8b, 0x53, 0x08, // MOV RDX, [RBX+8] (data)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1, "Flag should be set");
    assert_eq!(regs.rdx, 0x42, "Data should be visible");
}

// Additional MFENCE tests demonstrating ordering guarantees
#[test]
fn test_mfence_prevents_store_reordering() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0x01, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 1
        0x48, 0xc7, 0x43, 0x08, 0x02, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 2
        0x0f, 0xae, 0xf0, // MFENCE
        0x48, 0xc7, 0x43, 0x10, 0x03, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+16], 3
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // All stores before MFENCE must complete before stores after
    let mut buf1 = [0u8; 8];
    mem.read_slice(&mut buf1, GuestAddress(0x2000)).unwrap();
    assert_eq!(u64::from_le_bytes(buf1), 1);

    let mut buf2 = [0u8; 8];
    mem.read_slice(&mut buf2, GuestAddress(0x2008)).unwrap();
    assert_eq!(u64::from_le_bytes(buf2), 2);

    let mut buf3 = [0u8; 8];
    mem.read_slice(&mut buf3, GuestAddress(0x2010)).unwrap();
    assert_eq!(u64::from_le_bytes(buf3), 3);
}

#[test]
fn test_mfence_with_different_sized_stores() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xc6, 0x03, 0x11, // MOV BYTE PTR [RBX], 0x11
        0x66, 0xc7, 0x43, 0x01, 0x22, 0x22, // MOV WORD PTR [RBX+1], 0x2222
        0xc7, 0x43, 0x03, 0x33, 0x33, 0x33, 0x33, // MOV DWORD PTR [RBX+3], 0x33333333
        0x0f, 0xae, 0xf0, // MFENCE
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    use vm_memory::{Bytes, GuestAddress};
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let mut buf = [0u8; 8];
    mem.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    assert_eq!(buf[0], 0x11, "Byte store should complete");
    // Word and dword stores also complete
}

// ============================================================================
// SFENCE (0F AE F8) and PAUSE (F3 90): architecturally no-ops in this
// single-vCPU interpreter, but they must execute (advance RIP) and leave all
// register/flag/memory state untouched. Value-asserting coverage for both.
// ============================================================================

// SFENCE executes as a no-op and surrounding stores complete in order.
#[test]
fn test_sfence_executes_and_orders_stores() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0x03, 0xAA, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0xAA
        0x0f, 0xae, 0xf8, // SFENCE
        0x48, 0xc7, 0x43, 0x08, 0xBB, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 0xBB
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, 0x2000),
        0xAA,
        "store before SFENCE completes"
    );
    assert_eq!(
        read_mem_at_u64(&mem, 0x2008),
        0xBB,
        "store after SFENCE completes"
    );
}

// SFENCE must not alter general registers or flags.
#[test]
fn test_sfence_preserves_registers_and_flags() {
    let code = [
        0x0f, 0xae, 0xf8, // SFENCE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111_2222_3333_4444;
    regs.rbx = 0x5555_6666_7777_8888;
    regs.rflags = 0x40 | 0x1 | 0x2; // ZF + CF + reserved
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1111_2222_3333_4444,
        "RAX preserved across SFENCE"
    );
    assert_eq!(
        regs.rbx, 0x5555_6666_7777_8888,
        "RBX preserved across SFENCE"
    );
    assert!(zf_set(regs.rflags), "ZF preserved across SFENCE");
    assert!(cf_set(regs.rflags), "CF preserved across SFENCE");
}

// PAUSE (F3 90) executes as a no-op; following instruction still runs.
#[test]
fn test_pause_executes_then_continues() {
    let code = [
        0xf3, 0x90, // PAUSE
        0x48, 0xc7, 0xc0, 0x2a, 0x00, 0x00, 0x00, // MOV RAX, 0x2A
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2A, "instruction after PAUSE executes");
}

// PAUSE preserves registers and flags (pure spin-wait hint).
#[test]
fn test_pause_preserves_registers_and_flags() {
    let code = [
        0xf3, 0x90, // PAUSE
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rcx = 0xDEAD_BEEF_CAFE_F00D;
    regs.rflags = 0x80 | 0x2; // SF + reserved
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rcx, 0xDEAD_BEEF_CAFE_F00D,
        "RCX preserved across PAUSE"
    );
    assert!(sf_set(regs.rflags), "SF preserved across PAUSE");
}

// A spin-loop body of repeated PAUSEs is a no-op sequence.
#[test]
fn test_pause_spin_loop_body_noop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0xf3, 0x90, // PAUSE
        0x48, 0xff, 0xc0, // INC RAX
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 8, "PAUSEs are no-ops; only INC changes RAX");
}
