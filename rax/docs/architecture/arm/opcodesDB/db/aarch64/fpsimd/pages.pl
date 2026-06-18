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

PAGE ABS_advsimd => {
  title      => 'ABS',
  brief      => 'Absolute value (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ADD_advsimd => {
  title      => 'ADD (vector)',
  brief      => 'Add (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ADDHN_advsimd => {
  title      => 'ADDHN, ADDHN2',
  brief      => 'Add returning High Narrow',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ADDP_advsimd_pair => {
  title      => 'ADDP (scalar)',
  brief      => 'Add Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ADDP_advsimd_vec => {
  title      => 'ADDP (vector)',
  brief      => 'Add Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ADDV_advsimd => {
  title      => 'ADDV',
  brief      => 'Add across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESD_advsimd => {
  title      => 'AESD',
  brief      => 'AES single round decryption',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESE_advsimd => {
  title      => 'AESE',
  brief      => 'AES single round encryption',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESIMC_advsimd => {
  title      => 'AESIMC',
  brief      => 'AES inverse mix columns',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESIMC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AESMC_advsimd => {
  title      => 'AESMC',
  brief      => 'AES mix columns',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESMC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE AND_advsimd => {
  title      => 'AND (vector)',
  brief      => 'Bitwise AND (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=AND',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BCAX_advsimd => {
  title      => 'BCAX',
  brief      => 'Bit Clear and XOR',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=BCAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFCVT_float => {
  title      => 'BFCVT',
  brief      => 'Floating-point convert from single-precision to BFloat16 format (scalar)',
  docvars    => 'convert-type=single-to-bf16 instr-class=float isa=A64 mnemonic=BFCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFCVTN_advsimd => {
  title      => 'BFCVTN, BFCVTN2',
  brief      => 'Floating-point convert from single-precision to BFloat16 format (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-bf16 advsimd-type=simd datatype=single-and-bf16 instr-class=advsimd isa=A64 mnemonic=BFCVTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFDOT_advsimd_elt => {
  title      => 'BFDOT (by element)',
  brief      => 'BFloat16 floating-point dot product (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFDOT_advsimd_vec => {
  title      => 'BFDOT (vector)',
  brief      => 'BFloat16 floating-point dot product (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFMLAL_advsimd_elt => {
  title      => 'BFMLALB, BFMLALT (by element)',
  brief      => 'BFloat16 floating-point widening multiply-add long (by element)',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=BFMLAL reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFMLAL_advsimd_vec => {
  title      => 'BFMLALB, BFMLALT (vector)',
  brief      => 'BFloat16 floating-point widening multiply-add long (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BFMMLA_advsimd => {
  title      => 'BFMMLA',
  brief      => 'BFloat16 floating-point matrix multiply-accumulate into 2x2 matrix',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BIC_advsimd_imm => {
  title      => 'BIC (vector, immediate)',
  brief      => 'Bitwise bit Clear (vector, immediate)',
  docvars    => 'asimdimm-immtype=shifted-immediate instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BIC_advsimd_reg => {
  title      => 'BIC (vector, register)',
  brief      => 'Bitwise bit Clear (vector, register)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BIF_advsimd => {
  title      => 'BIF',
  brief      => 'Bitwise Insert if False',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BIT_advsimd => {
  title      => 'BIT',
  brief      => 'Bitwise Insert if True',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE BSL_advsimd => {
  title      => 'BSL',
  brief      => 'Bitwise Select',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CLS_advsimd => {
  title      => 'CLS (vector)',
  brief      => 'Count Leading Sign bits (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CLZ_advsimd => {
  title      => 'CLZ (vector)',
  brief      => 'Count Leading Zero bits (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMEQ_advsimd_reg => {
  title      => 'CMEQ (register)',
  brief      => 'Compare bitwise Equal (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMEQ_advsimd_zero => {
  title      => 'CMEQ (zero)',
  brief      => 'Compare bitwise Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMGE_advsimd_reg => {
  title      => 'CMGE (register)',
  brief      => 'Compare signed Greater than or Equal (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMGE_advsimd_zero => {
  title      => 'CMGE (zero)',
  brief      => 'Compare signed Greater than or Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMGT_advsimd_reg => {
  title      => 'CMGT (register)',
  brief      => 'Compare signed Greater than (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMGT_advsimd_zero => {
  title      => 'CMGT (zero)',
  brief      => 'Compare signed Greater than zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMHI_advsimd => {
  title      => 'CMHI (register)',
  brief      => 'Compare unsigned Higher (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMHI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMHS_advsimd => {
  title      => 'CMHS (register)',
  brief      => 'Compare unsigned Higher or Same (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMHS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMLE_advsimd => {
  title      => 'CMLE (zero)',
  brief      => 'Compare signed Less than or Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMLE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMLT_advsimd => {
  title      => 'CMLT (zero)',
  brief      => 'Compare signed Less than zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMLT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CMTST_advsimd => {
  title      => 'CMTST',
  brief      => 'Compare bitwise Test bits nonzero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=CMTST',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE CNT_advsimd => {
  title      => 'CNT',
  brief      => 'Population Count per byte',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CNT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE DUP_advsimd_elt => {
  title      => 'DUP (element)',
  brief      => 'Duplicate vector element to vector or scalar',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE DUP_advsimd_gen => {
  title      => 'DUP (general)',
  brief      => 'Duplicate general-purpose register to vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE EOR3_advsimd => {
  title      => 'EOR3',
  brief      => 'Three-way Exclusive OR',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EOR3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE EOR_advsimd => {
  title      => 'EOR (vector)',
  brief      => 'Bitwise Exclusive OR (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE EXT_advsimd => {
  title      => 'EXT',
  brief      => 'Extract vector from pair of vectors',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EXT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FABD_advsimd => {
  title      => 'FABD',
  brief      => 'Floating-point Absolute Difference (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FABD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FABS_advsimd => {
  title      => 'FABS (vector)',
  brief      => 'Floating-point Absolute value (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FABS_float => {
  title      => 'FABS (scalar)',
  brief      => 'Floating-point Absolute value (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FACGE_advsimd => {
  title      => 'FACGE',
  brief      => 'Floating-point Absolute Compare Greater than or Equal (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FACGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FACGT_advsimd => {
  title      => 'FACGT',
  brief      => 'Floating-point Absolute Compare Greater than (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FACGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FADD_advsimd => {
  title      => 'FADD (vector)',
  brief      => 'Floating-point Add (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FADD_float => {
  title      => 'FADD (scalar)',
  brief      => 'Floating-point Add (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FADDP_advsimd_pair => {
  title      => 'FADDP (scalar)',
  brief      => 'Floating-point Add Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FADDP_advsimd_vec => {
  title      => 'FADDP (vector)',
  brief      => 'Floating-point Add Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCADD_advsimd_vec => {
  title      => 'FCADD',
  brief      => 'Floating-point Complex Add',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCCMP_float => {
  title      => 'FCCMP',
  brief      => 'Floating-point Conditional quiet Compare (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCCMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCCMPE_float => {
  title      => 'FCCMPE',
  brief      => 'Floating-point Conditional signaling Compare (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCCMPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMEQ_advsimd_reg => {
  title      => 'FCMEQ (register)',
  brief      => 'Floating-point Compare Equal (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMEQ_advsimd_zero => {
  title      => 'FCMEQ (zero)',
  brief      => 'Floating-point Compare Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMGE_advsimd_reg => {
  title      => 'FCMGE (register)',
  brief      => 'Floating-point Compare Greater than or Equal (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMGE_advsimd_zero => {
  title      => 'FCMGE (zero)',
  brief      => 'Floating-point Compare Greater than or Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMGT_advsimd_reg => {
  title      => 'FCMGT (register)',
  brief      => 'Floating-point Compare Greater than (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMGT_advsimd_zero => {
  title      => 'FCMGT (zero)',
  brief      => 'Floating-point Compare Greater than zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMLA_advsimd_elt => {
  title      => 'FCMLA (by element)',
  brief      => 'Floating-point Complex Multiply Accumulate (by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMLA_advsimd_vec => {
  title      => 'FCMLA',
  brief      => 'Floating-point Complex Multiply Accumulate',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMLE_advsimd => {
  title      => 'FCMLE (zero)',
  brief      => 'Floating-point Compare Less than or Equal to zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMLT_advsimd => {
  title      => 'FCMLT (zero)',
  brief      => 'Floating-point Compare Less than zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMP_float => {
  title      => 'FCMP',
  brief      => 'Floating-point quiet Compare (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCMPE_float => {
  title      => 'FCMPE',
  brief      => 'Floating-point signaling Compare (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCSEL_float => {
  title      => 'FCSEL',
  brief      => 'Floating-point Conditional Select (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCSEL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVT_float => {
  title      => 'FCVT',
  brief      => 'Floating-point Convert precision (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTAS_advsimd => {
  title      => 'FCVTAS (vector)',
  brief      => 'Floating-point Convert to Signed integer, rounding to nearest with ties to Away (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTAS_float => {
  title      => 'FCVTAS (scalar)',
  brief      => 'Floating-point Convert to Signed integer, rounding to nearest with ties to Away (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTAU_advsimd => {
  title      => 'FCVTAU (vector)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding to nearest with ties to Away (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTAU_float => {
  title      => 'FCVTAU (scalar)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding to nearest with ties to Away (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTL_advsimd => {
  title      => 'FCVTL, FCVTL2',
  brief      => 'Floating-point Convert to higher precision Long (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTMS_advsimd => {
  title      => 'FCVTMS (vector)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Minus infinity (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTMS_float => {
  title      => 'FCVTMS (scalar)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Minus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTMU_advsimd => {
  title      => 'FCVTMU (vector)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Minus infinity (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTMU_float => {
  title      => 'FCVTMU (scalar)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Minus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTN_advsimd => {
  title      => 'FCVTN, FCVTN2',
  brief      => 'Floating-point Convert to lower precision Narrow (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTNS_advsimd => {
  title      => 'FCVTNS (vector)',
  brief      => 'Floating-point Convert to Signed integer, rounding to nearest with ties to even (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTNS_float => {
  title      => 'FCVTNS (scalar)',
  brief      => 'Floating-point Convert to Signed integer, rounding to nearest with ties to even (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTNU_advsimd => {
  title      => 'FCVTNU (vector)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding to nearest with ties to even (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTNU_float => {
  title      => 'FCVTNU (scalar)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding to nearest with ties to even (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTPS_advsimd => {
  title      => 'FCVTPS (vector)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Plus infinity (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTPS_float => {
  title      => 'FCVTPS (scalar)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Plus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTPU_advsimd => {
  title      => 'FCVTPU (vector)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Plus infinity (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTPU_float => {
  title      => 'FCVTPU (scalar)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Plus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTXN_advsimd => {
  title      => 'FCVTXN, FCVTXN2',
  brief      => 'Floating-point Convert to lower precision Narrow, rounding to odd (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTXN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZS_advsimd_fix => {
  title      => 'FCVTZS (vector, fixed-point)',
  brief      => 'Floating-point Convert to Signed fixed-point, rounding toward Zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZS_advsimd_int => {
  title      => 'FCVTZS (vector, integer)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZS_float_fix => {
  title      => 'FCVTZS (scalar, fixed-point)',
  brief      => 'Floating-point Convert to Signed fixed-point, rounding toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZS_float_int => {
  title      => 'FCVTZS (scalar, integer)',
  brief      => 'Floating-point Convert to Signed integer, rounding toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZU_advsimd_fix => {
  title      => 'FCVTZU (vector, fixed-point)',
  brief      => 'Floating-point Convert to Unsigned fixed-point, rounding toward Zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZU_advsimd_int => {
  title      => 'FCVTZU (vector, integer)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Zero (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZU_float_fix => {
  title      => 'FCVTZU (scalar, fixed-point)',
  brief      => 'Floating-point Convert to Unsigned fixed-point, rounding toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FCVTZU_float_int => {
  title      => 'FCVTZU (scalar, integer)',
  brief      => 'Floating-point Convert to Unsigned integer, rounding toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FDIV_advsimd => {
  title      => 'FDIV (vector)',
  brief      => 'Floating-point Divide (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FDIV_float => {
  title      => 'FDIV (scalar)',
  brief      => 'Floating-point Divide (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FJCVTZS => {
  title      => 'FJCVTZS',
  brief      => 'Floating-point Javascript Convert to Signed fixed-point, rounding toward Zero',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FJCVTZS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMADD_float => {
  title      => 'FMADD',
  brief      => 'Floating-point fused Multiply-Add (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAX_advsimd => {
  title      => 'FMAX (vector)',
  brief      => 'Floating-point Maximum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAX_float => {
  title      => 'FMAX (scalar)',
  brief      => 'Floating-point Maximum (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXNM_advsimd => {
  title      => 'FMAXNM (vector)',
  brief      => 'Floating-point Maximum Number (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXNM_float => {
  title      => 'FMAXNM (scalar)',
  brief      => 'Floating-point Maximum Number (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXNMP_advsimd_pair => {
  title      => 'FMAXNMP (scalar)',
  brief      => 'Floating-point Maximum Number of Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXNMP_advsimd_vec => {
  title      => 'FMAXNMP (vector)',
  brief      => 'Floating-point Maximum Number Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXNMV_advsimd => {
  title      => 'FMAXNMV',
  brief      => 'Floating-point Maximum Number across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMAXNMV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXP_advsimd_pair => {
  title      => 'FMAXP (scalar)',
  brief      => 'Floating-point Maximum of Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXP_advsimd_vec => {
  title      => 'FMAXP (vector)',
  brief      => 'Floating-point Maximum Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMAXV_advsimd => {
  title      => 'FMAXV',
  brief      => 'Floating-point Maximum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMAXV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMIN_advsimd => {
  title      => 'FMIN (vector)',
  brief      => 'Floating-point minimum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMIN_float => {
  title      => 'FMIN (scalar)',
  brief      => 'Floating-point Minimum (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINNM_advsimd => {
  title      => 'FMINNM (vector)',
  brief      => 'Floating-point Minimum Number (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINNM_float => {
  title      => 'FMINNM (scalar)',
  brief      => 'Floating-point Minimum Number (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINNMP_advsimd_pair => {
  title      => 'FMINNMP (scalar)',
  brief      => 'Floating-point Minimum Number of Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINNMP_advsimd_vec => {
  title      => 'FMINNMP (vector)',
  brief      => 'Floating-point Minimum Number Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINNMV_advsimd => {
  title      => 'FMINNMV',
  brief      => 'Floating-point Minimum Number across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMINNMV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINP_advsimd_pair => {
  title      => 'FMINP (scalar)',
  brief      => 'Floating-point Minimum of Pair of elements (scalar)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINP_advsimd_vec => {
  title      => 'FMINP (vector)',
  brief      => 'Floating-point Minimum Pairwise (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMINV_advsimd => {
  title      => 'FMINV',
  brief      => 'Floating-point Minimum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMINV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLA_advsimd_elt => {
  title      => 'FMLA (by element)',
  brief      => 'Floating-point fused Multiply-Add to accumulator (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLA_advsimd_vec => {
  title      => 'FMLA (vector)',
  brief      => 'Floating-point fused Multiply-Add to accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLAL_advsimd_elt => {
  title      => 'FMLAL, FMLAL2 (by element)',
  brief      => 'Floating-point fused Multiply-Add Long to accumulator (by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLAL_advsimd_vec => {
  title      => 'FMLAL, FMLAL2 (vector)',
  brief      => 'Floating-point fused Multiply-Add Long to accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLS_advsimd_elt => {
  title      => 'FMLS (by element)',
  brief      => 'Floating-point fused Multiply-Subtract from accumulator (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLS_advsimd_vec => {
  title      => 'FMLS (vector)',
  brief      => 'Floating-point fused Multiply-Subtract from accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLSL_advsimd_elt => {
  title      => 'FMLSL, FMLSL2 (by element)',
  brief      => 'Floating-point fused Multiply-Subtract Long from accumulator (by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMLSL_advsimd_vec => {
  title      => 'FMLSL, FMLSL2 (vector)',
  brief      => 'Floating-point fused Multiply-Subtract Long from accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMOV_advsimd => {
  title      => 'FMOV (vector, immediate)',
  brief      => 'Floating-point move immediate (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMOV_float => {
  title      => 'FMOV (register)',
  brief      => 'Floating-point Move register without conversion',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMOV_float_gen => {
  title      => 'FMOV (general)',
  brief      => 'Floating-point Move to or from general-purpose register without conversion',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMOV_float_imm => {
  title      => 'FMOV (scalar, immediate)',
  brief      => 'Floating-point move immediate (scalar)',
  docvars    => 'immediate-type=imm8f instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMSUB_float => {
  title      => 'FMSUB',
  brief      => 'Floating-point Fused Multiply-Subtract (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMUL_advsimd_elt => {
  title      => 'FMUL (by element)',
  brief      => 'Floating-point Multiply (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMUL_advsimd_vec => {
  title      => 'FMUL (vector)',
  brief      => 'Floating-point Multiply (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMUL_float => {
  title      => 'FMUL (scalar)',
  brief      => 'Floating-point Multiply (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMULX_advsimd_elt => {
  title      => 'FMULX (by element)',
  brief      => 'Floating-point Multiply extended (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FMULX_advsimd_vec => {
  title      => 'FMULX',
  brief      => 'Floating-point Multiply extended',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FNEG_advsimd => {
  title      => 'FNEG (vector)',
  brief      => 'Floating-point Negate (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FNEG_float => {
  title      => 'FNEG (scalar)',
  brief      => 'Floating-point Negate (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FNMADD_float => {
  title      => 'FNMADD',
  brief      => 'Floating-point Negated fused Multiply-Add (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FNMSUB_float => {
  title      => 'FNMSUB',
  brief      => 'Floating-point Negated fused Multiply-Subtract (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FNMUL_float => {
  title      => 'FNMUL (scalar)',
  brief      => 'Floating-point Multiply-Negate (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRECPE_advsimd => {
  title      => 'FRECPE',
  brief      => 'Floating-point Reciprocal Estimate',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRECPS_advsimd => {
  title      => 'FRECPS',
  brief      => 'Floating-point Reciprocal Step',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FRECPS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRECPX_advsimd => {
  title      => 'FRECPX',
  brief      => 'Floating-point Reciprocal exponent (scalar)',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FRECPX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT32X_advsimd => {
  title      => 'FRINT32X (vector)',
  brief      => 'Floating-point Round to 32-bit Integer, using current rounding mode (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32X',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT32X_float => {
  title      => 'FRINT32X (scalar)',
  brief      => 'Floating-point Round to 32-bit Integer, using current rounding mode (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT32X',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT32Z_advsimd => {
  title      => 'FRINT32Z (vector)',
  brief      => 'Floating-point Round to 32-bit Integer toward Zero (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT32Z_float => {
  title      => 'FRINT32Z (scalar)',
  brief      => 'Floating-point Round to 32-bit Integer toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT64X_advsimd => {
  title      => 'FRINT64X (vector)',
  brief      => 'Floating-point Round to 64-bit Integer, using current rounding mode (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64X',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT64X_float => {
  title      => 'FRINT64X (scalar)',
  brief      => 'Floating-point Round to 64-bit Integer, using current rounding mode (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT64X',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT64Z_advsimd => {
  title      => 'FRINT64Z (vector)',
  brief      => 'Floating-point Round to 64-bit Integer toward Zero (vector)',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINT64Z_float => {
  title      => 'FRINT64Z (scalar)',
  brief      => 'Floating-point Round to 64-bit Integer toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTA_advsimd => {
  title      => 'FRINTA (vector)',
  brief      => 'Floating-point Round to Integral, to nearest with ties to Away (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTA_float => {
  title      => 'FRINTA (scalar)',
  brief      => 'Floating-point Round to Integral, to nearest with ties to Away (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTI_advsimd => {
  title      => 'FRINTI (vector)',
  brief      => 'Floating-point Round to Integral, using current rounding mode (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTI_float => {
  title      => 'FRINTI (scalar)',
  brief      => 'Floating-point Round to Integral, using current rounding mode (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTM_advsimd => {
  title      => 'FRINTM (vector)',
  brief      => 'Floating-point Round to Integral, toward Minus infinity (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTM_float => {
  title      => 'FRINTM (scalar)',
  brief      => 'Floating-point Round to Integral, toward Minus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTN_advsimd => {
  title      => 'FRINTN (vector)',
  brief      => 'Floating-point Round to Integral, to nearest with ties to even (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTN_float => {
  title      => 'FRINTN (scalar)',
  brief      => 'Floating-point Round to Integral, to nearest with ties to even (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTP_advsimd => {
  title      => 'FRINTP (vector)',
  brief      => 'Floating-point Round to Integral, toward Plus infinity (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTP_float => {
  title      => 'FRINTP (scalar)',
  brief      => 'Floating-point Round to Integral, toward Plus infinity (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTX_advsimd => {
  title      => 'FRINTX (vector)',
  brief      => 'Floating-point Round to Integral exact, using current rounding mode (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTX_float => {
  title      => 'FRINTX (scalar)',
  brief      => 'Floating-point Round to Integral exact, using current rounding mode (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTZ_advsimd => {
  title      => 'FRINTZ (vector)',
  brief      => 'Floating-point Round to Integral, toward Zero (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRINTZ_float => {
  title      => 'FRINTZ (scalar)',
  brief      => 'Floating-point Round to Integral, toward Zero (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRSQRTE_advsimd => {
  title      => 'FRSQRTE',
  brief      => 'Floating-point Reciprocal Square Root Estimate',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FRSQRTS_advsimd => {
  title      => 'FRSQRTS',
  brief      => 'Floating-point Reciprocal Square Root Step',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=FRSQRTS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FSQRT_advsimd => {
  title      => 'FSQRT (vector)',
  brief      => 'Floating-point Square Root (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FSQRT_float => {
  title      => 'FSQRT (scalar)',
  brief      => 'Floating-point Square Root (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FSUB_advsimd => {
  title      => 'FSUB (vector)',
  brief      => 'Floating-point Subtract (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE FSUB_float => {
  title      => 'FSUB (scalar)',
  brief      => 'Floating-point Subtract (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE INS_advsimd_elt => {
  title      => 'INS (element)',
  brief      => 'Insert vector element from another vector element',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-element',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE INS_advsimd_gen => {
  title      => 'INS (general)',
  brief      => 'Insert vector element from general-purpose register',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD1_advsimd_mult => {
  title      => 'LD1 (multiple structures)',
  brief      => 'Load multiple single-element structures to one, two, three, or four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD1_advsimd_sngl => {
  title      => 'LD1 (single structure)',
  brief      => 'Load one single-element structure to one lane of one register',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD1R_advsimd => {
  title      => 'LD1R',
  brief      => 'Load one single-element structure and Replicate to all lanes (of one register)',
  docvars    => 'as-structure-org=to-all-lanes instr-class=advsimd isa=A64 mnemonic=LD1R',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD2_advsimd_mult => {
  title      => 'LD2 (multiple structures)',
  brief      => 'Load multiple 2-element structures to two registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD2_advsimd_sngl => {
  title      => 'LD2 (single structure)',
  brief      => 'Load single 2-element structure to one lane of two registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD2R_advsimd => {
  title      => 'LD2R',
  brief      => 'Load single 2-element structure and Replicate to all lanes of two registers',
  docvars    => 'as-structure-org=to-all-lanes instr-class=advsimd isa=A64 mnemonic=LD2R',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD3_advsimd_mult => {
  title      => 'LD3 (multiple structures)',
  brief      => 'Load multiple 3-element structures to three registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD3_advsimd_sngl => {
  title      => 'LD3 (single structure)',
  brief      => 'Load single 3-element structure to one lane of three registers)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD3R_advsimd => {
  title      => 'LD3R',
  brief      => 'Load single 3-element structure and Replicate to all lanes of three registers',
  docvars    => 'as-structure-org=to-all-lanes instr-class=advsimd isa=A64 mnemonic=LD3R',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD4_advsimd_mult => {
  title      => 'LD4 (multiple structures)',
  brief      => 'Load multiple 4-element structures to four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD4_advsimd_sngl => {
  title      => 'LD4 (single structure)',
  brief      => 'Load single 4-element structure to one lane of four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LD4R_advsimd => {
  title      => 'LD4R',
  brief      => 'Load single 4-element structure and Replicate to all lanes of four registers',
  docvars    => 'as-structure-org=to-all-lanes instr-class=advsimd isa=A64 mnemonic=LD4R',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDNP_fpsimd => {
  title      => 'LDNP (SIMD&FP)',
  brief      => 'Load Pair of SIMD&FP registers, with Non-temporal hint',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=LDNP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDP_fpsimd => {
  title      => 'LDP (SIMD&FP)',
  brief      => 'Load Pair of SIMD&FP registers',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDR_imm_fpsimd => {
  title      => 'LDR (immediate, SIMD&FP)',
  brief      => 'Load SIMD&FP Register (immediate offset)',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=LDR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDR_lit_fpsimd => {
  title      => 'LDR (literal, SIMD&FP)',
  brief      => 'Load SIMD&FP Register (PC-relative literal)',
  docvars    => 'address-form=literal instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off19s',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDR_reg_fpsimd => {
  title      => 'LDR (register, SIMD&FP)',
  brief      => 'Load SIMD&FP Register (register offset)',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE LDUR_fpsimd => {
  title      => 'LDUR (SIMD&FP)',
  brief      => 'Load SIMD&FP Register (unscaled offset)',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MLA_advsimd_elt => {
  title      => 'MLA (by element)',
  brief      => 'Multiply-Add to accumulator (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MLA_advsimd_vec => {
  title      => 'MLA (vector)',
  brief      => 'Multiply-Add to accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MLS_advsimd_elt => {
  title      => 'MLS (by element)',
  brief      => 'Multiply-Subtract from accumulator (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MLS_advsimd_vec => {
  title      => 'MLS (vector)',
  brief      => 'Multiply-Subtract from accumulator (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MOVI_advsimd => {
  title      => 'MOVI',
  brief      => 'Move Immediate (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MUL_advsimd_elt => {
  title      => 'MUL (by element)',
  brief      => 'Multiply (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MUL_advsimd_vec => {
  title      => 'MUL (vector)',
  brief      => 'Multiply (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MVNI_advsimd => {
  title      => 'MVNI',
  brief      => 'Move inverted Immediate (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=MVNI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE NEG_advsimd => {
  title      => 'NEG (vector)',
  brief      => 'Negate (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=NEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE NOT_advsimd => {
  title      => 'NOT',
  brief      => 'Bitwise NOT (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ORN_advsimd => {
  title      => 'ORN (vector)',
  brief      => 'Bitwise inclusive OR NOT (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ORR_advsimd_imm => {
  title      => 'ORR (vector, immediate)',
  brief      => 'Bitwise inclusive OR (vector, immediate)',
  docvars    => 'asimdimm-immtype=shifted-immediate instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ORR_advsimd_reg => {
  title      => 'ORR (vector, register)',
  brief      => 'Bitwise inclusive OR (vector, register)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE PMUL_advsimd => {
  title      => 'PMUL',
  brief      => 'Polynomial Multiply',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMUL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE PMULL_advsimd => {
  title      => 'PMULL, PMULL2',
  brief      => 'Polynomial Multiply Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE RADDHN_advsimd => {
  title      => 'RADDHN, RADDHN2',
  brief      => 'Rounding Add returning High Narrow',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RADDHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE RAX1_advsimd => {
  title      => 'RAX1',
  brief      => 'Rotate and Exclusive OR',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=RAX1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE RBIT_advsimd => {
  title      => 'RBIT (vector)',
  brief      => 'Reverse Bit order (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE REV16_advsimd => {
  title      => 'REV16 (vector)',
  brief      => 'Reverse elements in 16-bit halfwords (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE REV32_advsimd => {
  title      => 'REV32 (vector)',
  brief      => 'Reverse elements in 32-bit words (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE REV64_advsimd => {
  title      => 'REV64',
  brief      => 'Reverse elements in 64-bit doublewords (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV64',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE RSHRN_advsimd => {
  title      => 'RSHRN, RSHRN2',
  brief      => 'Rounding Shift Right Narrow (immediate)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE RSUBHN_advsimd => {
  title      => 'RSUBHN, RSUBHN2',
  brief      => 'Rounding Subtract returning High Narrow',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSUBHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SABA_advsimd => {
  title      => 'SABA',
  brief      => 'Signed Absolute difference and Accumulate',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SABAL_advsimd => {
  title      => 'SABAL, SABAL2',
  brief      => 'Signed Absolute difference and Accumulate Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SABD_advsimd => {
  title      => 'SABD',
  brief      => 'Signed Absolute Difference',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SABDL_advsimd => {
  title      => 'SABDL, SABDL2',
  brief      => 'Signed Absolute Difference Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SADALP_advsimd => {
  title      => 'SADALP',
  brief      => 'Signed Add and Accumulate Long Pairwise',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADALP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SADDL_advsimd => {
  title      => 'SADDL, SADDL2',
  brief      => 'Signed Add Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SADDLP_advsimd => {
  title      => 'SADDLP',
  brief      => 'Signed Add Long Pairwise',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDLP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SADDLV_advsimd => {
  title      => 'SADDLV',
  brief      => 'Signed Add Long across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SADDLV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SADDW_advsimd => {
  title      => 'SADDW, SADDW2',
  brief      => 'Signed Add Wide',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SCVTF_advsimd_fix => {
  title      => 'SCVTF (vector, fixed-point)',
  brief      => 'Signed fixed-point Convert to Floating-point (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SCVTF_advsimd_int => {
  title      => 'SCVTF (vector, integer)',
  brief      => 'Signed integer Convert to Floating-point (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SCVTF_float_fix => {
  title      => 'SCVTF (scalar, fixed-point)',
  brief      => 'Signed fixed-point Convert to Floating-point (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SCVTF_float_int => {
  title      => 'SCVTF (scalar, integer)',
  brief      => 'Signed integer Convert to Floating-point (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SDOT_advsimd_elt => {
  title      => 'SDOT (by element)',
  brief      => 'Dot Product signed arithmetic (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SDOT_advsimd_vec => {
  title      => 'SDOT (vector)',
  brief      => 'Dot Product signed arithmetic (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1C_advsimd => {
  title      => 'SHA1C',
  brief      => 'SHA1 hash update (choose)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1C',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1H_advsimd => {
  title      => 'SHA1H',
  brief      => 'SHA1 fixed rotate',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1H',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1M_advsimd => {
  title      => 'SHA1M',
  brief      => 'SHA1 hash update (majority)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1M',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1P_advsimd => {
  title      => 'SHA1P',
  brief      => 'SHA1 hash update (parity)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1P',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1SU0_advsimd => {
  title      => 'SHA1SU0',
  brief      => 'SHA1 schedule update 0',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU0',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA1SU1_advsimd => {
  title      => 'SHA1SU1',
  brief      => 'SHA1 schedule update 1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256H2_advsimd => {
  title      => 'SHA256H2',
  brief      => 'SHA256 hash update (part 2)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256H_advsimd => {
  title      => 'SHA256H',
  brief      => 'SHA256 hash update (part 1)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256SU0_advsimd => {
  title      => 'SHA256SU0',
  brief      => 'SHA256 schedule update 0',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU0',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA256SU1_advsimd => {
  title      => 'SHA256SU1',
  brief      => 'SHA256 schedule update 1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA512H2_advsimd => {
  title      => 'SHA512H2',
  brief      => 'SHA512 Hash update part 2',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA512H_advsimd => {
  title      => 'SHA512H',
  brief      => 'SHA512 Hash update part 1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA512SU0_advsimd => {
  title      => 'SHA512SU0',
  brief      => 'SHA512 Schedule Update 0',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU0',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHA512SU1_advsimd => {
  title      => 'SHA512SU1',
  brief      => 'SHA512 Schedule Update 1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHADD_advsimd => {
  title      => 'SHADD',
  brief      => 'Signed Halving Add',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHL_advsimd => {
  title      => 'SHL',
  brief      => 'Shift Left (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHLL_advsimd => {
  title      => 'SHLL, SHLL2',
  brief      => 'Shift Left Long (by element size)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHLL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHRN_advsimd => {
  title      => 'SHRN, SHRN2',
  brief      => 'Shift Right Narrow (immediate)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SHSUB_advsimd => {
  title      => 'SHSUB',
  brief      => 'Signed Halving Subtract',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SLI_advsimd => {
  title      => 'SLI',
  brief      => 'Shift Left and Insert (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SLI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3PARTW1_advsimd => {
  title      => 'SM3PARTW1',
  brief      => 'SM3PARTW1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3PARTW2_advsimd => {
  title      => 'SM3PARTW2',
  brief      => 'SM3PARTW2',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3SS1_advsimd => {
  title      => 'SM3SS1',
  brief      => 'SM3SS1',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3SS1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3TT1A_advsimd => {
  title      => 'SM3TT1A',
  brief      => 'SM3TT1A',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1A',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3TT1B_advsimd => {
  title      => 'SM3TT1B',
  brief      => 'SM3TT1B',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1B',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3TT2A_advsimd => {
  title      => 'SM3TT2A',
  brief      => 'SM3TT2A',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2A',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM3TT2B_advsimd => {
  title      => 'SM3TT2B',
  brief      => 'SM3TT2B',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2B',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM4E_advsimd => {
  title      => 'SM4E',
  brief      => 'SM4 Encode',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4E',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SM4EKEY_advsimd => {
  title      => 'SM4EKEY',
  brief      => 'SM4 Key',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4EKEY',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMAX_advsimd => {
  title      => 'SMAX',
  brief      => 'Signed Maximum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMAXP_advsimd => {
  title      => 'SMAXP',
  brief      => 'Signed Maximum Pairwise',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAXP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMAXV_advsimd => {
  title      => 'SMAXV',
  brief      => 'Signed Maximum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMAXV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMIN_advsimd => {
  title      => 'SMIN',
  brief      => 'Signed Minimum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMINP_advsimd => {
  title      => 'SMINP',
  brief      => 'Signed Minimum Pairwise',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMINP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMINV_advsimd => {
  title      => 'SMINV',
  brief      => 'Signed Minimum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMINV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMLAL_advsimd_elt => {
  title      => 'SMLAL, SMLAL2 (by element)',
  brief      => 'Signed Multiply-Add Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMLAL_advsimd_vec => {
  title      => 'SMLAL, SMLAL2 (vector)',
  brief      => 'Signed Multiply-Add Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMLSL_advsimd_elt => {
  title      => 'SMLSL, SMLSL2 (by element)',
  brief      => 'Signed Multiply-Subtract Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMLSL_advsimd_vec => {
  title      => 'SMLSL, SMLSL2 (vector)',
  brief      => 'Signed Multiply-Subtract Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMMLA_advsimd_vec => {
  title      => 'SMMLA (vector)',
  brief      => 'Signed 8-bit integer matrix multiply-accumulate (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMOV_advsimd => {
  title      => 'SMOV',
  brief      => 'Signed Move vector element to general-purpose register',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMULL_advsimd_elt => {
  title      => 'SMULL, SMULL2 (by element)',
  brief      => 'Signed Multiply Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SMULL_advsimd_vec => {
  title      => 'SMULL, SMULL2 (vector)',
  brief      => 'Signed Multiply Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQABS_advsimd => {
  title      => 'SQABS',
  brief      => 'Signed saturating Absolute value',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQABS',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQADD_advsimd => {
  title      => 'SQADD',
  brief      => 'Signed saturating Add',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMLAL_advsimd_elt => {
  title      => 'SQDMLAL, SQDMLAL2 (by element)',
  brief      => 'Signed saturating Doubling Multiply-Add Long (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMLAL_advsimd_vec => {
  title      => 'SQDMLAL, SQDMLAL2 (vector)',
  brief      => 'Signed saturating Doubling Multiply-Add Long',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMLSL_advsimd_elt => {
  title      => 'SQDMLSL, SQDMLSL2 (by element)',
  brief      => 'Signed saturating Doubling Multiply-Subtract Long (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMLSL_advsimd_vec => {
  title      => 'SQDMLSL, SQDMLSL2 (vector)',
  brief      => 'Signed saturating Doubling Multiply-Subtract Long',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMULH_advsimd_elt => {
  title      => 'SQDMULH (by element)',
  brief      => 'Signed saturating Doubling Multiply returning High half (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMULH_advsimd_vec => {
  title      => 'SQDMULH (vector)',
  brief      => 'Signed saturating Doubling Multiply returning High half',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMULL_advsimd_elt => {
  title      => 'SQDMULL, SQDMULL2 (by element)',
  brief      => 'Signed saturating Doubling Multiply Long (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQDMULL_advsimd_vec => {
  title      => 'SQDMULL, SQDMULL2 (vector)',
  brief      => 'Signed saturating Doubling Multiply Long',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQNEG_advsimd => {
  title      => 'SQNEG',
  brief      => 'Signed saturating Negate',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQNEG',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMLAH_advsimd_elt => {
  title      => 'SQRDMLAH (by element)',
  brief      => 'Signed Saturating Rounding Doubling Multiply Accumulate returning High Half (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMLAH_advsimd_vec => {
  title      => 'SQRDMLAH (vector)',
  brief      => 'Signed Saturating Rounding Doubling Multiply Accumulate returning High Half (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMLSH_advsimd_elt => {
  title      => 'SQRDMLSH (by element)',
  brief      => 'Signed Saturating Rounding Doubling Multiply Subtract returning High Half (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMLSH_advsimd_vec => {
  title      => 'SQRDMLSH (vector)',
  brief      => 'Signed Saturating Rounding Doubling Multiply Subtract returning High Half (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMULH_advsimd_elt => {
  title      => 'SQRDMULH (by element)',
  brief      => 'Signed saturating Rounding Doubling Multiply returning High half (by element)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRDMULH_advsimd_vec => {
  title      => 'SQRDMULH (vector)',
  brief      => 'Signed saturating Rounding Doubling Multiply returning High half',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRSHL_advsimd => {
  title      => 'SQRSHL',
  brief      => 'Signed saturating Rounding Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRSHRN_advsimd => {
  title      => 'SQRSHRN, SQRSHRN2',
  brief      => 'Signed saturating Rounded Shift Right Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQRSHRUN_advsimd => {
  title      => 'SQRSHRUN, SQRSHRUN2',
  brief      => 'Signed saturating Rounded Shift Right Unsigned Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQRSHRUN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSHL_advsimd_imm => {
  title      => 'SQSHL (immediate)',
  brief      => 'Signed saturating Shift Left (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSHL_advsimd_reg => {
  title      => 'SQSHL (register)',
  brief      => 'Signed saturating Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSHLU_advsimd => {
  title      => 'SQSHLU',
  brief      => 'Signed saturating Shift Left Unsigned (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSHLU',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSHRN_advsimd => {
  title      => 'SQSHRN, SQSHRN2',
  brief      => 'Signed saturating Shift Right Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSHRUN_advsimd => {
  title      => 'SQSHRUN, SQSHRUN2',
  brief      => 'Signed saturating Shift Right Unsigned Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSHRUN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQSUB_advsimd => {
  title      => 'SQSUB',
  brief      => 'Signed saturating Subtract',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQXTN_advsimd => {
  title      => 'SQXTN, SQXTN2',
  brief      => 'Signed saturating extract Narrow',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQXTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SQXTUN_advsimd => {
  title      => 'SQXTUN, SQXTUN2',
  brief      => 'Signed saturating extract Unsigned Narrow',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SQXTUN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SRHADD_advsimd => {
  title      => 'SRHADD',
  brief      => 'Signed Rounding Halving Add',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SRI_advsimd => {
  title      => 'SRI',
  brief      => 'Shift Right and Insert (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SRI',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SRSHL_advsimd => {
  title      => 'SRSHL',
  brief      => 'Signed Rounding Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SRSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SRSHR_advsimd => {
  title      => 'SRSHR',
  brief      => 'Signed Rounding Shift Right (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SRSHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SRSRA_advsimd => {
  title      => 'SRSRA',
  brief      => 'Signed Rounding Shift Right and Accumulate (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SRSRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSHL_advsimd => {
  title      => 'SSHL',
  brief      => 'Signed Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSHLL_advsimd => {
  title      => 'SSHLL, SSHLL2',
  brief      => 'Signed Shift Left Long (immediate)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHLL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSHR_advsimd => {
  title      => 'SSHR',
  brief      => 'Signed Shift Right (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SSHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSRA_advsimd => {
  title      => 'SSRA',
  brief      => 'Signed Shift Right and Accumulate (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SSRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSUBL_advsimd => {
  title      => 'SSUBL, SSUBL2',
  brief      => 'Signed Subtract Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SSUBW_advsimd => {
  title      => 'SSUBW, SSUBW2',
  brief      => 'Signed Subtract Wide',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST1_advsimd_mult => {
  title      => 'ST1 (multiple structures)',
  brief      => 'Store multiple single-element structures from one, two, three, or four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST1_advsimd_sngl => {
  title      => 'ST1 (single structure)',
  brief      => 'Store a single-element structure from one lane of one register',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST2_advsimd_mult => {
  title      => 'ST2 (multiple structures)',
  brief      => 'Store multiple 2-element structures from two registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST2_advsimd_sngl => {
  title      => 'ST2 (single structure)',
  brief      => 'Store single 2-element structure from one lane of two registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST3_advsimd_mult => {
  title      => 'ST3 (multiple structures)',
  brief      => 'Store multiple 3-element structures from three registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST3_advsimd_sngl => {
  title      => 'ST3 (single structure)',
  brief      => 'Store single 3-element structure from one lane of three registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST4_advsimd_mult => {
  title      => 'ST4 (multiple structures)',
  brief      => 'Store multiple 4-element structures from four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ST4_advsimd_sngl => {
  title      => 'ST4 (single structure)',
  brief      => 'Store single 4-element structure from one lane of four registers',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE STNP_fpsimd => {
  title      => 'STNP (SIMD&FP)',
  brief      => 'Store Pair of SIMD&FP registers, with Non-temporal hint',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=STNP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE STP_fpsimd => {
  title      => 'STP (SIMD&FP)',
  brief      => 'Store Pair of SIMD&FP registers',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE STR_imm_fpsimd => {
  title      => 'STR (immediate, SIMD&FP)',
  brief      => 'Store SIMD&FP register (immediate offset)',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=STR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE STR_reg_fpsimd => {
  title      => 'STR (register, SIMD&FP)',
  brief      => 'Store SIMD&FP register (register offset)',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE STUR_fpsimd => {
  title      => 'STUR (SIMD&FP)',
  brief      => 'Store SIMD&FP register (unscaled offset)',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SUB_advsimd => {
  title      => 'SUB (vector)',
  brief      => 'Subtract (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SUBHN_advsimd => {
  title      => 'SUBHN, SUBHN2',
  brief      => 'Subtract returning High Narrow',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUBHN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SUDOT_advsimd_elt => {
  title      => 'SUDOT (by element)',
  brief      => 'Dot product with signed and unsigned integers (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SUDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE SUQADD_advsimd => {
  title      => 'SUQADD',
  brief      => 'Signed saturating Accumulate of Unsigned value',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SUQADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE TBL_advsimd => {
  title      => 'TBL',
  brief      => 'Table vector Lookup',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE TBX_advsimd => {
  title      => 'TBX',
  brief      => 'Table vector lookup extension',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE TRN1_advsimd => {
  title      => 'TRN1',
  brief      => 'Transpose vectors (primary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE TRN2_advsimd => {
  title      => 'TRN2',
  brief      => 'Transpose vectors (secondary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UABA_advsimd => {
  title      => 'UABA',
  brief      => 'Unsigned Absolute difference and Accumulate',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UABAL_advsimd => {
  title      => 'UABAL, UABAL2',
  brief      => 'Unsigned Absolute difference and Accumulate Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UABD_advsimd => {
  title      => 'UABD',
  brief      => 'Unsigned Absolute Difference (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UABDL_advsimd => {
  title      => 'UABDL, UABDL2',
  brief      => 'Unsigned Absolute Difference Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UADALP_advsimd => {
  title      => 'UADALP',
  brief      => 'Unsigned Add and Accumulate Long Pairwise',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADALP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UADDL_advsimd => {
  title      => 'UADDL, UADDL2',
  brief      => 'Unsigned Add Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UADDLP_advsimd => {
  title      => 'UADDLP',
  brief      => 'Unsigned Add Long Pairwise',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDLP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UADDLV_advsimd => {
  title      => 'UADDLV',
  brief      => 'Unsigned sum Long across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UADDLV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UADDW_advsimd => {
  title      => 'UADDW, UADDW2',
  brief      => 'Unsigned Add Wide',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UCVTF_advsimd_fix => {
  title      => 'UCVTF (vector, fixed-point)',
  brief      => 'Unsigned fixed-point Convert to Floating-point (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UCVTF_advsimd_int => {
  title      => 'UCVTF (vector, integer)',
  brief      => 'Unsigned integer Convert to Floating-point (vector)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UCVTF_float_fix => {
  title      => 'UCVTF (scalar, fixed-point)',
  brief      => 'Unsigned fixed-point Convert to Floating-point (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UCVTF_float_int => {
  title      => 'UCVTF (scalar, integer)',
  brief      => 'Unsigned integer Convert to Floating-point (scalar)',
  docvars    => 'instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UDOT_advsimd_elt => {
  title      => 'UDOT (by element)',
  brief      => 'Dot Product unsigned arithmetic (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UDOT_advsimd_vec => {
  title      => 'UDOT (vector)',
  brief      => 'Dot Product unsigned arithmetic (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UHADD_advsimd => {
  title      => 'UHADD',
  brief      => 'Unsigned Halving Add',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UHSUB_advsimd => {
  title      => 'UHSUB',
  brief      => 'Unsigned Halving Subtract',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMAX_advsimd => {
  title      => 'UMAX',
  brief      => 'Unsigned Maximum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAX',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMAXP_advsimd => {
  title      => 'UMAXP',
  brief      => 'Unsigned Maximum Pairwise',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAXP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMAXV_advsimd => {
  title      => 'UMAXV',
  brief      => 'Unsigned Maximum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMAXV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMIN_advsimd => {
  title      => 'UMIN',
  brief      => 'Unsigned Minimum (vector)',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMIN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMINP_advsimd => {
  title      => 'UMINP',
  brief      => 'Unsigned Minimum Pairwise',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMINP',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMINV_advsimd => {
  title      => 'UMINV',
  brief      => 'Unsigned Minimum across Vector',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMINV',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMLAL_advsimd_elt => {
  title      => 'UMLAL, UMLAL2 (by element)',
  brief      => 'Unsigned Multiply-Add Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMLAL_advsimd_vec => {
  title      => 'UMLAL, UMLAL2 (vector)',
  brief      => 'Unsigned Multiply-Add Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLAL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMLSL_advsimd_elt => {
  title      => 'UMLSL, UMLSL2 (by element)',
  brief      => 'Unsigned Multiply-Subtract Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMLSL_advsimd_vec => {
  title      => 'UMLSL, UMLSL2 (vector)',
  brief      => 'Unsigned Multiply-Subtract Long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLSL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMMLA_advsimd_vec => {
  title      => 'UMMLA (vector)',
  brief      => 'Unsigned 8-bit integer matrix multiply-accumulate (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMOV_advsimd => {
  title      => 'UMOV',
  brief      => 'Unsigned Move vector element to general-purpose register',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMULL_advsimd_elt => {
  title      => 'UMULL, UMULL2 (by element)',
  brief      => 'Unsigned Multiply Long (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UMULL_advsimd_vec => {
  title      => 'UMULL, UMULL2 (vector)',
  brief      => 'Unsigned Multiply long (vector)',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMULL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQADD_advsimd => {
  title      => 'UQADD',
  brief      => 'Unsigned saturating Add',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQRSHL_advsimd => {
  title      => 'UQRSHL',
  brief      => 'Unsigned saturating Rounding Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQRSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQRSHRN_advsimd => {
  title      => 'UQRSHRN, UQRSHRN2',
  brief      => 'Unsigned saturating Rounded Shift Right Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQRSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQSHL_advsimd_imm => {
  title      => 'UQSHL (immediate)',
  brief      => 'Unsigned saturating Shift Left (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQSHL_advsimd_reg => {
  title      => 'UQSHL (register)',
  brief      => 'Unsigned saturating Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQSHRN_advsimd => {
  title      => 'UQSHRN, UQSHRN2',
  brief      => 'Unsigned saturating Shift Right Narrow (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQSHRN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQSUB_advsimd => {
  title      => 'UQSUB',
  brief      => 'Unsigned saturating Subtract',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQSUB',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UQXTN_advsimd => {
  title      => 'UQXTN, UQXTN2',
  brief      => 'Unsigned saturating extract Narrow',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UQXTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URECPE_advsimd => {
  title      => 'URECPE',
  brief      => 'Unsigned Reciprocal Estimate',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URECPE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URHADD_advsimd => {
  title      => 'URHADD',
  brief      => 'Unsigned Rounding Halving Add',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URHADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URSHL_advsimd => {
  title      => 'URSHL',
  brief      => 'Unsigned Rounding Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=URSHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URSHR_advsimd => {
  title      => 'URSHR',
  brief      => 'Unsigned Rounding Shift Right (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=URSHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URSQRTE_advsimd => {
  title      => 'URSQRTE',
  brief      => 'Unsigned Reciprocal Square Root Estimate',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSQRTE',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE URSRA_advsimd => {
  title      => 'URSRA',
  brief      => 'Unsigned Rounding Shift Right and Accumulate (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=URSRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USDOT_advsimd_elt => {
  title      => 'USDOT (by element)',
  brief      => 'Dot Product with unsigned and signed integers (vector, by element)',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USDOT_advsimd_vec => {
  title      => 'USDOT (vector)',
  brief      => 'Dot Product with unsigned and signed integers (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USHL_advsimd => {
  title      => 'USHL',
  brief      => 'Unsigned Shift Left (register)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=USHL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USHLL_advsimd => {
  title      => 'USHLL, USHLL2',
  brief      => 'Unsigned Shift Left Long (immediate)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHLL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USHR_advsimd => {
  title      => 'USHR',
  brief      => 'Unsigned Shift Right (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=USHR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USMMLA_advsimd_vec => {
  title      => 'USMMLA (vector)',
  brief      => 'Unsigned and signed 8-bit integer matrix multiply-accumulate (vector)',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USMMLA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USQADD_advsimd => {
  title      => 'USQADD',
  brief      => 'Unsigned saturating Accumulate of Signed value',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=USQADD',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USRA_advsimd => {
  title      => 'USRA',
  brief      => 'Unsigned Shift Right and Accumulate (immediate)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=USRA',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USUBL_advsimd => {
  title      => 'USUBL, USUBL2',
  brief      => 'Unsigned Subtract Long',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBL',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE USUBW_advsimd => {
  title      => 'USUBW, USUBW2',
  brief      => 'Unsigned Subtract Wide',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBW',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UZP1_advsimd => {
  title      => 'UZP1',
  brief      => 'Unzip vectors (primary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE UZP2_advsimd => {
  title      => 'UZP2',
  brief      => 'Unzip vectors (secondary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE XAR_advsimd => {
  title      => 'XAR',
  brief      => 'Exclusive OR and Rotate',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=XAR',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE XTN_advsimd => {
  title      => 'XTN, XTN2',
  brief      => 'Extract Narrow',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=XTN',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ZIP1_advsimd => {
  title      => 'ZIP1',
  brief      => 'Zip vectors (primary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP1',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE ZIP2_advsimd => {
  title      => 'ZIP2',
  brief      => 'Zip vectors (secondary)',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP2',
  metadata   => 'alias=0 alphaindex=FPSIMD',
};

PAGE MOV_DUP_advsimd_elt => {
  title      => 'MOV (scalar)',
  brief      => 'Move vector element to scalar',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=scalar-from-element',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE MOV_INS_advsimd_elt => {
  title      => 'MOV (element)',
  brief      => 'Move vector element to another vector element',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=element-from-element',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE MOV_INS_advsimd_gen => {
  title      => 'MOV (from general)',
  brief      => 'Move general-purpose register to a vector element',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE MOV_ORR_advsimd_reg => {
  title      => 'MOV (vector)',
  brief      => 'Move vector',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=ORR vector-xfer-type=vector-from-vector',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE MOV_UMOV_advsimd => {
  title      => 'MOV (to general)',
  brief      => 'Move vector element to general-purpose register',
  docvars    => 'alias_mnemonic=MOV instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE MVN_NOT_advsimd => {
  title      => 'MVN',
  brief      => 'Bitwise NOT (vector)',
  docvars    => 'advsimd-type=simd alias_mnemonic=MVN instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE SXTL_SSHLL_advsimd => {
  title      => 'SXTL, SXTL2',
  brief      => 'Signed extend Long',
  docvars    => 'advsimd-type=simd alias_mnemonic=SXTL instr-class=advsimd isa=A64 mnemonic=SSHLL',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};

PAGE UXTL_USHLL_advsimd => {
  title      => 'UXTL, UXTL2',
  brief      => 'Unsigned extend Long',
  docvars    => 'advsimd-type=simd alias_mnemonic=UXTL instr-class=advsimd isa=A64 mnemonic=USHLL',
  metadata   => 'alias=1 alphaindex=FPSIMD',
};


1;

