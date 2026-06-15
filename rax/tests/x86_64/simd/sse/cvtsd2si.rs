use crate::common::*;

// CVTSD2SI - Convert Scalar Double Precision to Signed Integer
// Opcode: F2 0F 2D /r    CVTSD2SI r32, xmm1/m64
//         F2 REX.W 0F 2D /r    CVTSD2SI r64, xmm1/m64

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtsd2si_eax_xmm0() {
    let code = [0xf2, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI EAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_ebx_xmm1() {
    let code = [0xf2, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSD2SI EBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_ecx_xmm2() {
    let code = [0xf2, 0x0f, 0x2d, 0xca, 0xf4]; // CVTSD2SI ECX, XMM2
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_edx_xmm3() {
    let code = [0xf2, 0x0f, 0x2d, 0xd3, 0xf4]; // CVTSD2SI EDX, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_esi_xmm7() {
    let code = [0xf2, 0x0f, 0x2d, 0xf7, 0xf4]; // CVTSD2SI ESI, XMM7
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtsd2si_rax_xmm0_64() {
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSD2SI RAX, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_rbx_xmm1_64() {
    let code = [0xf2, 0x48, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSD2SI RBX, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_r9_xmm8_64() {
    let code = [0xf2, 0x4d, 0x0f, 0x2d, 0xc8, 0xf4]; // CVTSD2SI R9, XMM8
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory Tests
#[test]
fn test_cvtsd2si_mem32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 42.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_mem64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1234567890.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Special Values
#[test]
fn test_cvtsd2si_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_negative_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -0.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Rounding Tests
#[test]
fn test_cvtsd2si_round_down() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 42.3;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_round_up() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 42.7;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_round_half_even() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 42.5;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_round_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -42.7;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Normal Values
#[test]
fn test_cvtsd2si_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_minus_one() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -1.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_positive_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1000000.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_negative_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -1000000.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Overflow Cases (should return indefinite integer)
#[test]
fn test_cvtsd2si_overflow_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 3.0e9; // Larger than i32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_overflow_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -3.0e9; // Less than i32::MIN
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_infinity_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_infinity_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::NEG_INFINITY;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = f64::NAN;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Edge Cases
#[test]
fn test_cvtsd2si_near_max_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 2147483000.0; // Near i32::MAX
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_near_min_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -2147483000.0; // Near i32::MIN
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_small_fractional() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.9;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_very_small() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 0.000001;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// 64-bit Edge Cases
#[test]
fn test_cvtsd2si_64bit_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 1000000000000.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_64bit_near_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x48, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 9.0e18; // Near i64 limits
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Misc
#[test]
fn test_cvtsd2si_100_point_99() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 100.99;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_minus_100_point_99() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = -100.99;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_displacement() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0x2d, 0x40, 0x10, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 777.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsd2si_multiple() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x2d, 0x00, // CVTSD2SI EAX, [RAX]
        0xf2, 0x0f, 0x2d, 0x08, // CVTSD2SI ECX, [RAX]
        0xf2, 0x0f, 0x2d, 0x10, // CVTSD2SI EDX, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let f1: f64 = 456.0;
    mem.write_slice(&f1.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
