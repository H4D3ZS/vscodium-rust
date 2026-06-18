use crate::common::TestCase;

// Legacy and miscellaneous x86 instructions

// ARPL - Adjust RPL field of segment selector

#[test]
fn test_arpl_r16_r16() {
    TestCase::from("63 c1").check();
}

#[test]
fn test_arpl_m16_r16() {
    TestCase::from("63 08").check();
}

#[test]
fn test_arpl_ax_bx() {
    TestCase::from("63 c3").check();
}

#[test]
fn test_arpl_cx_dx() {
    TestCase::from("63 ca").check();
}

#[test]
fn test_arpl_mem_ax() {
    TestCase::from("63 00").check();
}

// LDS/LES/LFS/LGS/LSS - Load far pointer

#[test]
fn test_lds_r16_m16_16() {
    TestCase::from("c5 00").check();
}

#[test]
fn test_lds_r32_m16_32() {
    TestCase::from("c5 00").check();
}

#[test]
fn test_les_r16_m16_16() {
    TestCase::from("c4 00").check();
}

#[test]
fn test_les_r32_m16_32() {
    TestCase::from("c4 00").check();
}

#[test]
fn test_lfs_r16_m16_16() {
    TestCase::from("0f b4 00").check();
}

#[test]
fn test_lfs_r32_m16_32() {
    TestCase::from("0f b4 00").check();
}

#[test]
fn test_lfs_r64_m16_64() {
    TestCase::from("48 0f b4 00").check();
}

#[test]
fn test_lgs_r16_m16_16() {
    TestCase::from("0f b5 00").check();
}

#[test]
fn test_lgs_r32_m16_32() {
    TestCase::from("0f b5 00").check();
}

#[test]
fn test_lgs_r64_m16_64() {
    TestCase::from("48 0f b5 00").check();
}

#[test]
fn test_lss_r16_m16_16() {
    TestCase::from("0f b2 00").check();
}

#[test]
fn test_lss_r32_m16_32() {
    TestCase::from("0f b2 00").check();
}

#[test]
fn test_lss_r64_m16_64() {
    TestCase::from("48 0f b2 00").check();
}

// MOVSX/MOVSXD - Move with sign extension

#[test]
fn test_movsx_r16_r8() {
    TestCase::from("66 0f be c1").check();
}

#[test]
fn test_movsx_r32_r8() {
    TestCase::from("0f be c1").check();
}

#[test]
fn test_movsx_r64_r8() {
    TestCase::from("48 0f be c1").check();
}

#[test]
fn test_movsx_r32_r16() {
    TestCase::from("0f bf c1").check();
}

#[test]
fn test_movsx_r64_r16() {
    TestCase::from("48 0f bf c1").check();
}

#[test]
fn test_movsxd_r64_r32() {
    TestCase::from("48 63 c1").check();
}

#[test]
fn test_movsx_eax_al() {
    TestCase::from("0f be c0").check();
}

#[test]
fn test_movsx_rax_al() {
    TestCase::from("48 0f be c0").check();
}

#[test]
fn test_movsx_eax_ax() {
    TestCase::from("0f bf c0").check();
}

#[test]
fn test_movsx_rax_ax() {
    TestCase::from("48 0f bf c0").check();
}

#[test]
fn test_movsxd_rax_eax() {
    TestCase::from("48 63 c0").check();
}

// POPA/POPAD/PUSHA/PUSHAD - Push/pop all general-purpose registers

#[test]
fn test_popa() {
    TestCase::from("61").check();
}

#[test]
fn test_popad() {
    TestCase::from("61").check();
}

#[test]
fn test_pusha() {
    TestCase::from("60").check();
}

#[test]
fn test_pushad() {
    TestCase::from("60").check();
}

// POPF/POPFD/POPFQ - Pop EFLAGS

#[test]
fn test_popf() {
    TestCase::from("66 9d").check();
}

#[test]
fn test_popfd() {
    TestCase::from("9d").check();
}

#[test]
fn test_popfq() {
    TestCase::from("48 9d").check();
}

// PUSHF/PUSHFD/PUSHFQ - Push EFLAGS

#[test]
fn test_pushf() {
    TestCase::from("66 9c").check();
}

#[test]
fn test_pushfd() {
    TestCase::from("9c").check();
}

#[test]
fn test_pushfq() {
    TestCase::from("48 9c").check();
}

// RCL/RCR/ROL/ROR - Rotate instructions

#[test]
fn test_rcl_r8_1() {
    TestCase::from("d0 c0").check();
}

#[test]
fn test_rcl_r8_cl() {
    TestCase::from("d2 c0").check();
}

#[test]
fn test_rcl_r8_imm8() {
    TestCase::from("c0 c0 04").check();
}

#[test]
fn test_rcl_r16_1() {
    TestCase::from("66 d1 c0").check();
}

#[test]
fn test_rcl_r32_1() {
    TestCase::from("d1 c0").check();
}

#[test]
fn test_rcl_r64_1() {
    TestCase::from("48 d1 c0").check();
}

#[test]
fn test_rcr_r8_1() {
    TestCase::from("d0 c8").check();
}

#[test]
fn test_rcr_r8_cl() {
    TestCase::from("d2 c8").check();
}

#[test]
fn test_rcr_r16_1() {
    TestCase::from("66 d1 c8").check();
}

#[test]
fn test_rcr_r32_1() {
    TestCase::from("d1 c8").check();
}

#[test]
fn test_rcr_r64_1() {
    TestCase::from("48 d1 c8").check();
}

#[test]
fn test_rol_r8_1() {
    TestCase::from("d0 c0").check();
}

#[test]
fn test_rol_r8_cl() {
    TestCase::from("d2 c0").check();
}

#[test]
fn test_rol_r16_1() {
    TestCase::from("66 d1 c0").check();
}

#[test]
fn test_rol_r32_1() {
    TestCase::from("d1 c0").check();
}

#[test]
fn test_rol_r64_1() {
    TestCase::from("48 d1 c0").check();
}

#[test]
fn test_ror_r8_1() {
    TestCase::from("d0 c8").check();
}

#[test]
fn test_ror_r8_cl() {
    TestCase::from("d2 c8").check();
}

#[test]
fn test_ror_r16_1() {
    TestCase::from("66 d1 c8").check();
}

#[test]
fn test_ror_r32_1() {
    TestCase::from("d1 c8").check();
}

#[test]
fn test_ror_r64_1() {
    TestCase::from("48 d1 c8").check();
}

// SAL/SAR/SHL/SHR - Shift instructions

#[test]
fn test_sal_r8_1() {
    TestCase::from("d0 e0").check();
}

#[test]
fn test_sal_r8_cl() {
    TestCase::from("d2 e0").check();
}

#[test]
fn test_sal_r16_1() {
    TestCase::from("66 d1 e0").check();
}

#[test]
fn test_sal_r32_1() {
    TestCase::from("d1 e0").check();
}

#[test]
fn test_sal_r64_1() {
    TestCase::from("48 d1 e0").check();
}

#[test]
fn test_sar_r8_1() {
    TestCase::from("d0 f8").check();
}

#[test]
fn test_sar_r8_cl() {
    TestCase::from("d2 f8").check();
}

#[test]
fn test_sar_r16_1() {
    TestCase::from("66 d1 f8").check();
}

#[test]
fn test_sar_r32_1() {
    TestCase::from("d1 f8").check();
}

#[test]
fn test_sar_r64_1() {
    TestCase::from("48 d1 f8").check();
}

#[test]
fn test_shl_r8_1() {
    TestCase::from("d0 e0").check();
}

#[test]
fn test_shl_r8_cl() {
    TestCase::from("d2 e0").check();
}

#[test]
fn test_shl_r16_1() {
    TestCase::from("66 d1 e0").check();
}

#[test]
fn test_shl_r32_1() {
    TestCase::from("d1 e0").check();
}

#[test]
fn test_shl_r64_1() {
    TestCase::from("48 d1 e0").check();
}

#[test]
fn test_shr_r8_1() {
    TestCase::from("d0 e8").check();
}

#[test]
fn test_shr_r8_cl() {
    TestCase::from("d2 e8").check();
}

#[test]
fn test_shr_r16_1() {
    TestCase::from("66 d1 e8").check();
}

#[test]
fn test_shr_r32_1() {
    TestCase::from("d1 e8").check();
}

#[test]
fn test_shr_r64_1() {
    TestCase::from("48 d1 e8").check();
}

// PREFETCHH - Prefetch hint to higher cache level

#[test]
fn test_prefetchh_m8() {
    TestCase::from("0f 18 10").check();
}

#[test]
fn test_prefetchh_mem() {
    TestCase::from("0f 18 10").check();
}
