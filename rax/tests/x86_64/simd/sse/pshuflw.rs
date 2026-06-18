use crate::common::*;

// PSHUFLW - Shuffle Packed Low Words
//
// Copies words from the low quadword of the source operand and inserts them
// in the low quadword of the destination operand at word locations selected
// with the immediate operand. The high quadword is copied unchanged.
//
// Each 2-bit field in the immediate operand selects the contents of one word:
// - Bits [1:0] select source word for DEST[15:0] (from low quadword)
// - Bits [3:2] select source word for DEST[31:16] (from low quadword)
// - Bits [5:4] select source word for DEST[47:32] (from low quadword)
// - Bits [7:6] select source word for DEST[63:48] (from low quadword)
// - DEST[127:64] = SRC[127:64] (high quadword unchanged)
//
// Opcode: F2 0F 70 /r ib    PSHUFLW xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with immediate value 0x00 (broadcast word 0)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x00() {
    // PSHUFLW XMM0, XMM1, 0x00
    // Low: [0, 0, 0, 0], High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x00, // PSHUFLW XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm2_xmm3_imm_0x00() {
    // PSHUFLW XMM2, XMM3, 0x00
    let code = [
        0xf2, 0x0f, 0x70, 0xd3, 0x00, // PSHUFLW XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x55 (broadcast word 1)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x55() {
    // PSHUFLW XMM0, XMM1, 0x55
    // Low: [1, 1, 1, 1], High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x55, // PSHUFLW XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm4_xmm5_imm_0x55() {
    // PSHUFLW XMM4, XMM5, 0x55
    let code = [
        0xf2, 0x0f, 0x70, 0xe5, 0x55, // PSHUFLW XMM4, XMM5, 0x55
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xAA (broadcast word 2)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xaa() {
    // PSHUFLW XMM0, XMM1, 0xAA
    // Low: [2, 2, 2, 2], High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xaa, // PSHUFLW XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm6_xmm7_imm_0xaa() {
    // PSHUFLW XMM6, XMM7, 0xAA
    let code = [
        0xf2, 0x0f, 0x70, 0xf7, 0xaa, // PSHUFLW XMM6, XMM7, 0xAA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xFF (broadcast word 3)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xff() {
    // PSHUFLW XMM0, XMM1, 0xFF
    // Low: [3, 3, 3, 3], High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xff, // PSHUFLW XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm1_xmm2_imm_0xff() {
    // PSHUFLW XMM1, XMM2, 0xFF
    let code = [
        0xf2, 0x0f, 0x70, 0xca, 0xff, // PSHUFLW XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xE4 (identity for low words)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xe4() {
    // PSHUFLW XMM0, XMM1, 0xE4
    // 0xE4 = 11 10 01 00 (binary)
    // Low: [0, 1, 2, 3] - identity (no change), High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFLW XMM0, XMM1, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x1B (reverse low words)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x1b() {
    // PSHUFLW XMM0, XMM1, 0x1B
    // 0x1B = 00 01 10 11 (binary)
    // Low: [3, 2, 1, 0] - reverse, High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x1b, // PSHUFLW XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm5_xmm6_imm_0x1b() {
    // PSHUFLW XMM5, XMM6, 0x1B
    let code = [
        0xf2, 0x0f, 0x70, 0xee, 0x1b, // PSHUFLW XMM5, XMM6, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0xB1 (swap pairs in low words)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xb1() {
    // PSHUFLW XMM0, XMM1, 0xB1
    // 0xB1 = 10 11 00 01 (binary)
    // Low: [1, 0, 3, 2] - swap pairs, High: unchanged
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xb1, // PSHUFLW XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm2_xmm3_imm_0xb1() {
    // PSHUFLW XMM2, XMM3, 0xB1
    let code = [
        0xf2, 0x0f, 0x70, 0xd3, 0xb1, // PSHUFLW XMM2, XMM3, 0xB1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with immediate value 0x4E (swap word pairs in low quadword)
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x4e() {
    // PSHUFLW XMM0, XMM1, 0x4E
    // 0x4E = 01 00 11 10 (binary)
    // Low: [2, 3, 0, 1] - swap low/high word pairs
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x4e, // PSHUFLW XMM0, XMM1, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm7_xmm0_imm_0x4e() {
    // PSHUFLW XMM7, XMM0, 0x4E
    let code = [
        0xf2, 0x0f, 0x70, 0xf8, 0x4e, // PSHUFLW XMM7, XMM0, 0x4E
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with various immediate values
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x27() {
    // PSHUFLW XMM0, XMM1, 0x27
    // 0x27 = 00 10 01 11 (binary)
    // Low: [3, 1, 2, 0]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x27, // PSHUFLW XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x39() {
    // PSHUFLW XMM0, XMM1, 0x39
    // 0x39 = 00 11 10 01 (binary)
    // Low: [1, 2, 3, 0]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x39, // PSHUFLW XMM0, XMM1, 0x39
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x72() {
    // PSHUFLW XMM0, XMM1, 0x72
    // 0x72 = 01 11 00 10 (binary)
    // Low: [2, 0, 3, 1]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x72, // PSHUFLW XMM0, XMM1, 0x72
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x93() {
    // PSHUFLW XMM0, XMM1, 0x93
    // 0x93 = 10 01 00 11 (binary)
    // Low: [3, 0, 1, 2]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x93, // PSHUFLW XMM0, XMM1, 0x93
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xc6() {
    // PSHUFLW XMM0, XMM1, 0xC6
    // 0xC6 = 11 00 01 10 (binary)
    // Low: [2, 1, 0, 3]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xc6, // PSHUFLW XMM0, XMM1, 0xC6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xd8() {
    // PSHUFLW XMM0, XMM1, 0xD8
    // 0xD8 = 11 01 10 00 (binary)
    // Low: [0, 2, 1, 3]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xd8, // PSHUFLW XMM0, XMM1, 0xD8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x44() {
    // PSHUFLW XMM0, XMM1, 0x44
    // 0x44 = 01 00 01 00 (binary)
    // Low: [0, 1, 0, 1] - duplicate low pair
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x44, // PSHUFLW XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xee() {
    // PSHUFLW XMM0, XMM1, 0xEE
    // 0xEE = 11 10 11 10 (binary)
    // Low: [2, 3, 2, 3] - duplicate high pair
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xee, // PSHUFLW XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x50() {
    // PSHUFLW XMM0, XMM1, 0x50
    // 0x50 = 01 01 00 00 (binary)
    // Low: [0, 0, 1, 1]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x50, // PSHUFLW XMM0, XMM1, 0x50
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xfa() {
    // PSHUFLW XMM0, XMM1, 0xFA
    // 0xFA = 11 11 10 10 (binary)
    // Low: [2, 2, 3, 3]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xfa, // PSHUFLW XMM0, XMM1, 0xFA
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_pshuflw_xmm1_xmm2_imm_0xe4() {
    // PSHUFLW XMM1, XMM2, 0xE4
    let code = [
        0xf2, 0x0f, 0x70, 0xca, 0xe4, // PSHUFLW XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm5_xmm6_imm_0xe4() {
    // PSHUFLW XMM5, XMM6, 0xE4
    let code = [
        0xf2, 0x0f, 0x70, 0xee, 0xe4, // PSHUFLW XMM5, XMM6, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm7_xmm0_imm_0xe4() {
    // PSHUFLW XMM7, XMM0, 0xE4
    let code = [
        0xf2, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFLW XMM7, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pshuflw_xmm8_xmm9_imm_0xe4() {
    // PSHUFLW XMM8, XMM9, 0xE4
    let code = [
        0xf2, 0x45, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFLW XMM8, XMM9, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm10_xmm11_imm_0xe4() {
    // PSHUFLW XMM10, XMM11, 0xE4
    let code = [
        0xf2, 0x45, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFLW XMM10, XMM11, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm12_xmm13_imm_0xe4() {
    // PSHUFLW XMM12, XMM13, 0xE4
    let code = [
        0xf2, 0x45, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFLW XMM12, XMM13, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm14_xmm15_imm_0xe4() {
    // PSHUFLW XMM14, XMM15, 0xE4
    let code = [
        0xf2, 0x45, 0x0f, 0x70, 0xf7, 0xe4, // PSHUFLW XMM14, XMM15, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm8_imm_0xe4() {
    // PSHUFLW XMM0, XMM8, 0xE4
    let code = [
        0xf2, 0x44, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFLW XMM0, XMM8, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm15_xmm0_imm_0xe4() {
    // PSHUFLW XMM15, XMM0, 0xE4
    let code = [
        0xf2, 0x44, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFLW XMM15, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_pshuflw_xmm0_mem_imm_0xe4() {
    // PSHUFLW XMM0, [mem], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x00, 0xe4, // PSHUFLW XMM0, [RAX], 0xE4
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
fn test_pshuflw_xmm1_mem_imm_0x1b() {
    // PSHUFLW XMM1, [mem], 0x1B
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x08, 0x1b, // PSHUFLW XMM1, [RAX], 0x1B
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
fn test_pshuflw_xmm2_mem_imm_0x00() {
    // PSHUFLW XMM2, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x10, 0x00, // PSHUFLW XMM2, [RAX], 0x00
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
fn test_pshuflw_xmm3_mem_imm_0xff() {
    // PSHUFLW XMM3, [mem], 0xFF
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x18, 0xff, // PSHUFLW XMM3, [RAX], 0xFF
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
fn test_pshuflw_xmm7_mem_imm_0x4e() {
    // PSHUFLW XMM7, [mem], 0x4E
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x38, 0x4e, // PSHUFLW XMM7, [RAX], 0x4E
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
fn test_pshuflw_xmm0_mem_displacement_imm_0xe4() {
    // PSHUFLW XMM0, [RAX + disp], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x40, 0x10, 0xe4, // PSHUFLW XMM0, [RAX+0x10], 0xE4
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
fn test_pshuflw_xmm1_mem_rbx_imm_0xe4() {
    // PSHUFLW XMM1, [RBX], 0xE4
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x70, 0x0b, 0xe4, // PSHUFLW XMM1, [RBX], 0xE4
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
fn test_pshuflw_sequential_operations() {
    // Multiple PSHUFLW operations in sequence
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFLW XMM0, XMM1, 0xE4
        0xf2, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFLW XMM2, XMM3, 0xE4
        0xf2, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFLW XMM4, XMM5, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_same_register_imm_0xe4() {
    // PSHUFLW XMM0, XMM0, 0xE4 (shuffle with itself)
    let code = [
        0xf2, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFLW XMM0, XMM0, 0xE4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_same_register_imm_0x1b() {
    // PSHUFLW XMM1, XMM1, 0x1B (shuffle with itself, reverse)
    let code = [
        0xf2, 0x0f, 0x70, 0xc9, 0x1b, // PSHUFLW XMM1, XMM1, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Additional permutation tests
// ============================================================================

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x0f() {
    // PSHUFLW XMM0, XMM1, 0x0F
    // 0x0F = 00 00 11 11 (binary)
    // Low: [3, 3, 0, 0]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x0f, // PSHUFLW XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xf0() {
    // PSHUFLW XMM0, XMM1, 0xF0
    // 0xF0 = 11 11 00 00 (binary)
    // Low: [0, 0, 3, 3]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xf0, // PSHUFLW XMM0, XMM1, 0xF0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0xa5() {
    // PSHUFLW XMM0, XMM1, 0xA5
    // 0xA5 = 10 10 01 01 (binary)
    // Low: [1, 1, 2, 2]
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0xa5, // PSHUFLW XMM0, XMM1, 0xA5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_pshuflw_xmm0_xmm1_imm_0x2d() {
    // PSHUFLW XMM0, XMM1, 0x2D
    let code = [
        0xf2, 0x0f, 0x70, 0xc1, 0x2d, // PSHUFLW XMM0, XMM1, 0x2D
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer value tests (register-to-register + imm8 via set_xmm/get_xmm)
//
// PSHUFLW shuffles the LOW four 16-bit words by imm8 (2 bits per result word);
// the high qword is copied unchanged.
//   SRC = XMM1 = 0x01020304050607081122334455667788
//     low words:  word0=0x7788, word1=0x5566, word2=0x3344, word3=0x1122
//     high qword (unchanged) = 0x0102030405060708
// Computed by hand.
// ============================================================================

const KAT_PSHUFLW_SRC: u128 = 0x01020304050607081122334455667788;

#[test]
fn kat_pshuflw_reverse_low() {
    // PSHUFLW XMM0, XMM1, 0x1B (F2 0F 70 C1 1B) reverses the low 4 words.
    let code = [0xf2, 0x0f, 0x70, 0xc1, 0x1b, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFLW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        0x01020304050607087788556633441122,
        "PSHUFLW got {:032x}",
        get_xmm(&regs, 0)
    );
}

#[test]
fn kat_pshuflw_identity_keeps_high() {
    // imm 0xE4 is identity for the low words; high qword always copied verbatim.
    let code = [0xf2, 0x0f, 0x70, 0xc1, 0xe4, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFLW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), KAT_PSHUFLW_SRC);
}

#[test]
fn kat_pshuflw_broadcast_word0() {
    // imm 0x00 broadcasts low word0 into all four low words; high qword intact.
    let code = [0xf2, 0x0f, 0x70, 0xc1, 0x00, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KAT_PSHUFLW_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x01020304050607087788778877887788);
}
