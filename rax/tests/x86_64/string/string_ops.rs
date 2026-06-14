use crate::common::TestCase;

// String Operation Instructions

// CMPS/CMPSB/CMPSW/CMPSD/CMPSQ - Compare strings

#[test]
fn test_cmpsb() {
    TestCase::from("a6").check();
}

#[test]
fn test_cmpsw() {
    TestCase::from("66 a7").check();
}

#[test]
fn test_cmpsd() {
    TestCase::from("a7").check();
}

#[test]
fn test_cmpsq() {
    TestCase::from("48 a7").check();
}

#[test]
fn test_cmps_m8_m8() {
    TestCase::from("a6").check();
}

#[test]
fn test_cmps_m16_m16() {
    TestCase::from("66 a7").check();
}

#[test]
fn test_cmps_m32_m32() {
    TestCase::from("a7").check();
}

#[test]
fn test_cmps_m64_m64() {
    TestCase::from("48 a7").check();
}

#[test]
fn test_rep_cmpsb() {
    TestCase::from("f3 a6").check();
}

#[test]
fn test_rep_cmpsw() {
    TestCase::from("f3 66 a7").check();
}

#[test]
fn test_rep_cmpsd() {
    TestCase::from("f3 a7").check();
}

#[test]
fn test_rep_cmpsq() {
    TestCase::from("f3 48 a7").check();
}

// LODS/LODSB/LODSW/LODSD/LODSQ - Load string

#[test]
fn test_lodsb() {
    TestCase::from("ac").check();
}

#[test]
fn test_lodsw() {
    TestCase::from("66 ad").check();
}

#[test]
fn test_lodsd() {
    TestCase::from("ad").check();
}

#[test]
fn test_lodsq() {
    TestCase::from("48 ad").check();
}

#[test]
fn test_lods_m8() {
    TestCase::from("ac").check();
}

#[test]
fn test_lods_m16() {
    TestCase::from("66 ad").check();
}

#[test]
fn test_lods_m32() {
    TestCase::from("ad").check();
}

#[test]
fn test_lods_m64() {
    TestCase::from("48 ad").check();
}

#[test]
fn test_rep_lodsb() {
    TestCase::from("f3 ac").check();
}

#[test]
fn test_rep_lodsw() {
    TestCase::from("f3 66 ad").check();
}

#[test]
fn test_rep_lodsd() {
    TestCase::from("f3 ad").check();
}

#[test]
fn test_rep_lodsq() {
    TestCase::from("f3 48 ad").check();
}

// MOVS/MOVSB/MOVSW/MOVSD/MOVSQ - Move string

#[test]
fn test_movsb() {
    TestCase::from("a4").check();
}

#[test]
fn test_movsw() {
    TestCase::from("66 a5").check();
}

#[test]
fn test_movsd_string() {
    TestCase::from("a5").check();
}

#[test]
fn test_movsq() {
    TestCase::from("48 a5").check();
}

#[test]
fn test_movs_m8_m8() {
    TestCase::from("a4").check();
}

#[test]
fn test_movs_m16_m16() {
    TestCase::from("66 a5").check();
}

#[test]
fn test_movs_m32_m32() {
    TestCase::from("a5").check();
}

#[test]
fn test_movs_m64_m64() {
    TestCase::from("48 a5").check();
}

#[test]
fn test_rep_movsb() {
    TestCase::from("f3 a4").check();
}

#[test]
fn test_rep_movsw() {
    TestCase::from("f3 66 a5").check();
}

#[test]
fn test_rep_movsd() {
    TestCase::from("f3 a5").check();
}

#[test]
fn test_rep_movsq() {
    TestCase::from("f3 48 a5").check();
}

// SCAS/SCASB/SCASW/SCASD/SCASQ - Scan string

#[test]
fn test_scasb() {
    TestCase::from("ae").check();
}

#[test]
fn test_scasw() {
    TestCase::from("66 af").check();
}

#[test]
fn test_scasd() {
    TestCase::from("af").check();
}

#[test]
fn test_scasq() {
    TestCase::from("48 af").check();
}

#[test]
fn test_scas_m8() {
    TestCase::from("ae").check();
}

#[test]
fn test_scas_m16() {
    TestCase::from("66 af").check();
}

#[test]
fn test_scas_m32() {
    TestCase::from("af").check();
}

#[test]
fn test_scas_m64() {
    TestCase::from("48 af").check();
}

#[test]
fn test_rep_scasb() {
    TestCase::from("f3 ae").check();
}

#[test]
fn test_rep_scasw() {
    TestCase::from("f3 66 af").check();
}

#[test]
fn test_rep_scasd() {
    TestCase::from("f3 af").check();
}

#[test]
fn test_rep_scasq() {
    TestCase::from("f3 48 af").check();
}

#[test]
fn test_repne_scasb() {
    TestCase::from("f2 ae").check();
}

#[test]
fn test_repne_scasw() {
    TestCase::from("f2 66 af").check();
}

#[test]
fn test_repne_scasd() {
    TestCase::from("f2 af").check();
}

#[test]
fn test_repne_scasq() {
    TestCase::from("f2 48 af").check();
}

// STOS/STOSB/STOSW/STOSD/STOSQ - Store string

#[test]
fn test_stosb() {
    TestCase::from("aa").check();
}

#[test]
fn test_stosw() {
    TestCase::from("66 ab").check();
}

#[test]
fn test_stosd() {
    TestCase::from("ab").check();
}

#[test]
fn test_stosq() {
    TestCase::from("48 ab").check();
}

#[test]
fn test_stos_m8() {
    TestCase::from("aa").check();
}

#[test]
fn test_stos_m16() {
    TestCase::from("66 ab").check();
}

#[test]
fn test_stos_m32() {
    TestCase::from("ab").check();
}

#[test]
fn test_stos_m64() {
    TestCase::from("48 ab").check();
}

#[test]
fn test_rep_stosb() {
    TestCase::from("f3 aa").check();
}

#[test]
fn test_rep_stosw() {
    TestCase::from("f3 66 ab").check();
}

#[test]
fn test_rep_stosd() {
    TestCase::from("f3 ab").check();
}

#[test]
fn test_rep_stosq() {
    TestCase::from("f3 48 ab").check();
}

// INS/INSB/INSW/INSD - Input string from port

#[test]
fn test_insb() {
    TestCase::from("6c").check();
}

#[test]
fn test_insw() {
    TestCase::from("66 6d").check();
}

#[test]
fn test_insd() {
    TestCase::from("6d").check();
}

#[test]
fn test_ins_m8_dx() {
    TestCase::from("6c").check();
}

#[test]
fn test_ins_m16_dx() {
    TestCase::from("66 6d").check();
}

#[test]
fn test_ins_m32_dx() {
    TestCase::from("6d").check();
}

#[test]
fn test_rep_insb() {
    TestCase::from("f3 6c").check();
}

#[test]
fn test_rep_insw() {
    TestCase::from("f3 66 6d").check();
}

#[test]
fn test_rep_insd() {
    TestCase::from("f3 6d").check();
}

// OUTS/OUTSB/OUTSW/OUTSD - Output string to port

#[test]
fn test_outsb() {
    TestCase::from("6e").check();
}

#[test]
fn test_outsw() {
    TestCase::from("66 6f").check();
}

#[test]
fn test_outsd() {
    TestCase::from("6f").check();
}

#[test]
fn test_outs_dx_m8() {
    TestCase::from("6e").check();
}

#[test]
fn test_outs_dx_m16() {
    TestCase::from("66 6f").check();
}

#[test]
fn test_outs_dx_m32() {
    TestCase::from("6f").check();
}

#[test]
fn test_rep_outsb() {
    TestCase::from("f3 6e").check();
}

#[test]
fn test_rep_outsw() {
    TestCase::from("f3 66 6f").check();
}

#[test]
fn test_rep_outsd() {
    TestCase::from("f3 6f").check();
}

// REP/REPE/REPZ/REPNE/REPNZ - Repeat prefixes

#[test]
fn test_rep_prefix() {
    TestCase::from("f3").check();
}

#[test]
fn test_repe_prefix() {
    TestCase::from("f3").check();
}

#[test]
fn test_repz_prefix() {
    TestCase::from("f3").check();
}

#[test]
fn test_repne_prefix() {
    TestCase::from("f2").check();
}

#[test]
fn test_repnz_prefix() {
    TestCase::from("f2").check();
}

// LOOP variants

#[test]
fn test_loop_rel8() {
    TestCase::from("e2 fe").check();
}

#[test]
fn test_loope_rel8() {
    TestCase::from("e1 fe").check();
}

#[test]
fn test_loopz_rel8() {
    TestCase::from("e1 fe").check();
}

#[test]
fn test_loopne_rel8() {
    TestCase::from("e0 fe").check();
}

#[test]
fn test_loopnz_rel8() {
    TestCase::from("e0 fe").check();
}
