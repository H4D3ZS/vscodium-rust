use crate::common::{VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm};

// LEA - Load Effective Address
// Computes address and stores it in destination register (doesn't access memory)
// Opcode: 8D /r - LEA r64, m

#[test]
fn test_lea_basic() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x48, 0x8d, 0x03, // LEA RAX, [RBX]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x3000); // Address loaded, not value
}

#[test]
fn test_lea_with_offset() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x48, 0x8d, 0x43, 0x10, // LEA RAX, [RBX+16]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x3010);
}

#[test]
fn test_lea_with_sib() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000 (base)
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, 5 (index)
        0x48, 0x8d, 0x04, 0x8b, // LEA RAX, [RBX+RCX*4]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x1000 + 5 * 4); // 0x1014
}

#[test]
fn test_lea_with_scale_2() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc2, 0x08, 0x00, 0x00, 0x00, // MOV RDX, 8
        0x48, 0x8d, 0x04, 0x53, // LEA RAX, [RBX+RDX*2]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x2000 + 8 * 2); // 0x2010
}

#[test]
fn test_lea_with_scale_8() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x30, 0x00, 0x00, // MOV RBX, 0x3000
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x8d, 0x04, 0xcb, // LEA RAX, [RBX+RCX*8]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x3000 + 3 * 8); // 0x3018
}

#[test]
fn test_lea_with_displacement() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0x8d, 0x84, 0x8b, 0x10, 0x00, 0x00, 0x00, // LEA RAX, [RBX+RCX*4+16]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x4000 + 2 * 4 + 16); // 0x4018
}

#[test]
fn test_lea_arithmetic_pattern() {
    // LEA can be used for arithmetic: x*3
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0x8d, 0x04, 0x40, // LEA RAX, [RAX+RAX*2] (RAX*3)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 15); // 5*3
}

#[test]
fn test_lea_multiply_by_5() {
    // x*5 = x*4 + x
    let code = [
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7
        0x48, 0x8d, 0x04, 0x80, // LEA RAX, [RAX+RAX*4] (RAX*5)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 35); // 7*5
}

#[test]
fn test_lea_multiply_by_9() {
    // x*9 = x*8 + x
    let code = [
        0x48, 0xc7, 0xc0, 0x04, 0x00, 0x00, 0x00, // MOV RAX, 4
        0x48, 0x8d, 0x04, 0xc0, // LEA RAX, [RAX+RAX*8] (RAX*9)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 36); // 4*9
}

#[test]
fn test_lea_add_pattern() {
    // LEA for addition without affecting flags
    let code = [
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x8d, 0x43, 0x0a, // LEA RAX, [RBX+10]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 110);
}

#[test]
fn test_lea_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF=1, CF=1)
        0x48, 0xc7, 0xc3, 0x10, 0x00, 0x00, 0x00, // MOV RBX, 16
        0x48, 0x8d, 0x4b, 0x05, // LEA RCX, [RBX+5] (doesn't affect flags)
        0x74, 0x01, // JZ +1 (should jump if ZF still set)
        0xf4, // HLT (should not reach)
        0xf4, // HLT (target)
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 21);
    assert_eq!(vm.rip, (0x1000 + code.len()) as u64); // ZF preserved
}

#[test]
fn test_lea_no_memory_access() {
    // LEA computes address but doesn't access memory
    let code = [
        0xbb, 0xff, 0xff, 0xff, 0xff, // MOV EBX, 0xFFFFFFFF (zero-extends)
        0x48, 0x8d, 0x43, 0x10, // LEA RAX, [RBX+16] (no fault)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0xFFFFFFFFu64 + 16); // Address computed, no access
}

#[test]
fn test_lea_array_indexing() {
    // Base + index*size pattern
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, array_base
        0x48, 0xc7, 0xc1, 0x05, 0x00, 0x00, 0x00, // MOV RCX, index=5
        0x48, 0x8d, 0x04, 0x8b, // LEA RAX, [RBX+RCX*4] (for int array)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x4000 + 5 * 4);
}

#[test]
fn test_lea_struct_field() {
    // Access struct field at offset
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x50, 0x00, 0x00, // MOV RBX, struct_ptr
        0x48, 0x8d, 0x43, 0x18, // LEA RAX, [RBX+24] (field at offset 24)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x5000 + 24);
}

#[test]
fn test_lea_negative_offset() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x60, 0x00, 0x00, // MOV RBX, 0x6000
        0x48, 0x8d, 0x43, 0xf0, // LEA RAX, [RBX-16]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x6000 - 16);
}

#[test]
fn test_lea_rip_relative() {
    // RIP-relative addressing
    let code = [
        0x48, 0x8d, 0x05, 0x00, 0x00, 0x00, 0x00, // LEA RAX, [RIP+0]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    // RAX should contain RIP after this instruction
    assert_eq!(vm.rax, 0x1000 + 7); // After LEA instruction
}

#[test]
fn test_lea_32bit_result() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000
        0x8d, 0x43, 0x10, // LEA EAX, [RBX+16] (32-bit)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x1010); // Zero-extended to 64-bit
}

#[test]
fn test_lea_chain_calculations() {
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x8d, 0x04, 0x40, // LEA RAX, [RAX+RAX*2] (9)
        0x48, 0x8d, 0x04, 0x40, // LEA RAX, [RAX+RAX*2] (27)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 27); // 3*3*3
}

#[test]
fn test_lea_zero_base() {
    // Index without base
    let code = [
        0x48, 0xc7, 0xc1, 0x08, 0x00, 0x00, 0x00, // MOV RCX, 8
        0x48, 0x8d, 0x04, 0x8d, 0x00, 0x00, 0x00, 0x00, // LEA RAX, [RCX*4+0]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 32); // 8*4
}

#[test]
fn test_lea_preserves_source() {
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0x8d, 0x43, 0x10, // LEA RAX, [RBX+16]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x42); // RBX unchanged
    assert_eq!(vm.rax, 0x52);
}

#[test]
fn test_lea_complex_addressing() {
    // Base + Index*Scale + Displacement
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x10, 0x00, 0x00, // MOV RBX, 0x1000 (base)
        0x48, 0xc7, 0xc2, 0x0a, 0x00, 0x00, 0x00, // MOV RDX, 10 (index)
        0x48, 0x8d, 0x84, 0x93, 0x40, 0x00, 0x00, 0x00, // LEA RAX, [RBX+RDX*4+64]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x1000 + 10 * 4 + 64); // 0x1068
}

#[test]
fn test_lea_function_pointer_table() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, table_base
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, index=3
        0x48, 0x8d, 0x04, 0xcb, // LEA RAX, [RBX+RCX*8] (8-byte pointers)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x2000 + 3 * 8);
}

#[test]
fn test_lea_offset_only() {
    // Displacement without base or index
    let code = [
        0x48, 0x8d, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // LEA RAX, [0x3000]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x3000);
}

#[test]
fn test_lea_same_dest_source() {
    // LEA can use destination as source
    let code = [
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 16
        0x48, 0x8d, 0x40, 0x05, // LEA RAX, [RAX+5]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 21);
}

#[test]
fn test_lea_power_of_two_multiply() {
    // Multiply by power of 2 using scale
    let code = [
        0x48, 0xc7, 0xc1, 0x06, 0x00, 0x00, 0x00, // MOV RCX, 6
        0x48, 0x8d, 0x04, 0xcd, 0x00, 0x00, 0x00, 0x00, // LEA RAX, [RCX*8]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 48); // 6*8
}

#[test]
fn test_lea_increment_by_constant() {
    let code = [
        0x48, 0xc7, 0xc3, 0x64, 0x00, 0x00, 0x00, // MOV RBX, 100
        0x48, 0x8d, 0x5b, 0x01, // LEA RBX, [RBX+1] (increment without affecting flags)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 101);
}

#[test]
fn test_lea_stride_calculation() {
    // Calculate address with stride
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, base_addr
        0x48, 0xc7, 0xc1, 0x0c, 0x00, 0x00, 0x00, // MOV RCX, index=12
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, stride=16
        0x48, 0x0f, 0xaf, 0xc8, // IMUL RCX, RAX (index * stride)
        0x48, 0x8d, 0x04, 0x0b, // LEA RAX, [RBX+RCX]
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x4000 + 12 * 16);
}
