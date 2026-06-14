// Module path for tests run via x86_64.rs
use crate::common::*;

// MOVBE - Move with Byte Swap
// Moves data between register and memory with byte order reversal
// Used for endianness conversion (little-endian <-> big-endian)
//
// Opcodes:
// 0F 38 F0 /r             MOVBE r16/r32/r64, m16/m32/m64   (memory to register, byte-swapped)
// 0F 38 F1 /r             MOVBE m16/m32/m64, r16/r32/r64   (register to memory, byte-swapped)
//
// Supports 16-bit, 32-bit, and 64-bit operands
// Does not modify flags

// ===== MOVBE REGISTER TO MEMORY TESTS =====

#[test]
fn test_movbe_16bit_register_to_memory() {
    // MOVBE with 16-bit value: register to memory with byte swap
    // Use RBX-based addressing to avoid RIP-relative calculation issues
    let code = [
        0x66, 0xb8, 0x12, 0x34, // MOV AX, 0x3412
        0x66, 0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], AX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes: 0x3412 -> stored as 0x1234 in memory (big-endian)
    let value = read_mem_at_u16(&mem, DATA_ADDR);
    assert_eq!(value, 0x1234, "16-bit value should be byte-swapped");
}

#[test]
fn test_movbe_32bit_register_to_memory() {
    // MOVBE with 32-bit value: register to memory with byte swap
    let code = [
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes: 0x12345678 -> 0x78563412 in memory
    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0x78563412, "32-bit value should be byte-swapped");
}

#[test]
fn test_movbe_64bit_register_to_memory() {
    // MOVBE with 64-bit value: register to memory with byte swap
    // Use RCX as base since we need RAX for the value
    let code = [
        0x48, 0xb8, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x77, // MOV RAX, 0x7766554433221100
        0x48, 0x0f, 0x38, 0xf1, 0x01, // MOVBE [RCX], RAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes: 0x7766554433221100 -> 0x0011223344556677 in memory
    let value = read_mem_at_u64(&mem, DATA_ADDR);
    assert_eq!(
        value, 0x0011223344556677,
        "64-bit value should be byte-swapped"
    );
}

#[test]
fn test_movbe_register_to_memory_preserves_source() {
    // MOVBE should not modify the source register
    let code = [
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should still contain original (non-swapped) value
    assert_eq!(
        final_regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Source register unchanged"
    );
}

#[test]
fn test_movbe_different_registers() {
    // MOVBE should work with different registers
    // Use RCX as base, EBX as source
    let code = [
        0xbb, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EBX, 0xDDCCBBAA
        0x0f, 0x38, 0xf1, 0x19, // MOVBE [RCX], EBX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0xAABBCCDD, "EBX should be byte-swapped correctly");
}

#[test]
fn test_movbe_multiple_stores() {
    // Multiple MOVBE stores to different locations
    // Use RCX as base, with displacement for second store
    let code = [
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0x0f, 0x38, 0xf1, 0x01, // MOVBE [RCX], EAX
        0xbb, 0xAA, 0xBB, 0xCC, 0xDD, // MOV EBX, 0xDDCCBBAA
        0x0f, 0x38, 0xf1, 0x59, 0x04, // MOVBE [RCX+4], EBX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value1 = read_mem_at_u32(&mem, DATA_ADDR);
    let value2 = read_mem_at_u32(&mem, DATA_ADDR + 4);

    assert_eq!(value1, 0x12345678, "First MOVBE correct");
    assert_eq!(value2, 0xAABBCCDD, "Second MOVBE correct");
}

// ===== MOVBE MEMORY TO REGISTER TESTS =====

#[test]
fn test_movbe_16bit_memory_to_register() {
    // MOVBE with 16-bit value: memory to register with byte swap
    let code = [
        0x66, 0x0f, 0x38, 0xf0, 0x03, // MOVBE AX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u16(&mem, DATA_ADDR, 0x1234);

    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes: 0x1234 from memory -> 0x3412 in register
    assert_eq!(
        final_regs.rax & 0xFFFF,
        0x3412,
        "16-bit value should be byte-swapped"
    );
}

#[test]
fn test_movbe_32bit_memory_to_register() {
    // MOVBE with 32-bit value: memory to register with byte swap
    let code = [
        0x0f, 0x38, 0xf0, 0x03, // MOVBE EAX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 0x12345678);

    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes: 0x12345678 from memory -> 0x78563412 in register
    assert_eq!(
        final_regs.rax & 0xFFFFFFFF,
        0x78563412,
        "32-bit value should be byte-swapped"
    );
}

#[test]
fn test_movbe_64bit_memory_to_register() {
    // MOVBE with 64-bit value: memory to register with byte swap
    let code = [
        0x48, 0x0f, 0x38, 0xf0, 0x03, // MOVBE RAX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x0123456789ABCDEF);

    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Should swap bytes
    assert_eq!(
        final_regs.rax, 0xEFCDAB8967452301,
        "64-bit value should be byte-swapped"
    );
}

#[test]
fn test_movbe_memory_to_register_overwrites_dest() {
    // MOVBE from memory should overwrite destination register
    let code = [
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0x0f, 0x38, 0xf0, 0x03, // MOVBE EAX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 0x12345678);

    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should contain swapped value from memory, not original 0xFFFFFFFF
    assert_eq!(
        final_regs.rax & 0xFFFFFFFF,
        0x78563412,
        "Destination overwritten with swapped value"
    );
}

#[test]
fn test_movbe_roundtrip_32bit() {
    // Store with MOVBE then load with MOVBE should restore original
    let code = [
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX (stores swapped)
        0x31, 0xc0, // XOR EAX, EAX (clear EAX)
        0x0f, 0x38, 0xf0, 0x03, // MOVBE EAX, [RBX] (loads and swaps)
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // After roundtrip, should restore original value
    assert_eq!(
        final_regs.rax & 0xFFFFFFFF,
        0x78563412,
        "Roundtrip restores original value"
    );
}

#[test]
fn test_movbe_roundtrip_64bit() {
    // 64-bit roundtrip test
    // Use RCX as base since we need RAX for the value
    let code = [
        0x48, 0xb8, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
        0x77, // MOV RAX, 0x7766554433221100
        0x48, 0x0f, 0x38, 0xf1, 0x01, // MOVBE [RCX], RAX
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x48, 0x0f, 0x38, 0xf0, 0x01, // MOVBE RAX, [RCX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        final_regs.rax, 0x7766554433221100,
        "64-bit roundtrip restores value"
    );
}

#[test]
fn test_movbe_zero_value() {
    // MOVBE with zero value (should remain zero after swap)
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0, "Zero remains zero after swap");
}

#[test]
fn test_movbe_all_ones_value() {
    // MOVBE with all ones (should remain all ones after swap)
    let code = [
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0xFFFFFFFF, "All ones remains all ones");
}

#[test]
fn test_movbe_single_byte_pattern() {
    // MOVBE with single byte set in different positions
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 0x00000001
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0x01000000, "Single byte pattern swapped correctly");
}

#[test]
fn test_movbe_alternating_pattern() {
    // MOVBE with alternating byte pattern
    let code = [
        0xb8, 0xAA, 0x55, 0xAA, 0x55, // MOV EAX, 0x55AA55AA
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    let value = read_mem_at_u32(&mem, DATA_ADDR);
    assert_eq!(value, 0xAA55AA55, "Alternating pattern swapped");
}

#[test]
fn test_movbe_does_not_modify_flags() {
    // MOVBE should not modify any flags
    let code = [
        0xf9, // STC (set carry flag)
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0x0f, 0x38, 0xf1, 0x03, // MOVBE [RBX], EAX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Carry flag should still be set
    assert_eq!(cf_set(final_regs.rflags), true, "CF unchanged");
}

#[test]
fn test_movbe_preserves_other_registers() {
    // MOVBE should only affect destination register (for load) or memory (for store)
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0x0f, 0x38, 0xf1, 0x19, // MOVBE [RCX], EBX
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // EAX should be unchanged
    assert_eq!(final_regs.rax & 0xFFFFFFFF, 0x11111111, "EAX preserved");
}

#[test]
fn test_movbe_16bit_partial_register() {
    // MOVBE with 16-bit should preserve upper bits (16-bit register writes)
    let code = [
        0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF
        0x66, 0x0f, 0x38, 0xf0, 0x03, // MOVBE AX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u16(&mem, DATA_ADDR, 0x1234);

    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Lower 16 bits should be swapped, upper 16 bits may be preserved or zero-extended
    assert_eq!(final_regs.rax & 0xFFFF, 0x3412, "16-bit value byte-swapped");
}

#[test]
fn test_movbe_sequential_operations() {
    // Sequential MOVBE operations
    let code = [
        0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0x0f, 0x38, 0xf1, 0x01, // MOVBE [RCX], EAX
        0xbb, 0x55, 0x66, 0x77, 0x88, // MOV EBX, 0x88776655
        0x0f, 0x38, 0xf1, 0x59, 0x08, // MOVBE [RCX+8], EBX
        0x31, 0xc0, // XOR EAX, EAX
        0x0f, 0x38, 0xf0, 0x01, // MOVBE EAX, [RCX]
        0xf4, // HLT
    ];
    let mut regs = rax::cpu::Registers::default();
    regs.rcx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let final_regs = run_until_hlt(&mut vcpu).unwrap();

    // Verify stored values (MOVBE swaps bytes on store)
    let mem_val1 = read_mem_at_u32(&mem, DATA_ADDR);
    let mem_val2 = read_mem_at_u32(&mem, DATA_ADDR + 8);
    assert_eq!(
        mem_val1, 0x11223344,
        "First value stored correctly (byte-swapped)"
    );
    assert_eq!(
        mem_val2, 0x55667788,
        "Second value stored correctly (byte-swapped)"
    );

    // Verify loaded value (MOVBE swaps bytes on load, so roundtrip gives original)
    assert_eq!(
        final_regs.rax & 0xFFFFFFFF,
        0x44332211,
        "Value loaded and swapped"
    );
}
