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

ICLASS addsub_carry => {
  title      => 'Add/subtract (with carry)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 Rm:u=0bxxxxx ig1=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS addsub_ext => {
  title      => 'Add/subtract (extended register)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b01011 opt:u=0bxx ig1=0b1 Rm:u=0bxxxxx option:u=0bxxx imm3:u=0bxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS addsub_imm => {
  title      => 'Add/subtract (immediate)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b100010 sh:u=0bx imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS addsub_immtags => {
  title      => 'Add/subtract (immediate, with tags)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b100011 o2:u=0bx uimm6:u=0bxxxxxx op3:u=0bxx uimm4:u=0bxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS addsub_shift => {
  title      => 'Add/subtract (shifted register)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b01011 shift:u=0bxx ig1=0b0 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS barriers => {
  title      => 'Barriers',
  diagram    => 'ig0=0b11010101000000110011 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS bitfield => {
  title      => 'Bitfield',
  diagram    => 'sf:u=0bx opc:u=0bxx ig0=0b100110 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS branch_imm => {
  title      => 'Unconditional branch (immediate)',
  diagram    => 'op:u=0bx ig0=0b00101 imm26:u=0bxxxxxxxxxxxxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS branch_reg => {
  title      => 'Unconditional branch (register)',
  diagram    => 'ig0=0b1101011 opc:u=0bxxxx op2:u=0bxxxxx op3:u=0bxxxxxx Rn:u=0bxxxxx op4:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS compbranch => {
  title      => 'Compare and branch (immediate)',
  diagram    => 'sf:u=0bx ig0=0b011010 op:u=0bx imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS condbranch => {
  title      => 'Conditional branch (immediate)',
  diagram    => 'ig0=0b0101010 o1:u=0bx imm19:u=0bxxxxxxxxxxxxxxxxxxx o0:u=0bx cond:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS condcmp_imm => {
  title      => 'Conditional compare (immediate)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010010 imm5:u=0bxxxxx cond:u=0bxxxx ig1=0b1 o2:u=0bx Rn:u=0bxxxxx o3:u=0bx nzcv:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS condcmp_reg => {
  title      => 'Conditional compare (register)',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010010 Rm:u=0bxxxxx cond:u=0bxxxx ig1=0b0 o2:u=0bx Rn:u=0bxxxxx o3:u=0bx nzcv:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS condsel => {
  title      => 'Conditional select',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010100 Rm:u=0bxxxxx cond:u=0bxxxx op2:u=0bxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS dp_1src => {
  title      => 'Data-processing (1 source)',
  diagram    => 'sf:u=0bx ig0=0b1 S:u=0bx ig1=0b11010110 opcode2:u=0bxxxxx opcode:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS dp_2src => {
  title      => 'Data-processing (2 source)',
  diagram    => 'sf:u=0bx ig0=0b0 S:u=0bx ig1=0b11010110 Rm:u=0bxxxxx opcode:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS dp_3src => {
  title      => 'Data-processing (3 source)',
  diagram    => 'sf:u=0bx op54:u=0bxx ig0=0b11011 op31:u=0bxxx Rm:u=0bxxxxx o0:u=0bx Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS exception => {
  title      => 'Exception generation',
  diagram    => 'ig0=0b11010100 opc:u=0bxxx imm16:u=0bxxxxxxxxxxxxxxxx op2:u=0bxxx LL:u=0bxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS extract => {
  title      => 'Extract',
  diagram    => 'sf:u=0bx op21:u=0bxx ig0=0b100111 N:u=0bx o0:u=0bx Rm:u=0bxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS hints => {
  title      => 'Hints',
  diagram    => 'ig0=0b11010101000000110010 CRm:u=0bxxxx op2:u=0bxxx ig1=0b11111',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS ldapstl_unscaled => {
  title      => 'LDAPR/STLR (unscaled immediate)',
  diagram    => 'size:u=0bxx ig0=0b011001 opc:u=0bxx ig1=0b0 imm9:u=0bxxxxxxxxx ig2=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_pac => {
  title      => 'Load/store register (pac)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 M:u=0bx S:u=0bx ig2=0b1 imm9:u=0bxxxxxxxxx W:u=0bx ig3=0b1 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldst_unpriv => {
  title      => 'Load/store register (unprivileged)',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 opc:u=0bxx ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldstexcl => {
  title      => 'Load/store exclusive',
  diagram    => 'size:u=0bxx ig0=0b001000 o2:u=0bx L:u=0bx o1:u=0bx Rs:u=0bxxxxx o0:u=0bx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS ldsttags => {
  title      => 'Load/store memory tags',
  diagram    => 'ig0=0b11011001 opc:u=0bxx ig1=0b1 imm9:u=0bxxxxxxxxx op2:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS log_imm => {
  title      => 'Logical (immediate)',
  diagram    => 'sf:u=0bx opc:u=0bxx ig0=0b100100 N:u=0bx immr:u=0bxxxxxx imms:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS log_shift => {
  title      => 'Logical (shifted register)',
  diagram    => 'sf:u=0bx opc:u=0bxx ig0=0b01010 shift:u=0bxx N:u=0bx Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS memop => {
  title      => 'Atomic memory operations',
  diagram    => 'size:u=0bxx ig0=0b111 V:u=0bx ig1=0b00 A:u=0bx R:u=0bx ig2=0b1 Rs:u=0bxxxxx o3:u=0bx opc:u=0bxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=ldst',
};

ICLASS movewide => {
  title      => 'Move wide (immediate)',
  diagram    => 'sf:u=0bx opc:u=0bxx ig0=0b100101 hw:u=0bxx imm16:u=0bxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS pcreladdr => {
  title      => 'PC-rel. addressing',
  diagram    => 'op:u=0bx immlo:u=0bxx ig0=0b10000 immhi:u=0bxxxxxxxxxxxxxxxxxxx Rd:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpimm',
};

ICLASS perm_undef => {
  title      => 'Reserved',
  diagram    => 'ig0=0b0000000000000000 imm16:u=0bxxxxxxxxxxxxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=reserved',
};

ICLASS pstate => {
  title      => 'PSTATE',
  diagram    => 'ig0=0b1101010100000 op1:u=0bxxx ig1=0b0100 CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS rmif => {
  title      => 'Rotate right into flags',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 imm6:u=0bxxxxxx ig1=0b00001 Rn:u=0bxxxxx o2:u=0bx mask:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS setf => {
  title      => 'Evaluate into flags',
  diagram    => 'sf:u=0bx op:u=0bx S:u=0bx ig0=0b11010000 opcode2:u=0bxxxxxx sz:u=0bx ig1=0b0010 Rn:u=0bxxxxx o3:u=0bx mask:u=0bxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=dpreg',
};

ICLASS systeminstrs => {
  title      => 'System instructions',
  diagram    => 'ig0=0b1101010100 L:u=0bx ig1=0b01 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS systemmove => {
  title      => 'System register move',
  diagram    => 'ig0=0b1101010100 L:u=0bx ig1=0b1 o0:u=0bx op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS systemresult => {
  title      => 'System with result',
  diagram    => 'ig0=0b1101010100100 op1:u=0bxxx CRn:u=0bxxxx CRm:u=0bxxxx op2:u=0bxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};

ICLASS testbranch => {
  title      => 'Test and branch (immediate)',
  diagram    => 'b5:u=0bx ig0=0b011011 op:u=0bx b40:u=0bxxxxx imm14:u=0bxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  metadata   => 'isa=A64 isaform=A64 alphaindex=BASIC',
  tags       => 'group=control',
};


1;

