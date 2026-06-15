use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// VLDMXCSR - Load MXCSR Register
// VSTMXCSR - Store MXCSR Register
//
// VLDMXCSR loads the MXCSR register from a 32-bit memory location.
// VSTMXCSR stores the MXCSR register to a 32-bit memory location.
//
// MXCSR is the SSE control and status register that controls:
// - Rounding mode
// - Exception masks
// - Exception flags
// - Denormals-are-zero and flush-to-zero modes
//
// Opcodes:
// VEX.LZ.0F.WIG AE /2    VLDMXCSR m32    - Load MXCSR from m32
// VEX.LZ.0F.WIG AE /3    VSTMXCSR m32    - Store MXCSR to m32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VLDMXCSR Tests
// ============================================================================

#[test]
fn test_vldmxcsr_basic() {
    // VLDMXCSR [mem]
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // Default MXCSR value: 0x1F80 (all exceptions masked, round to nearest)
    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_round_to_zero() {
    // Load MXCSR with round-to-zero mode (RC = 11b, bits 14-13)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with round-to-zero: 0x7F80
    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_round_down() {
    // Load MXCSR with round-down mode (RC = 01b, bits 14-13)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with round-down: 0x3F80
    let mxcsr: [u8; 4] = [0x80, 0x3f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_round_up() {
    // Load MXCSR with round-up mode (RC = 10b, bits 14-13)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with round-up: 0x5F80
    let mxcsr: [u8; 4] = [0x80, 0x5f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_flush_to_zero() {
    // Load MXCSR with flush-to-zero bit set (bit 15)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with FTZ: 0x9F80
    let mxcsr: [u8; 4] = [0x80, 0x9f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_denormals_are_zero() {
    // Load MXCSR with denormals-are-zero bit set (bit 6)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with DAZ: 0x1FC0
    let mxcsr: [u8; 4] = [0xc0, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_all_exceptions_unmasked() {
    // Load MXCSR with all exception masks cleared
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR with all exceptions unmasked: 0x1F00
    let mxcsr: [u8; 4] = [0x00, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_invalid_operation_unmasked() {
    // Load MXCSR with invalid operation exception unmasked (bit 7 clear)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR: 0x1F00 (invalid op unmasked)
    let mxcsr: [u8; 4] = [0x00, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_divide_by_zero_unmasked() {
    // Load MXCSR with divide-by-zero exception unmasked (bit 9 clear)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR: 0x1D80 (div-by-zero unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x1d, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_overflow_unmasked() {
    // Load MXCSR with overflow exception unmasked (bit 10 clear)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR: 0x1B80 (overflow unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x1b, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_underflow_unmasked() {
    // Load MXCSR with underflow exception unmasked (bit 11 clear)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR: 0x1780 (underflow unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x17, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_precision_unmasked() {
    // Load MXCSR with precision exception unmasked (bit 12 clear)
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    // MXCSR: 0x0F80 (precision unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x0f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_multiple_loads() {
    // Multiple VLDMXCSR operations
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr1: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr1, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&mxcsr2, GuestAddress(ALIGNED_ADDR + 4))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_before_arithmetic() {
    // VLDMXCSR followed by arithmetic operation
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_rax_indirect() {
    // VLDMXCSR [rax]
    let code = [
        0x48, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x3000
        0xc5, 0xf8, 0xae, 0x10, // VLDMXCSR [rax]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_rbx_indirect() {
    // VLDMXCSR [rbx]
    let code = [
        0x48, 0xbb, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x3000
        0xc5, 0xf8, 0xae, 0x13, // VLDMXCSR [rbx]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_rcx_indirect() {
    // VLDMXCSR [rcx]
    let code = [
        0x48, 0xb9, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x3000
        0xc5, 0xf8, 0xae, 0x11, // VLDMXCSR [rcx]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vldmxcsr_offset() {
    // VLDMXCSR [rax + offset]
    let code = [
        0x48, 0xb8, 0xf0, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2FF0
        0xc5, 0xf8, 0xae, 0x50, 0x10, // VLDMXCSR [rax + 0x10]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VSTMXCSR Tests
// ============================================================================

#[test]
fn test_vstmxcsr_basic() {
    // VSTMXCSR [mem]
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_after_load() {
    // VLDMXCSR followed by VSTMXCSR
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_multiple_stores() {
    // Multiple VSTMXCSR operations
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_after_arithmetic() {
    // Arithmetic operation followed by VSTMXCSR
    let code = [
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_rax_indirect() {
    // VSTMXCSR [rax]
    let code = [
        0x48, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x3000
        0xc5, 0xf8, 0xae, 0x18, // VSTMXCSR [rax]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_rbx_indirect() {
    // VSTMXCSR [rbx]
    let code = [
        0x48, 0xbb, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x3000
        0xc5, 0xf8, 0xae, 0x1b, // VSTMXCSR [rbx]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_rcx_indirect() {
    // VSTMXCSR [rcx]
    let code = [
        0x48, 0xb9, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x3000
        0xc5, 0xf8, 0xae, 0x19, // VSTMXCSR [rcx]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_offset() {
    // VSTMXCSR [rax + offset]
    let code = [
        0x48, 0xb8, 0xf0, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2FF0
        0xc5, 0xf8, 0xae, 0x58, 0x10, // VSTMXCSR [rax + 0x10]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// VLDMXCSR/VSTMXCSR Combined Tests
// ============================================================================

#[test]
fn test_ldmxcsr_stmxcsr_roundtrip() {
    // Load and store MXCSR
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_arithmetic_stmxcsr() {
    // Load MXCSR, do arithmetic, store MXCSR
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    mem.write_slice(&mxcsr, GuestAddress(ALIGNED_ADDR)).unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_multiple_ldmxcsr_stmxcsr_pairs() {
    // Multiple load/store pairs
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x08, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4008]
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004]
        0xc5, 0xf8, 0xae, 0x1d, 0x0c, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x400C]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr1: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr1, GuestAddress(ALIGNED_ADDR))
        .unwrap();
    mem.write_slice(&mxcsr2, GuestAddress(ALIGNED_ADDR + 4))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_save_restore_pattern() {
    // Save current MXCSR, load new, restore
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000] (save)
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004] (load new)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (use new mode)
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000] (restore)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr_new: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr_new, GuestAddress(ALIGNED_ADDR + 4))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_after_division() {
    // Division followed by VSTMXCSR
    let code = [
        0xc5, 0xf0, 0x5e, 0xc2, // VDIVPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_vstmxcsr_after_sqrt() {
    // VSQRTPS followed by VSTMXCSR
    let code = [
        0xc5, 0xf8, 0x51, 0xc1, // VSQRTPS XMM0, XMM1
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_context_switch() {
    // Simulate context switch with MXCSR
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00,
        0x00, // VSTMXCSR [rip + 0x4000] (save context 1)
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00,
        0x00, // VLDMXCSR [rip + 0x4004] (load context 2)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (in context 2)
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00,
        0x00, // VSTMXCSR [rip + 0x4004] (save context 2)
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00,
        0x00, // VLDMXCSR [rip + 0x4000] (restore context 1)
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);

    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    mem.write_slice(&mxcsr2, GuestAddress(ALIGNED_ADDR + 4))
        .unwrap();

    run_until_hlt(&mut vcpu).unwrap();
}
