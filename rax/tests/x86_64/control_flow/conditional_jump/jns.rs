use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// Strengthened: CMP 10,5 (10-5 positive => SF=0) => JNS taken; sentinel + RIP proof.
#[test]
fn test_jns_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (result 5, SF=0)
        0x79, 0x08, // JNS +8
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JNS taken");
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}

// JNS - Jump if Not Sign
// Jumps to target if SF = 0 (result is non-negative)

// Basic JNS with positive result
#[test]
fn test_jns_taken_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=0)
        0x79, 0x02, // JNS +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with zero (non-negative)
#[test]
fn test_jns_taken_zero() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=0)
        0x79, 0x02, // JNS +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with condition not met (negative)
#[test]
fn test_jns_not_taken_negative() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=1)
        0x79, 0x05, // JNS +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JNS forward jump
#[test]
fn test_jns_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x79, 0x07, // JNS +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX should remain 0x42");
}

// JNS backward jump
#[test]
fn test_jns_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // MOV RAX, -5
        // loop (offset 7):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x79, 0x02, // JNS +2 (exit when non-negative)
        0xeb, 0xf8, // JMP -8 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "RAX is 0");
}

// JNS preserves all registers
#[test]
fn test_jns_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x42, "RAX preserved");
    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
}

// JNS does not affect flags
#[test]
fn test_jns_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF=0)
        0x79, 0x02, // JNS +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x80 == 0, "SF should remain clear");
}

// JNS with zero offset
#[test]
fn test_jns_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x00, // JNS +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with maximum forward offset
#[test]
fn test_jns_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x7f, // JNS +127
    ];
    code.resize(12 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with maximum backward offset
#[test]
fn test_jns_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x80, // JNS -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with ADD (positive result)
#[test]
fn test_jns_after_add_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x03, // ADD RAX, 3 (result 8, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with CMP (positive difference)
#[test]
fn test_jns_after_cmp_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10-5=5, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS sign detection pattern
#[test]
fn test_jns_sign_detection() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5 (positive value)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x09, // JNS +9 (non-negative)
        // negative:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // non-negative:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Detected non-negative");
}

// JNS with 32-bit operands
#[test]
fn test_jns_32bit() {
    let code = [
        0xb8, 0x42, 0x00, 0x00, 0x00, // MOV EAX, 0x42
        0x85, 0xc0, // TEST EAX, EAX
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with 16-bit operands
#[test]
fn test_jns_16bit() {
    let code = [
        0x66, 0xb8, 0x42, 0x00, // MOV AX, 0x42
        0x66, 0x85, 0xc0, // TEST AX, AX
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with 8-bit operands
#[test]
fn test_jns_8bit() {
    let code = [
        0xb0, 0x42, // MOV AL, 0x42
        0x84, 0xc0, // TEST AL, AL
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS counting up from negative
#[test]
fn test_jns_count_to_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0xfc, 0xff, 0xff, 0xff, // MOV RAX, -4
        // loop (offset 7):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x79, 0x02, // JNS +2 (exit when non-negative)
        0xeb, 0xf8, // JMP -8 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0, "Counted to 0");
}

// JNS validation pattern
#[test]
fn test_jns_validation() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42 (valid value)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x09, // JNS +9 (valid)
        // invalid (negative):
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // valid (non-negative):
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Valid");
}

// JNS with INC
#[test]
fn test_jns_after_inc() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0xff, 0xc0, // INC RAX (result 0, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0);
}

// JNS with DEC (still positive)
#[test]
fn test_jns_after_dec_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JNS chaining
#[test]
fn test_jns_chaining() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x05, // JNS +5
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // jumped here:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JNS practical: success code checking
#[test]
fn test_jns_success_checking() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (success code)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x79, 0x09, // JNS +9 (success)
        // error (negative):
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // success (non-negative):
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Success detected");
}

// JNS with SHL (result with high bit clear)
#[test]
fn test_jns_after_shl() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xd1, 0xe0, // SHL RAX, 1 (result 2, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with AND resulting in positive
#[test]
fn test_jns_after_and_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0x25, 0x7f, 0x00, 0x00, 0x00, // AND RAX, 0x7F (clear high bit, SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS boundary testing
#[test]
fn test_jns_boundary() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFF (max positive)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JNS with OR (positive result)
#[test]
fn test_jns_after_or_positive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x09, 0xc0, // OR RAX, RAX (SF=0)
        0x79, 0x02, // JNS +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}
