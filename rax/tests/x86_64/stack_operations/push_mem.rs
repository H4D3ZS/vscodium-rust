use crate::common::{DATA_ADDR, read_mem_at_u64, run_until_hlt, setup_vm, write_mem_at_u64};
use rax::cpu::Registers;

// Comprehensive tests for PUSH with memory operands
//
// PUSH m64 - Push quadword from memory onto stack
// Various addressing modes:
// - Direct: PUSH [addr]
// - Register indirect: PUSH [reg]
// - Register + displacement: PUSH [reg + disp]
// - Base + index: PUSH [base + index]
// - Base + index + displacement: PUSH [base + index + disp]
// - RIP-relative: PUSH [RIP + disp]

// ============================================================================
// PUSH with register indirect addressing [reg]
// ============================================================================

#[test]
fn test_push_mem_indirect_rax() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Write value to memory
    write_mem_at_u64(&vm, DATA_ADDR, 0x1234567890ABCDEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x0FF8, "RSP decremented");
    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x1234567890ABCDEF, "Value from [RAX] pushed");
}

#[test]
fn test_push_mem_indirect_rbx() {
    let code = [
        0xff, 0x33, // PUSH [RBX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbx = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xDEADBEEFCAFEBABE);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xDEADBEEFCAFEBABE, "Value from [RBX] pushed");
}

#[test]
fn test_push_mem_indirect_rcx() {
    let code = [
        0xff, 0x31, // PUSH [RCX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rcx = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xAAAAAAAABBBBBBBB);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xAAAAAAAABBBBBBBB, "Value from [RCX] pushed");
}

#[test]
fn test_push_mem_indirect_r8() {
    let code = [
        0x41, 0xff, 0x30, // PUSH [R8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r8 = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1111222233334444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x1111222233334444, "Value from [R8] pushed");
}

// ============================================================================
// PUSH with displacement [reg + disp8]
// ============================================================================

#[test]
fn test_push_mem_disp8_positive() {
    let code = [
        0xff, 0x70, 0x08, // PUSH [RAX + 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 8, 0x4242424242424242);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x4242424242424242, "Value from [RAX+8] pushed");
}

#[test]
fn test_push_mem_disp8_negative() {
    let code = [
        0xff, 0x70, 0xf8, // PUSH [RAX - 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR + 16;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 8, 0x9999999999999999);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x9999999999999999, "Value from [RAX-8] pushed");
}

#[test]
fn test_push_mem_disp8_zero() {
    let code = [
        0xff, 0x70, 0x00, // PUSH [RAX + 0]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x5555555555555555);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x5555555555555555, "Value from [RAX+0] pushed");
}

// ============================================================================
// PUSH with 32-bit displacement [reg + disp32]
// ============================================================================

#[test]
fn test_push_mem_disp32_large() {
    let code = [
        0xff, 0xb0, 0x00, 0x10, 0x00, 0x00, // PUSH [RAX + 0x1000]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x1000, 0x7777777777777777);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x7777777777777777, "Value from [RAX+0x1000] pushed");
}

#[test]
fn test_push_mem_disp32_small() {
    let code = [
        0xff, 0xb0, 0x10, 0x00, 0x00, 0x00, // PUSH [RAX + 0x10]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x10, 0x1234123412341234);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x1234123412341234, "Value from [RAX+0x10] pushed");
}

// ============================================================================
// PUSH with SIB addressing [base + index]
// ============================================================================

#[test]
fn test_push_mem_sib_base_index() {
    let code = [
        0xff, 0x34, 0x18, // PUSH [RAX + RBX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x100;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x100, 0x8888888888888888);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x8888888888888888, "Value from [RAX+RBX] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale2() {
    let code = [
        0xff, 0x34, 0x58, // PUSH [RAX + RBX*2]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x20, 0x2222222222222222);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x2222222222222222, "Value from [RAX+RBX*2] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale4() {
    let code = [
        0xff, 0x34, 0x98, // PUSH [RAX + RBX*4]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x08;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x20, 0x3333333333333333);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x3333333333333333, "Value from [RAX+RBX*4] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale8() {
    let code = [
        0xff, 0x34, 0xd8, // PUSH [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x04;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x20, 0x4444444444444444);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x4444444444444444, "Value from [RAX+RBX*8] pushed");
}

// ============================================================================
// PUSH with SIB + displacement [base + index*scale + disp]
// ============================================================================

#[test]
fn test_push_mem_sib_disp8() {
    let code = [
        0xff, 0x74, 0x18, 0x10, // PUSH [RAX + RBX + 0x10]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x20;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x30, 0x6666666666666666);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x6666666666666666, "Value from [RAX+RBX+0x10] pushed");
}

#[test]
fn test_push_mem_sib_scale_disp8() {
    let code = [
        0xff, 0x74, 0x58, 0x08, // PUSH [RAX + RBX*2 + 8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x28, 0xABCDABCDABCDABCD);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xABCDABCDABCDABCD, "Value from [RAX+RBX*2+8] pushed");
}

#[test]
fn test_push_mem_sib_disp32() {
    let code = [
        0xff, 0xb4, 0x18, 0x00, 0x01, 0x00, 0x00, // PUSH [RAX + RBX + 0x100]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 0x50;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 0x150, 0xFEDCFEDCFEDCFEDC);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xFEDCFEDCFEDCFEDC, "Value from [RAX+RBX+0x100] pushed");
}

// ============================================================================
// PUSH multiple memory values
// ============================================================================

#[test]
fn test_push_mem_sequence() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xff, 0x33, // PUSH [RBX]
        0xff, 0x31, // PUSH [RCX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = DATA_ADDR + 0x10;
    regs.rcx = DATA_ADDR + 0x20;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1111111111111111);
    write_mem_at_u64(&vm, DATA_ADDR + 0x10, 0x2222222222222222);
    write_mem_at_u64(&vm, DATA_ADDR + 0x20, 0x3333333333333333);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rsp, 0x1000 - 24, "Three values pushed");
    assert_eq!(read_mem_at_u64(&vm, 0x1000 - 8), 0x1111111111111111);
    assert_eq!(read_mem_at_u64(&vm, 0x1000 - 16), 0x2222222222222222);
    assert_eq!(read_mem_at_u64(&vm, 0x1000 - 24), 0x3333333333333333);
}

#[test]
fn test_push_mem_array_elements() {
    let code = [
        // Push array[0], array[1], array[2]
        0xff, 0x30, // PUSH [RAX] (array[0])
        0xff, 0x70, 0x08, // PUSH [RAX + 8] (array[1])
        0xff, 0x70, 0x10, // PUSH [RAX + 16] (array[2])
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Set up array
    write_mem_at_u64(&vm, DATA_ADDR, 0xAA);
    write_mem_at_u64(&vm, DATA_ADDR + 8, 0xBB);
    write_mem_at_u64(&vm, DATA_ADDR + 16, 0xCC);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&vm, regs.rsp), 0xCC, "array[2]");
    assert_eq!(read_mem_at_u64(&vm, regs.rsp + 8), 0xBB, "array[1]");
    assert_eq!(read_mem_at_u64(&vm, regs.rsp + 16), 0xAA, "array[0]");
}

// ============================================================================
// PUSH memory then POP
// ============================================================================

#[test]
fn test_push_mem_pop_roundtrip() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0x5b, // POP RBX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xBEEFBEEFBEEFBEEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0xBEEFBEEFBEEFBEEF, "Memory value in RBX");
    assert_eq!(regs.rsp, 0x1000, "Stack balanced");
}

// ============================================================================
// PUSH memory preserves flags and other registers
// ============================================================================

#[test]
fn test_push_mem_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0xc7, 0xc1, 0x88, 0x00, 0x00, 0x00, // MOV RCX, 0x88
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1234567890ABCDEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(regs.rbx, 0x99, "RBX unchanged");
    assert_eq!(regs.rcx, 0x88, "RCX unchanged");
}

#[test]
fn test_push_mem_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x42);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_ne!(regs.rflags & 0x01, 0, "CF preserved");
}

// ============================================================================
// PUSH with different memory values
// ============================================================================

#[test]
fn test_push_mem_zero() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0, "Zero value pushed");
}

#[test]
fn test_push_mem_all_ones() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xFFFFFFFFFFFFFFFF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "All ones pushed");
}

#[test]
fn test_push_mem_alternating_bits() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xAAAAAAAAAAAAAAAA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xAAAAAAAAAAAAAAAA, "Alternating bits pushed");
}

// ============================================================================
// PUSH from stack memory (reading stack to push)
// ============================================================================

#[test]
fn test_push_mem_from_stack() {
    let code = [
        0x6a, 0x42, // PUSH 0x42 (put value on stack)
        0x48, 0x89, 0xe0, // MOV RAX, RSP (RAX points to stack)
        0xff, 0x30, // PUSH [RAX] (push copy of top of stack)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Should have two copies of 0x42 on stack
    assert_eq!(read_mem_at_u64(&vm, regs.rsp), 0x42, "Duplicate on top");
    assert_eq!(read_mem_at_u64(&vm, regs.rsp + 8), 0x42, "Original");
}

// ============================================================================
// PUSH with various base registers
// ============================================================================

#[test]
fn test_push_mem_base_rsi() {
    let code = [
        0xff, 0x36, // PUSH [RSI]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rsi = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x9876543210FEDCBA);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x9876543210FEDCBA, "Value from [RSI] pushed");
}

#[test]
fn test_push_mem_base_rdi() {
    let code = [
        0xff, 0x37, // PUSH [RDI]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rdi = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1122334455667788);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x1122334455667788, "Value from [RDI] pushed");
}

#[test]
fn test_push_mem_base_rbp() {
    let code = [
        0xff, 0x75, 0x00, // PUSH [RBP + 0]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rbp = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x5A5A5A5A5A5A5A5A);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x5A5A5A5A5A5A5A5A, "Value from [RBP] pushed");
}

// ============================================================================
// PUSH memory in function parameter passing
// ============================================================================

#[test]
fn test_push_mem_struct_fields() {
    let code = [
        // Assuming RAX points to a struct, push its fields
        0xff, 0x30, // PUSH [RAX] (field 0)
        0xff, 0x70, 0x08, // PUSH [RAX + 8] (field 1)
        0xff, 0x70, 0x10, // PUSH [RAX + 16] (field 2)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0x1111);
    write_mem_at_u64(&vm, DATA_ADDR + 8, 0x2222);
    write_mem_at_u64(&vm, DATA_ADDR + 16, 0x3333);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    assert_eq!(read_mem_at_u64(&vm, regs.rsp), 0x3333, "Field 2");
    assert_eq!(read_mem_at_u64(&vm, regs.rsp + 8), 0x2222, "Field 1");
    assert_eq!(read_mem_at_u64(&vm, regs.rsp + 16), 0x1111, "Field 0");
}

// ============================================================================
// PUSH memory edge cases
// ============================================================================

#[test]
fn test_push_mem_high_address() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = 0x100000;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, 0x100000, 0xDEADBEEF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xDEADBEEF, "Value from high address pushed");
}

#[test]
fn test_push_mem_with_extended_registers() {
    let code = [
        0x41, 0xff, 0x37, // PUSH [R15]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.r15 = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR, 0xF15F15F15F15F15);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xF15F15F15F15F15, "Value from [R15] pushed");
}

#[test]
fn test_push_mem_indexed_array_access() {
    let code = [
        // PUSH array[RBX] where array base is in RAX
        0xff, 0x34, 0xd8, // PUSH [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rbx = 5; // array[5]
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    write_mem_at_u64(&vm, DATA_ADDR + 40, 0x5555555555555555);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0x5555555555555555, "array[5] pushed");
}

#[test]
fn test_push_mem_complex_addressing() {
    let code = [
        // PUSH [RAX + RCX*4 + 0x20]
        0xff, 0x74, 0x88, 0x20, // PUSH [RAX + RCX*4 + 0x20]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    regs.rcx = 0x10;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    // Effective address: DATA_ADDR + 0x10*4 + 0x20 = DATA_ADDR + 0x60
    write_mem_at_u64(&vm, DATA_ADDR + 0x60, 0xC0FFEEC0FFEEC0FF);

    let regs = run_until_hlt(&mut vcpu).unwrap();

    let val = read_mem_at_u64(&vm, 0x0FF8);
    assert_eq!(val, 0xC0FFEEC0FFEEC0FF, "Complex address value pushed");
}

#[test]
fn test_push_mem_consecutive_locations() {
    let code = [
        0xff, 0x70, 0x00, // PUSH [RAX + 0]
        0xff, 0x70, 0x08, // PUSH [RAX + 8]
        0xff, 0x70, 0x10, // PUSH [RAX + 16]
        0xff, 0x70, 0x18, // PUSH [RAX + 24]
        0xff, 0x70, 0x20, // PUSH [RAX + 32]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rsp = 0x1000;
    regs.rax = DATA_ADDR;
    let (mut vcpu, vm) = setup_vm(&code, Some(regs));

    for i in 0..5 {
        write_mem_at_u64(&vm, DATA_ADDR + i * 8, (i + 1) as u64 * 0x1111111111111111);
    }

    let regs = run_until_hlt(&mut vcpu).unwrap();

    for i in 0..5 {
        let val = read_mem_at_u64(&vm, regs.rsp + i * 8);
        assert_eq!(val, (5 - i) as u64 * 0x1111111111111111, "Value {}", i);
    }
}
