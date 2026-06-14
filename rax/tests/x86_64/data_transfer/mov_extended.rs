// Module path for tests run via x86_64.rs
use crate::common::*;
use rax::cpu::Registers;

// MOV - Move Data (Comprehensive Extended Tests)
// Comprehensive test coverage for MOV instruction with all operand combinations
// including register-to-register, immediate-to-register, memory operands,
// and extended registers R8-R15.
//
// Opcodes:
// 88 /r        MOV r/m8, r8     - Move r8 to r/m8
// 89 /r        MOV r/m16/32/64, r16/32/64 - Move r16/32/64 to r/m16/32/64
// 8A /r        MOV r8, r/m8     - Move r/m8 to r8
// 8B /r        MOV r16/32/64, r/m16/32/64 - Move r/m16/32/64 to r16/32/64
// B0+rb ib     MOV r8, imm8     - Move imm8 to r8
// B8+rd id     MOV r32, imm32   - Move imm32 to r32
// C6 /0 ib     MOV r/m8, imm8   - Move imm8 to r/m8
// C7 /0 id     MOV r/m32, imm32 - Move imm32 to r/m32

#[test]
fn test_mov_rax_to_rbx() {
    // MOV RBX, RAX - 64-bit register to register
    let code = [
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_eax_to_ebx() {
    // MOV EBX, EAX - 32-bit register to register (zero extends)
    let code = [
        0x89, 0xc3, // MOV EBX, EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF12345678;
    regs.rbx = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0x0000000012345678); // Zero extended
}

#[test]
fn test_mov_ax_to_bx() {
    // MOV BX, AX - 16-bit register to register
    let code = [
        0x66, 0x89, 0xc3, // MOV BX, AX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF, 0x1234);
    assert_eq!(regs.rbx & 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFFFF0000); // Upper bits preserved
}

#[test]
fn test_mov_al_to_bl() {
    // MOV BL, AL - 8-bit register to register
    let code = [
        0x88, 0xc3, // MOV BL, AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    regs.rbx = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFF, 0x42);
    assert_eq!(regs.rbx & 0xFFFFFFFFFFFFFF00, 0xFFFFFFFFFFFFFF00); // Upper bits preserved
}

#[test]
fn test_mov_imm64_to_rax() {
    // MOV RAX, imm64 - 64-bit immediate to register
    let code = [
        0x48, 0xb8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, // MOV RAX, 0x8877665544332211
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8877665544332211);
}

#[test]
fn test_mov_imm32_to_eax() {
    // MOV EAX, imm32 - 32-bit immediate to register
    let code = [
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000000012345678); // Zero extended
}

#[test]
fn test_mov_imm16_to_ax() {
    // MOV AX, imm16 - 16-bit immediate to register
    let code = [
        0x66, 0xb8, 0x34, 0x12, // MOV AX, 0x1234
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1234);
    assert_eq!(regs.rax & 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFFFF0000); // Upper bits preserved
}

#[test]
fn test_mov_imm8_to_al() {
    // MOV AL, imm8 - 8-bit immediate to register
    let code = [
        0xb0, 0x42, // MOV AL, 0x42
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x42);
    assert_eq!(regs.rax & 0xFFFFFFFFFFFFFF00, 0xFFFFFFFFFFFFFF00); // Upper bits preserved
}

#[test]
fn test_mov_r8_to_r9() {
    // MOV R9, R8 - Extended register to extended register
    let code = [
        0x4d, 0x89, 0xc1, // MOV R9, R8
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xDEADBEEFCAFEBABE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r9, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_mov_r15_to_rax() {
    // MOV RAX, R15 - Extended register to normal register
    let code = [
        0x4c, 0x89, 0xf8, // MOV RAX, R15
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_rax_to_r15() {
    // MOV R15, RAX - Normal register to extended register
    let code = [
        0x49, 0x89, 0xc7, // MOV R15, RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFEDCBA9876543210;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r15, 0xFEDCBA9876543210);
}

#[test]
fn test_mov_r8d_to_r9d() {
    // MOV R9D, R8D - 32-bit extended registers
    let code = [
        0x45, 0x89, 0xc1, // MOV R9D, R8D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0xFFFFFFFF12345678;
    regs.r9 = 0xAAAAAAAAAAAAAAAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r9, 0x0000000012345678); // Zero extended
}

#[test]
fn test_mov_r8w_to_r9w() {
    // MOV R9W, R8W - 16-bit extended registers
    let code = [
        0x66, 0x45, 0x89, 0xc1, // MOV R9W, R8W
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x1234;
    regs.r9 = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r9 & 0xFFFF, 0x1234);
    assert_eq!(regs.r9 & 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFFFF0000); // Upper bits preserved
}

#[test]
fn test_mov_r8b_to_r9b() {
    // MOV R9B, R8B - 8-bit extended registers (low byte)
    let code = [
        0x45, 0x88, 0xc1, // MOV R9B, R8B
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r8 = 0x42;
    regs.r9 = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r9 & 0xFF, 0x42);
    assert_eq!(regs.r9 & 0xFFFFFFFFFFFFFF00, 0xFFFFFFFFFFFFFF00); // Upper bits preserved
}

#[test]
fn test_mov_to_memory_64bit() {
    // MOV [addr], RAX - 64-bit register to memory
    let code = [
        0x48, 0xa3, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u64(&mem);
    assert_eq!(value, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_from_memory_64bit() {
    // MOV RAX, [addr] - 64-bit memory to register
    let code = [
        0x48, 0xa1, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u64(&mem, 0xFEDCBA9876543210);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDCBA9876543210);
}

#[test]
fn test_mov_to_memory_32bit() {
    // MOV [addr], EAX - 32-bit register to memory
    let code = [
        0xa3, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], EAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u32(&mem);
    assert_eq!(value, 0x12345678);
}

#[test]
fn test_mov_from_memory_32bit() {
    // MOV EAX, [addr] - 32-bit memory to register
    let code = [
        0xa1, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV EAX, [0x2000]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_u32(&mem, 0xDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x00000000DEADBEEF); // Zero extended
}

#[test]
fn test_mov_to_memory_16bit() {
    // MOV [addr], AX - 16-bit register to memory
    let code = [
        0x66, 0xa3, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], AX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u16(&mem);
    assert_eq!(value, 0x1234);
}

#[test]
fn test_mov_from_memory_16bit() {
    // MOV AX, [addr] - 16-bit memory to register
    let code = [
        0x66, 0xa1, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV AX, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u16(&mem, 0xABCD);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xABCD);
    assert_eq!(regs.rax & 0xFFFFFFFFFFFF0000, 0xFFFFFFFFFFFF0000); // Upper bits preserved
}

#[test]
fn test_mov_to_memory_8bit() {
    // MOV [addr], AL - 8-bit register to memory
    let code = [
        0xa2, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV [0x2000], AL
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u8(&mem);
    assert_eq!(value, 0x42);
}

#[test]
fn test_mov_from_memory_8bit() {
    // MOV AL, [addr] - 8-bit memory to register
    let code = [
        0xa0, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV AL, [0x2000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u8(&mem, 0x99);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x99);
    assert_eq!(regs.rax & 0xFFFFFFFFFFFFFF00, 0xFFFFFFFFFFFFFF00); // Upper bits preserved
}

#[test]
fn test_mov_indirect_rbx() {
    // MOV RAX, [RBX] - Indirect addressing
    let code = [
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_indirect_with_displacement() {
    // MOV RAX, [RBX+16] - Indirect with displacement
    let code = [
        0x48, 0x8b, 0x43, 0x10, // MOV RAX, [RBX+16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_mov_sib_addressing() {
    // MOV RAX, [RBX+RCX*4] - SIB addressing
    let code = [
        0x48, 0x8b, 0x04, 0x8b, // MOV RAX, [RBX+RCX*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 4;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0x1122334455667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1122334455667788);
}

#[test]
fn test_mov_sib_with_displacement() {
    // MOV RAX, [RBX+RCX*8+32] - SIB with displacement
    let code = [
        0x48, 0x8b, 0x44, 0xcb, 0x20, // MOV RAX, [RBX+RCX*8+32]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = 2;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16 + 32, 0xAABBCCDDEEFF0011);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xAABBCCDDEEFF0011);
}

#[test]
fn test_mov_rip_relative() {
    // MOV RAX, [RIP+offset] - RIP-relative addressing
    let code = [
        0x48, 0x8b, 0x05, 0x01, 0x00, 0x00, 0x00, // MOV RAX, [RIP+1]
        0xf4, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, // Data
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x8877665544332211); // Little-endian
}

#[test]
fn test_mov_zero_value() {
    // MOV RAX, 0 - Move zero
    let code = [
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFFFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0);
}

#[test]
fn test_mov_all_ones_64bit() {
    // MOV RAX, -1 - Move all ones (64-bit)
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1 (sign extended)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_mov_negative_immediate() {
    // MOV EAX, -100 - Negative immediate
    let code = [
        0xb8, 0x9c, 0xff, 0xff, 0xff, // MOV EAX, -100
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax as u32, 0xFFFFFF9C); // -100 in two's complement
}

#[test]
fn test_mov_preserves_flags() {
    // MOV should not affect flags
    let code = [
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    regs.rflags = 0x246; // Some flags set
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rflags, initial_flags);
}

#[test]
fn test_mov_chain() {
    // Chain of MOV operations
    let code = [
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x48, 0x89, 0xd9, // MOV RCX, RBX
        0x48, 0x89, 0xca, // MOV RDX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEFCAFEBABE;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 0xDEADBEEFCAFEBABE);
    assert_eq!(regs.rcx, 0xDEADBEEFCAFEBABE);
    assert_eq!(regs.rdx, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_mov_all_gprs() {
    // Test MOV with all general purpose registers
    let code = [
        0x48, 0xb8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xbb, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xb9, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x48, 0xba, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 4
        0x48, 0xbe, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 5
        0x48, 0xbf, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 6
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 1);
    assert_eq!(regs.rbx, 2);
    assert_eq!(regs.rcx, 3);
    assert_eq!(regs.rdx, 4);
    assert_eq!(regs.rsi, 5);
    assert_eq!(regs.rdi, 6);
}

#[test]
fn test_mov_all_extended_regs() {
    // Test MOV with all extended registers R8-R15
    let code = [
        0x49, 0xb8, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 8
        0x49, 0xb9, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R9, 9
        0x49, 0xba, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 10
        0x49, 0xbb, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R11, 11
        0x49, 0xbc, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R12, 12
        0x49, 0xbd, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R13, 13
        0x49, 0xbe, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R14, 14
        0x49, 0xbf, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R15, 15
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r8, 8);
    assert_eq!(regs.r9, 9);
    assert_eq!(regs.r10, 10);
    assert_eq!(regs.r11, 11);
    assert_eq!(regs.r12, 12);
    assert_eq!(regs.r13, 13);
    assert_eq!(regs.r14, 14);
    assert_eq!(regs.r15, 15);
}

#[test]
fn test_mov_memory_to_memory_via_reg() {
    // MOV can't do memory-to-memory directly, use register as intermediary
    let code = [
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0x48, 0x89, 0x01, // MOV [RCX], RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    regs.rcx = DATA_ADDR + 16;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_at_u64(&mem, DATA_ADDR + 16);
    assert_eq!(value, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_self_assignment() {
    // MOV RAX, RAX - Self assignment (should be a no-op but valid)
    let code = [
        0x48, 0x89, 0xc0, // MOV RAX, RAX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x123456789ABCDEF0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_swap_via_third() {
    // Swap RAX and RBX using RCX as temporary
    let code = [
        0x48, 0x89, 0xc1, // MOV RCX, RAX
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0x48, 0x89, 0xcb, // MOV RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2222222222222222);
    assert_eq!(regs.rbx, 0x1111111111111111);
}

#[test]
fn test_mov_sign_bit_patterns() {
    // Test with sign bit set in various sizes
    let code = [
        0xb0, 0x80, // MOV AL, 0x80 (sign bit set in byte)
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x80);
}

#[test]
fn test_mov_boundary_values_8bit() {
    // Test boundary values for 8-bit operations
    let test_cases = vec![0x00, 0x01, 0x7F, 0x80, 0xFF];

    for value in test_cases {
        let code = [
            0xb0, value, // MOV AL, imm8
            0xf4,
        ];
        let (mut vcpu, _) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rax & 0xFF, value as u64);
    }
}

#[test]
fn test_mov_boundary_values_16bit() {
    // Test boundary values for 16-bit operations
    let test_cases = vec![
        (0x00, 0x00, 0x0000u16),
        (0x01, 0x00, 0x0001u16),
        (0xff, 0x7f, 0x7FFFu16),
        (0x00, 0x80, 0x8000u16),
        (0xff, 0xff, 0xFFFFu16),
    ];

    for (low, high, expected) in test_cases {
        let code = [
            0x66, 0xb8, low, high, // MOV AX, imm16
            0xf4,
        ];
        let (mut vcpu, _) = setup_vm(&code, None);
        let regs = run_until_hlt(&mut vcpu).unwrap();
        assert_eq!(regs.rax & 0xFFFF, expected as u64);
    }
}

#[test]
fn test_mov_with_base_pointer() {
    // MOV using RBP as base (common in stack frames)
    let code = [
        0x48, 0x8b, 0x45, 0x08, // MOV RAX, [RBP+8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbp = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 8, 0xFEDCBA9876543210);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xFEDCBA9876543210);
}

#[test]
fn test_mov_with_stack_pointer() {
    // MOV using RSP as base
    let code = [
        0x48, 0x8b, 0x04, 0x24, // MOV RAX, [RSP]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x123456789ABCDEF0);
}

#[test]
fn test_mov_negative_displacement() {
    // MOV with negative displacement
    let code = [
        0x48, 0x8b, 0x43, 0xf0, // MOV RAX, [RBX-16]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR + 32;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 16, 0xAABBCCDDEEFF0011);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xAABBCCDDEEFF0011);
}

#[test]
fn test_mov_large_displacement() {
    // MOV with large displacement (32-bit)
    let code = [
        0x48, 0x8b, 0x83, 0x00, 0x10, 0x00, 0x00, // MOV RAX, [RBX+0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_at_u64(&mem, DATA_ADDR + 0x1000, 0x1122334455667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1122334455667788);
}

#[test]
fn test_mov_imm_to_memory_8bit() {
    // MOV BYTE PTR [addr], imm8
    let code = [
        0xc6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x42, // MOV BYTE PTR [0x2000], 0x42
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u8(&mem);
    assert_eq!(value, 0x42);
}

#[test]
fn test_mov_imm_to_memory_32bit() {
    // MOV DWORD PTR [addr], imm32
    let code = [
        0xc7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x78, 0x56, 0x34,
        0x12, // MOV DWORD PTR [0x2000], 0x12345678
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let value = read_mem_u32(&mem);
    assert_eq!(value, 0x12345678);
}

#[test]
fn test_mov_r12_addressing() {
    // MOV with R12 (requires SIB byte)
    let code = [
        0x4c, 0x8b, 0x24, 0x24, // MOV R12, [RSP]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, STACK_ADDR, 0xDEADBEEFCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r12, 0xDEADBEEFCAFEBABE);
}

#[test]
fn test_mov_r13_with_displacement() {
    // MOV with R13 (requires displacement even if zero)
    let code = [
        0x4d, 0x8b, 0x6d, 0x00, // MOV R13, [R13+0]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r13 = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));
    write_mem_u64(&mem, 0x123456789ABCDEF0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.r13, 0x123456789ABCDEF0);
}
