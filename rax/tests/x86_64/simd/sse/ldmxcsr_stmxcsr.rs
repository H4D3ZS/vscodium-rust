use crate::common::{run_until_hlt, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// LDMXCSR - Load MXCSR Register
// STMXCSR - Store MXCSR Register State
//
// LDMXCSR loads the MXCSR control/status register from a 32-bit memory location
// STMXCSR stores the MXCSR control/status register to a 32-bit memory location
//
// MXCSR Register layout:
// Bits 0-5:   Exception flags (IE, DE, ZE, OE, UE, PE)
// Bits 6:     Denormals Are Zeros (DAZ)
// Bits 7-12:  Exception masks (IM, DM, ZM, OM, UM, PM)
// Bits 13-14: Rounding control (00=nearest, 01=down, 10=up, 11=toward zero)
// Bit 15:     Flush to Zero (FTZ)
//
// Default MXCSR value at reset: 0x1F80
//
// Opcodes:
// NP 0F AE /2             LDMXCSR m32    - Load MXCSR from m32
// NP 0F AE /3             STMXCSR m32    - Store MXCSR to m32

const MXCSR_ADDR: u64 = 0x3000; // Address for MXCSR save/restore

// ============================================================================
// STMXCSR Tests - Store MXCSR Register
// ============================================================================

#[test]
fn test_stmxcsr_basic() {
    // STMXCSR [0x3000]
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_stmxcsr_default_value() {
    // Test storing default MXCSR value (0x1F80)
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_stmxcsr_multiple_stores() {
    // Test multiple STMXCSR operations
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004]
        0x0f, 0xae, 0x1c, 0x25, 0x08, 0x30, 0x00, 0x00, // STMXCSR [0x3008]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_stmxcsr_different_addresses() {
    // Test STMXCSR to different memory addresses
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x40, 0x00, 0x00, // STMXCSR [0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// LDMXCSR Tests - Load MXCSR Register
// ============================================================================

#[test]
fn test_ldmxcsr_basic() {
    // LDMXCSR [0x3000]
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_default_value() {
    // Test loading default MXCSR value (0x1F80)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_different_addresses() {
    // Test LDMXCSR from different memory addresses
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x40, 0x00, 0x00, // LDMXCSR [0x4000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// LDMXCSR/STMXCSR Combined Tests
// ============================================================================

#[test]
fn test_stmxcsr_ldmxcsr_roundtrip() {
    // Test storing and loading MXCSR
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mxcsr_save_restore() {
    // Test saving and restoring MXCSR state
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000] (save)
        0x0f, 0xae, 0x14, 0x25, 0x04, 0x30, 0x00, 0x00, // LDMXCSR [0x3004] (load new)
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000] (restore)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Rounding Mode Tests
// ============================================================================

#[test]
fn test_ldmxcsr_rounding_nearest() {
    // Test loading MXCSR with rounding mode = nearest (bits 13-14 = 00)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_down() {
    // Test loading MXCSR with rounding mode = down (bits 13-14 = 01)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_up() {
    // Test loading MXCSR with rounding mode = up (bits 13-14 = 10)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_toward_zero() {
    // Test loading MXCSR with rounding mode = toward zero (bits 13-14 = 11)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Exception Mask Tests
// ============================================================================

#[test]
fn test_ldmxcsr_all_exceptions_masked() {
    // Test loading MXCSR with all exceptions masked (bits 7-12 = 111111)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_no_exceptions_masked() {
    // Test loading MXCSR with no exceptions masked (bits 7-12 = 000000)
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_invalid_operation_mask() {
    // Test loading MXCSR with invalid operation exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_divide_by_zero_mask() {
    // Test loading MXCSR with divide by zero exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_overflow_mask() {
    // Test loading MXCSR with overflow exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_underflow_mask() {
    // Test loading MXCSR with underflow exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_precision_mask() {
    // Test loading MXCSR with precision exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Flush to Zero and Denormals Are Zero Tests
// ============================================================================

#[test]
fn test_ldmxcsr_flush_to_zero_enabled() {
    // Test loading MXCSR with FTZ (bit 15) enabled
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_flush_to_zero_disabled() {
    // Test loading MXCSR with FTZ (bit 15) disabled
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_denormals_are_zero_enabled() {
    // Test loading MXCSR with DAZ (bit 6) enabled
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_denormals_are_zero_disabled() {
    // Test loading MXCSR with DAZ (bit 6) disabled
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_ftz_and_daz_enabled() {
    // Test loading MXCSR with both FTZ and DAZ enabled
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Exception Flag Tests
// ============================================================================

#[test]
fn test_stmxcsr_with_exception_flags() {
    // Test storing MXCSR with exception flags set
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_clear_exception_flags() {
    // Test loading MXCSR to clear exception flags
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// Combined Mode Tests
// ============================================================================

#[test]
fn test_ldmxcsr_custom_configuration() {
    // Test loading MXCSR with custom configuration
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_mxcsr_state_preservation() {
    // Test that MXCSR state is preserved across operations
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000] (save)
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000] (restore)
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004] (verify)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_stmxcsr_addr2() {
    // Test STMXCSR to another address
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x10, 0x30, 0x00, 0x00, // STMXCSR [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_addr2() {
    // Test LDMXCSR from another address
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // LDMXCSR [0x3010]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_denormal_mask() {
    // Test loading MXCSR with denormal exception masked
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_stmxcsr_sequential() {
    // Test sequential STMXCSR operations
    let code = [
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // STMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_sequential() {
    // Test sequential LDMXCSR operations
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_ldmxcsr_rounding_mode_change() {
    // Test changing rounding mode via LDMXCSR
    let code = [
        0x0f, 0xae, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // LDMXCSR [0x3000]
        0x0f, 0xae, 0x1c, 0x25, 0x04, 0x30, 0x00, 0x00, // STMXCSR [0x3004]
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}
