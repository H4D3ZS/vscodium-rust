use crate::common::TestCase;
use crate::common::*;
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// BNDCL - Check Lower Bound
// Compares address with lower bound in bounds register

#[test]
fn test_bndcl_r32() {
    TestCase::from("f3 0f 1a 08").check();
}

#[test]
fn test_bndcl_r64() {
    TestCase::from("f3 0f 1a 08").check();
}

#[test]
fn test_bndcl_bnd0_eax() {
    TestCase::from("f3 0f 1a 00").check();
}

#[test]
fn test_bndcl_bnd1_ecx() {
    TestCase::from("f3 0f 1a 09").check();
}

#[test]
fn test_bndcl_bnd2_edx() {
    TestCase::from("f3 0f 1a 12").check();
}

#[test]
fn test_bndcl_bnd3_ebx() {
    TestCase::from("f3 0f 1a 1b").check();
}

#[test]
fn test_bndcl_bnd0_mem32() {
    TestCase::from("f3 0f 1a 00").check();
}

#[test]
fn test_bndcl_bnd1_mem32() {
    TestCase::from("f3 0f 1a 09").check();
}

#[test]
fn test_bndcl_bnd0_rax() {
    TestCase::from("f3 48 0f 1a 00").check();
}

#[test]
fn test_bndcl_bnd1_rcx() {
    TestCase::from("f3 48 0f 1a 09").check();
}

#[test]
fn test_bndcl_bnd2_rdx() {
    TestCase::from("f3 48 0f 1a 12").check();
}

#[test]
fn test_bndcl_bnd3_rbx() {
    TestCase::from("f3 48 0f 1a 1b").check();
}

// BNDCU/BNDCN - Check Upper Bound

#[test]
fn test_bndcu_r32() {
    TestCase::from("f2 0f 1a 08").check();
}

#[test]
fn test_bndcu_r64() {
    TestCase::from("f2 0f 1a 08").check();
}

#[test]
fn test_bndcu_bnd0_eax() {
    TestCase::from("f2 0f 1a 00").check();
}

#[test]
fn test_bndcu_bnd1_ecx() {
    TestCase::from("f2 0f 1a 09").check();
}

#[test]
fn test_bndcu_bnd2_edx() {
    TestCase::from("f2 0f 1a 12").check();
}

#[test]
fn test_bndcu_bnd3_ebx() {
    TestCase::from("f2 0f 1a 1b").check();
}

#[test]
fn test_bndcn_bnd0_rax() {
    TestCase::from("f2 48 0f 1a 00").check();
}

#[test]
fn test_bndcn_bnd1_rcx() {
    TestCase::from("f2 48 0f 1a 09").check();
}

#[test]
fn test_bndcn_bnd2_rdx() {
    TestCase::from("f2 48 0f 1a 12").check();
}

#[test]
fn test_bndcn_bnd3_rbx() {
    TestCase::from("f2 48 0f 1a 1b").check();
}

// BNDMK - Make Bounds
// Creates bounds from memory operand

#[test]
fn test_bndmk_bnd0_m32() {
    TestCase::from("f3 0f 1b 00").check();
}

#[test]
fn test_bndmk_bnd1_m32() {
    TestCase::from("f3 0f 1b 08").check();
}

#[test]
fn test_bndmk_bnd2_m32() {
    TestCase::from("f3 0f 1b 10").check();
}

#[test]
fn test_bndmk_bnd3_m32() {
    TestCase::from("f3 0f 1b 18").check();
}

#[test]
fn test_bndmk_bnd0_m64() {
    TestCase::from("f3 48 0f 1b 00").check();
}

#[test]
fn test_bndmk_bnd1_m64() {
    TestCase::from("f3 48 0f 1b 08").check();
}

#[test]
fn test_bndmk_bnd2_m64() {
    TestCase::from("f3 48 0f 1b 10").check();
}

#[test]
fn test_bndmk_bnd3_m64() {
    TestCase::from("f3 48 0f 1b 18").check();
}

#[test]
fn test_bndmk_bnd0_rax_base() {
    TestCase::from("f3 48 0f 1b 00").check();
}

#[test]
fn test_bndmk_bnd1_rcx_base() {
    TestCase::from("f3 48 0f 1b 09").check();
}

#[test]
fn test_bndmk_bnd2_rdx_base() {
    TestCase::from("f3 48 0f 1b 12").check();
}

#[test]
fn test_bndmk_bnd3_rbx_base() {
    TestCase::from("f3 48 0f 1b 1b").check();
}

// BNDMOV - Move Bounds
// Moves bounds between bound registers or memory

#[test]
fn test_bndmov_bnd0_bnd1() {
    TestCase::from("66 0f 1a c1").check();
}

#[test]
fn test_bndmov_bnd1_bnd0() {
    TestCase::from("66 0f 1a c8").check();
}

#[test]
fn test_bndmov_bnd2_bnd3() {
    TestCase::from("66 0f 1a d3").check();
}

#[test]
fn test_bndmov_bnd3_bnd2() {
    TestCase::from("66 0f 1a da").check();
}

#[test]
fn test_bndmov_bnd0_m64() {
    TestCase::from("66 0f 1a 00").check();
}

#[test]
fn test_bndmov_bnd1_m64() {
    TestCase::from("66 0f 1a 08").check();
}

#[test]
fn test_bndmov_bnd2_m64() {
    TestCase::from("66 0f 1a 10").check();
}

#[test]
fn test_bndmov_bnd3_m64() {
    TestCase::from("66 0f 1a 18").check();
}

#[test]
fn test_bndmov_m64_bnd0() {
    TestCase::from("66 0f 1b 00").check();
}

#[test]
fn test_bndmov_m64_bnd1() {
    TestCase::from("66 0f 1b 08").check();
}

#[test]
fn test_bndmov_m64_bnd2() {
    TestCase::from("66 0f 1b 10").check();
}

#[test]
fn test_bndmov_m64_bnd3() {
    TestCase::from("66 0f 1b 18").check();
}

#[test]
fn test_bndmov_bnd0_m128() {
    TestCase::from("66 48 0f 1a 00").check();
}

#[test]
fn test_bndmov_bnd1_m128() {
    TestCase::from("66 48 0f 1a 08").check();
}

#[test]
fn test_bndmov_m128_bnd0() {
    TestCase::from("66 48 0f 1b 00").check();
}

#[test]
fn test_bndmov_m128_bnd1() {
    TestCase::from("66 48 0f 1b 08").check();
}

// BNDLDX - Load Extended Bounds Using Address Translation

#[test]
fn test_bndldx_bnd0() {
    TestCase::from("0f 1a 00").check();
}

#[test]
fn test_bndldx_bnd1() {
    TestCase::from("0f 1a 08").check();
}

#[test]
fn test_bndldx_bnd2() {
    TestCase::from("0f 1a 10").check();
}

#[test]
fn test_bndldx_bnd3() {
    TestCase::from("0f 1a 18").check();
}

#[test]
fn test_bndldx_bnd0_rax_rbx() {
    TestCase::from("0f 1a 04 18").check();
}

#[test]
fn test_bndldx_bnd1_rcx_rdx() {
    TestCase::from("0f 1a 0c 11").check();
}

#[test]
fn test_bndldx_bnd2_rsi_rdi() {
    TestCase::from("0f 1a 14 3e").check();
}

#[test]
fn test_bndldx_bnd3_r8_r9() {
    TestCase::from("44 0f 1a 1c 08").check();
}

// BNDSTX - Store Extended Bounds Using Address Translation

#[test]
fn test_bndstx_bnd0() {
    TestCase::from("0f 1b 00").check();
}

#[test]
fn test_bndstx_bnd1() {
    TestCase::from("0f 1b 08").check();
}

#[test]
fn test_bndstx_bnd2() {
    TestCase::from("0f 1b 10").check();
}

#[test]
fn test_bndstx_bnd3() {
    TestCase::from("0f 1b 18").check();
}

#[test]
fn test_bndstx_rax_rbx_bnd0() {
    TestCase::from("0f 1b 04 18").check();
}

#[test]
fn test_bndstx_rcx_rdx_bnd1() {
    TestCase::from("0f 1b 0c 11").check();
}

#[test]
fn test_bndstx_rsi_rdi_bnd2() {
    TestCase::from("0f 1b 14 3e").check();
}

#[test]
fn test_bndstx_r8_r9_bnd3() {
    TestCase::from("44 0f 1b 1c 08").check();
}

// ============================================================================
// Value/flag-asserting memory-addressing tests.
//
// These exercise the effective-address computation through LEA (which writes
// the computed EA directly into the destination register, so the *exact*
// address can be asserted) and through MOV with complex SIB encodings,
// non-temporal stores (MOVNTI/MOVNTDQ), every memory-operand size, the 0x67
// 32-bit-address override, misaligned accesses and page-crossing accesses.
//
// All values are computed by hand. Code is loaded at CODE_ADDR (0x1000) so any
// RIP-relative computation is anchored there. Helpers (setup_vm, read_mem_*,
// set_xmm, get_xmm, ...) come from crate::common.
// ============================================================================

// ----- LEA: every addressing form, exact effective address in dest -----

// LEA r64, [base] -- pure base register.
#[test]
fn test_lea_base_only() {
    let code = [
        0x48, 0x8d, 0x03, // LEA RAX, [RBX]
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_1234_5678_9AB0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_1234_5678_9AB0, "LEA [base] = RBX");
}

// LEA r64, [base + index] -- SIB scale 1.
#[test]
fn test_lea_base_plus_index() {
    let code = [
        0x48, 0x8d, 0x04, 0x0b, // LEA RAX, [RBX + RCX]  (SIB: scale=0 index=001 base=011)
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x4000;
    regs.rcx = 0x0123;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x4123, "LEA [RBX + RCX] = 0x4000 + 0x0123");
}

// LEA r64, [base + index*scale] for scale 2/4/8.
#[test]
fn test_lea_base_index_scale2() {
    let code = [
        0x48, 0x8d, 0x04, 0x4b, // LEA RAX, [RBX + RCX*2]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1000;
    regs.rcx = 0x0080;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1000 + 0x0080 * 2, "LEA [RBX + RCX*2]");
}

#[test]
fn test_lea_base_index_scale4() {
    let code = [
        0x48, 0x8d, 0x04, 0x8b, // LEA RAX, [RBX + RCX*4]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x2000;
    regs.rcx = 0x0040;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x2000 + 0x0040 * 4, "LEA [RBX + RCX*4]");
}

#[test]
fn test_lea_base_index_scale8() {
    let code = [
        0x48, 0x8d, 0x04, 0xcb, // LEA RAX, [RBX + RCX*8]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x3000;
    regs.rcx = 0x0011;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x3000 + 0x0011 * 8, "LEA [RBX + RCX*8]");
}

// LEA r64, [base + disp8].
#[test]
fn test_lea_base_disp8() {
    let code = [
        0x48, 0x8d, 0x43, 0x7f, // LEA RAX, [RBX + 0x7F]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x507F, "LEA [RBX + 0x7F]");
}

// LEA r64, [base + negative disp8] -- sign extension of disp8.
#[test]
fn test_lea_base_disp8_negative() {
    let code = [
        0x48, 0x8d, 0x43, 0xf8, // LEA RAX, [RBX - 8]  (disp8 = 0xF8 = -8)
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x5000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x4FF8, "LEA [RBX - 8] sign-extends disp8");
}

// LEA r64, [base + disp32].
#[test]
fn test_lea_base_disp32() {
    let code = [
        0x48, 0x8d, 0x83, 0x00, 0x00, 0x10, 0x00, // LEA RAX, [RBX + 0x100000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0001_0000_0000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x0000_0001_0010_0000, "LEA [RBX + disp32]");
}

// LEA r64, [base + index*scale + disp32] -- full SIB + disp32.
#[test]
fn test_lea_base_index_scale_disp32() {
    let code = [
        0x48, 0x8d, 0x84, 0x8b, 0x00, 0x10, 0x00, 0x00, // LEA RAX, [RBX + RCX*4 + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x6000;
    regs.rcx = 0x0010;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax,
        0x6000 + 0x0010 * 4 + 0x1000,
        "LEA [RBX + RCX*4 + 0x1000]"
    );
}

// LEA r64, [RIP + disp32] -- 64-bit-mode RIP-relative.
// Instruction at 0x1000: 48 8D 05 <disp32>. The ModR/M is at instruction
// offset 2, so RIP-after = 0x1000 + 2 + 1 + 4 = 0x1007. With disp = 0x100 the
// effective address is 0x1107.
#[test]
fn test_lea_rip_relative() {
    let code = [
        0x48, 0x8d, 0x05, 0x00, 0x01, 0x00, 0x00, // LEA RAX, [RIP + 0x100]
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x1107,
        "LEA [RIP + 0x100] = next_rip(0x1007) + 0x100"
    );
}

// LEA r64, [RIP - disp32] -- negative RIP-relative displacement.
#[test]
fn test_lea_rip_relative_negative() {
    let code = [
        0x48, 0x8d, 0x05, 0xf9, 0xff, 0xff, 0xff, // LEA RAX, [RIP - 7]
        0xf4,
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // next_rip = 0x1007, disp = -7 -> 0x1000 (start of this instruction).
    assert_eq!(regs.rax, 0x1000, "LEA [RIP - 7] = next_rip(0x1007) - 7");
}

// LEA EAX (32-bit dest) truncates the computed 64-bit address to 32 bits.
#[test]
fn test_lea_32bit_dest_truncates() {
    let code = [
        0x8d, 0x43, 0x10, // LEA EAX, [RBX + 0x10]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF_FFFF_FFFF_FFFF; // dirty upper bits
    regs.rbx = 0x0000_0002_0000_1000;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 64-bit EA = 0x2_0000_1010; 32-bit dest keeps low 32 and zero-extends.
    assert_eq!(
        regs.rax, 0x0000_1010,
        "LEA EAX zero-extends low 32 bits of EA"
    );
}

// LEA r64, [index*scale + disp32] with no base (SIB base=101, mod=00).
#[test]
fn test_lea_index_scale_no_base() {
    let code = [
        0x48, 0x8d, 0x04, 0x8d, 0x00, 0x10, 0x00, 0x00, // LEA RAX, [RCX*4 + 0x1000]
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rcx = 0x0020;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax,
        0x0020 * 4 + 0x1000,
        "LEA [RCX*4 + disp32], no base"
    );
}

// ----- MOV with complex SIB to/from memory: exact byte assertions -----

// MOV [base + index*scale + disp8], r32 then read raw bytes.
#[test]
fn test_mov_store_sib_scale4_disp8_exact_bytes() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (base)
        0x48, 0xc7, 0xc1, 0x04, 0x00, 0x00, 0x00, // MOV RCX, 4 (index)
        0xb8, 0x78, 0x56, 0x34, 0x12, // MOV EAX, 0x12345678
        0x89, 0x44, 0x8b, 0x10, // MOV [RBX + RCX*4 + 0x10], EAX
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // EA = 0x2000 + 4*4 + 0x10 = 0x2020.
    assert_eq!(regs.rax & 0xFFFF_FFFF, 0x12345678, "EAX unchanged by store");
    let mut buf = [0u8; 4];
    mem.read_slice(&mut buf, GuestAddress(0x2020)).unwrap();
    assert_eq!(
        buf,
        [0x78, 0x56, 0x34, 0x12],
        "little-endian bytes at EA 0x2020"
    );
    assert_eq!(read_mem_at_u32(&mem, 0x2020), 0x12345678, "u32 readback");
}

// MOV r64, [base + index*scale] then check loaded value.
#[test]
fn test_mov_load_sib_scale8_exact() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000 (base)
        0x48, 0xc7, 0xc1, 0x02, 0x00, 0x00, 0x00, // MOV RCX, 2 (index)
        0x48, 0x8b, 0x04, 0xcb, // MOV RAX, [RBX + RCX*8]
        0xf4, // HLT
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    // EA = 0x2000 + 2*8 = 0x2010.
    write_mem_at_u64(&mem, 0x2010, 0xCAFEF00DDEADBEEF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0xCAFEF00DDEADBEEF,
        "MOV RAX, [RBX + RCX*8] @ 0x2010"
    );
}

// MOV [base + index] (scale 1) byte store places exact byte.
#[test]
fn test_mov_store_sib_scale1_byte() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xc7, 0xc1, 0x07, 0x00, 0x00, 0x00, // MOV RCX, 7
        0xb0, 0xA5, // MOV AL, 0xA5
        0x88, 0x04, 0x0b, // MOV [RBX + RCX], AL
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u8(&mem, 0x2007),
        0xA5,
        "byte store at SIB EA 0x2007"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2006),
        0x00,
        "neighbour byte untouched"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x2008),
        0x00,
        "neighbour byte untouched"
    );
}

// ----- MOVNTI: non-temporal 32/64-bit integer store -----

#[test]
fn test_movnti_32bit_store() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xb8, 0xEF, 0xBE, 0xAD, 0xDE, // MOV EAX, 0xDEADBEEF
        0x0f, 0xc3, 0x03, // MOVNTI [RBX], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u32(&mem, 0x2000),
        0xDEADBEEF,
        "MOVNTI m32 store"
    );
}

#[test]
fn test_movnti_64bit_store() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0xb8, 0xEF, 0xBE, 0xAD, 0xDE, 0x0D, 0xF0, 0xFE,
        0xCA, // MOV RAX, 0xCAFEF00DDEADBEEF
        0x48, 0x0f, 0xc3, 0x03, // MOVNTI [RBX], RAX (REX.W)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, 0x2000),
        0xCAFEF00DDEADBEEF,
        "MOVNTI m64 store"
    );
}

// MOVNTI with a displaced destination address.
#[test]
fn test_movnti_32bit_store_disp() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0xb8, 0x44, 0x33, 0x22, 0x11, // MOV EAX, 0x11223344
        0x0f, 0xc3, 0x43, 0x20, // MOVNTI [RBX + 0x20], EAX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u32(&mem, 0x2020),
        0x11223344,
        "MOVNTI m32 @ disp 0x20"
    );
}

// ----- MOVNTDQ: non-temporal 128-bit XMM store (66 0F E7 /r) -----

#[test]
fn test_movntdq_128bit_store() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x0f, 0xe7, 0x03, // MOVNTDQ [RBX], XMM0
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let val: u128 = 0x0F0E0D0C0B0A09080706050403020100;
    set_xmm(&mem, &mut vcpu, 0, val);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    // Little-endian: byte 0 = 0x00 .. byte 15 = 0x0F.
    let lo = read_mem_at_u64(&mem, 0x2000);
    let hi = read_mem_at_u64(&mem, 0x2008);
    assert_eq!(lo, 0x0706050403020100, "MOVNTDQ low 64 bits");
    assert_eq!(hi, 0x0F0E0D0C0B0A0908, "MOVNTDQ high 64 bits");
}

// ----- 16/32/64-bit memory operand sizes (round-trip exact value) -----

#[test]
fn test_mem_operand_size_16bit() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x66, 0x8b, 0x03, // MOV AX, [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u16(&mem, 0x2000, 0xBEEF);
    let mut regs0 = vcpu.get_regs().unwrap();
    regs0.rax = 0xFFFF_FFFF_FFFF_FFFF;
    vcpu.set_regs(&regs0).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();
    // 16-bit load only writes AX; upper bits preserved.
    assert_eq!(regs.rax & 0xFFFF, 0xBEEF, "AX loaded from m16");
    assert_eq!(
        regs.rax >> 16,
        0xFFFF_FFFF_FFFF,
        "upper bits preserved on 16-bit load"
    );
}

#[test]
fn test_mem_operand_size_32bit_zero_extends() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x8b, 0x03, // MOV EAX, [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u32(&mem, 0x2000, 0xCAFEBABE);
    let mut regs0 = vcpu.get_regs().unwrap();
    regs0.rax = 0xFFFF_FFFF_FFFF_FFFF;
    vcpu.set_regs(&regs0).unwrap();
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0xCAFEBABE, "32-bit load zero-extends to RAX");
}

#[test]
fn test_mem_operand_size_64bit() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x48, 0x8b, 0x03, // MOV RAX, [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x2000, 0x1122334455667788);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax, 0x1122334455667788, "64-bit load");
}

// ----- 0x67 32-bit-address form -----

// With 0x67 in 64-bit mode, the base register is read as 32 bits and the
// effective address is masked to 32 bits. Here EBX = 0x2000 selects 0x2000.
#[test]
fn test_addr32_override_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x00, 0x20, 0x00, 0x00, // MOV RBX, 0x2000
        0x67, 0x48, 0x8b, 0x03, // MOV RAX, [EBX] (0x67 addr-size override)
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x2000, 0x0BADF00DCAFEBABE);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x0BADF00DCAFEBABE,
        "0x67 32-bit-address load reads EBX"
    );
}

// With 0x67, only the low 32 bits of the base participate: a dirty high half of
// RBX must be ignored and the EA wraps within 32 bits.
#[test]
fn test_addr32_override_ignores_high_base_bits() {
    let code = [
        0x48, 0xbb, 0x00, 0x20, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, // MOV RBX, 0xFFFFFFFF00002000
        0x67, 0x48, 0x8b, 0x03, // MOV RAX, [EBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    write_mem_at_u64(&mem, 0x2000, 0x5555AAAA5555AAAA);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        regs.rax, 0x5555AAAA5555AAAA,
        "0x67 ignores high 32 bits of base"
    );
}

// ----- Misaligned accesses (x86 allows them; value must be exact) -----

#[test]
fn test_misaligned_u32_store_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x03, 0x20, 0x00, 0x00, // MOV RBX, 0x2003 (misaligned)
        0xb8, 0x21, 0x43, 0x65, 0x87, // MOV EAX, 0x87654321
        0x89, 0x03, // MOV [RBX], EAX
        0x48, 0xc7, 0xc1, 0x03, 0x20, 0x00, 0x00, // MOV RCX, 0x2003
        0x8b, 0x11, // MOV EDX, [RCX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u32(&mem, 0x2003),
        0x87654321,
        "misaligned u32 store"
    );
    assert_eq!(regs.rdx & 0xFFFF_FFFF, 0x87654321, "misaligned u32 load");
}

#[test]
fn test_misaligned_u64_store_load() {
    let code = [
        0x48, 0xc7, 0xc3, 0x01, 0x20, 0x00, 0x00, // MOV RBX, 0x2001 (misaligned)
        0x48, 0xb8, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23,
        0x01, // MOV RAX, 0x0123456789ABCDEF
        0x48, 0x89, 0x03, // MOV [RBX], RAX
        0x48, 0x8b, 0x13, // MOV RDX, [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, 0x2001),
        0x0123456789ABCDEF,
        "misaligned u64 store"
    );
    assert_eq!(regs.rdx, 0x0123456789ABCDEF, "misaligned u64 load");
}

// ----- Page-crossing accesses (paging is disabled, but the access still spans
// a 4 KiB boundary; a u64 written at 0x2FFC straddles 0x2FFC..0x3004). -----

#[test]
fn test_page_crossing_u64_store_load() {
    let addr = 0x2FFC; // 4 bytes before the 0x3000 boundary
    let code = [
        0x48, 0xc7, 0xc3, 0xFC, 0x2F, 0x00, 0x00, // MOV RBX, 0x2FFC
        0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, // MOV RAX, 0x1122334455667788
        0x48, 0x89, 0x03, // MOV [RBX], RAX
        0x48, 0x8b, 0x13, // MOV RDX, [RBX]
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u64(&mem, addr),
        0x1122334455667788,
        "page-crossing u64 store"
    );
    assert_eq!(regs.rdx, 0x1122334455667788, "page-crossing u64 load");
    // The high dword physically lands on the next page.
    assert_eq!(
        read_mem_at_u32(&mem, 0x3000),
        0x11223344,
        "high dword on next page"
    );
}

#[test]
fn test_page_crossing_u16_store() {
    let code = [
        0x48, 0xc7, 0xc3, 0xFF, 0x2F, 0x00, 0x00, // MOV RBX, 0x2FFF (last byte of page)
        0x66, 0xb8, 0xCD, 0xAB, // MOV AX, 0xABCD
        0x66, 0x89, 0x03, // MOV [RBX], AX
        0xf4,
    ];
    let (mut vcpu, mem) = setup_vm(&code, None);
    let _ = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        read_mem_at_u8(&mem, 0x2FFF),
        0xCD,
        "low byte on page 0x2000"
    );
    assert_eq!(
        read_mem_at_u8(&mem, 0x3000),
        0xAB,
        "high byte on page 0x3000"
    );
}
