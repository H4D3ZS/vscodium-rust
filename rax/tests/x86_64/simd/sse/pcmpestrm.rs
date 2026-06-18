use crate::common::*;

// PCMPESTRM - Packed Compare Explicit Length Strings, Return Mask
//
// SSE4.2 instruction similar to PCMPISTRM but uses explicit string lengths
// provided in EAX (for first operand) and EDX (for second operand).
//
// Returns a mask in XMM0 based on the comparison result.
// String lengths are specified in EAX (XMM1/first operand) and EDX (XMM2/second operand).
//
// Sets flags based on the comparison result:
//   CF = 1 if any match found (IntRes2 != 0)
//   ZF = 1 if EDX <= string length (end of second operand reached)
//   SF = 1 if EAX <= string length (end of first operand reached)
//   OF = 1 if IntRes2 is non-zero at bit position 0
//
// Control byte format (imm8):
//   Bits 0-1: Source data format (00=ubytes, 01=uwords, 10=sbytes, 11=swords)
//   Bits 2-3: Aggregation (00=equal any, 01=ranges, 10=equal each, 11=equal ordered)
//   Bit 4: Polarity (0=positive, 1=negative)
//   Bit 5: Output selection (0=LSB, 1=MSB)
//   Bit 6: Mask type (0=bit mask, 1=byte/word mask)
//
// Opcode:
//   66 0F 3A 60 /r ib    PCMPESTRM xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Equal Any Mode - Bit Mask
// ============================================================================

#[test]
fn test_pcmpestrm_equal_any_bitmask() {
    // Control byte: 0x00 = unsigned bytes, equal any, bit mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, // PCMPESTRM XMM0, XMM1, 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abc" (length 3)
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "xbycz" (length 5) - matches 'b' at index 1, 'c' at index 3
    let data2: [u8; 16] = [
        0x78, 0x62, 0x79, 0x63, 0x7a, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_zero_length_first() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x00, 0x00, 0x00,
        0x00, // MOV EAX, 0
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_zero_length_second() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x03, 0x00, 0x00,
        0x00, // MOV EAX, 3
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Byte Mask Output (bit 6 = 1)
// ============================================================================

#[test]
fn test_pcmpestrm_equal_any_bytemask() {
    // Control byte: 0x40 = unsigned bytes, equal any, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x03, 0x00, 0x00,
        0x00, // MOV EAX, 3
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x40, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abc"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xbyczd" - matches produce 0xFF for matched bytes
    let data2: [u8; 16] = [
        0x78, 0x62, 0x79, 0x63, 0x7a, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_uwords_bytemask() {
    // Control byte: 0x41 = unsigned words, equal any, byte mask (word mask really)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2 (2 words)
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3 (3 words)
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x41, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: words [0x1234, 0x5678]
    let data1: [u8; 16] = [
        0x34, 0x12, 0x78, 0x56, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: words [0xABCD, 0x5678, 0x1234]
    let data2: [u8; 16] = [
        0xCD, 0xAB, 0x78, 0x56, 0x34, 0x12, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Ranges Mode
// ============================================================================

#[test]
fn test_pcmpestrm_ranges_bitmask() {
    // Control byte: 0x04 = unsigned bytes, ranges, bit mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2 (1 range pair)
        0xba, 0x08, 0x00, 0x00, 0x00, // MOV EDX, 8
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x04, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range 'a'-'z'
    let data1: [u8; 16] = [0x61, 0x7a, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "Hello123"
    let data2: [u8; 16] = [
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0xFF, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_ranges_bytemask() {
    // Control byte: 0x44 = unsigned bytes, ranges, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x09, 0x00, 0x00, 0x00, // MOV EDX, 9
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x44, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abc123xyz"
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x31, 0x32, 0x33, 0x78, 0x79, 0x7a, 0xFF, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_ranges_multiple() {
    // Control byte: 0x04 = unsigned bytes, ranges
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x04, 0x00, 0x00,
        0x00, // MOV EAX, 4 (2 range pairs)
        0xba, 0x0a, 0x00, 0x00, 0x00, // MOV EDX, 10
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x04, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: ranges 'a'-'z', '0'-'9'
    let data1: [u8; 16] = [
        0x61, 0x7a, 0x30, 0x39, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "Test123!@#"
    let data2: [u8; 16] = [
        0x54, 0x65, 0x73, 0x74, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0xFF, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Equal Each Mode
// ============================================================================

#[test]
fn test_pcmpestrm_equal_each_match() {
    // Control byte: 0x08 = unsigned bytes, equal each
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x04, 0x00, 0x00,
        0x00, // MOV EAX, 4
        0xba, 0x04, 0x00, 0x00, 0x00, // MOV EDX, 4
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x08, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "abcd"
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_equal_each_mismatch() {
    // Control byte: 0x48 = unsigned bytes, equal each, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x04, 0x00, 0x00,
        0x00, // MOV EAX, 4
        0xba, 0x04, 0x00, 0x00, 0x00, // MOV EDX, 4
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x48, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "abXd" - mismatch at index 2
    let data2: [u8; 16] = [
        0x61, 0x62, 0x58, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_equal_each_different_lengths() {
    // Control byte: 0x08 = unsigned bytes, equal each
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x05, 0x00, 0x00,
        0x00, // MOV EAX, 5
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3 (shorter)
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x08, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Equal Ordered Mode
// ============================================================================

#[test]
fn test_pcmpestrm_equal_ordered_substring() {
    // Control byte: 0x0C = unsigned bytes, equal ordered
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x0c, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "cd" (substring to find)
    let data1: [u8; 16] = [0x63, 0x64, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef" (contains "cd" at index 2-3)
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_equal_ordered_bytemask() {
    // Control byte: 0x4C = unsigned bytes, equal ordered, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x07, 0x00, 0x00, 0x00, // MOV EDX, 7
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x4c, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab"
    let data1: [u8; 16] = [0x61, 0x62, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xabxabx" - "ab" found at positions 1 and 4
    let data2: [u8; 16] = [
        0x78, 0x61, 0x62, 0x78, 0x61, 0x62, 0x78, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Negative Polarity
// ============================================================================

#[test]
fn test_pcmpestrm_negative_polarity() {
    // Control byte: 0x10 = unsigned bytes, equal any, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x05, 0x00, 0x00,
        0x00, // MOV EAX, 5
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x10, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "aeiou" (vowels)
    let data1: [u8; 16] = [
        0x61, 0x65, 0x69, 0x6f, 0x75, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "bcdfg" (consonants, all non-vowels)
    let data2: [u8; 16] = [
        0x62, 0x63, 0x64, 0x66, 0x67, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_negative_bytemask() {
    // Control byte: 0x50 = unsigned bytes, equal any, negative polarity, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x03, 0x00, 0x00,
        0x00, // MOV EAX, 3
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x50, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abc"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xayczd" - non-matching chars get 0xFF
    let data2: [u8; 16] = [
        0x78, 0x61, 0x79, 0x63, 0x7a, 0x64, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_negative_ranges() {
    // Control byte: 0x14 = unsigned bytes, ranges, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x06, 0x00, 0x00, 0x00, // MOV EDX, 6
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x14, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "123abc" - non-digits match
    let data2: [u8; 16] = [
        0x31, 0x32, 0x33, 0x61, 0x62, 0x63, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Most Significant Index
// ============================================================================

#[test]
fn test_pcmpestrm_msb() {
    // Control byte: 0x20 = unsigned bytes, equal any, MSB
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x07, 0x00, 0x00, 0x00, // MOV EDX, 7
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x20, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab"
    let data1: [u8; 16] = [0x61, 0x62, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax" - multiple matches
    let data2: [u8; 16] = [
        0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_msb_bytemask() {
    // Control byte: 0x60 = unsigned bytes, equal any, MSB, byte mask
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x02, 0x00, 0x00,
        0x00, // MOV EAX, 2
        0xba, 0x07, 0x00, 0x00, 0x00, // MOV EDX, 7
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x60, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab"
    let data1: [u8; 16] = [0x61, 0x62, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax"
    let data2: [u8; 16] = [
        0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Extended Register Tests
// ============================================================================

#[test]
fn test_pcmpestrm_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM9, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x66, 0x45, 0x0f, 0x3a, 0x60, 0xc1, 0x00, // PCMPESTRM XMM8, XMM9, 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM11, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, 0xba, 0x04, 0x00, 0x00, 0x00, 0x66, 0x45, 0x0f, 0x3a, 0x60,
        0xd3, 0x0c, // PCMPESTRM XMM10, XMM11, 0x0C
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm12_xmm13() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM13, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, 0xba, 0x03, 0x00, 0x00, 0x00, 0x66, 0x45, 0x0f, 0x3a, 0x60,
        0xe5, 0x44, // PCMPESTRM XMM12, XMM13, 0x44
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x30, 0x39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x35, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm14_xmm15() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM15, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, 0xba, 0x03, 0x00, 0x00, 0x00, 0x66, 0x45, 0x0f, 0x3a, 0x60,
        0xf7, 0x48, // PCMPESTRM XMM14, XMM15, 0x48
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory Operand and Additional Register Tests
// ============================================================================

#[test]
fn test_pcmpestrm_memory_operand() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xb8, 0x03, 0x00, 0x00, 0x00, 0xba, 0x03, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0x40,
        0x10, 0x00, // PCMPESTRM XMM0, [RAX+0x10], 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm2_xmm3() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM3, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, 0xba, 0x02, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xd3,
        0x00, // PCMPESTRM XMM2, XMM3, 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm4_xmm5() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM5, [RAX+0x10]
        0xb8, 0x03, 0x00, 0x00, 0x00, 0xba, 0x03, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xe5,
        0x08, // PCMPESTRM XMM4, XMM5, 0x08
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_xmm6_xmm7() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM7, [RAX+0x10]
        0xb8, 0x02, 0x00, 0x00, 0x00, 0xba, 0x04, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xf7,
        0x0c, // PCMPESTRM XMM6, XMM7, 0x0C
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Full Length and Edge Cases
// ============================================================================

#[test]
fn test_pcmpestrm_full_16_bytes() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x10, 0x00, 0x00,
        0x00, // MOV EAX, 16
        0xba, 0x10, 0x00, 0x00, 0x00, // MOV EDX, 16
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x40, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [
        0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61,
        0x61,
    ]; // All 'a'
    let data2: [u8; 16] = [
        0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62, 0x61, 0x62,
        0x61,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x03, 0x00, 0x00, 0x00, 0xba,
        0x03, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xc1,
        0x00, // PCMPESTRM XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x40, // PCMPESTRM XMM0, XMM1, 0x40
        0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x08, // PCMPESTRM XMM0, XMM1, 0x08
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpestrm_varying_lengths() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0xb8, 0x01, 0x00, 0x00,
        0x00, // Length 1
        0xba, 0x01, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, 0xb8, 0x05, 0x00, 0x00,
        0x00, // Length 5
        0xba, 0x05, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, 0xb8, 0x10, 0x00, 0x00,
        0x00, // Length 16
        0xba, 0x10, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x3a, 0x60, 0xc1, 0x00, 0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
        0x70,
    ];
    let data2: [u8; 16] = [
        0x71, 0x62, 0x72, 0x63, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x61, 0x62, 0x63,
        0x64,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (explicit-length string compare, mask in XMM0).
// ============================================================================

#[test]
fn kat_pcmpestrm_equal_each_bitmask() {
    // EAX=5, EDX=5. PCMPESTRM XMM1, XMM2, 0x08 (66 0F 3A 60 CA 08): byte,
    // equal-each, bit-mask. "HELLO" vs "HEXLO" => mask 0x1B in XMM0 low.
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0x66, 0x0f, 0x3a, 0x60, 0xca, 0x08, // PCMPESTRM XMM1, XMM2, 8
        0xf4,
    ];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00000000000000000000004f4c4c4548); // "HELLO"
    crate::common::set_xmm(&mem, &mut vcpu, 2, 0x00000000000000000000004f4c584548); // "HEXLO"
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x1B,
        "PCMPESTRM mask got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}

// NOTE: The expanded byte-mask form (imm8[6]=1) is intentionally exercised by a
// separate #[ignore]d test that documents an emulator truncation bug; see
// kat_pcmpestrm_expanded_mask_bug below.
#[test]
fn kat_pcmpestrm_expanded_mask_bug() {
    // EAX=5, EDX=5. PCMPESTRM XMM1, XMM2, 0x48 (66 0F 3A 60 CA 48): byte,
    // equal-each, EXPANDED byte mask (imm8[6]=1). "HELLO" vs "HEXLO".
    // Matches at byte positions 0,1,3,4 => those bytes 0xFF, others 0x00.
    let code = [
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0x66, 0x0f, 0x3a, 0x60, 0xca, 0x48, // PCMPESTRM XMM1, XMM2, 0x48
        0xf4,
    ];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x00000000000000000000004f4c4c4548); // "HELLO"
    crate::common::set_xmm(&mem, &mut vcpu, 2, 0x00000000000000000000004f4c584548); // "HEXLO"
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        crate::common::get_xmm(&regs, 0),
        0x0000000000000000000000ffff00ffff,
        "PCMPESTRM expanded mask got {:032x}",
        crate::common::get_xmm(&regs, 0)
    );
}
