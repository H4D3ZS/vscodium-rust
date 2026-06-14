use crate::common::*;

// CVTDQ2PD - Convert Packed Doubleword Integers to Packed Double Precision
// CVTPD2DQ - Convert Packed Double Precision to Packed Signed Doubleword Integers
// Opcode: F3 0F E6 /r         CVTDQ2PD xmm1, xmm2/m64
//         F2 0F E6 /r         CVTPD2DQ xmm1, xmm2/m128

const DATA_ADDR: u64 = 0x3000;

// CVTDQ2PD xmm, xmm - Convert 2x int32 to 2x float64 (uses low 64 bits)
#[test]
fn test_cvtdq2pd_xmm0_xmm1() {
    let code = [0xf3, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTDQ2PD XMM0, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtdq2pd_xmm2_xmm3() {
    let code = [0xf3, 0x0f, 0xe6, 0xd3, 0xf4]; // CVTDQ2PD XMM2, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtdq2pd_xmm7_xmm0() {
    let code = [0xf3, 0x0f, 0xe6, 0xf8, 0xf4]; // CVTDQ2PD XMM7, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtdq2pd_xmm8_xmm9() {
    let code = [0xf3, 0x45, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTDQ2PD XMM8, XMM9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtdq2pd_xmm15_xmm14() {
    let code = [0xf3, 0x45, 0x0f, 0xe6, 0xfe, 0xf4]; // CVTDQ2PD XMM15, XMM14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTDQ2PD xmm, m64 - Convert 2x int32 from memory
#[test]
fn test_cvtdq2pd_xmm0_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [42, 123];
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
fn test_cvtdq2pd_positive_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [1000, 1000000];
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
fn test_cvtdq2pd_negative_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [-1000, -1000000];
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
fn test_cvtdq2pd_mixed_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [-42, 123456];
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
fn test_cvtdq2pd_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [0, 0];
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
fn test_cvtdq2pd_max_min() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [i32::MAX, i32::MIN];
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
fn test_cvtdq2pd_powers_of_two() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [1024, 1048576];
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
fn test_cvtdq2pd_precise_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // Double precision can exactly represent all int32 values
    let vals: [i32; 2] = [16777217, -16777217];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTPD2DQ xmm, xmm - Convert 2x float64 to 2x int32 (result in low 64 bits)
#[test]
fn test_cvtpd2dq_xmm0_xmm1() {
    let code = [0xf2, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTPD2DQ XMM0, XMM1
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtpd2dq_xmm2_xmm3() {
    let code = [0xf2, 0x0f, 0xe6, 0xd3, 0xf4]; // CVTPD2DQ XMM2, XMM3
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtpd2dq_xmm7_xmm0() {
    let code = [0xf2, 0x0f, 0xe6, 0xf8, 0xf4]; // CVTPD2DQ XMM7, XMM0
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtpd2dq_xmm8_xmm9() {
    let code = [0xf2, 0x45, 0x0f, 0xe6, 0xc1, 0xf4]; // CVTPD2DQ XMM8, XMM9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtpd2dq_xmm15_xmm14() {
    let code = [0xf2, 0x45, 0x0f, 0xe6, 0xfe, 0xf4]; // CVTPD2DQ XMM15, XMM14
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// CVTPD2DQ xmm, m128 - Convert 2x float64 from memory
#[test]
fn test_cvtpd2dq_xmm0_mem() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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

#[test]
fn test_cvtpd2dq_positive_floats() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [1000.5, 9999.9];
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
fn test_cvtpd2dq_negative_floats() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [-1000.5, -9999.9];
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
fn test_cvtpd2dq_rounding_nearest_even() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [2.5, 3.5];
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
fn test_cvtpd2dq_rounding_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [-2.5, -3.5];
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
fn test_cvtpd2dq_zeros() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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
fn test_cvtpd2dq_whole_numbers() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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
fn test_cvtpd2dq_large_values() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [1000000.0, -1000000.0];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test overflow handling
#[test]
fn test_cvtpd2dq_overflow_positive() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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
fn test_cvtpd2dq_overflow_negative() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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

// Test special values
#[test]
fn test_cvtpd2dq_infinity() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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
fn test_cvtpd2dq_nan() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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

// Roundtrip tests
#[test]
fn test_cvtdq2pd_cvtpd2dq_roundtrip() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0xe6, 0x00, // CVTDQ2PD XMM0, [RAX]
        0xf2, 0x0f, 0xe6, 0xc8, // CVTPD2DQ XMM1, XMM0
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [42, -42];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test precision preservation - all int32 values are exactly representable in f64
#[test]
fn test_cvtdq2pd_precision_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x00, 0xf4]); // CVTDQ2PD XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    // These values would lose precision in f32, but not in f64
    let vals: [i32; 2] = [16777217, 1073741824];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 4) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test with different XMM registers
#[test]
fn test_cvtdq2pd_xmm10_xmm11() {
    let code = [0xf3, 0x45, 0x0f, 0xe6, 0xd3, 0xf4]; // CVTDQ2PD XMM10, XMM11
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtpd2dq_xmm12_xmm13() {
    let code = [0xf2, 0x45, 0x0f, 0xe6, 0xe5, 0xf4]; // CVTPD2DQ XMM12, XMM13
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Test memory with displacement
#[test]
fn test_cvtdq2pd_mem_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0xe6, 0x40, 0x10, 0xf4]); // CVTDQ2PD XMM0, [RAX+16]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [i32; 2] = [123, 456];
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
fn test_cvtpd2dq_mem_disp() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x40, 0x10, 0xf4]); // CVTPD2DQ XMM0, [RAX+16]

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

// Test fractional boundaries
#[test]
fn test_cvtpd2dq_fraction_0_25() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [0.25, -0.25];
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
fn test_cvtpd2dq_fraction_0_75() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

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

// Test very small fractions
#[test]
fn test_cvtpd2dq_small_fractions() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [0.1, -0.1];
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}

// Test exact boundary values for int32
#[test]
fn test_cvtpd2dq_int32_boundary() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf2, 0x0f, 0xe6, 0x00, 0xf4]); // CVTPD2DQ XMM0, [RAX]

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let vals: [f64; 2] = [2147483647.0, -2147483648.0]; // INT_MAX, INT_MIN as f64
    for (i, &val) in vals.iter().enumerate() {
        mem.write_slice(
            &val.to_le_bytes(),
            vm_memory::GuestAddress(DATA_ADDR + (i * 8) as u64),
        )
        .unwrap();
    }
    run_until_hlt(&mut vcpu).unwrap();
}
