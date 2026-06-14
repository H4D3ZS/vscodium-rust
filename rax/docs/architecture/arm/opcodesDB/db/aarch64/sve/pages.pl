#BEGIN_HEADER
#
# Copyright (C) 2020 Mahdi Safsafi.
#
# https://github.com/MahdiSafsafi/opcodesDB
#
# See licence file 'LICENCE' for use and distribution rights.
#
#END_HEADER

use strict;
use warnings;

PAGE abs_z_p_z => {
  title      => 'ABS',
  brief      => 'Absolute value (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ABS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE adclb_z_zzz => {
  title      => 'ADCLB',
  brief      => 'Add with carry long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE adclt_z_zzz => {
  title      => 'ADCLT',
  brief      => 'Add with carry long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADCLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE add_z_p_zz => {
  title      => 'ADD (vectors, predicated)',
  brief      => 'Add vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE add_z_zi => {
  title      => 'ADD (immediate)',
  brief      => 'Add immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE add_z_zz => {
  title      => 'ADD (vectors, unpredicated)',
  brief      => 'Add vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE addhnb_z_zz => {
  title      => 'ADDHNB',
  brief      => 'Add narrow high part (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE addhnt_z_zz => {
  title      => 'ADDHNT',
  brief      => 'Add narrow high part (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDHNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE addp_z_p_zz => {
  title      => 'ADDP',
  brief      => 'Add pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=ADDP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE addpl_r_ri => {
  title      => 'ADDPL',
  brief      => 'Add multiple of predicate register size to scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDPL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE addvl_r_ri => {
  title      => 'ADDVL',
  brief      => 'Add multiple of vector register size to scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADDVL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE adr_z_az => {
  title      => 'ADR',
  brief      => 'Compute vector address',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ADR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE aesd_z_zz => {
  title      => 'AESD',
  brief      => 'AES single round decryption',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE aese_z_zz => {
  title      => 'AESE',
  brief      => 'AES single round encryption',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE aesimc_z_z => {
  title      => 'AESIMC',
  brief      => 'AES inverse mix columns',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESIMC status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE aesmc_z_z => {
  title      => 'AESMC',
  brief      => 'AES mix columns',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=AESMC status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE and_p_p_pp => {
  title      => 'AND, ANDS (predicates)',
  brief      => 'Bitwise AND predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE and_z_p_zz => {
  title      => 'AND (vectors, predicated)',
  brief      => 'Bitwise AND vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE and_z_zi => {
  title      => 'AND (immediate)',
  brief      => 'Bitwise AND with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE and_z_zz => {
  title      => 'AND (vectors, unpredicated)',
  brief      => 'Bitwise AND vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE andv_r_p_z => {
  title      => 'ANDV',
  brief      => 'Bitwise AND reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ANDV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE asr_z_p_zi => {
  title      => 'ASR (immediate, predicated)',
  brief      => 'Arithmetic shift right by immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE asr_z_p_zw => {
  title      => 'ASR (wide elements, predicated)',
  brief      => 'Arithmetic shift right by 64-bit wide elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE asr_z_p_zz => {
  title      => 'ASR (vectors)',
  brief      => 'Arithmetic shift right by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE asr_z_zi => {
  title      => 'ASR (immediate, unpredicated)',
  brief      => 'Arithmetic shift right by immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE asr_z_zw => {
  title      => 'ASR (wide elements, unpredicated)',
  brief      => 'Arithmetic shift right by 64-bit wide elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE asrd_z_p_zi => {
  title      => 'ASRD',
  brief      => 'Arithmetic shift right for divide by immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE asrr_z_p_zz => {
  title      => 'ASRR',
  brief      => 'Reversed arithmetic shift right by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ASRR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE bcax_z_zzz => {
  title      => 'BCAX',
  brief      => 'Bitwise clear and exclusive OR',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BCAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bdep_z_zz => {
  title      => 'BDEP',
  brief      => 'Scatter lower bits into positions selected by bitmask',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BDEP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bext_z_zz => {
  title      => 'BEXT',
  brief      => 'Gather lower bits from positions selected by bitmask',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BEXT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bfcvt_z_p_z => {
  title      => 'BFCVT',
  brief      => 'Floating-point down convert to BFloat16 format (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE bfcvtnt_z_p_z => {
  title      => 'BFCVTNT',
  brief      => 'Floating-point down convert and narrow to BFloat16 (top, predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFCVTNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE bfdot_z_zzz => {
  title      => 'BFDOT (vectors)',
  brief      => 'BFloat16 floating-point dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfdot_z_zzzi => {
  title      => 'BFDOT (indexed)',
  brief      => 'BFloat16 floating-point indexed dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfmlalb_z_zzz => {
  title      => 'BFMLALB (vectors)',
  brief      => 'BFloat16 floating-point multiply-add long to single-precision (bottom)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfmlalb_z_zzzi => {
  title      => 'BFMLALB (indexed)',
  brief      => 'BFloat16 floating-point multiply-add long to single-precision (bottom, indexed)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfmlalt_z_zzz => {
  title      => 'BFMLALT (vectors)',
  brief      => 'BFloat16 floating-point multiply-add long to single-precision (top)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfmlalt_z_zzzi => {
  title      => 'BFMLALT (indexed)',
  brief      => 'BFloat16 floating-point multiply-add long to single-precision (top, indexed)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bfmmla_z_zzz => {
  title      => 'BFMMLA',
  brief      => 'BFloat16 floating-point matrix multiply-accumulate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BFMMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE bgrp_z_zz => {
  title      => 'BGRP',
  brief      => 'Group bits to right or left as selected by bitmask',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BGRP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bic_p_p_pp => {
  title      => 'BIC, BICS (predicates)',
  brief      => 'Bitwise clear predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bic_z_p_zz => {
  title      => 'BIC (vectors, predicated)',
  brief      => 'Bitwise clear vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE bic_z_zz => {
  title      => 'BIC (vectors, unpredicated)',
  brief      => 'Bitwise clear vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=BIC status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE brka_p_p_p => {
  title      => 'BRKA, BRKAS',
  brief      => 'Break after first true condition',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE brkb_p_p_p => {
  title      => 'BRKB, BRKBS',
  brief      => 'Break before first true condition',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE brkn_p_p_pp => {
  title      => 'BRKN, BRKNS',
  brief      => 'Propagate break to next partition',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE brkpa_p_p_pp => {
  title      => 'BRKPA, BRKPAS',
  brief      => 'Break after first true condition, propagating from previous partition',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE brkpb_p_p_pp => {
  title      => 'BRKPB, BRKPBS',
  brief      => 'Break before first true condition, propagating from previous partition',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE bsl1n_z_zzz => {
  title      => 'BSL1N',
  brief      => 'Bitwise select with first input inverted',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL1N status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bsl2n_z_zzz => {
  title      => 'BSL2N',
  brief      => 'Bitwise select with second input inverted',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL2N status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE bsl_z_zzz => {
  title      => 'BSL',
  brief      => 'Bitwise select',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=BSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cadd_z_zz => {
  title      => 'CADD',
  brief      => 'Complex integer add with rotate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cdot_z_zzz => {
  title      => 'CDOT (vectors)',
  brief      => 'Complex integer dot product',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE cdot_z_zzzi => {
  title      => 'CDOT (indexed)',
  brief      => 'Complex integer dot product (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clasta_r_p_z => {
  title      => 'CLASTA (scalar)',
  brief      => 'Conditionally extract element after last to general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clasta_v_p_z => {
  title      => 'CLASTA (SIMD&FP scalar)',
  brief      => 'Conditionally extract element after last to SIMD&FP scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clasta_z_p_zz => {
  title      => 'CLASTA (vectors)',
  brief      => 'Conditionally extract element after last to vector register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clastb_r_p_z => {
  title      => 'CLASTB (scalar)',
  brief      => 'Conditionally extract last element to general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clastb_v_p_z => {
  title      => 'CLASTB (SIMD&FP scalar)',
  brief      => 'Conditionally extract last element to SIMD&FP scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE clastb_z_p_zz => {
  title      => 'CLASTB (vectors)',
  brief      => 'Conditionally extract last element to vector register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLASTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE cls_z_p_z => {
  title      => 'CLS',
  brief      => 'Count leading sign bits (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE clz_z_p_z => {
  title      => 'CLZ',
  brief      => 'Count leading zero bits (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CLZ status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE cmla_z_zzz => {
  title      => 'CMLA (vectors)',
  brief      => 'Complex integer multiply-add with rotate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cmla_z_zzzi => {
  title      => 'CMLA (indexed)',
  brief      => 'Complex integer multiply-add with rotate (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=CMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cmpeq_p_p_zi => {
  title      => 'CMP<cc> (immediate)',
  brief      => 'Compare vector to immediate',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cmpeq_p_p_zw => {
  title      => 'CMP<cc> (wide elements)',
  brief      => 'Compare vector to 64-bit wide elements',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cmpeq_p_p_zz => {
  title      => 'CMP<cc> (vectors)',
  brief      => 'Compare vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cnot_z_p_z => {
  title      => 'CNOT',
  brief      => 'Logically invert boolean condition in vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE cnt_z_p_z => {
  title      => 'CNT',
  brief      => 'Count non-zero bits (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE cntb_r_s => {
  title      => 'CNTB, CNTD, CNTH, CNTW',
  brief      => 'Set scalar to multiple of predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cntp_r_p_p => {
  title      => 'CNTP',
  brief      => 'Set scalar to count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CNTP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE compact_z_p_z => {
  title      => 'COMPACT',
  brief      => 'Shuffle active elements of vector to the right and fill with zero',
  docvars    => 'instr-class=sve isa=A64 mnemonic=COMPACT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE cpy_z_o_i => {
  title      => 'CPY (immediate, zeroing)',
  brief      => 'Copy signed integer immediate to vector elements (zeroing)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE cpy_z_p_i => {
  title      => 'CPY (immediate, merging)',
  brief      => 'Copy signed integer immediate to vector elements (merging)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE cpy_z_p_r => {
  title      => 'CPY (scalar)',
  brief      => 'Copy general-purpose register to vector elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE cpy_z_p_v => {
  title      => 'CPY (SIMD&FP scalar)',
  brief      => 'Copy SIMD&FP scalar register to vector elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE ctermeq_rr => {
  title      => 'CTERMEQ, CTERMNE',
  brief      => 'Compare and terminate loop',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE decb_r_rs => {
  title      => 'DECB, DECD, DECH, DECW (scalar)',
  brief      => 'Decrement scalar by multiple of predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE decd_z_zs => {
  title      => 'DECD, DECH, DECW (vector)',
  brief      => 'Decrement vector by multiple of predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE decp_r_p_r => {
  title      => 'DECP (scalar)',
  brief      => 'Decrement scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE decp_z_p_z => {
  title      => 'DECP (vector)',
  brief      => 'Decrement vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE dup_z_i => {
  title      => 'DUP (immediate)',
  brief      => 'Broadcast signed immediate to vector elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE dup_z_r => {
  title      => 'DUP (scalar)',
  brief      => 'Broadcast general-purpose register to vector elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE dup_z_zi => {
  title      => 'DUP (indexed)',
  brief      => 'Broadcast indexed element to vector (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE dupm_z_i => {
  title      => 'DUPM',
  brief      => 'Broadcast logical bitmask immediate to vector (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eor3_z_zzz => {
  title      => 'EOR3',
  brief      => 'Bitwise exclusive OR of three vectors',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EOR3 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eor_p_p_pp => {
  title      => 'EOR, EORS (predicates)',
  brief      => 'Bitwise exclusive OR predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eor_z_p_zz => {
  title      => 'EOR (vectors, predicated)',
  brief      => 'Bitwise exclusive OR vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE eor_z_zi => {
  title      => 'EOR (immediate)',
  brief      => 'Bitwise exclusive OR with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eor_z_zz => {
  title      => 'EOR (vectors, unpredicated)',
  brief      => 'Bitwise exclusive OR vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eorbt_z_zz => {
  title      => 'EORBT',
  brief      => 'Interleaving exclusive OR (bottom, top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORBT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eortb_z_zz => {
  title      => 'EORTB',
  brief      => 'Interleaving exclusive OR (top, bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=EORTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE eorv_r_p_z => {
  title      => 'EORV',
  brief      => 'Bitwise exclusive OR reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EORV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ext_z_zi => {
  title      => 'EXT',
  brief      => 'Extract vector from pair of vectors',
  docvars    => 'instr-class=sve isa=A64 mnemonic=EXT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE fabd_z_p_zz => {
  title      => 'FABD',
  brief      => 'Floating-point absolute difference (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fabs_z_p_z => {
  title      => 'FABS',
  brief      => 'Floating-point absolute value (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FABS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE facge_p_p_zz => {
  title      => 'FAC<cc>',
  brief      => 'Floating-point absolute compare vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fadd_z_p_zs => {
  title      => 'FADD (immediate)',
  brief      => 'Floating-point add immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fadd_z_p_zz => {
  title      => 'FADD (vectors, predicated)',
  brief      => 'Floating-point add vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fadd_z_zz => {
  title      => 'FADD (vectors, unpredicated)',
  brief      => 'Floating-point add vector (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fadda_v_p_z => {
  title      => 'FADDA',
  brief      => 'Floating-point add strictly-ordered reduction, accumulating in scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE faddp_z_p_zz => {
  title      => 'FADDP',
  brief      => 'Floating-point add pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FADDP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE faddv_v_p_z => {
  title      => 'FADDV',
  brief      => 'Floating-point add recursive reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FADDV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcadd_z_p_zz => {
  title      => 'FCADD',
  brief      => 'Floating-point complex add with rotate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcmeq_p_p_z0 => {
  title      => 'FCM<cc> (zero)',
  brief      => 'Floating-point compare vector with zero',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcmeq_p_p_zz => {
  title      => 'FCM<cc> (vectors)',
  brief      => 'Floating-point compare vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcmla_z_p_zzz => {
  title      => 'FCMLA (vectors)',
  brief      => 'Floating-point complex multiply-add with rotate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcmla_z_zzzi => {
  title      => 'FCMLA (indexed)',
  brief      => 'Floating-point complex multiply-add by indexed values with rotate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fcpy_z_p_i => {
  title      => 'FCPY',
  brief      => 'Copy 8-bit floating-point immediate to vector elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcvt_z_p_z => {
  title      => 'FCVT',
  brief      => 'Floating-point convert precision (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCVT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcvtlt_z_p_z => {
  title      => 'FCVTLT',
  brief      => 'Floating-point up convert long (top, predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FCVTLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcvtnt_z_p_z => {
  title      => 'FCVTNT',
  brief      => 'Floating-point down convert and narrow (top, predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FCVTNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcvtx_z_p_z => {
  title      => 'FCVTX',
  brief      => 'Floating-point down convert, rounding to odd (predicated)',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcvtxnt_z_p_z => {
  title      => 'FCVTXNT',
  brief      => 'Floating-point down convert, rounding to odd (top, predicated)',
  docvars    => 'convert-type=double-to-single instr-class=sve2 isa=A64 mnemonic=FCVTXNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fcvtzs_z_p_z => {
  title      => 'FCVTZS',
  brief      => 'Floating-point convert to signed integer, rounding toward zero (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCVTZS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fcvtzu_z_p_z => {
  title      => 'FCVTZU',
  brief      => 'Floating-point convert to unsigned integer, rounding toward zero (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FCVTZU status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fdiv_z_p_zz => {
  title      => 'FDIV',
  brief      => 'Floating-point divide by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fdivr_z_p_zz => {
  title      => 'FDIVR',
  brief      => 'Floating-point reversed divide by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDIVR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fdup_z_i => {
  title      => 'FDUP',
  brief      => 'Broadcast 8-bit floating-point immediate to vector elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fexpa_z_z => {
  title      => 'FEXPA',
  brief      => 'Floating-point exponential accelerator',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FEXPA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE flogb_z_p_z => {
  title      => 'FLOGB',
  brief      => 'Floating-point base 2 logarithm as integer',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FLOGB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmad_z_p_zzz => {
  title      => 'FMAD',
  brief      => 'Floating-point fused multiply-add vectors (predicated), writing multiplicand [Zdn = Za + Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmax_z_p_zs => {
  title      => 'FMAX (immediate)',
  brief      => 'Floating-point maximum with immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmax_z_p_zz => {
  title      => 'FMAX (vectors)',
  brief      => 'Floating-point maximum (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmaxnm_z_p_zs => {
  title      => 'FMAXNM (immediate)',
  brief      => 'Floating-point maximum number with immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmaxnm_z_p_zz => {
  title      => 'FMAXNM (vectors)',
  brief      => 'Floating-point maximum number (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNM status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmaxnmp_z_p_zz => {
  title      => 'FMAXNMP',
  brief      => 'Floating-point maximum number pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXNMP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmaxnmv_v_p_z => {
  title      => 'FMAXNMV',
  brief      => 'Floating-point maximum number recursive reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXNMV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fmaxp_z_p_zz => {
  title      => 'FMAXP',
  brief      => 'Floating-point maximum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMAXP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmaxv_v_p_z => {
  title      => 'FMAXV',
  brief      => 'Floating-point maximum recursive reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMAXV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fmin_z_p_zs => {
  title      => 'FMIN (immediate)',
  brief      => 'Floating-point minimum with immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmin_z_p_zz => {
  title      => 'FMIN (vectors)',
  brief      => 'Floating-point minimum (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fminnm_z_p_zs => {
  title      => 'FMINNM (immediate)',
  brief      => 'Floating-point minimum number with immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fminnm_z_p_zz => {
  title      => 'FMINNM (vectors)',
  brief      => 'Floating-point minimum number (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNM status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fminnmp_z_p_zz => {
  title      => 'FMINNMP',
  brief      => 'Floating-point minimum number pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINNMP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fminnmv_v_p_z => {
  title      => 'FMINNMV',
  brief      => 'Floating-point minimum number recursive reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINNMV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fminp_z_p_zz => {
  title      => 'FMINP',
  brief      => 'Floating-point minimum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMINP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fminv_v_p_z => {
  title      => 'FMINV',
  brief      => 'Floating-point minimum recursive reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMINV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fmla_z_p_zzz => {
  title      => 'FMLA (vectors)',
  brief      => 'Floating-point fused multiply-add vectors (predicated), writing addend [Zda = Zda + Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmla_z_zzzi => {
  title      => 'FMLA (indexed)',
  brief      => 'Floating-point fused multiply-add by indexed elements (Zda = Zda + Zn * Zm[indexed])',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlalb_z_zzz => {
  title      => 'FMLALB (vectors)',
  brief      => 'Half-precision floating-point multiply-add long to single-precision (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlalb_z_zzzi => {
  title      => 'FMLALB (indexed)',
  brief      => 'Half-precision floating-point multiply-add long to single-precision (bottom, indexed)',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlalt_z_zzz => {
  title      => 'FMLALT (vectors)',
  brief      => 'Half-precision floating-point multiply-add long to single-precision (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlalt_z_zzzi => {
  title      => 'FMLALT (indexed)',
  brief      => 'Half-precision floating-point multiply-add long to single-precision (top, indexed)',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmls_z_p_zzz => {
  title      => 'FMLS (vectors)',
  brief      => 'Floating-point fused multiply-subtract vectors (predicated), writing addend [Zda = Zda + -Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmls_z_zzzi => {
  title      => 'FMLS (indexed)',
  brief      => 'Floating-point fused multiply-subtract by indexed elements (Zda = Zda + -Zn * Zm[indexed])',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlslb_z_zzz => {
  title      => 'FMLSLB (vectors)',
  brief      => 'Half-precision floating-point multiply-subtract long from single-precision (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlslb_z_zzzi => {
  title      => 'FMLSLB (indexed)',
  brief      => 'Half-precision floating-point multiply-subtract long from single-precision (bottom, indexed)',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlslt_z_zzz => {
  title      => 'FMLSLT (vectors)',
  brief      => 'Half-precision floating-point multiply-subtract long from single-precision (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmlslt_z_zzzi => {
  title      => 'FMLSLT (indexed)',
  brief      => 'Half-precision floating-point multiply-subtract long from single-precision (top, indexed)',
  docvars    => 'datatype=single instr-class=sve2 isa=A64 mnemonic=FMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmmla_z_zzz => {
  title      => 'FMMLA',
  brief      => 'Floating-point matrix multiply-accumulate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE fmsb_z_p_zzz => {
  title      => 'FMSB',
  brief      => 'Floating-point fused multiply-subtract vectors (predicated), writing multiplicand [Zdn = Za + -Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMSB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmul_z_p_zs => {
  title      => 'FMUL (immediate)',
  brief      => 'Floating-point multiply by immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmul_z_p_zz => {
  title      => 'FMUL (vectors, predicated)',
  brief      => 'Floating-point multiply vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fmul_z_zz => {
  title      => 'FMUL (vectors, unpredicated)',
  brief      => 'Floating-point multiply vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fmul_z_zzi => {
  title      => 'FMUL (indexed)',
  brief      => 'Floating-point multiply by indexed elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fmulx_z_p_zz => {
  title      => 'FMULX',
  brief      => 'Floating-point multiply-extended vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FMULX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fneg_z_p_z => {
  title      => 'FNEG',
  brief      => 'Floating-point negate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNEG status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fnmad_z_p_zzz => {
  title      => 'FNMAD',
  brief      => 'Floating-point negated fused multiply-add vectors (predicated), writing multiplicand [Zdn = -Za + -Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMAD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fnmla_z_p_zzz => {
  title      => 'FNMLA',
  brief      => 'Floating-point negated fused multiply-add vectors (predicated), writing addend [Zda = -Zda + -Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fnmls_z_p_zzz => {
  title      => 'FNMLS',
  brief      => 'Floating-point negated fused multiply-subtract vectors (predicated), writing addend [Zda = -Zda + Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fnmsb_z_p_zzz => {
  title      => 'FNMSB',
  brief      => 'Floating-point negated fused multiply-subtract vectors (predicated), writing multiplicand [Zdn = -Za + Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FNMSB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE frecpe_z_z => {
  title      => 'FRECPE',
  brief      => 'Floating-point reciprocal estimate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE frecps_z_zz => {
  title      => 'FRECPS',
  brief      => 'Floating-point reciprocal step (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE frecpx_z_p_z => {
  title      => 'FRECPX',
  brief      => 'Floating-point reciprocal exponent (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRECPX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE frinta_z_p_z => {
  title      => 'FRINT<r>',
  brief      => 'Floating-point round to integral value (predicated)',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE frsqrte_z_z => {
  title      => 'FRSQRTE',
  brief      => 'Floating-point reciprocal square root estimate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE frsqrts_z_zz => {
  title      => 'FRSQRTS',
  brief      => 'Floating-point reciprocal square root step (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FRSQRTS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fscale_z_p_zz => {
  title      => 'FSCALE',
  brief      => 'Floating-point adjust exponent by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSCALE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fsqrt_z_p_z => {
  title      => 'FSQRT',
  brief      => 'Floating-point square root (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSQRT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fsub_z_p_zs => {
  title      => 'FSUB (immediate)',
  brief      => 'Floating-point subtract immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fsub_z_p_zz => {
  title      => 'FSUB (vectors, predicated)',
  brief      => 'Floating-point subtract vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fsub_z_zz => {
  title      => 'FSUB (vectors, unpredicated)',
  brief      => 'Floating-point subtract vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE fsubr_z_p_zs => {
  title      => 'FSUBR (immediate)',
  brief      => 'Floating-point reversed subtract from immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE fsubr_z_p_zz => {
  title      => 'FSUBR (vectors)',
  brief      => 'Floating-point reversed subtract vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE ftmad_z_zzi => {
  title      => 'FTMAD',
  brief      => 'Floating-point trigonometric multiply-add coefficient',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTMAD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0',
};

PAGE ftsmul_z_zz => {
  title      => 'FTSMUL',
  brief      => 'Floating-point trigonometric starting value',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE ftssel_z_zz => {
  title      => 'FTSSEL',
  brief      => 'Floating-point trigonometric select coefficient',
  docvars    => 'instr-class=sve isa=A64 mnemonic=FTSSEL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE histcnt_z_p_zz => {
  title      => 'HISTCNT',
  brief      => 'Count matching elements in vector',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTCNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE histseg_z_zz => {
  title      => 'HISTSEG',
  brief      => 'Count matching elements in vector segments',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=HISTSEG status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE incb_r_rs => {
  title      => 'INCB, INCD, INCH, INCW (scalar)',
  brief      => 'Increment scalar by multiple of predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE incd_z_zs => {
  title      => 'INCD, INCH, INCW (vector)',
  brief      => 'Increment vector by multiple of predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE incp_r_p_r => {
  title      => 'INCP (scalar)',
  brief      => 'Increment scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE incp_z_p_z => {
  title      => 'INCP (vector)',
  brief      => 'Increment vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE index_z_ii => {
  title      => 'INDEX (immediates)',
  brief      => 'Create index starting from and incremented by immediate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE index_z_ir => {
  title      => 'INDEX (immediate, scalar)',
  brief      => 'Create index starting from immediate and incremented by general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE index_z_ri => {
  title      => 'INDEX (scalar, immediate)',
  brief      => 'Create index starting from general-purpose register and incremented by immediate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE index_z_rr => {
  title      => 'INDEX (scalars)',
  brief      => 'Create index starting from and incremented by general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INDEX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE insr_z_r => {
  title      => 'INSR (scalar)',
  brief      => 'Insert general-purpose register in shifted vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE insr_z_v => {
  title      => 'INSR (SIMD&FP scalar)',
  brief      => 'Insert SIMD&FP scalar register in shifted vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=INSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE lasta_r_p_z => {
  title      => 'LASTA (scalar)',
  brief      => 'Extract element after last to general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE lasta_v_p_z => {
  title      => 'LASTA (SIMD&FP scalar)',
  brief      => 'Extract element after last to SIMD&FP scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE lastb_r_p_z => {
  title      => 'LASTB (scalar)',
  brief      => 'Extract last element to general-purpose register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE lastb_v_p_z => {
  title      => 'LASTB (SIMD&FP scalar)',
  brief      => 'Extract last element to SIMD&FP scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LASTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE ld1b_z_p_ai => {
  title      => 'LD1B (vector plus immediate)',
  brief      => 'Gather load unsigned bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1b_z_p_bi => {
  title      => 'LD1B (scalar plus immediate)',
  brief      => 'Contiguous load unsigned bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1b_z_p_br => {
  title      => 'LD1B (scalar plus scalar)',
  brief      => 'Contiguous load unsigned bytes to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1b_z_p_bz => {
  title      => 'LD1B (scalar plus vector)',
  brief      => 'Gather load unsigned bytes to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1d_z_p_ai => {
  title      => 'LD1D (vector plus immediate)',
  brief      => 'Gather load doublewords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1d_z_p_bi => {
  title      => 'LD1D (scalar plus immediate)',
  brief      => 'Contiguous load doublewords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1d_z_p_br => {
  title      => 'LD1D (scalar plus scalar)',
  brief      => 'Contiguous load doublewords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1d_z_p_bz => {
  title      => 'LD1D (scalar plus vector)',
  brief      => 'Gather load doublewords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1h_z_p_ai => {
  title      => 'LD1H (vector plus immediate)',
  brief      => 'Gather load unsigned halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1h_z_p_bi => {
  title      => 'LD1H (scalar plus immediate)',
  brief      => 'Contiguous load unsigned halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1h_z_p_br => {
  title      => 'LD1H (scalar plus scalar)',
  brief      => 'Contiguous load unsigned halfwords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1h_z_p_bz => {
  title      => 'LD1H (scalar plus vector)',
  brief      => 'Gather load unsigned halfwords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rb_z_p_bi => {
  title      => 'LD1RB',
  brief      => 'Load and broadcast unsigned byte to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rd_z_p_bi => {
  title      => 'LD1RD',
  brief      => 'Load and broadcast doubleword to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rh_z_p_bi => {
  title      => 'LD1RH',
  brief      => 'Load and broadcast unsigned halfword to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rob_z_p_bi => {
  title      => 'LD1ROB (scalar plus immediate)',
  brief      => 'Contiguous load and replicate thirty-two bytes (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rob_z_p_br => {
  title      => 'LD1ROB (scalar plus scalar)',
  brief      => 'Contiguous load and replicate thirty-two bytes (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROB status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rod_z_p_bi => {
  title      => 'LD1ROD (scalar plus immediate)',
  brief      => 'Contiguous load and replicate four doublewords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rod_z_p_br => {
  title      => 'LD1ROD (scalar plus scalar)',
  brief      => 'Contiguous load and replicate four doublewords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROD status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1roh_z_p_bi => {
  title      => 'LD1ROH (scalar plus immediate)',
  brief      => 'Contiguous load and replicate sixteen halfwords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1roh_z_p_br => {
  title      => 'LD1ROH (scalar plus scalar)',
  brief      => 'Contiguous load and replicate sixteen halfwords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROH status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1row_z_p_bi => {
  title      => 'LD1ROW (scalar plus immediate)',
  brief      => 'Contiguous load and replicate eight words (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1row_z_p_br => {
  title      => 'LD1ROW (scalar plus scalar)',
  brief      => 'Contiguous load and replicate eight words (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1ROW status=Amber',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqb_z_p_bi => {
  title      => 'LD1RQB (scalar plus immediate)',
  brief      => 'Contiguous load and replicate sixteen bytes (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green sve-elem-type=8-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqb_z_p_br => {
  title      => 'LD1RQB (scalar plus scalar)',
  brief      => 'Contiguous load and replicate sixteen bytes (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqd_z_p_bi => {
  title      => 'LD1RQD (scalar plus immediate)',
  brief      => 'Contiguous load and replicate two doublewords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqd_z_p_br => {
  title      => 'LD1RQD (scalar plus scalar)',
  brief      => 'Contiguous load and replicate two doublewords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqh_z_p_bi => {
  title      => 'LD1RQH (scalar plus immediate)',
  brief      => 'Contiguous load and replicate eight halfwords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green sve-elem-type=16-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqh_z_p_br => {
  title      => 'LD1RQH (scalar plus scalar)',
  brief      => 'Contiguous load and replicate eight halfwords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqw_z_p_bi => {
  title      => 'LD1RQW (scalar plus immediate)',
  brief      => 'Contiguous load and replicate four words (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green sve-elem-type=32-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rqw_z_p_br => {
  title      => 'LD1RQW (scalar plus scalar)',
  brief      => 'Contiguous load and replicate four words (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RQW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rsb_z_p_bi => {
  title      => 'LD1RSB',
  brief      => 'Load and broadcast signed byte to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rsh_z_p_bi => {
  title      => 'LD1RSH',
  brief      => 'Load and broadcast signed halfword to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rsw_z_p_bi => {
  title      => 'LD1RSW',
  brief      => 'Load and broadcast signed word to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RSW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1rw_z_p_bi => {
  title      => 'LD1RW',
  brief      => 'Load and broadcast unsigned word to vector',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1RW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sb_z_p_ai => {
  title      => 'LD1SB (vector plus immediate)',
  brief      => 'Gather load signed bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sb_z_p_bi => {
  title      => 'LD1SB (scalar plus immediate)',
  brief      => 'Contiguous load signed bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sb_z_p_br => {
  title      => 'LD1SB (scalar plus scalar)',
  brief      => 'Contiguous load signed bytes to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sb_z_p_bz => {
  title      => 'LD1SB (scalar plus vector)',
  brief      => 'Gather load signed bytes to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sh_z_p_ai => {
  title      => 'LD1SH (vector plus immediate)',
  brief      => 'Gather load signed halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sh_z_p_bi => {
  title      => 'LD1SH (scalar plus immediate)',
  brief      => 'Contiguous load signed halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sh_z_p_br => {
  title      => 'LD1SH (scalar plus scalar)',
  brief      => 'Contiguous load signed halfwords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sh_z_p_bz => {
  title      => 'LD1SH (scalar plus vector)',
  brief      => 'Gather load signed halfwords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sw_z_p_ai => {
  title      => 'LD1SW (vector plus immediate)',
  brief      => 'Gather load signed words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sw_z_p_bi => {
  title      => 'LD1SW (scalar plus immediate)',
  brief      => 'Contiguous load signed words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sw_z_p_br => {
  title      => 'LD1SW (scalar plus scalar)',
  brief      => 'Contiguous load signed words to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1sw_z_p_bz => {
  title      => 'LD1SW (scalar plus vector)',
  brief      => 'Gather load signed words to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1w_z_p_ai => {
  title      => 'LD1W (vector plus immediate)',
  brief      => 'Gather load unsigned words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1w_z_p_bi => {
  title      => 'LD1W (scalar plus immediate)',
  brief      => 'Contiguous load unsigned words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1w_z_p_br => {
  title      => 'LD1W (scalar plus scalar)',
  brief      => 'Contiguous load unsigned words to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld1w_z_p_bz => {
  title      => 'LD1W (scalar plus vector)',
  brief      => 'Gather load unsigned words to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2b_z_p_bi => {
  title      => 'LD2B (scalar plus immediate)',
  brief      => 'Contiguous load two-byte structures to two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2b_z_p_br => {
  title      => 'LD2B (scalar plus scalar)',
  brief      => 'Contiguous load two-byte structures to two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2d_z_p_bi => {
  title      => 'LD2D (scalar plus immediate)',
  brief      => 'Contiguous load two-doubleword structures to two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2d_z_p_br => {
  title      => 'LD2D (scalar plus scalar)',
  brief      => 'Contiguous load two-doubleword structures to two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2h_z_p_bi => {
  title      => 'LD2H (scalar plus immediate)',
  brief      => 'Contiguous load two-halfword structures to two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2h_z_p_br => {
  title      => 'LD2H (scalar plus scalar)',
  brief      => 'Contiguous load two-halfword structures to two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2w_z_p_bi => {
  title      => 'LD2W (scalar plus immediate)',
  brief      => 'Contiguous load two-word structures to two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld2w_z_p_br => {
  title      => 'LD2W (scalar plus scalar)',
  brief      => 'Contiguous load two-word structures to two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD2W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3b_z_p_bi => {
  title      => 'LD3B (scalar plus immediate)',
  brief      => 'Contiguous load three-byte structures to three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3b_z_p_br => {
  title      => 'LD3B (scalar plus scalar)',
  brief      => 'Contiguous load three-byte structures to three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3d_z_p_bi => {
  title      => 'LD3D (scalar plus immediate)',
  brief      => 'Contiguous load three-doubleword structures to three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3d_z_p_br => {
  title      => 'LD3D (scalar plus scalar)',
  brief      => 'Contiguous load three-doubleword structures to three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3h_z_p_bi => {
  title      => 'LD3H (scalar plus immediate)',
  brief      => 'Contiguous load three-halfword structures to three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3h_z_p_br => {
  title      => 'LD3H (scalar plus scalar)',
  brief      => 'Contiguous load three-halfword structures to three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3w_z_p_bi => {
  title      => 'LD3W (scalar plus immediate)',
  brief      => 'Contiguous load three-word structures to three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld3w_z_p_br => {
  title      => 'LD3W (scalar plus scalar)',
  brief      => 'Contiguous load three-word structures to three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD3W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4b_z_p_bi => {
  title      => 'LD4B (scalar plus immediate)',
  brief      => 'Contiguous load four-byte structures to four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4b_z_p_br => {
  title      => 'LD4B (scalar plus scalar)',
  brief      => 'Contiguous load four-byte structures to four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4d_z_p_bi => {
  title      => 'LD4D (scalar plus immediate)',
  brief      => 'Contiguous load four-doubleword structures to four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4d_z_p_br => {
  title      => 'LD4D (scalar plus scalar)',
  brief      => 'Contiguous load four-doubleword structures to four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4h_z_p_bi => {
  title      => 'LD4H (scalar plus immediate)',
  brief      => 'Contiguous load four-halfword structures to four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4h_z_p_br => {
  title      => 'LD4H (scalar plus scalar)',
  brief      => 'Contiguous load four-halfword structures to four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4w_z_p_bi => {
  title      => 'LD4W (scalar plus immediate)',
  brief      => 'Contiguous load four-word structures to four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ld4w_z_p_br => {
  title      => 'LD4W (scalar plus scalar)',
  brief      => 'Contiguous load four-word structures to four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LD4W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1b_z_p_ai => {
  title      => 'LDFF1B (vector plus immediate)',
  brief      => 'Gather load first-fault unsigned bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1b_z_p_br => {
  title      => 'LDFF1B (scalar plus scalar)',
  brief      => 'Contiguous load first-fault unsigned bytes to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1b_z_p_bz => {
  title      => 'LDFF1B (scalar plus vector)',
  brief      => 'Gather load first-fault unsigned bytes to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1d_z_p_ai => {
  title      => 'LDFF1D (vector plus immediate)',
  brief      => 'Gather load first-fault doublewords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1d_z_p_br => {
  title      => 'LDFF1D (scalar plus scalar)',
  brief      => 'Contiguous load first-fault doublewords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1d_z_p_bz => {
  title      => 'LDFF1D (scalar plus vector)',
  brief      => 'Gather load first-fault doublewords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1h_z_p_ai => {
  title      => 'LDFF1H (vector plus immediate)',
  brief      => 'Gather load first-fault unsigned halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1h_z_p_br => {
  title      => 'LDFF1H (scalar plus scalar)',
  brief      => 'Contiguous load first-fault unsigned halfwords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1h_z_p_bz => {
  title      => 'LDFF1H (scalar plus vector)',
  brief      => 'Gather load first-fault unsigned halfwords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sb_z_p_ai => {
  title      => 'LDFF1SB (vector plus immediate)',
  brief      => 'Gather load first-fault signed bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sb_z_p_br => {
  title      => 'LDFF1SB (scalar plus scalar)',
  brief      => 'Contiguous load first-fault signed bytes to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sb_z_p_bz => {
  title      => 'LDFF1SB (scalar plus vector)',
  brief      => 'Gather load first-fault signed bytes to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sh_z_p_ai => {
  title      => 'LDFF1SH (vector plus immediate)',
  brief      => 'Gather load first-fault signed halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sh_z_p_br => {
  title      => 'LDFF1SH (scalar plus scalar)',
  brief      => 'Contiguous load first-fault signed halfwords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sh_z_p_bz => {
  title      => 'LDFF1SH (scalar plus vector)',
  brief      => 'Gather load first-fault signed halfwords to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sw_z_p_ai => {
  title      => 'LDFF1SW (vector plus immediate)',
  brief      => 'Gather load first-fault signed words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sw_z_p_br => {
  title      => 'LDFF1SW (scalar plus scalar)',
  brief      => 'Contiguous load first-fault signed words to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1sw_z_p_bz => {
  title      => 'LDFF1SW (scalar plus vector)',
  brief      => 'Gather load first-fault signed words to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1SW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1w_z_p_ai => {
  title      => 'LDFF1W (vector plus immediate)',
  brief      => 'Gather load first-fault unsigned words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1w_z_p_br => {
  title      => 'LDFF1W (scalar plus scalar)',
  brief      => 'Contiguous load first-fault unsigned words to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldff1w_z_p_bz => {
  title      => 'LDFF1W (scalar plus vector)',
  brief      => 'Gather load first-fault unsigned words to vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDFF1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1b_z_p_bi => {
  title      => 'LDNF1B',
  brief      => 'Contiguous load non-fault unsigned bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1d_z_p_bi => {
  title      => 'LDNF1D',
  brief      => 'Contiguous load non-fault doublewords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1D status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1h_z_p_bi => {
  title      => 'LDNF1H',
  brief      => 'Contiguous load non-fault unsigned halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1sb_z_p_bi => {
  title      => 'LDNF1SB',
  brief      => 'Contiguous load non-fault signed bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1sh_z_p_bi => {
  title      => 'LDNF1SH',
  brief      => 'Contiguous load non-fault signed halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1sw_z_p_bi => {
  title      => 'LDNF1SW',
  brief      => 'Contiguous load non-fault signed words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1SW status=Green sve-elem-type=64-elem',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnf1w_z_p_bi => {
  title      => 'LDNF1W',
  brief      => 'Contiguous load non-fault unsigned words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNF1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1b_z_p_ar => {
  title      => 'LDNT1B (vector plus scalar)',
  brief      => 'Gather load non-temporal unsigned bytes',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1b_z_p_bi => {
  title      => 'LDNT1B (scalar plus immediate)',
  brief      => 'Contiguous load non-temporal bytes to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1b_z_p_br => {
  title      => 'LDNT1B (scalar plus scalar)',
  brief      => 'Contiguous load non-temporal bytes to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1d_z_p_ar => {
  title      => 'LDNT1D (vector plus scalar)',
  brief      => 'Gather load non-temporal unsigned doublewords',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1d_z_p_bi => {
  title      => 'LDNT1D (scalar plus immediate)',
  brief      => 'Contiguous load non-temporal doublewords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1d_z_p_br => {
  title      => 'LDNT1D (scalar plus scalar)',
  brief      => 'Contiguous load non-temporal doublewords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1h_z_p_ar => {
  title      => 'LDNT1H (vector plus scalar)',
  brief      => 'Gather load non-temporal unsigned halfwords',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1h_z_p_bi => {
  title      => 'LDNT1H (scalar plus immediate)',
  brief      => 'Contiguous load non-temporal halfwords to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1h_z_p_br => {
  title      => 'LDNT1H (scalar plus scalar)',
  brief      => 'Contiguous load non-temporal halfwords to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1sb_z_p_ar => {
  title      => 'LDNT1SB',
  brief      => 'Gather load non-temporal signed bytes',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1sh_z_p_ar => {
  title      => 'LDNT1SH',
  brief      => 'Gather load non-temporal signed halfwords',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1sw_z_p_ar => {
  title      => 'LDNT1SW',
  brief      => 'Gather load non-temporal signed words',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1SW status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1w_z_p_ar => {
  title      => 'LDNT1W (vector plus scalar)',
  brief      => 'Gather load non-temporal unsigned words',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1w_z_p_bi => {
  title      => 'LDNT1W (scalar plus immediate)',
  brief      => 'Contiguous load non-temporal words to vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldnt1w_z_p_br => {
  title      => 'LDNT1W (scalar plus scalar)',
  brief      => 'Contiguous load non-temporal words to vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ldr_p_bi => {
  title      => 'LDR (predicate)',
  brief      => 'Load predicate register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 alphaindex=SVE',
};

PAGE ldr_z_bi => {
  title      => 'LDR (vector)',
  brief      => 'Load vector register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LDR status=Green',
  metadata   => 'alias=0 alphaindex=SVE',
};

PAGE lsl_z_p_zi => {
  title      => 'LSL (immediate, predicated)',
  brief      => 'Logical shift left by immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsl_z_p_zw => {
  title      => 'LSL (wide elements, predicated)',
  brief      => 'Logical shift left by 64-bit wide elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsl_z_p_zz => {
  title      => 'LSL (vectors)',
  brief      => 'Logical shift left by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsl_z_zi => {
  title      => 'LSL (immediate, unpredicated)',
  brief      => 'Logical shift left by immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE lsl_z_zw => {
  title      => 'LSL (wide elements, unpredicated)',
  brief      => 'Logical shift left by 64-bit wide elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE lslr_z_p_zz => {
  title      => 'LSLR',
  brief      => 'Reversed logical shift left by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsr_z_p_zi => {
  title      => 'LSR (immediate, predicated)',
  brief      => 'Logical shift right by immediate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsr_z_p_zw => {
  title      => 'LSR (wide elements, predicated)',
  brief      => 'Logical shift right by 64-bit wide elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsr_z_p_zz => {
  title      => 'LSR (vectors)',
  brief      => 'Logical shift right by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE lsr_z_zi => {
  title      => 'LSR (immediate, unpredicated)',
  brief      => 'Logical shift right by immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE lsr_z_zw => {
  title      => 'LSR (wide elements, unpredicated)',
  brief      => 'Logical shift right by 64-bit wide elements (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE lsrr_z_p_zz => {
  title      => 'LSRR',
  brief      => 'Reversed logical shift right by vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=LSRR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE mad_z_p_zzz => {
  title      => 'MAD',
  brief      => 'Multiply-add vectors (predicated), writing multiplicand [Zdn = Za + Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MAD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE match_p_p_zz => {
  title      => 'MATCH',
  brief      => 'Detect any matching elements, setting the condition flags',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MATCH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE mla_z_p_zzz => {
  title      => 'MLA (vectors)',
  brief      => 'Multiply-add vectors (predicated), writing addend [Zda = Zda + Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE mla_z_zzzi => {
  title      => 'MLA (indexed)',
  brief      => 'Multiply-add to accumulator (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE mls_z_p_zzz => {
  title      => 'MLS (vectors)',
  brief      => 'Multiply-subtract vectors (predicated), writing addend [Zda = Zda - Zn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE mls_z_zzzi => {
  title      => 'MLS (indexed)',
  brief      => 'Multiply-subtract from accumulator (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MLS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE movprfx_z_p_z => {
  title      => 'MOVPRFX (predicated)',
  brief      => 'Move prefix (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE movprfx_z_z => {
  title      => 'MOVPRFX (unpredicated)',
  brief      => 'Move prefix (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MOVPRFX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE msb_z_p_zzz => {
  title      => 'MSB',
  brief      => 'Multiply-subtract vectors (predicated), writing multiplicand [Zdn = Za - Zdn * Zm]',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MSB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE mul_z_p_zz => {
  title      => 'MUL (vectors, predicated)',
  brief      => 'Multiply vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE mul_z_zi => {
  title      => 'MUL (immediate)',
  brief      => 'Multiply by immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE mul_z_zz => {
  title      => 'MUL (vectors, unpredicated)',
  brief      => 'Multiply vectors (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE mul_z_zzi => {
  title      => 'MUL (indexed)',
  brief      => 'Multiply (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=MUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE nand_p_p_pp => {
  title      => 'NAND, NANDS',
  brief      => 'Bitwise NAND predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE nbsl_z_zzz => {
  title      => 'NBSL',
  brief      => 'Bitwise inverted select',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NBSL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE neg_z_p_z => {
  title      => 'NEG',
  brief      => 'Negate (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NEG status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE nmatch_p_p_zz => {
  title      => 'NMATCH',
  brief      => 'Detect no matching elements, setting the condition flags',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=NMATCH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE nor_p_p_pp => {
  title      => 'NOR, NORS',
  brief      => 'Bitwise NOR predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE not_z_p_z => {
  title      => 'NOT (vector)',
  brief      => 'Bitwise invert vector (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=NOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE orn_p_p_pp => {
  title      => 'ORN, ORNS (predicates)',
  brief      => 'Bitwise inclusive OR inverted predicate',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE orr_p_p_pp => {
  title      => 'ORR, ORRS (predicates)',
  brief      => 'Bitwise inclusive OR predicate',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE orr_z_p_zz => {
  title      => 'ORR (vectors, predicated)',
  brief      => 'Bitwise inclusive OR vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE orr_z_zi => {
  title      => 'ORR (immediate)',
  brief      => 'Bitwise inclusive OR with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE orr_z_zz => {
  title      => 'ORR (vectors, unpredicated)',
  brief      => 'Bitwise inclusive OR vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE orv_r_p_z => {
  title      => 'ORV',
  brief      => 'Bitwise inclusive OR reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ORV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE pfalse_p => {
  title      => 'PFALSE',
  brief      => 'Set all predicate elements to false',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFALSE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE pfirst_p_p_p => {
  title      => 'PFIRST',
  brief      => 'Set the first active predicate element to true',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PFIRST status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE pmul_z_zz => {
  title      => 'PMUL',
  brief      => 'Polynomial multiply vectors (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMUL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE pmullb_z_zz => {
  title      => 'PMULLB',
  brief      => 'Polynomial multiply long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE pmullt_z_zz => {
  title      => 'PMULLT',
  brief      => 'Polynomial multiply long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=PMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE pnext_p_p_p => {
  title      => 'PNEXT',
  brief      => 'Find next active predicate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PNEXT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE prfb_i_p_ai => {
  title      => 'PRFB (vector plus immediate)',
  brief      => 'Gather prefetch bytes (vector plus immediate)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfb_i_p_bi => {
  title      => 'PRFB (scalar plus immediate)',
  brief      => 'Contiguous prefetch bytes (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfb_i_p_br => {
  title      => 'PRFB (scalar plus scalar)',
  brief      => 'Contiguous prefetch bytes (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfb_i_p_bz => {
  title      => 'PRFB (scalar plus vector)',
  brief      => 'Gather prefetch bytes (scalar plus vector)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfd_i_p_ai => {
  title      => 'PRFD (vector plus immediate)',
  brief      => 'Gather prefetch doublewords (vector plus immediate)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfd_i_p_bi => {
  title      => 'PRFD (scalar plus immediate)',
  brief      => 'Contiguous prefetch doublewords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfd_i_p_br => {
  title      => 'PRFD (scalar plus scalar)',
  brief      => 'Contiguous prefetch doublewords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfd_i_p_bz => {
  title      => 'PRFD (scalar plus vector)',
  brief      => 'Gather prefetch doublewords (scalar plus vector)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfh_i_p_ai => {
  title      => 'PRFH (vector plus immediate)',
  brief      => 'Gather prefetch halfwords (vector plus immediate)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfh_i_p_bi => {
  title      => 'PRFH (scalar plus immediate)',
  brief      => 'Contiguous prefetch halfwords (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfh_i_p_br => {
  title      => 'PRFH (scalar plus scalar)',
  brief      => 'Contiguous prefetch halfwords (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfh_i_p_bz => {
  title      => 'PRFH (scalar plus vector)',
  brief      => 'Gather prefetch halfwords (scalar plus vector)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfw_i_p_ai => {
  title      => 'PRFW (vector plus immediate)',
  brief      => 'Gather prefetch words (vector plus immediate)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfw_i_p_bi => {
  title      => 'PRFW (scalar plus immediate)',
  brief      => 'Contiguous prefetch words (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfw_i_p_br => {
  title      => 'PRFW (scalar plus scalar)',
  brief      => 'Contiguous prefetch words (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE prfw_i_p_bz => {
  title      => 'PRFW (scalar plus vector)',
  brief      => 'Gather prefetch words (scalar plus vector)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PRFW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE ptest_p_p => {
  title      => 'PTEST',
  brief      => 'Set condition flags for predicate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=PTEST status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ptrue_p_s => {
  title      => 'PTRUE, PTRUES',
  brief      => 'Initialise predicate from named constraint',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE punpkhi_p_p => {
  title      => 'PUNPKHI, PUNPKLO',
  brief      => 'Unpack and widen half of predicate',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE raddhnb_z_zz => {
  title      => 'RADDHNB',
  brief      => 'Rounding add narrow high part (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE raddhnt_z_zz => {
  title      => 'RADDHNT',
  brief      => 'Rounding add narrow high part (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RADDHNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rax1_z_zz => {
  title      => 'RAX1',
  brief      => 'Bitwise rotate left by 1 and exclusive OR',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RAX1 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rbit_z_p_z => {
  title      => 'RBIT',
  brief      => 'Reverse bits (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RBIT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE rdffr_p_f => {
  title      => 'RDFFR (unpredicated)',
  brief      => 'Read the first-fault register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDFFR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rdffr_p_p_f => {
  title      => 'RDFFR, RDFFRS (predicated)',
  brief      => 'Return predicate of succesfully loaded elements',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rdvl_r_i => {
  title      => 'RDVL',
  brief      => 'Read multiple of vector register size to scalar register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=RDVL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rev_p_p => {
  title      => 'REV (predicate)',
  brief      => 'Reverse all elements in a predicate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rev_z_z => {
  title      => 'REV (vector)',
  brief      => 'Reverse all elements in a vector (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=REV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE revb_z_z => {
  title      => 'REVB, REVH, REVW',
  brief      => 'Reverse bytes / halfwords / words within elements (predicated)',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE rshrnb_z_zi => {
  title      => 'RSHRNB',
  brief      => 'Rounding shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rshrnt_z_zi => {
  title      => 'RSHRNT',
  brief      => 'Rounding shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rsubhnb_z_zz => {
  title      => 'RSUBHNB',
  brief      => 'Rounding subtract narrow high part (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE rsubhnt_z_zz => {
  title      => 'RSUBHNT',
  brief      => 'Rounding subtract narrow high part (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=RSUBHNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saba_z_zzz => {
  title      => 'SABA',
  brief      => 'Signed absolute difference and accumulate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sabalb_z_zzz => {
  title      => 'SABALB',
  brief      => 'Signed absolute difference and accumulate long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sabalt_z_zzz => {
  title      => 'SABALT',
  brief      => 'Signed absolute difference and accumulate long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sabd_z_p_zz => {
  title      => 'SABD',
  brief      => 'Signed absolute difference (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SABD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE sabdlb_z_zz => {
  title      => 'SABDLB',
  brief      => 'Signed absolute difference long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sabdlt_z_zz => {
  title      => 'SABDLT',
  brief      => 'Signed absolute difference long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SABDLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sadalp_z_p_z => {
  title      => 'SADALP',
  brief      => 'Signed add and accumulate long pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADALP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE saddlb_z_zz => {
  title      => 'SADDLB',
  brief      => 'Signed add long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saddlbt_z_zz => {
  title      => 'SADDLBT',
  brief      => 'Signed add long (bottom + top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLBT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saddlt_z_zz => {
  title      => 'SADDLT',
  brief      => 'Signed add long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saddv_r_p_z => {
  title      => 'SADDV',
  brief      => 'Signed add reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SADDV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saddwb_z_zz => {
  title      => 'SADDWB',
  brief      => 'Signed add wide (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE saddwt_z_zz => {
  title      => 'SADDWT',
  brief      => 'Signed add wide (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SADDWT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sbclb_z_zzz => {
  title      => 'SBCLB',
  brief      => 'Subtract with carry long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sbclt_z_zzz => {
  title      => 'SBCLT',
  brief      => 'Subtract with carry long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SBCLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE scvtf_z_p_z => {
  title      => 'SCVTF',
  brief      => 'Signed integer convert to floating-point (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SCVTF status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE sdiv_z_p_zz => {
  title      => 'SDIV',
  brief      => 'Signed divide (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sdivr_z_p_zz => {
  title      => 'SDIVR',
  brief      => 'Signed reversed divide (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDIVR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sdot_z_zzz => {
  title      => 'SDOT (vectors)',
  brief      => 'Signed integer dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sdot_z_zzzi => {
  title      => 'SDOT (indexed)',
  brief      => 'Signed integer indexed dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sel_p_p_pp => {
  title      => 'SEL (predicates)',
  brief      => 'Conditionally select elements from two predicates',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sel_z_p_zz => {
  title      => 'SEL (vectors)',
  brief      => 'Conditionally select elements from two vectors',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE setffr_f => {
  title      => 'SETFFR',
  brief      => 'Initialise the first-fault register to all true',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SETFFR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE shadd_z_p_zz => {
  title      => 'SHADD',
  brief      => 'Signed halving addition',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE shrnb_z_zi => {
  title      => 'SHRNB',
  brief      => 'Shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE shrnt_z_zi => {
  title      => 'SHRNT',
  brief      => 'Shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE shsub_z_p_zz => {
  title      => 'SHSUB',
  brief      => 'Signed halving subtract',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE shsubr_z_p_zz => {
  title      => 'SHSUBR',
  brief      => 'Signed halving subtract reversed vectors',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SHSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE sli_z_zzi => {
  title      => 'SLI',
  brief      => 'Shift left and insert (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SLI status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sm4e_z_zz => {
  title      => 'SM4E',
  brief      => 'SM4 encryption and decryption',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4E status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sm4ekey_z_zz => {
  title      => 'SM4EKEY',
  brief      => 'SM4 key updates',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SM4EKEY status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smax_z_p_zz => {
  title      => 'SMAX (vectors)',
  brief      => 'Signed maximum vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE smax_z_zi => {
  title      => 'SMAX (immediate)',
  brief      => 'Signed maximum with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smaxp_z_p_zz => {
  title      => 'SMAXP',
  brief      => 'Signed maximum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMAXP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smaxv_r_p_z => {
  title      => 'SMAXV',
  brief      => 'Signed maximum reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMAXV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smin_z_p_zz => {
  title      => 'SMIN (vectors)',
  brief      => 'Signed minimum vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE smin_z_zi => {
  title      => 'SMIN (immediate)',
  brief      => 'Signed minimum with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sminp_z_p_zz => {
  title      => 'SMINP',
  brief      => 'Signed minimum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMINP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sminv_r_p_z => {
  title      => 'SMINV',
  brief      => 'Signed minimum reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMINV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlalb_z_zzz => {
  title      => 'SMLALB (vectors)',
  brief      => 'Signed multiply-add long to accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlalb_z_zzzi => {
  title      => 'SMLALB (indexed)',
  brief      => 'Signed multiply-add long to accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlalt_z_zzz => {
  title      => 'SMLALT (vectors)',
  brief      => 'Signed multiply-add long to accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlalt_z_zzzi => {
  title      => 'SMLALT (indexed)',
  brief      => 'Signed multiply-add long to accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlslb_z_zzz => {
  title      => 'SMLSLB (vectors)',
  brief      => 'Signed multiply-subtract long from accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlslb_z_zzzi => {
  title      => 'SMLSLB (indexed)',
  brief      => 'Signed multiply-subtract long from accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlslt_z_zzz => {
  title      => 'SMLSLT (vectors)',
  brief      => 'Signed multiply-subtract long from accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smlslt_z_zzzi => {
  title      => 'SMLSLT (indexed)',
  brief      => 'Signed multiply-subtract long from accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smmla_z_zzz => {
  title      => 'SMMLA',
  brief      => 'Signed integer matrix multiply-accumulate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE smulh_z_p_zz => {
  title      => 'SMULH (predicated)',
  brief      => 'Signed multiply returning high half (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE smulh_z_zz => {
  title      => 'SMULH (unpredicated)',
  brief      => 'Signed multiply returning high half (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smullb_z_zz => {
  title      => 'SMULLB (vectors)',
  brief      => 'Signed multiply long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smullb_z_zzi => {
  title      => 'SMULLB (indexed)',
  brief      => 'Signed multiply long (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smullt_z_zz => {
  title      => 'SMULLT (vectors)',
  brief      => 'Signed multiply long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE smullt_z_zzi => {
  title      => 'SMULLT (indexed)',
  brief      => 'Signed multiply long (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE splice_z_p_zz => {
  title      => 'SPLICE',
  brief      => 'Splice two vectors under predicate control',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SPLICE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqabs_z_p_z => {
  title      => 'SQABS',
  brief      => 'Signed saturating absolute value',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQABS status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqadd_z_p_zz => {
  title      => 'SQADD (vectors, predicated)',
  brief      => 'Signed saturating addition (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqadd_z_zi => {
  title      => 'SQADD (immediate)',
  brief      => 'Signed saturating add immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqadd_z_zz => {
  title      => 'SQADD (vectors, unpredicated)',
  brief      => 'Signed saturating add vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqcadd_z_zz => {
  title      => 'SQCADD',
  brief      => 'Saturating complex integer add with rotate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQCADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecb_r_rs => {
  title      => 'SQDECB',
  brief      => 'Signed saturating decrement scalar by multiple of 8-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecd_r_rs => {
  title      => 'SQDECD (scalar)',
  brief      => 'Signed saturating decrement scalar by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecd_z_zs => {
  title      => 'SQDECD (vector)',
  brief      => 'Signed saturating decrement vector by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdech_r_rs => {
  title      => 'SQDECH (scalar)',
  brief      => 'Signed saturating decrement scalar by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdech_z_zs => {
  title      => 'SQDECH (vector)',
  brief      => 'Signed saturating decrement vector by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecp_r_p_r => {
  title      => 'SQDECP (scalar)',
  brief      => 'Signed saturating decrement scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecp_z_p_z => {
  title      => 'SQDECP (vector)',
  brief      => 'Signed saturating decrement vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecw_r_rs => {
  title      => 'SQDECW (scalar)',
  brief      => 'Signed saturating decrement scalar by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdecw_z_zs => {
  title      => 'SQDECW (vector)',
  brief      => 'Signed saturating decrement vector by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlalb_z_zzz => {
  title      => 'SQDMLALB (vectors)',
  brief      => 'Signed saturating doubling multiply-add long to accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlalb_z_zzzi => {
  title      => 'SQDMLALB (indexed)',
  brief      => 'Signed saturating doubling multiply-add long to accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlalbt_z_zzz => {
  title      => 'SQDMLALBT',
  brief      => 'Signed saturating doubling multiply-add long to accumulator (bottom x top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALBT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlalt_z_zzz => {
  title      => 'SQDMLALT (vectors)',
  brief      => 'Signed saturating doubling multiply-add long to accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlalt_z_zzzi => {
  title      => 'SQDMLALT (indexed)',
  brief      => 'Signed saturating doubling multiply-add long to accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlslb_z_zzz => {
  title      => 'SQDMLSLB (vectors)',
  brief      => 'Signed saturating doubling multiply-subtract long from accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlslb_z_zzzi => {
  title      => 'SQDMLSLB (indexed)',
  brief      => 'Signed saturating doubling multiply-subtract long from accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlslbt_z_zzz => {
  title      => 'SQDMLSLBT',
  brief      => 'Signed saturating doubling multiply-subtract long from accumulator (bottom x top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLBT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlslt_z_zzz => {
  title      => 'SQDMLSLT (vectors)',
  brief      => 'Signed saturating doubling multiply-subtract long from accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmlslt_z_zzzi => {
  title      => 'SQDMLSLT (indexed)',
  brief      => 'Signed saturating doubling multiply-subtract long from accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmulh_z_zz => {
  title      => 'SQDMULH (vectors)',
  brief      => 'Signed saturating doubling multiply high (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmulh_z_zzi => {
  title      => 'SQDMULH (indexed)',
  brief      => 'Signed saturating doubling multiply high (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmullb_z_zz => {
  title      => 'SQDMULLB (vectors)',
  brief      => 'Signed saturating doubling multiply long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmullb_z_zzi => {
  title      => 'SQDMULLB (indexed)',
  brief      => 'Signed saturating doubling multiply long (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmullt_z_zz => {
  title      => 'SQDMULLT (vectors)',
  brief      => 'Signed saturating doubling multiply long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqdmullt_z_zzi => {
  title      => 'SQDMULLT (indexed)',
  brief      => 'Signed saturating doubling multiply long (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQDMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincb_r_rs => {
  title      => 'SQINCB',
  brief      => 'Signed saturating increment scalar by multiple of 8-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincd_r_rs => {
  title      => 'SQINCD (scalar)',
  brief      => 'Signed saturating increment scalar by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincd_z_zs => {
  title      => 'SQINCD (vector)',
  brief      => 'Signed saturating increment vector by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqinch_r_rs => {
  title      => 'SQINCH (scalar)',
  brief      => 'Signed saturating increment scalar by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqinch_z_zs => {
  title      => 'SQINCH (vector)',
  brief      => 'Signed saturating increment vector by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincp_r_p_r => {
  title      => 'SQINCP (scalar)',
  brief      => 'Signed saturating increment scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincp_z_p_z => {
  title      => 'SQINCP (vector)',
  brief      => 'Signed saturating increment vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincw_r_rs => {
  title      => 'SQINCW (scalar)',
  brief      => 'Signed saturating increment scalar by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqincw_z_zs => {
  title      => 'SQINCW (vector)',
  brief      => 'Signed saturating increment vector by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqneg_z_p_z => {
  title      => 'SQNEG',
  brief      => 'Signed saturating negate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQNEG status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqrdcmlah_z_zzz => {
  title      => 'SQRDCMLAH (vectors)',
  brief      => 'Saturating rounding doubling complex integer multiply-add high with rotate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdcmlah_z_zzzi => {
  title      => 'SQRDCMLAH (indexed)',
  brief      => 'Saturating rounding doubling complex integer multiply-add high with rotate (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDCMLAH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmlah_z_zzz => {
  title      => 'SQRDMLAH (vectors)',
  brief      => 'Signed saturating rounding doubling multiply-add high to accumulator (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmlah_z_zzzi => {
  title      => 'SQRDMLAH (indexed)',
  brief      => 'Signed saturating rounding doubling multiply-add high to accumulator (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLAH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmlsh_z_zzz => {
  title      => 'SQRDMLSH (vectors)',
  brief      => 'Signed saturating rounding doubling multiply-subtract high from accumulator (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmlsh_z_zzzi => {
  title      => 'SQRDMLSH (indexed)',
  brief      => 'Signed saturating rounding doubling multiply-subtract high from accumulator (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMLSH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmulh_z_zz => {
  title      => 'SQRDMULH (vectors)',
  brief      => 'Signed saturating rounding doubling multiply high (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrdmulh_z_zzi => {
  title      => 'SQRDMULH (indexed)',
  brief      => 'Signed saturating rounding doubling multiply high (indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRDMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrshl_z_p_zz => {
  title      => 'SQRSHL',
  brief      => 'Signed saturating rounding shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqrshlr_z_p_zz => {
  title      => 'SQRSHLR',
  brief      => 'Signed saturating rounding shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqrshrnb_z_zi => {
  title      => 'SQRSHRNB',
  brief      => 'Signed saturating rounding shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrshrnt_z_zi => {
  title      => 'SQRSHRNT',
  brief      => 'Signed saturating rounding shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrshrunb_z_zi => {
  title      => 'SQRSHRUNB',
  brief      => 'Signed saturating rounding shift right unsigned narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqrshrunt_z_zi => {
  title      => 'SQRSHRUNT',
  brief      => 'Signed saturating rounding shift right unsigned narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQRSHRUNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqshl_z_p_zi => {
  title      => 'SQSHL (immediate)',
  brief      => 'Signed saturating shift left by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqshl_z_p_zz => {
  title      => 'SQSHL (vectors)',
  brief      => 'Signed saturating shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqshlr_z_p_zz => {
  title      => 'SQSHLR',
  brief      => 'Signed saturating shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqshlu_z_p_zi => {
  title      => 'SQSHLU',
  brief      => 'Signed saturating shift left unsigned by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHLU status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqshrnb_z_zi => {
  title      => 'SQSHRNB',
  brief      => 'Signed saturating shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqshrnt_z_zi => {
  title      => 'SQSHRNT',
  brief      => 'Signed saturating shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqshrunb_z_zi => {
  title      => 'SQSHRUNB',
  brief      => 'Signed saturating shift right unsigned narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqshrunt_z_zi => {
  title      => 'SQSHRUNT',
  brief      => 'Signed saturating shift right unsigned narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSHRUNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqsub_z_p_zz => {
  title      => 'SQSUB (vectors, predicated)',
  brief      => 'Signed saturating subtraction (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqsub_z_zi => {
  title      => 'SQSUB (immediate)',
  brief      => 'Signed saturating subtract immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqsub_z_zz => {
  title      => 'SQSUB (vectors, unpredicated)',
  brief      => 'Signed saturating subtract vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqsubr_z_p_zz => {
  title      => 'SQSUBR',
  brief      => 'Signed saturating subtraction reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sqxtnb_z_zz => {
  title      => 'SQXTNB',
  brief      => 'Signed saturating extract narrow (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqxtnt_z_zz => {
  title      => 'SQXTNT',
  brief      => 'Signed saturating extract narrow (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqxtunb_z_zz => {
  title      => 'SQXTUNB',
  brief      => 'Signed saturating unsigned extract narrow (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sqxtunt_z_zz => {
  title      => 'SQXTUNT',
  brief      => 'Signed saturating unsigned extract narrow (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SQXTUNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE srhadd_z_p_zz => {
  title      => 'SRHADD',
  brief      => 'Signed rounding halving addition',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRHADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sri_z_zzi => {
  title      => 'SRI',
  brief      => 'Shift right and insert (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRI status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE srshl_z_p_zz => {
  title      => 'SRSHL',
  brief      => 'Signed rounding shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE srshlr_z_p_zz => {
  title      => 'SRSHLR',
  brief      => 'Signed rounding shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE srshr_z_p_zi => {
  title      => 'SRSHR',
  brief      => 'Signed rounding shift right by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSHR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE srsra_z_zi => {
  title      => 'SRSRA',
  brief      => 'Signed rounding shift right and accumulate (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SRSRA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sshllb_z_zi => {
  title      => 'SSHLLB',
  brief      => 'Signed shift left long by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sshllt_z_zi => {
  title      => 'SSHLLT',
  brief      => 'Signed shift left long by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSHLLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssra_z_zi => {
  title      => 'SSRA',
  brief      => 'Signed shift right and accumulate (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSRA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssublb_z_zz => {
  title      => 'SSUBLB',
  brief      => 'Signed subtract long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssublbt_z_zz => {
  title      => 'SSUBLBT',
  brief      => 'Signed subtract long (bottom - top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLBT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssublt_z_zz => {
  title      => 'SSUBLT',
  brief      => 'Signed subtract long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssubltb_z_zz => {
  title      => 'SSUBLTB',
  brief      => 'Signed subtract long (top - bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBLTB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssubwb_z_zz => {
  title      => 'SSUBWB',
  brief      => 'Signed subtract wide (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ssubwt_z_zz => {
  title      => 'SSUBWT',
  brief      => 'Signed subtract wide (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SSUBWT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE st1b_z_p_ai => {
  title      => 'ST1B (vector plus immediate)',
  brief      => 'Scatter store bytes from a vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1b_z_p_bi => {
  title      => 'ST1B (scalar plus immediate)',
  brief      => 'Contiguous store bytes from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1b_z_p_br => {
  title      => 'ST1B (scalar plus scalar)',
  brief      => 'Contiguous store bytes from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1b_z_p_bz => {
  title      => 'ST1B (scalar plus vector)',
  brief      => 'Scatter store bytes from a vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1d_z_p_ai => {
  title      => 'ST1D (vector plus immediate)',
  brief      => 'Scatter store doublewords from a vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1d_z_p_bi => {
  title      => 'ST1D (scalar plus immediate)',
  brief      => 'Contiguous store doublewords from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1d_z_p_br => {
  title      => 'ST1D (scalar plus scalar)',
  brief      => 'Contiguous store doublewords from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1d_z_p_bz => {
  title      => 'ST1D (scalar plus vector)',
  brief      => 'Scatter store doublewords from a vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1h_z_p_ai => {
  title      => 'ST1H (vector plus immediate)',
  brief      => 'Scatter store halfwords from a vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1h_z_p_bi => {
  title      => 'ST1H (scalar plus immediate)',
  brief      => 'Contiguous store halfwords from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1h_z_p_br => {
  title      => 'ST1H (scalar plus scalar)',
  brief      => 'Contiguous store halfwords from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1h_z_p_bz => {
  title      => 'ST1H (scalar plus vector)',
  brief      => 'Scatter store halfwords from a vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1w_z_p_ai => {
  title      => 'ST1W (vector plus immediate)',
  brief      => 'Scatter store words from a vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1w_z_p_bi => {
  title      => 'ST1W (scalar plus immediate)',
  brief      => 'Contiguous store words from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1w_z_p_br => {
  title      => 'ST1W (scalar plus scalar)',
  brief      => 'Contiguous store words from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st1w_z_p_bz => {
  title      => 'ST1W (scalar plus vector)',
  brief      => 'Scatter store words from a vector (vector index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2b_z_p_bi => {
  title      => 'ST2B (scalar plus immediate)',
  brief      => 'Contiguous store two-byte structures from two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2b_z_p_br => {
  title      => 'ST2B (scalar plus scalar)',
  brief      => 'Contiguous store two-byte structures from two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2d_z_p_bi => {
  title      => 'ST2D (scalar plus immediate)',
  brief      => 'Contiguous store two-doubleword structures from two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2d_z_p_br => {
  title      => 'ST2D (scalar plus scalar)',
  brief      => 'Contiguous store two-doubleword structures from two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2h_z_p_bi => {
  title      => 'ST2H (scalar plus immediate)',
  brief      => 'Contiguous store two-halfword structures from two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2h_z_p_br => {
  title      => 'ST2H (scalar plus scalar)',
  brief      => 'Contiguous store two-halfword structures from two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2w_z_p_bi => {
  title      => 'ST2W (scalar plus immediate)',
  brief      => 'Contiguous store two-word structures from two vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st2w_z_p_br => {
  title      => 'ST2W (scalar plus scalar)',
  brief      => 'Contiguous store two-word structures from two vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST2W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3b_z_p_bi => {
  title      => 'ST3B (scalar plus immediate)',
  brief      => 'Contiguous store three-byte structures from three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3b_z_p_br => {
  title      => 'ST3B (scalar plus scalar)',
  brief      => 'Contiguous store three-byte structures from three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3d_z_p_bi => {
  title      => 'ST3D (scalar plus immediate)',
  brief      => 'Contiguous store three-doubleword structures from three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3d_z_p_br => {
  title      => 'ST3D (scalar plus scalar)',
  brief      => 'Contiguous store three-doubleword structures from three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3h_z_p_bi => {
  title      => 'ST3H (scalar plus immediate)',
  brief      => 'Contiguous store three-halfword structures from three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3h_z_p_br => {
  title      => 'ST3H (scalar plus scalar)',
  brief      => 'Contiguous store three-halfword structures from three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3w_z_p_bi => {
  title      => 'ST3W (scalar plus immediate)',
  brief      => 'Contiguous store three-word structures from three vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st3w_z_p_br => {
  title      => 'ST3W (scalar plus scalar)',
  brief      => 'Contiguous store three-word structures from three vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST3W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4b_z_p_bi => {
  title      => 'ST4B (scalar plus immediate)',
  brief      => 'Contiguous store four-byte structures from four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4b_z_p_br => {
  title      => 'ST4B (scalar plus scalar)',
  brief      => 'Contiguous store four-byte structures from four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4d_z_p_bi => {
  title      => 'ST4D (scalar plus immediate)',
  brief      => 'Contiguous store four-doubleword structures from four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4d_z_p_br => {
  title      => 'ST4D (scalar plus scalar)',
  brief      => 'Contiguous store four-doubleword structures from four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4h_z_p_bi => {
  title      => 'ST4H (scalar plus immediate)',
  brief      => 'Contiguous store four-halfword structures from four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4h_z_p_br => {
  title      => 'ST4H (scalar plus scalar)',
  brief      => 'Contiguous store four-halfword structures from four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4w_z_p_bi => {
  title      => 'ST4W (scalar plus immediate)',
  brief      => 'Contiguous store four-word structures from four vectors (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE st4w_z_p_br => {
  title      => 'ST4W (scalar plus scalar)',
  brief      => 'Contiguous store four-word structures from four vectors (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=ST4W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1b_z_p_ar => {
  title      => 'STNT1B (vector plus scalar)',
  brief      => 'Scatter store non-temporal bytes',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1b_z_p_bi => {
  title      => 'STNT1B (scalar plus immediate)',
  brief      => 'Contiguous store non-temporal bytes from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1b_z_p_br => {
  title      => 'STNT1B (scalar plus scalar)',
  brief      => 'Contiguous store non-temporal bytes from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1B status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1d_z_p_ar => {
  title      => 'STNT1D (vector plus scalar)',
  brief      => 'Scatter store non-temporal doublewords',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1D status=Green sve-offset-type=off_d_64_unscaled',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1d_z_p_bi => {
  title      => 'STNT1D (scalar plus immediate)',
  brief      => 'Contiguous store non-temporal doublewords from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1d_z_p_br => {
  title      => 'STNT1D (scalar plus scalar)',
  brief      => 'Contiguous store non-temporal doublewords from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1D status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1h_z_p_ar => {
  title      => 'STNT1H (vector plus scalar)',
  brief      => 'Scatter store non-temporal halfwords',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1h_z_p_bi => {
  title      => 'STNT1H (scalar plus immediate)',
  brief      => 'Contiguous store non-temporal halfwords from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1h_z_p_br => {
  title      => 'STNT1H (scalar plus scalar)',
  brief      => 'Contiguous store non-temporal halfwords from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1H status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1w_z_p_ar => {
  title      => 'STNT1W (vector plus scalar)',
  brief      => 'Scatter store non-temporal words',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1w_z_p_bi => {
  title      => 'STNT1W (scalar plus immediate)',
  brief      => 'Contiguous store non-temporal words from vector (immediate index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE stnt1w_z_p_br => {
  title      => 'STNT1W (scalar plus scalar)',
  brief      => 'Contiguous store non-temporal words from vector (scalar index)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STNT1W status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1',
};

PAGE str_p_bi => {
  title      => 'STR (predicate)',
  brief      => 'Store predicate register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 alphaindex=SVE',
};

PAGE str_z_bi => {
  title      => 'STR (vector)',
  brief      => 'Store vector register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=STR status=Green',
  metadata   => 'alias=0 alphaindex=SVE',
};

PAGE sub_z_p_zz => {
  title      => 'SUB (vectors, predicated)',
  brief      => 'Subtract vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE sub_z_zi => {
  title      => 'SUB (immediate)',
  brief      => 'Subtract immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sub_z_zz => {
  title      => 'SUB (vectors, unpredicated)',
  brief      => 'Subtract vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE subhnb_z_zz => {
  title      => 'SUBHNB',
  brief      => 'Subtract narrow high part (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE subhnt_z_zz => {
  title      => 'SUBHNT',
  brief      => 'Subtract narrow high part (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUBHNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE subr_z_p_zz => {
  title      => 'SUBR (vectors)',
  brief      => 'Reversed subtract vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE subr_z_zi => {
  title      => 'SUBR (immediate)',
  brief      => 'Reversed subtract from immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE sudot_z_zzzi => {
  title      => 'SUDOT',
  brief      => 'Signed by unsigned integer indexed dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=SUDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE sunpkhi_z_z => {
  title      => 'SUNPKHI, SUNPKLO',
  brief      => 'Signed unpack and extend half of vector',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE suqadd_z_p_zz => {
  title      => 'SUQADD',
  brief      => 'Signed saturating addition of unsigned value',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=SUQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE sxtb_z_p_z => {
  title      => 'SXTB, SXTH, SXTW',
  brief      => 'Signed byte / halfword / word extend (predicated)',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE tbl_z_zz => {
  title      => 'TBL',
  brief      => 'Programmable table lookup in one or two vector table (zeroing)',
  docvars    => 'isa=A64 mnemonic=TBL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE tbx_z_zz => {
  title      => 'TBX',
  brief      => 'Programmable table lookup in single vector table (merging)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=TBX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE trn1_p_pp => {
  title      => 'TRN1, TRN2 (predicates)',
  brief      => 'Interleave even or odd elements from two predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE trn1_z_zz => {
  title      => 'TRN1, TRN2 (vectors)',
  brief      => 'Interleave even or odd elements from two vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uaba_z_zzz => {
  title      => 'UABA',
  brief      => 'Unsigned absolute difference and accumulate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uabalb_z_zzz => {
  title      => 'UABALB',
  brief      => 'Unsigned absolute difference and accumulate long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uabalt_z_zzz => {
  title      => 'UABALT',
  brief      => 'Unsigned absolute difference and accumulate long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uabd_z_p_zz => {
  title      => 'UABD',
  brief      => 'Unsigned absolute difference (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UABD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE uabdlb_z_zz => {
  title      => 'UABDLB',
  brief      => 'Unsigned absolute difference long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uabdlt_z_zz => {
  title      => 'UABDLT',
  brief      => 'Unsigned absolute difference long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UABDLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uadalp_z_p_z => {
  title      => 'UADALP',
  brief      => 'Unsigned add and accumulate long pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADALP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE uaddlb_z_zz => {
  title      => 'UADDLB',
  brief      => 'Unsigned add long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uaddlt_z_zz => {
  title      => 'UADDLT',
  brief      => 'Unsigned add long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uaddv_r_p_z => {
  title      => 'UADDV',
  brief      => 'Unsigned add reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UADDV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uaddwb_z_zz => {
  title      => 'UADDWB',
  brief      => 'Unsigned add wide (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uaddwt_z_zz => {
  title      => 'UADDWT',
  brief      => 'Unsigned add wide (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UADDWT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ucvtf_z_p_z => {
  title      => 'UCVTF',
  brief      => 'Unsigned integer convert to floating-point (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UCVTF status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE udiv_z_p_zz => {
  title      => 'UDIV',
  brief      => 'Unsigned divide (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE udivr_z_p_zz => {
  title      => 'UDIVR',
  brief      => 'Unsigned reversed divide (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDIVR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE udot_z_zzz => {
  title      => 'UDOT (vectors)',
  brief      => 'Unsigned integer dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE udot_z_zzzi => {
  title      => 'UDOT (indexed)',
  brief      => 'Unsigned integer indexed dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uhadd_z_p_zz => {
  title      => 'UHADD',
  brief      => 'Unsigned halving addition',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE uhsub_z_p_zz => {
  title      => 'UHSUB',
  brief      => 'Unsigned halving subtract',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE uhsubr_z_p_zz => {
  title      => 'UHSUBR',
  brief      => 'Unsigned halving subtract reversed vectors',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UHSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE umax_z_p_zz => {
  title      => 'UMAX (vectors)',
  brief      => 'Unsigned maximum vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE umax_z_zi => {
  title      => 'UMAX (immediate)',
  brief      => 'Unsigned maximum with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAX status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umaxp_z_p_zz => {
  title      => 'UMAXP',
  brief      => 'Unsigned maximum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMAXP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umaxv_r_p_z => {
  title      => 'UMAXV',
  brief      => 'Unsigned maximum reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMAXV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umin_z_p_zz => {
  title      => 'UMIN (vectors)',
  brief      => 'Unsigned minimum vectors (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE umin_z_zi => {
  title      => 'UMIN (immediate)',
  brief      => 'Unsigned minimum with immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMIN status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uminp_z_p_zz => {
  title      => 'UMINP',
  brief      => 'Unsigned minimum pairwise',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMINP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uminv_r_p_z => {
  title      => 'UMINV',
  brief      => 'Unsigned minimum reduction to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMINV status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlalb_z_zzz => {
  title      => 'UMLALB (vectors)',
  brief      => 'Unsigned multiply-add long to accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlalb_z_zzzi => {
  title      => 'UMLALB (indexed)',
  brief      => 'Unsigned multiply-add long to accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlalt_z_zzz => {
  title      => 'UMLALT (vectors)',
  brief      => 'Unsigned multiply-add long to accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlalt_z_zzzi => {
  title      => 'UMLALT (indexed)',
  brief      => 'Unsigned multiply-add long to accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLALT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlslb_z_zzz => {
  title      => 'UMLSLB (vectors)',
  brief      => 'Unsigned multiply-subtract long from accumulator (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlslb_z_zzzi => {
  title      => 'UMLSLB (indexed)',
  brief      => 'Unsigned multiply-subtract long from accumulator (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlslt_z_zzz => {
  title      => 'UMLSLT (vectors)',
  brief      => 'Unsigned multiply-subtract long from accumulator (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umlslt_z_zzzi => {
  title      => 'UMLSLT (indexed)',
  brief      => 'Unsigned multiply-subtract long from accumulator (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMLSLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ummla_z_zzz => {
  title      => 'UMMLA',
  brief      => 'Unsigned integer matrix multiply-accumulate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE umulh_z_p_zz => {
  title      => 'UMULH (predicated)',
  brief      => 'Unsigned multiply returning high half (predicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE umulh_z_zz => {
  title      => 'UMULH (unpredicated)',
  brief      => 'Unsigned multiply returning high half (unpredicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULH status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umullb_z_zz => {
  title      => 'UMULLB (vectors)',
  brief      => 'Unsigned multiply long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umullb_z_zzi => {
  title      => 'UMULLB (indexed)',
  brief      => 'Unsigned multiply long (bottom, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umullt_z_zz => {
  title      => 'UMULLT (vectors)',
  brief      => 'Unsigned multiply long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE umullt_z_zzi => {
  title      => 'UMULLT (indexed)',
  brief      => 'Unsigned multiply long (top, indexed)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UMULLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uqadd_z_p_zz => {
  title      => 'UQADD (vectors, predicated)',
  brief      => 'Unsigned saturating addition (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqadd_z_zi => {
  title      => 'UQADD (immediate)',
  brief      => 'Unsigned saturating add immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqadd_z_zz => {
  title      => 'UQADD (vectors, unpredicated)',
  brief      => 'Unsigned saturating add vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecb_r_rs => {
  title      => 'UQDECB',
  brief      => 'Unsigned saturating decrement scalar by multiple of 8-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecd_r_rs => {
  title      => 'UQDECD (scalar)',
  brief      => 'Unsigned saturating decrement scalar by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecd_z_zs => {
  title      => 'UQDECD (vector)',
  brief      => 'Unsigned saturating decrement vector by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdech_r_rs => {
  title      => 'UQDECH (scalar)',
  brief      => 'Unsigned saturating decrement scalar by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdech_z_zs => {
  title      => 'UQDECH (vector)',
  brief      => 'Unsigned saturating decrement vector by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecp_r_p_r => {
  title      => 'UQDECP (scalar)',
  brief      => 'Unsigned saturating decrement scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecp_z_p_z => {
  title      => 'UQDECP (vector)',
  brief      => 'Unsigned saturating decrement vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecw_r_rs => {
  title      => 'UQDECW (scalar)',
  brief      => 'Unsigned saturating decrement scalar by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqdecw_z_zs => {
  title      => 'UQDECW (vector)',
  brief      => 'Unsigned saturating decrement vector by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQDECW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincb_r_rs => {
  title      => 'UQINCB',
  brief      => 'Unsigned saturating increment scalar by multiple of 8-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCB status=Green sve-esize=esize-byte',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincd_r_rs => {
  title      => 'UQINCD (scalar)',
  brief      => 'Unsigned saturating increment scalar by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincd_z_zs => {
  title      => 'UQINCD (vector)',
  brief      => 'Unsigned saturating increment vector by multiple of 64-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCD status=Green sve-esize=esize-doubleword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqinch_r_rs => {
  title      => 'UQINCH (scalar)',
  brief      => 'Unsigned saturating increment scalar by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqinch_z_zs => {
  title      => 'UQINCH (vector)',
  brief      => 'Unsigned saturating increment vector by multiple of 16-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCH status=Green sve-esize=esize-halfword',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincp_r_p_r => {
  title      => 'UQINCP (scalar)',
  brief      => 'Unsigned saturating increment scalar by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincp_z_p_z => {
  title      => 'UQINCP (vector)',
  brief      => 'Unsigned saturating increment vector by count of true predicate elements',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCP status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincw_r_rs => {
  title      => 'UQINCW (scalar)',
  brief      => 'Unsigned saturating increment scalar by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqincw_z_zs => {
  title      => 'UQINCW (vector)',
  brief      => 'Unsigned saturating increment vector by multiple of 32-bit predicate constraint element count',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQINCW status=Green sve-esize=esize-word',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqrshl_z_p_zz => {
  title      => 'UQRSHL',
  brief      => 'Unsigned saturating rounding shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqrshlr_z_p_zz => {
  title      => 'UQRSHLR',
  brief      => 'Unsigned saturating rounding shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqrshrnb_z_zi => {
  title      => 'UQRSHRNB',
  brief      => 'Unsigned saturating rounding shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqrshrnt_z_zi => {
  title      => 'UQRSHRNT',
  brief      => 'Unsigned saturating rounding shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQRSHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqshl_z_p_zi => {
  title      => 'UQSHL (immediate)',
  brief      => 'Unsigned saturating shift left by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqshl_z_p_zz => {
  title      => 'UQSHL (vectors)',
  brief      => 'Unsigned saturating shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqshlr_z_p_zz => {
  title      => 'UQSHLR',
  brief      => 'Unsigned saturating shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqshrnb_z_zi => {
  title      => 'UQSHRNB',
  brief      => 'Unsigned saturating shift right narrow by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqshrnt_z_zi => {
  title      => 'UQSHRNT',
  brief      => 'Unsigned saturating shift right narrow by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSHRNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqsub_z_p_zz => {
  title      => 'UQSUB (vectors, predicated)',
  brief      => 'Unsigned saturating subtraction (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqsub_z_zi => {
  title      => 'UQSUB (immediate)',
  brief      => 'Unsigned saturating subtract immediate (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqsub_z_zz => {
  title      => 'UQSUB (vectors, unpredicated)',
  brief      => 'Unsigned saturating subtract vectors (unpredicated)',
  docvars    => 'instr-class=sve isa=A64 mnemonic=UQSUB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqsubr_z_p_zz => {
  title      => 'UQSUBR',
  brief      => 'Unsigned saturating subtraction reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQSUBR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE uqxtnb_z_zz => {
  title      => 'UQXTNB',
  brief      => 'Unsigned saturating extract narrow (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE uqxtnt_z_zz => {
  title      => 'UQXTNT',
  brief      => 'Unsigned saturating extract narrow (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=UQXTNT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE urecpe_z_p_z => {
  title      => 'URECPE',
  brief      => 'Unsigned reciprocal estimate (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URECPE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE urhadd_z_p_zz => {
  title      => 'URHADD',
  brief      => 'Unsigned rounding halving addition',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URHADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE urshl_z_p_zz => {
  title      => 'URSHL',
  brief      => 'Unsigned rounding shift left by vector (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHL status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE urshlr_z_p_zz => {
  title      => 'URSHLR',
  brief      => 'Unsigned rounding shift left reversed vectors (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHLR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE urshr_z_p_zi => {
  title      => 'URSHR',
  brief      => 'Unsigned rounding shift right by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSHR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE ursqrte_z_p_z => {
  title      => 'URSQRTE',
  brief      => 'Unsigned reciprocal square root estimate (predicated)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSQRTE status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE ursra_z_zi => {
  title      => 'URSRA',
  brief      => 'Unsigned rounding shift right and accumulate (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=URSRA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE usdot_z_zzz => {
  title      => 'USDOT (vectors)',
  brief      => 'Unsigned by signed integer dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE usdot_z_zzzi => {
  title      => 'USDOT (indexed)',
  brief      => 'Unsigned by signed integer indexed dot product',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USDOT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE ushllb_z_zi => {
  title      => 'USHLLB',
  brief      => 'Unsigned shift left long by immediate (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ushllt_z_zi => {
  title      => 'USHLLT',
  brief      => 'Unsigned shift left long by immediate (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USHLLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE usmmla_z_zzz => {
  title      => 'USMMLA',
  brief      => 'Unsigned by signed integer matrix multiply-accumulate',
  docvars    => 'instr-class=sve isa=A64 mnemonic=USMMLA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=0',
};

PAGE usqadd_z_p_zz => {
  title      => 'USQADD',
  brief      => 'Unsigned saturating addition of signed value',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USQADD status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=0',
};

PAGE usra_z_zi => {
  title      => 'USRA',
  brief      => 'Unsigned shift right and accumulate (immediate)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USRA status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE usublb_z_zz => {
  title      => 'USUBLB',
  brief      => 'Unsigned subtract long (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE usublt_z_zz => {
  title      => 'USUBLT',
  brief      => 'Unsigned subtract long (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBLT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE usubwb_z_zz => {
  title      => 'USUBWB',
  brief      => 'Unsigned subtract wide (bottom)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWB status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE usubwt_z_zz => {
  title      => 'USUBWT',
  brief      => 'Unsigned subtract wide (top)',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=USUBWT status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uunpkhi_z_z => {
  title      => 'UUNPKHI, UUNPKLO',
  brief      => 'Unsigned unpack and extend half of vector',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uxtb_z_p_z => {
  title      => 'UXTB, UXTH, UXTW',
  brief      => 'Unsigned byte / halfword / word extend (predicated)',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE uzp1_p_pp => {
  title      => 'UZP1, UZP2 (predicates)',
  brief      => 'Concatenate even or odd elements from two predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE uzp1_z_zz => {
  title      => 'UZP1, UZP2 (vectors)',
  brief      => 'Concatenate even or odd elements from two vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilege_p_p_rr => {
  title      => 'WHILEGE',
  brief      => 'While decrementing signed scalar greater than or equal to scalar',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGE status=Green sve-compare-type=ge',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilegt_p_p_rr => {
  title      => 'WHILEGT',
  brief      => 'While decrementing signed scalar greater than scalar',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEGT status=Green sve-compare-type=gt',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilehi_p_p_rr => {
  title      => 'WHILEHI',
  brief      => 'While decrementing unsigned scalar higher than scalar',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHI status=Green sve-compare-type=hi',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilehs_p_p_rr => {
  title      => 'WHILEHS',
  brief      => 'While decrementing unsigned scalar higher or same as scalar',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEHS status=Green sve-compare-type=hs',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilele_p_p_rr => {
  title      => 'WHILELE',
  brief      => 'While incrementing signed scalar less than or equal to scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELE status=Green sve-compare-type=le',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilelo_p_p_rr => {
  title      => 'WHILELO',
  brief      => 'While incrementing unsigned scalar lower than scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELO status=Green sve-compare-type=lo',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilels_p_p_rr => {
  title      => 'WHILELS',
  brief      => 'While incrementing unsigned scalar lower or same as scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELS status=Green sve-compare-type=ls',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilelt_p_p_rr => {
  title      => 'WHILELT',
  brief      => 'While incrementing signed scalar less than scalar',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WHILELT status=Green sve-compare-type=lt',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE whilerw_p_rr => {
  title      => 'WHILERW',
  brief      => 'While free of read-after-write conflicts',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILERW status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE whilewr_p_rr => {
  title      => 'WHILEWR',
  brief      => 'While free of write-after-read/write conflicts',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=WHILEWR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=0',
};

PAGE wrffr_f_p => {
  title      => 'WRFFR',
  brief      => 'Write the first-fault register',
  docvars    => 'instr-class=sve isa=A64 mnemonic=WRFFR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE xar_z_zzi => {
  title      => 'XAR',
  brief      => 'Bitwise exclusive OR and rotate right by immediate',
  docvars    => 'instr-class=sve2 isa=A64 mnemonic=XAR status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE zip1_p_pp => {
  title      => 'ZIP1, ZIP2 (predicates)',
  brief      => 'Interleave elements from two half predicates',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE zip1_z_zz => {
  title      => 'ZIP1, ZIP2 (vectors)',
  brief      => 'Interleave elements from two half vectors',
  docvars    => 'instr-class=sve isa=A64 status=Green',
  metadata   => 'alias=0 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE BIC_and_z_zi => {
  title      => 'BIC (immediate)',
  brief      => 'Bitwise clear bits using immediate (unpredicated)',
  docvars    => 'alias_mnemonic=BIC instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE CMPLE_cmpeq_p_p_zz => {
  title      => 'CMPLE (vectors)',
  brief      => 'Compare signed less than or equal to vector, setting the condition flags',
  docvars    => 'alias_mnemonic=CMPLE instr-class=sve isa=A64 mnemonic=CMPGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE CMPLO_cmpeq_p_p_zz => {
  title      => 'CMPLO (vectors)',
  brief      => 'Compare unsigned lower than vector, setting the condition flags',
  docvars    => 'alias_mnemonic=CMPLO instr-class=sve isa=A64 mnemonic=CMPHI status=Green sve-compare-type=hi',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE CMPLS_cmpeq_p_p_zz => {
  title      => 'CMPLS (vectors)',
  brief      => 'Compare unsigned lower or same as vector, setting the condition flags',
  docvars    => 'alias_mnemonic=CMPLS instr-class=sve isa=A64 mnemonic=CMPHS status=Green sve-compare-type=hs',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE CMPLT_cmpeq_p_p_zz => {
  title      => 'CMPLT (vectors)',
  brief      => 'Compare signed less than vector, setting the condition flags',
  docvars    => 'alias_mnemonic=CMPLT instr-class=sve isa=A64 mnemonic=CMPGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE EON_eor_z_zi => {
  title      => 'EON',
  brief      => 'Bitwise exclusive OR with inverted immediate (unpredicated)',
  docvars    => 'alias_mnemonic=EON instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};

PAGE FACLE_facge_p_p_zz => {
  title      => 'FACLE',
  brief      => 'Floating-point absolute compare less than or equal',
  docvars    => 'alias_mnemonic=FACLE instr-class=sve isa=A64 mnemonic=FACGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE FACLT_facge_p_p_zz => {
  title      => 'FACLT',
  brief      => 'Floating-point absolute compare less than',
  docvars    => 'alias_mnemonic=FACLT instr-class=sve isa=A64 mnemonic=FACGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE FCMLE_fcmeq_p_p_zz => {
  title      => 'FCMLE (vectors)',
  brief      => 'Floating-point compare less than or equal to vector',
  docvars    => 'alias_mnemonic=FCMLE instr-class=sve isa=A64 mnemonic=FCMGE status=Green sve-compare-type=ge',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE FCMLT_fcmeq_p_p_zz => {
  title      => 'FCMLT (vectors)',
  brief      => 'Floating-point compare less than vector',
  docvars    => 'alias_mnemonic=FCMLT instr-class=sve isa=A64 mnemonic=FCMGT status=Green sve-compare-type=gt',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE FMOV_cpy_z_p_i => {
  title      => 'FMOV (zero, predicated)',
  brief      => 'Move floating-point +0.0 to vector elements (predicated)',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE FMOV_dup_z_i => {
  title      => 'FMOV (zero, unpredicated)',
  brief      => 'Move floating-point +0.0 to vector elements (unpredicated)',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE FMOV_fcpy_z_p_i => {
  title      => 'FMOV (immediate, predicated)',
  brief      => 'Move 8-bit floating-point immediate to vector elements (predicated)',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=FCPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1',
};

PAGE FMOV_fdup_z_i => {
  title      => 'FMOV (immediate, unpredicated)',
  brief      => 'Move 8-bit floating-point immediate to vector elements (unpredicated)',
  docvars    => 'alias_mnemonic=FMOV instr-class=sve isa=A64 mnemonic=FDUP status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0',
};

PAGE MOV_and_p_p_pp => {
  title      => 'MOV (predicate, predicated, zeroing)',
  brief      => 'Move predicates (zeroing)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s instr-class=sve isa=A64 mnemonic=AND status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_cpy_z_o_i => {
  title      => 'MOV (immediate, predicated, zeroing)',
  brief      => 'Move signed integer immediate to vector elements (zeroing)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_cpy_z_p_i => {
  title      => 'MOV (immediate, predicated, merging)',
  brief      => 'Move signed integer immediate to vector elements (merging)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE MOV_cpy_z_p_r => {
  title      => 'MOV (scalar, predicated)',
  brief      => 'Move general-purpose register to vector elements (predicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE MOV_cpy_z_p_v => {
  title      => 'MOV (SIMD&FP scalar, predicated)',
  brief      => 'Move SIMD&FP scalar register to vector elements (predicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=CPY status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=1 takes_pred_movprfx=1 uses_dit=1',
};

PAGE MOV_dup_z_i => {
  title      => 'MOV (immediate, unpredicated)',
  brief      => 'Move signed immediate to vector elements (unpredicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_dup_z_r => {
  title      => 'MOV (scalar, unpredicated)',
  brief      => 'Move general-purpose register to vector elements (unpredicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_dup_z_zi => {
  title      => 'MOV (SIMD&FP scalar, unpredicated)',
  brief      => 'Move indexed element or SIMD&FP scalar to vector (unpredicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUP status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_dupm_z_i => {
  title      => 'MOV (bitmask immediate)',
  brief      => 'Move logical bitmask immediate to vector (unpredicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=DUPM status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_orr_p_p_pp => {
  title      => 'MOV (predicate, unpredicated)',
  brief      => 'Move predicate (unpredicated)',
  docvars    => 'alias_mnemonic=MOV cond-setting=no-s instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_orr_z_zz => {
  title      => 'MOV (vector, unpredicated)',
  brief      => 'Move vector register (unpredicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_sel_p_p_pp => {
  title      => 'MOV (predicate, predicated, merging)',
  brief      => 'Move predicates (merging)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOV_sel_z_p_zz => {
  title      => 'MOV (vector, predicated)',
  brief      => 'Move vector elements (predicated)',
  docvars    => 'alias_mnemonic=MOV instr-class=sve isa=A64 mnemonic=SEL status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOVS_and_p_p_pp => {
  title      => 'MOVS (predicated)',
  brief      => 'Move predicates (zeroing), setting the condition flags',
  docvars    => 'alias_mnemonic=MOVS cond-setting=s instr-class=sve isa=A64 mnemonic=ANDS status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE MOVS_orr_p_p_pp => {
  title      => 'MOVS (unpredicated)',
  brief      => 'Move predicate (unpredicated), setting the condition flags',
  docvars    => 'alias_mnemonic=MOVS cond-setting=s instr-class=sve isa=A64 mnemonic=ORRS status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE NOT_eor_p_p_pp => {
  title      => 'NOT (predicate)',
  brief      => 'Bitwise invert predicate',
  docvars    => 'alias_mnemonic=NOT cond-setting=no-s instr-class=sve isa=A64 mnemonic=EOR status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE NOTS_eor_p_p_pp => {
  title      => 'NOTS',
  brief      => 'Bitwise invert predicate, setting the condition flags',
  docvars    => 'alias_mnemonic=NOTS cond-setting=s instr-class=sve isa=A64 mnemonic=EORS status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=1 takes_movprfx=0 takes_pred_movprfx=0 uses_dit=1',
};

PAGE ORN_orr_z_zi => {
  title      => 'ORN (immediate)',
  brief      => 'Bitwise inclusive OR with inverted immediate (unpredicated)',
  docvars    => 'alias_mnemonic=ORN instr-class=sve isa=A64 mnemonic=ORR status=Green',
  metadata   => 'alias=1 alphaindex=SVE predicated=0 takes_movprfx=1 takes_pred_movprfx=0 uses_dit=1',
};


1;

