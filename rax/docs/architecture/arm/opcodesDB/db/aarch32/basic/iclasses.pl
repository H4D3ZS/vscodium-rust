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

ICLASS addpcsp16 => {
  title      => 'Add PC/SP (immediate)',
  diagram    => 'ig0=0b1010 SP:u=0bx Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS addsub16_1l_imm => {
  title      => 'Add, subtract, compare, move (one low register and immediate)',
  diagram    => 'ig0=0b001 op:u=0bxx Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi',
};

ICLASS addsub16_2h => {
  title      => 'Add, subtract, compare, move (two high registers)',
  diagram    => 'ig0=0b010001 op:u=0bxx D:u=0bx Rs:u=0bxxxx Rd:u=0bxxx',
  cnsts      => 'op!=0b11',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd',
};

ICLASS addsub16_2l_imm => {
  title      => 'Add, subtract (two low registers and immediate)',
  diagram    => 'ig0=0b000111 S:u=0bx imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi',
};

ICLASS addsub16_3l => {
  title      => 'Add, subtract (three low registers)',
  diagram    => 'ig0=0b000110 S:u=0bx Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi',
};

ICLASS addsub_par => {
  title      => 'Parallel add-subtract',
  diagram    => 'ig0=0b111110101 op1:u=0bxxx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U:u=0bx H:u=0bx S:u=0bx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg',
};

ICLASS adjsp16 => {
  title      => 'Adjust SP (immediate)',
  diagram    => 'ig0=0b10110000 S:u=0bx imm7:u=0bxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS b => {
  title      => 'Unconditional branch',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b10 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS b16 => {
  title      => 'Unconditional branch',
  diagram    => 'ig0=0b11100 imm11:u=0bxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
};

ICLASS b_imm => {
  title      => 'Branch (immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b101 H:u=0bx imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk',
};

ICLASS barriers => {
  title      => 'Barriers',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode:u=0bxxxx option:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints',
};

ICLASS bcond => {
  title      => 'Conditional branch',
  diagram    => 'ig0=0b11110 S:u=0bx cond:u=0bxxxx imm6:u=0bxxxxxx ig1=0b10 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS bcond16 => {
  title      => 'Conditional branch',
  diagram    => 'ig0=0b1101 cond:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc',
};

ICLASS bfi => {
  title      => 'Bitfield Insert',
  diagram    => 'cond:u=0bxxxx ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS bfx => {
  title      => 'Bitfield Extract',
  diagram    => 'cond:u=0bxxxx ig0=0b01111 U:u=0bx ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS bkpt16 => {
  title      => 'Software breakpoint',
  diagram    => 'ig0=0b10111110 imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS bl => {
  title      => 'Unconditional branch and link',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS blx => {
  title      => 'Unconditional branch and link exchange',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS blx_reg => {
  title      => 'Branch with Link and Exchange (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS bx16 => {
  title      => 'Branch and exchange',
  diagram    => 'ig0=0b01000111 L:u=0bx Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd',
};

ICLASS bx_jaz => {
  title      => 'Branch and Exchange Jazelle',
  diagram    => 'ig0=0b111100111100 Rm:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS bx_reg => {
  title      => 'Branch and Exchange (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS bxj_reg => {
  title      => 'Branch and Exchange to Jazelle (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0010 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS cbznz16 => {
  title      => 'Compare and branch zero/non-zero',
  diagram    => 'ig0=0b1011 op:u=0bx ig1=0b0 i:u=0bx ig2=0b1 imm5:u=0bxxxxx Rn:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS clz => {
  title      => 'Count Leading Zeros',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bS ig2=0bS ig3=0bS ig4=0bS Rd:u=0bxxxx ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS cp_ldst => {
  title      => 'System register Load/Store',
  diagram    => 'ig0=0b111 o0:u=0bx ig1=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx CRd:u=0bxxxx ig2=0b111 cp15:u=0bx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64',
};

ICLASS cp_mov32 => {
  title      => 'System register 32-bit move',
  diagram    => 'ig0=0b111 o0:u=0bx ig1=0b1110 opc1:u=0bxxx L:u=0bx CRn:u=0bxxxx Rt:u=0bxxxx ig2=0b111 cp15:u=0bx opc2:u=0bxxx ig3=0b1 CRm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf',
};

ICLASS cp_mov64 => {
  title      => 'System register 64-bit move',
  diagram    => 'ig0=0b111 o0:u=0bx ig1=0b11000 D:u=0bx ig2=0b0 L:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig3=0b111 cp15:u=0bx opc1:u=0bxxxx CRm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64',
};

ICLASS cps => {
  title      => 'Change Process State',
  diagram    => 'ig0=0b111100010000 imod:u=0bxx M:u=0bx op:u=0bx ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E:u=0bx A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc',
};

ICLASS cps => {
  title      => 'Change processor state',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0bxx M:u=0bx A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS cps16 => {
  title      => 'Change Processor State',
  diagram    => 'ig0=0b1011011001 op:u=0bx flags:u=0bxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS crc32 => {
  title      => 'Cyclic Redundancy Check',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0bx ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS dcps => {
  title      => 'DCPS',
  diagram    => 'ig0=0b111101111000 imm4:u=0bxxxx ig1=0b1000 imm10:u=0bxxxxxxxxxx opt:u=0bxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS dpint16_2l => {
  title      => 'Data-processing (two low registers)',
  diagram    => 'ig0=0b010000 op:u=0bxxxx Rs:u=0bxxx Rd:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS dpint_2r => {
  title      => 'Data-processing (two source registers)',
  diagram    => 'ig0=0b111110101 op1:u=0bxxx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg',
};

ICLASS dpint_immm => {
  title      => 'Data-processing (modified immediate)',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1:u=0bxxxx S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

ICLASS dpint_imms => {
  title      => 'Data-processing (simple immediate)',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1:u=0bx ig2=0b0 o2:u=0bx ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm',
};

ICLASS dpint_shiftr => {
  title      => 'Data-processing (shifted register)',
  diagram    => 'ig0=0b1110101 op1:u=0bxxxx S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

ICLASS eret => {
  title      => 'Exception Return',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0b0110 ig14=0bS ig15=0bS ig16=0bS ig17=0bZ',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS eret => {
  title      => 'Exception return',
  diagram    => 'ig0=0b111100111101 Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS except => {
  title      => 'Exception Generation',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS except => {
  title      => 'Exception generation',
  diagram    => 'ig0=0b11110111111 o1:u=0bx imm4:u=0bxxxx ig1=0b10 o2:u=0bx ig2=0b0 imm12:u=0bxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS except16 => {
  title      => 'Exception generation',
  diagram    => 'ig0=0b1101111 S:u=0bx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc',
};

ICLASS ext16 => {
  title      => 'Extend',
  diagram    => 'ig0=0b10110010 U:u=0bx B:u=0bx Rm:u=0bxxx Rd:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS extend => {
  title      => 'Extend and Add',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U:u=0bx op:u=0bxx Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS extendr => {
  title      => 'Register extends',
  diagram    => 'ig0=0b111110100 op1:u=0bxx U:u=0bx Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg',
};

ICLASS hints => {
  title      => 'Hints',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint:u=0bxxxx option:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS hints16 => {
  title      => 'Hints',
  diagram    => 'ig0=0b10111111 hint:u=0bxxxx ig1=0b0000',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS hlt16 => {
  title      => 'Halting breakpoint',
  diagram    => 'ig0=0b1011101010 imm6:u=0bxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS intdp1reg_imm => {
  title      => 'Integer Test and Compare (one register and immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS intdp2reg_imm => {
  title      => 'Integer Data Processing (two register and immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS intdp2reg_immsh => {
  title      => 'Integer Test and Compare (two register, immediate shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis',
};

ICLASS intdp2reg_regsh => {
  title      => 'Integer Test and Compare (two register, register shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs',
};

ICLASS intdp3reg_immsh => {
  title      => 'Integer Data Processing (three register, immediate shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis',
};

ICLASS intdp3reg_regsh => {
  title      => 'Integer Data Processing (three register, register shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc:u=0bxxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs',
};

ICLASS intsat => {
  title      => 'Integer Saturating Arithmetic',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS it16 => {
  title      => 'If-Then',
  diagram    => 'ig0=0b10111111 firstcond:u=0bxxxx mask:u=0bxxxx',
  cnsts      => 'mask!=0b0000',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS ldastl => {
  title      => 'Load-acquire / Store-release',
  diagram    => 'ig0=0b11101000110 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op:u=0bx sz:u=0bxx Rd:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS lddlit => {
  title      => 'Load dual (literal)',
  diagram    => 'ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L:u=0bx ig2=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldlit16 => {
  title      => 'Load literal',
  diagram    => 'ig0=0b01001 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldlit_signed => {
  title      => 'Load, signed (literal)',
  diagram    => 'ig0=0b11111001 U:u=0bx size:u=0bxx ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldlit_unsigned => {
  title      => 'Load, unsigned (literal)',
  diagram    => 'ig0=0b11111000 U:u=0bx size:u=0bxx L:u=0bx ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst16_imm => {
  title      => 'Load/store word/byte (immediate offset)',
  diagram    => 'ig0=0b011 B:u=0bx L:u=0bx imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldst16_reg => {
  title      => 'Load/store (register offset)',
  diagram    => 'ig0=0b0101 L:u=0bx B:u=0bx H:u=0bx Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldst16_sp => {
  title      => 'Load/store (SP-relative)',
  diagram    => 'ig0=0b1001 L:u=0bx Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldst_excl => {
  title      => 'Load/Store Exclusive and Load-Acquire/Store-Release',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size:u=0bxx L:u=0bx Rn:u=0bxxxx xRd:u=0bxxxx ig1=0bS ig2=0bS ex:u=0bx ord:u=0bx ig3=0b1001 xRt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync',
};

ICLASS ldst_signed_nimm => {
  title      => 'Load/store, signed (negative immediate)',
  diagram    => 'ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_signed_pimm => {
  title      => 'Load/store, signed (positive immediate)',
  diagram    => 'ig0=0b111110011 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_signed_post => {
  title      => 'Load/store, signed (immediate, post-indexed)',
  diagram    => 'ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b10 U:u=0bx ig3=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_signed_pre => {
  title      => 'Load/store, signed (immediate, pre-indexed)',
  diagram    => 'ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b11 U:u=0bx ig3=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_signed_reg => {
  title      => 'Load/store, signed (register offset)',
  diagram    => 'ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_signed_unpriv => {
  title      => 'Load/store, signed (unprivileged)',
  diagram    => 'ig0=0b111110010 size:u=0bxx ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_nimm => {
  title      => 'Load/store, unsigned (negative immediate)',
  diagram    => 'ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_pimm => {
  title      => 'Load/store, unsigned (positive immediate)',
  diagram    => 'ig0=0b111110001 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_post => {
  title      => 'Load/store, unsigned (immediate, post-indexed)',
  diagram    => 'ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b10 U:u=0bx ig2=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_pre => {
  title      => 'Load/store, unsigned (immediate, pre-indexed)',
  diagram    => 'ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b11 U:u=0bx ig2=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_reg => {
  title      => 'Load/store, unsigned (register offset)',
  diagram    => 'ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unsigned_unpriv => {
  title      => 'Load/store, unsigned (unprivileged)',
  diagram    => 'ig0=0b111110000 size:u=0bxx L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldstcp => {
  title      => 'System register load/store',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx CRd:u=0bxxxx ig1=0b111 cp15:u=0bx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64',
};

ICLASS ldstd_imm => {
  title      => 'Load/store dual (immediate)',
  diagram    => 'ig0=0b11101001 U:u=0bx ig1=0b10 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldstd_post => {
  title      => 'Load/store dual (immediate, post-indexed)',
  diagram    => 'ig0=0b11101000 U:u=0bx ig1=0b11 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldstd_pre => {
  title      => 'Load/store dual (immediate, pre-indexed)',
  diagram    => 'ig0=0b11101001 U:u=0bx ig1=0b11 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldstex => {
  title      => 'Load/store exclusive',
  diagram    => 'ig0=0b11101000010 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldstex_bhd => {
  title      => 'Load/store exclusive byte/half/dual',
  diagram    => 'ig0=0b11101000110 L:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz:u=0bxx Rd:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS ldstexcept => {
  title      => 'Exception Save/Restore',
  diagram    => 'ig0=0b1111100 P:u=0bx U:u=0bx S:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx op:u=0bxxxxxxxxxxx mode:u=0bxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk',
};

ICLASS ldsth16_imm => {
  title      => 'Load/store halfword (immediate offset)',
  diagram    => 'ig0=0b1000 L:u=0bx imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldstimm => {
  title      => 'Load/Store Word, Unsigned Byte (immediate, literal)',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2:u=0bx W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

ICLASS ldstm => {
  title      => 'Load/Store Multiple',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op:u=0bx W:u=0bx L:u=0bx Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk',
};

ICLASS ldstm => {
  title      => 'Load/store multiple',
  diagram    => 'ig0=0b1110100 opc:u=0bxx ig1=0b0 W:u=0bx L:u=0bx Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

ICLASS ldstm16 => {
  title      => 'Load/store multiple',
  diagram    => 'ig0=0b1100 L:u=0bx Rn:u=0bxxx register_list:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n',
};

ICLASS ldstreg => {
  title      => 'Load/Store Word, Unsigned Byte (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0bx U:u=0bx o2:u=0bx W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
};

ICLASS ldstximm => {
  title      => 'Load/Store Dual, Half, Signed Byte (immediate, literal)',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2:u=0bxx ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 op2!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst',
};

ICLASS ldstxreg => {
  title      => 'Load/Store Dual, Half, Signed Byte (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1:u=0bx Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 op2!=0b00',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst',
};

ICLASS lmul_div => {
  title      => 'Long multiply and divide',
  diagram    => 'ig0=0b111110111 op1:u=0bxxx Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2:u=0bxxxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w',
};

ICLASS log2reg_imm => {
  title      => 'Logical Arithmetic  (two register and immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS logic3reg_immsh => {
  title      => 'Logical Arithmetic (three register, immediate shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis',
};

ICLASS logic3reg_regsh => {
  title      => 'Logical Arithmetic (three register, register shift)',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc:u=0bxx S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs',
};

ICLASS movcpgp32 => {
  title      => 'System register 32-bit move',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L:u=0bx CRn:u=0bxxxx Rt:u=0bxxxx ig1=0b111 cp15:u=0bx opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as',
};

ICLASS movcpgp64 => {
  title      => 'System register 64-bit move',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D:u=0bx ig1=0b0 L:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b111 cp15:u=0bx opc1:u=0bxxxx CRm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64',
};

ICLASS movsr_hint_imm => {
  title      => 'Move Special Register and Hints (immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R:u=0bx ig1=0b10 imm4:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS movsr_reg => {
  title      => 'Move special register (register)',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b0 mask:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ B:u=0bx m:u=0bx ig4=0b0000 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc',
};

ICLASS movw => {
  title      => 'Move Halfword (immediate)',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 H:u=0bx ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS movw => {
  title      => 'Move Wide (16-bit immediate)',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1:u=0bx ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm',
};

ICLASS mrs_bank => {
  title      => 'MRS (banked)',
  diagram    => 'ig0=0b11110011111 R:u=0bx m1:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 Rd:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 m:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS mrs_spec => {
  title      => 'MRS (special)',
  diagram    => 'ig0=0b11110011111 R:u=0bx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 Rd:u=0bxxxx ig8=0bZ ig9=0bZ ig10=0b0 ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS msr_bank => {
  title      => 'MSR (banked)',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 m1:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 m:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS msr_spec => {
  title      => 'MSR (special)',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 mask:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b0 ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS mul_abd => {
  title      => 'Multiply and absolute difference',
  diagram    => 'ig0=0b111110110 op1:u=0bxxx Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2:u=0bxx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul',
};

ICLASS mul_half => {
  title      => 'Halfword Multiply and Accumulate',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc:u=0bxx ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0bx N:u=0bx ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

ICLASS mul_word => {
  title      => 'Multiply and Accumulate',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc:u=0bxxx S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp',
};

ICLASS pack => {
  title      => 'Pack Halfword',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx tb:u=0bx ig1=0b01 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS parallel => {
  title      => 'Parallel Arithmetic',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1:u=0bxxx Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B:u=0bx op2:u=0bxx ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS preload_imm => {
  title      => 'Preload (immediate)',
  diagram    => 'ig0=0b1111010 D:u=0bx U:u=0bx R:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints',
};

ICLASS preload_reg => {
  title      => 'Preload (register)',
  diagram    => 'ig0=0b1111011 D:u=0bx U:u=0bx o2:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints',
};

ICLASS pushpop16 => {
  title      => 'Push and Pop',
  diagram    => 'ig0=0b1011 L:u=0bx ig1=0b10 P:u=0bx register_list:u=0bxxxxxxxx',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS rev16 => {
  title      => 'Reverse bytes',
  diagram    => 'ig0=0b10111010 op:u=0bxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'op!=0b10',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS reverse => {
  title      => 'Reverse Bit/Byte',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1:u=0bx ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2:u=0bx ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS sat16 => {
  title      => 'Saturate 16-bit',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U:u=0bx ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS sat32 => {
  title      => 'Saturate 32-bit',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U:u=0bx ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0bx ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS sat_bit => {
  title      => 'Saturate, Bitfield',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1:u=0bxxx ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm',
};

ICLASS selbytes => {
  title      => 'Select Bytes',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b1011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS setpan => {
  title      => 'SETPAN',
  diagram    => 'ig0=0b111100010001 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ imm1:u=0bx ig11=0bZ ig12=0b0000 ig13=0bZ ig14=0bZ ig15=0bZ ig16=0bZ',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc',
};

ICLASS setpan16 => {
  title      => 'SETPAN',
  diagram    => 'ig0=0b10110110000 ig1=0bS imm1:u=0bx ig2=0bZ ig3=0bZ ig4=0bZ',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16',
};

ICLASS shift16_imm => {
  title      => 'Shift (immediate)',
  diagram    => 'ig0=0b000 op:u=0bxx imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'op!=0b11',
  metadata   => 'isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi',
};

ICLASS shiftr => {
  title      => 'Register shifts',
  diagram    => 'ig0=0b111110100 stype:u=0bxx S:u=0bx Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg',
};

ICLASS smul_div => {
  title      => 'Signed multiply, Divide',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1:u=0bxxx Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx op2:u=0bxxx ig1=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS svc => {
  title      => 'Supervisor Call',
  diagram    => 'cond:u=0bxxxx ig0=0b1111 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=svcall',
};

ICLASS system => {
  title      => 'Miscellaneous system',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc:u=0bxxxx option:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl',
};

ICLASS tblbr => {
  title      => 'Table branch',
  diagram    => 'ig0=0b111010001101 Rn:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0b000 H:u=0bx Rm:u=0bxxxx',
  metadata   => 'isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd',
};

ICLASS udf => {
  title      => 'Permanently UNDEFINED',
  diagram    => 'cond:u=0bxxxx ig0=0b01111111 imm12:u=0bxxxxxxxxxxxx ig1=0b1111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};

ICLASS usad => {
  title      => 'Unsigned Sum of Absolute Differences',
  diagram    => 'cond:u=0bxxxx ig0=0b01111000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  metadata   => 'isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media',
};


1;

