use crate::common::*;

// CVTSS2SI - Convert Scalar Single Precision to Signed Integer
// Opcode: F3 0F 2D /r    CVTSS2SI r32, xmm1/m32
//         F3 REX.W 0F 2D /r    CVTSS2SI r64, xmm1/m32

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtss2si_eax_xmm0() {
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_ebx_xmm1() {
    let code = [0xf3, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSS2SI EBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_ecx_xmm2() {
    let code = [0xf3, 0x0f, 0x2d, 0xca, 0xf4]; // CVTSS2SI ECX, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_edx_xmm3() {
    let code = [0xf3, 0x0f, 0x2d, 0xd3, 0xf4]; // CVTSS2SI EDX, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_esi_xmm7() {
    let code = [0xf3, 0x0f, 0x2d, 0xf7, 0xf4]; // CVTSS2SI ESI, XMM7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtss2si_rax_xmm0_64() {
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI RAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_rbx_xmm1_64() {
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSS2SI RBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_r9_xmm8_64() {
    let code = [0xf3, 0x4d, 0x0f, 0x2d, 0xc8, 0xf4]; // CVTSS2SI R9, XMM8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory Tests
#[test]
fn test_cvtss2si_mem32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 42.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_mem64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 1234567.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Special Values
#[test]
fn test_cvtss2si_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_negative_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Rounding Tests
#[test]
fn test_cvtss2si_round_down() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 42.3;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_round_up() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 42.7;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_round_half_even() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_round_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -42.7;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Normal Values
#[test]
fn test_cvtss2si_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_minus_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_positive_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 1000000.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_negative_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -1000000.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Overflow Cases (should return indefinite integer)
#[test]
fn test_cvtss2si_overflow_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 3.0e9; // Larger than i32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_overflow_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -3.0e9; // Less than i32::MIN
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_infinity_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = f32::INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_infinity_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = f32::NEG_INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = f32::NAN;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Edge Cases
#[test]
fn test_cvtss2si_near_max_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 2147483000.0; // Near i32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_near_min_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -2147483000.0; // Near i32::MIN
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_small_fractional() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 0.9;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_very_small() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 0.000001;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Misc
#[test]
fn test_cvtss2si_100_point_99() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 100.99;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_minus_100_point_99() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = -100.99;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_displacement() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x40, 0x10, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 777.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtss2si_multiple() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x2d, 0x00, // CVTSS2SI EAX, [RAX]
        0xf3, 0x0f, 0x2d, 0x08, // CVTSS2SI ECX, [RAX]
        0xf3, 0x0f, 0x2d, 0x10, // CVTSS2SI EDX, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f32 = 456.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
