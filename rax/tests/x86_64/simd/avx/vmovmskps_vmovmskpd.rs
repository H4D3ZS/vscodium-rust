use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VMOVMSKPS - Extract Packed Single-Precision Floating-Point Sign Mask
// VMOVMSKPD - Extract Packed Double-Precision Floating-Point Sign Mask
//
// VMOVMSKPS extracts the sign bits from packed single-precision floating-point values
// and stores them in a general-purpose register. Each sign bit becomes one bit in the result.
//
// VMOVMSKPD extracts the sign bits from packed double-precision floating-point values
// and stores them in a general-purpose register.
//
// For 128-bit (XMM) operands:
// - VMOVMSKPS extracts 4 sign bits (bits 3:0 of result)
// - VMOVMSKPD extracts 2 sign bits (bits 1:0 of result)
//
// For 256-bit (YMM) operands:
// - VMOVMSKPS extracts 8 sign bits (bits 7:0 of result)
// - VMOVMSKPD extracts 4 sign bits (bits 3:0 of result)
//
// Opcodes:
// VEX.128.0F.WIG 50 /r    VMOVMSKPS r32, xmm2   - Extract sign mask from XMM (4 bits)
// VEX.256.0F.WIG 50 /r    VMOVMSKPS r32, ymm2   - Extract sign mask from YMM (8 bits)
// VEX.128.66.0F.WIG 50 /r VMOVMSKPD r32, xmm2   - Extract sign mask from XMM (2 bits)
// VEX.256.66.0F.WIG 50 /r VMOVMSKPD r32, ymm2   - Extract sign mask from YMM (4 bits)

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMOVMSKPS Tests - 128-bit XMM registers (4 sign bits)
// ============================================================================

#[test]
fn test_vmovmskps_xmm0_to_eax() {
    // VMOVMSKPS EAX, XMM0
    let code = [
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm1_to_eax() {
    // VMOVMSKPS EAX, XMM1
    let code = [
        0xc5, 0xf8, 0x50, 0xc1, // VMOVMSKPS EAX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm2_to_ebx() {
    // VMOVMSKPS EBX, XMM2
    let code = [
        0xc5, 0xf8, 0x50, 0xda, // VMOVMSKPS EBX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm3_to_ecx() {
    // VMOVMSKPS ECX, XMM3
    let code = [
        0xc5, 0xf8, 0x50, 0xcb, // VMOVMSKPS ECX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm4_to_edx() {
    // VMOVMSKPS EDX, XMM4
    let code = [
        0xc5, 0xf8, 0x50, 0xd4, // VMOVMSKPS EDX, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm5_to_esi() {
    // VMOVMSKPS ESI, XMM5
    let code = [
        0xc5, 0xf8, 0x50, 0xf5, // VMOVMSKPS ESI, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm6_to_edi() {
    // VMOVMSKPS EDI, XMM6
    let code = [
        0xc5, 0xf8, 0x50, 0xfe, // VMOVMSKPS EDI, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm7_to_eax() {
    // VMOVMSKPS EAX, XMM7
    let code = [
        0xc5, 0xf8, 0x50, 0xc7, // VMOVMSKPS EAX, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmovmskps_xmm8_to_eax() {
    // VMOVMSKPS EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc0, // VMOVMSKPS EAX, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm9_to_ebx() {
    // VMOVMSKPS EBX, XMM9
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xd9, // VMOVMSKPS EBX, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm10_to_ecx() {
    // VMOVMSKPS ECX, XMM10
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xca, // VMOVMSKPS ECX, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm11_to_edx() {
    // VMOVMSKPS EDX, XMM11
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xd3, // VMOVMSKPS EDX, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm12_to_esi() {
    // VMOVMSKPS ESI, XMM12
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xf4, // VMOVMSKPS ESI, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm13_to_edi() {
    // VMOVMSKPS EDI, XMM13
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xfd, // VMOVMSKPS EDI, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm14_to_eax() {
    // VMOVMSKPS EAX, XMM14
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc6, // VMOVMSKPS EAX, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_xmm15_to_eax() {
    // VMOVMSKPS EAX, XMM15
    let code = [
        0xc4, 0xc1, 0x78, 0x50, 0xc7, // VMOVMSKPS EAX, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - 256-bit YMM registers (8 sign bits)
// ============================================================================

#[test]
fn test_vmovmskps_ymm0_to_eax() {
    // VMOVMSKPS EAX, YMM0
    let code = [
        0xc5, 0xfc, 0x50, 0xc0, // VMOVMSKPS EAX, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm1_to_eax() {
    // VMOVMSKPS EAX, YMM1
    let code = [
        0xc5, 0xfc, 0x50, 0xc1, // VMOVMSKPS EAX, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm2_to_ebx() {
    // VMOVMSKPS EBX, YMM2
    let code = [
        0xc5, 0xfc, 0x50, 0xda, // VMOVMSKPS EBX, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm3_to_ecx() {
    // VMOVMSKPS ECX, YMM3
    let code = [
        0xc5, 0xfc, 0x50, 0xcb, // VMOVMSKPS ECX, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm4_to_edx() {
    // VMOVMSKPS EDX, YMM4
    let code = [
        0xc5, 0xfc, 0x50, 0xd4, // VMOVMSKPS EDX, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm5_to_esi() {
    // VMOVMSKPS ESI, YMM5
    let code = [
        0xc5, 0xfc, 0x50, 0xf5, // VMOVMSKPS ESI, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm6_to_edi() {
    // VMOVMSKPS EDI, YMM6
    let code = [
        0xc5, 0xfc, 0x50, 0xfe, // VMOVMSKPS EDI, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm7_to_eax() {
    // VMOVMSKPS EAX, YMM7
    let code = [
        0xc5, 0xfc, 0x50, 0xc7, // VMOVMSKPS EAX, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm8_to_eax() {
    // VMOVMSKPS EAX, YMM8
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xc0, // VMOVMSKPS EAX, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm9_to_ebx() {
    // VMOVMSKPS EBX, YMM9
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xd9, // VMOVMSKPS EBX, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm10_to_ecx() {
    // VMOVMSKPS ECX, YMM10
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xca, // VMOVMSKPS ECX, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_ymm15_to_eax() {
    // VMOVMSKPS EAX, YMM15
    let code = [
        0xc4, 0xc1, 0x7c, 0x50, 0xc7, // VMOVMSKPS EAX, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPS Tests - After comparison operations
// ============================================================================

#[test]
fn test_vmovmskps_after_vcmpps_eq() {
    // VCMPPS (EQ) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf0, 0xc2, 0xc2, 0x00, // VCMPPS XMM0, XMM1, XMM2, 0 (EQ)
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_after_vcmpps_lt() {
    // VCMPPS (LT) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf0, 0xc2, 0xc2, 0x01, // VCMPPS XMM0, XMM1, XMM2, 1 (LT)
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_after_vcmpps_ymm() {
    // VCMPPS YMM (EQ) followed by VMOVMSKPS
    let code = [
        0xc5, 0xf4, 0xc2, 0xc2, 0x00, // VCMPPS YMM0, YMM1, YMM2, 0 (EQ)
        0xc5, 0xfc, 0x50, 0xc0, // VMOVMSKPS EAX, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - 128-bit XMM registers (2 sign bits)
// ============================================================================

#[test]
fn test_vmovmskpd_xmm0_to_eax() {
    // VMOVMSKPD EAX, XMM0
    let code = [
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm1_to_eax() {
    // VMOVMSKPD EAX, XMM1
    let code = [
        0xc5, 0xf9, 0x50, 0xc1, // VMOVMSKPD EAX, XMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm2_to_ebx() {
    // VMOVMSKPD EBX, XMM2
    let code = [
        0xc5, 0xf9, 0x50, 0xda, // VMOVMSKPD EBX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm3_to_ecx() {
    // VMOVMSKPD ECX, XMM3
    let code = [
        0xc5, 0xf9, 0x50, 0xcb, // VMOVMSKPD ECX, XMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm4_to_edx() {
    // VMOVMSKPD EDX, XMM4
    let code = [
        0xc5, 0xf9, 0x50, 0xd4, // VMOVMSKPD EDX, XMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm5_to_esi() {
    // VMOVMSKPD ESI, XMM5
    let code = [
        0xc5, 0xf9, 0x50, 0xf5, // VMOVMSKPD ESI, XMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm6_to_edi() {
    // VMOVMSKPD EDI, XMM6
    let code = [
        0xc5, 0xf9, 0x50, 0xfe, // VMOVMSKPD EDI, XMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm7_to_eax() {
    // VMOVMSKPD EAX, XMM7
    let code = [
        0xc5, 0xf9, 0x50, 0xc7, // VMOVMSKPD EAX, XMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vmovmskpd_xmm8_to_eax() {
    // VMOVMSKPD EAX, XMM8
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc0, // VMOVMSKPD EAX, XMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm9_to_ebx() {
    // VMOVMSKPD EBX, XMM9
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xd9, // VMOVMSKPD EBX, XMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm10_to_ecx() {
    // VMOVMSKPD ECX, XMM10
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xca, // VMOVMSKPD ECX, XMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm11_to_edx() {
    // VMOVMSKPD EDX, XMM11
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xd3, // VMOVMSKPD EDX, XMM11
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm12_to_esi() {
    // VMOVMSKPD ESI, XMM12
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xf4, // VMOVMSKPD ESI, XMM12
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm13_to_edi() {
    // VMOVMSKPD EDI, XMM13
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xfd, // VMOVMSKPD EDI, XMM13
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm14_to_eax() {
    // VMOVMSKPD EAX, XMM14
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc6, // VMOVMSKPD EAX, XMM14
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_xmm15_to_eax() {
    // VMOVMSKPD EAX, XMM15
    let code = [
        0xc4, 0xc1, 0x79, 0x50, 0xc7, // VMOVMSKPD EAX, XMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - 256-bit YMM registers (4 sign bits)
// ============================================================================

#[test]
fn test_vmovmskpd_ymm0_to_eax() {
    // VMOVMSKPD EAX, YMM0
    let code = [
        0xc5, 0xfd, 0x50, 0xc0, // VMOVMSKPD EAX, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm1_to_eax() {
    // VMOVMSKPD EAX, YMM1
    let code = [
        0xc5, 0xfd, 0x50, 0xc1, // VMOVMSKPD EAX, YMM1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm2_to_ebx() {
    // VMOVMSKPD EBX, YMM2
    let code = [
        0xc5, 0xfd, 0x50, 0xda, // VMOVMSKPD EBX, YMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm3_to_ecx() {
    // VMOVMSKPD ECX, YMM3
    let code = [
        0xc5, 0xfd, 0x50, 0xcb, // VMOVMSKPD ECX, YMM3
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm4_to_edx() {
    // VMOVMSKPD EDX, YMM4
    let code = [
        0xc5, 0xfd, 0x50, 0xd4, // VMOVMSKPD EDX, YMM4
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm5_to_esi() {
    // VMOVMSKPD ESI, YMM5
    let code = [
        0xc5, 0xfd, 0x50, 0xf5, // VMOVMSKPD ESI, YMM5
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm6_to_edi() {
    // VMOVMSKPD EDI, YMM6
    let code = [
        0xc5, 0xfd, 0x50, 0xfe, // VMOVMSKPD EDI, YMM6
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm7_to_eax() {
    // VMOVMSKPD EAX, YMM7
    let code = [
        0xc5, 0xfd, 0x50, 0xc7, // VMOVMSKPD EAX, YMM7
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm8_to_eax() {
    // VMOVMSKPD EAX, YMM8
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xc0, // VMOVMSKPD EAX, YMM8
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm9_to_ebx() {
    // VMOVMSKPD EBX, YMM9
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xd9, // VMOVMSKPD EBX, YMM9
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm10_to_ecx() {
    // VMOVMSKPD ECX, YMM10
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xca, // VMOVMSKPD ECX, YMM10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_ymm15_to_eax() {
    // VMOVMSKPD EAX, YMM15
    let code = [
        0xc4, 0xc1, 0x7d, 0x50, 0xc7, // VMOVMSKPD EAX, YMM15
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VMOVMSKPD Tests - After comparison operations
// ============================================================================

#[test]
fn test_vmovmskpd_after_vcmppd_eq() {
    // VCMPPD (EQ) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf1, 0xc2, 0xc2, 0x00, // VCMPPD XMM0, XMM1, XMM2, 0 (EQ)
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_after_vcmppd_lt() {
    // VCMPPD (LT) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf1, 0xc2, 0xc2, 0x01, // VCMPPD XMM0, XMM1, XMM2, 1 (LT)
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_after_vcmppd_ymm() {
    // VCMPPD YMM (EQ) followed by VMOVMSKPD
    let code = [
        0xc5, 0xf5, 0xc2, 0xc2, 0x00, // VCMPPD YMM0, YMM1, YMM2, 0 (EQ)
        0xc5, 0xfd, 0x50, 0xc0, // VMOVMSKPD EAX, YMM0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskps_multiple_extracts() {
    // Multiple VMOVMSKPS operations
    let code = [
        0xc5, 0xf8, 0x50, 0xc0, // VMOVMSKPS EAX, XMM0
        0xc5, 0xf8, 0x50, 0xd9, // VMOVMSKPS EBX, XMM1
        0xc5, 0xf8, 0x50, 0xca, // VMOVMSKPS ECX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vmovmskpd_multiple_extracts() {
    // Multiple VMOVMSKPD operations
    let code = [
        0xc5, 0xf9, 0x50, 0xc0, // VMOVMSKPD EAX, XMM0
        0xc5, 0xf9, 0x50, 0xd9, // VMOVMSKPD EBX, XMM1
        0xc5, 0xf9, 0x50, 0xca, // VMOVMSKPD ECX, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Known-answer VALUE tests : VMOVMSKPS/PD gather element sign bits into a GPR.
//   PS: 1 bit per 32-bit lane ; PD: 1 bit per 64-bit lane.
// ============================================================================

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::VCpu;

fn kmm_set(vcpu: &mut X86_64Vcpu, idx: usize, lo: u128, hi: u128) {
    let mut regs = vcpu.get_regs().unwrap();
    regs.xmm[idx][0] = lo as u64;
    regs.xmm[idx][1] = (lo >> 64) as u64;
    regs.ymm_high[idx][0] = hi as u64;
    regs.ymm_high[idx][1] = (hi >> 64) as u64;
    vcpu.set_regs(&regs).unwrap();
}

// Lane sign patterns: bit set in a lane => that element's MSB is 1.
const PS_SIGN_HI_BIT: u128 = 0x8000_0000u128; // MSB of a 32-bit lane.
const PD_SIGN_HI_BIT: u128 = 0x8000_0000_0000_0000u128; // MSB of a 64-bit lane.

#[test]
fn test_vmovmskps_xmm_value() {
    // VMOVMSKPS EAX, XMM0 ; lanes 0 and 3 negative -> mask 0b1001 = 0x9.
    let code = [0xc5, 0xf8, 0x50, 0xc0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let lo = (PS_SIGN_HI_BIT << 0) | (PS_SIGN_HI_BIT << 96);
    kmm_set(&mut vcpu, 0, lo, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0b1001);
}

#[test]
fn test_vmovmskps_ymm_value() {
    // VMOVMSKPS EAX, YMM0 ; 8 lanes -> set lanes {0,2,5,7} = 0b1010_0101 = 0xA5.
    let code = [0xc5, 0xfc, 0x50, 0xc0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let lo = (PS_SIGN_HI_BIT << 0) | (PS_SIGN_HI_BIT << 64); // lanes 0,2
    let hi = (PS_SIGN_HI_BIT << 32) | (PS_SIGN_HI_BIT << 96); // lanes 5,7
    kmm_set(&mut vcpu, 0, lo, hi);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0b1010_0101);
}

#[test]
fn test_vmovmskpd_xmm_value() {
    // VMOVMSKPD EAX, XMM0 ; lane 1 negative -> mask 0b10 = 0x2.
    let code = [0xc5, 0xf9, 0x50, 0xc0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let lo = PD_SIGN_HI_BIT << 64; // high qword sign set
    kmm_set(&mut vcpu, 0, lo, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xF, 0b10);
}

#[test]
fn test_vmovmskpd_ymm_value() {
    // VMOVMSKPD EAX, YMM0 ; 4 lanes -> set lanes {0,3} = 0b1001 = 0x9.
    let code = [0xc5, 0xfd, 0x50, 0xc0, 0xf4];
    let (mut vcpu, _) = setup_vm(&code, None);
    let lo = PD_SIGN_HI_BIT; // lane 0
    let hi = PD_SIGN_HI_BIT << 64; // lane 3 (high qword of high lane)
    kmm_set(&mut vcpu, 0, lo, hi);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xF, 0b1001);
}
