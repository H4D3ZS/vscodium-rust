use rax::cpu::Registers;

use crate::common::{run_until_hlt, setup_vm};

// LEA - Load Effective Address
// Calculates address and stores it in destination register (doesn't access memory)

// Basic LEA with base register only
#[test]
fn test_lea_eax_rbx() {
    let code = [0x67, 0x8d, 0x03, 0xf4]; // LEA EAX, [RBX]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1000,
        "EAX should contain address from RBX"
    );
}

// LEA with displacement
#[test]
fn test_lea_eax_rbx_disp8() {
    let code = [0x67, 0x8d, 0x43, 0x10, 0xf4]; // LEA EAX, [RBX + 0x10]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1010,
        "EAX should contain RBX + 0x10"
    );
}

#[test]
fn test_lea_eax_rbx_disp32() {
    let code = [0x67, 0x8d, 0x83, 0x00, 0x10, 0x00, 0x00, 0xf4]; // LEA EAX, [RBX + 0x1000]
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x3000,
        "EAX should contain RBX + 0x1000"
    );
}

// LEA with base + index
#[test]
fn test_lea_eax_rbx_rcx() {
    let code = [0x67, 0x8d, 0x04, 0x0b, 0xf4]; // LEA EAX, [RBX + RCX]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1100,
        "EAX should contain RBX + RCX"
    );
}

// LEA with scale factor (SIB byte)
#[test]
fn test_lea_eax_rbx_rcx_scale2() {
    let code = [0x67, 0x8d, 0x04, 0x4b, 0xf4]; // LEA EAX, [RBX + RCX*2]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1200,
        "EAX should contain RBX + RCX*2"
    );
}

#[test]
fn test_lea_eax_rbx_rcx_scale4() {
    let code = [0x67, 0x8d, 0x04, 0x8b, 0xf4]; // LEA EAX, [RBX + RCX*4]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1400,
        "EAX should contain RBX + RCX*4"
    );
}

#[test]
fn test_lea_eax_rbx_rcx_scale8() {
    let code = [0x67, 0x8d, 0x04, 0xcb, 0xf4]; // LEA EAX, [RBX + RCX*8]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1800,
        "EAX should contain RBX + RCX*8"
    );
}

// LEA with base + index*scale + displacement
#[test]
fn test_lea_eax_complex() {
    let code = [0x67, 0x8d, 0x44, 0x8b, 0x20, 0xf4]; // LEA EAX, [RBX + RCX*4 + 0x20]
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0010;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x1060,
        "EAX should contain RBX + RCX*4 + 0x20"
    );
}

// 64-bit LEA
#[test]
fn test_lea_rax_rbx() {
    let code = [0x48, 0x8d, 0x03, 0xf4]; // LEA RAX, [RBX]
    let mut regs = Registers::default();
    regs.rbx = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x123456789ABCDEF0,
        "RAX should contain full 64-bit address"
    );
}

#[test]
fn test_lea_rax_rbx_disp() {
    let code = [0x48, 0x8d, 0x83, 0x00, 0x00, 0x00, 0x10, 0xf4]; // LEA RAX, [RBX + 0x10000000]
    let mut regs = Registers::default();
    regs.rbx = 0x0000000100000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000110000000,
        "RAX should contain RBX + displacement"
    );
}

// 16-bit LEA
#[test]
fn test_lea_ax_bx() {
    let code = [0x66, 0x67, 0x8d, 0x03, 0xf4]; // LEA AX, [BX]
    let mut regs = Registers::default();
    regs.rbx = 0xFFFFFFFFFFFF1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX should contain lower 16 bits");
}

// LEA with index register only (base = 0)
#[test]
fn test_lea_eax_rcx_scale4_no_base() {
    let code = [0x67, 0x8d, 0x04, 0x8d, 0x00, 0x00, 0x00, 0x00, 0xf4]; // LEA EAX, [RCX*4]
    let mut regs = Registers::default();
    regs.rcx = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0x0400, "EAX should contain RCX*4");
}

// LEA practical use: array indexing
#[test]
fn test_lea_array_indexing() {
    // Calculate address of array[index] where element size is 4 bytes
    // address = base + index * 4
    let code = [0x67, 0x8d, 0x04, 0x8b, 0xf4]; // LEA EAX, [RBX + RCX*4]
    let mut regs = Registers::default();
    regs.rbx = 0x00400000; // array base
    regs.rcx = 5; // index
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00400014,
        "Should calculate array[5] address"
    );
}

// LEA practical use: structure member access
#[test]
fn test_lea_struct_member() {
    // Calculate address of struct member at offset 0x18
    let code = [0x67, 0x8d, 0x43, 0x18, 0xf4]; // LEA EAX, [RBX + 0x18]
    let mut regs = Registers::default();
    regs.rbx = 0x00500000; // struct base
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x00500018,
        "Should calculate struct member address"
    );
}

// LEA practical use: arithmetic (multiply by 3)
#[test]
fn test_lea_multiply_by_3() {
    // LEA can do x*3 efficiently: LEA RAX, [RBX + RBX*2]
    let code = [0x48, 0x8d, 0x04, 0x5b, 0xf4]; // LEA RAX, [RBX + RBX*2]
    let mut regs = Registers::default();
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 300, "Should multiply by 3 (100*3=300)");
}

// LEA practical use: arithmetic (multiply by 5)
#[test]
fn test_lea_multiply_by_5() {
    // LEA can do x*5 efficiently: LEA RAX, [RBX + RBX*4]
    let code = [0x48, 0x8d, 0x04, 0x9b, 0xf4]; // LEA RAX, [RBX + RBX*4]
    let mut regs = Registers::default();
    regs.rbx = 20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100, "Should multiply by 5 (20*5=100)");
}

// LEA practical use: arithmetic (multiply by 9)
#[test]
fn test_lea_multiply_by_9() {
    // LEA can do x*9 efficiently: LEA RAX, [RBX + RBX*8]
    let code = [0x48, 0x8d, 0x04, 0xdb, 0xf4]; // LEA RAX, [RBX + RBX*8]
    let mut regs = Registers::default();
    regs.rbx = 10;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 90, "Should multiply by 9 (10*9=90)");
}

// LEA practical use: add constant efficiently
#[test]
fn test_lea_add_constant() {
    let code = [0x48, 0x8d, 0x83, 0xe8, 0x03, 0x00, 0x00, 0xf4]; // LEA RAX, [RBX + 1000]
    let mut regs = Registers::default();
    regs.rbx = 500;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1500, "Should add 1000 to RBX");
}

// Test that LEA doesn't affect flags
#[test]
fn test_lea_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x48, 0x8d, 0x04, 0x5b, // LEA RAX, [RBX + RBX*2]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & 0x40 != 0, "ZF should still be set");
    assert_eq!(regs.rax, 300, "LEA calculation should work");
}

// Test that 32-bit LEA zeros upper 32 bits
#[test]
fn test_lea_32bit_zeros_upper() {
    let code = [0x8d, 0x04, 0x5b, 0xf4]; // LEA EAX, [RBX + RBX*2]
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFDEADBEEF;
    regs.rbx = 100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x000000000000012C,
        "Upper 32 bits should be zeroed"
    );
}

// Test with extended registers
#[test]
fn test_lea_r8_r9() {
    let code = [0x4f, 0x8d, 0x04, 0x59, 0xf4]; // LEA R8, [R9 + R11*2]
    let mut regs = Registers::default();
    regs.r9 = 0x1000;
    regs.r11 = 0x0100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 0x1200, "R8 should contain R9 + R11*2");
}

#[test]
fn test_lea_rax_r10_r11() {
    let code = [0x4a, 0x8d, 0x04, 0xda, 0xf4]; // LEA RAX, [RDX + R11*8]
    let mut regs = Registers::default();
    regs.rdx = 0x2000;
    regs.r11 = 0x0040;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2200, "RAX should contain RDX + R11*8");
}

// Test negative displacement
#[test]
fn test_lea_negative_disp() {
    let code = [0x48, 0x8d, 0x43, 0xf0, 0xf4]; // LEA RAX, [RBX - 16] (0xf0 = -16 signed)
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0FF0, "RAX should be RBX - 16");
}

// Test with RIP-relative addressing (position-independent code)
#[test]
fn test_lea_rip_relative() {
    let code = [0x48, 0x8d, 0x05, 0x00, 0x00, 0x00, 0x00, 0xf4]; // LEA RAX, [RIP + 0]
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // RIP should point to the instruction after LEA (offset 7)
    // At runtime, RIP will be VM base + 7
}

// Test zero calculations
#[test]
fn test_lea_zero_result() {
    let code = [0x48, 0x8d, 0x04, 0x5b, 0xf4]; // LEA RAX, [RBX + RBX*2]
    let mut regs = Registers::default();
    regs.rbx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "RAX should be 0");
}

// Test large address calculation
#[test]
fn test_lea_large_address() {
    let code = [0x48, 0x8d, 0x84, 0xcb, 0x00, 0x10, 0x00, 0x00, 0xf4]; // LEA RAX, [RBX + RCX*8 + 0x1000]
    let mut regs = Registers::default();
    regs.rbx = 0x0000000100000000;
    regs.rcx = 0x0000000010000000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000000180001000,
        "Should handle large addresses"
    );
}

// Test all different register combinations
#[test]
fn test_lea_different_registers() {
    let code = [0x48, 0x8d, 0x04, 0xf5, 0x00, 0x00, 0x00, 0x00, 0xf4]; // LEA RAX, [RSI*8]
    let mut regs = Registers::default();
    regs.rsi = 0x100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x800, "RAX should be RSI*8");
}

#[test]
fn test_lea_rdi_rsi_rdx() {
    let code = [0x48, 0x8d, 0x3c, 0x96, 0xf4]; // LEA RDI, [RSI + RDX*4]
    let mut regs = Registers::default();
    regs.rsi = 0x2000;
    regs.rdx = 0x0080;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi, 0x2200, "RDI should be RSI + RDX*4");
}

// Test chaining multiple LEAs
#[test]
fn test_lea_chain() {
    let code = [
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x8d, 0x04, 0x5b, // LEA RAX, [RBX + RBX*2] (RAX = 30)
        0x48, 0x8d, 0x14, 0x85, 0x00, 0x00, 0x00, 0x00, // LEA RDX, [RAX*4] (RDX = 120)
        0x48, 0x8d, 0x0c, 0x52, // LEA RCX, [RDX + RDX*2] (RCX = 360)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 30, "RAX should be 30");
    assert_eq!(regs.rdx, 120, "RDX should be 120");
    assert_eq!(regs.rcx, 360, "RCX should be 360");
}

// ============================================================================
// Strengthened LEA tests (appended): exact effective addresses for full SIB
// (base + index*scale + disp), operand-size truncation of the result, and the
// guarantee that LEA never touches flags.
// ============================================================================

#[test]
fn test_strict_lea_full_sib_scale8_disp32() {
    // LEA RAX, [RBX + RCX*8 + 0x1000]
    // EA = 0x10000 + 0x20*8 + 0x1000 = 0x10000 + 0x100 + 0x1000 = 0x11100
    let code = [0x48, 0x8d, 0x84, 0xcb, 0x00, 0x10, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = 0x10000;
    regs.rcx = 0x20;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x11100, "RAX = RBX + RCX*8 + 0x1000");
}

#[test]
fn test_strict_lea_index_only_scale4_disp32() {
    // LEA RAX, [RCX*4 + 0x40]  (no base; mod=00, base=101 => disp32 only)
    // EA = 0x100*4 + 0x40 = 0x440
    let code = [0x48, 0x8d, 0x04, 0x8d, 0x40, 0x00, 0x00, 0x00, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x440, "RAX = RCX*4 + 0x40");
}

#[test]
fn test_strict_lea_negative_disp8() {
    // LEA RAX, [RBX - 8]  (disp8 = 0xF8 = -8)
    let code = [0x48, 0x8d, 0x43, 0xf8, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1FF8, "RAX = RBX - 8");
}

#[test]
fn test_strict_lea_32bit_truncates_result() {
    // LEA EAX, [RBX + RCX]  — 32-bit operand size truncates the EA to 32 bits.
    // EA = 0xFFFF_FFFF + 1 = 0x1_0000_0000, truncated to 0x0000_0000.
    let code = [0x8d, 0x04, 0x0b, 0xf4]; // LEA EAX, [RBX+RCX]
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF_FFFF;
    regs.rcx = 0x1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0000_0000,
        "32-bit LEA truncates and zero-extends result"
    );
}

#[test]
fn test_strict_lea_does_not_touch_flags() {
    // LEA must never change flags; seed CF/ZF/SF/OF and verify unchanged.
    let code = [0x48, 0x8d, 0x43, 0x10, 0xf4]; // LEA RAX, [RBX+0x10]
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    regs.rflags = 0x2 | 0x1 | 0x40 | 0x80 | 0x800;
    let before = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2010);
    assert_eq!(
        regs.rflags & 0x8D5,
        before & 0x8D5,
        "LEA must not alter status flags"
    );
}
