use crate::common::{
    Bytes, VM, run_until_hlt_legacy as run_until_hlt, setup_vm_legacy as setup_vm,
};

// XCHG - Exchange Register/Memory with Register
// Swaps the contents of two operands
// Opcodes:
//   90+rd - XCHG RAX, r64 (special encoding)
//   87 /r - XCHG r/m64, r64

#[test]
fn test_xchg_rax_rbx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0x93, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x99); // Swapped
    assert_eq!(vm.rbx, 0x42); // Swapped
}

#[test]
fn test_xchg_rax_rcx() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc1, 0x22, 0x00, 0x00, 0x00, // MOV RCX, 0x22
        0x48, 0x91, // XCHG RAX, RCX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x22);
    assert_eq!(vm.rcx, 0x11);
}

#[test]
fn test_xchg_rax_rdx() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x48, 0xc7, 0xc2, 0xbb, 0x00, 0x00, 0x00, // MOV RDX, 0xBB
        0x48, 0x92, // XCHG RAX, RDX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0xBB);
    assert_eq!(vm.rdx, 0xAA);
}

#[test]
fn test_xchg_two_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33
        0x48, 0xc7, 0xc1, 0x44, 0x00, 0x00, 0x00, // MOV RCX, 0x44
        0x48, 0x87, 0xcb, // XCHG RBX, RCX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x44);
    assert_eq!(vm.rcx, 0x33);
}

#[test]
fn test_xchg_register_with_memory() {
    let code = [
        0x48, 0xc7, 0xc0, 0x55, 0x00, 0x00, 0x00, // MOV RAX, 0x55
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xc7, 0x03, 0x66, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0x66
        0x48, 0x87, 0x03, // XCHG [RBX], RAX
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x66); // Got memory value
    assert_eq!(vm.read_memory(0x4000, 8), &[0x55, 0, 0, 0, 0, 0, 0, 0]); // Memory got RAX
}

#[test]
fn test_xchg_memory_with_register() {
    let code = [
        0x48, 0xc7, 0xc1, 0x77, 0x00, 0x00, 0x00, // MOV RCX, 0x77
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xc7, 0x03, 0x88, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0x88
        0x48, 0x87, 0x0b, // XCHG [RBX], RCX
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0x88);
    assert_eq!(vm.read_memory(0x4000, 8), &[0x77, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_xchg_rax_rax_nop() {
    // XCHG RAX, RAX is a NOP
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0x90, // XCHG RAX, RAX (NOP)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42); // Unchanged
}

#[test]
fn test_xchg_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc2, 0x99, 0x00, 0x00, 0x00, // MOV RDX, 0x99
        0x48, 0x93, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rdx, 0x99); // Preserved
}

#[test]
fn test_xchg_32bit() {
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0x93, // XCHG EAX, EBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x22222222); // Zero-extended
    assert_eq!(vm.rbx, 0x11111111);
}

#[test]
fn test_xchg_16bit() {
    let code = [
        0x66, 0xb8, 0x11, 0x11, // MOV AX, 0x1111
        0x66, 0xbb, 0x22, 0x22, // MOV BX, 0x2222
        0x66, 0x93, // XCHG AX, BX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFFFF, 0x2222);
    assert_eq!(vm.rbx & 0xFFFF, 0x1111);
}

#[test]
fn test_xchg_8bit() {
    let code = [
        0xb0, 0x42, // MOV AL, 0x42
        0xb3, 0x99, // MOV BL, 0x99
        0x86, 0xc3, // XCHG AL, BL
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax & 0xFF, 0x99);
    assert_eq!(vm.rbx & 0xFF, 0x42);
}

#[test]
fn test_xchg_temp_variable_pattern() {
    // Swap without temporary variable
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x14, 0x00, 0x00, 0x00, // MOV RBX, 20
        0x48, 0x87, 0xd8, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 20);
    assert_eq!(vm.rbx, 10);
}

#[test]
fn test_xchg_double_swap() {
    // Swap twice returns to original
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0x93, // XCHG RAX, RBX
        0x48, 0x93, // XCHG RAX, RBX (again)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x42); // Back to original
    assert_eq!(vm.rbx, 0x99);
}

#[test]
fn test_xchg_chain() {
    // Rotate values through registers
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0x93, // XCHG RAX, RBX (RAX=2, RBX=1)
        0x48, 0x91, // XCHG RAX, RCX (RAX=3, RCX=2)
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 3);
    assert_eq!(vm.rbx, 1);
    assert_eq!(vm.rcx, 2);
}

#[test]
fn test_xchg_with_offset() {
    let code = [
        0x48, 0xc7, 0xc1, 0xaa, 0x00, 0x00, 0x00, // MOV RCX, 0xAA
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xc7, 0x43, 0x08, 0xbb, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+8], 0xBB
        0x48, 0x87, 0x4b, 0x08, // XCHG [RBX+8], RCX
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rcx, 0xBB);
    assert_eq!(vm.read_memory(0x4008, 8), &[0xAA, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_xchg_atomic_semantics() {
    // XCHG with memory is implicitly locked (atomic)
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000
        0x48, 0xc7, 0x03, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0
        0x48, 0x87, 0x03, // XCHG [RBX], RAX (atomic)
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0);
    assert_eq!(vm.read_memory(0x4000, 8), &[1, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_xchg_all_gp_registers() {
    // Test XCHG with various register combinations
    let code = [
        0x48, 0xc7, 0xc6, 0x55, 0x00, 0x00, 0x00, // MOV RSI, 0x55
        0x48, 0xc7, 0xc7, 0x66, 0x00, 0x00, 0x00, // MOV RDI, 0x66
        0x48, 0x87, 0xf7, // XCHG RSI, RDI
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rsi, 0x66);
    assert_eq!(vm.rdi, 0x55);
}

#[test]
fn test_xchg_self() {
    // XCHG reg, reg with same register (besides RAX, RAX)
    let code = [
        0x48, 0xc7, 0xc3, 0x42, 0x00, 0x00, 0x00, // MOV RBX, 0x42
        0x48, 0x87, 0xdb, // XCHG RBX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rbx, 0x42); // Unchanged
}

#[test]
fn test_xchg_large_values() {
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, i64::MAX
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, // MOV RBX, i64::MIN
        0x48, 0x93, // XCHG RAX, RBX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax as i64, i64::MIN);
    assert_eq!(vm.rbx as i64, i64::MAX);
}

#[test]
fn test_xchg_with_sib() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, 0x4000 (base)
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2 (index)
        0x48, 0xc7, 0x04, 0xcb, 0x22, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX+RCX*8], 0x22
        0x48, 0x87, 0x04, 0xcb, // XCHG [RBX+RCX*8], RAX
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0x22);
    let addr = 0x4000 + 2 * 8;
    assert_eq!(vm.read_memory(addr, 8), &[0x11, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_xchg_spin_lock_pattern() {
    // Test-and-set lock pattern
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x40, 0x00, 0x00, // MOV RBX, lock_addr
        0x48, 0xc7, 0x03, 0x00, 0x00, 0x00, 0x00, // MOV QWORD PTR [RBX], 0 (unlocked)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1 (locked value)
        0x48, 0x87, 0x03, // XCHG [RBX], RAX (atomic test-and-set)
        0xf4, // HLT
    ];
    let mut vm = setup_vm(&code);
    vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 0); // Previous value (was unlocked)
    assert_eq!(vm.read_memory(0x4000, 8), &[1, 0, 0, 0, 0, 0, 0, 0]); // Now locked
}

#[test]
fn test_xchg_consecutive() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xc7, 0xc2, 0x04, 0x00, 0x00, 0x00, // MOV RDX, 4
        0x48, 0x93, // XCHG RAX, RBX
        0x48, 0x91, // XCHG RAX, RCX
        0x48, 0x92, // XCHG RAX, RDX
        0xf4, // HLT
    ];
    let vm = setup_vm(&code);
    let vm = run_until_hlt(vm);
    assert_eq!(vm.rax, 4);
    assert_eq!(vm.rbx, 1);
    assert_eq!(vm.rcx, 2);
    assert_eq!(vm.rdx, 3);
}
