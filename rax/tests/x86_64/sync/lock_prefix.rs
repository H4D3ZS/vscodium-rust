use crate::common::{
    cf_set, of_set, pf_set, read_mem_u8, read_mem_u16, read_mem_u32, read_mem_u64, run_until_hlt,
    setup_vm, setup_vm_no_idt, sf_set, write_mem_u8, write_mem_u16, write_mem_u32, write_mem_u64,
    zf_set,
};
use rax::cpu::{Registers, VCpu, VcpuExit};

// LOCK Prefix Tests - Comprehensive tests for LOCK prefix with various instructions
// The LOCK prefix (0xF0) ensures atomic execution on multiprocessor systems

// ===== LOCK ADD TESTS =====

#[test]
fn test_lock_add_8bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x80, 0x03, 0x05, // LOCK ADD BYTE PTR [RBX], 5
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 10);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u8(&mem),
        15,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_add_16bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x66, 0x81, 0x03, 0xe8, 0x03, // LOCK ADD WORD PTR [RBX], 1000
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 5000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u16(&mem),
        6000,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_add_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x03, 0x64, 0x00, 0x00, 0x00, // LOCK ADD DWORD PTR [RBX], 100
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        1100,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_add_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x03, 0x10, 0x27, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 10000
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 100000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        110000,
        "Memory should be atomically incremented"
    );
}

// ===== LOCK SUB TESTS =====

#[test]
fn test_lock_sub_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x2b, 0x32, 0x00, 0x00, 0x00, // LOCK SUB DWORD PTR [RBX], 50
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 200);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        150,
        "Memory should be atomically decremented"
    );
}

#[test]
fn test_lock_sub_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x2b, 0xe8, 0x03, 0x00, 0x00, // LOCK SUB QWORD PTR [RBX], 1000
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 5000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        4000,
        "Memory should be atomically decremented"
    );
}

// ===== LOCK AND TESTS =====

#[test]
fn test_lock_and_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x23, 0x0f, 0x00, 0x00, 0x00, // LOCK AND DWORD PTR [RBX], 0x0F
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0x0F,
        "Memory should be atomically ANDed"
    );
}

#[test]
fn test_lock_and_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x23, 0xff, 0xff, 0x00, 0x00, // LOCK AND QWORD PTR [RBX], 0xFFFF
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xCDEF,
        "Memory should be atomically ANDed"
    );
}

// ===== LOCK OR TESTS =====

#[test]
fn test_lock_or_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x0b, 0xf0, 0x00, 0x00, 0x00, // LOCK OR DWORD PTR [RBX], 0xF0
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x0F);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0xFF, "Memory should be atomically ORed");
}

#[test]
fn test_lock_or_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x0b, 0x00, 0x00, 0xff, 0x00, // LOCK OR QWORD PTR [RBX], 0xFF0000
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x12345678);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0x12FF5678,
        "Memory should be atomically ORed"
    );
}

// ===== LOCK XOR TESTS =====

#[test]
fn test_lock_xor_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x33, 0xff, 0xff, 0xff, 0xff, // LOCK XOR DWORD PTR [RBX], 0xFFFFFFFF
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xEDCBA987,
        "Memory should be atomically XORed"
    );
}

#[test]
fn test_lock_xor_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x33, 0xff, 0xff, 0xff,
        0xff, // LOCK XOR QWORD PTR [RBX], 0xFFFFFFFF
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x1234567890ABCDEF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xEDCBA987_6F543210,
        "Memory should be atomically XORed"
    );
}

// ===== LOCK INC/DEC TESTS =====

#[test]
fn test_lock_inc_8bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0xfe, 0x03, // LOCK INC BYTE PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u8(&mem, 99);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u8(&mem),
        100,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_inc_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0xff, 0x03, // LOCK INC DWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 999);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        1000,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_dec_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0xff, 0x0b, // LOCK DEC DWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1001);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        1000,
        "Memory should be atomically decremented"
    );
}

#[test]
fn test_lock_inc_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 9999999);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        10000000,
        "Memory should be atomically incremented"
    );
}

#[test]
fn test_lock_dec_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0xff, 0x0b, // LOCK DEC QWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 10000001);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        10000000,
        "Memory should be atomically decremented"
    );
}

// ===== LOCK NEG TESTS =====

#[test]
fn test_lock_neg_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0xf7, 0x1b, // LOCK NEG DWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 42);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem) as i32,
        -42,
        "Memory should be atomically negated"
    );
}

#[test]
fn test_lock_neg_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0xf7, 0x1b, // LOCK NEG QWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 1000);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem) as i64,
        -1000,
        "Memory should be atomically negated"
    );
}

// ===== LOCK NOT TESTS =====

#[test]
fn test_lock_not_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0xf7, 0x13, // LOCK NOT DWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xEDCBA987,
        "Memory should be atomically NOTed"
    );
}

#[test]
fn test_lock_not_64bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0xf7, 0x13, // LOCK NOT QWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0xEDCBA98765432110 - 1,
        "Memory should be atomically NOTed"
    );
}

// ===== LOCK BTC/BTR/BTS TESTS =====

#[test]
fn test_lock_bts_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x0f, 0xba, 0x2b, 0x08, // LOCK BTS DWORD PTR [RBX], 8
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x100, "Bit 8 should be atomically set");
}

#[test]
fn test_lock_btr_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x0f, 0xba, 0x33, 0x08, // LOCK BTR DWORD PTR [RBX], 8
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFFFFF);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xFFFFFEFF,
        "Bit 8 should be atomically reset"
    );
}

#[test]
fn test_lock_btc_32bit_memory() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x0f, 0xba, 0x3b, 0x08, // LOCK BTC DWORD PTR [RBX], 8
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0x100,
        "Bit 8 should be atomically complemented"
    );
}

// ===== PRACTICAL ATOMIC PATTERNS =====

#[test]
fn test_lock_sequence_counter_increment() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Multiple atomic increments
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf0, 0x48, 0xff, 0x03, // LOCK INC QWORD PTR [RBX]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 100);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        105,
        "Counter should be incremented 5 times"
    );
}

#[test]
fn test_lock_flags_operations() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Set flag bit 0
        0xf0, 0x0f, 0xba, 0x2b, 0x00, // LOCK BTS DWORD PTR [RBX], 0
        // Set flag bit 1
        0xf0, 0x0f, 0xba, 0x2b, 0x01, // LOCK BTS DWORD PTR [RBX], 1
        // Set flag bit 7
        0xf0, 0x0f, 0xba, 0x2b, 0x07, // LOCK BTS DWORD PTR [RBX], 7
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x83, "Bits 0, 1, and 7 should be set");
}

#[test]
fn test_lock_reference_count_pattern() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Add 3 references
        0xf0, 0x48, 0x81, 0x03, 0x03, 0x00, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 3
        // Remove 1 reference
        0xf0, 0x48, 0x81, 0x2b, 0x01, 0x00, 0x00, 0x00, // LOCK SUB QWORD PTR [RBX], 1
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 1);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u64(&mem), 3, "Reference count should be 3");
}

#[test]
fn test_lock_bitmask_operations() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        // Set some bits
        0xf0, 0x81, 0x0b, 0x0f, 0xf0, 0x00, 0x00, // LOCK OR DWORD PTR [RBX], 0xF00F
        // Clear some bits
        0xf0, 0x81, 0x23, 0xf0, 0xff, 0xff, 0xff, // LOCK AND DWORD PTR [RBX], 0xFFFFFFF0
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0xF000,
        "Bitmask should be applied atomically"
    );
}

#[test]
fn test_lock_accumulator_pattern() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x48, 0x81, 0x03, 0x0a, 0x00, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 10
        0xf0, 0x48, 0x81, 0x03, 0x14, 0x00, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 20
        0xf0, 0x48, 0x81, 0x03, 0x1e, 0x00, 0x00, 0x00, // LOCK ADD QWORD PTR [RBX], 30
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u64(&mem), 60, "Accumulator should sum all values");
}

// ============================================================================
// LOCK prefix enforcement (#UD on illegal use) + atomic RMW correctness.
//
// The LOCK prefix (0xF0) is architecturally legal ONLY on the
// memory-destination read-modify-write forms of ADD/ADC/AND/BTC/BTR/BTS/
// CMPXCHG/CMPXCHG8B/CMPXCHG16B/DEC/INC/NEG/NOT/OR/SBB/SUB/XOR/XADD/XCHG.
// Using it on any other opcode, or on a lockable opcode whose destination is a
// register (ModR/M mod == 3), raises #UD (vector 6). rax is a single-vCPU
// interpreter so the RMW is already atomic; the added behaviour is the #UD
// enforcement and correct XADD/CMPXCHG results+flags.
//
// The #UD tests use `setup_vm_no_idt`: with no present IDT entry, delivering
// #UD surfaces as an Err / non-HLT exit, so reaching HLT means the LOCK was
// (incorrectly) allowed to execute. This mirrors tests/x86_64/misc/ud.rs.
// ============================================================================

/// Helper mirroring the ud.rs harness: returns true iff the run hit HLT
/// (i.e. NO exception was raised).
fn reached_hlt(code: &[u8], regs: Option<Registers>) -> bool {
    let (mut vcpu, _mem) = setup_vm_no_idt(code, regs);
    matches!(vcpu.run(), Ok(VcpuExit::Hlt))
}

// ----- Positive: LOCK ADD [mem], reg works (item 1 of the task) -----

#[test]
fn test_lock_add_mem_reg_works() {
    // LOCK ADD DWORD PTR [RAX], EBX   (F0 01 18: ModRM mod=00 reg=011(EBX) rm=000(RAX))
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0xf0, 0x01, 0x18, // LOCK ADD [RAX], EBX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        105,
        "LOCK ADD [mem], reg must add atomically"
    );
    assert!(!zf_set(regs.rflags), "ZF clear (result non-zero)");
    assert!(!cf_set(regs.rflags), "CF clear (no carry)");
}

#[test]
fn test_lock_add_mem_reg_sets_flags() {
    // LOCK ADD DWORD PTR [RAX], EBX with operands that wrap to zero -> ZF + CF.
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf0, 0x01, 0x18, // LOCK ADD [RAX], EBX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFF_FFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0, "0xFFFFFFFF + 1 wraps to 0");
    assert!(zf_set(regs.rflags), "ZF set (result zero)");
    assert!(cf_set(regs.rflags), "CF set (carry out)");
}

// ----- #UD: LOCK on a register destination -----

#[test]
fn test_lock_add_reg_dest_ud() {
    // LOCK ADD EBX, EAX   (F0 01 C3: ModRM mod=11 -> register destination) => #UD
    let code = [
        0xf0, 0x01, 0xc3, // LOCK ADD EBX, EAX (register dest is illegal w/ LOCK)
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(
        !reached_hlt(&code, None),
        "LOCK on register dest must raise #UD"
    );
}

#[test]
fn test_lock_inc_reg_dest_ud() {
    // LOCK INC EAX  (F0 FF C0: group5 /0 INC with mod=11 register dest) => #UD
    let code = [
        0xf0, 0xff, 0xc0, // LOCK INC EAX (register dest)
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(!reached_hlt(&code, None), "LOCK INC reg must raise #UD");
}

// ----- #UD: LOCK on a non-lockable opcode -----

#[test]
fn test_lock_mov_ud() {
    // LOCK MOV [RAX], EBX  (F0 89 18) -- MOV is never lockable, even to memory.
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let code = [
        0xf0, 0x89, 0x18, // LOCK MOV [RAX], EBX
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(!reached_hlt(&code, Some(regs)), "LOCK MOV must raise #UD");
}

#[test]
fn test_lock_cmp_mem_ud() {
    // LOCK CMP DWORD PTR [RAX], 0  (F0 81 38 ..): group1 /7 = CMP, not lockable
    // even with a memory destination, because CMP does not write back.
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let code = [
        0xf0, 0x81, 0x38, 0x00, 0x00, 0x00, 0x00, // LOCK CMP [RAX], 0
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(!reached_hlt(&code, Some(regs)), "LOCK CMP must raise #UD");
}

#[test]
fn test_lock_test_mem_ud() {
    // LOCK TEST DWORD PTR [RAX], imm32  (F0 F7 00 ..): group3 /0 = TEST, not lockable.
    let mut regs = Registers::default();
    regs.rax = 0x2000;
    let code = [
        0xf0, 0xf7, 0x00, 0x01, 0x00, 0x00, 0x00, // LOCK TEST [RAX], 1
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(!reached_hlt(&code, Some(regs)), "LOCK TEST must raise #UD");
}

// ----- LOCK XADD correctness + #UD on register destination -----

#[test]
fn test_lock_xadd_mem_reg_works() {
    // LOCK XADD DWORD PTR [RAX], EBX  (F0 0F C1 18)
    // DEST = DEST + SRC; SRC(=EBX) = old DEST.
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0xf0, 0x0f, 0xc1, 0x18, // LOCK XADD [RAX], EBX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 110, "XADD: memory = old(100) + EBX(10)");
    assert_eq!(
        regs.rbx & 0xFFFF_FFFF,
        100,
        "XADD: EBX = old memory value (100)"
    );
}

#[test]
fn test_lock_xadd_reg_dest_ud() {
    // LOCK XADD EBX, EAX  (F0 0F C1 C3: ModRM mod=11 register dest) => #UD
    let code = [
        0xf0, 0x0f, 0xc1, 0xc3, // LOCK XADD EBX, EAX (register dest)
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(
        !reached_hlt(&code, None),
        "LOCK XADD reg dest must raise #UD"
    );
}

// ----- LOCK CMPXCHG correctness (both branches) + #UD on register destination -----

#[test]
fn test_lock_cmpxchg_mem_reg_success() {
    // LOCK CMPXCHG DWORD PTR [RDI], EBX  (F0 0F B1 1F)
    // EAX == DEST -> ZF=1, DEST = EBX (source). The pointer is held in RDI (not
    // the accumulator EAX, which CMPXCHG compares against the destination).
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000 (pointer)
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99 (new/source value)
        0xb8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100 (accumulator = expected)
        0xf0, 0x0f, 0xb1, 0x1f, // LOCK CMPXCHG [RDI], EBX (ModRM mod=00 reg=011 rm=111)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 100); // DEST == EAX -> success branch
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0x99,
        "CMPXCHG success: DEST takes source EBX"
    );
    assert!(zf_set(regs.rflags), "CMPXCHG success sets ZF");
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        100,
        "Accumulator unchanged on success"
    );
}

#[test]
fn test_lock_cmpxchg_mem_reg_failure() {
    // LOCK CMPXCHG DWORD PTR [RDI], EBX  -- EAX != DEST -> ZF=0, EAX = DEST.
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000 (pointer)
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99 (source value)
        0xb8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100 (accumulator = expected)
        0xf0, 0x0f, 0xb1, 0x1f, // LOCK CMPXCHG [RDI], EBX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 200); // DEST(200) != EAX(100) -> failure branch
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 200, "CMPXCHG failure: DEST unchanged");
    assert!(!zf_set(regs.rflags), "CMPXCHG failure clears ZF");
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        200,
        "Accumulator loaded with DEST on failure"
    );
}

#[test]
fn test_lock_cmpxchg_reg_dest_ud() {
    // LOCK CMPXCHG EBX, EAX  (F0 0F B1 C3: ModRM mod=11 register dest) => #UD
    let code = [
        0xf0, 0x0f, 0xb1, 0xc3, // LOCK CMPXCHG EBX, EAX (register dest)
        0xf4, // HLT (must NOT be reached)
    ];
    assert!(
        !reached_hlt(&code, None),
        "LOCK CMPXCHG reg dest must raise #UD"
    );
}

// ----- LOCK CMPXCHG8B [mem] is legal (group9 /1, memory dest) -----

#[test]
fn test_lock_cmpxchg8b_mem_legal() {
    // LOCK CMPXCHG8B QWORD PTR [RDI]  (F0 0F C7 0F): group9 /1, memory dest -> legal.
    // EDX:EAX == [mem] -> success: [mem] = ECX:EBX, ZF=1.
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000 (pointer)
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678 (low expected)
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0 (high expected)
        0xbb, 0xef, 0xbe, 0xad, 0xde, // MOV EBX, 0xDEADBEEF (low new)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (high new)
        0xf0, 0x0f, 0xc7, 0x0f, // LOCK CMPXCHG8B [RDI]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0000_0000_1234_5678); // matches EDX:EAX -> success
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_u64(&mem),
        0x0000_0000_DEAD_BEEF,
        "CMPXCHG8B success writes ECX:EBX"
    );
    assert!(zf_set(regs.rflags), "CMPXCHG8B success sets ZF");
}

// ----- Plain (non-LOCK) register-destination forms still execute fine -----

#[test]
fn test_no_lock_add_reg_dest_still_works() {
    // ADD EBX, EAX without LOCK must NOT be affected by the enforcement.
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x01, 0xc3, // ADD EBX, EAX  (no LOCK)
        0xf4, // HLT
    ];
    let (mut vcpu, _mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rbx & 0xFFFF_FFFF,
        12,
        "ADD EBX, EAX = 5 + 7 (no LOCK, no #UD)"
    );
}

// ============================================================================
// LOCK-prefixed RMW: exact memory result AND exact flags, computed by hand.
// These complement the result-only LOCK tests above with full flag checks.
// ============================================================================

// LOCK ADD [mem], imm32 producing a signed overflow: 0x7FFFFFFF + 1.
// Result 0x80000000 -> SF=1, OF=1, ZF=0, CF=0.
#[test]
fn test_lock_add_imm_signed_overflow_flags() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x03, 0x01, 0x00, 0x00, 0x00, // LOCK ADD DWORD PTR [RBX], 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x7FFF_FFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x8000_0000, "0x7FFFFFFF + 1");
    assert!(sf_set(regs.rflags), "SF set (bit 31 set)");
    assert!(of_set(regs.rflags), "OF set (signed overflow)");
    assert!(!zf_set(regs.rflags), "ZF clear");
    assert!(!cf_set(regs.rflags), "CF clear (no unsigned carry)");
}

// LOCK SUB [mem], imm32 to exactly zero: 50 - 50.
// Result 0 -> ZF=1, CF=0, SF=0, OF=0, PF=1 (0x00 has even parity).
#[test]
fn test_lock_sub_imm_to_zero_flags() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x2b, 0x32, 0x00, 0x00, 0x00, // LOCK SUB DWORD PTR [RBX], 0x32
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 50);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0, "50 - 50 = 0");
    assert!(zf_set(regs.rflags), "ZF set (result zero)");
    assert!(!cf_set(regs.rflags), "CF clear (no borrow)");
    assert!(!sf_set(regs.rflags), "SF clear");
    assert!(!of_set(regs.rflags), "OF clear");
    assert!(pf_set(regs.rflags), "PF set (0x00 even parity)");
}

// LOCK SUB producing a borrow: 0 - 1 (32-bit) -> 0xFFFFFFFF, CF=1, SF=1.
#[test]
fn test_lock_sub_borrow_flags() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf0, 0x81, 0x2b, 0x01, 0x00, 0x00, 0x00, // LOCK SUB DWORD PTR [RBX], 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0xFFFF_FFFF, "0 - 1 wraps to 0xFFFFFFFF");
    assert!(cf_set(regs.rflags), "CF set (borrow)");
    assert!(sf_set(regs.rflags), "SF set (bit 31 set)");
    assert!(!zf_set(regs.rflags), "ZF clear");
    assert!(!of_set(regs.rflags), "OF clear (no signed overflow)");
}

// LOCK INC/DEC do NOT affect CF (architectural rule). DEC to zero sets ZF but
// leaves a pre-set CF untouched.
#[test]
fn test_lock_dec_to_zero_preserves_cf() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xf9, // STC (set CF)
        0xf0, 0xff, 0x0b, // LOCK DEC DWORD PTR [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 1);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0, "1 - 1 = 0");
    assert!(zf_set(regs.rflags), "ZF set (result zero)");
    assert!(
        cf_set(regs.rflags),
        "CF preserved by DEC (INC/DEC do not touch CF)"
    );
}

// LOCK XADD [mem], reg flags: memory = old + src; reg = old. Flags reflect the
// addition. old=0xFFFFFFFF + EBX=1 -> 0 with CF=1, ZF=1; EBX gets old (0xFFFFFFFF).
#[test]
fn test_lock_xadd_flags_carry_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x20, 0x00, 0x00, // MOV RAX, 0x2000
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf0, 0x0f, 0xc1, 0x18, // LOCK XADD [RAX], EBX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFF_FFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0, "XADD: 0xFFFFFFFF + 1 wraps to 0");
    assert_eq!(
        regs.rbx & 0xFFFF_FFFF,
        0xFFFF_FFFF,
        "XADD: EBX = old memory value"
    );
    assert!(zf_set(regs.rflags), "ZF set (sum zero)");
    assert!(cf_set(regs.rflags), "CF set (carry out)");
}

// XCHG with memory is implicitly atomic (no LOCK needed) and must NOT modify any
// arithmetic flags. Seed CF+ZF and confirm they survive the swap.
#[test]
fn test_xchg_mem_atomic_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0x87, 0x03, // XCHG [RBX], EAX (implicit lock)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x40 | 0x1 | 0x2; // ZF + CF + reserved
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x9ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(read_mem_u32(&mem), 0x12345678, "memory takes EAX");
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0x9ABCDEF0, "EAX takes old memory");
    assert!(zf_set(regs.rflags), "ZF preserved by XCHG");
    assert!(cf_set(regs.rflags), "CF preserved by XCHG");
}

// LOCK CMPXCHG8B failure path: EDX:EAX loaded from memory, ZF cleared.
#[test]
fn test_lock_cmpxchg8b_failure_loads_edx_eax() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000
        0xb8, 0x99, 0x99, 0x99, 0x99, // MOV EAX, 0x99999999 (wrong low)
        0xba, 0x88, 0x88, 0x88, 0x88, // MOV EDX, 0x88888888 (wrong high)
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0xf0, 0x0f, 0xc7, 0x0f, // LOCK CMPXCHG8B [RDI]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x2222_2222_1111_1111); // != EDX:EAX -> failure
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(!zf_set(regs.rflags), "CMPXCHG8B failure clears ZF");
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        0x1111_1111,
        "EAX loaded from memory low"
    );
    assert_eq!(
        regs.rdx & 0xFFFF_FFFF,
        0x2222_2222,
        "EDX loaded from memory high"
    );
    assert_eq!(
        read_mem_u64(&mem),
        0x2222_2222_1111_1111,
        "memory unchanged on failure"
    );
}

// LOCK CMPXCHG16B success path (REX.W + group9 /1): memory = RCX:RBX, ZF set.
#[test]
fn test_lock_cmpxchg16b_success_writes_rcx_rbx() {
    let code = [
        0x48, 0xc7, 0xc7, 0x00, 0x20, 0x00, 0x00, // MOV RDI, 0x2000
        0x48, 0x31, 0xc0, // XOR RAX, RAX (expected low = 0)
        0x48, 0x31, 0xd2, // XOR RDX, RDX (expected high = 0)
        0x48, 0xb9, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00,
        0x00, // MOV RCX, 0xDEADBEEF (new high)
        0x48, 0xbb, 0x0D, 0xF0, 0xFE, 0xCA, 0x00, 0x00, 0x00,
        0x00, // MOV RBX, 0xCAFEF00D (new low)
        0xf0, 0x48, 0x0f, 0xc7, 0x0f, // LOCK CMPXCHG16B [RDI]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // Memory == RDX:RAX (both zero) -> success.
    write_mem_u64(&mem, 0);
    use vm_memory::{Bytes, GuestAddress};
    mem.write_slice(&0u64.to_le_bytes(), GuestAddress(0x2008))
        .unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(zf_set(regs.rflags), "CMPXCHG16B success sets ZF");
    assert_eq!(read_mem_u64(&mem), 0xCAFEF00D, "memory low = RBX");
    let mut hi = [0u8; 8];
    mem.read_slice(&mut hi, GuestAddress(0x2008)).unwrap();
    assert_eq!(u64::from_le_bytes(hi), 0xDEADBEEF, "memory high = RCX");
}
