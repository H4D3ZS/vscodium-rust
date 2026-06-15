use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

use crate::common::*;

// BOUND - Check Array Index Against Bounds
// Opcode: 62 /r
// Operand: register, memory(lower_bound:upper_bound pair)
//
// BOUND reg, m
// - reg: 16-bit or 32-bit register containing array index
// - m: 4 or 8 byte memory location containing bounds
//   - First word/dword: lower bound (inclusive)
//   - Second word/dword: upper bound (inclusive)
//
// If index < lower_bound or index > upper_bound, generates #BR exception
//
// Encoding for BOUND EAX, [disp32]:
// - 0x62 = BOUND opcode
// - 0x05 = ModRM (mod=00, reg=000, rm=101) meaning [disp32] in compat mode
// - 4 bytes of disp32 (little-endian address)
//
// With 0x66 prefix: uses 32-bit operands (since CS.D=0 means 16-bit default)
// Bounds data: 8 bytes (2x DWORD for 32-bit mode)

const BOUNDS_ADDR: u64 = 0x2000;

fn write_bounds_32(mem: &vm_memory::GuestMemoryMmap, lower: i32, upper: i32) {
    mem.write_slice(&lower.to_le_bytes(), GuestAddress(BOUNDS_ADDR))
        .unwrap();
    mem.write_slice(&upper.to_le_bytes(), GuestAddress(BOUNDS_ADDR + 4))
        .unwrap();
}

// Helper to create BOUND EAX, [0x2000] with 32-bit operand size
fn bound_eax_code() -> Vec<u8> {
    vec![
        0x66, // Operand size prefix (16->32 bit)
        0x62, 0x05, // BOUND EAX, [disp32]
        0x00, 0x20, 0x00, 0x00, // disp32 = 0x2000
        0xf4, // HLT
    ]
}

// Helper to create BOUND with different registers
fn bound_reg_code(reg: u8) -> Vec<u8> {
    // ModRM: mod=00, reg=reg, rm=101 (disp32)
    let modrm = 0x05 | (reg << 3);
    vec![
        0x66, // Operand size prefix
        0x62, modrm, // BOUND reg, [disp32]
        0x00, 0x20, 0x00, 0x00, // disp32 = 0x2000
        0xf4, // HLT
    ]
}

// BOUND: Index equals lower bound (should succeed)
#[test]
fn test_bound_index_equals_lower_bound() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 0; // index = lower bound
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10); // bounds [0, 10]

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Index should remain unchanged");
}

// BOUND: Index within bounds (middle)
#[test]
fn test_bound_index_within_bounds() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 5; // index = 5, within bounds
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5, "Index within bounds accepted");
}

// BOUND: Index equals upper bound (should succeed)
#[test]
fn test_bound_index_equals_upper_bound() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 10; // index = upper bound
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 10, "Index at upper bound accepted");
}

// BOUND: Zero index with positive bounds
#[test]
fn test_bound_zero_index() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 255);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Zero index within bounds");
}

// BOUND: Test with EBX register
#[test]
fn test_bound_with_rbx() {
    let code = bound_reg_code(3); // reg=3 is EBX
    let mut regs = Registers::default();
    regs.rbx = 5;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFFFFFF, 5, "EBX index accepted");
}

// BOUND: Test with ECX register
#[test]
fn test_bound_with_rcx() {
    let code = bound_reg_code(1); // reg=1 is ECX
    let mut regs = Registers::default();
    regs.rcx = 7;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx & 0xFFFFFFFF, 7, "ECX index accepted");
}

// BOUND: Test with EDX register
#[test]
fn test_bound_with_rdx() {
    let code = bound_reg_code(2); // reg=2 is EDX
    let mut regs = Registers::default();
    regs.rdx = 3;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 15);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx & 0xFFFFFFFF, 3, "EDX index accepted");
}

// BOUND: Test with ESI register
#[test]
fn test_bound_with_rsi() {
    let code = bound_reg_code(6); // reg=6 is ESI
    let mut regs = Registers::default();
    regs.rsi = 8;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rsi & 0xFFFFFFFF, 8, "ESI index accepted");
}

// BOUND: Test with EDI register
#[test]
fn test_bound_with_rdi() {
    let code = bound_reg_code(7); // reg=7 is EDI
    let mut regs = Registers::default();
    regs.rdi = 2;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdi & 0xFFFFFFFF, 2, "EDI index accepted");
}

// BOUND: Same lower and upper bounds
#[test]
fn test_bound_same_bounds() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 5; // index = 5, bounds = [5, 5]
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 5, 5);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5, "Index equals both bounds");
}

// BOUND: Negative bounds
#[test]
fn test_bound_negative_bounds() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = (-5i32) as u64; // index = -5
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, -10, 0); // bounds [-10, 0]

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax as i32, -5, "Negative index within bounds");
}

// BOUND: Bounds crossing zero
#[test]
fn test_bound_bounds_crossing_zero() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 0;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, -5, 5);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "Zero within negative-to-positive bounds");
}

// BOUND: High bounds values
#[test]
fn test_bound_high_bounds_values() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFF00; // large positive value
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0x7FFFFF00u32 as i32, 0x7FFFFFFFu32 as i32);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x7FFFFF00, "High value within bounds");
}

// BOUND: 32-bit operands explicitly
#[test]
fn test_bound_32bit_operands() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 200);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 100, "32-bit bounds check passed");
}

// BOUND: Preserves flags
#[test]
fn test_bound_preserves_flags() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rsp = 0x8000;
    regs.rflags = 0x246; // Set some flags

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // BOUND should not modify flags
    assert_eq!(regs.rflags & 0x8D5, 0x246 & 0x8D5, "Flags preserved");
}

// BOUND: Preserves other registers
#[test]
fn test_bound_preserves_other_registers() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rbx = 0xDEADBEEF;
    regs.rcx = 0xCAFEBABE;
    regs.rdx = 0x12345678;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xDEADBEEF, "RBX preserved");
    assert_eq!(regs.rcx, 0xCAFEBABE, "RCX preserved");
    assert_eq!(regs.rdx, 0x12345678, "RDX preserved");
}

// BOUND: Memory bounds read from correct address
#[test]
fn test_bound_with_memory_bounds() {
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 50;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    // Write bounds at 0x2000
    write_bounds_32(&mem, 25, 75);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 50, "Memory bounds read correctly");
}

// BOUND: Practical array bounds example
#[test]
fn test_bound_practical_array_bounds() {
    // Simulating array[100] bounds check
    let code = bound_eax_code();
    let mut regs = Registers::default();
    regs.rax = 50; // index 50 of array[100]
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 99); // valid indices: 0-99

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 50, "Array bounds check passed");
}

// BOUND: Multiple sequential checks
#[test]
fn test_bound_multiple_sequential_checks() {
    // Two BOUND instructions in sequence
    let code = vec![
        0x66, 0x62, 0x05, 0x00, 0x20, 0x00, 0x00, // BOUND EAX, [0x2000]
        0x66, 0x62, 0x0D, 0x00, 0x20, 0x00, 0x00, // BOUND ECX, [0x2000]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rcx = 7;
    regs.rsp = 0x8000;

    let (mut vcpu, mem) = setup_vm_compat(&code, Some(regs));
    write_bounds_32(&mem, 0, 10);

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 5, "First BOUND passed");
    assert_eq!(regs.rcx, 7, "Second BOUND passed");
}
