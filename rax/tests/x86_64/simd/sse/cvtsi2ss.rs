use crate::common::*;

// CVTSI2SS - Convert Signed Integer to Scalar Single Precision
// Opcode: F3 0F 2A /r    CVTSI2SS xmm1, r/m32
//         F3 REX.W 0F 2A /r    CVTSI2SS xmm1, r/m64

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtsi2ss_xmm0_eax() {
    let code = [0xf3, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, EAX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm1_ebx() {
    let code = [0xf3, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SS XMM1, EBX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm2_ecx() {
    let code = [0xf3, 0x0f, 0x2a, 0xd1, 0xf4]; // CVTSI2SS XMM2, ECX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm3_edx() {
    let code = [0xf3, 0x0f, 0x2a, 0xda, 0xf4]; // CVTSI2SS XMM3, EDX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm7_esi() {
    let code = [0xf3, 0x0f, 0x2a, 0xfe, 0xf4]; // CVTSI2SS XMM7, ESI
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtsi2ss_xmm0_rax_64() {
    let code = [0xf3, 0x48, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, RAX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm1_rbx_64() {
    let code = [0xf3, 0x48, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SS XMM1, RBX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm8_r9_64() {
    let code = [0xf3, 0x4d, 0x0f, 0x2a, 0xc1, 0xf4]; // CVTSI2SS XMM8, R9
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// Memory Tests
#[test]
fn test_cvtsi2ss_mem32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 42;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_mem64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i64 = 1234567890;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Special Values
#[test]
fn test_cvtsi2ss_zero() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 0;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_max_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = i32::MAX;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_min_i32() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = i32::MIN;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_max_i64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i64 = i64::MAX;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_min_i64() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i64 = i64::MIN;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_positive_small() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 1;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_small() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = -1;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_positive_medium() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 1000000;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_medium() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = -1000000;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Powers of 2
#[test]
fn test_cvtsi2ss_power_of_2() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 1024;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_power_of_2() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = -1024;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Precision Loss Tests (large integers may lose precision in f32)
#[test]
fn test_cvtsi2ss_large_with_precision_loss() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 16777217; // Larger than 2^24, may lose precision
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_64bit_large() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i64 = 1000000000000;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Misc edge cases
#[test]
fn test_cvtsi2ss_100() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 100;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_minus_100() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = -100;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_12345() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 12345;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_minus_67890() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = -67890;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

// Addressing modes
#[test]
fn test_cvtsi2ss_displacement() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x40, 0x10, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 999;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_multiple() {
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x2a, 0x00, // CVTSI2SS XMM0, [RAX]
        0xf3, 0x0f, 0x2a, 0x08, // CVTSI2SS XMM1, [RAX]
        0xf3, 0x0f, 0x2a, 0x10, // CVTSI2SS XMM2, [RAX]
        0xf4,
    ]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 777;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm15() {
    let code = [0xf3, 0x44, 0x0f, 0x2a, 0xf8, 0xf4]; // CVTSI2SS XMM15, EAX
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_r8d() {
    let code = [0xf3, 0x41, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, R8D
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_boundary_8388608() {
    // 2^23 - exact boundary for f32 mantissa
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 8388608;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_cvtsi2ss_boundary_16777216() {
    // 2^24 - largest exactly representable integer
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    let (mut vcpu, mem) = setup_vm(&full_code, None);
    let val: i32 = 16777216;
    mem.write_slice(&val.to_le_bytes(), vm_memory::GuestAddress(DATA_ADDR))
        .unwrap();
    run_until_hlt(&mut vcpu).unwrap();
}
