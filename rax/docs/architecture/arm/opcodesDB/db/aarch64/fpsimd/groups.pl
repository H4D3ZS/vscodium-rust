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

GROUP ldst => {
  title      => 'Loads and Stores',
  diagram    => 'op0:u=0bxxxx ig0=0b1 op1:u=0bx ig1=0b0 op2:u=0bxx ig2=0bx op3:u=0bxxxxxx ig3=0bxxxx op4:u=0bxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
};

GROUP simd_dp => {
  title      => 'Data Processing -- Scalar Floating-Point and Advanced SIMD',
  diagram    => 'op0:u=0bxxxx ig0=0b111 op1:u=0bxx op2:u=0bxxxx op3:u=0bxxxxxxxxx ig1=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=FPSIMD',
};


1;

