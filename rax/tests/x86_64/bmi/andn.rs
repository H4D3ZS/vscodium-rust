use crate::common::*;
use rax::cpu::Registers;

// ANDN - Logical AND NOT (BMI1)
// dest = (NOT src1) AND src2, where src1 = VEX.vvvv and src2 = r/m.
// (Mnemonic `ANDN dest, src1, src2`.) Sets ZF from the result, clears CF/OF;
// SF from the high bit; AF/PF are undefined.
//
// Opcodes:
// VEX.NDS.LZ.0F38.W0 F2 /r   ANDN r32, r32, r/m32
// VEX.NDS.LZ.0F38.W1 F2 /r   ANDN r64, r64, r/m64
//
// NOTE: expected values below are computed from the spec formula `(!src1) & src2`
// so they cannot drift from the architectural definition (cross-checked against
// KVM by tests/differential.rs::bmi_andn).

/// Run `ANDN EAX, EBX, ECX` (src1=EBX, src2=ECX) and return EAX (low 32 bits).
fn andn32(src1: u32, src2: u32) -> (u64, u64) {
    let code = [
        0xc4, 0xe2, 0x60, 0xf2, 0xc1, // ANDN EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = src1 as u64;
    regs.rcx = src2 as u64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    (regs.rax & 0xFFFF_FFFF, regs.rflags)
}

/// Run `ANDN RAX, RBX, RCX` (src1=RBX, src2=RCX) and return RAX.
fn andn64(src1: u64, src2: u64) -> (u64, u64) {
    let code = [
        0xc4, 0xe2, 0xe0, 0xf2, 0xc1, // ANDN RAX, RBX, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = src1;
    regs.rcx = src2;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    (regs.rax, regs.rflags)
}

#[test]
fn test_andn32_value_matrix() {
    // Every case asserts against the computed spec result (!src1) & src2.
    let cases: [(u32, u32); 24] = [
        (0xFF, 0x0F),
        (0x0F, 0xFFFF_FFFF),
        (0x1234_5678, 0x0000_0000),
        (0x0000_0000, 0xFFFF_FFFF),
        (0xFFFF_FFFF, 0x0000_0000),
        (0xAAAA_AAAA, 0x5555_5555),
        (0x5555_5555, 0xAAAA_AAAA),
        (0xFF00_FF00, 0x00FF_00FF),
        (0xF0F0_F0F0, 0x0F0F_0F0F),
        (0x1234_5678, 0x1234_5678),
        (0xDEAD_BEEF, 0x0000_FFFF),
        (0xFEDC_BA98, 0x00FF_00FF),
        (0x00FF_FF00, 0xFF00_00FF),
        (0x3333_3333, 0xCCCC_CCCC),
        (0xCCCC_CCCC, 0x3333_3333),
        (0x9999_9999, 0x6666_6666),
        (0x1234_5678, 0x0F0F_0F0F),
        (0xFF00_0000, 0x00FF_FFFF),
        (0x00FF_0000, 0xFF00_FFFF),
        (0x0000_FF00, 0xFFFF_00FF),
        (0x0000_00FF, 0xFFFF_FF00),
        (0xF000_0000, 0x0F00_0000),
        (0xABCD_EF12, 0xFFFF_FFF0),
        (0x12345678, 0x0F0F_0F0F),
    ];
    for (src1, src2) in cases {
        let expected = ((!src1) & src2) as u64;
        let (eax, _) = andn32(src1, src2);
        assert_eq!(
            eax, expected,
            "ANDN(src1={:08x}, src2={:08x}) = (~src1)&src2 = {:08x}",
            src1, src2, expected
        );
    }
}

#[test]
fn test_andn64_value_matrix() {
    let cases: [(u64, u64); 8] = [
        (0xFFFF_FFFF_FFFF_FFFF, 0x0000_0000_FFFF_FFFF),
        (0x0000_0000_FFFF_FFFF, 0xFFFF_FFFF_0000_0000),
        (0x0000_0000_0000_0000, 0xFFFF_FFFF_FFFF_FFFF),
        (0xFFFF_FFFF_FFFF_FFFF, 0x0000_0000_0000_0000),
        (0xAAAA_AAAA_AAAA_AAAA, 0x5555_5555_5555_5555),
        (0xFF00_FF00_FF00_FF00, 0x00FF_00FF_00FF_00FF),
        (0x1234_5678_ABCD_EF00, 0xFFFF_FFFF_0000_0000),
        (0xDEAD_BEEF_DEAD_BEEF, 0x0000_0000_FFFF_FFFF),
    ];
    for (src1, src2) in cases {
        let expected = (!src1) & src2;
        let (rax, _) = andn64(src1, src2);
        assert_eq!(
            rax, expected,
            "ANDN64(src1={:016x}, src2={:016x}) = {:016x}",
            src1, src2, expected
        );
    }
}

#[test]
fn test_andn_single_bit_sweep() {
    // src1 = single bit, src2 = all ones: result clears exactly that bit.
    for bit in 0..32u32 {
        let src1 = 1u32 << bit;
        let src2 = 0xFFFF_FFFFu32;
        let expected = ((!src1) & src2) as u64;
        let (eax, _) = andn32(src1, src2);
        assert_eq!(eax, expected, "ANDN clears bit {}", bit);
        // src1 = all ones, src2 = single bit: result is 0.
        let (eax2, fl2) = andn32(0xFFFF_FFFF, 1u32 << bit);
        assert_eq!(eax2, 0, "~all-ones & bit = 0");
        assert!(zf_set(fl2), "ZF set when result is 0");
    }
}

#[test]
fn test_andn_flags_zf_cf() {
    // Non-zero result -> ZF clear, CF clear.
    let (eax, fl) = andn32(0x0000_0000, 0xFFFF_FFFF); // ~0 & all = all ones
    assert_eq!(eax, 0xFFFF_FFFF);
    assert!(!zf_set(fl), "ZF clear for non-zero result");
    assert!(!cf_set(fl), "CF always clear");
    // Zero result -> ZF set.
    let (eax, fl) = andn32(0xFFFF_FFFF, 0x1234_5678); // ~all & x = 0
    assert_eq!(eax, 0);
    assert!(zf_set(fl), "ZF set for zero result");
    assert!(!cf_set(fl), "CF always clear");
}

#[test]
fn test_andn_preserves_sources() {
    let code = [
        0xc4, 0xe2, 0x60, 0xf2, 0xc1, // ANDN EAX, EBX, ECX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x1234_5678;
    regs.rcx = 0x8765_4321;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rbx & 0xFFFF_FFFF, 0x1234_5678, "src1 (EBX) unchanged");
    assert_eq!(regs.rcx & 0xFFFF_FFFF, 0x8765_4321, "src2 (ECX) unchanged");
    // dest = ~EBX & ECX
    assert_eq!(
        regs.rax & 0xFFFF_FFFF,
        ((!0x1234_5678u32) & 0x8765_4321u32) as u64
    );
}

#[test]
fn test_andn_extended_registers() {
    // ANDN R8D, R9D, R10D : src1=R9D, src2=R10D
    let code = [
        0xc4, 0x42, 0x30, 0xf2, 0xc2, // ANDN R8D, R9D, R10D
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r9 = 0b1111_0000;
    regs.r10 = 0b0011_1100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let expected = ((!0b1111_0000u32) & 0b0011_1100u32) as u64; // 0x0C
    assert_eq!(regs.r8 & 0xFFFF_FFFF, expected, "R8D = ~R9D & R10D");
}

#[test]
fn test_andn_r15() {
    // ANDN R15, R15, RCX : src1=R15, src2=RCX
    let code = [
        0xc4, 0x62, 0x80, 0xf2, 0xf9, // ANDN R15, R15, RCX
        0xf4,
    ];
    let mut regs = Registers::default();
    regs.r15 = 0xDEAD_BEEF_DEAD_BEEF;
    regs.rcx = 0x0000_0000_FFFF_FFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let expected = (!0xDEAD_BEEF_DEAD_BEEFu64) & 0x0000_0000_FFFF_FFFFu64;
    assert_eq!(regs.r15, expected, "R15 = ~R15 & RCX");
}

#[test]
fn test_andn_mem32() {
    // ANDN EAX, EBX, [DATA_ADDR] : src1=EBX, src2=[mem]
    let code = [
        0xc4, 0xe2, 0x60, 0xf2, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xf4,
    ];
    let mut initial_regs = Registers::default();
    initial_regs.rbx = 0x0000_0000; // ~0 = all ones -> result == [mem]
    let (mut vcpu, mem) = setup_vm(&code, Some(initial_regs));
    write_mem_u32(&mem, 0xABCD_1234);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let expected = ((!0x0000_0000u32) & 0xABCD_1234u32) as u64;
    assert_eq!(regs.rax & 0xFFFF_FFFF, expected, "EAX = ~EBX & [mem]");
}

#[test]
fn test_andn_mem64() {
    // ANDN RAX, RBX, [DATA_ADDR] : src1=RBX, src2=[mem]
    let code = [
        0xc4, 0xe2, 0xe0, 0xf2, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0xf4,
    ];
    let mut initial_regs = Registers::default();
    initial_regs.rbx = 0x0000_0000_FFFF_FFFF;
    let (mut vcpu, mem) = setup_vm(&code, Some(initial_regs));
    write_mem_u64(&mem, 0xFFFF_FFFF_FFFF_FFFF);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    let expected = (!0x0000_0000_FFFF_FFFFu64) & 0xFFFF_FFFF_FFFF_FFFFu64;
    assert_eq!(regs.rax, expected, "RAX = ~RBX & [mem]");
}

// ============================================================================
// Lazy-flags regression tests (BMI flag-writing instructions)
//
// rax uses lazy flags: ALU ops (e.g. OR) defer flag computation until a flag
// reader (SETcc/Jcc) materializes them. ANDN/BLSR/BEXTR write defined flags
// EAGERLY and must clear the stale pending lazy op. Each test runs an OR
// producing ZF=0, then a BMI op producing ZF=1, then a flag consumer that must
// observe ZF=1. Inputs are chosen so the BMI result is 0 under the correct
// semantics (ANDN = ~src1 & src2).
// ============================================================================

#[test]
fn test_andn_clears_stale_lazy_zf_setz() {
    // ANDN EAX,EBX,ECX with EBX(src1)=0xFFFFFFFF, ECX(src2)=0x0F -> ~all & 0x0F = 0.
    let code = [
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0x09, 0xd2, // OR EDX, EDX  (lazy ZF=0)
        0xc4, 0xe2, 0x60, 0xf2, 0xc1, // ANDN EAX, EBX, ECX -> 0 -> ZF=1
        0x0f, 0x94, 0xc2, // SETZ DL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xFFFF_FFFF; // src1
    regs.rcx = 0x0000_000F; // src2
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "ANDN result should be zero");
    assert!(zf_set(regs.rflags), "ZF must reflect ANDN, not stale OR");
    assert_eq!(regs.rdx, 1, "SETZ must see ANDN's ZF=1");
}

#[test]
fn test_blsr_clears_stale_lazy_zf_jz() {
    // BLSR EAX,EBX with EBX=1 -> result 0, ZF=1.
    let code = [
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0x09, 0xd2, // OR EDX, EDX  (lazy ZF=0)
        0xc4, 0xe2, 0x78, 0xf3, 0xcb, // BLSR EAX, EBX (EBX=1 -> 0 -> ZF=1)
        0x74, 0x09, // JZ +9
        0x48, 0xc7, 0xc3, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0xeb, 0x07, // JMP +7
        0x48, 0xc7, 0xc3, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0x0000_0001;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "BLSR result should be zero");
    assert_eq!(regs.rbx, 1, "JZ must jump on BLSR's ZF=1");
}

#[test]
fn test_bextr_clears_stale_lazy_zf_setz() {
    // BEXTR EAX,EBX,ECX with ECX=0 (len=0) -> result 0, ZF=1.
    let code = [
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0x09, 0xd2, // OR EDX, EDX  (lazy ZF=0)
        0xc4, 0xe2, 0x70, 0xf7, 0xc3, // BEXTR EAX, EBX, ECX (len=0 -> 0 -> ZF=1)
        0x0f, 0x94, 0xc2, // SETZ DL
        0xf4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rbx = 0xDEAD_BEEF;
    regs.rcx = 0x0000_0000; // start=0, len=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(regs.rax & 0xFFFFFFFF, 0, "BEXTR with len=0 should be zero");
    assert!(zf_set(regs.rflags), "ZF must reflect BEXTR");
    assert_eq!(regs.rdx, 1, "SETZ must see BEXTR's ZF=1");
}
