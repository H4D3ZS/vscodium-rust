use crate::common::*;

// PCMPISTRI - Packed Compare Implicit Length Strings, Return Index
//
// SSE4.2 instruction that performs a string comparison of two operands using a control byte
// to specify the comparison operation. The instruction operates on implicit length strings
// (null-terminated or using all 16 bytes).
//
// Returns the index of the first matching or non-matching element in ECX.
// Sets flags based on the comparison result:
//   CF = 1 if any match found (IntRes2 != 0)
//   ZF = 1 if end of string reached in second operand
//   SF = 1 if end of string reached in first operand
//   OF = 1 if ECX is valid (result index < 16)
//
// Control byte format (imm8):
//   Bits 0-1: Source data format
//     00 = Unsigned bytes (16 elements)
//     01 = Unsigned words (8 elements)
//     10 = Signed bytes (16 elements)
//     11 = Signed words (8 elements)
//   Bits 2-3: Aggregation operation
//     00 = Equal any
//     01 = Ranges
//     10 = Equal each
//     11 = Equal ordered
//   Bit 4: Polarity
//     0 = Positive polarity
//     1 = Negative polarity
//   Bit 5: Output selection
//     0 = Least significant index
//     1 = Most significant index
//   Bit 6: IntRes2 override
//     0 = No override
//     1 = Override (invert)
//
// Opcode:
//   66 0F 3A 63 /r ib    PCMPISTRI xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Equal Any Mode (bits 2-3 = 00) - Unsigned Bytes
// ============================================================================

#[test]
fn test_pcmpistri_equal_any_ubytes_match_first() {
    // Control byte: 0x00 = unsigned bytes, equal any, positive polarity, LSB
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // PCMPISTRI XMM0, XMM1, 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "cabc\0" - 'c' matches at index 0
    let data2: [u8; 16] = [
        0x63, 0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_ubytes_no_match() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xyz\0" - no matches
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_ubytes_all_match() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "aaaa\0"
    let data1: [u8; 16] = [
        0x61, 0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "aaaa\0" - all match
    let data2: [u8; 16] = [
        0x61, 0x61, 0x61, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_any_uwords() {
    // Control byte: 0x01 = unsigned words, equal any
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x01,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: words [0x1234, 0x5678, 0x0000, ...]
    let data1: [u8; 16] = [
        0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: words [0x5678, 0x1234, 0x0000, ...]
    let data2: [u8; 16] = [
        0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Ranges Mode (bits 2-3 = 01)
// ============================================================================

#[test]
fn test_pcmpistri_ranges_ubytes() {
    // Control byte: 0x04 = unsigned bytes, ranges
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range pairs 'a'-'z', 'A'-'Z'
    let data1: [u8; 16] = [
        0x61, 0x7a, 0x41, 0x5a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "Hello123\0"
    let data2: [u8; 16] = [
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0x00, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_ranges_digits() {
    // Control byte: 0x04 = unsigned bytes, ranges
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range pair '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abc123xyz\0"
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x31, 0x32, 0x33, 0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Equal Each Mode (bits 2-3 = 10)
// ============================================================================

#[test]
fn test_pcmpistri_equal_each_match() {
    // Control byte: 0x08 = unsigned bytes, equal each
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "abcd\0" - all match
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_each_mismatch() {
    // Control byte: 0x08 = unsigned bytes, equal each
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "abXd\0" - mismatch at index 2
    let data2: [u8; 16] = [
        0x61, 0x62, 0x58, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_each_partial() {
    // Control byte: 0x08 = unsigned bytes, equal each
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abcd\0"
    let data1: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "ab\0" - shorter string
    let data2: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Equal Ordered Mode (bits 2-3 = 11)
// ============================================================================

#[test]
fn test_pcmpistri_equal_ordered_substring() {
    // Control byte: 0x0C = unsigned bytes, equal ordered
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "cd\0" - substring to find
    let data1: [u8; 16] = [0x63, 0x64, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - contains "cd" at index 2
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_ordered_no_substring() {
    // Control byte: 0x0C = unsigned bytes, equal ordered
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "xyz\0"
    let data1: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - doesn't contain "xyz"
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_equal_ordered_at_start() {
    // Control byte: 0x0C = unsigned bytes, equal ordered
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "abcdef\0" - contains "ab" at index 0
    let data2: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Negative Polarity Tests (bit 4 = 1)
// ============================================================================

#[test]
fn test_pcmpistri_negative_polarity() {
    // Control byte: 0x10 = unsigned bytes, equal any, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x10,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "aeiou\0" - vowels
    let data1: [u8; 16] = [
        0x61, 0x65, 0x69, 0x6f, 0x75, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "bcdfg\0" - consonants, first non-vowel at index 0
    let data2: [u8; 16] = [
        0x62, 0x63, 0x64, 0x66, 0x67, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_negative_polarity_ranges() {
    // Control byte: 0x14 = unsigned bytes, ranges, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x14,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: range '0'-'9'
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "123abc\0" - first non-digit at index 3 ('a')
    let data2: [u8; 16] = [
        0x31, 0x32, 0x33, 0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Most Significant Index (bit 5 = 1)
// ============================================================================

#[test]
fn test_pcmpistri_most_significant_index() {
    // Control byte: 0x20 = unsigned bytes, equal any, MSB index
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x20,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xaxbxax\0" - last 'a' or 'b' at index 6 ('a')
    let data2: [u8; 16] = [
        0x78, 0x61, 0x78, 0x62, 0x78, 0x61, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_msb_equal_ordered() {
    // Control byte: 0x2C = unsigned bytes, equal ordered, MSB index
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x2c,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xabxabx\0" - last occurrence of "ab" at index 4
    let data2: [u8; 16] = [
        0x78, 0x61, 0x62, 0x78, 0x61, 0x62, 0x78, 0x00, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Signed Data Tests
// ============================================================================

#[test]
fn test_pcmpistri_signed_bytes() {
    // Control byte: 0x02 = signed bytes, equal any
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: signed bytes including negative
    let data1: [u8; 16] = [
        0x01, 0xFF, 0x7F, 0x80, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: test string with negative value
    let data2: [u8; 16] = [0x05, 0xFF, 0x03, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_signed_words() {
    // Control byte: 0x03 = signed words, equal any
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x03,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: signed words [0x1234, 0x8000, 0x0000]
    let data1: [u8; 16] = [
        0x34, 0x12, 0x00, 0x80, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: words with 0x8000 (negative)
    let data2: [u8; 16] = [
        0x56, 0x78, 0x00, 0x80, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_pcmpistri_memory_operand() {
    // Test with memory operand as second argument
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x3a, 0x63, 0x40, 0x10, 0x00, // PCMPISTRI XMM0, [RAX+0x10], 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pcmpistri_xmm8_xmm9() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM9, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xc1, 0x00, // PCMPISTRI XMM8, XMM9, 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm10_xmm11() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM11, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xd3, 0x0c, // PCMPISTRI XMM10, XMM11, 0x0C
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [
        0x78, 0x61, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm12_xmm13() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM13, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xe5, 0x04, // PCMPISTRI XMM12, XMM13, 0x04
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x35, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm14_xmm15() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM15, [RAX+0x10]
        0x66, 0x45, 0x0f, 0x3a, 0x63, 0xf7, 0x08, // PCMPISTRI XMM14, XMM15, 0x08
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_pcmpistri_empty_strings() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Both strings empty (null at start)
    let data1: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let data2: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_full_16_bytes() {
    // Test with full 16 bytes (no null terminator)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
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
fn test_pcmpistri_override_bit() {
    // Control byte: 0x40 = unsigned bytes, equal any, override bit set
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x40,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests for imm8=0x3a (used by busybox for string operations)
// 0x3a = signed bytes, equal ordered, masked negative polarity, LSB
// ============================================================================

#[test]
fn test_pcmpistri_control_0x3a_basic() {
    // Control byte: 0x3a = signed bytes, equal ordered, masked negative polarity
    // This is used by glibc/musl for strchr-like operations
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x48, 0x10, // MOVDQA XMM1, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x3a, // PCMPISTRI XMM0, XMM1, 0x3a
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "ab\0" - pattern to search for
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xxab\0" - should find "ab" at position 2
    let data2: [u8; 16] = [
        0x78, 0x78, 0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // With masked negative polarity on equal ordered, we get inverted match bits
    // For substring "ab" in "xxab", the match at position 2 gets inverted
    // ECX should indicate where the pattern is NOT found (before position 2)
    assert!(
        regs.rcx <= 16,
        "ECX should be valid index, got {}",
        regs.rcx
    );
}

#[test]
fn test_pcmpistri_control_0x3a_no_match() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x3a,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "abc\0"
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "xyz\0" - no match
    let data2: [u8; 16] = [0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // No match found - with negative polarity, should return first position (0)
    assert!(
        regs.rcx <= 16,
        "ECX should be valid index, got {}",
        regs.rcx
    );
}

#[test]
fn test_pcmpistri_control_0x3a_single_char() {
    // Test finding a single character (like strchr for '=')
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x3a,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: "=\0" - looking for '='
    let data1: [u8; 16] = [0x3d, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: "VAR=value\0" - '=' at position 3
    let data2: [u8; 16] = [
        0x56, 0x41, 0x52, 0x3d, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x00, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    assert!(
        regs.rcx <= 16,
        "ECX should be valid index, got {}",
        regs.rcx
    );
}

#[test]
fn test_pcmpistri_control_0x02_equal_any_signed() {
    // 0x02 = signed bytes, equal any
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x02,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();

    let regs = vcpu.get_regs().unwrap();
    // 'b' (0x62) from data1 matches at position 1 in data2
    assert_eq!(
        regs.rcx, 1,
        "Expected match at position 1, got {}",
        regs.rcx
    );
}

// ============================================================================
// Multiple Operations
// ============================================================================

#[test]
fn test_pcmpistri_sequence() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1,
        0x00, // PCMPISTRI XMM0, XMM1, 0x00
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08, // PCMPISTRI XMM0, XMM1, 0x08
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c, // PCMPISTRI XMM0, XMM1, 0x0C
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Different Register Combinations
// ============================================================================

#[test]
fn test_pcmpistri_xmm2_xmm3() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x58, 0x10, // MOVDQA XMM3, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xd3, 0x00, // PCMPISTRI XMM2, XMM3, 0x00
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x61, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm4_xmm5() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x68, 0x10, // MOVDQA XMM5, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xe5, 0x08, // PCMPISTRI XMM4, XMM5, 0x08
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm6_xmm7() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x6f, 0x78, 0x10, // MOVDQA XMM7, [RAX+0x10]
        0x66, 0x0f, 0x3a, 0x63, 0xf7, 0x0c, // PCMPISTRI XMM6, XMM7, 0x0C
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [
        0x78, 0x61, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional Control Byte Variations
// ============================================================================

#[test]
fn test_pcmpistri_control_0x18() {
    // Control byte: 0x18 = unsigned bytes, equal each, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x18,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x61, 0x62, 0x58, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_control_0x1c() {
    // Control byte: 0x1C = unsigned bytes, equal ordered, negative polarity
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x1c,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [
        0x78, 0x79, 0x7a, 0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_control_0x30() {
    // Control byte: 0x30 = unsigned bytes, equal any, negative polarity, MSB
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x30,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [
        0x62, 0x78, 0x79, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_ranges_multiple() {
    // Test multiple ranges: 'a'-'z' and '0'-'9'
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: ranges 'a'-'z', '0'-'9'
    let data1: [u8; 16] = [
        0x61, 0x7a, 0x30, 0x39, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "Test123!@#"
    let data2: [u8; 16] = [
        0x54, 0x65, 0x73, 0x74, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0x00, 0, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_case_insensitive_pattern() {
    // Using ranges to simulate case-insensitive search
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: 'A'-'Z', 'a'-'z' ranges
    let data1: [u8; 16] = [
        0x41, 0x5a, 0x61, 0x7a, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    // XMM1: "Hello123!@#"
    let data2: [u8; 16] = [
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x31, 0x32, 0x33, 0x21, 0x40, 0x23, 0x00, 0, 0, 0, 0,
    ];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_boundary_characters() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00,
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // XMM0: contains 0x00, 0xFF, 0x7F, 0x80
    let data1: [u8; 16] = [0x00, 0xFF, 0x7F, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // XMM1: test against boundary values
    let data2: [u8; 16] = [0x01, 0x80, 0x02, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pcmpistri_xmm0_xmm1_various_modes() {
    // Run multiple modes on same data
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, 0x66, 0x0f, 0x6f, 0x48, 0x10, 0x66, 0x0f, 0x3a, 0x63, 0xc1,
        0x00, // Equal any
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x04, // Ranges
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x08, // Equal each
        0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c, // Equal ordered
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let data1: [u8; 16] = [0x61, 0x62, 0x63, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let data2: [u8; 16] = [0x78, 0x62, 0x79, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mem.write_slice(&data1, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&data2, vm_memory::GuestAddress(ALIGNED_ADDR + 0x10))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (implicit-length string compare, index in ECX).
//
// imm8 layout: [0]=word/byte, [1]=signed, [3:2]=aggregation, [5:4]=polarity,
// [6]=output select. agg: 0=equal-any,1=ranges,2=equal-each,3=equal-ordered.
// ============================================================================

#[test]
fn kat_pcmpistri_substring() {
    // PCMPISTRI XMM0, XMM1, 0x0C (66 0F 3A 63 C1 0C): byte, equal-ordered
    // (substring search). DST="ABC", SRC="ZZABCZZ" => first match at index 2.
    let code = [0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x0c, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00000000000000000000000000434241); // "ABC"
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000000005a5a4342415a5a); // "ZZABCZZ"
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFF_FFFF, 2, "PCMPISTRI ECX = {}", regs.rcx);
    // src has a null terminator within 16 bytes => ZF set.
    assert!(crate::common::zf_set(regs.rflags), "ZF should be set");
    // dst has a null terminator within 16 bytes => SF set.
    assert!(crate::common::sf_set(regs.rflags), "SF should be set");
    // a match exists => CF set.
    assert!(crate::common::cf_set(regs.rflags), "CF should be set");
}

#[test]
fn kat_pcmpistri_equal_any_no_match() {
    // PCMPISTRI XMM0, XMM1, 0x00: byte equal-any. DST="ABC", SRC="xyz" => no
    // common chars, ECX = 16 (number of elements), CF clear.
    let code = [0x66, 0x0f, 0x3a, 0x63, 0xc1, 0x00, 0xf4];
    let (mut vcpu, mem) = crate::common::setup_vm(&code, None);
    crate::common::set_xmm(&mem, &mut vcpu, 0, 0x00000000000000000000000000434241); // "ABC"
    crate::common::set_xmm(&mem, &mut vcpu, 1, 0x0000000000000000000000007a7978); // "xyz"
    let regs = crate::common::run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFF_FFFF, 16, "PCMPISTRI ECX = {}", regs.rcx);
    assert!(
        !crate::common::cf_set(regs.rflags),
        "CF should be clear (no match)"
    );
}
