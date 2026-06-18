use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 5,10 (5-10 negative => SF=1) => JS taken; sentinel + RIP proof.
#[test]
fn test_js_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (result -5, SF=1)
        0x78, 0x08, // JS +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JS taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JS - Jump if Sign
// Jumps to target if SF = 1 (result is negative)

// Basic JS with negative result
#[test]
fn test_js_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1)
        0x78, 0x02, // JS +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with condition not met (positive)
#[test]
fn test_js_not_taken_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=0)
        0x78, 0x05, // JS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JS with SUB resulting in negative
#[test]
fn test_js_after_sub_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xe8, 0x0a, // SUB RAX, 10 (result -5, sets SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS forward jump
#[test]
fn test_js_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=1)
        0x78, 0x07, // JS +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax as i32, -5, "RAX should remain -5");
}

// JS backward jump
#[test]
fn test_js_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        // loop (offset 7):
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1
        0x78, 0x02, // JS +2 (exit when negative)
        0xeb, 0xf8, // JMP -8 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF, "RAX is -1");
}

// JS preserves all registers
#[test]
fn test_js_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
}

// JS does not affect flags
#[test]
fn test_js_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1)
        0x78, 0x02, // JS +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x80 != 0, "SF should remain set");
}

// JS with zero offset
#[test]
fn test_js_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x00, // JS +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with maximum forward offset
#[test]
fn test_js_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x7f, // JS +127
    ];
    code.resize(12 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with maximum backward offset
#[test]
fn test_js_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x80, // JS -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with NEG instruction
#[test]
fn test_js_after_neg() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xf7, 0xd8, // NEG RAX (result -5, sets SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with CMP (negative result conceptually)
#[test]
fn test_js_after_cmp() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5-10=-5, sets SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS sign detection pattern
#[test]
fn test_js_sign_detection() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5 (signed value)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x09, // JS +9 (negative)
        // positive:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // negative:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Detected negative");
}

// JS with zero (not negative)
#[test]
fn test_js_zero_not_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x78, 0x05, // JS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JS with 32-bit operands
#[test]
fn test_js_32bit() {
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, -1
        0x85, 0xc0, // TEST EAX, EAX
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with 16-bit operands
#[test]
fn test_js_16bit() {
    let code = [
        0x66, 0xb8, 0xff, 0xff, // MOV AX, -1
        0x66, 0x85, 0xc0, // TEST AX, AX
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with 8-bit operands
#[test]
fn test_js_8bit() {
    let code = [
        0xb0, 0xff, // MOV AL, -1
        0x84, 0xc0, // TEST AL, AL
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS absolute value pattern
#[test]
fn test_js_absolute_value() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x03, // JS +3 (is negative)
        0xeb, 0x03, // JMP +3 (skip negate)
        // negate:
        0x48, 0xf7, 0xd8, // NEG RAX (make positive)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 5, "Absolute value is 5");
}

// JS with SHL (shift doesn't set SF based on result sign)
#[test]
fn test_js_after_shl() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, // MOV RAX, bit 62 set
        0x48, 0xd1, 0xe0, // SHL RAX, 1 (bit 63 set, SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with ADD overflow
#[test]
fn test_js_after_add_overflow() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x7f, // MOV RAX, 0x7F00000000000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x7f, // MOV RBX, 0x7F00000000000000
        0x48, 0x01, 0xd8, // ADD RAX, RBX (overflow to negative)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS chaining
#[test]
fn test_js_chaining() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x07, // JS +7
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // jumped here:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JS practical: error code checking
#[test]
fn test_js_error_checking() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (error code)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x78, 0x09, // JS +9 (error)
        // success:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // error:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Error detected");
}

// JS with DEC (sets SF based on result)
#[test]
fn test_js_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xff, 0xc8, // DEC RAX (result -1, sets SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JS with INC (positive result)
#[test]
fn test_js_after_inc_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc0, // INC RAX (result 6, SF=0)
        0x78, 0x05, // JS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JS high bit set
#[test]
fn test_js_high_bit_set() {
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, // MOV RAX, 0x8000000000000000
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=1)
        0x78, 0x02, // JS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}
