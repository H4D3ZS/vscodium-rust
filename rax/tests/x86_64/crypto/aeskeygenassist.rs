use crate::common::{get_xmm, run_until_hlt, set_xmm, setup_vm};
use rax::cpu::Registers;
use vm_memory::{Bytes, GuestAddress};

// AESKEYGENASSIST - AES Round Key Generation Assist
//
// Assists in expanding the AES cipher key by computing steps towards
// generating round keys for encryption. Takes 128-bit data and an
// 8-bit round constant (RCON) to produce intermediate values for
// key expansion.
//
// Operation:
//   X3[31:0] := SRC[127:96]
//   X2[31:0] := SRC[95:64]
//   X1[31:0] := SRC[63:32]
//   X0[31:0] := SRC[31:0]
//   RCON[31:0] := ZeroExtend(imm8[7:0])
//   DEST[31:0] := SubWord(X1)
//   DEST[63:32] := RotWord(SubWord(X1)) XOR RCON
//   DEST[95:64] := SubWord(X3)
//   DEST[127:96] := RotWord(SubWord(X3)) XOR RCON
//
// Opcodes:
// 66 0F 3A DF /r ib           AESKEYGENASSIST xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// AESKEYGENASSIST Tests - Register Forms with Different RCON Values
// ============================================================================

#[test]
fn test_aeskeygenassist_xmm0_xmm1_rcon_0x01() {
    // AESKEYGENASSIST XMM0, XMM1, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm1_xmm2_rcon_0x02() {
    // AESKEYGENASSIST XMM1, XMM2, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xca, 0x02, // AESKEYGENASSIST XMM1, XMM2, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm2_xmm3_rcon_0x04() {
    // AESKEYGENASSIST XMM2, XMM3, 0x04
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xd3, 0x04, // AESKEYGENASSIST XMM2, XMM3, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm3_xmm4_rcon_0x08() {
    // AESKEYGENASSIST XMM3, XMM4, 0x08
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xdc, 0x08, // AESKEYGENASSIST XMM3, XMM4, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm4_xmm5_rcon_0x10() {
    // AESKEYGENASSIST XMM4, XMM5, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xe5, 0x10, // AESKEYGENASSIST XMM4, XMM5, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm5_xmm6_rcon_0x20() {
    // AESKEYGENASSIST XMM5, XMM6, 0x20
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xee, 0x20, // AESKEYGENASSIST XMM5, XMM6, 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm6_xmm7_rcon_0x40() {
    // AESKEYGENASSIST XMM6, XMM7, 0x40
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xf7, 0x40, // AESKEYGENASSIST XMM6, XMM7, 0x40
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm7_xmm0_rcon_0x80() {
    // AESKEYGENASSIST XMM7, XMM0, 0x80
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xf8, 0x80, // AESKEYGENASSIST XMM7, XMM0, 0x80
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - Standard AES RCON Values
// ============================================================================

#[test]
fn test_aeskeygenassist_rcon_0x1b() {
    // AESKEYGENASSIST with RCON 0x1B (AES round 9)
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x1b, // AESKEYGENASSIST XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_rcon_0x36() {
    // AESKEYGENASSIST with RCON 0x36 (AES round 10)
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xca, 0x36, // AESKEYGENASSIST XMM1, XMM2, 0x36
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_rcon_zero() {
    // AESKEYGENASSIST with RCON 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xd3, 0x00, // AESKEYGENASSIST XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_rcon_max() {
    // AESKEYGENASSIST with RCON 0xFF
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xdc, 0xff, // AESKEYGENASSIST XMM3, XMM4, 0xFF
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - Extended Registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_aeskeygenassist_xmm8_xmm9_rcon_0x01() {
    // AESKEYGENASSIST XMM8, XMM9, 0x01 (requires REX.R and REX.B)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM8, XMM9, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm9_xmm10_rcon_0x02() {
    // AESKEYGENASSIST XMM9, XMM10, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xca, 0x02, // AESKEYGENASSIST XMM9, XMM10, 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm10_xmm11_rcon_0x04() {
    // AESKEYGENASSIST XMM10, XMM11, 0x04
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xd3, 0x04, // AESKEYGENASSIST XMM10, XMM11, 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm15_xmm8_rcon_0x10() {
    // AESKEYGENASSIST XMM15, XMM8, 0x10
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xf8, 0x10, // AESKEYGENASSIST XMM15, XMM8, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - Memory Operand Forms
// ============================================================================

#[test]
fn test_aeskeygenassist_xmm0_mem_rcon_0x01() {
    // AESKEYGENASSIST XMM0, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // AESKEYGENASSIST XMM0, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm1_mem_rcon_0x02() {
    // AESKEYGENASSIST XMM1, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x02, // AESKEYGENASSIST XMM1, [0x3000], 0x02
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm7_mem_rcon_0x10() {
    // AESKEYGENASSIST XMM7, [ALIGNED_ADDR], 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x10, // AESKEYGENASSIST XMM7, [0x3000], 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm8_mem_rcon_0x01() {
    // AESKEYGENASSIST XMM8, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0xdf, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x01, // AESKEYGENASSIST XMM8, [0x3000], 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm15_mem_rcon_0x1b() {
    // AESKEYGENASSIST XMM15, [ALIGNED_ADDR], 0x1B
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0xdf, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x1b, // AESKEYGENASSIST XMM15, [0x3000], 0x1B
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - Same Register (Destination = Source)
// ============================================================================

#[test]
fn test_aeskeygenassist_xmm0_xmm0_rcon_0x01() {
    // AESKEYGENASSIST XMM0, XMM0, 0x01 (in-place)
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc0, 0x01, // AESKEYGENASSIST XMM0, XMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm7_xmm7_rcon_0x10() {
    // AESKEYGENASSIST XMM7, XMM7, 0x10
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xff, 0x10, // AESKEYGENASSIST XMM7, XMM7, 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm15_xmm15_rcon_0x01() {
    // AESKEYGENASSIST XMM15, XMM15, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xff, 0x01, // AESKEYGENASSIST XMM15, XMM15, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - AES-128 Key Expansion Sequence
// ============================================================================

#[test]
fn test_aeskeygenassist_aes128_key_expansion() {
    // Simulating AES-128 key expansion (10 rounds, RCON 0x01-0x36)
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01 (round 1)
        0x66, 0x0f, 0x3a, 0xdf, 0xc2, 0x02, // AESKEYGENASSIST XMM0, XMM2, 0x02 (round 2)
        0x66, 0x0f, 0x3a, 0xdf, 0xc3, 0x04, // AESKEYGENASSIST XMM0, XMM3, 0x04 (round 3)
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_sequential_rcons() {
    // Sequential RCON values
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // RCON 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x02, // RCON 0x02
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x04, // RCON 0x04
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x08, // RCON 0x08
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x10, // RCON 0x10
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Tests - Mixed Combinations
// ============================================================================

#[test]
fn test_aeskeygenassist_mixed_regs_low_to_high() {
    // AESKEYGENASSIST XMM0, XMM15, 0x01 (low to high extended)
    let code = [
        0x66, 0x41, 0x0f, 0x3a, 0xdf, 0xc7, 0x01, // AESKEYGENASSIST XMM0, XMM15, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_mixed_regs_high_to_low() {
    // AESKEYGENASSIST XMM15, XMM0, 0x01 (high extended to low)
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0xdf, 0xf8, 0x01, // AESKEYGENASSIST XMM15, XMM0, 0x01
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_all_registers() {
    // Test multiple register combinations
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM0, XMM1, 0x01
        0x66, 0x0f, 0x3a, 0xdf, 0xd3, 0x02, // AESKEYGENASSIST XMM2, XMM3, 0x02
        0x66, 0x0f, 0x3a, 0xdf, 0xe5, 0x04, // AESKEYGENASSIST XMM4, XMM5, 0x04
        0x66, 0x0f, 0x3a, 0xdf, 0xf7, 0x08, // AESKEYGENASSIST XMM6, XMM7, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_all_extended_registers() {
    // Test extended register combinations
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, // AESKEYGENASSIST XMM8, XMM9, 0x01
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xd3, 0x02, // AESKEYGENASSIST XMM10, XMM11, 0x02
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xe5, 0x04, // AESKEYGENASSIST XMM12, XMM13, 0x04
        0x66, 0x45, 0x0f, 0x3a, 0xdf, 0xf7, 0x08, // AESKEYGENASSIST XMM14, XMM15, 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm2_mem_rcon_0x04() {
    // AESKEYGENASSIST XMM2, [ALIGNED_ADDR], 0x04
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x04, // AESKEYGENASSIST XMM2, [0x3000], 0x04
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm3_mem_rcon_0x08() {
    // AESKEYGENASSIST XMM3, [ALIGNED_ADDR], 0x08
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x08, // AESKEYGENASSIST XMM3, [0x3000], 0x08
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm4_mem_rcon_0x20() {
    // AESKEYGENASSIST XMM4, [ALIGNED_ADDR], 0x20
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x20, // AESKEYGENASSIST XMM4, [0x3000], 0x20
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm5_mem_rcon_0x40() {
    // AESKEYGENASSIST XMM5, [ALIGNED_ADDR], 0x40
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x2c, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x40, // AESKEYGENASSIST XMM5, [0x3000], 0x40
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

#[test]
fn test_aeskeygenassist_xmm6_mem_rcon_0x80() {
    // AESKEYGENASSIST XMM6, [ALIGNED_ADDR], 0x80
    let code = [
        0x66, 0x0f, 0x3a, 0xdf, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x80, // AESKEYGENASSIST XMM6, [0x3000], 0x80
        0xf4, // HLT
    ];
    let (mut vcpu, _) = setup_vm(&code, None);
    run_until_hlt(&mut vcpu).unwrap();
}

// ============================================================================
// AESKEYGENASSIST Known-Answer Tests (Intel SDM operation)
// ============================================================================
//
// Per the Intel SDM:
//   X1 := SRC[63:32]; X3 := SRC[127:96]
//   DEST[31:0]    := SubWord(X1)
//   DEST[63:32]   := RotWord(SubWord(X1)) XOR RCON
//   DEST[95:64]   := SubWord(X3)
//   DEST[127:96]  := RotWord(SubWord(X3)) XOR RCON
//
// Input src = 0x7b5b54657374566563746f725d53475d loaded little-endian into the
// XMM register (low 64 bits = 0x63746f725d53475d, high 64 = 0x7b5b546573745665),
// with RCON=1. RotWord on a little-endian dword [b0,b1,b2,b3] -> [b1,b2,b3,b0]
// is a rotate-RIGHT by 8, so this yields 4d2139212139204d40fb92a9fb92a840.
// (This value is verified against the KVM hardware oracle in the differential
// harness. The old expected 0x39204d20...92a840 used the wrong rotate direction.)

const KEYGEN_SRC: u128 = 0x7b5b54657374566563746f725d53475d;
const KEYGEN_RESULT_RCON1: u128 = 0x4d2139212139204d40fb92a9fb92a840;

#[test]
fn kat_aeskeygenassist_intel_vector_rcon1() {
    // AESKEYGENASSIST XMM0, XMM1, 0x01  (66 0F 3A DF C1 01)
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, KEYGEN_SRC);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(
        get_xmm(&regs, 0),
        KEYGEN_RESULT_RCON1,
        "AESKEYGENASSIST produced {:032x}, expected {:032x}",
        get_xmm(&regs, 0),
        KEYGEN_RESULT_RCON1
    );
}

#[test]
fn kat_aeskeygenassist_zero_rcon0() {
    // SubWord(0) = 0x63636363 for every word; RotWord of a uniform word is a
    // no-op; XOR RCON=0 leaves it unchanged => all words 0x63636363.
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x00, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x6363636363636363_6363636363636363u128);
}

#[test]
fn kat_aeskeygenassist_zero_rcon1() {
    // Same as above but RCON=1 is XORed into the low byte of words 1 and 3:
    //   word1 = word3 = RotWord(0x63636363) XOR 0x00000001 = 0x63636362
    //   word0 = word2 = 0x63636363
    let code = [0x66, 0x0f, 0x3a, 0xdf, 0xc1, 0x01, 0xf4];
    let (mut vcpu, mem) = setup_vm(&code, None);
    set_xmm(&mem, &mut vcpu, 1, 0);
    let regs = run_until_hlt(&mut vcpu).unwrap();
    assert_eq!(get_xmm(&regs, 0), 0x63636362636363636363636263636363u128);
}
