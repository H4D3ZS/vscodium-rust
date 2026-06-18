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

ENCODING ADC_i_A1 => {
  name       => 'ADC',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b101 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADC_i cclass=ADC_i_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_i_A1 => {
  name       => 'ADCS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b101 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADC_i cclass=ADC_i_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_i_T1 => {
  name       => 'ADC',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1010 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADC_i cclass=ADC_i_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_i_T1 => {
  name       => 'ADCS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1010 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADCS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADC_i cclass=ADC_i_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_r_A1_RRX => {
  name       => 'ADC, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADC mnemonic-shift-type=ADC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADC_r cclass=ADC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_r_A1 => {
  name       => 'ADC, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADC mnemonic-shift-type=ADC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADC_r cclass=ADC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_r_A1_RRX => {
  name       => 'ADCS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADCS mnemonic-shift-type=ADCS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADC_r cclass=ADC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_r_A1 => {
  name       => 'ADCS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADCS mnemonic-shift-type=ADCS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADC_r cclass=ADC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0101 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ADC_r cclass=ADC_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_r_T2_RRX => {
  name       => 'ADC, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1010 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADC mnemonic-shift-type=ADC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADC_r cclass=ADC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_r_T2 => {
  name       => 'ADC, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1010 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADC mnemonic-shift-type=ADC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADC_r cclass=ADC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_r_T2_RRX => {
  name       => 'ADCS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1010 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADCS mnemonic-shift-type=ADCS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADC_r cclass=ADC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_r_T2 => {
  name       => 'ADCS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1010 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADCS mnemonic-shift-type=ADCS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADC_r cclass=ADC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADCS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=ADCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADC_rr cclass=ADC_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADC_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=ADC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADC_rr cclass=ADC_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING ADD_i_A1 => {
  name       => 'ADD',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b11x1',
  bitdiffs   => 'S == 0b0 && Rn != 0b11x1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_i cclass=ADD_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_i_A1 => {
  name       => 'ADDS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b1 && Rn != 0b1101',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_i cclass=ADD_i_a1',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000111 S=0b0 imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_2l_imm page=ADD_i cclass=ADD_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b001 op=0b10 Rdn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=ADD_i cclass=ADD_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_i_T3 => {
  name       => 'ADD',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_i cclass=ADD_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_i_T3 => {
  name       => 'ADDS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_i cclass=ADD_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_i_T4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b11x1',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=ADDW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_i cclass=ADD_i_t4',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_A1_RRX => {
  name       => 'ADD, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD mnemonic-shift-type=ADD-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_r cclass=ADD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_A1 => {
  name       => 'ADD, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD mnemonic-shift-type=ADD-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_r cclass=ADD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_r_A1_RRX => {
  name       => 'ADDS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS mnemonic-shift-type=ADDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_r cclass=ADD_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_r_A1 => {
  name       => 'ADDS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS mnemonic-shift-type=ADDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_r cclass=ADD_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000110 S=0b0 Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_3l page=ADD_r cclass=ADD_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b00 DN:u=0bx Rm:u=0bxxxx Rdn:u=0bxxx',
  cnsts      => 'Rm!=0b1101 DN:Rdn!=0b1101',
  bitdiffs   => '!(DN == 0b1 && Rdn == 0b101)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_r cclass=ADD_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_T3_RRX => {
  name       => 'ADD, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD mnemonic-shift-type=ADD-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_r cclass=ADD_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_r_T3 => {
  name       => 'ADD, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD mnemonic-shift-type=ADD-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_r cclass=ADD_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_r_T3_RRX => {
  name       => 'ADDS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS mnemonic-shift-type=ADDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_r cclass=ADD_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_r_T3 => {
  name       => 'ADDS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS mnemonic-shift-type=ADDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_r cclass=ADD_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=ADDS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADD_rr cclass=ADD_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=ADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADD_rr cclass=ADD_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_i_A1 => {
  name       => 'ADD',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_SP_i cclass=ADD_SP_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_i_A1 => {
  name       => 'ADDS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_SP_i cclass=ADD_SP_i_a1',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b1 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADD_SP_i cclass=ADD_SP_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b10110000 S=0b0 imm7:u=0bxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=adjsp16 page=ADD_SP_i cclass=ADD_SP_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_i_T3 => {
  name       => 'ADD',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0b0 Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_SP_i cclass=ADD_SP_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_i_T3 => {
  name       => 'ADDS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0b1 Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_SP_i cclass=ADD_SP_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_i_T4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1101 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=ADDW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_SP_i cclass=ADD_SP_i_t4',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_A1_RRX => {
  name       => 'ADD, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD mnemonic-shift-type=ADD-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_SP_r cclass=ADD_SP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_A1 => {
  name       => 'ADD, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADD mnemonic-shift-type=ADD-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_SP_r cclass=ADD_SP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_r_A1_RRX => {
  name       => 'ADDS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS mnemonic-shift-type=ADDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_SP_r cclass=ADD_SP_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_r_A1 => {
  name       => 'ADDS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADDS mnemonic-shift-type=ADDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_SP_r cclass=ADD_SP_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010001 op=0b00 DM:u=0bx Rs=0b1101 Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_SP_r cclass=ADD_SP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b00 D=0b1 Rm:u=0bxxxx Rd=0b101',
  cnsts      => 'Rm!=0b1101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_SP_r cclass=ADD_SP_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_T3_RRX => {
  name       => 'ADD, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b0 Rn=0b1101 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD mnemonic-shift-type=ADD-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_SP_r cclass=ADD_SP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADD_SP_r_T3 => {
  name       => 'ADD, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b0 Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADD mnemonic-shift-type=ADD-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_SP_r cclass=ADD_SP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_r_T3_RRX => {
  name       => 'ADDS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b1 Rn=0b1101 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS mnemonic-shift-type=ADDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_SP_r cclass=ADD_SP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADDS_SP_r_T3 => {
  name       => 'ADDS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0b1 Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADDS mnemonic-shift-type=ADDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_SP_r cclass=ADD_SP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING ADR_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADR cclass=ADR_a1',
  categories => 'GENERAL|MISC',
};

ENCODING ADR_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADR cclass=ADR_a2',
  categories => 'GENERAL|MISC',
};

ENCODING ADR_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b0 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADR cclass=ADR_t1',
  categories => 'GENERAL|MISC',
};

ENCODING ADR_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADR cclass=ADR_t2',
  categories => 'GENERAL|MISC',
};

ENCODING ADR_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADR cclass=ADR_t3',
  categories => 'GENERAL|MISC',
};

ENCODING AND_i_A1 => {
  name       => 'AND',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b000 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=AND',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=AND_i cclass=AND_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_i_A1 => {
  name       => 'ANDS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b000 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ANDS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=AND_i cclass=AND_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_i_T1 => {
  name       => 'AND',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0000 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=AND',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=AND_i cclass=AND_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_i_T1 => {
  name       => 'ANDS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0000 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ANDS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=AND_i cclass=AND_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_r_A1_RRX => {
  name       => 'AND, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=AND mnemonic-shift-type=AND-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=AND_r cclass=AND_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_r_A1 => {
  name       => 'AND, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=AND mnemonic-shift-type=AND-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=AND_r cclass=AND_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_r_A1_RRX => {
  name       => 'ANDS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ANDS mnemonic-shift-type=ANDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=AND_r cclass=AND_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_r_A1 => {
  name       => 'ANDS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ANDS mnemonic-shift-type=ANDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=AND_r cclass=AND_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0000 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=AND',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=AND_r cclass=AND_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_r_T2_RRX => {
  name       => 'AND, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0000 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=AND mnemonic-shift-type=AND-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=AND_r cclass=AND_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_r_T2 => {
  name       => 'AND, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0000 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=AND mnemonic-shift-type=AND-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=AND_r cclass=AND_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_r_T2_RRX => {
  name       => 'ANDS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0000 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ANDS mnemonic-shift-type=ANDS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=AND_r cclass=AND_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_r_T2 => {
  name       => 'ANDS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0000 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ANDS mnemonic-shift-type=ANDS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=AND_r cclass=AND_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ANDS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=ANDS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=AND_rr cclass=AND_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING AND_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=AND',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=AND_rr cclass=AND_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING B_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b101 H=0b0 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=B',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=B cclass=B_a1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING B_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101 cond:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=bcond16 page=B cclass=B_t1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING B_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11100 imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'iclass=b16 page=B cclass=B_t2',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING B_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 S:u=0bx cond:u=0bxxxx imm6:u=0bxxxxxx ig1=0b10 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bcond page=B cclass=B_t3',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING B_T4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b10 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=b page=B cclass=B_t4',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BFC_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn=0b1111',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BFC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfi page=BFC cclass=BFC_a1',
  categories => 'GENERAL|BITFIELD',
};

ENCODING BFC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b011 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ msb:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BFC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=BFC cclass=BFC_t1',
  categories => 'GENERAL|BITFIELD',
};

ENCODING BFI_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BFI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfi page=BFI cclass=BFI_a1',
  categories => 'GENERAL|INSERTION',
};

ENCODING BFI_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b011 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ msb:u=0bxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BFI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=BFI cclass=BFI_t1',
  categories => 'GENERAL|INSERTION',
};

ENCODING BIC_i_A1 => {
  name       => 'BIC',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b10 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BIC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=BIC_i cclass=BIC_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_i_A1 => {
  name       => 'BICS',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b10 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BICS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=BIC_i cclass=BIC_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_i_T1 => {
  name       => 'BIC',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0001 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BIC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=BIC_i cclass=BIC_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_i_T1 => {
  name       => 'BICS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0001 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BICS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=BIC_i cclass=BIC_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_r_A1_RRX => {
  name       => 'BIC, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BIC mnemonic-shift-type=BIC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=BIC_r cclass=BIC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_r_A1 => {
  name       => 'BIC, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BIC mnemonic-shift-type=BIC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=BIC_r cclass=BIC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_r_A1_RRX => {
  name       => 'BICS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BICS mnemonic-shift-type=BICS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=BIC_r cclass=BIC_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_r_A1 => {
  name       => 'BICS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BICS mnemonic-shift-type=BICS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=BIC_r cclass=BIC_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1110 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BIC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=BIC_r cclass=BIC_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_r_T2_RRX => {
  name       => 'BIC, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0001 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BIC mnemonic-shift-type=BIC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=BIC_r cclass=BIC_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_r_T2 => {
  name       => 'BIC, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0001 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BIC mnemonic-shift-type=BIC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=BIC_r cclass=BIC_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_r_T2_RRX => {
  name       => 'BICS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0001 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BICS mnemonic-shift-type=BICS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=BIC_r cclass=BIC_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_r_T2 => {
  name       => 'BICS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0001 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BICS mnemonic-shift-type=BICS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=BIC_r cclass=BIC_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING BICS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=BICS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=BIC_rr cclass=BIC_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING BIC_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=BIC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=BIC_rr cclass=BIC_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING BKPT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BKPT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=BKPT cclass=BKPT_a1',
  exceptions => 'SOFTWAREBREAKPOINT',
  categories => 'GENERAL|DEBUG',
};

ENCODING BKPT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111110 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BKPT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=bkpt16 page=BKPT cclass=BKPT_t1',
  exceptions => 'SOFTWAREBREAKPOINT',
  categories => 'GENERAL|DEBUG',
};

ENCODING BL_i_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b101 H=0b1 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=BL_i cclass=BL_i_a1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BL_i_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0b1111 ig0=0b101 H:u=0bx imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=BLX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=BL_i cclass=BL_i_a2',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BL_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bl page=BL_i cclass=BL_i_t1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BL_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 S:u=0bx imm10H:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b0 J2:u=0bx imm10L:u=0bxxxxxxxxxx H:u=0bx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BLX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=blx page=BL_i cclass=BL_i_t2',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BLX_r_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BLX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=blx_reg page=BLX_r cclass=BLX_r_a1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BLX_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01000111 L=0b1 Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BLX',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=bx16 page=BLX_r cclass=BLX_r_t1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=bx_reg page=BX cclass=BX_a1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01000111 L=0b0 Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BX',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=bx16 page=BX cclass=BX_t1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BXJ_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0010 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BXJ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=bxj_reg page=BXJ cclass=BXJ_a1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING BXJ_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111100 Rm:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BXJ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bx_jaz page=BXJ cclass=BXJ_t1',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING CBNZ_T1 => {
  name       => 'CBNZ',
  diagram    => 'ig0=0b1011 op:u=0b1 ig1=0b0 i:u=0bx ig2=0b1 imm5:u=0bxxxxx Rn:u=0bxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CBNZ',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cbznz16 page=CBNZ cclass=CBNZ_t1',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CBZ_T1 => {
  name       => 'CBZ',
  diagram    => 'ig0=0b1011 op:u=0b0 ig1=0b0 i:u=0bx ig2=0b1 imm5:u=0bxxxxx Rn:u=0bxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CBZ',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cbznz16 page=CBNZ cclass=CBNZ_t1',
  categories => 'GENERAL|BRANCH|CONDITIONALLY|COMPARISON',
};

ENCODING CLREX_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0001 option=0bSSSS',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CLREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=CLREX cclass=CLREX_a1',
  categories => 'GENERAL',
};

ENCODING CLREX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0010 option=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CLREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=CLREX cclass=CLREX_t1',
  categories => 'GENERAL',
};

ENCODING CLZ_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bS ig2=0bS ig3=0bS ig4=0bS Rd:u=0bxxxx ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CLZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=clz page=CLZ cclass=CLZ_a1',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CLZ_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b011 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CLZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CLZ cclass=CLZ_t1',
  categories => 'GENERAL|BITFIELD',
};

ENCODING CMN_i_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=CMN_i cclass=CMN_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=CMN_i cclass=CMN_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_r_A1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN mnemonic-shift-type=CMN-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMN_r cclass=CMN_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_r_A1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => '!(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN mnemonic-shift-type=CMN-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMN_r cclass=CMN_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1011 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=CMN_r cclass=CMN_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_r_T2_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd=0b1111 imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMN mnemonic-shift-type=CMN-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMN_r cclass=CMN_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_r_T2 => {
  name       => 'Shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMN mnemonic-shift-type=CMN-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMN_r cclass=CMN_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMN_rr_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=CMN_rr cclass=CMN_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_i_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=CMP_i cclass=CMP_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b001 op=0b01 Rn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=CMP_i cclass=CMP_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=CMP_i cclass=CMP_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_A1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP mnemonic-shift-type=CMP-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMP_r cclass=CMP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_A1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => '!(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP mnemonic-shift-type=CMP-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMP_r cclass=CMP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1010 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=CMP_r cclass=CMP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b01 N:u=0bx Rm:u=0bxxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=CMP_r cclass=CMP_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_T3_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1101 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd=0b1111 imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=CMP mnemonic-shift-type=CMP-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMP_r cclass=CMP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_r_T3 => {
  name       => 'Shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1101 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=CMP mnemonic-shift-type=CMP-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMP_r cclass=CMP_r_t3',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CMP_rr_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=CMP_rr cclass=CMP_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING CPS_A1_AS => {
  name       => 'CPS',
  diagram    => 'ig0=0b111100010000 imod:u=0b00 M:u=0b1 op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b00 && M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CPS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS cclass=CPS_a1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSID_A1_AS => {
  name       => 'CPSID',
  diagram    => 'ig0=0b111100010000 imod:u=0b11 M:u=0b0 op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b11 && M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CPSID',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS cclass=CPS_a1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSID_A1_ASM => {
  name       => 'CPSID',
  diagram    => 'ig0=0b111100010000 imod:u=0b11 M:u=0b1 op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b11 && M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CPSID',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS cclass=CPS_a1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSIE_A1_AS => {
  name       => 'CPSIE',
  diagram    => 'ig0=0b111100010000 imod:u=0b10 M:u=0b0 op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b10 && M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CPSIE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS cclass=CPS_a1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSIE_A1_ASM => {
  name       => 'CPSIE',
  diagram    => 'ig0=0b111100010000 imod:u=0b10 M:u=0b1 op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b10 && M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CPSIE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS cclass=CPS_a1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSID_T1_AS => {
  name       => 'CPSID',
  diagram    => 'ig0=0b1011011001 op=0b1 im:u=0b1 ig1=0bZ A:u=0bx I:u=0bx F:u=0bx',
  bitdiffs   => 'im == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CPSID',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cps16 page=CPS cclass=CPS_t1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSIE_T1_AS => {
  name       => 'CPSIE',
  diagram    => 'ig0=0b1011011001 op=0b1 im:u=0b0 ig1=0bZ A:u=0bx I:u=0bx F:u=0bx',
  bitdiffs   => 'im == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CPSIE',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cps16 page=CPS cclass=CPS_t1',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPS_T2_AS => {
  name       => 'CPS',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0b00 M:u=0b1 A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b00 && M == 0b1',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CPS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS cclass=CPS_t2',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSID_T2_AS => {
  name       => 'CPSID',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0b11 M:u=0b0 A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b11 && M == 0b0',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CPSID',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS cclass=CPS_t2',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSID_T2_ASM => {
  name       => 'CPSID',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0b11 M:u=0b1 A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b11 && M == 0b1',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CPSID',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS cclass=CPS_t2',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSIE_T2_AS => {
  name       => 'CPSIE',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0b10 M:u=0b0 A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b10 && M == 0b0',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CPSIE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS cclass=CPS_t2',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CPSIE_T2_ASM => {
  name       => 'CPSIE',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0b10 M:u=0b1 A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  bitdiffs   => 'imod == 0b10 && M == 0b1',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CPSIE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS cclass=CPS_t2',
  pstate     => 'A=CZ EL=CR F=CZ I=CZ',
  categories => 'SYSTEM',
};

ENCODING CRC32B_A1 => {
  name       => 'CRC32B',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b00 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b0 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b00',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32B',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32 cclass=CRC32_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32H_A1 => {
  name       => 'CRC32H',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b01 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b0 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b01',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32H',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32 cclass=CRC32_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32W_A1 => {
  name       => 'CRC32W',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b10 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b0 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b10',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32W',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32 cclass=CRC32_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32B_T1 => {
  name       => 'CRC32B',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b0 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b00 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b00',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32B',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32 cclass=CRC32_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32H_T1 => {
  name       => 'CRC32H',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b0 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b01 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b01',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32H',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32 cclass=CRC32_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32W_T1 => {
  name       => 'CRC32W',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b0 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b10 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b10',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32W',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32 cclass=CRC32_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CB_A1 => {
  name       => 'CRC32CB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b00 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b1 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b00',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32CB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32C cclass=CRC32C_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CH_A1 => {
  name       => 'CRC32CH',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b01 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b1 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b01',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32CH',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32C cclass=CRC32C_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CW_A1 => {
  name       => 'CRC32CW',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0b10 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b1 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sz == 0b10',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CRC32CW',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32C cclass=CRC32C_a1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CB_T1 => {
  name       => 'CRC32CB',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b1 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b00 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b00',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32CB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32C cclass=CRC32C_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CH_T1 => {
  name       => 'CRC32CH',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b1 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b01 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b01',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32CH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32C cclass=CRC32C_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CRC32CW_T1 => {
  name       => 'CRC32CW',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b1 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0b10 Rm:u=0bxxxx',
  bitdiffs   => 'sz == 0b10',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CRC32CW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32C cclass=CRC32C_t1',
  categories => 'GENERAL|CRYPTO',
};

ENCODING CSDB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CSDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=CSDB cclass=CSDB_a1',
  categories => 'SYSTEM',
};

ENCODING CSDB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0100',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CSDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=CSDB cclass=CSDB_t1',
  categories => 'SYSTEM',
};

ENCODING DBG_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bZZZZ1111 option:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DBG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=DBG cclass=DBG_a1',
  categories => 'GENERAL|DEBUG|HINT',
};

ENCODING DBG_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b1111 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DBG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=DBG cclass=DBG_t1',
  categories => 'GENERAL|DEBUG|HINT',
};

ENCODING DCPS1_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b01',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS1 cclass=DCPS1_t1',
  pstate     => 'E=CW EL=RCW M=CZ PAN=CS SP=CS UAO=CZ nRW=CZ',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DCPS2_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b10',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS2 cclass=DCPS2_t1',
  pstate     => 'E=CW EL=CW PAN=CS SP=CS UAO=CZ nRW=CZ',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DCPS3_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS3 cclass=DCPS3_t1',
  pstate     => 'E=CW EL=CW M=CZ PAN=CZ SP=CS UAO=CZ nRW=CZ',
  categories => 'SYSTEM|DEBUG',
};

ENCODING DMB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0101 option:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DMB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=DMB cclass=DMB_a1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING DMB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0101 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DMB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=DMB cclass=DMB_t1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING DSB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option:u=0bxxxx',
  cnsts      => 'option!=0b0x00',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=DSB cclass=DSB_a1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING DSB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option:u=0bxxxx',
  cnsts      => 'option!=0b0x00',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=DSB cclass=DSB_t1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING EOR_i_A1 => {
  name       => 'EOR',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b001 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EOR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=EOR_i cclass=EOR_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_i_A1 => {
  name       => 'EORS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b001 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EORS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=EOR_i cclass=EOR_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_i_T1 => {
  name       => 'EOR',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0100 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=EOR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=EOR_i cclass=EOR_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_i_T1 => {
  name       => 'EORS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0100 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=EORS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=EOR_i cclass=EOR_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_r_A1_RRX => {
  name       => 'EOR, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EOR mnemonic-shift-type=EOR-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=EOR_r cclass=EOR_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_r_A1 => {
  name       => 'EOR, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EOR mnemonic-shift-type=EOR-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=EOR_r cclass=EOR_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_r_A1_RRX => {
  name       => 'EORS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EORS mnemonic-shift-type=EORS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=EOR_r cclass=EOR_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_r_A1 => {
  name       => 'EORS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=EORS mnemonic-shift-type=EORS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=EOR_r cclass=EOR_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0001 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=EOR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=EOR_r cclass=EOR_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_r_T2_RRX => {
  name       => 'EOR, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0100 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=EOR mnemonic-shift-type=EOR-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=EOR_r cclass=EOR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_r_T2 => {
  name       => 'EOR, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0100 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=EOR mnemonic-shift-type=EOR-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=EOR_r cclass=EOR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_r_T2_RRX => {
  name       => 'EORS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0100 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=EORS mnemonic-shift-type=EORS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=EOR_r cclass=EOR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_r_T2 => {
  name       => 'EORS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0100 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=EORS mnemonic-shift-type=EORS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=EOR_r cclass=EOR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING EORS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=EORS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=EOR_rr cclass=EOR_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING EOR_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=EOR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=EOR_rr cclass=EOR_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ERET_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0b0110 ig14=0bS ig15=0bS ig16=0bS ig17=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ERET',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=eret page=ERET cclass=ERET_a1',
  pstate     => 'A=CW C=U EL=CR F=CW GE=CW I=CW IT=CZ M=CR N=U Q=CW SS=CW T=CS V=U Z=U',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING ERET_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111101 Rn=0b1110 ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8=0b00000000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ERET',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=eret page=ERET cclass=ERET_t1',
  pstate     => 'A=CW C=CW EL=CR F=CW GE=CW I=CW IT=CZ M=CR N=CW Q=CW SS=CW T=CS V=CW Z=CW',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING ESB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010000',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ESB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=ESB cclass=ESB_a1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING ESB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ESB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=ESB cclass=ESB_t1',
  pstate     => 'EL=CR',
  categories => 'SYSTEM',
};

ENCODING HLT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=HLT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=HLT cclass=HLT_a1',
  categories => 'GENERAL|DEBUG',
};

ENCODING HLT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011101010 imm6:u=0bxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=HLT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hlt16 page=HLT cclass=HLT_t1',
  categories => 'GENERAL|DEBUG',
};

ENCODING HVC_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=HVC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=HVC cclass=HVC_a1',
  exceptions => 'HYPERVISORCALL',
  pstate     => 'EL=CR',
  categories => 'GENERAL|CALL',
};

ENCODING HVC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110111111 o1=0b0 imm4:u=0bxxxx ig1=0b10 o2=0b0 ig2=0b0 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=HVC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=HVC cclass=HVC_t1',
  exceptions => 'HYPERVISORCALL',
  pstate     => 'EL=CR',
  categories => 'GENERAL|CALL',
};

ENCODING ISB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0110 option:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ISB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=ISB cclass=ISB_a1',
  categories => 'SYSTEM',
};

ENCODING ISB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0110 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ISB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=ISB cclass=ISB_t1',
  categories => 'SYSTEM',
};

ENCODING IT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 firstcond:u=0bxxxx mask:u=0bxxxx',
  cnsts      => 'mask!=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=IT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=it16 page=IT cclass=IT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'IT=CW',
  categories => 'GENERAL|CONTROL_FLOW',
};

ENCODING LDA_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDA cclass=LDA_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDA_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDA cclass=LDA_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAB cclass=LDAB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAB cclass=LDAB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEX cclass=LDAEX_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEX cclass=LDAEX_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXB cclass=LDAEXB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXB cclass=LDAEXB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXD cclass=LDAEXD_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op=0b1 sz=0b11 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXD cclass=LDAEXD_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXH cclass=LDAEXH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAEXH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXH cclass=LDAEXH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAH cclass=LDAH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDAH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAH cclass=LDAH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_i cclass=LDC_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0bx D=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_i cclass=LDC_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_i cclass=LDC_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_A1_unind => {
  name       => 'Unindexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1 && W == 0b0',
  docvars    => 'address-form=unindexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_i cclass=LDC_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_T1_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_i cclass=LDC_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_T1_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b0 U:u=0bx D=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_i cclass=LDC_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_T1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_i cclass=LDC_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_i_T1_unind => {
  name       => 'Unindexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b0 U:u=0b1 D=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1 && W == 0b0',
  docvars    => 'address-form=unindexed armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_i cclass=LDC_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn=0b1111 CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 P:U:W!=0b000',
  bitdiffs   => '!(P == 0b0 && U == 0b0 && W == 0b0)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_l cclass=LDC_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDC_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn=0b1111 CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'P:U:W!=0b000',
  bitdiffs   => '!(P == 0b0 && U == 0b0 && W == 0b0)',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_l cclass=LDC_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDM_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM cclass=LDM_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDM_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1100 L=0b1 Rn:u=0bxxx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldstm16 page=LDM cclass=LDM_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDM_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=LDM cclass=LDM_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDM_e_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W:u=0bx L=0b1 Rn:u=0bxxxx ig1=0b1 register_list:u=0bxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM_e cclass=LDM_e_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|RET|EXCEPTION|DATA_TRANSFER|LOAD',
};

ENCODING LDM_u_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W=0bZ L=0b1 Rn:u=0bxxxx ig1=0b0 register_list:u=0bxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM_u cclass=LDM_u_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDMDA_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b0 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMDA cclass=LDMDA_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDMDB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMDB cclass=LDMDB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDMDB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=LDMDB cclass=LDMDB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDMIB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b1 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMIB cclass=LDMIB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_i cclass=LDR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0bx o2=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_i cclass=LDR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b0 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_i cclass=LDR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b0 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=LDR_i cclass=LDR_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1001 L=0b1 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_sp page=LDR_i cclass=LDR_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110001 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDR_i cclass=LDR_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T4_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=LDR_i cclass=LDR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T4_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=LDR_i cclass=LDR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_i_T4_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=LDR_i cclass=LDR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 P:W!=0b01',
  bitdiffs   => '!(P == 0b0 && W == 0b1)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_l cclass=LDR_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01001 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldlit16 page=LDR_l cclass=LDR_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_l_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b10 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDR_l cclass=LDR_l_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDR_r cclass=LDR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b0 U:u=0bx o2=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDR_r cclass=LDR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b0 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDR_r cclass=LDR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b0 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDR_r cclass=LDR_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDR_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDR_r cclass=LDR_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_i cclass=LDRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0bx o2=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_i cclass=LDRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b1 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_i cclass=LDRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b1 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=LDRB_i cclass=LDRB_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDRB_i cclass=LDRB_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_T3_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  bitdiffs   => 'Rt != 0b1111 && P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=LDRB_i cclass=LDRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_T3_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=LDRB_i cclass=LDRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_i_T3_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=LDRB_i cclass=LDRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 P:W!=0b01',
  bitdiffs   => '!(P == 0b0 && W == 0b1)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_l cclass=LDRB_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b00 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDRB_l cclass=LDRB_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRB_r cclass=LDRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b0 U:u=0bx o2=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRB_r cclass=LDRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b1 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRB_r cclass=LDRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b1 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRB_r cclass=LDRB_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRB_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDRB_r cclass=LDRB_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRBT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRBT cclass=LDRBT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRBT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRBT cclass=LDRBT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRBT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRBT cclass=LDRBT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_i cclass=LDRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_i cclass=LDRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_i cclass=LDRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_T1_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b1110100 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_imm page=LDRD_i cclass=LDRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_T1_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b1110100 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b1 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_post page=LDRD_i cclass=LDRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_i_T1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b1110100 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_pre page=LDRD_i cclass=LDRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0bS U:u=0bx ig1=0b1 W=0bZ o1=0b0 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_l cclass=LDRD_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L=0b1 ig2=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'P:W!=0b00',
  bitdiffs   => '!(P == 0b0 && W == 0b0)',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=lddlit page=LDRD_l cclass=LDRD_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRD_r cclass=LDRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRD_r cclass=LDRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRD_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRD_r cclass=LDRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREX cclass=LDREX_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000010 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rd=0bSSSS imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex page=LDREX cclass=LDREX_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXB cclass=LDREXB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXB cclass=LDREXB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXD cclass=LDREXD_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz=0b11 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXD cclass=LDREXD_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXH cclass=LDREXH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDREXH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXH cclass=LDREXH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_i cclass=LDRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_i cclass=LDRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_i cclass=LDRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1000 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldsth16_imm page=LDRH_i cclass=LDRH_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDRH_i cclass=LDRH_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_T3_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  bitdiffs   => 'Rt != 0b1111 && P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=LDRH_i cclass=LDRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_T3_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=LDRH_i cclass=LDRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_i_T3_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=LDRH_i cclass=LDRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 P:W!=0b01',
  bitdiffs   => '!(P == 0b0 && W == 0b1)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_l cclass=LDRH_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b01 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDRH_l cclass=LDRH_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRH_r cclass=LDRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRH_r cclass=LDRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRH_r cclass=LDRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b0 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRH_r cclass=LDRH_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRH_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDRH_r cclass=LDRH_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRHT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRHT cclass=LDRHT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRHT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRHT cclass=LDRHT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRHT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRHT cclass=LDRHT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRSB_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_i cclass=LDRSB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_i cclass=LDRSB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_i cclass=LDRSB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=LDRSB_i cclass=LDRSB_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_T2_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  bitdiffs   => 'Rt != 0b1111 && P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_nimm page=LDRSB_i cclass=LDRSB_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_T2_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_post page=LDRSB_i cclass=LDRSB_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_i_T2_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pre page=LDRSB_i cclass=LDRSB_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 P:W!=0b01',
  bitdiffs   => '!(P == 0b0 && W == 0b1)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_l cclass=LDRSB_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b00 ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=LDRSB_l cclass=LDRSB_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSB_r cclass=LDRSB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSB_r cclass=LDRSB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSB_r cclass=LDRSB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b1 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRSB_r cclass=LDRSB_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSB_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=LDRSB_r cclass=LDRSB_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSBT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSBT cclass=LDRSBT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSBT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSBT cclass=LDRSBT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSBT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_unpriv page=LDRSBT cclass=LDRSBT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_i cclass=LDRSH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_i cclass=LDRSH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_i cclass=LDRSH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=LDRSH_i cclass=LDRSH_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_T2_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  bitdiffs   => 'Rt != 0b1111 && P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_nimm page=LDRSH_i cclass=LDRSH_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_T2_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_post page=LDRSH_i cclass=LDRSH_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_i_T2_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pre page=LDRSH_i cclass=LDRSH_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_l_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 P:W!=0b01',
  bitdiffs   => '!(P == 0b0 && W == 0b1)',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_l cclass=LDRSH_l_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b01 ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=LDRSH_l cclass=LDRSH_l_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSH_r cclass=LDRSH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSH_r cclass=LDRSH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSH_r cclass=LDRSH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b1 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRSH_r cclass=LDRSH_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSH_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=LDRSH_r cclass=LDRSH_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSHT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSHT cclass=LDRSHT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSHT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSHT cclass=LDRSHT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRSHT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_unpriv page=LDRSHT cclass=LDRSHT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|SIGNED|LOAD',
};

ENCODING LDRT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRT cclass=LDRT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRT cclass=LDRT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING LDRT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRT cclass=LDRT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|LOAD',
};

ENCODING MCR_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L=0b0 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig1=0b1 CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MCR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as iclass=movcpgp32 page=MCR cclass=MCR_a1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MCR_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b1110 opc1:u=0bxxx L=0b0 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MCR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf iclass=cp_mov32 page=MCR cclass=MCR_t1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MCRR_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 L=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MCRR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=movcpgp64 page=MCRR cclass=MCRR_a1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MCRR_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b11000 D=0b1 ig2=0b0 L=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MCRR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_mov64 page=MCRR cclass=MCRR_t1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MLAS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b1 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MLAS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MLA cclass=MLA_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MLA_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MLA cclass=MLA_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MLA_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MLA cclass=MLA_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MLS_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MLS cclass=MLS_a1',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MLS_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MLS cclass=MLS_t1',
  categories => 'GENERAL|CARITHMETIC',
};

ENCODING MOV_i_A1 => {
  name       => 'MOV',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MOV_i cclass=MOV_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_i_A1 => {
  name       => 'MOVS',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MOV_i cclass=MOV_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_i_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 H=0b0 ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=MOVW',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movw page=MOV_i cclass=MOV_i_a2',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b001 op=0b00 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=MOV_i cclass=MOV_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_i_T2 => {
  name       => 'MOV',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0b0 Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MOV_i cclass=MOV_i_t2',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_i_T2 => {
  name       => 'MOVS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0b1 Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MOV_i cclass=MOV_i_t2',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_i_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=movw page=MOV_i cclass=MOV_i_t3',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_A1_RRX => {
  name       => 'MOV, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MOV_r cclass=MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_A1 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MOV_r cclass=MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_r_A1_RRX => {
  name       => 'MOVS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MOV_r cclass=MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_r_A1 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MOV_r cclass=MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010001 op=0b10 D:u=0bx Rm:u=0bxxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=MOV_r cclass=MOV_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0bxx imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'op!=0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=MOV_r cclass=MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_T3_RRX => {
  name       => 'MOV, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MOV_r cclass=MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_r_T3 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MOV_r cclass=MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_r_T3_RRX => {
  name       => 'MOVS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MOV_r cclass=MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_r_T3 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MOV_r cclass=MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MOV_rr cclass=MOV_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MOV_rr cclass=MOV_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_T1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'ig0=0b010000 op:u=0b0100 Rs:u=0bxxx Rdm:u=0bxxx',
  bitdiffs   => 'op == 0b0100',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-asr shift-type=asr',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MOV_rr cclass=MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_T1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'ig0=0b010000 op:u=0b0010 Rs:u=0bxxx Rdm:u=0bxxx',
  bitdiffs   => 'op == 0b0010',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsl shift-type=lsl',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MOV_rr cclass=MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_T1_LSR => {
  name       => 'Logical shift right',
  diagram    => 'ig0=0b010000 op:u=0b0011 Rs:u=0bxxx Rdm:u=0bxxx',
  bitdiffs   => 'op == 0b0011',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsr shift-type=lsr',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MOV_rr cclass=MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_T1_ROR => {
  name       => 'Rotate right',
  diagram    => 'ig0=0b010000 op:u=0b0111 Rs:u=0bxxx Rdm:u=0bxxx',
  bitdiffs   => 'op == 0b0111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-ror shift-type=ror',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MOV_rr cclass=MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVS_rr_T2 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0bxx S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T2 cond-setting=s instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=MOV_rr cclass=MOV_rr_t2',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOV_rr_T2 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0bxx S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T2 cond-setting=no-s instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=MOV_rr cclass=MOV_rr_t2',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 H=0b1 ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movw page=MOVT cclass=MOVT_a1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MOVT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=movw page=MOVT cclass=MOVT_t1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRC_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L=0b1 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig1=0b1 CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as iclass=movcpgp32 page=MRC cclass=MRC_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b1110 opc1:u=0bxxx L=0b1 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf iclass=cp_mov32 page=MRC cclass=MRC_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRRC_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 L=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRRC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=movcpgp64 page=MRRC cclass=MRRC_a1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRRC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b11000 D=0b1 ig2=0b0 L=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRRC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_mov64 page=MRRC cclass=MRRC_t1',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRS_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b0 ig2=0b0 mask=0bSSSS Rd:u=0bxxxx ig3=0bZ ig4=0bZ B=0b0 m=0bZ ig5=0b0000 Rn=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MRS cclass=MRS_a1',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRS_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011111 R:u=0bx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 Rd:u=0bxxxx ig8=0bZ ig9=0bZ ig10=0b0 ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=mrs_spec page=MRS cclass=MRS_t1',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRS_br_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b0 ig2=0b0 M1:u=0bxxxx Rd:u=0bxxxx ig3=0bZ ig4=0bZ B=0b1 M:u=0bx ig5=0b0000 Rn=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MRS_br cclass=MRS_br_a1',
  exceptions => 'UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MRS_br_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011111 R:u=0bx M1:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 Rd:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 M:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=mrs_bank page=MRS_br cclass=MRS_br_t1',
  exceptions => 'UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MSR_br_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b1 ig2=0b0 M1:u=0bxxxx Rd=0bSSSS ig3=0bZ ig4=0bZ B=0b1 M:u=0bx ig5=0b0000 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MSR_br cclass=MSR_br_a1',
  exceptions => 'UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MSR_br_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 M1:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 M:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=msr_bank page=MSR_br cclass=MSR_br_t1',
  exceptions => 'UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MSR_i_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R:u=0bx ig1=0b10 mask:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 R:mask!=0b00000',
  bitdiffs   => '!(R == 0b0 && mask == 0b0000)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=MSR_i cclass=MSR_i_a1',
  pstate     => 'M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MSR_r_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b1 ig2=0b0 mask:u=0bxxxx Rd=0bSSSS ig3=0bZ ig4=0bZ B=0b0 m=0bZ ig5=0b0000 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MSR_r cclass=MSR_r_a1',
  pstate     => 'M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MSR_r_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 mask:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b0 ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=msr_spec page=MSR_r cclass=MSR_r_t1',
  pstate     => 'M=CR',
  categories => 'GENERAL|DATA_TRANSFER',
};

ENCODING MULS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b1 Rd:u=0bxxxx RdLo=0bZZZZ Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MULS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MUL cclass=MUL_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING MUL_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0b0 Rd:u=0bxxxx RdLo=0bZZZZ Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MUL cclass=MUL_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING MUL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1101 Rn:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MUL',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MUL cclass=MUL_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING MUL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MUL cclass=MUL_t2',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING MVN_i_A1 => {
  name       => 'MVN',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b11 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MVN_i cclass=MVN_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_i_A1 => {
  name       => 'MVNS',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b11 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVNS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MVN_i cclass=MVN_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_i_T1 => {
  name       => 'MVN',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0b0 Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MVN_i cclass=MVN_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_i_T1 => {
  name       => 'MVNS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0b1 Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MVNS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MVN_i cclass=MVN_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_r_A1_RRX => {
  name       => 'MVN, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVN mnemonic-shift-type=MVN-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MVN_r cclass=MVN_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_r_A1 => {
  name       => 'MVN, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVN mnemonic-shift-type=MVN-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MVN_r cclass=MVN_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_r_A1_RRX => {
  name       => 'MVNS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVNS mnemonic-shift-type=MVNS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MVN_r cclass=MVN_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_r_A1 => {
  name       => 'MVNS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MVNS mnemonic-shift-type=MVNS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MVN_r cclass=MVN_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1111 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MVN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MVN_r cclass=MVN_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_r_T2_RRX => {
  name       => 'MVN, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MVN mnemonic-shift-type=MVN-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MVN_r cclass=MVN_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_r_T2 => {
  name       => 'MVN, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MVN mnemonic-shift-type=MVN-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MVN_r cclass=MVN_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_r_T2_RRX => {
  name       => 'MVNS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MVNS mnemonic-shift-type=MVNS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MVN_r cclass=MVN_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_r_T2 => {
  name       => 'MVNS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MVNS mnemonic-shift-type=MVNS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MVN_r cclass=MVN_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVNS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MVNS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MVN_rr cclass=MVN_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING MVN_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MVN_rr cclass=MVN_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING NOP_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000000',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=NOP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=NOP cclass=NOP_a1',
  categories => 'GENERAL|NOP',
};

ENCODING NOP_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0000 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=NOP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=NOP cclass=NOP_t1',
  categories => 'GENERAL|NOP',
};

ENCODING NOP_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0000',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=NOP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=NOP cclass=NOP_t2',
  categories => 'GENERAL|NOP',
};

ENCODING ORNS_i_T1 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 cond-setting=s instr-class=general isa=T32 mnemonic=ORNS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORN_i cclass=ORN_i_t1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORN_i_T1 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 cond-setting=no-s instr-class=general isa=T32 mnemonic=ORN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORN_i cclass=ORN_i_t1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORN_r_T1_RRX => {
  name       => 'ORN, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 cond-setting=no-s instr-class=general isa=T32 mnemonic=ORN mnemonic-shift-type=ORN-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORN_r cclass=ORN_r_t1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORN_r_T1 => {
  name       => 'ORN, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 cond-setting=no-s instr-class=general isa=T32 mnemonic=ORN mnemonic-shift-type=ORN-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORN_r cclass=ORN_r_t1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORNS_r_T1_RRX => {
  name       => 'ORNS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 cond-setting=s instr-class=general isa=T32 mnemonic=ORNS mnemonic-shift-type=ORNS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORN_r cclass=ORN_r_t1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORNS_r_T1 => {
  name       => 'ORNS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 cond-setting=s instr-class=general isa=T32 mnemonic=ORNS mnemonic-shift-type=ORNS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORN_r cclass=ORN_r_t1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_i_A1 => {
  name       => 'ORR',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b00 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=ORR_i cclass=ORR_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_i_A1 => {
  name       => 'ORRS',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b00 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=ORR_i cclass=ORR_i_a1',
  pstate     => 'C=W N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_i_T1 => {
  name       => 'ORR',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ORR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORR_i cclass=ORR_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_i_T1 => {
  name       => 'ORRS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ORRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORR_i cclass=ORR_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_r_A1_RRX => {
  name       => 'ORR, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORR mnemonic-shift-type=ORR-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ORR_r cclass=ORR_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_r_A1 => {
  name       => 'ORR, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORR mnemonic-shift-type=ORR-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ORR_r cclass=ORR_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_r_A1_RRX => {
  name       => 'ORRS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORRS mnemonic-shift-type=ORRS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ORR_r cclass=ORR_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_r_A1 => {
  name       => 'ORRS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ORRS mnemonic-shift-type=ORRS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ORR_r cclass=ORR_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1100 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ORR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ORR_r cclass=ORR_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_r_T2_RRX => {
  name       => 'ORR, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ORR mnemonic-shift-type=ORR-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORR_r cclass=ORR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_r_T2 => {
  name       => 'ORR, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ORR mnemonic-shift-type=ORR-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORR_r cclass=ORR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_r_T2_RRX => {
  name       => 'ORRS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ORRS mnemonic-shift-type=ORRS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORR_r cclass=ORR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_r_T2 => {
  name       => 'ORRS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ORRS mnemonic-shift-type=ORRS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORR_r cclass=ORR_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORRS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=ORRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ORR_rr cclass=ORR_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|BITWISE',
};

ENCODING ORR_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=ORR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ORR_rr cclass=ORR_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|BITWISE',
};

ENCODING PKHBT_A1 => {
  name       => 'PKHBT',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx tb:u=0b0 ig1=0b01 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'tb == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PKHBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=pack page=PKH cclass=PKH_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING PKHTB_A1 => {
  name       => 'PKHTB',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx tb:u=0b1 ig1=0b01 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'tb == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PKHTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=pack page=PKH cclass=PKH_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING PKHBT_T1 => {
  name       => 'PKHBT',
  diagram    => 'ig0=0b1110101 op1=0b0110 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx tb:u=0b0 T:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'tb == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PKHBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=PKH cclass=PKH_t1',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING PKHTB_T1 => {
  name       => 'PKHTB',
  diagram    => 'ig0=0b1110101 op1=0b0110 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx tb:u=0b1 T:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'tb == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PKHTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=PKH cclass=PKH_t1',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL',
};

ENCODING PLD_i_A1 => {
  name       => 'Preload read',
  diagram    => 'ig0=0b1111010 D=0b1 U:u=0bx R:u=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLD_i cclass=PLD_i_a1',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_i_A1 => {
  name       => 'Preload write',
  diagram    => 'ig0=0b1111010 D=0b1 U:u=0bx R:u=0b0 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLDW preload-type=pldw',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLD_i cclass=PLD_i_a1',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_i_T1 => {
  name       => 'Preload read',
  diagram    => 'ig0=0b111110001 ig1=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=PLD_i cclass=PLD_i_t1',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_i_T1 => {
  name       => 'Preload write',
  diagram    => 'ig0=0b111110001 ig1=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b1',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLDW preload-type=pldw',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=PLD_i cclass=PLD_i_t1',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_i_T2 => {
  name       => 'Preload read',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=PLD_i cclass=PLD_i_t2',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_i_T2 => {
  name       => 'Preload write',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b1',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=PLDW preload-type=pldw',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=PLD_i cclass=PLD_i_t2',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_l_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111010 D=0b1 U:u=0bx R=0bS ig1=0b01 Rn=0b1111 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLD_l cclass=PLD_l_a1',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_l_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx ig1=0b0 W=0bZ L=0b1 ig2=0b1111 Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=PLD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=PLD_l cclass=PLD_l_t1',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_r_A1 => {
  name       => 'Preload read, optional shift or rotate',
  diagram    => 'ig0=0b1111011 D=0b1 U:u=0bx R:u=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'imm5:stype!=0b0000011',
  bitdiffs   => 'R == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLD preload-type-shift-type=pld-shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLD_r cclass=PLD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_r_A1_RRX => {
  name       => 'Preload read, rotate right with extend',
  diagram    => 'ig0=0b1111011 D=0b1 U:u=0bx R:u=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLD preload-type-shift-type=pld-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLD_r cclass=PLD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_r_A1 => {
  name       => 'Preload write, optional shift or rotate',
  diagram    => 'ig0=0b1111011 D=0b1 U:u=0bx R:u=0b0 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'imm5:stype!=0b0000011',
  bitdiffs   => 'R == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLDW preload-type-shift-type=pldw-shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLD_r cclass=PLD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_r_A1_RRX => {
  name       => 'Preload write, rotate right with extend',
  diagram    => 'ig0=0b1111011 D=0b1 U:u=0bx R:u=0b0 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLDW preload-type-shift-type=pldw-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLD_r cclass=PLD_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLD_r_T1 => {
  name       => 'Preload read',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0b0 L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b0',
  docvars    => 'address-form=register-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=PLD_r cclass=PLD_r_t1',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLDW_r_T1 => {
  name       => 'Preload write',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0b1 L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'W == 0b1',
  docvars    => 'address-form=register-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLDW preload-type=pldw',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=PLD_r cclass=PLD_r_t1',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111010 D=0b0 U:u=0bx R=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLI_i cclass=PLI_i_a1',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=PLI_i cclass=PLI_i_t1',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_nimm page=PLI_i cclass=PLI_i_t2',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_i_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b00 ig1=0b11111 Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T3 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=PLI_i cclass=PLI_i_t3',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_r_A1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'ig0=0b1111011 D=0b0 U:u=0bx o2=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLI mnemonic-shift-type=PLI-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLI_r cclass=PLI_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_r_A1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'ig0=0b1111011 D=0b0 U:u=0bx o2=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'imm5:stype!=0b0000011',
  bitdiffs   => '!(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLI mnemonic-shift-type=PLI-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLI_r cclass=PLI_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING PLI_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=PLI_r cclass=PLI_r_t1',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|HINT',
};

ENCODING POP_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011 L=0b1 ig1=0b10 P:u=0bx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=POP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=pushpop16 page=POP cclass=POP_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
};

ENCODING PSSBB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option=0b0100',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=PSSBB cclass=PSSBB_a1',
  categories => 'SYSTEM',
};

ENCODING PSSBB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option=0b0100',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=PSSBB cclass=PSSBB_t1',
  categories => 'SYSTEM',
};

ENCODING PUSH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011 L=0b0 ig1=0b10 M:u=0bx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PUSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=pushpop16 page=PUSH cclass=PUSH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
};

ENCODING QADD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QADD cclass=QADD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QADD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QADD cclass=QADD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QADD16 cclass=QADD16_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QADD16 cclass=QADD16_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QADD8 cclass=QADD8_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QADD8 cclass=QADD8_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QASX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QASX cclass=QASX_a1',
  categories => 'GENERAL|CARITHMETIC|SATURATING',
};

ENCODING QASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QASX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QASX cclass=QASX_t1',
  categories => 'GENERAL|CARITHMETIC|SATURATING',
};

ENCODING QDADD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QDADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QDADD cclass=QDADD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QDADD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QDADD',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QDADD cclass=QDADD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QDSUB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QDSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QDSUB cclass=QDSUB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QDSUB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QDSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QDSUB cclass=QDSUB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSAX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSAX cclass=QSAX_a1',
  categories => 'GENERAL|CARITHMETIC|SATURATING',
};

ENCODING QSAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSAX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSAX cclass=QSAX_t1',
  categories => 'GENERAL|CARITHMETIC|SATURATING',
};

ENCODING QSUB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QSUB cclass=QSUB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSUB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QSUB cclass=QSUB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSUB16 cclass=QSUB16_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSUB16 cclass=QSUB16_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSUB8 cclass=QSUB8_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING QSUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSUB8 cclass=QSUB8_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING',
};

ENCODING RBIT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b1 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b0 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RBIT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=RBIT cclass=RBIT_a1',
  categories => 'GENERAL|MISC',
};

ENCODING RBIT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RBIT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=RBIT cclass=RBIT_t1',
  categories => 'GENERAL|MISC',
};

ENCODING REV_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b0 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b0 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REV cclass=REV_a1',
  categories => 'GENERAL|MISC',
};

ENCODING REV_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b00 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REV cclass=REV_t1',
  categories => 'GENERAL|MISC',
};

ENCODING REV_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REV cclass=REV_t2',
  categories => 'GENERAL|MISC',
};

ENCODING REV16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b0 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b1 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REV16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REV16 cclass=REV16_a1',
  categories => 'GENERAL|MISC',
};

ENCODING REV16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b01 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REV16',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REV16 cclass=REV16_t1',
  categories => 'GENERAL|MISC',
};

ENCODING REV16_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REV16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REV16 cclass=REV16_t2',
  categories => 'GENERAL|MISC',
};

ENCODING REVSH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b1 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b1 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REVSH cclass=REVSH_a1',
  categories => 'GENERAL|MISC|SIGNED',
};

ENCODING REVSH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b11 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REVSH cclass=REVSH_t1',
  categories => 'GENERAL|MISC|SIGNED',
};

ENCODING REVSH_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REVSH cclass=REVSH_t2',
  categories => 'GENERAL|MISC|SIGNED',
};

ENCODING RFEDA_A1_AS => {
  name       => 'Decrement After',
  diagram    => 'ig0=0b1111100 P:u=0b0 U:u=0b0 S=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx op=0bZZZZSZSZZZZ mode=0bZZZZZ',
  bitdiffs   => 'P == 0b0 && U == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=dec-after mnemonic=RFEDA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=RFE cclass=RFE_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RFEDB_A1_AS => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1111100 P:u=0b1 U:u=0b0 S=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx op=0bZZZZSZSZZZZ mode=0bZZZZZ',
  bitdiffs   => 'P == 0b1 && U == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=dec-before mnemonic=RFEDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=RFE cclass=RFE_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RFEIA_A1_AS => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1111100 P:u=0b0 U:u=0b1 S=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx op=0bZZZZSZSZZZZ mode=0bZZZZZ',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=inc-after mnemonic=RFE{IA}',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=RFE cclass=RFE_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RFEIB_A1_AS => {
  name       => 'Increment Before',
  diagram    => 'ig0=0b1111100 P:u=0b1 U:u=0b1 S=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx op=0bZZZZSZSZZZZ mode=0bZZZZZ',
  bitdiffs   => 'P == 0b1 && U == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=inc-before mnemonic=RFEIB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=RFE cclass=RFE_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RFE_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b00 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P=0bS M=0bS register_list=0bZZZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 ldmstm-mode=dec-before mnemonic=RFEDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=RFE cclass=RFE_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RFE_T2_AS => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b11 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P=0bS M=0bS register_list=0bZZZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 ldmstm-mode=inc-after mnemonic=RFE{IA}',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=RFE cclass=RFE_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|RET|EXCEPTION',
};

ENCODING RSB_i_A1 => {
  name       => 'RSB',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b011 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSB_i cclass=RSB_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_i_A1 => {
  name       => 'RSBS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b011 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSBS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSB_i cclass=RSB_i_a1',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1001 Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=RSB_i cclass=RSB_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_i_T2 => {
  name       => 'RSB',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1110 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=RSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=RSB_i cclass=RSB_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_i_T2 => {
  name       => 'RSBS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1110 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=RSBS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=RSB_i cclass=RSB_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_r_A1_RRX => {
  name       => 'RSB, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSB mnemonic-shift-type=RSB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSB_r cclass=RSB_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_r_A1 => {
  name       => 'RSB, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSB mnemonic-shift-type=RSB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSB_r cclass=RSB_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_r_A1_RRX => {
  name       => 'RSBS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSBS mnemonic-shift-type=RSBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSB_r cclass=RSB_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_r_A1 => {
  name       => 'RSBS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSBS mnemonic-shift-type=RSBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSB_r cclass=RSB_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_r_T1_RRX => {
  name       => 'RSB, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1110 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSB mnemonic-shift-type=RSB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RSB_r cclass=RSB_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_r_T1 => {
  name       => 'RSB, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1110 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSB mnemonic-shift-type=RSB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RSB_r cclass=RSB_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_r_T1_RRX => {
  name       => 'RSBS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1110 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSBS mnemonic-shift-type=RSBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RSB_r cclass=RSB_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_r_T1 => {
  name       => 'RSBS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1110 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSBS mnemonic-shift-type=RSBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RSB_r cclass=RSB_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSBS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=RSBS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSB_rr cclass=RSB_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSB_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=RSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSB_rr cclass=RSB_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSC_i_A1 => {
  name       => 'RSC',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b111 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSC_i cclass=RSC_i_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSCS_i_A1 => {
  name       => 'RSCS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b111 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSC_i cclass=RSC_i_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSC_r_A1_RRX => {
  name       => 'RSC, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSC mnemonic-shift-type=RSC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSC_r cclass=RSC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSC_r_A1 => {
  name       => 'RSC, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSC mnemonic-shift-type=RSC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSC_r cclass=RSC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSCS_r_A1_RRX => {
  name       => 'RSCS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSCS mnemonic-shift-type=RSCS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSC_r cclass=RSC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSCS_r_A1 => {
  name       => 'RSCS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RSCS mnemonic-shift-type=RSCS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSC_r cclass=RSC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING RSCS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=RSCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSC_rr cclass=RSC_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING RSC_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=RSC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSC_rr cclass=RSC_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SADD16 cclass=SADD16_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SADD16 cclass=SADD16_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SADD8 cclass=SADD8_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SADD8 cclass=SADD8_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SASX cclass=SASX_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SASX cclass=SASX_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0111 option=0bZZZZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=SB cclass=SB_a1',
  categories => 'SYSTEM',
};

ENCODING SB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0111 option=0bZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=SB cclass=SB_t1',
  categories => 'SYSTEM',
};

ENCODING SBC_i_A1 => {
  name       => 'SBC',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b110 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SBC_i cclass=SBC_i_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_i_A1 => {
  name       => 'SBCS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b110 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SBC_i cclass=SBC_i_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_i_T1 => {
  name       => 'SBC',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1011 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SBC_i cclass=SBC_i_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_i_T1 => {
  name       => 'SBCS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1011 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBCS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SBC_i cclass=SBC_i_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_r_A1_RRX => {
  name       => 'SBC, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBC mnemonic-shift-type=SBC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SBC_r cclass=SBC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_r_A1 => {
  name       => 'SBC, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBC mnemonic-shift-type=SBC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SBC_r cclass=SBC_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_r_A1_RRX => {
  name       => 'SBCS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBCS mnemonic-shift-type=SBCS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SBC_r cclass=SBC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_r_A1 => {
  name       => 'SBCS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBCS mnemonic-shift-type=SBCS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SBC_r cclass=SBC_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0110 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=SBC_r cclass=SBC_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_r_T2_RRX => {
  name       => 'SBC, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1011 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SBC mnemonic-shift-type=SBC-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SBC_r cclass=SBC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_r_T2 => {
  name       => 'SBC, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1011 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SBC mnemonic-shift-type=SBC-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SBC_r cclass=SBC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_r_T2_RRX => {
  name       => 'SBCS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1011 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SBCS mnemonic-shift-type=SBCS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SBC_r cclass=SBC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_r_T2 => {
  name       => 'SBCS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1011 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SBCS mnemonic-shift-type=SBCS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SBC_r cclass=SBC_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBCS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=SBCS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SBC_rr cclass=SBC_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBC_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=SBC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SBC_rr cclass=SBC_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|CARRY',
};

ENCODING SBFX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111 U=0b0 ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBFX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfx page=SBFX cclass=SBFX_a1',
  categories => 'GENERAL|EXTRACTION|SIGNED',
};

ENCODING SBFX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b010 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBFX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SBFX cclass=SBFX_t1',
  categories => 'GENERAL|EXTRACTION|SIGNED',
};

ENCODING SDIV_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b001 Rd:u=0bxxxx Ra:u=0bSSSS Rm:u=0bxxxx op2=0b000 ig1=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SDIV cclass=SDIV_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SDIV_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b001 Rn:u=0bxxxx Ra:u=0bSSSS Rd:u=0bxxxx op2=0b1111 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SDIV cclass=SDIV_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SEL_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b1011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=selbytes page=SEL cclass=SEL_a1',
  pstate     => 'GE=CR',
  categories => 'GENERAL|SELECTION',
};

ENCODING SEL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=SEL cclass=SEL_t1',
  pstate     => 'GE=CR',
  categories => 'GENERAL|SELECTION',
};

ENCODING SETEND_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100010000 imod=0bZZ M=0bZ op=0b1 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E:u=0bx A=0bZ I=0b0 F=0b0 ig7=0b0 mode=0b0ZZZZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SETEND',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=SETEND cclass=SETEND_a1',
  pstate     => 'E=CW',
  categories => 'GENERAL|MISC',
};

ENCODING SETEND_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011011001 op=0b0 ig1=0bS E:u=0bx ig2=0bZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SETEND',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cps16 page=SETEND cclass=SETEND_t1',
  pstate     => 'E=CW',
  categories => 'GENERAL|MISC',
};

ENCODING SETPAN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100010001 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ imm1:u=0bx ig11=0bZ ig12=0b0000 ig13=0bZ ig14=0bZ ig15=0bZ ig16=0bZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SETPAN',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=setpan page=SETPAN cclass=SETPAN_a1',
  pstate     => 'EL=CR PAN=CW',
  categories => 'GENERAL',
};

ENCODING SETPAN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110110000 ig1=0bS imm1:u=0bx ig2=0bZ ig3=0bZ ig4=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SETPAN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=setpan16 page=SETPAN cclass=SETPAN_t1',
  pstate     => 'EL=CR PAN=CW',
  categories => 'GENERAL',
};

ENCODING SEV_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=SEV cclass=SEV_a1',
  categories => 'SYSTEM',
};

ENCODING SEV_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0100 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=SEV cclass=SEV_t1',
  categories => 'SYSTEM',
};

ENCODING SEV_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0100',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SEV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=SEV cclass=SEV_t2',
  categories => 'SYSTEM',
};

ENCODING SEVL_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000101',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=SEVL cclass=SEVL_a1',
  categories => 'SYSTEM',
};

ENCODING SEVL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0101 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=SEVL cclass=SEVL_t1',
  categories => 'SYSTEM',
};

ENCODING SEVL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=SEVL cclass=SEVL_t2',
  categories => 'SYSTEM',
};

ENCODING SHADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHADD16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHADD16 cclass=SHADD16_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHADD16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHADD16 cclass=SHADD16_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHADD8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHADD8 cclass=SHADD8_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHADD8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHADD8 cclass=SHADD8_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHASX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHASX cclass=SHASX_a1',
  categories => 'GENERAL|CARITHMETIC|HALVING|SIGNED',
};

ENCODING SHASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHASX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHASX cclass=SHASX_t1',
  categories => 'GENERAL|CARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSAX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSAX cclass=SHSAX_a1',
  categories => 'GENERAL|CARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSAX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSAX cclass=SHSAX_t1',
  categories => 'GENERAL|CARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSUB16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSUB16 cclass=SHSUB16_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSUB16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSUB16 cclass=SHSUB16_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSUB8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSUB8 cclass=SHSUB8_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SHSUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSUB8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSUB8 cclass=SHSUB8_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|SIGNED',
};

ENCODING SMC_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 imm12=0bZZZZZZZZZZZZ ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=SMC cclass=SMC_a1',
  exceptions => 'MONITORCALL',
  categories => 'GENERAL|CALL',
};

ENCODING SMC_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11110111111 o1=0b1 imm4:u=0bxxxx ig1=0b10 o2=0b0 ig2=0b0 imm12=0bZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=SMC cclass=SMC_t1',
  exceptions => 'MONITORCALL',
  categories => 'GENERAL|CALL',
};

ENCODING SMLABB_A1 => {
  name       => 'SMLABB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLABB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLABB cclass=SMLABB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLABT_A1 => {
  name       => 'SMLABT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLABT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLABB cclass=SMLABB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLATB_A1 => {
  name       => 'SMLATB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLATB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLABB cclass=SMLABB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLATT_A1 => {
  name       => 'SMLATT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLATT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLABB cclass=SMLABB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLABB_T1 => {
  name       => 'SMLABB',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 N:u=0b0 M:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'N == 0b0 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLABB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLABB cclass=SMLABB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLABT_T1 => {
  name       => 'SMLABT',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 N:u=0b0 M:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'N == 0b0 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLABT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLABB cclass=SMLABB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLATB_T1 => {
  name       => 'SMLATB',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 N:u=0b1 M:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'N == 0b1 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLATB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLABB cclass=SMLABB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLATT_T1 => {
  name       => 'SMLATT',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 N:u=0b1 M:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'N == 0b1 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLATT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLABB cclass=SMLABB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAD_A1 => {
  name       => 'SMLAD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLAD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLAD cclass=SMLAD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLADX_A1 => {
  name       => 'SMLADX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLADX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLAD cclass=SMLAD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAD_T1 => {
  name       => 'SMLAD',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLAD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAD cclass=SMLAD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLADX_T1 => {
  name       => 'SMLADX',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLADX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAD cclass=SMLAD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b1 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=SMLALS',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMLAL cclass=SMLAL_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAL_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=SMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMLAL cclass=SMLAL_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLAL cclass=SMLAL_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALBB_A1 => {
  name       => 'SMLALBB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALBB',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLALBB cclass=SMLALBB_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALBT_A1 => {
  name       => 'SMLALBT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALBT',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLALBB cclass=SMLALBB_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALTB_A1 => {
  name       => 'SMLALTB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALTB',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLALBB cclass=SMLALBB_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALTT_A1 => {
  name       => 'SMLALTT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALTT',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLALBB cclass=SMLALBB_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALBB_T1 => {
  name       => 'SMLALBB',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b10 N:u=0b0 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b0 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALBB',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALBB cclass=SMLALBB_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALBT_T1 => {
  name       => 'SMLALBT',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b10 N:u=0b0 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b0 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALBT',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALBB cclass=SMLALBB_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALTB_T1 => {
  name       => 'SMLALTB',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b10 N:u=0b1 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b1 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALTB',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALBB cclass=SMLALBB_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALTT_T1 => {
  name       => 'SMLALTT',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b10 N:u=0b1 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b1 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALTT',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALBB cclass=SMLALBB_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALD_A1 => {
  name       => 'SMLALD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALD',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLALD cclass=SMLALD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALDX_A1 => {
  name       => 'SMLALDX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLALDX',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLALD cclass=SMLALD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALD_T1 => {
  name       => 'SMLALD',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALD',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALD cclass=SMLALD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLALDX_T1 => {
  name       => 'SMLALDX',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLALDX',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALD cclass=SMLALD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAWB_A1 => {
  name       => 'SMLAWB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b0 N=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLAWB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLAWB cclass=SMLAWB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAWT_A1 => {
  name       => 'SMLAWT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0b1 N=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLAWT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLAWB cclass=SMLAWB_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAWB_T1 => {
  name       => 'SMLAWB',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLAWB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAWB cclass=SMLAWB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLAWT_T1 => {
  name       => 'SMLAWT',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLAWT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAWB cclass=SMLAWB_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSD_A1 => {
  name       => 'SMLSD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLSD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSD cclass=SMLSD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSDX_A1 => {
  name       => 'SMLSDX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLSDX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSD cclass=SMLSD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSD_T1 => {
  name       => 'SMLSD',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLSD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLSD cclass=SMLSD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSDX_T1 => {
  name       => 'SMLSDX',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLSDX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLSD cclass=SMLSD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSLD_A1 => {
  name       => 'SMLSLD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLSLD',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSLD cclass=SMLSLD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSLDX_A1 => {
  name       => 'SMLSLDX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMLSLDX',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSLD cclass=SMLSLD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSLD_T1 => {
  name       => 'SMLSLD',
  diagram    => 'ig0=0b111110111 op1=0b101 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLSLD',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLSLD cclass=SMLSLD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMLSLDX_T1 => {
  name       => 'SMLSLDX',
  diagram    => 'ig0=0b111110111 op1=0b101 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLSLDX',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLSLD cclass=SMLSLD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLA_A1 => {
  name       => 'SMMLA',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 R:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLA cclass=SMMLA_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLAR_A1 => {
  name       => 'SMMLAR',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 R:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMLAR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLA cclass=SMMLA_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLA_T1 => {
  name       => 'SMMLA',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLA cclass=SMMLA_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLAR_T1 => {
  name       => 'SMMLAR',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b1 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMLAR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLA cclass=SMMLA_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLS_A1 => {
  name       => 'SMMLS',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b11 R:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLS cclass=SMMLS_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLSR_A1 => {
  name       => 'SMMLSR',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b11 R:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMLSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLS cclass=SMMLS_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLS_T1 => {
  name       => 'SMMLS',
  diagram    => 'ig0=0b111110110 op1=0b110 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLS cclass=SMMLS_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMLSR_T1 => {
  name       => 'SMMLSR',
  diagram    => 'ig0=0b111110110 op1=0b110 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMLSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLS cclass=SMMLS_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMMUL_A1 => {
  name       => 'SMMUL',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 R:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMUL cclass=SMMUL_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMMULR_A1 => {
  name       => 'SMMULR',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 R:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMMULR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMUL cclass=SMMUL_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMMUL_T1 => {
  name       => 'SMMUL',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMUL cclass=SMMUL_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMMULR_T1 => {
  name       => 'SMMULR',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'R == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMMULR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMUL cclass=SMMUL_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMUAD_A1 => {
  name       => 'SMUAD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMUAD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUAD cclass=SMUAD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUADX_A1 => {
  name       => 'SMUADX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMUADX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUAD cclass=SMUAD_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUAD_T1 => {
  name       => 'SMUAD',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMUAD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUAD cclass=SMUAD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUADX_T1 => {
  name       => 'SMUADX',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMUADX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUAD cclass=SMUAD_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMULBB_A1 => {
  name       => 'SMULBB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULBB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULBB cclass=SMULBB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULBT_A1 => {
  name       => 'SMULBT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULBB cclass=SMULBB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULTB_A1 => {
  name       => 'SMULTB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b0 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULBB cclass=SMULBB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULTT_A1 => {
  name       => 'SMULTT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b1 N:u=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1 && N == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULBB cclass=SMULBB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULBB_T1 => {
  name       => 'SMULBB',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 N:u=0b0 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b0 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULBB cclass=SMULBB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULBT_T1 => {
  name       => 'SMULBT',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 N:u=0b0 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b0 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULBB cclass=SMULBB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULTB_T1 => {
  name       => 'SMULTB',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 N:u=0b1 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b1 && M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULBB cclass=SMULBB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULTT_T1 => {
  name       => 'SMULTT',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 N:u=0b1 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'N == 0b1 && M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULBB cclass=SMULBB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULLS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b1 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=SMULLS',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMULL cclass=SMULL_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULL_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=SMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMULL cclass=SMULL_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b000 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMULL cclass=SMULL_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULWB_A1 => {
  name       => 'SMULWB',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b0 N=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULWB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULWB cclass=SMULWB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULWT_A1 => {
  name       => 'SMULWT',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0b1 N=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMULWT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULWB cclass=SMULWB_a1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULWB_T1 => {
  name       => 'SMULWB',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULWB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULWB cclass=SMULWB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMULWT_T1 => {
  name       => 'SMULWT',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULWT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULWB cclass=SMULWB_t1',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SMUSD_A1 => {
  name       => 'SMUSD',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b01 M:u=0b0 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMUSD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUSD cclass=SMUSD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUSDX_A1 => {
  name       => 'SMUSDX',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b01 M:u=0b1 ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMUSDX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUSD cclass=SMUSD_a1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUSD_T1 => {
  name       => 'SMUSD',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMUSD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUSD cclass=SMUSD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SMUSDX_T1 => {
  name       => 'SMUSDX',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'M == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMUSDX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUSD cclass=SMUSD_t1',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SRSDA_A1_AS => {
  name       => 'Decrement After',
  diagram    => 'ig0=0b1111100 P:u=0b0 U:u=0b0 S=0b1 W:u=0bx L=0b0 Rn=0bSSZS op=0bZZZZZSZSZZZ mode:u=0bxxxxx',
  bitdiffs   => 'P == 0b0 && U == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=dec-after mnemonic=SRSDA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=SRS cclass=SRS_a1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SRSDB_A1_AS => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1111100 P:u=0b1 U:u=0b0 S=0b1 W:u=0bx L=0b0 Rn=0bSSZS op=0bZZZZZSZSZZZ mode:u=0bxxxxx',
  bitdiffs   => 'P == 0b1 && U == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=dec-before mnemonic=SRSDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=SRS cclass=SRS_a1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SRSIA_A1_AS => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1111100 P:u=0b0 U:u=0b1 S=0b1 W:u=0bx L=0b0 Rn=0bSSZS op=0bZZZZZSZSZZZ mode:u=0bxxxxx',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=inc-after mnemonic=SRS{IA}',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=SRS cclass=SRS_a1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SRSIB_A1_AS => {
  name       => 'Increment Before',
  diagram    => 'ig0=0b1111100 P:u=0b1 U:u=0b1 S=0b1 W:u=0bx L=0b0 Rn=0bSSZS op=0bZZZZZSZSZZZ mode:u=0bxxxxx',
  bitdiffs   => 'P == 0b1 && U == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 ldmstm-mode=inc-before mnemonic=SRSIB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=SRS cclass=SRS_a1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SRS_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b00 ig1=0b0 W:u=0bx L=0b0 Rn=0bSSZS P=0bS M=0bS ig2=0bZZZZZZZZZ mode:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 ldmstm-mode=dec-before mnemonic=SRSDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=SRS cclass=SRS_t1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SRS_T2_AS => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b11 ig1=0b0 W:u=0bx L=0b0 Rn=0bSSZS P=0bS M=0bS ig2=0bZZZZZZZZZ mode:u=0bxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 ldmstm-mode=inc-after mnemonic=SRS{IA}',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=SRS cclass=SRS_t2',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SSAT_A1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0b1 ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sh == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAT mnemonic-shift-type=SSAT-asr shift-type=asr',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=SSAT cclass=SSAT_a1',
  pstate     => 'C=R N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAT_A1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0b0 ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sh == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAT mnemonic-shift-type=SSAT-lsl shift-type=lsl',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=SSAT cclass=SSAT_a1',
  pstate     => 'C=R N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAT_T1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b00 sh:u=0b1 ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  cnsts      => 'imm3:imm2!=0b00000',
  bitdiffs   => 'sh == 0b1 && !(imm3 == 0b000 && imm2 == 0b00)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAT mnemonic-shift-type=SSAT-asr shift-type=asr',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SSAT cclass=SSAT_t1',
  pstate     => 'C=CR N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAT_T1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b00 sh:u=0b0 ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  bitdiffs   => 'sh == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAT mnemonic-shift-type=SSAT-lsl shift-type=lsl',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SSAT cclass=SSAT_t1',
  pstate     => 'C=CR N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAT16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAT16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat16 page=SSAT16 cclass=SSAT16_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAT16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b001 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3=0b000 Rd:u=0bxxxx imm2=0b00 ig5=0bZ ig6=0bZ sat_imm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAT16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SSAT16 cclass=SSAT16_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|SATURATING|SIGNED',
};

ENCODING SSAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSAX cclass=SSAX_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SSAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSAX cclass=SSAX_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|SIGNED',
};

ENCODING SSBB_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option=0b0000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSBB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=SSBB cclass=SSBB_a1',
  categories => 'SYSTEM',
};

ENCODING SSBB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=SSBB cclass=SSBB_t1',
  categories => 'SYSTEM',
};

ENCODING SSUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSUB16 cclass=SSUB16_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SSUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSUB16 cclass=SSUB16_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SSUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSUB8 cclass=SSUB8_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING SSUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSUB8 cclass=SSUB8_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|SIGNED',
};

ENCODING STC_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b0 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=STC cclass=STC_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0bx D=0b0 W:u=0b1 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=STC cclass=STC_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b1 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=STC cclass=STC_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_A1_unind => {
  name       => 'Unindexed',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D=0b0 W:u=0b0 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1 && W == 0b0',
  docvars    => 'address-form=unindexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=STC cclass=STC_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_T1_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b0 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=STC cclass=STC_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_T1_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b0 U:u=0bx D=0b0 W:u=0b1 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=STC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=STC cclass=STC_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_T1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b1 U:u=0bx D=0b0 W:u=0b1 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=STC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=STC cclass=STC_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STC_T1_unind => {
  name       => 'Unindexed',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0b0 U:u=0b1 D=0b0 W:u=0b0 L=0b0 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b0 && U == 0b1 && W == 0b0',
  docvars    => 'address-form=unindexed armarmheading=T1 instr-class=general isa=T32 mnemonic=STC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=STC cclass=STC_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STL_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STL cclass=STL_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STL cclass=STL_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLB cclass=STLB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLB cclass=STLB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEX cclass=STLEX_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b10 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEX cclass=STLEX_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXB cclass=STLEXB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b00 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXB cclass=STLEXB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXD cclass=STLEXD_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op=0b1 sz=0b11 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXD cclass=STLEXD_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXH cclass=STLEXH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLEXH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b01 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXH cclass=STLEXH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLH cclass=STLH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STLH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLH cclass=STLH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STM_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STM cclass=STM_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STM_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1100 L=0b0 Rn:u=0bxxx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldstm16 page=STM cclass=STM_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STM_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=STM cclass=STM_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STM_u_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W=0bZ L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STM_u cclass=STM_u_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STMDA_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b0 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMDA cclass=STMDA_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STMDB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMDB cclass=STMDB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STMDB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=STMDB cclass=STMDB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STMIB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b1 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMIB cclass=STMIB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STR_i cclass=STR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0bx o2=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STR_i cclass=STR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b0 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STR_i cclass=STR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b0 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=STR_i cclass=STR_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1001 L=0b0 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_sp page=STR_i cclass=STR_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110001 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STR_i cclass=STR_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T4_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=STR_i cclass=STR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T4_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=STR_i cclass=STR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_i_T4_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=STR_i cclass=STR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STR_r cclass=STR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b0 U:u=0bx o2=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STR_r cclass=STR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b0 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STR_r cclass=STR_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b0 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STR_r cclass=STR_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STR_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STR_r cclass=STR_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRB_i cclass=STRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0bx o2=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRB_i cclass=STRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0bx o2=0b1 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRB_i cclass=STRB_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b1 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=STRB_i cclass=STRB_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STRB_i cclass=STRB_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_T3_offn => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=STRB_i cclass=STRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_T3_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=STRB_i cclass=STRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_i_T3_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=STRB_i cclass=STRB_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRB_r cclass=STRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b0 U:u=0bx o2=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRB_r cclass=STRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0b1 U:u=0bx o2=0b1 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRB_r cclass=STRB_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b1 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STRB_r cclass=STRB_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRB_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STRB_r cclass=STRB_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRBT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRBT cclass=STRBT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRBT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRBT cclass=STRBT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRBT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRBT cclass=STRBT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRD_i cclass=STRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRD_i cclass=STRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRD_i cclass=STRD_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_T1_off => {
  name       => 'Offset',
  diagram    => 'ig0=0b1110100 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_imm page=STRD_i cclass=STRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_T1_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b1110100 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b1 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=STRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_post page=STRD_i cclass=STRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_i_T1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b1110100 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T1 instr-class=general isa=T32 mnemonic=STRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstd_pre page=STRD_i cclass=STRD_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRD_r cclass=STRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRD_r cclass=STRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRD_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRD_r cclass=STRD_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREX cclass=STREX_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000010 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex page=STREX cclass=STREX_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXB cclass=STREXB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b00 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXB cclass=STREXB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXD cclass=STREXD_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz=0b11 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXD cclass=STREXD_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXH cclass=STREXH_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STREXH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b01 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXH cclass=STREXH_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRH_i cclass=STRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b1 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRH_i cclass=STRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b1 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRH_i cclass=STRH_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1000 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldsth16_imm page=STRH_i cclass=STRH_i_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STRH_i cclass=STRH_i_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_T3_offn => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b0',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=STRH_i cclass=STRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_T3_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b1',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=STRH_i cclass=STRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_i_T3_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0bx W:u=0b1 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=T3 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=STRH_i cclass=STRH_i_t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_r_A1_off => {
  name       => 'Offset',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b0',
  docvars    => 'address-form=base-plus-offset address-offset=signed-offset armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRH_r cclass=STRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_r_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b0 U:u=0bx ig1=0b0 W:u=0b0 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && W == 0b0',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRH_r cclass=STRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_r_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0b1 U:u=0bx ig1=0b0 W:u=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && W == 0b1',
  docvars    => 'address-form=pre-indexed armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRH_r cclass=STRH_r_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b0 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STRH_r cclass=STRH_r_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRH_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STRH_r cclass=STRH_r_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRHT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRHT cclass=STRHT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRHT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRHT cclass=STRHT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRHT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRHT cclass=STRHT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'EL=CR',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRT cclass=STRT_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRT_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRT cclass=STRT_a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=R EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING STRT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRT cclass=STRT_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  pstate     => 'C=CR EL=CR N=U V=U Z=U',
  categories => 'GENERAL|DATA_TRANSFER|STORE',
};

ENCODING SUB_i_A1 => {
  name       => 'SUB',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b11x1',
  bitdiffs   => 'S == 0b0 && Rn != 0b11x1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_i cclass=SUB_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_i_A1 => {
  name       => 'SUBS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b1 && Rn != 0b1101',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_i cclass=SUB_i_a1',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000111 S=0b1 imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_2l_imm page=SUB_i cclass=SUB_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_i_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b001 op=0b11 Rdn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=SUB_i cclass=SUB_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_i_T3 => {
  name       => 'SUB',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0b0 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_i cclass=SUB_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_i_T3 => {
  name       => 'SUBS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_i cclass=SUB_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_i_T4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b11x1',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=SUBW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_i cclass=SUB_i_t4',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_PC_T5_AS => {
  name       => 'T5',
  diagram    => 'ig0=0b111100111101 Rn:u=0bSSSZ ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8:u=0bxxxxxxxx',
  cnsts      => 'imm8!=0b00000000',
  docvars    => 'armarmheading=T5 instr-class=general isa=T32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=eret page=SUB_i cclass=SUB_i_t5',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_r_A1_RRX => {
  name       => 'SUB, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB mnemonic-shift-type=SUB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_r cclass=SUB_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_r_A1 => {
  name       => 'SUB, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB mnemonic-shift-type=SUB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_r cclass=SUB_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_r_A1_RRX => {
  name       => 'SUBS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS mnemonic-shift-type=SUBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_r cclass=SUB_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_r_A1 => {
  name       => 'SUBS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS mnemonic-shift-type=SUBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_r cclass=SUB_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000110 S=0b1 Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_3l page=SUB_r cclass=SUB_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_r_T2_RRX => {
  name       => 'SUB, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUB mnemonic-shift-type=SUB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_r cclass=SUB_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_r_T2 => {
  name       => 'SUB, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUB mnemonic-shift-type=SUB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_r cclass=SUB_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_r_T2_RRX => {
  name       => 'SUBS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUBS mnemonic-shift-type=SUBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_r cclass=SUB_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_r_T2 => {
  name       => 'SUBS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101 imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUBS mnemonic-shift-type=SUBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_r cclass=SUB_r_t2',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b1 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SUB_rr cclass=SUB_rr_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b0 Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=SUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SUB_rr cclass=SUB_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_i_A1 => {
  name       => 'SUB',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_SP_i cclass=SUB_SP_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_i_A1 => {
  name       => 'SUBS',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_SP_i cclass=SUB_SP_i_a1',
  pstate     => 'C=W N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110000 S=0b1 imm7:u=0bxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=adjsp16 page=SUB_SP_i cclass=SUB_SP_i_t1',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_i_T2 => {
  name       => 'SUB',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0b0 Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_SP_i cclass=SUB_SP_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_i_T2 => {
  name       => 'SUBS',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0b1 Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && Rd != 0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_SP_i cclass=SUB_SP_i_t2',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_i_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1101 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=SUBW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_SP_i cclass=SUB_SP_i_t3',
  pstate     => 'C=CW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_r_A1_RRX => {
  name       => 'SUB, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB mnemonic-shift-type=SUB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_SP_r cclass=SUB_SP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_r_A1 => {
  name       => 'SUB, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b0 Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUB mnemonic-shift-type=SUB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_SP_r cclass=SUB_SP_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_r_A1_RRX => {
  name       => 'SUBS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1 && imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS mnemonic-shift-type=SUBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_SP_r cclass=SUB_SP_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_r_A1 => {
  name       => 'SUBS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0b1 Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => 'S == 0b1 && !(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SUBS mnemonic-shift-type=SUBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_SP_r cclass=SUB_SP_r_a1',
  pstate     => 'C=X N=W V=W Z=W',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_r_T1_RRX => {
  name       => 'SUB, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b0 Rn=0b1101 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB mnemonic-shift-type=SUB-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_SP_r cclass=SUB_SP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUB_SP_r_T1 => {
  name       => 'SUB, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b0 Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => 'S == 0b0 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB mnemonic-shift-type=SUB-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_SP_r cclass=SUB_SP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_r_T1_RRX => {
  name       => 'SUBS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b1 Rn=0b1101 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && imm3 == 0b000 && Rd != 0b1111 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUBS mnemonic-shift-type=SUBS-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_SP_r cclass=SUB_SP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SUBS_SP_r_T1 => {
  name       => 'SUBS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0b1 Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011 Rd!=0b1111',
  bitdiffs   => 'S == 0b1 && !(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11) && Rd != 0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUBS mnemonic-shift-type=SUBS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_SP_r cclass=SUB_SP_r_t1',
  pstate     => 'C=RCW N=CW V=CW Z=CW',
  categories => 'GENERAL|ARITHMETIC',
};

ENCODING SVC_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1111 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SVC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=svcall iclass=svc page=SVC cclass=SVC_a1',
  exceptions => 'SUPERVISORCALL',
  categories => 'GENERAL|CALL',
};

ENCODING SVC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101111 S=0b1 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SVC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=except16 page=SVC cclass=SVC_t1',
  exceptions => 'SUPERVISORCALL',
  categories => 'GENERAL|CALL',
};

ENCODING SXTAB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b10 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAB cclass=SXTAB_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTAB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAB cclass=SXTAB_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTAB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b00 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAB16 cclass=SXTAB16_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTAB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAB16 cclass=SXTAB16_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTAH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b11 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAH cclass=SXTAH_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTAH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAH cclass=SXTAH_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b10 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTB cclass=SXTB_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b0 B=0b1 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=SXTB cclass=SXTB_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTB_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTB cclass=SXTB_t2',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b00 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTB16 cclass=SXTB16_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTB16 cclass=SXTB16_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b11 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTH cclass=SXTH_a1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b0 B=0b0 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=SXTH cclass=SXTH_t1',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING SXTH_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTH cclass=SXTH_t2',
  categories => 'GENERAL|SIGN_EXTEND|SIGNED',
};

ENCODING TBB_T1 => {
  name       => 'Byte',
  diagram    => 'ig0=0b111010001101 Rn:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0b000 H:u=0b0 Rm:u=0bxxxx',
  bitdiffs   => 'H == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 loadstore-size=ls-byte mnemonic=TBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=tblbr page=TBB cclass=TBB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING TBH_T1 => {
  name       => 'Halfword',
  diagram    => 'ig0=0b111010001101 Rn:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0b000 H:u=0b1 Rm:u=0bxxxx',
  bitdiffs   => 'H == 0b1',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 loadstore-size=ls-halfword mnemonic=TBH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=tblbr page=TBB cclass=TBB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|BRANCH|UNCONDITIONALLY',
};

ENCODING TEQ_i_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=TEQ_i cclass=TEQ_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0100 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=TEQ_i cclass=TEQ_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_r_A1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ mnemonic-shift-type=TEQ-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TEQ_r cclass=TEQ_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_r_A1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => '!(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ mnemonic-shift-type=TEQ-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TEQ_r cclass=TEQ_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_r_T1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0100 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd=0b1111 imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TEQ mnemonic-shift-type=TEQ-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TEQ_r cclass=TEQ_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_r_T1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0100 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TEQ mnemonic-shift-type=TEQ-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TEQ_r cclass=TEQ_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TEQ_rr_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=TEQ_rr cclass=TEQ_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TSB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010010',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=TSB cclass=TSB_a1',
  categories => 'SYSTEM',
};

ENCODING TSB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0010',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=TSB cclass=TSB_t1',
  categories => 'SYSTEM',
};

ENCODING TST_i_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=TST_i cclass=TST_i_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0000 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TST',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=TST_i cclass=TST_i_t1',
  pstate     => 'C=CW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_r_A1_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0b00000 stype:u=0b11 ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'imm5 == 0b00000 && stype == 0b11',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST mnemonic-shift-type=TST-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TST_r cclass=TST_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_r_A1 => {
  name       => 'Shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5:stype!=0b0000011',
  bitdiffs   => '!(imm5 == 0b00000 && stype == 0b11)',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST mnemonic-shift-type=TST-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TST_r cclass=TST_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1000 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TST',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=TST_r cclass=TST_r_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_r_T2_RRX => {
  name       => 'Rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0b000 Rd=0b1111 imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  bitdiffs   => 'imm3 == 0b000 && imm2 == 0b00 && stype == 0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=TST mnemonic-shift-type=TST-rrx shift-type=rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TST_r cclass=TST_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_r_T2 => {
  name       => 'Shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2:stype!=0b0000011',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00 && stype == 0b11)',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=TST mnemonic-shift-type=TST-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TST_r cclass=TST_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|COMPARISON',
};

ENCODING TST_rr_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=TST_rr cclass=TST_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|COMPARISON',
};

ENCODING UADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UADD16 cclass=UADD16_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UADD16 cclass=UADD16_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UADD8 cclass=UADD8_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UADD8 cclass=UADD8_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UASX cclass=UASX_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UASX cclass=UASX_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UBFX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111 U=0b1 ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UBFX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfx page=UBFX cclass=UBFX_a1',
  categories => 'GENERAL|EXTRACTION|UNSIGNED',
};

ENCODING UBFX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b110 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UBFX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=UBFX cclass=UBFX_t1',
  categories => 'GENERAL|EXTRACTION|UNSIGNED',
};

ENCODING UDF_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0b1110 ig0=0b01111111 imm12:u=0bxxxxxxxxxxxx ig1=0b1111 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UDF',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=udf page=UDF cclass=UDF_a1',
  categories => 'GENERAL|UNDEFINED',
};

ENCODING UDF_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101111 S=0b0 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UDF',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=except16 page=UDF cclass=UDF_t1',
  categories => 'GENERAL|UNDEFINED',
};

ENCODING UDF_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110111111 o1=0b1 imm4:u=0bxxxx ig1=0b10 o2=0b1 ig2=0b0 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UDF',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=UDF cclass=UDF_t2',
  categories => 'GENERAL|UNDEFINED',
};

ENCODING UDIV_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b011 Rd:u=0bxxxx Ra:u=0bSSSS Rm:u=0bxxxx op2=0b000 ig1=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=UDIV cclass=UDIV_a1',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UDIV_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b011 Rn:u=0bxxxx Ra:u=0bSSSS Rd:u=0bxxxx op2=0b1111 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UDIV cclass=UDIV_t1',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UHADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHADD16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHADD16 cclass=UHADD16_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHADD16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHADD16 cclass=UHADD16_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHADD8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHADD8 cclass=UHADD8_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHADD8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHADD8 cclass=UHADD8_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHASX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHASX cclass=UHASX_a1',
  categories => 'GENERAL|CARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHASX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHASX cclass=UHASX_t1',
  categories => 'GENERAL|CARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSAX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSAX cclass=UHSAX_a1',
  categories => 'GENERAL|CARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSAX',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSAX cclass=UHSAX_t1',
  categories => 'GENERAL|CARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSUB16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSUB16 cclass=UHSUB16_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSUB16',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSUB16 cclass=UHSUB16_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSUB8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSUB8 cclass=UHSUB8_a1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UHSUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSUB8',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSUB8 cclass=UHSUB8_t1',
  categories => 'GENERAL|ARITHMETIC|HALVING|UNSIGNED',
};

ENCODING UMAAL_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UMAAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMAAL cclass=UMAAL_a1',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMAAL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b110 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0110 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMAAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMAAL cclass=UMAAL_t1',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMLALS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b1 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=UMLALS',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMLAL cclass=UMLAL_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMLAL_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=UMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMLAL cclass=UMLAL_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMLAL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b110 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMLAL cclass=UMLAL_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING UMULLS_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b1 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b1',
  docvars    => 'armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=UMULLS',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMULL cclass=UMULL_a1',
  pstate     => 'C=U N=W V=U Z=W',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UMULL_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'S == 0b0',
  docvars    => 'armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=UMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMULL cclass=UMULL_a1',
  pstate     => 'C=U N=U V=U Z=U',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UMULL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b010 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMULL cclass=UMULL_t1',
  pstate     => 'C=U N=CW V=U Z=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UQADD16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQADD16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQADD16 cclass=UQADD16_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQADD16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQADD16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQADD16 cclass=UQADD16_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQADD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQADD8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQADD8 cclass=UQADD8_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQADD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQADD8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQADD8 cclass=UQADD8_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQASX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQASX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQASX cclass=UQASX_a1',
  categories => 'GENERAL|CARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQASX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQASX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQASX cclass=UQASX_t1',
  categories => 'GENERAL|CARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSAX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSAX cclass=UQSAX_a1',
  categories => 'GENERAL|CARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSAX',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSAX cclass=UQSAX_t1',
  categories => 'GENERAL|CARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSUB16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSUB16 cclass=UQSUB16_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSUB16',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSUB16 cclass=UQSUB16_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSUB8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSUB8 cclass=UQSUB8_a1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING UQSUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSUB8',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSUB8 cclass=UQSUB8_t1',
  categories => 'GENERAL|ARITHMETIC|SATURATING|UNSIGNED',
};

ENCODING USAD8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=usad page=USAD8 cclass=USAD8_a1',
  categories => 'GENERAL|MATH|UNSIGNED',
};

ENCODING USAD8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b111 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=USAD8 cclass=USAD8_t1',
  categories => 'GENERAL|MATH|UNSIGNED',
};

ENCODING USADA8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USADA8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=usad page=USADA8 cclass=USADA8_a1',
  categories => 'GENERAL|MATH|UNSIGNED',
};

ENCODING USADA8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b111 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USADA8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=USADA8 cclass=USADA8_t1',
  categories => 'GENERAL|MATH|UNSIGNED',
};

ENCODING USAT_A1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0b1 ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sh == 0b1',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAT mnemonic-shift-type=USAT-asr shift-type=asr',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=USAT cclass=USAT_a1',
  pstate     => 'C=R N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAT_A1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0b0 ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'sh == 0b0',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAT mnemonic-shift-type=USAT-lsl shift-type=lsl',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=USAT cclass=USAT_a1',
  pstate     => 'C=R N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAT_T1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b10 sh:u=0b1 ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  cnsts      => 'imm3:imm2!=0b00000',
  bitdiffs   => 'sh == 0b1 && !(imm3 == 0b000 && imm2 == 0b00)',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAT mnemonic-shift-type=USAT-asr shift-type=asr',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=USAT cclass=USAT_t1',
  pstate     => 'C=CR N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAT_T1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b10 sh:u=0b0 ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  bitdiffs   => 'sh == 0b0',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAT mnemonic-shift-type=USAT-lsl shift-type=lsl',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=USAT cclass=USAT_t1',
  pstate     => 'C=CR N=U Q=CS V=U Z=U',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAT16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAT16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat16 page=USAT16 cclass=USAT16_a1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAT16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b101 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3=0b000 Rd:u=0bxxxx imm2=0b00 ig5=0bZ ig6=0bZ sat_imm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAT16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=USAT16 cclass=USAT16_t1',
  pstate     => 'Q=CS',
  categories => 'GENERAL|SATURATING|UNSIGNED',
};

ENCODING USAX_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USAX cclass=USAX_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING USAX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USAX cclass=USAX_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|CARITHMETIC|UNSIGNED',
};

ENCODING USUB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USUB16 cclass=USUB16_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING USUB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USUB16 cclass=USUB16_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING USUB8_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USUB8 cclass=USUB8_a1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING USUB8_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USUB8 cclass=USUB8_t1',
  pstate     => 'GE=CW',
  categories => 'GENERAL|ARITHMETIC|UNSIGNED',
};

ENCODING UXTAB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b10 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAB cclass=UXTAB_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTAB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAB cclass=UXTAB_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTAB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b00 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAB16 cclass=UXTAB16_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTAB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAB16 cclass=UXTAB16_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTAH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b11 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAH cclass=UXTAH_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTAH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAH cclass=UXTAH_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b10 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTB cclass=UXTB_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b1 B=0b1 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=UXTB cclass=UXTB_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTB_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTB cclass=UXTB_t2',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTB16_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b00 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTB16 cclass=UXTB16_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTB16_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTB16 cclass=UXTB16_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTH_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b11 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTH cclass=UXTH_a1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTH_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b1 B=0b0 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=UXTH cclass=UXTH_t1',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING UXTH_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTH cclass=UXTH_t2',
  categories => 'GENERAL|SIGN_EXTEND|UNSIGNED',
};

ENCODING WFE_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000010',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=WFE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=WFE cclass=WFE_a1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'SYSTEM',
};

ENCODING WFE_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0010 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=WFE',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=WFE cclass=WFE_t1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'SYSTEM',
};

ENCODING WFE_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0010',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=WFE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=WFE cclass=WFE_t2',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'SYSTEM',
};

ENCODING WFI_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000011',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=WFI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=WFI cclass=WFI_a1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|HINT',
};

ENCODING WFI_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0011 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=WFI',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=WFI cclass=WFI_t1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|HINT',
};

ENCODING WFI_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0011',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=WFI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=WFI cclass=WFI_t2',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
  pstate     => 'EL=CR M=CR',
  categories => 'GENERAL|HINT',
};

ENCODING YIELD_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000001',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=YIELD cclass=YIELD_a1',
  categories => 'GENERAL|HINT',
};

ENCODING YIELD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0001 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=YIELD cclass=YIELD_t1',
  categories => 'GENERAL|HINT',
};

ENCODING YIELD_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0001',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=YIELD cclass=YIELD_t2',
  categories => 'GENERAL|HINT',
};

ENCODING ADD_ADR_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=ADD armarmheading=A1 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=ADR_A1 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_ADR cclass=ADD_ADR_a1',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'ADR{<c>}{<q>} <Rd>, <label>',
};

ENCODING ADD_ADR_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b0 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=ADD armarmheading=T1 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=ADR_T1 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADD_ADR cclass=ADD_ADR_t1',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'ADR{<c>}{<q>} <Rd>, <label>',
};

ENCODING ADD_ADR_T3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=ADD armarmheading=T3 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=ADR_T3 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_ADR cclass=ADD_ADR_t3',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'ADR{<c>}{<q>} <Rd>, <label>',
};

ENCODING ASR_MOV_r_A1 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b10 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ASR_MOV_r cclass=ASR_MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASR_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b10 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=ASR armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=ASR_MOV_r cclass=ASR_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV<c>{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASR_MOV_r_T3 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b10 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ASR_MOV_r cclass=ASR_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASR_MOV_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b10 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ASR_MOV_rr cclass=ASR_MOV_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ASR <Rs>',
};

ENCODING ASR_MOV_rr_T1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'ig0=0b010000 op:u=0b0100 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=ASR armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-asr shift-type=asr',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_ASR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ASR_MOV_rr cclass=ASR_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV<c>{<q>} <Rdm>, <Rdm>, ASR <Rs>',
};

ENCODING ASR_MOV_rr_T2 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b10 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASR armarmheading=T2 cond-setting=no-s instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ASR_MOV_rr cclass=ASR_MOV_rr_t2',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ASR <Rs>',
};

ENCODING ASRS_MOVS_r_A1 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b10 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASRS armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ASRS_MOV_r cclass=ASRS_MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASRS_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b10 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=ASRS armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=ASRS_MOV_r cclass=ASRS_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASRS_MOVS_r_T3 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b10 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASRS armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ASRS_MOV_r cclass=ASRS_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ASR #<imm>',
};

ENCODING ASRS_MOVS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b10 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASRS armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ASRS_MOV_rr cclass=ASRS_MOV_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ASR <Rs>',
};

ENCODING ASRS_MOV_rr_T1_ASR => {
  name       => 'Arithmetic shift right',
  diagram    => 'ig0=0b010000 op:u=0b0100 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=ASRS armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-asr shift-type=asr',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_ASR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ASRS_MOV_rr cclass=ASRS_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<q>} <Rdm>, <Rdm>, ASR <Rs>',
};

ENCODING ASRS_MOVS_rr_T2 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b10 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ASRS armarmheading=T2 cond-setting=s instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ASRS_MOV_rr cclass=ASRS_MOV_rr_t2',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|ARITHMETIC',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ASR <Rs>',
};

ENCODING LSL_MOV_r_A1 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b00 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSL_MOV_r cclass=LSL_MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSL_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b00 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'imm5!=0b00000',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=LSL armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSL_MOV_r cclass=LSL_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV<c>{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSL_MOV_r_T3 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b00 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSL_MOV_r cclass=LSL_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSL_MOV_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b00 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSL_MOV_rr cclass=LSL_MOV_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSL <Rs>',
};

ENCODING LSL_MOV_rr_T1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'ig0=0b010000 op:u=0b0010 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=LSL armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsl shift-type=lsl',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_LSL alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSL_MOV_rr cclass=LSL_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV<c>{<q>} <Rdm>, <Rdm>, LSL <Rs>',
};

ENCODING LSL_MOV_rr_T2 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b00 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSL armarmheading=T2 cond-setting=no-s instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSL_MOV_rr cclass=LSL_MOV_rr_t2',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSL <Rs>',
};

ENCODING LSLS_MOVS_r_A1 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b00 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSLS armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSLS_MOV_r cclass=LSLS_MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSLS_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b00 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'imm5!=0b00000',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=LSLS armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSLS_MOV_r cclass=LSLS_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSLS_MOVS_r_T3 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b00 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSLS armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSLS_MOV_r cclass=LSLS_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSL #<imm>',
};

ENCODING LSLS_MOVS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b00 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSLS armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSLS_MOV_rr cclass=LSLS_MOV_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSL <Rs>',
};

ENCODING LSLS_MOV_rr_T1_LSL => {
  name       => 'Logical shift left',
  diagram    => 'ig0=0b010000 op:u=0b0010 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=LSLS armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsl shift-type=lsl',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_LSL alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSLS_MOV_rr cclass=LSLS_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<q>} <Rdm>, <Rdm>, LSL <Rs>',
};

ENCODING LSLS_MOVS_rr_T2 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b00 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSLS armarmheading=T2 cond-setting=s instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSLS_MOV_rr cclass=LSLS_MOV_rr_t2',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_LEFT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSL <Rs>',
};

ENCODING LSR_MOV_r_A1 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b01 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSR_MOV_r cclass=LSR_MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSR_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b01 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=LSR armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSR_MOV_r cclass=LSR_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV<c>{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSR_MOV_r_T3 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b01 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSR_MOV_r cclass=LSR_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSR_MOV_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b01 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSR_MOV_rr cclass=LSR_MOV_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSR <Rs>',
};

ENCODING LSR_MOV_rr_T1_LSR => {
  name       => 'Logical shift right',
  diagram    => 'ig0=0b010000 op:u=0b0011 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=LSR armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsr shift-type=lsr',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_LSR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSR_MOV_rr cclass=LSR_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV<c>{<q>} <Rdm>, <Rdm>, LSR <Rs>',
};

ENCODING LSR_MOV_rr_T2 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b01 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSR armarmheading=T2 cond-setting=no-s instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSR_MOV_rr cclass=LSR_MOV_rr_t2',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, LSR <Rs>',
};

ENCODING LSRS_MOVS_r_A1 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b01 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSRS armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSRS_MOV_r cclass=LSRS_MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSRS_MOV_r_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b01 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=LSRS armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_r_T2 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSRS_MOV_r cclass=LSRS_MOV_r_t2',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSRS_MOVS_r_T3 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b01 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSRS armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSRS_MOV_r cclass=LSRS_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSR #<imm>',
};

ENCODING LSRS_MOVS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b01 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSRS armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSRS_MOV_rr cclass=LSRS_MOV_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSR <Rs>',
};

ENCODING LSRS_MOV_rr_T1_LSR => {
  name       => 'Logical shift right',
  diagram    => 'ig0=0b010000 op:u=0b0011 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=LSRS armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-lsr shift-type=lsr',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_LSR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSRS_MOV_rr cclass=LSRS_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<q>} <Rdm>, <Rdm>, LSR <Rs>',
};

ENCODING LSRS_MOVS_rr_T2 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b01 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=LSRS armarmheading=T2 cond-setting=s instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSRS_MOV_rr cclass=LSRS_MOV_rr_t2',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|SHIFT_RIGHT|LOGICAL',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, LSR <Rs>',
};

ENCODING POP_LDM_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0b1 L=0b1 Rn:u=0b1101 register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'BitCount(register_list) > 1',
  docvars    => 'alias_mnemonic=POP armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=LDM_A1 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=POP_LDM cclass=POP_LDM_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'LDM{<c>}{<q>} SP!, <registers>',
};

ENCODING POP_LDM_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0b1 L=0b1 Rn:u=0b1101 P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  aliascond  => 'BitCount(P:M:register_list) > 1',
  docvars    => 'alias_mnemonic=POP armarmheading=T2 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=LDM_T2 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=POP_LDM cclass=POP_LDM_t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'LDM{<c>}{<q>} SP!, <registers>',
};

ENCODING POP_LDR_i_A1_post => {
  name       => 'Post-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0b1 o2=0b0 W:u=0b0 o1=0b1 Rn:u=0b1101 Rt:u=0bxxxx imm12:u=0b000000000100',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'address-form=post-indexed alias_mnemonic=POP armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=LDR_i_A1_post alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=POP_LDR_i cclass=POP_LDR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'LDR{<c>}{<q>} <Rt>, [SP], #4',
};

ENCODING POP_LDR_i_T4_post => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0b1101 Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0b1 W:u=0b1 imm8:u=0b00000100',
  aliascond  => 'Unconditionally',
  docvars    => 'address-form=post-indexed alias_mnemonic=POP armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=LDR_i_T4_post alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=POP_LDR_i cclass=POP_LDR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'LDR{<c>}{<q>} <Rt>, [SP], #4',
};

ENCODING PUSH_STMDB_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0b1 L=0b0 Rn:u=0b1101 register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'BitCount(register_list) > 1',
  docvars    => 'alias_mnemonic=PUSH armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=STMDB_A1 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=PUSH_STMDB cclass=PUSH_STMDB_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'STMDB{<c>}{<q>} SP!, <registers>',
};

ENCODING PUSH_STMDB_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0b1 L=0b0 Rn:u=0b1101 P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  aliascond  => 'BitCount(M:register_list) > 1',
  docvars    => 'alias_mnemonic=PUSH armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=STMDB_T1 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=PUSH_STMDB cclass=PUSH_STMDB_t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'STMDB{<c>}{<q>} SP!, <registers>',
};

ENCODING PUSH_STR_i_A1_pre => {
  name       => 'Pre-indexed',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0b0 o2=0b0 W:u=0b1 o1=0b0 Rn:u=0b1101 Rt:u=0bxxxx imm12:u=0b000000000100',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'address-form=pre-indexed alias_mnemonic=PUSH armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=STR_i_A1_pre alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=PUSH_STR_i cclass=PUSH_STR_i_a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'STR{<c>}{<q>} <Rt>, [SP, #-4]!',
};

ENCODING PUSH_STR_i_T4_pre => {
  name       => 'Pre-indexed',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0b1101 Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b1 imm8:u=0b00000100',
  aliascond  => 'Unconditionally',
  docvars    => 'address-form=pre-indexed alias_mnemonic=PUSH armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=STR_i_T4_pre alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=PUSH_STR_i cclass=PUSH_STR_i_t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  categories => 'GENERAL|STACK_TRANSFER',
  eq_to      => 'STR{<c>}{<q>} <Rt>, [SP, #-4]!',
};

ENCODING ROR_MOV_r_A1 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ROR_MOV_r cclass=ROR_MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ROR #<imm>',
};

ENCODING ROR_MOV_r_T3 => {
  name       => 'MOV, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2!=0b00000',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00)',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ROR_MOV_r cclass=ROR_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ROR #<imm>',
};

ENCODING ROR_MOV_rr_A1 => {
  name       => 'Not flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b11 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR armarmheading=A1 cond-setting=no-s instr-class=general isa=A32 mnemonic=MOV',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ROR_MOV_rr cclass=ROR_MOV_rr_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ROR <Rs>',
};

ENCODING ROR_MOV_rr_T1_ROR => {
  name       => 'Rotate right',
  diagram    => 'ig0=0b010000 op:u=0b0111 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => 'InITBlock',
  docvars    => 'alias_mnemonic=ROR armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-ror shift-type=ror',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_ROR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ROR_MOV_rr cclass=ROR_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV<c>{<q>} <Rdm>, <Rdm>, ROR <Rs>',
};

ENCODING ROR_MOV_rr_T2 => {
  name       => 'Not flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b11 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=ROR armarmheading=T2 cond-setting=no-s instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ROR_MOV_rr cclass=ROR_MOV_rr_t2',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, ROR <Rs>',
};

ENCODING RORS_MOVS_r_A1 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RORS armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_r_A1 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RORS_MOV_r cclass=RORS_MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ROR #<imm>',
};

ENCODING RORS_MOVS_r_T3 => {
  name       => 'MOVS, shift or rotate by value',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b11 Rm:u=0bxxxx',
  cnsts      => 'imm3:imm2!=0b00000',
  bitdiffs   => '!(imm3 == 0b000 && imm2 == 0b00)',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RORS armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-shift-no-rrx shift-type=shift-no-rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_r_T3 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RORS_MOV_r cclass=RORS_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ROR #<imm>',
};

ENCODING RORS_MOVS_rr_A1 => {
  name       => 'Flag setting',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b11 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RORS armarmheading=A1 cond-setting=s instr-class=general isa=A32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_rr_A1 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=RORS_MOV_rr cclass=RORS_MOV_rr_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ROR <Rs>',
};

ENCODING RORS_MOV_rr_T1_ROR => {
  name       => 'Rotate right',
  diagram    => 'ig0=0b010000 op:u=0b0111 Rs:u=0bxxx Rdm:u=0bxxx',
  aliascond  => '!InITBlock',
  docvars    => 'alias_mnemonic=RORS armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-ror shift-type=ror',
  metadata   => 'alias=1 isa=THUMB isaform=T16 aliasof=MOV_rr_T1_ROR alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=RORS_MOV_rr cclass=RORS_MOV_rr_t1',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<q>} <Rdm>, <Rdm>, ROR <Rs>',
};

ENCODING RORS_MOVS_rr_T2 => {
  name       => 'Flag setting',
  diagram    => 'ig0=0b111110100 stype:u=0b11 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RORS armarmheading=T2 cond-setting=s instr-class=general isa=T32 mnemonic=MOVS',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_rr_T2 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=RORS_MOV_rr cclass=RORS_MOV_rr_t2',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, ROR <Rs>',
};

ENCODING RRX_MOV_r_A1_RRX => {
  name       => 'MOV, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RRX armarmheading=A1 instr-class=general isa=A32 mnemonic=MOV mnemonic-shift-type=MOV-rrx shift-type=rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOV_r_A1_RRX alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RRX_MOV_r cclass=RRX_MOV_r_a1',
  pstate     => 'C=R N=U V=U Z=U',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, RRX',
};

ENCODING RRX_MOV_r_T3_RRX => {
  name       => 'MOV, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RRX armarmheading=T3 instr-class=general isa=T32 mnemonic=MOV mnemonic-shift-type=MOV-rrx shift-type=rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOV_r_T3_RRX alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RRX_MOV_r cclass=RRX_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOV{<c>}{<q>} <Rd>, <Rm>, RRX',
};

ENCODING RRXS_MOVS_r_A1_RRX => {
  name       => 'MOVS, rotate right with extend',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RRXS armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVS mnemonic-shift-type=MOVS-rrx shift-type=rrx',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=MOVS_r_A1_RRX alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RRXS_MOV_r cclass=RRXS_MOV_r_a1',
  pstate     => 'C=X N=W V=U Z=W',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, RRX',
};

ENCODING RRXS_MOVS_r_T3_RRX => {
  name       => 'MOVS, rotate right with extend',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=RRXS armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVS mnemonic-shift-type=MOVS-rrx shift-type=rrx',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=MOVS_r_T3_RRX alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RRXS_MOV_r cclass=RRXS_MOV_r_t3',
  pstate     => 'C=RCW N=CW V=U Z=CW',
  categories => 'GENERAL|ROTATE_RIGHT',
  eq_to      => 'MOVS{<c>}{<q>} <Rd>, <Rm>, RRX',
};

ENCODING SUB_ADR_A2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'imm12 == 0b000000000000',
  docvars    => 'alias_mnemonic=SUB armarmheading=A2 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=ADR_A2 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_ADR cclass=SUB_ADR_a2',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'ADR{<c>}{<q>} <Rd>, <label>',
};

ENCODING SUB_ADR_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  aliascond  => 'i:imm3:imm8 == 0b000000000000',
  docvars    => 'alias_mnemonic=SUB armarmheading=T2 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=ADR_T2 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_ADR cclass=SUB_ADR_t2',
  categories => 'GENERAL|ARITHMETIC',
  eq_to      => 'ADR{<c>}{<q>} <Rd>, <label>',
};


1;

