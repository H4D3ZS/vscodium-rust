use crate::common::{VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm};

// JNP/JPO - Jump if Not Parity / Jump if Parity Odd
// Jumps when PF=0 (odd number of 1 bits in low byte of result)
// Opcode: 7B cb (JNP rel8), 0F 8B cw/cd (JNP rel16/32)

#[test]
fn test_jnp_taken_odd_parity() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (0b1, 1 bit, odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0 for odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
    assert_eq!(vm.executed_instructions, 4); // MOV, TEST, JNP, HLT (skipped 2 HLTs)
}

#[test]
fn test_jnp_not_taken_even_parity() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (0b11, 2 bits, even)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1 for even parity)
        0x7b, 0x02, // JNP +2 (should not jump)
        0xf4, // HLT (should execute)
        0xf4, // HLT (target, should not reach)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len() - 1) as u64);
    assert_eq!(vm.executed_instructions, 4); // MOV, TEST, JNP, HLT
}

#[test]
fn test_jpo_alias_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (0b111, 3 bits, odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x7b, 0x02, // JPO +2 (alias for JNP, should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
    assert_eq!(vm.executed_instructions, 4);
}

#[test]
fn test_jnp_forward_jump() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15 (0b1111, 4 bits, even)
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1 (result=14, 0b1110, 3 bits, odd, PF=0)
        0x7b, 0x05, // JNP +5 (should jump)
        0x48, 0xff, 0xc0, // INC RAX (should not execute)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 14);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
}

#[test]
fn test_jnp_backward_jump() {
    let code = [
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3 (counter)
        // loop_start (offset 7):
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x48, 0xff, 0xc9, // DEC RCX
        0x7b, 0xf1, // JNP -15 (loop back if odd parity - always true)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0);
    assert_eq!(vm.executed_instructions, 1 + 3 * 4 + 1); // initial MOV + 3*(MOV,TEST,DEC,JNP) + HLT
}

#[test]
fn test_jnp_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x7b, 0x7f, // JNP +127 (max positive offset)
    ];
    code.resize(12 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT at offset +127

    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
}

#[test]
fn test_jnp_max_backward_offset() {
    let mut code = vec![];
    code.resize(10, 0x90); // 10 NOPs
    code.push(0xf4); // HLT at offset 10
    code.resize(126, 0x90); // More NOPs to offset 126
    code.extend([
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (odd parity) at offset 126
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0) at offset 133
        0x7b,
        0x80, // JNP -128 (max negative offset) at offset 136, jumps to offset 138-128=10
    ]);

    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, 0x1000 + 11); // HLT at offset 10, RIP after = 11
}

#[test]
fn test_jnp_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x7b, 0x00, // JNP +0 (jump to next instruction)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64);
}

#[test]
fn test_jnp_with_add_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x83, 0xc0, 0x02, // ADD RAX, 2 (result=7, 0b111, 3 bits, odd, PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
    assert_eq!(vm.executed_instructions, 4);
}

#[test]
fn test_jnp_with_sub_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x83, 0xe8, 0x09, // SUB RAX, 9 (result=7, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_with_and_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0x83, 0xe0, 0x07, // AND RAX, 7 (result=7, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_with_or_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4 (0b100)
        0x48, 0x83, 0xc8, 0x01, // OR RAX, 1 (result=5, 0b101, 2 bits, even, PF=1)
        0x7b, 0x02, // JNP +2 (should not jump)
        0xf4, // HLT (should execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 5);
    assert_eq!(vm.rip, (0x1000 + code.len() - 1) as u64);
}

#[test]
fn test_jnp_with_xor_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15 (0b1111)
        0x48, 0x83, 0xf0, 0x08, // XOR RAX, 8 (result=7, 0b111, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_with_inc_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x06, 0x00, 0x00, 0x00, // MOV RAX, 6 (0b110, 2 bits)
        0x48, 0xff, 0xc0, // INC RAX (result=7, 0b111, 3 bits, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_with_dec_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x08, 0x00, 0x00, 0x00, // MOV RAX, 8 (0b1000, 1 bit)
        0x48, 0xff, 0xc8, // DEC RAX (result=7, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_with_cmp_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x83, 0xf8, 0x09, // CMP RAX, 9 (result=7, odd parity for flags)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 16); // CMP doesn't modify RAX
}

#[test]
fn test_jnp_with_neg_odd_result() {
    let code = [
        0x48, 0xc7, 0xc0, 0xf9, 0xff, 0xff, 0xff, // MOV RAX, -7
        0x48, 0xf7, 0xd8, // NEG RAX (result=7, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 7);
}

#[test]
fn test_jnp_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x43, 0x00, 0x00, 0x00, // MOV RBX, 0x43
        0x48, 0xc7, 0xc1, 0x44, 0x00, 0x00, 0x00, // MOV RCX, 0x44
        0x48, 0xc7, 0xc2, 0x45, 0x00, 0x00, 0x00, // MOV RDX, 0x45
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1);
    assert_eq!(vm.rbx, 0x43);
    assert_eq!(vm.rcx, 0x44);
    assert_eq!(vm.rdx, 0x45);
}

#[test]
fn test_jnp_preserves_other_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF=1, CF=1)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=0, clears CF, ZF)
        0x7b, 0x02, // JNP +2 (should jump, only checks PF)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1);
}

#[test]
fn test_jnp_with_8bit_operand_odd() {
    let code = [
        0xb0, 0x01, // MOV AL, 1 (odd parity)
        0x84, 0xc0, // TEST AL, AL (sets PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 1);
}

#[test]
fn test_jnp_with_16bit_operand_odd() {
    let code = [
        0x66, 0xb8, 0x01, 0x00, // MOV AX, 1 (odd parity in low byte)
        0x66, 0x85, 0xc0, // TEST AX, AX (sets PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFF, 1);
}

#[test]
fn test_jnp_with_32bit_operand_odd() {
    let code = [
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1 (odd parity in low byte)
        0x85, 0xc0, // TEST EAX, EAX (sets PF=0)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFFFFFF, 1);
}

#[test]
fn test_jnp_parity_checking_pattern() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA (10101010, 4 bits, even)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7b, 0x0a, // JNP +10 (should not jump - even parity)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (even marker)
        0xeb, 0x07, // JMP +7 (skip odd marker)
        // odd_parity:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (odd marker)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 1); // Even parity path taken
}

#[test]
fn test_jnp_error_detection_pattern() {
    // Check if data has odd parity (possible corruption if expecting even)
    let code = [
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55 (01010101, 4 bits, even)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7b, 0x0a, // JNP +10 (jump if odd parity - corruption detected)
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1 (data OK)
        0xeb, 0x07, // JMP +7
        // corrupted:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (data corrupted)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 1); // Data has even parity, OK
}

#[test]
fn test_jnp_serial_port_pattern() {
    // Serial communication often uses odd parity
    let code = [
        0x48, 0xc7, 0xc0, 0x41, 0x00, 0x00, 0x00, // MOV RAX, 'A' (0x41, 2 bits, even)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF=1)
        0x7b, 0x03, // JNP +3 (need odd parity)
        // add_parity_bit:
        0x0c, 0x80, // OR AL, 0x80 (set parity bit to make odd)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x41 | 0x80); // Parity bit added
}

#[test]
fn test_jnp_multiple_sequential_checks() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15 (odd)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (final)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 15);
}

#[test]
fn test_jnp_all_odd_parity_values() {
    // Test various single-bit values (all have odd parity)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x01 (1 bit)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x01, // JNP +1
        0xf4, // HLT (should not reach)
        0x48, 0xc7, 0xc0, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 0x02 (1 bit)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x01, // JNP +1
        0xf4, // HLT (should not reach)
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 0x04 (1 bit)
        0x48, 0x85, 0xc0, // TEST RAX, RAX
        0x7b, 0x01, // JNP +1
        0xf4, // HLT (should not reach)
        0xf4, // HLT (final)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x04);
}

#[test]
fn test_jnp_combined_with_other_conditions() {
    // JNP only checks PF, other flags don't matter
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (result=0, ZF=1, CF=1, PF=1)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (PF=0, but ZF and CF from ADD)
        0x48, 0x0b, 0xc0, // OR RAX, RAX (PF=0, clears CF/ZF)
        0x7b, 0x02, // JNP +2 (should jump - only PF matters)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 1);
}

#[test]
fn test_jnp_parity_only_low_byte() {
    // Parity is only calculated on low byte
    let code = [
        0x48, 0xb8, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFF...FF01
        0x48, 0x85, 0xc0, // TEST RAX, RAX (only low byte 0x01 matters, odd parity)
        0x7b, 0x02, // JNP +2 (should jump)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0xFFFFFFFFFFFFFF01);
}

#[test]
fn test_jnp_with_zero_even_parity() {
    // Zero has even parity (0 bits set)
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (result=0, even parity, PF=1)
        0x7b, 0x02, // JNP +2 (should not jump)
        0xf4, // HLT (should execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0);
    assert_eq!(vm.rip, (0x1000 + code.len() - 1) as u64);
}

#[test]
fn test_jnp_with_0xff_even_parity() {
    // 0xFF has even parity (8 bits set)
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0x00, 0x00, 0x00, // MOV RAX, 0xFF
        0x48, 0x85, 0xc0, // TEST RAX, RAX (even parity, PF=1)
        0x7b, 0x02, // JNP +2 (should not jump)
        0xf4, // HLT (should execute)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0xFF);
    assert_eq!(vm.rip, (0x1000 + code.len() - 1) as u64);
}
