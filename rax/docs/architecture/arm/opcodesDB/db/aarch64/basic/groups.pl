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

GROUP control => {
  title      => 'Branches, Exception Generating and System instructions',
  diagram    => 'op0:u=0bxxx ig0=0b101 op1:u=0bxxxxxxxxxxxxxx ig1=0bxxxxxxx op2:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
};

GROUP dpimm => {
  title      => 'Data Processing -- Immediate',
  diagram    => 'ig0=0bxxx ig1=0b100 op0:u=0bxxx ig2=0bxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
};

GROUP dpreg => {
  title      => 'Data Processing -- Register',
  diagram    => 'ig0=0bx op0:u=0bx ig1=0bx op1:u=0bx ig2=0b101 op2:u=0bxxxx ig3=0bxxxxx op3:u=0bxxxxxx ig4=0bxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
};

GROUP reserved => {
  title      => 'Reserved',
  diagram    => 'op0:u=0bxxx ig0=0b0000 op1:u=0bxxxxxxxxx ig1=0bxxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
};


1;

