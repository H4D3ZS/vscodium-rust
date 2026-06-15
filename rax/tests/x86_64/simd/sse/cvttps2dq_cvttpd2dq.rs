use crate::common::*;

// CVTTPS2DQ - Convert With Truncation Packed Single Precision to Packed Signed Doubleword Integers
// CVTTPD2DQ - Convert With Truncation Packed Double Precision to Packed Signed Doubleword Integers
// Opcode: F3 0F 5B /r         CVTTPS2DQ xmm1, xmm2/m128
//         66 0F E6 /r         CVTTPD2DQ xmm1, xmm2/m128

const DATA_ADDR: u64 = 0x3000;

// CVTTPS2DQ xmm, xmm - Truncate 4x float32 to 4x int32
#[test]
fn test_cvttps2dq_xmm0_xmm1() {
    let code = [0xf3, 0x0f, 0x5b, 0xc1, 0xf4]; // CVTTPS2DQ XMM0, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_xmm2_xmm3() {
    let code = [0xf3, 0x0f, 0x5b, 0xd3, 0xf4]; // CVTTPS2DQ XMM2, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_xmm7_xmm0() {
    let code = [0xf3, 0x0f, 0x5b, 0xf8, 0xf4]; // CVTTPS2DQ XMM7, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_xmm8_xmm9() {
    let code = [0xf3, 0x45, 0x0f, 0x5b, 0xc1, 0xf4]; // CVTTPS2DQ XMM8, XMM9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_xmm15_xmm14() {
    let code = [0xf3, 0x45, 0x0f, 0x5b, 0xfe, 0xf4]; // CVTTPS2DQ XMM15, XMM14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTTPS2DQ xmm, m128 - Truncate from memory
#[test]
fn test_cvttps2dq_xmm0_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test truncation toward zero
#[test]
fn test_cvttps2dq_truncate_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [1.9, 2.5, 3.1, 4.999];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_truncate_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [-1.9, -2.5, -3.1, -4.999];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_truncate_half() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All .5 values should truncate toward zero
    let vals: [f32; 4] = [2.5, 3.5, -2.5, -3.5];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_truncate_small_fractions() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [0.1, 0.9, -0.1, -0.9];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [0.0, -0.0, 0.0, 0.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_whole_numbers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [1.0, -1.0, 100.0, -100.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [1000000.9, -1000000.9, 8388608.7, -8388608.3];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test overflow to indefinite integer (0x80000000)
#[test]
fn test_cvttps2dq_overflow_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [3e9, 1e10, f32::MAX, 2147483648.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_overflow_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [-3e9, -1e10, f32::MIN, -2147483649.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_infinity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [f32::INFINITY, f32::NEG_INFINITY, 0.0, 1.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttps2dq_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [f32::NAN, 1.0, 2.0, 3.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTTPD2DQ xmm, xmm - Truncate 2x float64 to 2x int32
#[test]
fn test_cvttpd2dq_xmm0_xmm1() {
    let code = [0x66, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTTPD2DQ XMM0, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_xmm2_xmm3() {
    let code = [0x66, 0x0f, 0xe6, 0xd3, 0xf4]; // CVTTPD2DQ XMM2, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_xmm7_xmm0() {
    let code = [0x66, 0x0f, 0xe6, 0xf8, 0xf4]; // CVTTPD2DQ XMM7, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_xmm8_xmm9() {
    let code = [0x66, 0x45, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTTPD2DQ XMM8, XMM9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_xmm15_xmm14() {
    let code = [0x66, 0x45, 0x0f, 0xe6, 0xfe, 0xf4]; // CVTTPD2DQ XMM15, XMM14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTTPD2DQ xmm, m128 - Truncate from memory
#[test]
fn test_cvttpd2dq_xmm0_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [1.0, 2.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test truncation toward zero
#[test]
fn test_cvttpd2dq_truncate_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [1.9, 2.999];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_truncate_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [-1.9, -2.999];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_truncate_half() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // All .5 values should truncate toward zero
    let vals: [f64; 2] = [2.5, -2.5];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_truncate_small_fractions() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [0.1, -0.9];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [0.0, -0.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_whole_numbers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [100.0, -100.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [1000000.9, -1000000.9];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test overflow to indefinite integer
#[test]
fn test_cvttpd2dq_overflow_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [3e9, 1e10];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_overflow_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [-3e9, -1e10];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_infinity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [f64::INFINITY, f64::NEG_INFINITY];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [f64::NAN, 1.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with different XMM registers
#[test]
fn test_cvttps2dq_xmm10_xmm11() {
    let code = [0xf3, 0x45, 0x0f, 0x5b, 0xd3, 0xf4]; // CVTTPS2DQ XMM10, XMM11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_xmm12_xmm13() {
    let code = [0x66, 0x45, 0x0f, 0xe6, 0xe5, 0xf4]; // CVTTPD2DQ XMM12, XMM13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory with displacement
#[test]
fn test_cvttps2dq_mem_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x40, 0x10, 0xf4]); // CVTTPS2DQ XMM0, [RAX+16]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [123.5, 456.7, 789.1, 1011.9];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_mem_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x40, 0x10, 0xf4]); // CVTTPD2DQ XMM0, [RAX+16]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [123.5, 456.7];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test edge cases near boundaries
#[test]
fn test_cvttps2dq_near_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [2147483647.0, -2147483648.0, 2147483520.0, -2147483520.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_near_max() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [2147483647.0, -2147483648.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test various fractional parts
#[test]
fn test_cvttps2dq_fraction_0_25() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x5b, 0x00, 0xf4]); // CVTTPS2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f32; 4] = [0.25, 1.25, -0.25, -1.25];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvttpd2dq_fraction_0_75() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0xe6, 0x00, 0xf4]); // CVTTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [0.75, -0.75];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}
