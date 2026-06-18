use crate::common::*;

// PSHUFHW - Shuffle Packed High Words
//
// Copies words from the high quadword of the source operand and inserts them
// in the high quadword of the destination operand at word locations selected
// with the immediate operand. The low quadword is copied unchanged.
//
// Each 2-bit field in the immediate operand selects the contents of one word:
// - Bits [1:0] select source word for DEST[79:64] (from high quadword)
// - Bits [3:2] select source word for DEST[95:80] (from high quadword)
// - Bits [5:4] select source word for DEST[111:96] (from high quadword)
// - Bits [7:6] select source word for DEST[127:112] (from high quadword)
// - DEST[63:0] = SRC[63:0] (low quadword unchanged)
//
// Opcode: F3 0F 70 /r ib    PSHUFHW xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with immediate value 0x00 (broadcast word 4)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x00() {
    // PSHUFHW XMM0, XMM1, 0x00
    // High: [4, 4, 4, 4], Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x00, // PSHUFHW XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm2_xmm3_imm_0x00() {
    // PSHUFHW XMM2, XMM3, 0x00
    let code = [
        0xf3, 0x0f, 0x70, 0xd3, 0x00, // PSHUFHW XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x55 (broadcast word 5)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x55() {
    // PSHUFHW XMM0, XMM1, 0x55
    // High: [5, 5, 5, 5], Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x55, // PSHUFHW XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm4_xmm5_imm_0x55() {
    // PSHUFHW XMM4, XMM5, 0x55
    let code = [
        0xf3, 0x0f, 0x70, 0xe5, 0x55, // PSHUFHW XMM4, XMM5, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xAA (broadcast word 6)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xaa() {
    // PSHUFHW XMM0, XMM1, 0xAA
    // High: [6, 6, 6, 6], Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xaa, // PSHUFHW XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm6_xmm7_imm_0xaa() {
    // PSHUFHW XMM6, XMM7, 0xAA
    let code = [
        0xf3, 0x0f, 0x70, 0xf7, 0xaa, // PSHUFHW XMM6, XMM7, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xFF (broadcast word 7)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xff() {
    // PSHUFHW XMM0, XMM1, 0xFF
    // High: [7, 7, 7, 7], Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xff, // PSHUFHW XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm1_xmm2_imm_0xff() {
    // PSHUFHW XMM1, XMM2, 0xFF
    let code = [
        0xf3, 0x0f, 0x70, 0xca, 0xff, // PSHUFHW XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xE4 (identity for high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xe4() {
    // PSHUFHW XMM0, XMM1, 0xE4
    // 0xE4 = 11 10 01 00 (binary)
    // High: [4, 5, 6, 7] - identity (no change), Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM0, XMM1, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x1B (reverse high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x1b() {
    // PSHUFHW XMM0, XMM1, 0x1B
    // 0x1B = 00 01 10 11 (binary)
    // High: [7, 6, 5, 4] - reverse, Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x1b, // PSHUFHW XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm5_xmm6_imm_0x1b() {
    // PSHUFHW XMM5, XMM6, 0x1B
    let code = [
        0xf3, 0x0f, 0x70, 0xee, 0x1b, // PSHUFHW XMM5, XMM6, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xB1 (swap pairs in high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xb1() {
    // PSHUFHW XMM0, XMM1, 0xB1
    // 0xB1 = 10 11 00 01 (binary)
    // High: [5, 4, 7, 6] - swap pairs, Low: unchanged
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xb1, // PSHUFHW XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm2_xmm3_imm_0xb1() {
    // PSHUFHW XMM2, XMM3, 0xB1
    let code = [
        0xf3, 0x0f, 0x70, 0xd3, 0xb1, // PSHUFHW XMM2, XMM3, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x4E (swap word pairs in high quadword)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x4e() {
    // PSHUFHW XMM0, XMM1, 0x4E
    // 0x4E = 01 00 11 10 (binary)
    // High: [6, 7, 4, 5] - swap low/high word pairs
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x4e, // PSHUFHW XMM0, XMM1, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm7_xmm0_imm_0x4e() {
    // PSHUFHW XMM7, XMM0, 0x4E
    let code = [
        0xf3, 0x0f, 0x70, 0xf8, 0x4e, // PSHUFHW XMM7, XMM0, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with various immediate values
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x27() {
    // PSHUFHW XMM0, XMM1, 0x27
    // 0x27 = 00 10 01 11 (binary)
    // High: [7, 5, 6, 4]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x27, // PSHUFHW XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x39() {
    // PSHUFHW XMM0, XMM1, 0x39
    // 0x39 = 00 11 10 01 (binary)
    // High: [5, 6, 7, 4]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x39, // PSHUFHW XMM0, XMM1, 0x39
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x72() {
    // PSHUFHW XMM0, XMM1, 0x72
    // 0x72 = 01 11 00 10 (binary)
    // High: [6, 4, 7, 5]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x72, // PSHUFHW XMM0, XMM1, 0x72
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x93() {
    // PSHUFHW XMM0, XMM1, 0x93
    // 0x93 = 10 01 00 11 (binary)
    // High: [7, 4, 5, 6]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x93, // PSHUFHW XMM0, XMM1, 0x93
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xc6() {
    // PSHUFHW XMM0, XMM1, 0xC6
    // 0xC6 = 11 00 01 10 (binary)
    // High: [6, 5, 4, 7]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xc6, // PSHUFHW XMM0, XMM1, 0xC6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xd8() {
    // PSHUFHW XMM0, XMM1, 0xD8
    // 0xD8 = 11 01 10 00 (binary)
    // High: [4, 6, 5, 7]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xd8, // PSHUFHW XMM0, XMM1, 0xD8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x44() {
    // PSHUFHW XMM0, XMM1, 0x44
    // 0x44 = 01 00 01 00 (binary)
    // High: [4, 5, 4, 5] - duplicate low pair
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x44, // PSHUFHW XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xee() {
    // PSHUFHW XMM0, XMM1, 0xEE
    // 0xEE = 11 10 11 10 (binary)
    // High: [6, 7, 6, 7] - duplicate high pair
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xee, // PSHUFHW XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x50() {
    // PSHUFHW XMM0, XMM1, 0x50
    // 0x50 = 01 01 00 00 (binary)
    // High: [4, 4, 5, 5]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x50, // PSHUFHW XMM0, XMM1, 0x50
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xfa() {
    // PSHUFHW XMM0, XMM1, 0xFA
    // 0xFA = 11 11 10 10 (binary)
    // High: [6, 6, 7, 7]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xfa, // PSHUFHW XMM0, XMM1, 0xFA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_pshufhw_xmm1_xmm2_imm_0xe4() {
    // PSHUFHW XMM1, XMM2, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xca, 0xe4, // PSHUFHW XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm5_xmm6_imm_0xe4() {
    // PSHUFHW XMM5, XMM6, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xee, 0xe4, // PSHUFHW XMM5, XMM6, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm7_xmm0_imm_0xe4() {
    // PSHUFHW XMM7, XMM0, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFHW XMM7, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pshufhw_xmm8_xmm9_imm_0xe4() {
    // PSHUFHW XMM8, XMM9, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM8, XMM9, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm10_xmm11_imm_0xe4() {
    // PSHUFHW XMM10, XMM11, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFHW XMM10, XMM11, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm12_xmm13_imm_0xe4() {
    // PSHUFHW XMM12, XMM13, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFHW XMM12, XMM13, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm14_xmm15_imm_0xe4() {
    // PSHUFHW XMM14, XMM15, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xf7, 0xe4, // PSHUFHW XMM14, XMM15, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm8_imm_0xe4() {
    // PSHUFHW XMM0, XMM8, 0xE4
    let code = [
        0xf3, 0x44, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFHW XMM0, XMM8, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm15_xmm0_imm_0xe4() {
    // PSHUFHW XMM15, XMM0, 0xE4
    let code = [
        0xf3, 0x44, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFHW XMM15, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_pshufhw_xmm0_mem_imm_0xe4() {
    // PSHUFHW XMM0, [mem], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x00, 0xe4, // PSHUFHW XMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (8 words)
    let word_data: [u16; 8] = [
        0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777, 0x8888,
    ];
    let mut bytes = Vec::new();
    for w in &word_data {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm1_mem_imm_0x1b() {
    // PSHUFHW XMM1, [mem], 0x1B
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x08, 0x1b, // PSHUFHW XMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let word_data: [u16; 8] = [
        0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD, 0xEEEE, 0xFFFF, 0x0000, 0x1111,
    ];
    let mut bytes = Vec::new();
    for w in &word_data {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm2_mem_imm_0x00() {
    // PSHUFHW XMM2, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x10, 0x00, // PSHUFHW XMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm3_mem_imm_0xff() {
    // PSHUFHW XMM3, [mem], 0xFF
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x18, 0xff, // PSHUFHW XMM3, [RAX], 0xFF
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm7_mem_imm_0x4e() {
    // PSHUFHW XMM7, [mem], 0x4E
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x38, 0x4e, // PSHUFHW XMM7, [RAX], 0x4E
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
            0x55, 0x55,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Addressing mode tests
// ============================================================================

#[test]
fn test_pshufhw_xmm0_mem_displacement_imm_0xe4() {
    // PSHUFHW XMM0, [RAX + disp], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x40, 0x10, 0xe4, // PSHUFHW XMM0, [RAX+0x10], 0xE4
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm1_mem_rbx_imm_0xe4() {
    // PSHUFHW XMM1, [RBX], 0xE4
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x0b, 0xe4, // PSHUFHW XMM1, [RBX], 0xE4
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88,
            0x88, 0x88,
        ],
        vm_memory::GuestAddress(ALIGNED_ADDR),
    )
    .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Sequential shuffle tests
// ============================================================================

#[test]
fn test_pshufhw_sequential_operations() {
    // Multiple PSHUFHW operations in sequence
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM0, XMM1, 0xE4
        0xf3, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFHW XMM2, XMM3, 0xE4
        0xf3, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFHW XMM4, XMM5, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_same_register_imm_0xe4() {
    // PSHUFHW XMM0, XMM0, 0xE4 (shuffle with itself)
    let code = [
        0xf3, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFHW XMM0, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_same_register_imm_0x1b() {
    // PSHUFHW XMM1, XMM1, 0x1B (shuffle with itself, reverse)
    let code = [
        0xf3, 0x0f, 0x70, 0xc9, 0x1b, // PSHUFHW XMM1, XMM1, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional permutation tests
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x0f() {
    // PSHUFHW XMM0, XMM1, 0x0F
    // 0x0F = 00 00 11 11 (binary)
    // High: [7, 7, 4, 4]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x0f, // PSHUFHW XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xf0() {
    // PSHUFHW XMM0, XMM1, 0xF0
    // 0xF0 = 11 11 00 00 (binary)
    // High: [4, 4, 7, 7]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xf0, // PSHUFHW XMM0, XMM1, 0xF0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xa5() {
    // PSHUFHW XMM0, XMM1, 0xA5
    // 0xA5 = 10 10 01 01 (binary)
    // High: [5, 5, 6, 6]
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xa5, // PSHUFHW XMM0, XMM1, 0xA5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x2d() {
    // PSHUFHW XMM0, XMM1, 0x2D
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x2d, // PSHUFHW XMM0, XMM1, 0x2D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register + imm8 via set_xmm/get_xmm)
//
// PSHUFHW shuffles the HIGH four 16-bit words by imm8 (2 bits per result word,
// relative to the high quadword); the low qword is copied unchanged.
//   SRC = XMM1 = 0x01020304050607081122334455667788
//     high words: word4=0x0708, word5=0x0506, word6=0x0304, word7=0x0102
//     low qword (unchanged) = 0x1122334455667788
// Computed by hand.
// ============================================================================

const KAT_PSHUFHW_SRC: u128 = 0x01020304050607081122334455667788;

#[test]
fn kat_pshufhw_reverse_high() {
    // PSHUFHW XMM0, XMM1, 0x1B (F3 0F 70 C1 1B) reverses the high 4 words.
    let code = [0xf3, 0x0f, 0x70, 0xc1, 0x1b, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFHW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x07080506030401021122334455667788,
        "PSHUFHW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pshufhw_identity_keeps_low() {
    // imm 0xE4 is identity for the high words; low qword always copied verbatim.
    let code = [0xf3, 0x0f, 0x70, 0xc1, 0xe4, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFHW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), KAT_PSHUFHW_SRC);
}

#[test]
fn kat_pshufhw_broadcast_word4() {
    // imm 0x00 broadcasts high word4 into all four high words; low qword intact.
    let code = [0xf3, 0x0f, 0x70, 0xc1, 0x00, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFHW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x07080708070807081122334455667788);
}
