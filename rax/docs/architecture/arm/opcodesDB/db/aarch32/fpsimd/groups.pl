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

GROUP a_simd_12reg => {
  title      => 'Advanced SIMD shifts and immediate generation',
  diagram    => 'ig0=0b1111001 ig1=0bx ig2=0b1 ig3=0bx op0:u=0bxxxxxxxxxxxxxxx ig4=0bxx ig5=0b1 ig6=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp',
};

GROUP a_simd_mulreg => {
  title      => 'Advanced SIMD two registers, or three registers of different lengths',
  diagram    => 'ig0=0b1111001 op0:u=0bx ig1=0b1 ig2=0bx op1:u=0bxx ig3=0bxxxxxxxx op2:u=0bxx ig4=0bxxx op3:u=0bx ig5=0bx ig6=0b0 ig7=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp',
};

GROUP advsimddp => {
  title      => 'Advanced SIMD data-processing',
  diagram    => 'ig0=0b1111001 ig1=0bx op0:u=0bx ig2=0bxxxxxxxxxxxxxxxxxx op1:u=0bx ig3=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=uncond_as',
};

GROUP advsimdls => {
  title      => 'Advanced SIMD element or structure load/store',
  diagram    => 'ig0=0b11110100 op0:u=0bx ig1=0bxx ig2=0b0 ig3=0bxxxxxxxx op1:u=0bxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=uncond_as',
};

GROUP cops_as => {
  title      => 'System register access, Advanced SIMD, floating-point, and Supervisor call',
  diagram    => 'cond:u=0bxxxx ig0=0b11 op0:u=0bxx ig1=0bxxxxxxxxxxxx op1:u=0bxxx ig2=0bxxxx op2:u=0bx ig3=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
};

GROUP cpaf => {
  title      => 'System register access, Advanced SIMD, and floating-point',
  diagram    => 'ig0=0b111 op0:u=0bx ig1=0b11 op1:u=0bxx ig2=0bxxxxxxxxxxxx op2:u=0bxxx ig3=0bxxxx op3:u=0bx ig4=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=w',
};

GROUP fpdp => {
  title      => 'Floating-point data-processing',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 op0:u=0bxxxx op1:u=0bxxxx ig1=0bxxxx ig2=0b10 op2:u=0bxx ig3=0bx op3:u=0bx ig4=0bx ig5=0b0 ig6=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=cops_as mapto=fpdp',
};

GROUP fpdp => {
  title      => 'Floating-point data-processing',
  diagram    => 'ig0=0b111 op0:u=0bx ig1=0b1110 op1:u=0bxxxx op2:u=0bxxxx ig2=0bxxxx ig3=0b10 op3:u=0bxx ig4=0bx op4:u=0bx ig5=0bx ig6=0b0 ig7=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=fpdp',
};

GROUP fpsimd_mov32 => {
  title      => 'Advanced SIMD and floating-point 32-bit move',
  diagram    => 'ig0=0bxxxx ig1=0b1110 op0:u=0bxxx ig2=0bxxxxxxxxx ig3=0b10 op1:u=0bxx ig4=0bxxx ig5=0b11111',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=cops_as mapto=fpsimd_mov32',
};

GROUP fpsimd_mov32 => {
  title      => 'Advanced SIMD and floating-point 32-bit move',
  diagram    => 'ig0=0b11101110 op0:u=0bxxx ig1=0bxxxxxxxxx ig2=0b101 op1:u=0bx ig3=0bxxx ig4=0b11111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=fpsimd_mov32',
};

GROUP simddp => {
  title      => 'Advanced SIMD data-processing',
  diagram    => 'ig0=0b111 ig1=0bx ig2=0b1111 op0:u=0bx ig3=0bxxxxxxxxxxxxxxxxxx op1:u=0bx ig4=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=advsimddp',
};

GROUP simdldst_mov64 => {
  title      => 'Advanced SIMD load/store and 64-bit move',
  diagram    => 'ig0=0bxxxx ig1=0b110 op0:u=0bxxxx ig2=0bxxxxxxxxx ig3=0b10 ig4=0bxxxxxxxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=cops_as mapto=simdldst_mov64',
};

GROUP simdldst_mov64 => {
  title      => 'Advanced SIMD load/store and 64-bit move',
  diagram    => 'ig0=0b1110110 op0:u=0bxxxx ig1=0bxxxxxxxxx ig2=0b10 ig3=0bxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=cpaf mapto=simdldst_mov64',
};

GROUP t_simd_12reg => {
  title      => 'Advanced SIMD shifts and immediate generation',
  diagram    => 'ig0=0b111 ig1=0bx ig2=0b11111 ig3=0bx op0:u=0bxxxxxxxxxxxxxxx ig4=0bxx ig5=0b1 ig6=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp',
};

GROUP t_simd_mulreg => {
  title      => 'Advanced SIMD two registers, or three registers of different lengths',
  diagram    => 'ig0=0b111 op0:u=0bx ig1=0b11111 ig2=0bx op1:u=0bxx ig3=0bxxxxxxxx op2:u=0bxx ig4=0bxxx op3:u=0bx ig5=0bx ig6=0b0 ig7=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp',
};

GROUP vldst => {
  title      => 'Advanced SIMD element or structure load/store',
  diagram    => 'ig0=0b11111001 op0:u=0bx ig1=0bxx ig2=0b0 ig3=0bxxxxxxxx op1:u=0bxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=w mapto=advsimdls',
};


1;

