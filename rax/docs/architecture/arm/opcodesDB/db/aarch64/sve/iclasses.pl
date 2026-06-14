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

ICLASS sve_crypto_binary_const => {
  title      => 'SVE2 crypto constructive binary operations',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_crypto',
};

ICLASS sve_crypto_binary_dest => {
  title      => 'SVE2 crypto destructive binary operations',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b10001 op:u=0bx ig2=0b11100 o2:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_crypto',
};

ICLASS sve_crypto_unary => {
  title      => 'SVE2 crypto unary operations',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b10000011100 op:u=0bx ig2=0b00000 Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_crypto',
};

ICLASS sve_fp_2op_i_p_zds => {
  title      => 'SVE floating-point arithmetic with immediate (predicated)',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b100 Pg:u=0bxxx ig3=0b0000 i1:u=0bx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_pred',
};

ICLASS sve_fp_2op_p_pd => {
  title      => 'SVE floating-point compare with zero',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0100 eq:u=0bx lt:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_cmpzero',
};

ICLASS sve_fp_2op_p_vd => {
  title      => 'SVE floating-point serial reduction (predicated)',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_2op_p_zd_a => {
  title      => 'SVE floating-point round to integral value',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary',
};

ICLASS sve_fp_2op_p_zd_b_0 => {
  title      => 'SVE floating-point convert precision',
  diagram    => 'ig0=0b01100101 opc:u=0bxx ig1=0b0010 opc2:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary',
};

ICLASS sve_fp_2op_p_zd_b_1 => {
  title      => 'SVE floating-point unary operations',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0011 opc:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary',
};

ICLASS sve_fp_2op_p_zd_c => {
  title      => 'SVE integer convert to floating-point',
  diagram    => 'ig0=0b01100101 opc:u=0bxx ig1=0b010 opc2:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary',
};

ICLASS sve_fp_2op_p_zd_d => {
  title      => 'SVE floating-point convert to integer',
  diagram    => 'ig0=0b01100101 opc:u=0bxx ig1=0b011 opc2:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary',
};

ICLASS sve_fp_2op_p_zds => {
  title      => 'SVE floating-point arithmetic (predicated)',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b00 opc:u=0bxxxx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_pred',
};

ICLASS sve_fp_2op_u_zd => {
  title      => 'SVE floating-point reciprocal estimate (unpredicated)',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b001 opc:u=0bxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_unary_unpred',
};

ICLASS sve_fp_3op_p_pd => {
  title      => 'SVE floating-point compare vectors',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op:u=0bx ig2=0b1 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx o3:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_3op_p_zds_a => {
  title      => 'SVE floating-point multiply-accumulate writing addend',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 opc:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma',
};

ICLASS sve_fp_3op_p_zds_b => {
  title      => 'SVE floating-point multiply-accumulate writing multiplicand',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b1 Za:u=0bxxxxx ig2=0b1 opc:u=0bxx Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma',
};

ICLASS sve_fp_3op_u_zd => {
  title      => 'SVE floating-point arithmetic (unpredicated)',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b000 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fast_red => {
  title      => 'SVE floating-point recursive reduction',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fcadd => {
  title      => 'SVE floating-point complex add (predicated)',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b00000 rot:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fcmla => {
  title      => 'SVE floating-point complex multiply-add (predicated)',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 rot:u=0bxx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fcmla_by_indexed_elem => {
  title      => 'SVE floating-point complex multiply-add (indexed)',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fcvt2 => {
  title      => 'SVE floating-point convert precision odd elements',
  diagram    => 'ig0=0b01100100 opc:u=0bxx ig1=0b0010 opc2:u=0bxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fdot => {
  title      => 'SVE BFloat16 floating-point dot product',
  diagram    => 'ig0=0b011001000 op:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b100000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma_long',
};

ICLASS sve_fp_fdot_by_indexed_elem => {
  title      => 'SVE BFloat16 floating-point dot product (indexed)',
  diagram    => 'ig0=0b011001000 op:u=0bx ig1=0b1 i2:u=0bxx Zm:u=0bxxx ig2=0b010000 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem',
};

ICLASS sve_fp_fma_by_indexed_elem => {
  title      => 'SVE floating-point multiply-add (indexed)',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00000 op:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fma_long => {
  title      => 'SVE floating-point multiply-add long',
  diagram    => 'ig0=0b011001001 o2:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b10 op:u=0bx ig3=0b00 T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma_long',
};

ICLASS sve_fp_fma_long_by_indexed_elem => {
  title      => 'SVE floating-point multiply-add long (indexed)',
  diagram    => 'ig0=0b011001001 o2:u=0bx ig1=0b1 i3h:u=0bxx Zm:u=0bxxx ig2=0b01 op:u=0bx ig3=0b0 i3l:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_fma_long_by_indexed_elem',
};

ICLASS sve_fp_fmmla => {
  title      => 'SVE floating point matrix multiply accumulate',
  diagram    => 'ig0=0b01100100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b111001 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_fmul_by_indexed_elem => {
  title      => 'SVE floating-point multiply (indexed)',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_fp_ftmad => {
  title      => 'SVE floating-point trig multiply-add coefficient',
  diagram    => 'ig0=0b01100101 size:u=0bxx ig1=0b010 imm3:u=0bxxx ig2=0b100000 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_fp_pred',
};

ICLASS sve_fp_pairwise => {
  title      => 'SVE2 floating-point pairwise operations',
  diagram    => 'ig0=0b01100100 size:u=0bxx ig1=0b010 opc:u=0bxxx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_arith_imm0 => {
  title      => 'SVE integer add/subtract immediate (unpredicated)',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc:u=0bxxx ig2=0b11 sh:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_unpred',
};

ICLASS sve_int_arith_imm1 => {
  title      => 'SVE integer min/max immediate (unpredicated)',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b101 opc:u=0bxxx ig2=0b11 o2:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_unpred',
};

ICLASS sve_int_arith_imm2 => {
  title      => 'SVE integer multiply immediate (unpredicated)',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b110 opc:u=0bxxx ig2=0b11 o2:u=0bx imm8:u=0bxxxxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_unpred',
};

ICLASS sve_int_arith_vl => {
  title      => 'SVE stack frame adjustment',
  diagram    => 'ig0=0b000001000 op:u=0bx ig1=0b1 Rn:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_alloca',
};

ICLASS sve_int_bin_cons_arit_0 => {
  title      => 'SVE integer add/subtract vectors (unpredicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_bin_cons_log => {
  title      => 'SVE bitwise logical operations (unpredicated)',
  diagram    => 'ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_logical',
};

ICLASS sve_int_bin_cons_misc_0_a => {
  title      => 'SVE address generation',
  diagram    => 'ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1010 msz:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_bin_cons_misc_0_b => {
  title      => 'SVE floating-point trig select coefficient',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b10110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_misc',
};

ICLASS sve_int_bin_cons_misc_0_c => {
  title      => 'SVE floating-point exponential accelerator',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b101110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_misc',
};

ICLASS sve_int_bin_cons_misc_0_d => {
  title      => 'SVE constructive prefix (unpredicated)',
  diagram    => 'ig0=0b00000100 opc:u=0bxx ig1=0b1 opc2:u=0bxxxxx ig2=0b101111 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_misc',
};

ICLASS sve_int_bin_cons_shift_a => {
  title      => 'SVE bitwise shift by wide elements (unpredicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b1000 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_shift',
};

ICLASS sve_int_bin_cons_shift_b => {
  title      => 'SVE bitwise shift by immediate (unpredicated)',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b1001 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_shift',
};

ICLASS sve_int_bin_pred_arit_0 => {
  title      => 'SVE integer add/subtract vectors (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc:u=0bxxx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_bin',
};

ICLASS sve_int_bin_pred_arit_1 => {
  title      => 'SVE integer min/max/difference (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc:u=0bxx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_bin',
};

ICLASS sve_int_bin_pred_arit_2 => {
  title      => 'SVE integer multiply vectors (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0100 H:u=0bx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_bin',
};

ICLASS sve_int_bin_pred_div => {
  title      => 'SVE integer divide vectors (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0101 R:u=0bx U:u=0bx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_bin',
};

ICLASS sve_int_bin_pred_log => {
  title      => 'SVE bitwise logical operations (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b000 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_bin',
};

ICLASS sve_int_bin_pred_shift_0 => {
  title      => 'SVE bitwise shift by immediate (predicated)',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b00 opc:u=0bxx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx tszl:u=0bxx imm3:u=0bxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_shift',
};

ICLASS sve_int_bin_pred_shift_1 => {
  title      => 'SVE bitwise shift by vector (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 R:u=0bx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_shift',
};

ICLASS sve_int_bin_pred_shift_2 => {
  title      => 'SVE bitwise shift by wide elements (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 R:u=0bx L:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_shift',
};

ICLASS sve_int_break => {
  title      => 'SVE partition break condition',
  diagram    => 'ig0=0b00100101 B:u=0bx S:u=0bx ig1=0b01000001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx M:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_c',
};

ICLASS sve_int_brkn => {
  title      => 'SVE propagate break to next partition',
  diagram    => 'ig0=0b001001010 S:u=0bx ig1=0b01100001 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pdm:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_c',
};

ICLASS sve_int_brkp => {
  title      => 'SVE propagate break from previous partition',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b00 Pm:u=0bxxxx ig2=0b11 Pg:u=0bxxxx ig3=0b0 Pn:u=0bxxxx B:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_b',
};

ICLASS sve_int_cmp_0 => {
  title      => 'SVE integer compare vectors',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx op:u=0bx ig2=0b0 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_cmpvec',
};

ICLASS sve_int_cmp_1 => {
  title      => 'SVE integer compare with wide elements',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx U:u=0bx ig2=0b1 lt:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_cmpvec',
};

ICLASS sve_int_count => {
  title      => 'SVE element count',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b10 imm4:u=0bxxxx ig2=0b11100 op:u=0bx pattern:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_countelt',
};

ICLASS sve_int_count_r => {
  title      => 'SVE inc/dec register by predicate count',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op:u=0bx D:u=0bx ig2=0b10001 opc2:u=0bxx Pm:u=0bxxxx Rdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_count_b',
};

ICLASS sve_int_count_r_sat => {
  title      => 'SVE saturating inc/dec register by predicate count',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D:u=0bx U:u=0bx ig2=0b10001 sf:u=0bx op:u=0bx Pm:u=0bxxxx Rdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_count_b',
};

ICLASS sve_int_count_v => {
  title      => 'SVE inc/dec vector by predicate count',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1011 op:u=0bx D:u=0bx ig2=0b10000 opc2:u=0bxx Pm:u=0bxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_count_b',
};

ICLASS sve_int_count_v_sat => {
  title      => 'SVE saturating inc/dec vector by predicate count',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1010 D:u=0bx U:u=0bx ig2=0b10000 opc:u=0bxx Pm:u=0bxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_count_b',
};

ICLASS sve_int_countvlv0 => {
  title      => 'SVE saturating inc/dec vector by element count',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b10 imm4:u=0bxxxx ig2=0b1100 D:u=0bx U:u=0bx pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_countelt',
};

ICLASS sve_int_countvlv1 => {
  title      => 'SVE inc/dec vector by element count',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b11 imm4:u=0bxxxx ig2=0b11000 D:u=0bx pattern:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_countelt',
};

ICLASS sve_int_cterm => {
  title      => 'SVE conditionally terminate scalars',
  diagram    => 'ig0=0b00 ig1=0b100101 op:u=0bx sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b001000 Rn:u=0bxxxxx ne:u=0bx ig4=0b0 ig5=0b0 ig6=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_cmpgpr',
};

ICLASS sve_int_dup_fpimm => {
  title      => 'SVE broadcast floating-point immediate (unpredicated)',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc:u=0bxx ig2=0b111 o2:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_unpred',
};

ICLASS sve_int_dup_fpimm_pred => {
  title      => 'SVE copy floating-point immediate (predicated)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b110 imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_pred',
};

ICLASS sve_int_dup_imm => {
  title      => 'SVE broadcast integer immediate (unpredicated)',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b111 opc:u=0bxx ig2=0b011 sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_unpred',
};

ICLASS sve_int_dup_imm_pred => {
  title      => 'SVE copy integer immediate (predicated)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b01 Pg:u=0bxxxx ig2=0b0 M:u=0bx sh:u=0bx imm8:u=0bxxxxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_wideimm_pred',
};

ICLASS sve_int_dup_mask_imm => {
  title      => 'SVE broadcast bitmask immediate',
  diagram    => 'ig0=0b00000101110000 imm13:u=0bxxxxxxxxxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_maskimm',
};

ICLASS sve_int_index_ii => {
  title      => 'SVE index generation (immediate start, immediate increment)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5b:u=0bxxxxx ig2=0b010000 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_index',
};

ICLASS sve_int_index_ir => {
  title      => 'SVE index generation (immediate start, register increment)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010010 imm5:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_index',
};

ICLASS sve_int_index_ri => {
  title      => 'SVE index generation (register start, immediate increment)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 imm5:u=0bxxxxx ig2=0b010001 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_index',
};

ICLASS sve_int_index_rr => {
  title      => 'SVE index generation (register start, register increment)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b010011 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_index',
};

ICLASS sve_int_log_imm => {
  title      => 'SVE bitwise logical with immediate (unpredicated)',
  diagram    => 'ig0=0b00000101 opc:u=0bxx ig1=0b0000 imm13:u=0bxxxxxxxxxxxxx Zdn:u=0bxxxxx',
  cnsts      => 'opc!=0b11',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_maskimm',
};

ICLASS sve_int_mladdsub_vvv_pred => {
  title      => 'SVE integer multiply-add writing multiplicand (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11 op:u=0bx Pg:u=0bxxx Za:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_muladd_pred',
};

ICLASS sve_int_mlas_vvv_pred => {
  title      => 'SVE integer multiply-accumulate writing addend (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01 op:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_muladd_pred',
};

ICLASS sve_int_movprfx_pred => {
  title      => 'SVE constructive prefix (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc:u=0bxx M:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_red',
};

ICLASS sve_int_mul_b => {
  title      => 'SVE2 integer multiply vectors (unpredicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b0110 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_arit_b',
};

ICLASS sve_int_pcount_pred => {
  title      => 'SVE predicate count',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b100 opc:u=0bxxx ig2=0b10 Pg:u=0bxxxx o2:u=0bx Pn:u=0bxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_perm_bin_long_perm_zz => {
  title      => 'SVE permute vector segments',
  diagram    => 'ig0=0b000001011 op:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b000 opc2:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_perm_bin_perm_pp => {
  title      => 'SVE permute predicate elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10 Pm:u=0bxxxx ig2=0b010 opc:u=0bxx H:u=0bx ig3=0b0 Pn:u=0bxxxx ig4=0b0 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_predicates',
};

ICLASS sve_int_perm_bin_perm_zz => {
  title      => 'SVE permute vector elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 opc:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_perm_clast_rz => {
  title      => 'SVE conditionally extract element to general register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b11000 B:u=0bx ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Rdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_clast_vz => {
  title      => 'SVE conditionally extract element to SIMD&FP scalar',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10101 B:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Vdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_clast_zz => {
  title      => 'SVE conditionally broadcast element to vector',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10100 B:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_compact => {
  title      => 'SVE compress active elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100001100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_cpy_r => {
  title      => 'SVE copy general register to vector (predicated)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101000101 Pg:u=0bxxx Rn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_cpy_v => {
  title      => 'SVE copy SIMD&FP scalar register to vector (predicated)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000100 Pg:u=0bxxx Vn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_dup_i => {
  title      => 'SVE broadcast indexed element',
  diagram    => 'ig0=0b00000101 imm2:u=0bxx ig1=0b1 tsz:u=0bxxxxx ig2=0b001000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_dup_r => {
  title      => 'SVE broadcast general register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100000001110 Rn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_extract_i => {
  title      => 'SVE extract vector (immediate offset, destructive)',
  diagram    => 'ig0=0b00000101001 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_extract',
};

ICLASS sve_int_perm_insrs => {
  title      => 'SVE insert general register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b100100001110 Rm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_insrv => {
  title      => 'SVE insert SIMD&FP scalar register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100001110 Vm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_last_r => {
  title      => 'SVE extract element to general register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10000 B:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_last_v => {
  title      => 'SVE extract element to SIMD&FP scalar register',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b10001 B:u=0bx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_punpk => {
  title      => 'SVE unpack predicate elements',
  diagram    => 'ig0=0b000001010011000 H:u=0bx ig1=0b010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_predicates',
};

ICLASS sve_int_perm_rev => {
  title      => 'SVE reverse within elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1001 opc:u=0bxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_reverse_p => {
  title      => 'SVE reverse predicate elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b110100010000 ig2=0b0 Pn:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_predicates',
};

ICLASS sve_int_perm_reverse_z => {
  title      => 'SVE reverse vector elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b111000001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_splice => {
  title      => 'SVE vector splice (destructive)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101100100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_int_perm_tbl => {
  title      => 'SVE table lookup',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b001100 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_tbl_3src => {
  title      => 'SVE table lookup (three sources)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00101 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_perm_unpk => {
  title      => 'SVE unpack vector elements',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1100 U:u=0bx H:u=0bx ig2=0b001110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_unpred',
};

ICLASS sve_int_pfalse => {
  title      => 'SVE predicate zero',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000111001 ig2=0b000000 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_pfirst => {
  title      => 'SVE predicate first active',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000110000 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_pnext => {
  title      => 'SVE predicate next active',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b011001110001 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pdn:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_pred_log => {
  title      => 'SVE predicate logical operations',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b00 Pm:u=0bxxxx ig2=0b01 Pg:u=0bxxxx o2:u=0bx Pn:u=0bxxxx o3:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_pred_pattern_a => {
  title      => 'SVE inc/dec register by element count',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b11 imm4:u=0bxxxx ig2=0b11100 D:u=0bx pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_countelt',
};

ICLASS sve_int_pred_pattern_b => {
  title      => 'SVE saturating inc/dec register by element count',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 sf:u=0bx imm4:u=0bxxxx ig2=0b1111 D:u=0bx U:u=0bx pattern:u=0bxxxxx Rdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_countelt',
};

ICLASS sve_int_ptest => {
  title      => 'SVE predicate test',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b01000011 Pg:u=0bxxxx ig2=0b0 Pn:u=0bxxxx ig3=0b0 opc2:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_ptrue => {
  title      => 'SVE predicate initialize',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b01100 S:u=0bx ig2=0b111000 pattern:u=0bxxxxx ig3=0b0 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_rdffr => {
  title      => 'SVE predicate read from FFR (predicated)',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011000111100 ig2=0b0 Pg:u=0bxxxx ig3=0b0 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_rdffr_2 => {
  title      => 'SVE predicate read from FFR (unpredicated)',
  diagram    => 'ig0=0b00100101 op:u=0bx S:u=0bx ig1=0b011001111100 ig2=0b000000 Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_gen_d',
};

ICLASS sve_int_read_vl_a => {
  title      => 'SVE stack frame size',
  diagram    => 'ig0=0b000001001 op:u=0bx ig1=0b1 opc2:u=0bxxxxx ig2=0b01010 imm6:u=0bxxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_alloca',
};

ICLASS sve_int_reduce_0 => {
  title      => 'SVE integer add reduction (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b000 opc:u=0bxx U:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_red',
};

ICLASS sve_int_reduce_1 => {
  title      => 'SVE integer min/max reduction (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b001 opc:u=0bxx U:u=0bx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_red',
};

ICLASS sve_int_reduce_2 => {
  title      => 'SVE bitwise logical reduction (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Vd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_red',
};

ICLASS sve_int_rotate_imm => {
  title      => 'sve_int_rotate_imm',
  diagram    => 'ig0=0b00000100 tszh:u=0bxx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b001101 Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_logical',
};

ICLASS sve_int_scmp_vi => {
  title      => 'SVE integer compare with signed immediate',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b0 imm5:u=0bxxxxx op:u=0bx ig2=0b0 o2:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_sel_vvv => {
  title      => 'SVE select vector elements (predicated)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b11 Pg:u=0bxxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_setffr => {
  title      => 'SVE FFR initialise',
  diagram    => 'ig0=0b00 ig1=0b100101 opc:u=0bxx ig2=0b10 ig3=0b1100100100 ig4=0b000000 ig5=0b0 ig6=0b0 ig7=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_wrffr',
};

ICLASS sve_int_sqdmulh => {
  title      => 'SVE2 signed saturating doubling multiply high (unpredicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b01110 R:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_arit_b',
};

ICLASS sve_int_tern_log => {
  title      => 'SVE2 bitwise ternary operations',
  diagram    => 'ig0=0b00000100 opc:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b00111 o2:u=0bx Zk:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_unpred_logical',
};

ICLASS sve_int_ucmp_vi => {
  title      => 'SVE integer compare with unsigned immediate',
  diagram    => 'ig0=0b00100100 size:u=0bxx ig1=0b1 imm7:u=0bxxxxxxx lt:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx ne:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_int_un_pred_arit_0 => {
  title      => 'SVE integer unary operations (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b010 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_un',
};

ICLASS sve_int_un_pred_arit_1 => {
  title      => 'SVE bitwise unary operations (predicated)',
  diagram    => 'ig0=0b00000100 size:u=0bxx ig1=0b011 opc:u=0bxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_int_pred_un',
};

ICLASS sve_int_while_rr => {
  title      => 'SVE integer compare scalar count and limit',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b000 sf:u=0bx U:u=0bx lt:u=0bx Rn:u=0bxxxxx eq:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_cmpgpr',
};

ICLASS sve_int_whilenc => {
  title      => 'SVE pointer conflict compare',
  diagram    => 'ig0=0b00100101 size:u=0bxx ig1=0b1 Rm:u=0bxxxxx ig2=0b001100 Rn:u=0bxxxxx rw:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_cmpgpr',
};

ICLASS sve_int_wrffr => {
  title      => 'SVE FFR write from predicate',
  diagram    => 'ig0=0b00 ig1=0b100101 opc:u=0bxx ig2=0b10 ig3=0b1000100100 ig4=0b0 Pn:u=0bxxxx ig5=0b0 ig6=0b0 ig7=0b0 ig8=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_pred_wrffr',
};

ICLASS sve_intx_aba => {
  title      => 'SVE2 integer absolute difference and accumulate',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11111 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_intx_aba_long => {
  title      => 'SVE2 integer absolute difference and accumulate long',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1100 U:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_intx_accumulate_long_pairs => {
  title      => 'SVE2 integer pairwise add and accumulate long',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00010 U:u=0bx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_adc_long => {
  title      => 'SVE2 integer add/subtract long with carry',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b11010 T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_intx_arith_binary_pairs => {
  title      => 'SVE2 integer pairwise arithmetic',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 opc:u=0bxx U:u=0bx ig2=0b101 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_arith_narrow => {
  title      => 'SVE2 integer add/subtract narrow high part',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b011 S:u=0bx R:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_narrowing',
};

ICLASS sve_intx_bin_pred_shift_sat_round => {
  title      => 'SVE2 saturating/rounding bitwise shift left (predicated)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q:u=0bx R:u=0bx N:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_cadd => {
  title      => 'SVE2 complex integer add',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b00000 op:u=0bx ig2=0b11011 rot:u=0bx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_intx_cdot => {
  title      => 'SVE2 complex integer dot product',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0001 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_cdot_by_indexed_elem => {
  title      => 'SVE2 complex integer dot product (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0100 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_clong => {
  title      => 'SVE2 integer add/subtract interleaved long',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1000 S:u=0bx tb:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_constructive',
};

ICLASS sve_intx_cmla => {
  title      => 'SVE2 complex integer multiply-add',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b001 op:u=0bx rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_cmla_by_indexed_elem => {
  title      => 'SVE2 complex integer multiply-add (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0110 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_cons_arith_long => {
  title      => 'SVE2 integer add/subtract long',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00 op:u=0bx S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_cons_widening',
};

ICLASS sve_intx_cons_arith_wide => {
  title      => 'SVE2 integer add/subtract wide',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_cons_widening',
};

ICLASS sve_intx_cons_mul_long => {
  title      => 'SVE2 integer multiply long',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011 op:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_cons_widening',
};

ICLASS sve_intx_dot => {
  title      => 'SVE integer dot product (unpredicated)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00000 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_dot_by_indexed_elem => {
  title      => 'SVE integer dot product (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00000 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_eorx => {
  title      => 'SVE2 bitwise exclusive-or interleaved',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b10010 tb:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_constructive',
};

ICLASS sve_intx_extract_narrow => {
  title      => 'SVE2 saturating extract narrow',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx ig2=0b000010 opc:u=0bxx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_narrowing',
};

ICLASS sve_intx_histcnt => {
  title      => 'SVE2 histogram generation (vector)',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_intx_histseg => {
  title      => 'SVE2 histogram generation  (segment)',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b101000 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_histseg',
};

ICLASS sve_intx_match => {
  title      => 'SVE2 character match',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b1 Zm:u=0bxxxxx ig2=0b100 Pg:u=0bxxx Zn:u=0bxxxxx op:u=0bx Pd:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

ICLASS sve_intx_mixed_dot => {
  title      => 'SVE mixed sign dot product',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b011110 Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_mixed_dot_by_indexed_elem => {
  title      => 'SVE mixed sign dot product (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00011 U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_mla_by_indexed_elem => {
  title      => 'SVE2 integer multiply-add (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00001 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_mla_long_by_indexed_elem => {
  title      => 'SVE2 integer multiply-add long (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b10 S:u=0bx U:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_mlal_long => {
  title      => 'SVE2 integer multiply-add long',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b010 S:u=0bx U:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_mmla => {
  title      => 'SVE integer matrix multiply accumulate',
  diagram    => 'ig0=0b01000101 uns:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b100110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_constructive',
};

ICLASS sve_intx_mul_by_indexed_elem => {
  title      => 'SVE2 integer multiply (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b111110 Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_mul_long_by_indexed_elem => {
  title      => 'SVE2 integer multiply long (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b110 U:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_perm_bit => {
  title      => 'SVE2 bitwise permute',
  diagram    => 'ig0=0b01000101 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b1011 opc:u=0bxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_constructive',
};

ICLASS sve_intx_perm_extract_i => {
  title      => 'SVE2 extract vector (immediate offset, constructive)',
  diagram    => 'ig0=0b00000101011 imm8h:u=0bxxxxx ig1=0b000 imm8l:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_extract',
};

ICLASS sve_intx_perm_splice => {
  title      => 'SVE2 vector splice (constructive)',
  diagram    => 'ig0=0b00000101 size:u=0bxx ig1=0b101101100 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_perm_pred',
};

ICLASS sve_intx_pred_arith_binary => {
  title      => 'SVE2 integer halving add/subtract (predicated)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b010 R:u=0bx S:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_pred_arith_binary_sat => {
  title      => 'SVE2 saturating add/subtract',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b011 op:u=0bx S:u=0bx U:u=0bx ig2=0b100 Pg:u=0bxxx Zm:u=0bxxxxx Zdn:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_pred_arith_unary => {
  title      => 'SVE2 integer unary operations (predicated)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b00 Q:u=0bx ig2=0b0 opc:u=0bxx ig3=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_predicated',
};

ICLASS sve_intx_qdmla_long_by_indexed_elem => {
  title      => 'SVE2 saturating multiply-add (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b001 S:u=0bx il:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_qdmlal_long => {
  title      => 'SVE2 saturating  multiply-add long',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b0110 S:u=0bx T:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_qdmlalbt => {
  title      => 'SVE2 saturating multiply-add interleaved long',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b00001 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_qdmul_long_by_indexed_elem => {
  title      => 'SVE2 saturating multiply (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b1110 il:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_qdmulh_by_indexed_elem => {
  title      => 'SVE2 saturating  multiply high (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b11110 R:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_qrdcmla_by_indexed_elem => {
  title      => 'SVE2 complex saturating multiply-add (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b0111 rot:u=0bxx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_qrdmlah => {
  title      => 'SVE2 saturating multiply-add high',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b0 Zm:u=0bxxxxx ig2=0b01110 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_muladd_unpred',
};

ICLASS sve_intx_qrdmlah_by_indexed_elem => {
  title      => 'SVE2 saturating multiply-add high (indexed)',
  diagram    => 'ig0=0b01000100 size:u=0bxx ig1=0b1 opc:u=0bxxxxx ig2=0b00010 S:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_by_indexed_elem',
};

ICLASS sve_intx_shift_insert => {
  title      => 'SVE2 bitwise shift and insert',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b11110 op:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_intx_shift_long => {
  title      => 'SVE2 bitwise shift left long',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1010 U:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_constructive',
};

ICLASS sve_intx_shift_narrow => {
  title      => 'SVE2 bitwise shift right narrow',
  diagram    => 'ig0=0b010001010 tszh:u=0bx ig1=0b1 tszl:u=0bxx imm3:u=0bxxx ig2=0b00 op:u=0bx U:u=0bx R:u=0bx T:u=0bx Zn:u=0bxxxxx Zd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_narrowing',
};

ICLASS sve_intx_sra => {
  title      => 'SVE2 bitwise shift right and accumulate',
  diagram    => 'ig0=0b01000101 tszh:u=0bxx ig1=0b0 tszl:u=0bxx imm3:u=0bxxx ig2=0b1110 R:u=0bx U:u=0bx Zn:u=0bxxxxx Zda:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_intx_acc',
};

ICLASS sve_mem_32b_fill => {
  title      => 'SVE load vector register',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_gld_sv_a => {
  title      => 'SVE 32-bit gather load halfwords (scalar plus 32-bit scaled offsets)',
  diagram    => 'ig0=0b100001001 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_gld_sv_b => {
  title      => 'SVE 32-bit gather load words (scalar plus 32-bit scaled offsets)',
  diagram    => 'ig0=0b100001010 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_gld_vi => {
  title      => 'SVE 32-bit gather load (vector plus immediate)',
  diagram    => 'ig0=0b1000010 msz:u=0bxx ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_gld_vs => {
  title      => 'SVE 32-bit gather load (scalar plus 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1000010 opc:u=0bxx xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b11',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_gldnt_vs => {
  title      => 'SVE2 32-bit gather non-temporal load (scalar plus 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1000010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b10 U:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_pfill => {
  title      => 'SVE load predicate register',
  diagram    => 'ig0=0b1000010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_prfm_sv => {
  title      => 'SVE 32-bit gather prefetch (scalar plus 32-bit scaled offsets)',
  diagram    => 'ig0=0b100001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_32b_prfm_vi => {
  title      => 'SVE 32-bit gather prefetch (vector plus immediate)',
  diagram    => 'ig0=0b1000010 msz:u=0bxx ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_64b_gld_sv => {
  title      => 'SVE 64-bit gather load (scalar plus 32-bit unpacked scaled offsets)',
  diagram    => 'ig0=0b1100010 opc:u=0bxx xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_gld_sv2 => {
  title      => 'SVE 64-bit gather load (scalar plus 64-bit scaled offsets)',
  diagram    => 'ig0=0b1100010 opc:u=0bxx ig1=0b11 Zm:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_gld_vi => {
  title      => 'SVE 64-bit gather load (vector plus immediate)',
  diagram    => 'ig0=0b1100010 msz:u=0bxx ig1=0b01 imm5:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_gld_vs => {
  title      => 'SVE 64-bit gather load (scalar plus unpacked 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1100010 msz:u=0bxx xs:u=0bx ig1=0b0 Zm:u=0bxxxxx ig2=0b0 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_gld_vs2 => {
  title      => 'SVE 64-bit gather load (scalar plus 64-bit unscaled offsets)',
  diagram    => 'ig0=0b1100010 msz:u=0bxx ig1=0b10 Zm:u=0bxxxxx ig2=0b1 U:u=0bx ff:u=0bx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_gldnt_vs => {
  title      => 'SVE2 64-bit gather non-temporal load (scalar plus unpacked 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1100010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b1 U:u=0bx ig3=0b0 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_prfm_sv => {
  title      => 'SVE 64-bit gather prefetch (scalar plus unpacked 32-bit scaled offsets)',
  diagram    => 'ig0=0b110001000 xs:u=0bx ig1=0b1 Zm:u=0bxxxxx ig2=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_prfm_sv2 => {
  title      => 'SVE 64-bit gather prefetch (scalar plus 64-bit scaled offsets)',
  diagram    => 'ig0=0b11000100011 Zm:u=0bxxxxx ig1=0b1 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_64b_prfm_vi => {
  title      => 'SVE 64-bit gather prefetch (vector plus immediate)',
  diagram    => 'ig0=0b1100010 msz:u=0bxx ig1=0b00 imm5:u=0bxxxxx ig2=0b111 Pg:u=0bxxx Zn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem64',
};

ICLASS sve_mem_cld_si => {
  title      => 'SVE contiguous load (scalar plus immediate)',
  diagram    => 'ig0=0b1010010 dtype:u=0bxxxx ig1=0b0 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cld_ss => {
  title      => 'SVE contiguous load (scalar plus scalar)',
  diagram    => 'ig0=0b1010010 dtype:u=0bxxxx Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cldff_ss => {
  title      => 'SVE contiguous first-fault load (scalar plus scalar)',
  diagram    => 'ig0=0b1010010 dtype:u=0bxxxx Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cldnf_si => {
  title      => 'SVE contiguous non-fault load (scalar plus immediate)',
  diagram    => 'ig0=0b1010010 dtype:u=0bxxxx ig1=0b1 imm4:u=0bxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cldnt_si => {
  title      => 'SVE contiguous non-temporal load (scalar plus immediate)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx ig1=0b000 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cldnt_ss => {
  title      => 'SVE contiguous non-temporal load (scalar plus scalar)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_cst_si => {
  title      => 'SVE contiguous store (scalar plus immediate)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx size:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_si',
};

ICLASS sve_mem_cst_ss => {
  title      => 'SVE contiguous store (scalar plus scalar)',
  diagram    => 'ig0=0b1110010 opc:u=0bxxx o2:u=0bx Rm:u=0bxxxxx ig1=0b010 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b110',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs',
};

ICLASS sve_mem_cstnt_si => {
  title      => 'SVE contiguous non-temporal store (scalar plus immediate)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b001 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_si',
};

ICLASS sve_mem_cstnt_ss => {
  title      => 'SVE contiguous non-temporal store (scalar plus scalar)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_nt',
};

ICLASS sve_mem_eld_si => {
  title      => 'SVE load multiple structures (scalar plus immediate)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx opc:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_eld_ss => {
  title      => 'SVE load multiple structures (scalar plus scalar)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx opc:u=0bxx Rm:u=0bxxxxx ig1=0b110 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_est_si => {
  title      => 'SVE store multiple structures (scalar plus immediate)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx opc:u=0bxx ig1=0b1 imm4:u=0bxxxx ig2=0b111 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_si',
};

ICLASS sve_mem_est_ss => {
  title      => 'SVE store multiple structures (scalar plus scalar)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx opc:u=0bxx Rm:u=0bxxxxx ig1=0b011 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  cnsts      => 'opc!=0b00',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_nt',
};

ICLASS sve_mem_ld_dup => {
  title      => 'SVE load and broadcast element',
  diagram    => 'ig0=0b1000010 dtypeh:u=0bxx ig1=0b1 imm6:u=0bxxxxxx ig2=0b1 dtypel:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_ldqr_si => {
  title      => 'SVE load and broadcast quadword (scalar plus immediate)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx ssz:u=0bxx ig1=0b0 imm4:u=0bxxxx ig2=0b001 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_ldqr_ss => {
  title      => 'SVE load and broadcast quadword (scalar plus scalar)',
  diagram    => 'ig0=0b1010010 msz:u=0bxx ssz:u=0bxx Rm:u=0bxxxxx ig1=0b000 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memcld',
};

ICLASS sve_mem_prfm_si => {
  title      => 'SVE contiguous prefetch (scalar plus immediate)',
  diagram    => 'ig0=0b1000010111 imm6:u=0bxxxxxx ig1=0b0 msz:u=0bxx Pg:u=0bxxx Rn:u=0bxxxxx ig2=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_prfm_ss => {
  title      => 'SVE contiguous prefetch (scalar plus scalar)',
  diagram    => 'ig0=0b1000010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b110 Pg:u=0bxxx Rn:u=0bxxxxx ig3=0b0 prfop:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_mem32',
};

ICLASS sve_mem_pspill => {
  title      => 'SVE store predicate register',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b000 imm9l:u=0bxxx Rn:u=0bxxxxx ig2=0b0 Pt:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs',
};

ICLASS sve_mem_spill => {
  title      => 'SVE store vector register',
  diagram    => 'ig0=0b1110010110 imm9h:u=0bxxxxxx ig1=0b010 imm9l:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_cs',
};

ICLASS sve_mem_sst_sv2 => {
  title      => 'SVE 64-bit scatter store (scalar plus 64-bit scaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b01 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss2',
};

ICLASS sve_mem_sst_sv_a => {
  title      => 'SVE 64-bit scatter store (scalar plus unpacked 32-bit scaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b01 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss',
};

ICLASS sve_mem_sst_sv_b => {
  title      => 'SVE 32-bit scatter store (scalar plus 32-bit scaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b11 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss',
};

ICLASS sve_mem_sst_vi_a => {
  title      => 'SVE 64-bit scatter store (vector plus immediate)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b10 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss2',
};

ICLASS sve_mem_sst_vi_b => {
  title      => 'SVE 32-bit scatter store (vector plus immediate)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b11 imm5:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss2',
};

ICLASS sve_mem_sst_vs2 => {
  title      => 'SVE 64-bit scatter store (scalar plus 64-bit unscaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b00 Zm:u=0bxxxxx ig2=0b101 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss2',
};

ICLASS sve_mem_sst_vs_a => {
  title      => 'SVE 64-bit scatter store (scalar plus unpacked 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b00 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss',
};

ICLASS sve_mem_sst_vs_b => {
  title      => 'SVE 32-bit scatter store (scalar plus 32-bit unscaled offsets)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b10 Zm:u=0bxxxxx ig2=0b1 xs:u=0bx ig3=0b0 Pg:u=0bxxx Rn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_ss',
};

ICLASS sve_mem_sstnt_32b_vs => {
  title      => 'SVE2 32-bit scatter non-temporal store (vector plus scalar)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b10 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_nt',
};

ICLASS sve_mem_sstnt_64b_vs => {
  title      => 'SVE2 64-bit scatter non-temporal store (vector plus scalar)',
  diagram    => 'ig0=0b1110010 msz:u=0bxx ig1=0b00 Rm:u=0bxxxxx ig2=0b001 Pg:u=0bxxx Zn:u=0bxxxxx Zt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve_memst_nt',
};


1;

