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

ICLASS asimldall => {
  title      => 'Advanced SIMD load single structure to all lanes',
  diagram    => 'ig0=0b111110011 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N:u=0bxx size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst mapto=ldv_ssall',
};

ICLASS asimldstms => {
  title      => 'Advanced SIMD load/store multiple structures',
  diagram    => 'ig0=0b111110010 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0bxxxx size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst mapto=ldstv_ms',
};

ICLASS asimldstss => {
  title      => 'Advanced SIMD load/store single structure to one lane',
  diagram    => 'ig0=0b111110011 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0bxx N:u=0bxx index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst mapto=ldstv_ssone',
};

ICLASS fp_2r => {
  title      => 'Floating-point data-processing (two registers)',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1:u=0bx opc2:u=0bxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpdp2reg',
};

ICLASS fp_3r => {
  title      => 'Floating-point data-processing (three registers)',
  diagram    => 'ig0=0b11101110 o0:u=0bx D:u=0bx o1:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpdp3reg',
};

ICLASS fp_csel => {
  title      => 'Floating-point conditional select',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpcsel',
};

ICLASS fp_extins => {
  title      => 'Floating-point extraction and insertion',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fp_minmax => {
  title      => 'Floating-point minNum/maxNum',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpminmaxnm',
};

ICLASS fp_mov32 => {
  title      => 'Floating-point 32-bit move',
  diagram    => 'ig0=0b11101110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 mapto=movfpgp32',
};

ICLASS fp_movi => {
  title      => 'Floating-point move immediate',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpimm',
};

ICLASS fp_msr => {
  title      => 'Floating-point move special register',
  diagram    => 'ig0=0b11101110111 L:u=0bx reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 mapto=movfpsr',
};

ICLASS fp_toint => {
  title      => 'Floating-point directed convert to integer',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1:u=0bx RM:u=0bxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp mapto=fpcvtrnd',
};

ICLASS fpcsel => {
  title      => 'Floating-point conditional select',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpcvtrnd => {
  title      => 'Floating-point directed convert to integer',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1:u=0bx RM:u=0bxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpdp2reg => {
  title      => 'Floating-point data-processing (two registers)',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1:u=0bx opc2:u=0bxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpdp3reg => {
  title      => 'Floating-point data-processing (three registers)',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0:u=0bx D:u=0bx o1:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpextins => {
  title      => 'Floating-point extraction and insertion',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpimm => {
  title      => 'Floating-point move immediate',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS fpminmaxnm => {
  title      => 'Floating-point minNum/maxNum',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp',
};

ICLASS ldstsimdfp => {
  title      => 'Advanced SIMD and floating-point load/store',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64',
};

ICLASS ldstv_ms => {
  title      => 'Advanced SIMD load/store multiple structures',
  diagram    => 'ig0=0b111101000 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0bxxxx size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls',
};

ICLASS ldstv_ssone => {
  title      => 'Advanced SIMD load/store single structure to one lane',
  diagram    => 'ig0=0b111101001 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0bxx N:u=0bxx index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls',
};

ICLASS ldv_ssall => {
  title      => 'Advanced SIMD load single structure to all lanes',
  diagram    => 'ig0=0b111101001 D:u=0bx L:u=0bx ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N:u=0bxx size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls',
};

ICLASS movfpgp16 => {
  title      => 'Floating-point 16-bit move',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32',
};

ICLASS movfpgp32 => {
  title      => 'Floating-point 32-bit move',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32',
};

ICLASS movfpsr => {
  title      => 'Floating-point move special register',
  diagram    => 'cond:u=0bxxxx ig0=0b1110111 L:u=0bx reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32',
};

ICLASS movsimdfpgp64 => {
  title      => 'Advanced SIMD and floating-point 64-bit move',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D:u=0bx ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size:u=0bxx opc2:u=0bxx M:u=0bx o3:u=0bx Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64',
};

ICLASS movsimdgp => {
  title      => 'Advanced SIMD 8/16/32-bit element move/duplicate',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32',
};

ICLASS p_cpaf => {
  title      => 'System register access, Advanced SIMD, and floating-point',
  diagram    => 'ig0=0b111 op0:u=0bx ig1=0b11 op1:u=0bxx ig2=0bxxxxxxxxxxxx op2:u=0bxxx ig3=0bxxxx op3:u=0bx ig4=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf',
};

ICLASS simd1reg_imm => {
  title      => 'Advanced SIMD one register and modified immediate',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0bxxxx ig3=0b0 Q:u=0bx op:u=0bx ig4=0b1 imm4:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg',
};

ICLASS simd2reg_dup => {
  title      => 'Advanced SIMD duplicate (scalar)',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc:u=0bxxx Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd2reg_misc => {
  title      => 'Advanced SIMD two registers misc',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1:u=0bxx Vd:u=0bxxxx ig2=0b0 opc2:u=0bxxxx Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd2reg_scalar => {
  title      => 'Advanced SIMD two registers and a scalar',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd2reg_scalarext => {
  title      => 'Advanced SIMD two registers and a scalar extension',
  diagram    => 'ig0=0b11111110 op1:u=0bx D:u=0bx op2:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3:u=0bx ig2=0b0 op4:u=0bx N:u=0bx Q:u=0bx M:u=0bx U:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=cops_as',
};

ICLASS simd2reg_shift => {
  title      => 'Advanced SIMD two registers and shift amount',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm3H:u=0bxxx imm3L:u=0bxxx Vd:u=0bxxxx opc:u=0bxxxx L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg',
};

ICLASS simd3reg_diff => {
  title      => 'Advanced SIMD three registers of different lengths',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd3reg_ext => {
  title      => 'Advanced SIMD vector extract',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd3reg_same => {
  title      => 'Advanced SIMD three registers of the same length',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx o1:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp',
};

ICLASS simd3reg_sameext => {
  title      => 'Advanced SIMD three registers of the same length extension',
  diagram    => 'ig0=0b1111110 op1:u=0bxx D:u=0bx op2:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3:u=0bx ig2=0b0 op4:u=0bx N:u=0bx Q:u=0bx M:u=0bx U:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=cops_as',
};

ICLASS simd3reg_tbl => {
  title      => 'Advanced SIMD table permute',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg',
};

ICLASS simd_1r_imm => {
  title      => 'Advanced SIMD one register and modified immediate',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0bxxxx ig3=0b0 Q:u=0bx op:u=0bx ig4=0b1 imm4:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg mapto=simd1reg_imm',
};

ICLASS simd_2r_misc => {
  title      => 'Advanced SIMD two registers misc',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1:u=0bxx Vd:u=0bxxxx ig2=0b0 opc2:u=0bxxxx Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd2reg_misc',
};

ICLASS simd_2r_sc => {
  title      => 'Advanced SIMD two registers and a scalar',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd2reg_scalar',
};

ICLASS simd_2r_scext => {
  title      => 'Advanced SIMD two registers and a scalar extension',
  diagram    => 'ig0=0b11111110 op1:u=0bx D:u=0bx op2:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3:u=0bx ig2=0b0 op4:u=0bx N:u=0bx Q:u=0bx M:u=0bx U:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=simd2reg_scalarext',
};

ICLASS simd_2r_shift => {
  title      => 'Advanced SIMD two registers and shift amount',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm3H:u=0bxxx imm3L:u=0bxxx Vd:u=0bxxxx opc:u=0bxxxx L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg mapto=simd2reg_shift',
};

ICLASS simd_3diff => {
  title      => 'Advanced SIMD three registers of different lengths',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd3reg_diff',
};

ICLASS simd_3same => {
  title      => 'Advanced SIMD three registers of the same length',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx o1:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp mapto=simd3reg_same',
};

ICLASS simd_3sameext => {
  title      => 'Advanced SIMD three registers of the same length extension',
  diagram    => 'ig0=0b1111110 op1:u=0bxx D:u=0bx op2:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3:u=0bx ig2=0b0 op4:u=0bx N:u=0bx Q:u=0bx M:u=0bx U:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=simd3reg_sameext',
};

ICLASS simd_dup_el => {
  title      => 'Advanced SIMD 8/16/32-bit element move/duplicate',
  diagram    => 'ig0=0b11101110 opc1:u=0bxxx L:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 mapto=movsimdgp',
};

ICLASS simd_dup_sc => {
  title      => 'Advanced SIMD duplicate (scalar)',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc:u=0bxxx Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd2reg_dup',
};

ICLASS simd_ext => {
  title      => 'Advanced SIMD vector extract',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd3reg_ext',
};

ICLASS simd_tbl => {
  title      => 'Advanced SIMD table permute',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg mapto=simd3reg_tbl',
};

ICLASS simdfp_ldst => {
  title      => 'Advanced SIMD and floating-point load/store',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 mapto=ldstsimdfp',
};

ICLASS simdfp_mov64 => {
  title      => 'Advanced SIMD and floating-point 64-bit move',
  diagram    => 'ig0=0b111011000 D:u=0bx ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size:u=0bxx opc2:u=0bxx M:u=0bx o3:u=0bx Vm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 mapto=movsimdfpgp64',
};


1;

