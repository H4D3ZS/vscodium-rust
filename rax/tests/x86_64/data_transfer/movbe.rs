// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOVBE - Move Data After Swapping Bytes
// Performs a byte swap operation on data copied from source to destination.
// Provides support for converting little-endian values to big-endian format and vice versa.
//
// Opcodes:
// 0F 38 F0 /r             MOVBE r16, m16    - Reverse byte order in m16 and move to r16
// 0F 38 F0 /r             MOVBE r32, m32    - Reverse byte order in m32 and move to r32
// REX.W + 0F 38 F0 /r     MOVBE r64, m64    - Reverse byte order in m64 and move to r64
// 0F 38 F1 /r             MOVBE m16, r16    - Reverse byte order in r16 and move to m16
// 0F 38 F1 /r             MOVBE m32, r32    - Reverse byte order in r32 and move to m32
// REX.W + 0F 38 F1 /r     MOVBE m64, r64    - Reverse byte order in r64 and move to m64

// ===== 16-bit MOVBE Tests =====

#[test]
fn test_movbe_r16_m16_basic() {
    // MOVBE AX, [DATA_ADDR] - load 16-bit with byte swap (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25, // 66H prefix + MOVBE AX, [DATA_ADDR]
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFF,
        0x3412,
        "AX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r16_m16_all_zeros() {
    // MOVBE AX, [DATA_ADDR] - with zeros (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0x0000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x0000, "AX should be zero");
}

#[test]
fn test_movbe_r16_m16_all_ones() {
    // MOVBE AX, [DATA_ADDR] - with all ones (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0xFFFF, "AX should be all ones");
}

#[test]
fn test_movbe_r16_m16_pattern() {
    // MOVBE AX, [DATA_ADDR] - with pattern (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u16(&mem, 0xAA55);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x55AA, "AX should be byte-swapped");
}

#[test]
fn test_movbe_m16_r16_basic() {
    // MOVBE [DATA_ADDR], AX - store 16-bit with byte swap (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u16(&mem),
        0x3412,
        "Memory should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_m16_r16_pattern() {
    // MOVBE [DATA_ADDR], AX - store pattern (requires 66H prefix)
    let code = [
        0x66,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAA55;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u16(&mem),
        0x55AA,
        "Memory should contain byte-swapped pattern"
    );
}

// ===== 32-bit MOVBE Tests =====

#[test]
fn test_movbe_r32_m32_basic() {
    // MOVBE EAX, [DATA_ADDR] - load 32-bit with byte swap
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x78563412,
        "EAX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r32_m32_all_zeros() {
    // MOVBE EAX, [DATA_ADDR] - with zeros
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0x00000000, "EAX should be zero");
}

#[test]
fn test_movbe_r32_m32_all_ones() {
    // MOVBE EAX, [DATA_ADDR] - with all ones
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xFFFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFFFFFF, 0xFFFFFFFF, "EAX should be all ones");
}

#[test]
fn test_movbe_r32_m32_pattern() {
    // MOVBE EAX, [DATA_ADDR] - with pattern
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xAABBCCDD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xDDCCBBAA,
        "EAX should be byte-swapped"
    );
}

#[test]
fn test_movbe_r32_m32_endian_conversion() {
    // MOVBE EAX, [DATA_ADDR] - endianness conversion
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x00000100); // Big-endian 256
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00010000,
        "EAX should convert endianness"
    );
}

#[test]
fn test_movbe_m32_r32_basic() {
    // MOVBE [DATA_ADDR], EAX - store 32-bit with byte swap
    let code = [
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x78563412,
        "Memory should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_m32_r32_pattern() {
    // MOVBE [DATA_ADDR], EAX - store pattern
    let code = [
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xAABBCCDD;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0xDDCCBBAA,
        "Memory should contain byte-swapped pattern"
    );
}

#[test]
fn test_movbe_m32_r32_endian_conversion() {
    // MOVBE [DATA_ADDR], EAX - endianness conversion
    let code = [
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x00010000; // Little-endian 256
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u32(&mem),
        0x00000100,
        "Memory should contain big-endian value"
    );
}

// ===== 64-bit MOVBE Tests =====

#[test]
fn test_movbe_r64_m64_basic() {
    // MOVBE RAX, [DATA_ADDR] - load 64-bit with byte swap
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xEFCDAB8967452301,
        "RAX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r64_m64_all_zeros() {
    // MOVBE RAX, [DATA_ADDR] - with zeros
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0000000000000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0000000000000000, "RAX should be zero");
}

#[test]
fn test_movbe_r64_m64_all_ones() {
    // MOVBE RAX, [DATA_ADDR] - with all ones
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xFFFFFFFFFFFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX should be all ones");
}

#[test]
fn test_movbe_r64_m64_pattern() {
    // MOVBE RAX, [DATA_ADDR] - with pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xFEDCBA9876543210);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x1032547698BADCFE, "RAX should be byte-swapped");
}

#[test]
fn test_movbe_r64_m64_asymmetric() {
    // MOVBE RAX, [DATA_ADDR] - asymmetric pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0102030405060708);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0807060504030201, "RAX should be byte-swapped");
}

#[test]
fn test_movbe_m64_r64_basic() {
    // MOVBE [DATA_ADDR], RAX - store 64-bit with byte swap
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0xEFCDAB8967452301,
        "Memory should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_m64_r64_pattern() {
    // MOVBE [DATA_ADDR], RAX - store pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFEDCBA9876543210;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x1032547698BADCFE,
        "Memory should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_m64_r64_asymmetric() {
    // MOVBE [DATA_ADDR], RAX - asymmetric pattern
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0102030405060708;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        read_mem_u64(&mem),
        0x0807060504030201,
        "Memory should contain byte-swapped value"
    );
}

// ===== Extended Register Tests =====

#[test]
fn test_movbe_r32_m32_ebx() {
    // MOVBE EBX, [DATA_ADDR] - test with EBX
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x1c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x11223344);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx & 0xFFFFFFFF,
        0x44332211,
        "EBX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r32_m32_ecx() {
    // MOVBE ECX, [DATA_ADDR] - test with ECX
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x0c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xAABBCCDD);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rcx & 0xFFFFFFFF,
        0xDDCCBBAA,
        "ECX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r32_m32_r8d() {
    // MOVBE R8D, [DATA_ADDR] - test with extended register
    let code = [
        0x44,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8 & 0xFFFFFFFF,
        0xEFBEADDE,
        "R8D should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r64_m64_rbx() {
    // MOVBE RBX, [DATA_ADDR] - test with RBX
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x1c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0011223344556677);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rbx, 0x7766554433221100,
        "RBX should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r64_m64_r8() {
    // MOVBE R8, [DATA_ADDR] - test with extended register
    let code = [
        0x4c,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xFEDCBA9876543210);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r8, 0x1032547698BADCFE,
        "R8 should contain byte-swapped value"
    );
}

#[test]
fn test_movbe_r64_m64_r15() {
    // MOVBE R15, [DATA_ADDR] - test with R15
    let code = [
        0x4c,
        0x0f,
        0x38,
        0xf0,
        0x3c,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x0123456789ABCDEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.r15, 0xEFCDAB8967452301,
        "R15 should contain byte-swapped value"
    );
}

// ===== Roundtrip Tests =====

#[test]
fn test_movbe_roundtrip_16bit() {
    // Test that store followed by load gives original value
    let code = [
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25, // MOVBE [DATA_ADDR], AX
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25, // MOVBE AX, [DATA_ADDR]
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax & 0xFFFF, 0x1234, "Roundtrip should preserve value");
}

#[test]
fn test_movbe_roundtrip_32bit() {
    // Test that store followed by load gives original value
    let code = [
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25, // MOVBE [DATA_ADDR], EAX
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25, // MOVBE EAX, [DATA_ADDR]
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "Roundtrip should preserve value"
    );
}

#[test]
fn test_movbe_roundtrip_64bit() {
    // Test that store followed by load gives original value
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf1,
        0x04,
        0x25, // MOVBE [DATA_ADDR], RAX
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25, // MOVBE RAX, [DATA_ADDR]
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x0123456789ABCDEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0123456789ABCDEF,
        "Roundtrip should preserve value"
    );
}

// ===== Special Value Tests =====

#[test]
fn test_movbe_max_signed_32bit() {
    // Test with maximum signed 32-bit value
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x7FFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0xFFFFFF7F,
        "Max signed 32-bit should be byte-swapped"
    );
}

#[test]
fn test_movbe_min_signed_32bit() {
    // Test with minimum signed 32-bit value
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0x80000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00000080,
        "Min signed 32-bit should be byte-swapped"
    );
}

#[test]
fn test_movbe_max_signed_64bit() {
    // Test with maximum signed 64-bit value
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x7FFFFFFFFFFFFFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0xFFFFFFFFFFFFFF7F,
        "Max signed 64-bit should be byte-swapped"
    );
}

#[test]
fn test_movbe_min_signed_64bit() {
    // Test with minimum signed 64-bit value
    let code = [
        0x48,
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0x8000000000000000);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(
        regs.rax, 0x0000000000000080,
        "Min signed 64-bit should be byte-swapped"
    );
}

#[test]
fn test_movbe_preserves_other_registers() {
    // MOVBE should not affect other registers
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1111111111111111;
    regs.rcx = 0x2222222222222222;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x1111111111111111, "RBX should be unchanged");
    assert_eq!(regs.rcx, 0x2222222222222222, "RCX should be unchanged");
}

#[test]
fn test_movbe_does_not_modify_flags() {
    // MOVBE does not modify any flags
    let code = [
        0x0f,
        0x38,
        0xf0,
        0x04,
        0x25,
        (DATA_ADDR & 0xFF) as u8,
        ((DATA_ADDR >> 8) & 0xFF) as u8,
        ((DATA_ADDR >> 16) & 0xFF) as u8,
        ((DATA_ADDR >> 24) & 0xFF) as u8,
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // Only reserved bit 1
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u32(&mem, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rflags, initial_flags, "Flags should not be modified");
}

// ============================================================================
// Strengthened MOVBE tests (appended): exact byte-swapped values for load
// (0F 38 F0) and store (0F 38 F1) at 16/32/64-bit operand sizes.
// ============================================================================

#[test]
fn test_strict_movbe_load_r32_exact() {
    // MOVBE EAX, [RBX]: memory 0x12345678 -> EAX 0x78563412, upper RAX cleared.
    let code = [0x0f, 0x38, 0xf0, 0x03, 0xf4]; // MOVBE EAX, [RBX]
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u32(&mem, DATA_ADDR, 0x12345678);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000_7856_3412,
        "MOVBE load byte-swaps and clears upper RAX"
    );
}

#[test]
fn test_strict_movbe_load_r64_exact() {
    // MOVBE RAX, [RBX]: 0x0102030405060708 -> 0x0807060504030201.
    let code = [0x48, 0x0f, 0x38, 0xf0, 0x03, 0xf4]; // MOVBE RAX, [RBX]
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR, 0x0102_0304_0506_0708);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0807_0605_0403_0201,
        "MOVBE RAX load byte-swaps 8 bytes"
    );
}

#[test]
fn test_strict_movbe_load_r16_preserves_upper() {
    // MOVBE AX, [RBX] (0x66 operand size): 16-bit swap, upper 48 bits preserved.
    let code = [0x66, 0x0f, 0x38, 0xf0, 0x03, 0xf4]; // MOVBE AX, [RBX]
    let mut regs = Registers::default();
    regs.rax = 0xAAAA_BBBB_CCCC_DDDD;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u16(&mem, DATA_ADDR, 0x1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xAAAA_BBBB_CCCC_3412,
        "MOVBE AX 16-bit byte-swap, upper preserved"
    );
}

#[test]
fn test_strict_movbe_store_r32_exact() {
    // MOVBE [RBX], EAX (0F 38 F1): EAX 0xAABBCCDD stored as DD CC BB AA.
    let code = [0x0f, 0x38, 0xf1, 0x03, 0xf4]; // MOVBE [RBX], EAX
    let mut regs = Registers::default();
    regs.rax = 0xAABB_CCDD;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u32(&mem, DATA_ADDR),
        0xDDCC_BBAA,
        "MOVBE store byte-swaps to memory"
    );
}

#[test]
fn test_strict_movbe_store_r64_exact() {
    // MOVBE [RBX], RAX (REX.W 0F 38 F1).
    let code = [0x48, 0x0f, 0x38, 0xf1, 0x03, 0xf4]; // MOVBE [RBX], RAX
    let mut regs = Registers::default();
    regs.rax = 0x1122_3344_5566_7788;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, DATA_ADDR),
        0x8877_6655_4433_2211,
        "MOVBE RAX store byte-swaps"
    );
}
