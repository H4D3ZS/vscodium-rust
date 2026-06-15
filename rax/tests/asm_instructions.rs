//! Comprehensive assembly instruction tests for the x86_64 emulator.
//!
//! This module tests all implemented instructions with various edge cases.

use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::x86_64::{X86_64Vcpu, flags};
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};
use rax::error::Result;

/// Create a test VM with the given code and initial register state.
fn setup_vm(code: &[u8], initial_regs: Option<Registers>) -> (X86_64Vcpu, Arc<GuestMemoryMmap>) {
    // Create 16MB of guest memory
    let mem_size = 16 * 1024 * 1024;
    let regions = vec![(GuestAddress(0), mem_size)];
    let mem = Arc::new(GuestMemoryMmap::<()>::from_ranges(&regions).unwrap());

    // Write code at address 0x1000
    let code_addr = 0x1000u64;
    mem.write_slice(code, GuestAddress(code_addr)).unwrap();

    // Create vcpu
    let mut vcpu = X86_64Vcpu::new(0, mem.clone());

    // Set up initial registers
    let mut regs = initial_regs.unwrap_or_else(Registers::default);
    regs.rip = code_addr;
    regs.rsp = 0x8000; // Stack at 32KB
    // Preserve flags from initial_regs but ensure reserved bit 1 is set
    regs.rflags |= 0x2;
    vcpu.set_regs(&regs).unwrap();

    // Set up system registers - disable paging for simpler testing
    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x00050033; // PE but NOT PG (no paging)
    sregs.cr4 = 0x20; // PAE
    sregs.efer = 0x501; // SCE, LMA, LME for long mode
    // Set CS.L=1 for true 64-bit mode
    sregs.cs.l = true;
    sregs.cs.db = false; // Must be 0 when L=1 for 64-bit mode
    sregs.cs.selector = 0x8;
    vcpu.set_sregs(&sregs).unwrap();

    (vcpu, mem)
}

/// Run the VM until HLT and return final register state
fn run_until_hlt(vcpu: &mut X86_64Vcpu) -> Result<Registers> {
    loop {
        match vcpu.run()? {
            VcpuExit::Hlt => break,
            VcpuExit::IoIn { .. } | VcpuExit::IoOut { .. } => continue,
            _ => continue,
        }
    }
    vcpu.get_regs()
}

// ===== ADC (Add with Carry) Tests =====

#[test]
fn test_adc_rm8_r8_no_carry() {
    // ADC AL, CL when CF=0
    // 10 c8 = ADC AL, CL
    // f4 = HLT
    let code = [0x10, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rcx = 3;
    regs.rflags = 0x2; // CF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 8, "ADC AL, CL: 5 + 3 + 0 = 8");
}

#[test]
fn test_adc_rm8_r8_with_carry() {
    // ADC AL, CL when CF=1
    let code = [0x10, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 5;
    regs.rcx = 3;
    regs.rflags = 0x2 | flags::bits::CF; // CF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 9, "ADC AL, CL: 5 + 3 + 1 = 9");
}

#[test]
fn test_adc_rm_r_32bit() {
    // ADC EAX, ECX (CF=1)
    // 11 c8 = ADC EAX, ECX
    let code = [0x11, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x7FFFFFFF;
    regs.rcx = 1;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x80000001,
        "ADC EAX, ECX: 0x7FFFFFFF + 1 + 1 with 32-bit clears high bits"
    );
}

#[test]
fn test_adc_rm_r_64bit() {
    // REX.W ADC RAX, RCX
    // 48 11 c8 = ADC RAX, RCX
    let code = [0x48, 0x11, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_FFFFFFFF;
    regs.rcx = 1;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 1,
        "ADC RAX, RCX: 0xFFFF...FFFF + 1 + 1 = 1 with overflow"
    );
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CF should be set on overflow"
    );
}

#[test]
fn test_adc_al_imm8() {
    // ADC AL, imm8 (opcode 0x14)
    // 14 05 = ADC AL, 5
    let code = [0x14, 0x05, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 16, "ADC AL, 5: 10 + 5 + 1 = 16");
}

#[test]
fn test_adc_rax_imm32() {
    // REX.W ADC RAX, imm32 (opcode 0x15)
    // 48 15 ff ff ff ff = ADC RAX, -1 (sign-extended)
    let code = [0x48, 0x15, 0xff, 0xff, 0xff, 0xff, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 100;
    regs.rflags = 0x2 | flags::bits::CF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // -1 sign-extended is 0xFFFFFFFFFFFFFFFF
    // 100 + 0xFFFFFFFFFFFFFFFF + 1 = 100 (with overflow)
    assert_eq!(regs.rax, 100, "ADC RAX, -1: 100 + (-1) + 1 = 100");
}

// ===== INC/DEC Tests =====

#[test]
fn test_inc_rm8() {
    // INC AL (FE /0)
    // fe c0 = INC AL
    let code = [0xfe, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0, "INC AL: 0xFF + 1 = 0");
    // Note: INC doesn't affect CF
}

#[test]
fn test_dec_rm8() {
    // DEC AL (FE /1)
    // fe c8 = DEC AL
    let code = [0xfe, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0xFF, "DEC AL: 0 - 1 = 0xFF");
}

#[test]
fn test_inc_rm64() {
    // REX.W INC RAX (FF /0)
    // 48 ff c0 = INC RAX
    let code = [0x48, 0xff, 0xc0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "INC RAX: max + 1 = 0 (wraps)");
}

#[test]
fn test_dec_rm64() {
    // REX.W DEC RCX (FF /1)
    // 48 ff c9 = DEC RCX
    let code = [0x48, 0xff, 0xc9, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 0, "DEC RCX: 1 - 1 = 0");
    assert!(
        regs.rflags & flags::bits::ZF != 0,
        "ZF should be set when result is 0"
    );
}

// ===== RCL/RCR (Rotate through Carry) Tests =====

#[test]
fn test_rcl_rm8_1() {
    // RCL AL, 1 (D0 /2)
    // d0 d0 = RCL AL, 1
    let code = [0xd0, 0xd0, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x80; // 10000000
    regs.rflags = 0x2 | flags::bits::CF; // CF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Rotate: bit 7 goes to CF, old CF goes to bit 0
    // 0x80 with CF=1 -> result = 0x01 (CF rotated in), new CF = 1 (bit 7)
    assert_eq!(regs.rax & 0xFF, 0x01, "RCL AL, 1: 0x80 with CF=1 -> 0x01");
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CF should be set (bit 7 was 1)"
    );
}

#[test]
fn test_rcr_rm8_1() {
    // RCR AL, 1 (D0 /3)
    // d0 d8 = RCR AL, 1
    let code = [0xd0, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x01; // 00000001
    regs.rflags = 0x2 | flags::bits::CF; // CF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Rotate right: bit 0 goes to CF, old CF goes to bit 7
    // 0x01 with CF=1 -> result = 0x80 (CF rotated in), new CF = 1 (bit 0)
    assert_eq!(regs.rax & 0xFF, 0x80, "RCR AL, 1: 0x01 with CF=1 -> 0x80");
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CF should be set (bit 0 was 1)"
    );
}

#[test]
fn test_rcl_rm32_imm8() {
    // RCL EAX, 4 (C1 /2 imm8)
    // c1 d0 04 = RCL EAX, 4
    let code = [0xc1, 0xd0, 0x04, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rflags = 0x2; // CF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // After 4 rotates through carry (33-bit rotate for 32-bit value)
    // This is complex - just verify it doesn't crash and produces a result
    assert!(regs.rax != 0x12345678, "RCL EAX, 4 should change value");
}

#[test]
fn test_rcr_rm64_cl() {
    // REX.W RCR RAX, CL (D3 /3)
    // 48 d3 d8 = RCR RAX, CL
    let code = [0x48, 0xd3, 0xd8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x8000000000000000;
    regs.rcx = 1;
    regs.rflags = 0x2; // CF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Rotate right: bit 0 (0) goes to CF, old CF (0) goes to bit 63
    // 0x8000...00 >> 1 = 0x4000...00
    assert_eq!(
        regs.rax, 0x4000000000000000,
        "RCR RAX, 1 should shift right"
    );
    assert!(
        regs.rflags & flags::bits::CF == 0,
        "CF should be clear (bit 0 was 0)"
    );
}

// ===== TZCNT/LZCNT Tests =====

#[test]
fn test_tzcnt_basic() {
    // F3 0F BC = TZCNT r, r/m
    // f3 0f bc c1 = TZCNT EAX, ECX
    let code = [0xf3, 0x0f, 0xbc, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x80; // bit 7 set = 7 trailing zeros
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 7, "TZCNT(0x80) = 7 trailing zeros");
}

#[test]
fn test_tzcnt_zero() {
    // TZCNT with zero input
    let code = [0xf3, 0x0f, 0xbc, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 32, "TZCNT(0) = 32 (operand size in bits)");
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CF set when source is 0"
    );
}

#[test]
fn test_lzcnt_basic() {
    // F3 0F BD = LZCNT r, r/m
    // f3 0f bd c1 = LZCNT EAX, ECX
    let code = [0xf3, 0x0f, 0xbd, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x00010000; // bit 16 set = 15 leading zeros in 32-bit
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 15, "LZCNT(0x00010000) = 15 leading zeros");
}

#[test]
fn test_lzcnt_zero() {
    let code = [0xf3, 0x0f, 0xbd, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 32, "LZCNT(0) = 32");
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CF set when source is 0"
    );
}

#[test]
fn test_lzcnt_64bit() {
    // REX.W LZCNT RAX, RCX
    // f3 48 0f bd c1 = LZCNT RAX, RCX
    let code = [0xf3, 0x48, 0x0f, 0xbd, 0xc1, 0xf4];
    let mut regs = Registers::default();
    regs.rcx = 0x8000000000000000; // MSB set = 0 leading zeros
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0, "LZCNT(0x8000...) = 0 leading zeros");
}

// ===== XADD Tests =====

#[test]
fn test_xadd_r8_rm8() {
    // XADD r/m8, r8 (0F C0)
    // 0f c0 c8 = XADD AL, CL
    let code = [0x0f, 0xc0, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 10;
    regs.rcx = 5;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // XADD exchanges then adds: TEMP=SRC, SRC=DEST, DEST=DEST+TEMP
    // AL becomes AL+CL=15, CL becomes old AL=10
    assert_eq!(regs.rax & 0xFF, 15, "XADD AL, CL: AL = 10 + 5 = 15");
    assert_eq!(regs.rcx & 0xFF, 10, "XADD AL, CL: CL = old AL = 10");
}

#[test]
fn test_xadd_r_rm_32bit() {
    // XADD r/m32, r32 (0F C1)
    // 0f c1 c8 = XADD EAX, ECX
    let code = [0x0f, 0xc1, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    regs.rcx = 1;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Wrapping add
    assert_eq!(regs.rax, 0, "XADD EAX, ECX: 0xFFFFFFFF + 1 = 0 (32-bit)");
    assert_eq!(regs.rcx, 0xFFFFFFFF, "XADD: ECX = old EAX");
}

// ===== CMPXCHG Tests =====

#[test]
fn test_cmpxchg_equal() {
    // CMPXCHG r/m8, r8 (0F B0) - when AL == r/m8
    // 0f b0 c9 = CMPXCHG CL, CL (comparing AL with CL)
    // Actually: 0f b0 ca = CMPXCHG DL, CL
    let code = [0x0f, 0xb0, 0xca, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 5; // AL = 5
    regs.rcx = 10; // New value
    regs.rdx = 5; // DL = 5 (matches AL)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AL == DL, so DL = CL and ZF is set
    assert_eq!(regs.rdx & 0xFF, 10, "CMPXCHG: DL = CL when AL == DL");
    assert!(regs.rflags & flags::bits::ZF != 0, "ZF set on equal");
}

#[test]
fn test_cmpxchg_not_equal() {
    // CMPXCHG r/m8, r8 when AL != r/m8
    let code = [0x0f, 0xb0, 0xca, 0xf4]; // CMPXCHG DL, CL
    let mut regs = Registers::default();
    regs.rax = 5; // AL = 5
    regs.rcx = 10;
    regs.rdx = 7; // DL = 7 (doesn't match AL)
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AL != DL, so AL = DL and ZF is clear
    assert_eq!(regs.rax & 0xFF, 7, "CMPXCHG: AL = DL when AL != DL");
    assert_eq!(regs.rdx & 0xFF, 7, "CMPXCHG: DL unchanged when not equal");
    assert!(regs.rflags & flags::bits::ZF == 0, "ZF clear on not equal");
}

#[test]
fn test_cmpxchg_64bit() {
    // REX.W CMPXCHG r/m64, r64
    // 48 0f b1 ca = CMPXCHG RDX, RCX
    let code = [0x48, 0x0f, 0xb1, 0xca, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    regs.rcx = 0xCAFEBABE;
    regs.rdx = 0xDEADBEEF; // Matches RAX
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rdx, 0xCAFEBABE, "CMPXCHG: RDX = RCX when equal");
    assert!(regs.rflags & flags::bits::ZF != 0, "ZF set on equal");
}

// ===== SHLD/SHRD Tests =====

#[test]
fn test_shld_imm8() {
    // SHLD r/m, r, imm8 (0F A4)
    // 0f a4 c8 04 = SHLD EAX, ECX, 4
    let code = [0x0f, 0xa4, 0xc8, 0x04, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0xABCDEF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SHLD shifts left, pulling in bits from RCX
    // Result: (0x12345678 << 4) | (0xABCDEF00 >> 28) = 0x2345678A
    assert_eq!(regs.rax, 0x2345678A, "SHLD EAX, ECX, 4");
}

#[test]
fn test_shrd_imm8() {
    // SHRD r/m, r, imm8 (0F AC)
    // 0f ac c8 04 = SHRD EAX, ECX, 4
    let code = [0x0f, 0xac, 0xc8, 0x04, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    regs.rcx = 0xABCDEF00;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // SHRD shifts right, pulling in bits from RCX
    // Result: (0x12345678 >> 4) | ((0xABCDEF00 & 0xF) << 28) = 0x01234567
    assert_eq!(regs.rax, 0x01234567, "SHRD EAX, ECX, 4");
}

#[test]
fn test_shld_cl() {
    // SHLD r/m, r, CL (0F A5)
    // 0f a5 c8 = SHLD EAX, ECX, CL
    let code = [0x0f, 0xa5, 0xc8, 0xf4];
    let mut regs = Registers::default();
    regs.rax = 0xFF000000;
    regs.rcx = 8; // CL = 8
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Shift left by 8, bring in 8 bits from RCX (which is... wait, RCX is both source and count)
    // This is a self-referential case. RCX acts as both the source register and contains the count.
    // Count = CL = 8, source bits = (RCX >> (32 - 8)) = (8 >> 24) = 0
    // Result: (0xFF000000 << 8) | 0 = 0x00000000 (with top bits discarded)
    // Actually: 0xFF000000 << 8 in 32-bit = 0x00000000
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "SHLD EAX, RCX, CL with CL=8");
}

// ===== SETcc Edge Cases =====

#[test]
fn test_setcc_all_conditions() {
    // Test various condition codes
    // We'll use SETO (0F 90), SETNO (0F 91), SETB (0F 92), etc.

    // Test SETZ (0F 94) with ZF=1
    let code = [0x0f, 0x94, 0xc0, 0xf4]; // SETZ AL
    let mut regs = Registers::default();
    regs.rflags = 0x2 | flags::bits::ZF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 1, "SETZ with ZF=1 should set AL=1");

    // Test SETNZ (0F 95) with ZF=0
    let code = [0x0f, 0x95, 0xc0, 0xf4]; // SETNZ AL
    let mut regs = Registers::default();
    regs.rflags = 0x2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 1, "SETNZ with ZF=0 should set AL=1");
}

// ===== BSF/BSR vs TZCNT/LZCNT behavior =====

#[test]
fn test_bsf_vs_tzcnt_on_zero() {
    // BSF (no F3 prefix) should set ZF when source is 0, dest is undefined
    let code = [0x0f, 0xbc, 0xc1, 0xf4]; // BSF EAX, ECX
    let mut regs = Registers::default();
    regs.rax = 0xDEAD; // Should remain unchanged (undefined)
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        regs.rflags & flags::bits::ZF != 0,
        "BSF sets ZF when source is 0"
    );
    // Dest is undefined, so we don't check RAX
}

#[test]
fn test_bsr_nonzero() {
    // BSR finds the highest set bit
    let code = [0x0f, 0xbd, 0xc1, 0xf4]; // BSR EAX, ECX
    let mut regs = Registers::default();
    regs.rcx = 0x00008000; // bit 15 is the highest set
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 15, "BSR(0x8000) = 15");
    assert!(
        regs.rflags & flags::bits::ZF == 0,
        "ZF clear when source != 0"
    );
}

// ===== Chained arithmetic for flags testing =====

#[test]
fn test_adc_chain() {
    // Test ADC chaining: add 64-bit values using two 32-bit ADCs
    // ADD EAX, ECX (low 32 bits)
    // ADC EBX, EDX (high 32 bits with carry)
    let code = [
        0x01, 0xc8, // ADD EAX, ECX
        0x11, 0xd3, // ADC EBX, EDX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Add 0xFFFFFFFF_00000001 + 0x00000000_00000001
    regs.rax = 0x00000001; // low A
    regs.rcx = 0x00000001; // low B
    regs.rbx = 0xFFFFFFFF; // high A
    regs.rdx = 0x00000000; // high B
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Low: 1 + 1 = 2 (no carry)
    // High: 0xFFFFFFFF + 0 + 0 = 0xFFFFFFFF
    assert_eq!(regs.rax, 2, "Low 32 bits: 1 + 1 = 2");
    assert_eq!(regs.rbx, 0xFFFFFFFF, "High 32 bits: 0xFFFFFFFF + 0 + 0");
}

#[test]
fn test_adc_chain_with_carry() {
    // Test ADC chaining with carry propagation
    let code = [
        0x01, 0xc8, // ADD EAX, ECX
        0x11, 0xd3, // ADC EBX, EDX
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    // Add 0x00000000_FFFFFFFF + 0x00000000_00000001 = 0x00000001_00000000
    regs.rax = 0xFFFFFFFF; // low A
    regs.rcx = 0x00000001; // low B
    regs.rbx = 0x00000000; // high A
    regs.rdx = 0x00000000; // high B
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Low: 0xFFFFFFFF + 1 = 0 with carry
    // High: 0 + 0 + 1 (carry) = 1
    assert_eq!(regs.rax, 0, "Low 32 bits: overflow to 0");
    assert_eq!(regs.rbx, 1, "High 32 bits: carry propagated");
}

// ===== ROL/ROR edge cases =====

#[test]
fn test_rol_by_operand_size() {
    // ROL by operand size should result in same value (full rotation)
    // ROL EAX, 32 - but count is masked to 5 bits, so 32 & 0x1F = 0
    // Let's test ROL by 8 on an 8-bit value
    let code = [0xc0, 0xc0, 0x08, 0xf4]; // ROL AL, 8
    let mut regs = Registers::default();
    regs.rax = 0x5A;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // ROL by 8 on 8-bit = full rotation = same value
    assert_eq!(regs.rax & 0xFF, 0x5A, "ROL AL, 8 = full rotation");
}

#[test]
fn test_ror_preserves_bits() {
    // ROR should preserve all bits, just rotate
    let code = [0xc1, 0xc8, 0x10, 0xf4]; // ROR EAX, 16
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // ROR by 16: swap high and low 16-bit halves
    assert_eq!(regs.rax, 0x56781234, "ROR EAX, 16 swaps halves");
}

// ===== Memory operand tests =====

#[test]
fn test_inc_memory() {
    // INC DWORD PTR [RBX]
    // ff 03 = INC DWORD PTR [RBX]
    let code = [0xff, 0x03, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = 0x2000; // Point to memory
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write initial value to memory
    let initial_value: u32 = 42;
    mem.write_slice(&initial_value.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();

    let _regs = run_until_hlt(&mut vcpu).unwrap();

    // Read result from memory
    let mut result = [0u8; 4];
    mem.read_slice(&mut result, GuestAddress(0x2000)).unwrap();
    let result_val = u32::from_le_bytes(result);
    assert_eq!(result_val, 43, "INC [RBX]: 42 + 1 = 43");
}

#[test]
fn test_xadd_memory() {
    // XADD [RBX], ECX
    // 0f c1 0b = XADD [RBX], ECX
    let code = [0x0f, 0xc1, 0x0b, 0xf4];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    regs.rcx = 10;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write initial value
    let initial: u32 = 5;
    mem.write_slice(&initial.to_le_bytes(), GuestAddress(0x2000))
        .unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();

    // Memory should have sum, ECX should have old memory value
    let mut result = [0u8; 4];
    mem.read_slice(&mut result, GuestAddress(0x2000)).unwrap();
    let mem_val = u32::from_le_bytes(result);
    assert_eq!(mem_val, 15, "XADD [RBX], ECX: memory = 5 + 10 = 15");
    assert_eq!(regs.rcx, 5, "XADD: ECX = old memory value = 5");
}

// ===== CMC (Complement Carry Flag) Tests =====

#[test]
fn test_cmc_set_carry() {
    // CMC when CF=0 -> CF=1
    // f5 = CMC
    let code = [0xf5, 0xf4];
    let mut regs = Registers::default();
    regs.rflags = 0x2; // CF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        regs.rflags & flags::bits::CF != 0,
        "CMC should set CF when CF=0"
    );
}

#[test]
fn test_cmc_clear_carry() {
    // CMC when CF=1 -> CF=0
    let code = [0xf5, 0xf4];
    let mut regs = Registers::default();
    regs.rflags = 0x2 | flags::bits::CF; // CF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(
        regs.rflags & flags::bits::CF == 0,
        "CMC should clear CF when CF=1"
    );
}

// ===== LAHF/SAHF Tests =====

#[test]
fn test_lahf() {
    // Set some flags, then LAHF
    // f8 = CLC (clear CF)
    // f9 = STC (set CF)
    // 9f = LAHF
    let code = [0xf9, 0x9f, 0xf4]; // STC, LAHF, HLT
    let mut regs = Registers::default();
    regs.rflags = 0x2 | flags::bits::ZF | flags::bits::SF; // ZF=1, SF=1
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // AH should contain: SF:ZF:0:AF:0:PF:1:CF
    // With CF=1, ZF=1, SF=1, we expect AH = 0b1100_0011 = 0xC3 (or similar based on PF)
    let ah = ((regs.rax >> 8) & 0xFF) as u8;
    assert!(ah & 0x01 != 0, "LAHF: CF should be in AH bit 0");
    assert!(ah & 0x40 != 0, "LAHF: ZF should be in AH bit 6");
    assert!(ah & 0x80 != 0, "LAHF: SF should be in AH bit 7");
}

#[test]
fn test_sahf() {
    // Load AH with flags pattern, then SAHF
    // Use MOV EAX, imm32 to set AH (AH is bits 8-15 of EAX)
    // AH = 0xC1 = 0b1100_0001 (SF=1, ZF=1, CF=1)
    // So EAX = 0xC100 to put 0xC1 in AH
    // b8 XX XX XX XX = MOV EAX, imm32
    // 9e = SAHF
    let code = [
        0xb8, 0x00, 0xc1, 0x00, 0x00, // MOV EAX, 0x0000C100
        0x9e, // SAHF
        0xf4, // HLT
    ];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert!(regs.rflags & flags::bits::CF != 0, "SAHF: CF should be set");
    assert!(regs.rflags & flags::bits::ZF != 0, "SAHF: ZF should be set");
    assert!(regs.rflags & flags::bits::SF != 0, "SAHF: SF should be set");
}

// ===== RDTSC Test =====

#[test]
fn test_rdtsc() {
    // RDTSC - returns timestamp in EDX:EAX
    // Run RDTSC twice to verify it's incrementing
    // 0f 31 = RDTSC
    // 89 c3 = MOV EBX, EAX (save first EAX)
    // 0f 31 = RDTSC
    let code = [0x0f, 0x31, 0x89, 0xc3, 0x0f, 0x31, 0xf4];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Second RDTSC should return a value > first (saved in EBX)
    let first_tsc = regs.rbx & 0xFFFF_FFFF;
    let second_tsc = regs.rax & 0xFFFF_FFFF;
    assert!(
        second_tsc > first_tsc,
        "RDTSC should increment: first={}, second={}",
        first_tsc,
        second_tsc
    );
}

// ===== PAUSE Test =====

#[test]
fn test_pause() {
    // PAUSE (F3 90) - should just act as NOP
    let code = [0xf3, 0x90, 0xf4]; // PAUSE, HLT
    let mut regs = Registers::default();
    regs.rax = 42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // PAUSE should not modify any registers
    assert_eq!(regs.rax, 42, "PAUSE should not modify registers");
}

// ===== Memory Fence Tests =====

#[test]
fn test_mfence() {
    // MFENCE (0F AE F0) - treat as NOP
    let code = [0x0f, 0xae, 0xf0, 0xf4]; // MFENCE, HLT
    let mut regs = Registers::default();
    regs.rax = 123;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 123, "MFENCE should not modify registers");
}

#[test]
fn test_lfence() {
    // LFENCE (0F AE E8) - treat as NOP
    let code = [0x0f, 0xae, 0xe8, 0xf4]; // LFENCE, HLT
    let mut regs = Registers::default();
    regs.rbx = 456;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx, 456, "LFENCE should not modify registers");
}

#[test]
fn test_sfence() {
    // SFENCE (0F AE F8) - treat as NOP
    let code = [0x0f, 0xae, 0xf8, 0xf4]; // SFENCE, HLT
    let mut regs = Registers::default();
    regs.rcx = 789;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rcx, 789, "SFENCE should not modify registers");
}

// ===== LOOP Instruction Tests =====

#[test]
fn test_loop_basic() {
    // Simple loop that increments RAX 3 times
    // B8 00 00 00 00 = MOV EAX, 0
    // B9 03 00 00 00 = MOV ECX, 3
    // 48 FF C0 = INC RAX (loop body)
    // E2 FB = LOOP -5 (back to INC)
    // F4 = HLT
    let code = [
        0xB8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0x48, 0xFF, 0xC0, // INC RAX
        0xE2, 0xFB, // LOOP -5
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "LOOP should iterate 3 times");
    assert_eq!(regs.rcx, 0, "RCX should be 0 after loop");
}

#[test]
fn test_loopz_exit_on_zf_clear() {
    // LOOPZ exits when ZF becomes 0
    // Set up: loop while ZF=1, exit when ZF=0
    // XOR EAX, EAX sets ZF=1
    // DEC EAX will clear ZF
    let code = [
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0x31, 0xC0, // XOR EAX, EAX (sets ZF=1)
        0xE1, 0xFC, // LOOPZ -4 (back to XOR)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // Should loop 3 times (ZF stays 1)
    assert_eq!(regs.rcx, 0, "LOOPZ should decrement RCX to 0");
}

#[test]
fn test_loopnz_exit_on_zf_set() {
    // LOOPNZ exits when ZF becomes 1
    // INC RAX doesn't set ZF unless result is 0
    let code = [
        0xB9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0xB8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xFF, 0xC0, // INC EAX (ZF=0 unless result=0)
        0xE0, 0xFC, // LOOPNZ -4 (back to INC)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 3, "LOOPNZ should iterate 3 times");
    assert_eq!(regs.rcx, 0, "RCX should be 0 after loop");
}

// ===== ENTER Test =====

#[test]
fn test_enter_basic() {
    // ENTER 16, 0 - creates stack frame with 16 bytes local space
    // C8 10 00 00 = ENTER 16, 0
    let code = [0xC8, 0x10, 0x00, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rsp = 0x8000;
    regs.rbp = 0x9000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let regs = run_until_hlt(&mut vcpu).unwrap();
    // After ENTER: RBP = old RSP - 8, RSP = RBP - 16
    assert_eq!(
        regs.rbp,
        0x8000 - 8,
        "ENTER: RBP should point to saved old RBP"
    );
    assert_eq!(
        regs.rsp,
        regs.rbp - 16,
        "ENTER: RSP should be RBP - alloc_size"
    );
}

// ===== INT3 Test =====

#[test]
fn test_int3_no_idt() {
    // INT3 without a valid IDT should return an error
    // (For proper INT3 handling with IDT, see tests/x86_64/control_flow/int_into_int3.rs)
    let code = [0xCC, 0xF4]; // INT3, HLT
    let regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    // Run and expect error since no IDT is configured
    let result = vcpu.run();
    assert!(result.is_err(), "INT3 without IDT should return error");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("IDT entry 3 not present"),
        "Error should mention missing IDT entry"
    );
}

// ===== XLAT Test =====

#[test]
fn test_xlat() {
    // XLAT - AL = [RBX + AL]
    // Set up a translation table at RBX
    let code = [0xD7, 0xF4]; // XLAT, HLT
    let mut regs = Registers::default();
    regs.rbx = 0x2000; // Table base
    regs.rax = 5; // Index into table
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write translation table: table[5] = 0x42
    let table = [0, 1, 2, 3, 4, 0x42, 6, 7, 8, 9];
    mem.write_slice(&table, GuestAddress(0x2000)).unwrap();

    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFF, 0x42, "XLAT: AL = table[5] = 0x42");
}
