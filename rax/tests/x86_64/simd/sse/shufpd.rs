use crate::common::*;

// SHUFPD - Packed Interleave Shuffle of Pairs of Double Precision Floating-Point Values
//
// Selects double precision floating-point values from two source operands using bits in the
// immediate byte and stores the results in the destination operand.
//
// The immediate byte uses 2 bits (only bits 0-1 are used for 128-bit version):
// - Bit 0 selects from SRC1 (0=low element, 1=high element) -> DEST[63:0]
// - Bit 1 selects from SRC2 (0=low element, 1=high element) -> DEST[127:64]
//
// Opcode: 66 0F C6 /r ib    SHUFPD xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with all 4 possible immediate values (0x00-0x03)
// ============================================================================

#[test]
fn test_shufpd_xmm0_xmm1_imm_0x00() {
    // SHUFPD XMM0, XMM1, 0x00
    // Bit pattern: 00
    // Result: XMM0[low], XMM1[low]
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x00, // SHUFPD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_xmm1_imm_0x01() {
    // SHUFPD XMM0, XMM1, 0x01
    // Bit pattern: 01
    // Result: XMM0[high], XMM1[low]
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x01, // SHUFPD XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_xmm1_imm_0x02() {
    // SHUFPD XMM0, XMM1, 0x02
    // Bit pattern: 10
    // Result: XMM0[low], XMM1[high]
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x02, // SHUFPD XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_xmm1_imm_0x03() {
    // SHUFPD XMM0, XMM1, 0x03
    // Bit pattern: 11
    // Result: XMM0[high], XMM1[high]
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x03, // SHUFPD XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs - imm 0x00
// ============================================================================

#[test]
fn test_shufpd_xmm2_xmm3_imm_0x00() {
    // SHUFPD XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0xc6, 0xd3, 0x00, // SHUFPD XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm4_xmm5_imm_0x00() {
    // SHUFPD XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0xc6, 0xe5, 0x00, // SHUFPD XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm6_xmm7_imm_0x00() {
    // SHUFPD XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0xc6, 0xf7, 0x00, // SHUFPD XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs - imm 0x01
// ============================================================================

#[test]
fn test_shufpd_xmm1_xmm2_imm_0x01() {
    // SHUFPD XMM1, XMM2, 0x01
    let code = [
        0x66, 0x0f, 0xc6, 0xca, 0x01, // SHUFPD XMM1, XMM2, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm3_xmm4_imm_0x01() {
    // SHUFPD XMM3, XMM4, 0x01
    let code = [
        0x66, 0x0f, 0xc6, 0xdc, 0x01, // SHUFPD XMM3, XMM4, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm5_xmm6_imm_0x01() {
    // SHUFPD XMM5, XMM6, 0x01
    let code = [
        0x66, 0x0f, 0xc6, 0xee, 0x01, // SHUFPD XMM5, XMM6, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm7_xmm0_imm_0x01() {
    // SHUFPD XMM7, XMM0, 0x01
    let code = [
        0x66, 0x0f, 0xc6, 0xf8, 0x01, // SHUFPD XMM7, XMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs - imm 0x02
// ============================================================================

#[test]
fn test_shufpd_xmm2_xmm3_imm_0x02() {
    // SHUFPD XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0xc6, 0xd3, 0x02, // SHUFPD XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm4_xmm5_imm_0x02() {
    // SHUFPD XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0xc6, 0xe5, 0x02, // SHUFPD XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm1_xmm7_imm_0x02() {
    // SHUFPD XMM1, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0xc6, 0xcf, 0x02, // SHUFPD XMM1, XMM7, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with different register pairs - imm 0x03
// ============================================================================

#[test]
fn test_shufpd_xmm1_xmm2_imm_0x03() {
    // SHUFPD XMM1, XMM2, 0x03
    let code = [
        0x66, 0x0f, 0xc6, 0xca, 0x03, // SHUFPD XMM1, XMM2, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm3_xmm4_imm_0x03() {
    // SHUFPD XMM3, XMM4, 0x03
    let code = [
        0x66, 0x0f, 0xc6, 0xdc, 0x03, // SHUFPD XMM3, XMM4, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm5_xmm6_imm_0x03() {
    // SHUFPD XMM5, XMM6, 0x03
    let code = [
        0x66, 0x0f, 0xc6, 0xee, 0x03, // SHUFPD XMM5, XMM6, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm6_xmm7_imm_0x03() {
    // SHUFPD XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0xc6, 0xf7, 0x03, // SHUFPD XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_shufpd_xmm8_xmm9_imm_0x00() {
    // SHUFPD XMM8, XMM9, 0x00
    let code = [
        0x66, 0x45, 0x0f, 0xc6, 0xc1, 0x00, // SHUFPD XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm10_xmm11_imm_0x01() {
    // SHUFPD XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0xc6, 0xd3, 0x01, // SHUFPD XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm12_xmm13_imm_0x02() {
    // SHUFPD XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0xc6, 0xe5, 0x02, // SHUFPD XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm14_xmm15_imm_0x03() {
    // SHUFPD XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0xc6, 0xf7, 0x03, // SHUFPD XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_xmm8_imm_0x00() {
    // SHUFPD XMM0, XMM8, 0x00
    let code = [
        0x66, 0x44, 0x0f, 0xc6, 0xc0, 0x00, // SHUFPD XMM0, XMM8, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm15_xmm0_imm_0x03() {
    // SHUFPD XMM15, XMM0, 0x03
    let code = [
        0x66, 0x44, 0x0f, 0xc6, 0xf8, 0x03, // SHUFPD XMM15, XMM0, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm8_xmm0_imm_0x01() {
    // SHUFPD XMM8, XMM0, 0x01
    let code = [
        0x66, 0x44, 0x0f, 0xc6, 0xc0, 0x01, // SHUFPD XMM8, XMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_xmm15_imm_0x02() {
    // SHUFPD XMM0, XMM15, 0x02
    let code = [
        0x66, 0x44, 0x0f, 0xc6, 0xc7, 0x02, // SHUFPD XMM0, XMM15, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shufpd_xmm0_mem_imm_0x00() {
    // SHUFPD XMM0, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x00, 0x00, // SHUFPD XMM0, [RAX], 0x00
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    // Write test data to memory (2 doubles)
    let double_data: [f64; 2] = [1.0, 2.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_mem_imm_0x01() {
    // SHUFPD XMM0, [mem], 0x01
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x00, 0x01, // SHUFPD XMM0, [RAX], 0x01
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    let double_data: [f64; 2] = [3.0, 4.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    mem.write_slice(&bytes, vm_memory::GuestAddress(ALIGNED_ADDR))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_xmm0_mem_imm_0x02() {
    // SHUFPD XMM0, [mem], 0x02
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x00, 0x02, // SHUFPD XMM0, [RAX], 0x02
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
fn test_shufpd_xmm0_mem_imm_0x03() {
    // SHUFPD XMM0, [mem], 0x03
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x00, 0x03, // SHUFPD XMM0, [RAX], 0x03
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
fn test_shufpd_xmm1_mem_imm_0x00() {
    // SHUFPD XMM1, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x08, 0x00, // SHUFPD XMM1, [RAX], 0x00
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

#[test]
fn test_shufpd_xmm7_mem_imm_0x03() {
    // SHUFPD XMM7, [mem], 0x03
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x38, 0x03, // SHUFPD XMM7, [RAX], 0x03
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);

    mem.write_slice(
        &[
            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
            0x33, 0x33,
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
fn test_shufpd_xmm0_mem_displacement_imm_0x00() {
    // SHUFPD XMM0, [RAX + disp], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x40, 0x10, 0x00, // SHUFPD XMM0, [RAX+0x10], 0x00
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
fn test_shufpd_xmm1_mem_rbx_imm_0x01() {
    // SHUFPD XMM1, [RBX], 0x01
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x0b, 0x01, // SHUFPD XMM1, [RBX], 0x01
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

#[test]
fn test_shufpd_xmm2_mem_rcx_imm_0x02() {
    // SHUFPD XMM2, [RCX], 0x02
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xc6, 0x11, 0x02, // SHUFPD XMM2, [RCX], 0x02
        0xf4, // HLT
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    mem.write_slice(
        &[
            0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99,
            0x99, 0x99,
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
fn test_shufpd_sequential_operations() {
    // Multiple SHUFPD operations in sequence
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x00, // SHUFPD XMM0, XMM1, 0x00
        0x66, 0x0f, 0xc6, 0xd3, 0x01, // SHUFPD XMM2, XMM3, 0x01
        0x66, 0x0f, 0xc6, 0xe5, 0x02, // SHUFPD XMM4, XMM5, 0x02
        0x66, 0x0f, 0xc6, 0xf7, 0x03, // SHUFPD XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_same_register_imm_0x00() {
    // SHUFPD XMM0, XMM0, 0x00 (shuffle with itself)
    let code = [
        0x66, 0x0f, 0xc6, 0xc0, 0x00, // SHUFPD XMM0, XMM0, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_same_register_imm_0x01() {
    // SHUFPD XMM1, XMM1, 0x01 (shuffle with itself, swap)
    let code = [
        0x66, 0x0f, 0xc6, 0xc9, 0x01, // SHUFPD XMM1, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_same_register_imm_0x03() {
    // SHUFPD XMM2, XMM2, 0x03
    let code = [
        0x66, 0x0f, 0xc6, 0xd2, 0x03, // SHUFPD XMM2, XMM2, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_element_swap_imm_0x01() {
    // SHUFPD XMM0, XMM1, 0x01 - swap pattern
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x01, // SHUFPD XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_broadcast_low_imm_0x00() {
    // SHUFPD XMM3, XMM3, 0x00 - broadcast low element
    let code = [
        0x66, 0x0f, 0xc6, 0xdb, 0x00, // SHUFPD XMM3, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_broadcast_high_imm_0x03() {
    // SHUFPD XMM4, XMM4, 0x03 - broadcast high element
    let code = [
        0x66, 0x0f, 0xc6, 0xe4, 0x03, // SHUFPD XMM4, XMM4, 0x03
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_shufpd_chain_operations() {
    // Chain of SHUFPD operations
    let code = [
        0x66, 0x0f, 0xc6, 0xc1, 0x00, // SHUFPD XMM0, XMM1, 0x00
        0x66, 0x0f, 0xc6, 0xc2, 0x01, // SHUFPD XMM0, XMM2, 0x01
        0x66, 0x0f, 0xc6, 0xc3, 0x02, // SHUFPD XMM0, XMM3, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
