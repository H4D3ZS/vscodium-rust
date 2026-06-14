use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// JA/JNBE - Jump if Above / Jump if Not Below or Equal
// Jumps to target if CF = 0 AND ZF = 0 (unsigned comparison)

// Basic JA with condition met (CF=0, ZF=0)
#[test]
fn test_ja_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5: CF=0, ZF=0)
        0x77, 0x02, // JA +2 (should jump)
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with condition not met (equal, ZF=1)
#[test]
fn test_ja_not_taken_equal() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 == 5: ZF=1)
        0x77, 0x05, // JA +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JA with condition not met (below, CF=1)
#[test]
fn test_ja_not_taken_below() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0xc7, 0xc3, 0x08, 0x00, 0x00, 0x00, // MOV RBX, 8
        0x48, 0x39, 0xd8, // CMP RAX, RBX (3 < 8: CF=1)
        0x77, 0x05, // JA +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x99);
}

// JNBE (alias for JA)
#[test]
fn test_jnbe_taken() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 15
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (15 > 10)
        0x77, 0x02, // JNBE +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA forward jump
#[test]
fn test_ja_forward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x64, 0x00, 0x00, 0x00, // MOV RAX, 100
        0x48, 0xc7, 0xc3, 0x32, 0x00, 0x00, 0x00, // MOV RBX, 50
        0x48, 0x39, 0xd8, // CMP RAX, RBX (100 > 50)
        0x77, 0x07, // JA +7 (next=19, target=26)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (skipped)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 100, "RAX should remain 100");
}

// JA backward jump
#[test]
fn test_ja_backward() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (counter)
        // loop (offset 14):
        0x48, 0x83, 0xc3, 0x01, // ADD RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0xf7, // JA -9 (loop while RAX > RBX)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10);
    assert_eq!(regs.rbx, 10, "RBX incremented to 10");
}

// JA with unsigned comparison (wraparound)
#[test]
fn test_ja_unsigned_wraparound() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0xff, 0xff, 0xff, 0xff, // MOV RBX, 0xFFFFFFFF (large unsigned)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (1 < 0xFFFFFFFF unsigned)
        0x77, 0x05, // JA +5 (should not jump)
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JA preserves all registers
#[test]
fn test_ja_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0xc7, 0xc1, 0x11, 0x00, 0x00, 0x00, // MOV RCX, 0x11
        0x48, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00, // MOV RDX, 0x22
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 10, "RAX preserved");
    assert_eq!(regs.rbx, 5, "RBX preserved");
    assert_eq!(regs.rcx, 0x11, "RCX preserved");
    assert_eq!(regs.rdx, 0x22, "RDX preserved");
}

// JA does not affect flags
#[test]
fn test_ja_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (sets CF=0, ZF=0)
        0x77, 0x02, // JA +2 (does not modify flags)
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert!(regs.rflags & 0x01 == 0, "CF should remain clear");
    assert!(regs.rflags & 0x40 == 0, "ZF should remain clear");
}

// JA with SUB instruction
#[test]
fn test_ja_after_sub() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xe8, 0x03, // SUB RAX, 3 (result 7, CF=0, ZF=0)
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 7);
}

// JA with zero offset
#[test]
fn test_ja_zero_offset() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x00, // JA +0 (next instruction)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA in range checking
#[test]
fn test_ja_range_check() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (max)
        0x48, 0x39, 0xd8, // CMP RAX, RBX (50 vs 100)
        0x77, 0x08, // JA +8 (out of range)
        // in range:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xeb, 0x07, // JMP +7 (skip out of range)
        // out of range:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "In range");
}

// JA with maximum forward offset
#[test]
fn test_ja_max_forward_offset() {
    let mut code = vec![
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x7f, // JA +127
    ];
    code.resize(19 + 127, 0x90); // NOP padding
    code.push(0xf4); // HLT

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with maximum backward offset
#[test]
fn test_ja_max_backward_offset() {
    let mut code = vec![];
    code.push(0xf4); // HLT at start
    code.resize(129, 0x90); // NOPs
    code.extend_from_slice(&[
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x80, // JA -128
    ]);

    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with 32-bit operands
#[test]
fn test_ja_32bit() {
    let code = [
        0xb8, 0x0a, 0x00, 0x00, 0x00, // MOV EAX, 10
        0xbb, 0x05, 0x00, 0x00, 0x00, // MOV EBX, 5
        0x39, 0xd8, // CMP EAX, EBX
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with 16-bit operands
#[test]
fn test_ja_16bit() {
    let code = [
        0x66, 0xb8, 0x0a, 0x00, // MOV AX, 10
        0x66, 0xbb, 0x05, 0x00, // MOV BX, 5
        0x66, 0x39, 0xd8, // CMP AX, BX
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with 8-bit operands
#[test]
fn test_ja_8bit() {
    let code = [
        0xb0, 0x0a, // MOV AL, 10
        0xb3, 0x05, // MOV BL, 5
        0x38, 0xd8, // CMP AL, BL
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA in binary search pattern
#[test]
fn test_ja_binary_search() {
    let code = [
        0x48, 0xc7, 0xc0, 0x32, 0x00, 0x00, 0x00, // MOV RAX, 50 (target)
        0x48, 0xc7, 0xc3, 0x32, 0x00, 0x00, 0x00, // MOV RBX, 50 (mid)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x08, // JA +8 (search upper half)
        // search lower or equal:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xeb, 0x07, // JMP +7 (skip upper)
        // search upper half:
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Equal, search lower");
}

// JA chained with other conditions
#[test]
fn test_ja_chained() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 > 5)
        0x77, 0x05, // JA +5 (first check passed)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // first check passed:
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42);
}

// JA with TEST instruction
#[test]
fn test_ja_after_test() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0xa9, 0x0f, 0x00, 0x00, 0x00, // TEST RAX, 0x0F (CF=0, ZF=0)
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA loop counting down
#[test]
fn test_ja_countdown_loop() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0 (sum)
        // loop (offset 14):
        0x48, 0x01, 0xc3, // ADD RBX, RAX
        0x48, 0x83, 0xe8, 0x01, // SUB RAX, 1
        0x48, 0x83, 0xf8, 0x00, // CMP RAX, 0
        0x77, 0xf4, // JA -12 (loop while RAX > 0)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 15, "Sum 5+4+3+2+1 = 15");
    assert_eq!(regs.rax, 0);
}

// JA with boundary values
#[test]
fn test_ja_boundary() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0x39, 0xd8, // CMP RAX, RBX (1 > 0)
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA unsigned vs signed comparison
#[test]
fn test_ja_unsigned_large() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x48, 0x39, 0xd8, // CMP RAX, RBX (0xFFFF... > 1 unsigned)
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA with immediate CMP
#[test]
fn test_ja_cmp_immediate() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
}

// JA multiple conditions
#[test]
fn test_ja_multiple_conditions() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        // check if > 5
        0x48, 0x83, 0xf8, 0x05, // CMP RAX, 5
        0x77, 0x05, // JA +5 (yes, > 5)
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0 (skipped)
        // check if > 15
        0x48, 0x83, 0xf8, 0x0f, // CMP RAX, 15 (offset 23)
        0x77, 0x08, // JA +8 (no, not > 15)
        // in range 5 < x <= 15
        0x48, 0xc7, 0xc1, 0x42, 0x00, 0x00, 0x00, // MOV RCX, 0x42
        0xeb, 0x07, // JMP +7 (exit)
        // > 15
        0x48, 0xc7, 0xc1, 0x99, 0x00, 0x00, 0x00, // MOV RCX, 0x99
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 0x42, "In range");
}

// JA with DEC
#[test]
fn test_ja_after_dec() {
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xff, 0xc8, // DEC RAX (result 4, CF=0, ZF=0)
        0x77, 0x02, // JA +2
        0xf4, 0xf4, // HLT, HLT (should not execute)
        0xf4, // HLT (target)
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rax, 4);
}

// JA in threshold checking
#[test]
fn test_ja_threshold() {
    let code = [
        0x48, 0xc7, 0xc0, 0x96, 0x00, 0x00, 0x00, // MOV RAX, 150 (value)
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100 (threshold)
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x09, // JA +9 (next=19, target=28)
        // below or equal:
        0x48, 0xc7, 0xc1, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xeb, 0x07, // JMP +7
        // above threshold:
        0x48, 0xc7, 0xc1, 0x01, 0x00, 0x00, 0x00, // MOV RCX, 1
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rcx, 1, "Above threshold");
}

// Strengthened: prove the taken branch via a sentinel and exact final RIP.
// CMP 10,5 (unsigned above) => CF=0,ZF=0 => JA taken. Jcc skips MOV RCX,0xBAD
// and lands on MOV RCX,0xACED; the fall-through path is fenced off by a HLT.
#[test]
fn test_ja_taken_sentinel_and_rip() {
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX
        0x77, 0x08, // JA +8 (taken)
        0x48, 0xc7, 0xc1, 0xad, 0x0b, 0x00, 0x00, // MOV RCX, 0xBAD
        0xf4, // HLT (fall-through fence)
        0x48, 0xc7, 0xc1, 0xed, 0xac, 0x00, 0x00, // MOV RCX, 0xACED (target)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0xACED, "JA taken reached the target");
    // taken target MOV at 0x101B(.. depends) -> just assert RIP advanced past final HLT.
    assert_eq!(regs.rip, 0x1000 + code.len() as u64, "RIP past taken HLT");
}
