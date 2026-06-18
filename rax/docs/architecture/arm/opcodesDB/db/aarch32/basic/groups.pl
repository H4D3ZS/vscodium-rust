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

GROUP bcrtrl => {
  title      => 'Branches and miscellaneous control',
  diagram    => 'ig0=0b11110 op0:u=0bx op1:u=0bxxxx op2:u=0bxx ig1=0bxxxx ig2=0b1 op3:u=0bxxx ig3=0bx op4:u=0bxxx ig4=0bxx op5:u=0bx ig5=0bxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP brblk => {
  title      => 'Branch, branch with link, and block data transfer',
  diagram    => 'cond:u=0bxxxx ig0=0b10 op0:u=0bx ig1=0bxxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

GROUP brc => {
  title      => 'Conditional branch, and Supervisor Call',
  diagram    => 'ig0=0b1101 op0:u=0bxxxx ig1=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

GROUP dp => {
  title      => 'Data-processing and miscellaneous instructions',
  diagram    => 'ig0=0bxxxx ig1=0b00 op0:u=0bx op1:u=0bxxxxx ig2=0bxxxxxxxxxxxx op2:u=0bx op3:u=0bxx op4:u=0bx ig3=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

GROUP dpimm => {
  title      => 'Data-processing immediate',
  diagram    => 'ig0=0bxxxx ig1=0b001 op0:u=0bxx ig2=0bx op1:u=0bxx ig3=0bxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

GROUP dpmisc => {
  title      => 'Miscellaneous',
  diagram    => 'ig0=0bxxxx ig1=0b00010 op0:u=0bxx ig2=0b0 ig3=0bxxxxxxxxxxxx ig4=0b0 op1:u=0bxxx ig5=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

GROUP dpregis => {
  title      => 'Data-processing register (immediate shift)',
  diagram    => 'ig0=0bxxxx ig1=0b000 op0:u=0bxx ig2=0bxx op1:u=0bx ig3=0bxxxxxxxxxxxxxxx ig4=0b0 ig5=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

GROUP dpregrs => {
  title      => 'Data-processing register (register shift)',
  diagram    => 'ig0=0bxxxx ig1=0b000 op0:u=0bxx ig2=0bxx op1:u=0bx ig3=0bxxxxxxxxxxxx ig4=0b0 ig5=0bxx ig6=0b1 ig7=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

GROUP dstd => {
  title      => 'Load/store dual, load/store exclusive, load-acquire/store-release, and table branch',
  diagram    => 'ig0=0b1110100 op0:u=0bxxxx op1:u=0bx op2:u=0bxxxx ig1=0bxxxxxxxx op3:u=0bxxx ig2=0bxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP imm => {
  title      => 'Data-processing (plain binary immediate)',
  diagram    => 'ig0=0b11110 ig1=0bx ig2=0b1 op0:u=0bx ig3=0bx op1:u=0bxx ig4=0b0 ig5=0bxxxx ig6=0b0 ig7=0bxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP ldst => {
  title      => 'Load/store single',
  diagram    => 'ig0=0b1111100 op0:u=0bxx ig1=0bxx op1:u=0bx op2:u=0bxxxx ig2=0bxxxx op3:u=0bxxxxxx ig3=0bxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP media => {
  title      => 'Media instructions',
  diagram    => 'ig0=0bxxxx ig1=0b011 op0:u=0bxxxxx ig2=0bxxxxxxxxxxxx op1:u=0bxxx ig3=0b1 ig4=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

GROUP misc16 => {
  title      => 'Miscellaneous 16-bit instructions',
  diagram    => 'ig0=0b1011 op0:u=0bxxxx op1:u=0bxx op2:u=0bx ig1=0bx op3:u=0bxxxx ig2=0bxxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

GROUP mul => {
  title      => 'Multiply, multiply accumulate, and absolute difference',
  diagram    => 'ig0=0b111110110 ig1=0bxxxxxxxxxxxxxxx op0:u=0bxx ig2=0bxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP n => {
  title      => '16-bit',
  diagram    => 'op0:u=0bxxxxxx ig0=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
};

GROUP reg => {
  title      => 'Data-processing (register)',
  diagram    => 'ig0=0b11111010 op0:u=0bx ig1=0bxxxxxxx ig2=0b1111 ig3=0bxxxx op1:u=0bxxxx ig4=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

GROUP sftdpi => {
  title      => 'Shift (immediate), add, subtract, move, and compare',
  diagram    => 'ig0=0b00 op0:u=0bx op1:u=0bxx op2:u=0bx ig1=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

GROUP spcd => {
  title      => 'Special data instructions and branch and exchange',
  diagram    => 'ig0=0b010001 op0:u=0bxx ig1=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

GROUP svcall => {
  title      => 'Supervisor call',
  diagram    => 'cond:u=0bxxxx ig0=0b1111 ig1=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as',
};

GROUP sync => {
  title      => 'Synchronization primitives and Load-Acquire/Store-Release',
  diagram    => 'ig0=0bxxxx ig1=0b0001 op0:u=0bx ig2=0bxxxxxxxxxxx ig3=0b11 ig4=0bxx ig5=0b1001 ig6=0bxxxx',
  cnsts      => 'ig0!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

GROUP sysldst_mov64 => {
  title      => 'System register load/store and 64-bit move',
  diagram    => 'ig0=0bxxxx ig1=0b110 op0:u=0bxxxx ig2=0bxxxxxxxxx ig3=0b111 ig4=0bxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as mapto=sysldst_mov64',
};

GROUP sysldst_mov64 => {
  title      => 'System register load/store and 64-bit move',
  diagram    => 'ig0=0b111 ig1=0bx ig2=0b110 op0:u=0bxxxx ig3=0bxxxxxxxxx ig4=0b111 ig5=0bxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf mapto=sysldst_mov64',
};

GROUP uncond_as => {
  title      => 'Unconditional instructions',
  diagram    => 'ig0=0b11110 op0:u=0bxx ig1=0bxxxx op1:u=0bx ig2=0bxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

GROUP uncondhints => {
  title      => 'Memory hints and barriers',
  diagram    => 'ig0=0b111101 op0:u=0bxxxxx ig1=0b1 ig2=0bxxxxxxxxxxxxxxx op1:u=0bx ig3=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncond_as',
};

GROUP uncondmisc => {
  title      => 'Miscellaneous',
  diagram    => 'ig0=0b1111000 op0:u=0bxxxxx ig1=0bxxxxxxxxxxxx op1:u=0bxxxx ig2=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncond_as',
};

GROUP w => {
  title      => '32-bit',
  diagram    => 'ig0=0b111 op0:u=0bxxxx op1:u=0bxxxxx ig1=0bxxxx op3:u=0bx ig2=0bxxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
};

GROUP xldst => {
  title      => 'Extra load/store',
  diagram    => 'ig0=0bxxxx ig1=0b000 ig2=0bxx op0:u=0bx ig3=0bxxxxxxxxxxxxxx ig4=0b1 ig5=0bxx ig6=0b1 ig7=0bxxxx',
  cnsts      => 'ig0!=0b1111 ig5!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};


1;

