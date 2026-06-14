use crate::common::*;

// MOVD/MOVQ - Move Doubleword/Quadword
// Opcode: 66 0F 6E /r         MOVD xmm, r/m32
//         66 REX.W 0F 6E /r   MOVQ xmm, r/m64
//         66 0F 7E /r         MOVD r/m32, xmm
//         66 REX.W 0F 7E /r   MOVQ r/m64, xmm
//         F3 0F 7E /r         MOVQ xmm, xmm/m64

const DATA_ADDR: u64 = 0x3000;

// MOVD xmm, r32 - Move 32-bit GPR to XMM (zero upper)
#[test]
fn test_movd_xmm0_eax() {
    let code = [0x66, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM0, EAX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm1_ebx() {
    let code = [0x66, 0x0f, 0x6e, 0xcb, 0xf4]; // MOVD XMM1, EBX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm2_ecx() {
    let code = [0x66, 0x0f, 0x6e, 0xd1, 0xf4]; // MOVD XMM2, ECX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm3_edx() {
    let code = [0x66, 0x0f, 0x6e, 0xda, 0xf4]; // MOVD XMM3, EDX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm7_esi() {
    let code = [0x66, 0x0f, 0x6e, 0xfe, 0xf4]; // MOVD XMM7, ESI
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm8_r8d() {
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM8, R8D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm15_r15d() {
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xff, 0xf4]; // MOVD XMM15, R15D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVD r32, xmm - Extract low 32 bits to GPR
#[test]
fn test_movd_eax_xmm0() {
    let code = [0x66, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVD EAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_ebx_xmm1() {
    let code = [0x66, 0x0f, 0x7e, 0xcb, 0xf4]; // MOVD EBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_ecx_xmm2() {
    let code = [0x66, 0x0f, 0x7e, 0xd1, 0xf4]; // MOVD ECX, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_r8d_xmm8() {
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVD R8D, XMM8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVQ xmm, r64 - Move 64-bit GPR to XMM (zero upper)
#[test]
fn test_movq_xmm0_rax() {
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVQ XMM0, RAX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm1_rbx() {
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xcb, 0xf4]; // MOVQ XMM1, RBX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm2_rcx() {
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xd1, 0xf4]; // MOVQ XMM2, RCX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm3_rdx() {
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xda, 0xf4]; // MOVQ XMM3, RDX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm7_rsi() {
    let code = [0x66, 0x48, 0x0f, 0x6e, 0xfe, 0xf4]; // MOVQ XMM7, RSI
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm8_r8() {
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVQ XMM8, R8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm15_r15() {
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xff, 0xf4]; // MOVQ XMM15, R15
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVQ r64, xmm - Extract low 64 bits to GPR
#[test]
fn test_movq_rax_xmm0() {
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVQ RAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_rbx_xmm1() {
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xcb, 0xf4]; // MOVQ RBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_rcx_xmm2() {
    let code = [0x66, 0x48, 0x0f, 0x7e, 0xd1, 0xf4]; // MOVQ RCX, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_r8_xmm8() {
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xc0, 0xf4]; // MOVQ R8, XMM8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_r15_xmm15() {
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xff, 0xf4]; // MOVQ R15, XMM15
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVD xmm, m32 - Load 32 bits from memory, zero upper
#[test]
fn test_movd_xmm0_mem32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u32 = 0x12345678;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm5_mem32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x28, 0xf4]); // MOVD XMM5, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u32 = 0xABCDEF00;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVD m32, xmm - Store low 32 bits to memory
#[test]
fn test_movd_mem32_xmm0() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x7e, 0x00, 0xf4]); // MOVD [RAX], XMM0

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_mem32_xmm7() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x7e, 0x38, 0xf4]); // MOVD [RAX], XMM7

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVQ xmm, m64 - Load 64 bits from memory, zero upper
#[test]
fn test_movq_xmm0_mem64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u64 = 0x123456789ABCDEF0;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm3_mem64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x18, 0xf4]); // MOVQ XMM3, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u64 = 0xFEDCBA9876543210;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVQ m64, xmm - Store low 64 bits to memory
#[test]
fn test_movq_mem64_xmm0() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x7e, 0x00, 0xf4]); // MOVQ [RAX], XMM0

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_mem64_xmm6() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x7e, 0x30, 0xf4]); // MOVQ [RAX], XMM6

    let (mut vcpu, _) = setup_vm(&full_code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// MOVQ xmm, xmm - Copy low 64 bits, zero upper
#[test]
fn test_movq_xmm0_xmm1() {
    let code = [0xf3, 0x0f, 0x7e, 0xc1, 0xf4]; // MOVQ XMM0, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm2_xmm3() {
    let code = [0xf3, 0x0f, 0x7e, 0xd3, 0xf4]; // MOVQ XMM2, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm7_xmm0() {
    let code = [0xf3, 0x0f, 0x7e, 0xf8, 0xf4]; // MOVQ XMM7, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm8_xmm15() {
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xc7, 0xf4]; // MOVQ XMM8, XMM15
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm15_xmm8() {
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xf8, 0xf4]; // MOVQ XMM15, XMM8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with displacement addressing
#[test]
fn test_movd_xmm0_mem32_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 8).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x40, 0x08, 0xf4]); // MOVD XMM0, [RAX+8]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u32 = 0xDEADBEEF;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm1_mem64_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x48, 0x10, 0xf4]); // MOVQ XMM1, [RAX+16]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u64 = 0xCAFEBABEDEADBEEF;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test special values
#[test]
fn test_movd_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u32 = 0;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_all_ones() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x6e, 0x00, 0xf4]); // MOVD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u32 = 0xFFFFFFFF;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u64 = 0;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_all_ones() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x48, 0x0f, 0x6e, 0x00, 0xf4]); // MOVQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: u64 = 0xFFFFFFFFFFFFFFFF;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Test chained operations
#[test]
fn test_movd_chain() {
    let code = [
        0x66, 0x0f, 0x6e, 0xc0, // MOVD XMM0, EAX
        0x66, 0x0f, 0x6e, 0xcb, // MOVD XMM1, EBX
        0x66, 0x0f, 0x6e, 0xd1, // MOVD XMM2, ECX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_chain() {
    let code = [
        0x66, 0x48, 0x0f, 0x6e, 0xc0, // MOVQ XMM0, RAX
        0x66, 0x48, 0x0f, 0x6e, 0xcb, // MOVQ XMM1, RBX
        0x66, 0x48, 0x0f, 0x6e, 0xd1, // MOVQ XMM2, RCX
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_roundtrip() {
    let code = [
        0x66, 0x0f, 0x6e, 0xc0, // MOVD XMM0, EAX
        0x66, 0x0f, 0x7e, 0xc3, // MOVD EBX, XMM0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_roundtrip() {
    let code = [
        0x66, 0x48, 0x0f, 0x6e, 0xc0, // MOVQ XMM0, RAX
        0x66, 0x48, 0x0f, 0x7e, 0xc3, // MOVQ RBX, XMM0
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test high XMM registers with high GPRs
#[test]
fn test_movd_xmm8_r8d_high() {
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc0, 0xf4]; // MOVD XMM8, R8D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm9_r9d() {
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xc9, 0xf4]; // MOVD XMM9, R9D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_xmm14_r14d() {
    let code = [0x66, 0x45, 0x0f, 0x6e, 0xf6, 0xf4]; // MOVD XMM14, R14D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm9_r9() {
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xc9, 0xf4]; // MOVQ XMM9, R9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm10_r10() {
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xd2, 0xf4]; // MOVQ XMM10, R10
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm14_r14() {
    let code = [0x66, 0x4d, 0x0f, 0x6e, 0xf6, 0xf4]; // MOVQ XMM14, R14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test extract from high XMM to high GPR
#[test]
fn test_movd_r9d_xmm9() {
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xc9, 0xf4]; // MOVD R9D, XMM9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movd_r12d_xmm12() {
    let code = [0x66, 0x45, 0x0f, 0x7e, 0xe4, 0xf4]; // MOVD R12D, XMM12
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_r11_xmm11() {
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xdb, 0xf4]; // MOVQ R11, XMM11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_r13_xmm13() {
    let code = [0x66, 0x4d, 0x0f, 0x7e, 0xed, 0xf4]; // MOVQ R13, XMM13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test MOVQ xmm, xmm with high registers
#[test]
fn test_movq_xmm10_xmm11() {
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xd3, 0xf4]; // MOVQ XMM10, XMM11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm12_xmm13() {
    let code = [0xf3, 0x45, 0x0f, 0x7e, 0xe5, 0xf4]; // MOVQ XMM12, XMM13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_movq_xmm14_xmm1() {
    let code = [0xf3, 0x44, 0x0f, 0x7e, 0xf1, 0xf4]; // MOVQ XMM14, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
