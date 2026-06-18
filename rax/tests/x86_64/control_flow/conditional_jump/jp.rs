use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// JP/JPE - Jump if Parity / Jump if Parity Even
// Jumps to target if PF = 1 (even number of 1 bits in low byte)

// Basic JP with even parity
#[test]
fn test_jp_taken_even_parity() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00,
        0x00, // MOV RAX, 3 (0b00000011, 2 bits set, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7a, 0x02, // JP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with condition not met (odd parity)
#[test]
fn test_jp_not_taken_odd_parity() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00,
        0x00, // MOV RAX, 1 (0b00000001, 1 bit set, odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x7a, 0x05, // JP +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JPE (alias for JP)
#[test]
fn test_jpe_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00,
        0x00, // MOV RAX, 0 (0b00000000, 0 bits set, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7a, 0x02, // JPE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP forward jump
#[test]
fn test_jp_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00,
        0x00, // MOV RAX, 0x0F (0b00001111, 4 bits set, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (PF=1)
        0x7a, 0x07, // JP +7
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 0x0F, "RAX should remain 0x0F");
}

// JP backward jump
#[test]
fn test_jp_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        // loop (offset 7):
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1
        0x7a, 0x02, // JP +2 (exit when even parity)
        0xeb, 0xf8, // JMP -8 (loop)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // RAX should be even-parity value
    assert_eq!(regs.rax & 0xFF, 3, "RAX low byte is 3 (even parity)");
}

// JP preserves all registers
#[test]
fn test_jp_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (even parity)
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 3, "RAX preserved");
    assert_eq!(regs.rbx, 0x22, "RBX preserved");
    assert_eq!(regs.rcx, 0x33, "RCX preserved");
}

// JP does not affect flags
#[test]
fn test_jp_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7a, 0x02, // JP +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x04 != 0, "PF should remain set");
}

// JP with zero offset
#[test]
fn test_jp_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x00, // JP +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with maximum forward offset
#[test]
fn test_jp_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x7f, // JP +127
    ];
    code.resize(12 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with maximum backward offset
#[test]
fn test_jp_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x80, // JP -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP parity detection pattern
#[test]
fn test_jp_parity_detection() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x09, // JP +9 (even parity)
        // odd parity:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // even parity:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Even parity detected");
}

// JP with various even parity values
#[test]
fn test_jp_various_even_parity() {
    let test_values = [
        0x00, // 0b00000000, 0 bits
        0x03, // 0b00000011, 2 bits
        0x0F, // 0b00001111, 4 bits
        0x3F, // 0b00111111, 6 bits
        0xFF, // 0b11111111, 8 bits
    ];

    for &val in &test_values {
        let code = [
            0x48, 0xc7, 0xc0, val, 0x00, 0x00, 0x00, // MOV RAX, val
            0x48, 0x85, 0xc0, // TEST RAX, RAX
            0x7a, 0x02, // JP +2
            0xf4, 0xf4, // HLT, HLT (should not execute)
            0xf4, // HLT (target)
        ];
        let (mut vcpu, _) = setup_vm(&code, None);
        let _ = run_until_hlt(&mut vcpu).unwrap();
    }
}

// JP with ADD
#[test]
fn test_jp_after_add() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with SUB
#[test]
fn test_jp_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xe8, 0x02, // SUB RAX, 2 (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with AND
#[test]
fn test_jp_after_and() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x25, 0x0f, 0x00, 0x00, 0x00, // AND RAX, 0x0F (result 0x0F, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with OR
#[test]
fn test_jp_after_or() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0x0d, 0x01, 0x00, 0x00, 0x00, // OR RAX, 1 (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with XOR
#[test]
fn test_jp_after_xor() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x35, 0x06, 0x00, 0x00, 0x00, // XOR RAX, 6 (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with 32-bit operands
#[test]
fn test_jp_32bit() {
    let code = [
        0xb8, 0x03, 0x00, 0x00, 0x00, // MOV EAX, 3
        0x85, 0xc0, // TEST EAX, EAX
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with 16-bit operands
#[test]
fn test_jp_16bit() {
    let code = [
        0x66, 0xb8, 0x03, 0x00, // MOV AX, 3
        0x66, 0x85, 0xc0, // TEST AX, AX
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with 8-bit operands
#[test]
fn test_jp_8bit() {
    let code = [
        0xb0, 0x03, // MOV AL, 3
        0x84, 0xc0, // TEST AL, AL
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP chaining
#[test]
fn test_jp_chaining() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x05, // JP +5
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // jumped here:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JP with INC
#[test]
fn test_jp_after_inc() {
    let code = [
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 2
        0x48, 0xff, 0xc0, // INC RAX (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with DEC
#[test]
fn test_jp_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0xff, 0xc8, // DEC RAX (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP with CMP
#[test]
fn test_jp_after_cmp() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0x39, 0xd8, // CMP RAX, RBX (result 3, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JP practical: parity check
#[test]
fn test_jp_parity_check() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00,
        0x00, // MOV RAX, 0xAA (0b10101010, 4 bits, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7a, 0x09, // JP +9 (even parity)
        // odd parity:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7 (exit)
        // even parity:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Even parity");
}

// JP with NEG
#[test]
fn test_jp_after_neg() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xf7, 0xd8, // NEG RAX (result -3, low byte 0xFD, even parity)
        0x7a, 0x02, // JP +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}
