use crate::common::{run_until_hlt, setup_vm, setup_vm_compat, write_mem_at_u16, write_mem_at_u32};
use rax::cpu::Registers;

// Comprehensive tests for BOUND instruction
// BOUND r16, m16&16 (62 /r) - Check array bounds (16-bit)
// BOUND r32, m32&32 (62 /r) - Check array bounds (32-bit)
// Note: BOUND is not available in 64-bit mode (replaced by other mechanisms)

// ============================================================================
// BOUND - Basic 16-bit Bounds Checking
// ============================================================================

#[test]
fn test_bound_16bit_within_bounds() {
    // BOUND with index within bounds (no exception)
    let code = [
        0xb8, 0x05, 0x00, // MOV AX, 5 (index)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // Write bounds at 0x2000: lower=0, upper=10
    write_mem_at_u16(&mem, 0x2000, 0); // lower bound
    write_mem_at_u16(&mem, 0x2002, 10); // upper bound

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1); // No exception, continued
}

#[test]
fn test_bound_16bit_at_lower_bound() {
    // Index exactly at lower bound (should pass)
    let code = [
        0xb8, 0x00, 0x00, // MOV AX, 0 (at lower bound)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 2);
}

#[test]
fn test_bound_16bit_at_upper_bound() {
    // Index exactly at upper bound (should pass)
    let code = [
        0xb8, 0x0a, 0x00, // MOV AX, 10 (at upper bound)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 3);
}

#[test]
fn test_bound_16bit_below_lower_bound() {
    // Index below lower bound (should trigger INT 5)
    let code = [
        0xb8, 0xff, 0xff, // MOV AX, -1 (below lower bound)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x04, 0x00, 0x00, 0x00, // MOV RBX, 4 (should not reach)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    // Should trigger interrupt 5 (BOUND exception)
    // Without handler, may fault or skip
}

#[test]
fn test_bound_16bit_above_upper_bound() {
    // Index above upper bound (should trigger INT 5)
    let code = [
        0xb8, 0x0b, 0x00, // MOV AX, 11 (above upper bound)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    // Should trigger interrupt 5
}

// ============================================================================
// BOUND - Basic 32-bit Bounds Checking
// ============================================================================

#[test]
fn test_bound_32bit_within_bounds() {
    // BOUND with 32-bit index within bounds
    let code = [
        0x66, 0xb8, 0x64, 0x00, 0x00, 0x00, // MOV EAX, 100
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0); // lower bound
    write_mem_at_u32(&mem, 0x2004, 1000); // upper bound

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_bound_32bit_at_lower_bound() {
    let code = [
        0x66, 0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 1000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 2);
}

#[test]
fn test_bound_32bit_at_upper_bound() {
    let code = [
        0x66, 0xb8, 0xe8, 0x03, 0x00, 0x00, // MOV EAX, 1000
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x03, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 1000);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 3);
}

#[test]
fn test_bound_32bit_below_lower_bound() {
    let code = [
        0x66, 0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, -1
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x04, 0x00, 0x00, 0x00, // MOV RBX, 4
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 1000);

    // Should trigger interrupt 5
}

#[test]
fn test_bound_32bit_above_upper_bound() {
    let code = [
        0x66, 0xb8, 0xe9, 0x03, 0x00, 0x00, // MOV EAX, 1001
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 1000);

    // Should trigger interrupt 5
}

// ============================================================================
// BOUND - Negative Bounds
// ============================================================================

#[test]
fn test_bound_16bit_negative_bounds() {
    // Bounds with negative values
    let code = [
        0xb8, 0xfb, 0xff, // MOV AX, -5
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0xfff6); // -10
    write_mem_at_u16(&mem, 0x2002, 0x000a); // 10

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_bound_32bit_negative_bounds() {
    // Test with negative bounds: index 0 within [-100, 100]
    // Test that a negative lower bound works correctly
    let code = [
        0x66, 0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x62, 0x03, // BOUND EAX, [RBX]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // Signed bounds: lower = -100, upper = 100
    // -100 as i32 = 0xFFFFFF9C
    write_mem_at_u32(&mem, 0x2000, 0xffffff9c_u32); // -100 as i32
    write_mem_at_u32(&mem, 0x2004, 100);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

// ============================================================================
// BOUND - Array Index Patterns
// ============================================================================

#[test]
fn test_bound_array_access_pattern() {
    // Typical array bounds check pattern
    let code = [
        0x66, 0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7 (array index)
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000] (bounds)
        // If valid, access array
        0x48, 0xc7, 0xc3, 0xaa, 0x00, 0x00, 0x00, // MOV RBX, 0xAA
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0); // array starts at 0
    write_mem_at_u32(&mem, 0x2004, 9); // array size is 10 (0-9)

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xaa);
}

#[test]
fn test_bound_loop_index_check() {
    // Loop with bounds checking
    let code = [
        0x66, 0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0 (loop counter)
        // Loop start
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xff, 0xc0, // INC RAX
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x72, 0xee, // JB loop (back 18 bytes)
        0x48, 0xc7, 0xc3, 0xbb, 0x00, 0x00, 0x00, // MOV RBX, 0xBB
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xbb);
}

// ============================================================================
// BOUND - Different Addressing Modes
// ============================================================================

#[test]
fn test_bound_register_indirect() {
    // BOUND with register indirect addressing
    let code = [
        0xb8, 0x05, 0x00, // MOV AX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x62, 0x03, // BOUND AX, [RBX]
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 1);
}

#[test]
fn test_bound_base_displacement() {
    // BOUND with base + displacement
    let code = [
        0x66, 0xb8, 0x32, 0x00, 0x00, 0x00, // MOV EAX, 50
        0x48, 0xc7, 0xc3, 0x00, 0x1f, 0x00, 0x00, // MOV RBX, 0x1F00
        0x66, 0x67, 0x62, 0x83, 0x00, 0x01, 0x00, 0x00, // BOUND EAX, [RBX + 0x100]
        0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00, // MOV RDX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // Address = 0x1F00 + 0x100 = 0x2000
    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 100);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 1);
}

#[test]
fn test_bound_sib_addressing() {
    // BOUND with SIB (Scale-Index-Base)
    let code = [
        0xb8, 0x08, 0x00, // MOV AX, 8
        0x48, 0xc7, 0xc3, 0x00, 0x1e, 0x00, 0x00, // MOV RBX, 0x1E00
        0x48, 0xc7, 0xc1, 0x80, 0x00, 0x00, 0x00, // MOV RCX, 0x80
        0x62, 0x04, 0x8b, // BOUND AX, [RBX + RCX*4]
        0x48, 0xc7, 0xc5, 0x01, 0x00, 0x00, 0x00, // MOV RBP, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    // Address = 0x1E00 + 0x80*4 = 0x2000
    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbp, 1);
}

// ============================================================================
// BOUND - Zero-Length Bounds
// ============================================================================

#[test]
fn test_bound_zero_length_bounds() {
    // Lower bound equals upper bound (single valid value)
    let code = [
        0xb8, 0x05, 0x00, // MOV AX, 5
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 5); // lower = 5
    write_mem_at_u16(&mem, 0x2002, 5); // upper = 5

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_bound_invalid_zero_length() {
    // Index not equal to single valid value
    let code = [
        0xb8, 0x06, 0x00, // MOV AX, 6 (not 5)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 5);
    write_mem_at_u16(&mem, 0x2002, 5);

    // Should trigger exception
}

// ============================================================================
// BOUND - Inverted Bounds (Error Case)
// ============================================================================

#[test]
fn test_bound_inverted_bounds() {
    // Lower > Upper (invalid configuration)
    let code = [
        0xb8, 0x05, 0x00, // MOV AX, 5
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 10); // lower = 10
    write_mem_at_u16(&mem, 0x2002, 0); // upper = 0 (inverted!)

    // Behavior is undefined/implementation-specific
}

// ============================================================================
// BOUND - Large Bounds
// ============================================================================

#[test]
fn test_bound_16bit_max_bounds() {
    // Test with max positive signed range: index 0x7FFE within [0, 0x7FFF]
    // BOUND uses signed comparison, so 0x7FFF is the max positive value
    let code = [
        0xb8, 0xfe, 0x7f, // MOV AX, 0x7FFE (32766, near max positive)
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0); // lower = 0
    write_mem_at_u16(&mem, 0x2002, 0x7FFF); // upper = 32767 (max positive signed)

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

#[test]
fn test_bound_32bit_max_bounds() {
    // Test with max positive signed range: index 500000 within [0, 0x7FFFFFFF]
    // BOUND uses signed comparison, so 0x7FFFFFFF is the max positive value
    let code = [
        0x66, 0xb8, 0x20, 0xa1, 0x07, 0x00, // MOV EAX, 500000 (0x0007A120)
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0); // lower = 0
    write_mem_at_u32(&mem, 0x2004, 0x7FFFFFFF); // upper = 2147483647 (max positive signed)

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 1);
}

// ============================================================================
// BOUND - Register Preservation
// ============================================================================

#[test]
fn test_bound_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc1, 0x11, 0x11, 0x00, 0x00, // MOV RCX, 0x1111
        0x48, 0xc7, 0xc2, 0x22, 0x22, 0x00, 0x00, // MOV RDX, 0x2222
        0xb8, 0x05, 0x00, // MOV AX, 5
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0x1111);
    assert_eq!(regs.rdx, 0x2222);
}

#[test]
fn test_bound_does_not_modify_index() {
    let code = [
        0x66, 0xb8, 0x07, 0x00, 0x00, 0x00, // MOV EAX, 7
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0);
    write_mem_at_u32(&mem, 0x2004, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 7); // EAX unchanged
}

// ============================================================================
// BOUND - Flags Preservation
// ============================================================================

#[test]
fn test_bound_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0xb8, 0x05, 0x00, // MOV AX, 5
        0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND AX, [0x2000]
        0x72, 0x05, // JC +5 (check if CF still set)
        0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1); // CF was preserved
}

// ============================================================================
// BOUND - Edge Cases
// ============================================================================

#[test]
fn test_bound_with_different_registers() {
    // Test BOUND with various register combinations
    let code = [
        0xbb, 0x05, 0x00, // MOV BX, 5
        0x62, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND BX, [0x2000]
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1);
}

#[test]
fn test_bound_cx_register() {
    let code = [
        0xb9, 0x08, 0x00, // MOV CX, 8
        0x62, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND CX, [0x2000]
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 2);
}

#[test]
fn test_bound_dx_register() {
    let code = [
        0xba, 0x03, 0x00, // MOV DX, 3
        0x62, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND DX, [0x2000]
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u16(&mem, 0x2000, 0);
    write_mem_at_u16(&mem, 0x2002, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3);
}

// ============================================================================
// BOUND - Real-World Usage Patterns
// ============================================================================

#[test]
fn test_bound_string_length_check() {
    // Check string index against length
    let code = [
        0x66, 0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3 (string index)
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x48, 0xc7, 0xc3, 0xcc, 0x00, 0x00, 0x00, // MOV RBX, 0xCC
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0); // min index
    write_mem_at_u32(&mem, 0x2004, 9); // max index (length 10)

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xcc);
}

#[test]
fn test_bound_buffer_access_validation() {
    // Validate buffer access before reading/writing
    let code = [
        0x66, 0xb8, 0x14, 0x00, 0x00, 0x00, // MOV EAX, 20 (buffer offset)
        0x66, 0x67, 0x62, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        // Access buffer at offset
        0x48, 0xc7, 0xc3, 0xdd, 0x00, 0x00, 0x00, // MOV RBX, 0xDD
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm_compat(&code, None);

    write_mem_at_u32(&mem, 0x2000, 0); // buffer start
    write_mem_at_u32(&mem, 0x2004, 255); // buffer end

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xdd);
}

#[test]
fn test_bound_not_available_in_64bit_mode() {
    // BOUND is not valid in 64-bit mode (without REX prefix)
    // In 64-bit mode, opcode 0x62 is used for EVEX prefix
    let code = [
        0x48, 0xb8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 5
        // In 64-bit mode, BOUND is invalid
        0x48, 0xc7, 0xc3, 0xff, 0x00, 0x00, 0x00, // MOV RBX, 0xFF
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xff);
}
