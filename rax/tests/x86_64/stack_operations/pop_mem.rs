use crate::common::{DATA_ADDR, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u64};
use rax::cpu::Registers;

// Comprehensive tests for POP with memory operands
//
// POP m64 - Pop quadword from stack into memory
// Various addressing modes:
// - Register indirect: POP [reg]
// - Register + displacement: POP [reg + disp]
// - Base + index: POP [base + index]
// - Base + index + displacement: POP [base + index + disp]

// ============================================================================
// POP with register indirect addressing [reg]
// ============================================================================

#[test]
fn test_pop_mem_indirect_rax() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "RSP balanced");
    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x42, "Value popped to [RAX]");
}

#[test]
fn test_pop_mem_indirect_rbx() {
    let code = [
        0x68, 0x78, 0x56, 0x34, 0x12, // PUSH 0x12345678
        0x8f, 0x03, // POP [RBX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x12345678, "Value popped to [RBX]");
}

#[test]
fn test_pop_mem_indirect_rcx() {
    let code = [
        0x6a, 0xff, // PUSH -1
        0x8f, 0x01, // POP [RCX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rcx = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "Value popped to [RCX]");
}

#[test]
fn test_pop_mem_indirect_r8() {
    let code = [
        0x6a, 0x11, // PUSH 0x11
        0x41, 0x8f, 0x00, // POP [R8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r8 = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x11, "Value popped to [R8]");
}

// ============================================================================
// POP with displacement [reg + disp8]
// ============================================================================

#[test]
fn test_pop_mem_disp8_positive() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x8f, 0x40, 0x08, // POP [RAX + 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 8);
    assert_eq!(val, 0x42, "Value popped to [RAX+8]");
}

#[test]
fn test_pop_mem_disp8_negative() {
    let code = [
        0x6a, 0x99, // PUSH 0x99
        0x8f, 0x40, 0xf8, // POP [RAX - 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR + 16;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 8);
    // PUSH imm8 sign-extends: 0x99 (bit 7 set) -> 0xFFFFFFFFFFFFFF99
    assert_eq!(val, 0xFFFFFFFFFFFFFF99, "Value popped to [RAX-8]");
}

#[test]
fn test_pop_mem_disp8_zero() {
    let code = [
        0x6a, 0x55, // PUSH 0x55
        0x8f, 0x40, 0x00, // POP [RAX + 0]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x55, "Value popped to [RAX+0]");
}

// ============================================================================
// POP with 32-bit displacement [reg + disp32]
// ============================================================================

#[test]
fn test_pop_mem_disp32_large() {
    let code = [
        0x68, 0x34, 0x12, 0x00, 0x00, // PUSH 0x1234
        0x8f, 0x80, 0x00, 0x10, 0x00, 0x00, // POP [RAX + 0x1000]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x1000);
    assert_eq!(val, 0x1234, "Value popped to [RAX+0x1000]");
}

#[test]
fn test_pop_mem_disp32_small() {
    let code = [
        0x6a, 0x77, // PUSH 0x77
        0x8f, 0x80, 0x10, 0x00, 0x00, 0x00, // POP [RAX + 0x10]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x10);
    assert_eq!(val, 0x77, "Value popped to [RAX+0x10]");
}

// ============================================================================
// POP with SIB addressing [base + index]
// ============================================================================

#[test]
fn test_pop_mem_sib_base_index() {
    let code = [
        0x6a, 0x88, // PUSH 0x88
        0x8f, 0x04, 0x18, // POP [RAX + RBX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x100;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x100);
    // PUSH imm8 sign-extends: 0x88 (bit 7 set) -> 0xFFFFFFFFFFFFFF88
    assert_eq!(val, 0xFFFFFFFFFFFFFF88, "Value popped to [RAX+RBX]");
}

#[test]
fn test_pop_mem_sib_base_index_scale2() {
    let code = [
        0x6a, 0x22, // PUSH 0x22
        0x8f, 0x04, 0x58, // POP [RAX + RBX*2]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x20);
    assert_eq!(val, 0x22, "Value popped to [RAX+RBX*2]");
}

#[test]
fn test_pop_mem_sib_base_index_scale4() {
    let code = [
        0x6a, 0x33, // PUSH 0x33
        0x8f, 0x04, 0x98, // POP [RAX + RBX*4]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x08;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x20);
    assert_eq!(val, 0x33, "Value popped to [RAX+RBX*4]");
}

#[test]
fn test_pop_mem_sib_base_index_scale8() {
    let code = [
        0x6a, 0x44, // PUSH 0x44
        0x8f, 0x04, 0xd8, // POP [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x04;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x20);
    assert_eq!(val, 0x44, "Value popped to [RAX+RBX*8]");
}

// ============================================================================
// POP with SIB + displacement
// ============================================================================

#[test]
fn test_pop_mem_sib_disp8() {
    let code = [
        0x6a, 0x66, // PUSH 0x66
        0x8f, 0x44, 0x18, 0x10, // POP [RAX + RBX + 0x10]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x20;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x30);
    assert_eq!(val, 0x66, "Value popped to [RAX+RBX+0x10]");
}

#[test]
fn test_pop_mem_sib_scale_disp8() {
    let code = [
        0x68, 0xcd, 0xab, 0x00, 0x00, // PUSH 0xABCD
        0x8f, 0x44, 0x58, 0x08, // POP [RAX + RBX*2 + 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x28);
    assert_eq!(val, 0xABCD, "Value popped to [RAX+RBX*2+8]");
}

#[test]
fn test_pop_mem_sib_disp32() {
    let code = [
        0x68, 0xdc, 0xfe, 0x00, 0x00, // PUSH 0xFEDC
        0x8f, 0x84, 0x18, 0x00, 0x01, 0x00, 0x00, // POP [RAX + RBX + 0x100]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x50;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x150);
    assert_eq!(val, 0xFEDC, "Value popped to [RAX+RBX+0x100]");
}

// ============================================================================
// POP multiple values to memory
// ============================================================================

#[test]
fn test_pop_mem_sequence() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0x8f, 0x00, // POP [RAX]
        0x8f, 0x03, // POP [RBX]
        0x8f, 0x01, // POP [RCX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = DATA_ADDR + 0x10;
    regs.rcx = DATA_ADDR + 0x20;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR), 3, "First pop (top)");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 0x10), 2, "Second pop");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 0x20), 1, "Third pop");
}

#[test]
fn test_pop_mem_to_array() {
    let code = [
        0x6a, 0x0a, // PUSH 10
        0x6a, 0x14, // PUSH 20
        0x6a, 0x1e, // PUSH 30
        0x8f, 0x00, // POP [RAX] (array[0])
        0x8f, 0x40, 0x08, // POP [RAX + 8] (array[1])
        0x8f, 0x40, 0x10, // POP [RAX + 16] (array[2])
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR), 30, "array[0]");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 8), 20, "array[1]");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 16), 10, "array[2]");
}

// ============================================================================
// POP memory preserves registers and flags
// ============================================================================

#[test]
fn test_pop_mem_preserves_registers() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0xc7, 0xc1, 0x88, 0x00, 0x00, 0x00, // MOV RCX, 0x88
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x99, "RBX unchanged");
    assert_eq!(regs.rcx, 0x88, "RCX unchanged");
}

#[test]
fn test_pop_mem_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0x6a, 0x42, // PUSH 0x42
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rflags & 0x01, 0, "CF preserved");
}

// ============================================================================
// POP to stack memory
// ============================================================================

#[test]
fn test_pop_mem_to_stack() {
    // This test pops a value to the same stack location it was pushed from.
    // After PUSH 0x42, RSP=0x0FF8, [0x0FF8]=0x42
    // After MOV RAX, RSP, RAX=0x0FF8
    // After POP [RAX], reads from [RSP=0x0FF8]=0x42, writes to [RAX=0x0FF8]=0x42
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x48, 0x89, 0xe0, // MOV RAX, RSP
        0x8f, 0x00, // POP [RAX] (pop to stack area where RAX points)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Check that 0x42 was written to the stack area
    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x42, "Value popped to stack memory");
}

// ============================================================================
// POP with different base registers
// ============================================================================

#[test]
fn test_pop_mem_base_rsi() {
    let code = [
        0x6a, 0x78, // PUSH 0x78
        0x8f, 0x06, // POP [RSI]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rsi = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x78, "Value popped to [RSI]");
}

#[test]
fn test_pop_mem_base_rdi() {
    let code = [
        0x68, 0x88, 0x77, 0x66, 0x55, // PUSH 0x55667788
        0x8f, 0x07, // POP [RDI]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rdi = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x55667788, "Value popped to [RDI]");
}

#[test]
fn test_pop_mem_base_rbp() {
    let code = [
        0x6a, 0x5a, // PUSH 0x5A
        0x8f, 0x45, 0x00, // POP [RBP + 0]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0x5A, "Value popped to [RBP]");
}

// ============================================================================
// POP memory roundtrip tests
// ============================================================================

#[test]
fn test_push_mem_pop_mem_roundtrip() {
    let code = [
        0xff, 0x30, // PUSH [RAX] (read from DATA_ADDR)
        0x8f, 0x03, // POP [RBX] (write to DATA_ADDR+0x100)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = DATA_ADDR + 0x100;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1234567890ABCDEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x100);
    assert_eq!(val, 0x1234567890ABCDEF, "Value copied via stack");
}

// ============================================================================
// POP memory edge cases
// ============================================================================

#[test]
fn test_pop_mem_zero_value() {
    let code = [
        0x6a, 0x00, // PUSH 0
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Pre-fill with non-zero
    write_mem_at_u64(&vm, DATA_ADDR, 0xFFFFFFFFFFFFFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0, "Zero value popped");
}

#[test]
fn test_pop_mem_all_ones() {
    let code = [
        0x6a, 0xff, // PUSH -1
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "All ones popped");
}

#[test]
fn test_pop_mem_high_address() {
    let code = [
        0x6a, 0xef, // PUSH 0xEF
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0x100000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x100000);
    // PUSH imm8 sign-extends: 0xEF (bit 7 set) -> 0xFFFFFFFFFFFFFFEF
    assert_eq!(val, 0xFFFFFFFFFFFFFFEF, "Value popped to high address");
}

#[test]
fn test_pop_mem_with_extended_registers() {
    let code = [
        0x68, 0x15, 0xf1, 0x00, 0x00, // PUSH 0xF115
        0x41, 0x8f, 0x07, // POP [R15]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r15 = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    assert_eq!(val, 0xF115, "Value popped to [R15]");
}

#[test]
fn test_pop_mem_indexed_array() {
    let code = [
        0x6a, 0x55, // PUSH 0x55
        0x8f, 0x04, 0xd8, // POP [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 5; // array[5]
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR + 40);
    assert_eq!(val, 0x55, "array[5] = 0x55");
}

#[test]
fn test_pop_mem_complex_addressing() {
    let code = [
        0x68, 0xff, 0xee, 0x0c, 0x00, // PUSH 0xCEEFF
        0x8f, 0x44, 0x88, 0x20, // POP [RAX + RCX*4 + 0x20]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rcx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Effective address: DATA_ADDR + 0x10*4 + 0x20 = DATA_ADDR + 0x60
    let val = read_mem_at_u64(&vm, DATA_ADDR + 0x60);
    assert_eq!(val, 0xCEEFF, "Complex address value popped");
}

#[test]
fn test_pop_mem_struct_fields() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0x8f, 0x00, // POP [RAX] (field 0)
        0x8f, 0x40, 0x08, // POP [RAX + 8] (field 1)
        0x8f, 0x40, 0x10, // POP [RAX + 16] (field 2)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR), 3, "field 0");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 8), 2, "field 1");
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 16), 1, "field 2");
}

#[test]
fn test_pop_mem_consecutive_locations() {
    let code = [
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, 0x6a, 0x04, 0x6a, 0x05, 0x8f, 0x40,
        0x00, // POP [RAX + 0]
        0x8f, 0x40, 0x08, // POP [RAX + 8]
        0x8f, 0x40, 0x10, // POP [RAX + 16]
        0x8f, 0x40, 0x18, // POP [RAX + 24]
        0x8f, 0x40, 0x20, // POP [RAX + 32]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR), 5);
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 8), 4);
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 16), 3);
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 24), 2);
    assert_eq!(read_mem_at_u64(&vm, DATA_ADDR + 32), 1);
}

#[test]
fn test_pop_mem_overwrite_existing() {
    let code = [
        0x6a, 0x99, // PUSH 0x99
        0x8f, 0x00, // POP [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Pre-fill with different value
    write_mem_at_u64(&vm, DATA_ADDR, 0x1111111111111111);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, DATA_ADDR);
    // PUSH imm8 sign-extends: 0x99 (bit 7 set) -> 0xFFFFFFFFFFFFFF99
    assert_eq!(val, 0xFFFFFFFFFFFFFF99, "Old value overwritten");
}
