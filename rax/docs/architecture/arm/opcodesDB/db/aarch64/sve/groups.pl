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

GROUP sve => {
  title      => 'SVE encodings',
  diagram    => 'op0:u=0bxxx ig0=0b0010 op1:u=0bxx ig1=0bx op2:u=0bxxxxx ig2=0bx op3:u=0bxxxxxx ig3=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
};

GROUP sve_alloca => {
  title      => 'SVE Stack Allocation',
  diagram    => 'ig0=0b00000100 op0:u=0bx ig1=0bx ig2=0b1 ig3=0bxxxxx ig4=0b0101 op1:u=0bx ig5=0bxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_cmpgpr => {
  title      => 'SVE Integer Compare - Scalars',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b00 op0:u=0bxx op1:u=0bxx ig5=0bxxxxxx op2:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_cmpvec => {
  title      => 'SVE Integer Compare - Vectors',
  diagram    => 'ig0=0b00100100 ig1=0bxx ig2=0b0 ig3=0bxxxxxx op0:u=0bx ig4=0bxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_countelt => {
  title      => 'SVE Element Count',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 op0:u=0bx ig3=0bxxxx ig4=0b11 op1:u=0bxxx ig5=0bxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_cmpzero => {
  title      => 'SVE Floating Point Compare - with Zero',
  diagram    => 'ig0=0b01100101 ig1=0bxx ig2=0b010 op0:u=0bx ig3=0bxx ig4=0b001 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_fma => {
  title      => 'SVE Floating Point Multiply-Add',
  diagram    => 'ig0=0b01100101 ig1=0bxx ig2=0b1 ig3=0bxxxxx op0:u=0bx ig4=0bxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_fma_long => {
  title      => 'SVE Floating Point Widening Multiply-Add',
  diagram    => 'ig0=0b01100100 op0:u=0bx ig1=0bx ig2=0b1 ig3=0bxxxxx ig4=0b10 op1:u=0bx ig5=0b00 op2:u=0bx ig6=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_fma_long_by_indexed_elem => {
  title      => 'SVE Floating Point Widening Multiply-Add - Indexed',
  diagram    => 'ig0=0b01100100 op0:u=0bx ig1=0bx ig2=0b1 ig3=0bxxxxx ig4=0b01 op1:u=0bx ig5=0b0 op2:u=0bxx ig6=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_pred => {
  title      => 'SVE Floating Point Arithmetic - Predicated',
  diagram    => 'ig0=0b01100101 ig1=0bxx ig2=0b0 op0:u=0bxx ig3=0bxxx ig4=0b100 op1:u=0bxxx op2:u=0bxxxx ig5=0bxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_unary => {
  title      => 'SVE Floating Point Unary Operations - Predicated',
  diagram    => 'ig0=0b01100101 ig1=0bxx ig2=0b0 op0:u=0bxxx ig3=0bxx ig4=0b101 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_fp_unary_unpred => {
  title      => 'SVE Floating Point Unary Operations - Unpredicated',
  diagram    => 'ig0=0b01100101 ig1=0bxx ig2=0b001 ig3=0bxxx ig4=0b0011 op0:u=0bxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_index => {
  title      => 'SVE Index Generation',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b0100 op0:u=0bxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_muladd_pred => {
  title      => 'SVE Integer Multiply-Add - Predicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b0 ig3=0bxxxxx op0:u=0bx ig4=0b1 ig5=0bxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_pred_bin => {
  title      => 'SVE Integer Binary Arithmetic - Predicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b0 op0:u=0bxxx ig3=0bxx ig4=0b000 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_pred_red => {
  title      => 'SVE Integer Reduction',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b0 op0:u=0bxx ig3=0bxxx ig4=0b001 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_pred_shift => {
  title      => 'SVE Bitwise Shift - Predicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b0 op0:u=0bxx ig3=0bxxx ig4=0b100 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_pred_un => {
  title      => 'SVE Integer Unary Arithmetic - Predicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b0 op0:u=0bxx ig3=0bxxx ig4=0b101 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_unpred_arit_b => {
  title      => 'SVE2 Integer Multiply - Unpredicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b011 op0:u=0bxx ig5=0bxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_unpred_logical => {
  title      => 'SVE Bitwise Logical - Unpredicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b001 op0:u=0bxxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_unpred_misc => {
  title      => 'SVE Integer Misc - Unpredicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b1011 op0:u=0bxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_int_unpred_shift => {
  title      => 'SVE Bitwise Shift - Unpredicated',
  diagram    => 'ig0=0b00000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b100 op0:u=0bx ig5=0bxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_acc => {
  title      => 'SVE2 Accumulate',
  diagram    => 'ig0=0b01000101 ig1=0bxx ig2=0b0 op0:u=0bxxxx ig3=0bx ig4=0b11 op1:u=0bxxx ig5=0bxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_by_indexed_elem => {
  title      => 'SVE Multiply - Indexed',
  diagram    => 'ig0=0b01000100 ig1=0bxx ig2=0b1 ig3=0bxxxxx op0:u=0bxxxxxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_cons_widening => {
  title      => 'SVE2 Widening Integer Arithmetic',
  diagram    => 'ig0=0b01000101 ig1=0bxx ig2=0b0 ig3=0bxxxxx ig4=0b0 op0:u=0bxx ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_constructive => {
  title      => 'SVE Misc',
  diagram    => 'ig0=0b01000101 op0:u=0bx ig1=0bx ig2=0b0 ig3=0bxxxxx ig4=0b10 op1:u=0bxxxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_crypto => {
  title      => 'SVE2 Crypto Extensions',
  diagram    => 'ig0=0b01000101 ig1=0bxx ig2=0b1 op0:u=0bxxx op1:u=0bxx ig3=0b111 op2:u=0bxx ig4=0bx op3:u=0bxxxxx ig5=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_histseg => {
  title      => 'SVE2 Histogram Computation - Segment',
  diagram    => 'ig0=0b01000101 ig1=0bxx ig2=0b1 ig3=0bxxxxx ig4=0b101 op0:u=0bxxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_muladd_unpred => {
  title      => 'SVE Integer Multiply-Add - Unpredicated',
  diagram    => 'ig0=0b01000100 ig1=0bxx ig2=0b0 ig3=0bxxxxx ig4=0b0 op0:u=0bxxxxx ig5=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_narrowing => {
  title      => 'SVE2 Narrowing',
  diagram    => 'ig0=0b01000101 op0:u=0bx ig1=0bx ig2=0b1 ig3=0bxx op1:u=0bxxx ig4=0b0 op2:u=0bxx ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_intx_predicated => {
  title      => 'SVE2 Integer - Predicated',
  diagram    => 'ig0=0b01000100 ig1=0bxx ig2=0b0 op0:u=0bxxxx ig3=0bx ig4=0b10 op1:u=0bx ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_maskimm => {
  title      => 'SVE Bitwise Immediate',
  diagram    => 'ig0=0b00000101 op0:u=0bxx ig1=0b00 op1:u=0bxx ig2=0bxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_mem32 => {
  title      => 'SVE Memory - 32-bit Gather and Unsized Contiguous',
  diagram    => 'ig0=0b1000010 op0:u=0bxx op1:u=0bxx ig1=0bxxxxx op2:u=0bxxx ig2=0bxxxxxxxx op3:u=0bx ig3=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_mem64 => {
  title      => 'SVE Memory - 64-bit Gather',
  diagram    => 'ig0=0b1100010 op0:u=0bxx op1:u=0bxx ig1=0bxxxxx op2:u=0bxxx ig2=0bxxxxxxxx op3:u=0bx ig3=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memcld => {
  title      => 'SVE Memory - Contiguous Load',
  diagram    => 'ig0=0b1010010 ig1=0bxx op0:u=0bxx op1:u=0bx ig2=0bxxxx op2:u=0bxxx ig3=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memst_cs => {
  title      => 'SVE Memory - Contiguous Store and Unsized Contiguous',
  diagram    => 'ig0=0b1110010 op0:u=0bxxx ig1=0bxxxxxx ig2=0b0 op1:u=0bx ig3=0b0 ig4=0bxxxxxxxx op2:u=0bx ig5=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memst_nt => {
  title      => 'SVE Memory - Non-temporal and Multi-register Store',
  diagram    => 'ig0=0b1110010 ig1=0bxx op0:u=0bxx ig2=0bxxxxx ig3=0b0 op1:u=0bx ig4=0b1 ig5=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memst_si => {
  title      => 'SVE Memory - Contiguous Store with Immediate Offset',
  diagram    => 'ig0=0b1110010 ig1=0bxx op0:u=0bxx op1:u=0bx ig2=0bxxxx ig3=0b111 ig4=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memst_ss => {
  title      => 'SVE Memory - Scatter with Optional Sign Extend',
  diagram    => 'ig0=0b1110010 ig1=0bxx op0:u=0bxx ig2=0bxxxxx ig3=0b1 ig4=0bx ig5=0b0 ig6=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_memst_ss2 => {
  title      => 'SVE Memory - Scatter',
  diagram    => 'ig0=0b1110010 ig1=0bxx op0:u=0bxx ig2=0bxxxxx ig3=0b101 ig4=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_perm_extract => {
  title      => 'SVE Permute Vector - Extract',
  diagram    => 'ig0=0b000001010 op0:u=0bx ig1=0b1 ig2=0bxxxxx ig3=0b000 ig4=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_perm_pred => {
  title      => 'SVE Permute Vector - Predicated',
  diagram    => 'ig0=0b00000101 ig1=0bxx ig2=0b1 op0:u=0bx op1:u=0bxxx op2:u=0bx ig3=0b10 op3:u=0bx ig4=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_perm_predicates => {
  title      => 'SVE Permute Predicate',
  diagram    => 'ig0=0b00000101 op0:u=0bxx ig1=0b1 op1:u=0bxxxxx ig2=0b010 op2:u=0bxxxx ig3=0bxxxx op3:u=0bx ig4=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_perm_unpred => {
  title      => 'SVE Permute Vector - Unpredicated',
  diagram    => 'ig0=0b00000101 ig1=0bxx ig2=0b1 op0:u=0bxx op1:u=0bxx op2:u=0bx ig3=0b001 op3:u=0bxxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_pred_count_b => {
  title      => 'SVE Inc/Dec by Predicate Count',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b101 op0:u=0bx ig3=0bxx ig4=0b1000 op1:u=0bx ig5=0bxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_pred_gen_b => {
  title      => 'SVE Propagate Break',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b00 ig3=0bxxxx ig4=0b11 ig5=0bxxxx op0:u=0bx ig6=0bxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_pred_gen_c => {
  title      => 'SVE Partition Break',
  diagram    => 'ig0=0b00100101 op0:u=0bx ig1=0bx ig2=0b01 op1:u=0bxxxx ig3=0b01 ig4=0bxxxx op2:u=0bx ig5=0bxxxx op3:u=0bx ig6=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_pred_gen_d => {
  title      => 'SVE Predicate Misc',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b01 op0:u=0bxxxx ig3=0b11 op1:u=0bxxx op2:u=0bxx op3:u=0bxxxx op4:u=0bx ig4=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_pred_wrffr => {
  title      => 'SVE Write FFR',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b101 op0:u=0bx op1:u=0bxx ig3=0b1001 op2:u=0bxxx op3:u=0bxxxx op4:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_wideimm_pred => {
  title      => 'SVE Integer Wide Immediate - Predicated',
  diagram    => 'ig0=0b00000101 ig1=0bxx ig2=0b01 ig3=0bxxxx op0:u=0bxxx ig4=0bxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};

GROUP sve_wideimm_unpred => {
  title      => 'SVE Integer Wide Immediate - Unpredicated',
  diagram    => 'ig0=0b00100101 ig1=0bxx ig2=0b1 op0:u=0bxx ig3=0bxx op1:u=0bx ig4=0b11 ig5=0bxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=SVE',
  tags       => 'group=sve',
};


1;

