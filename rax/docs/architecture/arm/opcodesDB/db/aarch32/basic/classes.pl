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

CLASS ADC_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b101 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADC_i xid=a1',
};

CLASS ADC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1010 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADC_i xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS ADC_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADC_r xid=a1',
};

CLASS ADC_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0101 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ADC_r xid=t1',
};

CLASS ADC_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b1010 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADC_r xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS ADC_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADC_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ADD_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_i xid=a1',
};

CLASS ADD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000111 S=0b0 imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_2l_imm page=ADD_i xid=t1',
};

CLASS ADD_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b001 op=0b10 Rdn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=ADD_i xid=t2',
};

CLASS ADD_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_i xid=t3',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15',
};

CLASS ADD_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b11x1',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=ADDW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_i xid=t4',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ADD_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_r xid=a1',
};

CLASS ADD_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000110 S=0b0 Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_3l page=ADD_r xid=t1',
};

CLASS ADD_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b00 DN:u=0bx Rm:u=0bxxxx Rdn:u=0bxxx',
  cnsts      => 'Rm!=0b1101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_r xid=t2',
  unpredict  => '((UInt(DN:Rdn)) == 15 && UInt(Rm) == 15) || (UInt(DN:Rdn) == 15 && InITBlock() && !LastInITBlock())',
};

CLASS ADD_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_r xid=t3',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS ADD_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=ADD_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ADD_SP_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S:u=0bx Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_SP_i xid=a1',
};

CLASS ADD_SP_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b1 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADD_SP_i xid=t1',
};

CLASS ADD_SP_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b10110000 S=0b0 imm7:u=0bxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=adjsp16 page=ADD_SP_i xid=t2',
};

CLASS ADD_SP_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S:u=0bx Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ADD_SP_i xid=t3',
  unpredict  => 'UInt(Rd) == 15 && !((S == 1))',
};

CLASS ADD_SP_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1101 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=ADDW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_SP_i xid=t4',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ADD_SP_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0bx Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=ADD_SP_r xid=a1',
};

CLASS ADD_SP_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010001 op=0b00 DM:u=0bx Rs=0b1101 Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_SP_r xid=t1',
  unpredict  => 'UInt(DM:Rdm) == 15 && InITBlock() && !LastInITBlock()',
};

CLASS ADD_SP_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b00 D=0b1 Rm:u=0bxxxx Rd=0b101',
  cnsts      => 'Rm!=0b1101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=ADD_SP_r xid=t2',
};

CLASS ADD_SP_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b1000 S:u=0bx Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ADD_SP_r xid=t3',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rm) == 15',
};

CLASS ADR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADR xid=a1',
};

CLASS ADR_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADR xid=a2',
};

CLASS ADR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b0 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADR xid=t1',
};

CLASS ADR_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADR xid=t2',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ADR_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADR xid=t3',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS AND_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b000 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=AND_i xid=a1',
};

CLASS AND_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0000 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=AND_i xid=t1',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15',
};

CLASS AND_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=AND_r xid=a1',
};

CLASS AND_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0000 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=AND',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=AND_r xid=t1',
};

CLASS AND_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0000 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=AND_r xid=t2',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS AND_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=AND_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS B_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b101 H=0b0 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=B',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=B xid=a1',
};

CLASS B_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101 cond:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=bcond16 page=B xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS B_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11100 imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'iclass=b16 page=B xid=t2',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS B_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 S:u=0bx cond:u=0bxxxx imm6:u=0bxxxxxx ig1=0b10 J1:u=0bx ig2=0b0 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  cnsts      => 'cond!=0b111x',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bcond page=B xid=t3',
  unpredict  => 'InITBlock()',
};

CLASS B_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b10 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=B',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=b page=B xid=t4',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS BFC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn=0b1111',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BFC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfi page=BFC xid=a1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS BFC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b011 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ msb:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BFC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=BFC xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS BFI_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0111110 msb:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig1=0b001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BFI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfi page=BFI xid=a1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS BFI_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b011 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ msb:u=0bxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BFI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=BFI xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS BIC_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b10 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=BIC_i xid=a1',
};

CLASS BIC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0001 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=BIC_i xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS BIC_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=BIC_r xid=a1',
};

CLASS BIC_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1110 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BIC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=BIC_r xid=t1',
};

CLASS BIC_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0001 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=BIC_r xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS BIC_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b10 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=BIC_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS BKPT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BKPT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=BKPT xid=a1',
  exceptions => 'SOFTWAREBREAKPOINT',
  unpredict  => 'cond != 14',
};

CLASS BKPT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111110 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BKPT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=bkpt16 page=BKPT xid=t1',
  exceptions => 'SOFTWAREBREAKPOINT',
};

CLASS BL_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b101 H=0b1 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=BL_i xid=a1',
};

CLASS BL_i_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0b1111 ig0=0b101 H:u=0bx imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=BLX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=b_imm page=BL_i xid=a2',
};

CLASS BL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 S:u=0bx imm10:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b1 J2:u=0bx imm11:u=0bxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bl page=BL_i xid=t1',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS BL_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 S:u=0bx imm10H:u=0bxxxxxxxxxx ig1=0b11 J1:u=0bx ig2=0b0 J2:u=0bx imm10L:u=0bxxxxxxxxxx H:u=0bx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=BLX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=blx page=BL_i xid=t2',
  undefined  => 'H == 1',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS BLX_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BLX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=blx_reg page=BLX_r xid=a1',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS BLX_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01000111 L=0b1 Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BLX',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=bx16 page=BLX_r xid=t1',
  unpredict  => '(UInt(Rm) == 15) || (InITBlock() && !LastInITBlock())',
};

CLASS BX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=bx_reg page=BX xid=a1',
};

CLASS BX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01000111 L=0b0 Rm:u=0bxxxx ig1=0bZ ig2=0bZ ig3=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BX',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=bx16 page=BX xid=t1',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS BXJ_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bS ig10=0bS ig11=0bS ig12=0bS ig13=0b0010 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=BXJ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=bxj_reg page=BXJ xid=a1',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS BXJ_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111100 Rm:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=BXJ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=bx_jaz page=BXJ xid=t1',
  unpredict  => '(UInt(Rm) == 15) || (InITBlock() && !LastInITBlock())',
};

CLASS CBNZ_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011 op:u=0bx ig1=0b0 i:u=0bx ig2=0b1 imm5:u=0bxxxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cbznz16 page=CBNZ xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS CLREX_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0001 option=0bSSSS',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CLREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=CLREX xid=a1',
};

CLASS CLREX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0010 option=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CLREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=CLREX xid=t1',
};

CLASS CLZ_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bS ig2=0bS ig3=0bS ig4=0bS Rd:u=0bxxxx ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0b0001 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CLZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=clz page=CLZ xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS CLZ_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b011 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CLZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CLZ xid=t1',
  unpredict  => 'UInt(Rm) != UInt(Rn) || UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS CMN_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=CMN_i xid=a1',
};

CLASS CMN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1000 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=CMN_i xid=t1',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS CMN_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMN_r xid=a1',
};

CLASS CMN_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1011 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=CMN_r xid=t1',
};

CLASS CMN_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b1000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMN_r xid=t2',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS CMN_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=CMN_rr xid=a1',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS CMP_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=CMP_i xid=a1',
};

CLASS CMP_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b001 op=0b01 Rn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=CMP_i xid=t1',
};

CLASS CMP_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=CMP_i xid=t2',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS CMP_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=CMP_r xid=a1',
};

CLASS CMP_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1010 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=CMP_r xid=t1',
};

CLASS CMP_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b010001 op=0b01 N:u=0bx Rm:u=0bxxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=CMP_r xid=t2',
  unpredict  => '(UInt(N:Rn) < 8 && UInt(Rm) < 8) || (UInt(N:Rn) == 15 || UInt(Rm) == 15)',
};

CLASS CMP_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b1101 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=CMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=CMP_r xid=t3',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS CMP_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=CMP_rr xid=a1',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS CPS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100010000 imod:u=0bxx M:u=0bx op=0b0 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E=0bZ A:u=0bx I:u=0bx F:u=0bx ig7=0b0 mode:u=0bxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=CPS xid=a1',
  unpredict  => '(mode != 0 && M == 0) || ((imod<1> == 1 && A:I:F == 0) || (imod<1> == 0 && A:I:F != 0)) || ((imod == 0 && M == 0) || imod == 1)',
};

CLASS CPS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011011001 op=0b1 im:u=0bx ig1=0bZ A:u=0bx I:u=0bx F:u=0bx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cps16 page=CPS xid=t1',
  unpredict  => '(A:I:F == 0) || (InITBlock())',
};

CLASS CPS_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ imod:u=0bxx M:u=0bx A:u=0bx I:u=0bx F:u=0bx mode:u=0bxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=cps page=CPS xid=t2',
  unpredict  => '(mode != 0 && M == 0) || ((imod<1> == 1 && A:I:F == 0) || (imod<1> == 0 && A:I:F != 0)) || (imod == 1 || InITBlock())',
};

CLASS CRC32_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b0 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32 xid=a1',
  undefined  => '! HaveCRCExt()',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || ((8 << UInt(sz)) == 64) || (cond != 14)',
};

CLASS CRC32_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b0 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32 xid=t1',
  undefined  => '! HaveCRCExt()',
  unpredict  => '(InITBlock()) || (UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || ((8 << UInt(sz)) == 64)',
};

CLASS CRC32C_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 sz:u=0bxx ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ C:u=0b1 ig4=0bZ ig5=0b0100 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=crc32 page=CRC32C xid=a1',
  undefined  => '! HaveCRCExt()',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || ((8 << UInt(sz)) == 64) || (cond != 14)',
};

CLASS CRC32C_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 ig1=0b10 C:u=0b1 Rn:u=0bxxxx ig2=0b1111 Rd:u=0bxxxx ig3=0b10 sz:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=CRC32C xid=t1',
  undefined  => '! HaveCRCExt()',
  unpredict  => '(InITBlock()) || (UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || ((8 << UInt(sz)) == 64)',
};

CLASS CSDB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=CSDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=CSDB xid=a1',
  unpredict  => 'cond != 14',
};

CLASS CSDB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0100',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=CSDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=CSDB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS DBG_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bZZZZ1111 option:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DBG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=DBG xid=a1',
};

CLASS DBG_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b1111 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DBG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=DBG xid=t1',
};

CLASS DCPS1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b01',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS1 xid=t1',
};

CLASS DCPS2_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b10',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS2 xid=t1',
  undefined  => '!HaveEL(EL2)',
};

CLASS DCPS3_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111101111000 imm4=0b1111 ig1=0b1000 imm10=0b0000000000 opt=0b11',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DCPS3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=dcps page=DCPS3 xid=t1',
  undefined  => '!HaveEL(EL3)',
};

CLASS DMB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0101 option:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DMB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=DMB xid=a1',
};

CLASS DMB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0101 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DMB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=DMB xid=t1',
};

CLASS DSB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option:u=0bxxxx',
  cnsts      => 'option!=0b0x00',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=DSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=DSB xid=a1',
};

CLASS DSB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option:u=0bxxxx',
  cnsts      => 'option!=0b0x00',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=DSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=DSB xid=t1',
};

CLASS EOR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b001 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=EOR_i xid=a1',
};

CLASS EOR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0100 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=EOR_i xid=t1',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15',
};

CLASS EOR_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=EOR_r xid=a1',
};

CLASS EOR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0001 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=EOR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=EOR_r xid=t1',
};

CLASS EOR_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0100 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=EOR_r xid=t2',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS EOR_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=EOR_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ERET_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010110 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ ig13=0b0110 ig14=0bS ig15=0bS ig16=0bS ig17=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ERET',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=eret page=ERET xid=a1',
};

CLASS ERET_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111101 Rn=0b1110 ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8=0b00000000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ERET',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=eret page=ERET xid=t1',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS ESB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010000',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ESB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=ESB xid=a1',
  unpredict  => 'cond != 14',
};

CLASS ESB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ESB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=ESB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS HLT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=HLT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=HLT xid=a1',
  undefined  => 'EDSCR.HDE == 0 || !HaltingAllowed()',
  unpredict  => 'cond != 14',
};

CLASS HLT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011101010 imm6:u=0bxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=HLT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hlt16 page=HLT xid=t1',
  undefined  => 'EDSCR.HDE == 0 || !HaltingAllowed()',
};

CLASS HVC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 imm12:u=0bxxxxxxxxxxxx ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=HVC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=HVC xid=a1',
  exceptions => 'HYPERVISORCALL',
  unpredict  => 'cond != 14',
};

CLASS HVC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110111111 o1=0b0 imm4:u=0bxxxx ig1=0b10 o2=0b0 ig2=0b0 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=HVC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=HVC xid=t1',
  exceptions => 'HYPERVISORCALL',
  unpredict  => 'InITBlock()',
};

CLASS ISB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0110 option:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ISB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=ISB xid=a1',
};

CLASS ISB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0110 option:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ISB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=ISB xid=t1',
};

CLASS IT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 firstcond:u=0bxxxx mask:u=0bxxxx',
  cnsts      => 'mask!=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=IT',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=it16 page=IT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(firstcond == 15 || (firstcond == 14 && BitCount(mask) != 1)) || (InITBlock())',
};

CLASS LDA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDA xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDA xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEX xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEX xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEXB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEXB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEXD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXD xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'Rt<0> == 1 || (UInt(Rt) + 1) == 15 || UInt(Rn) == 15',
};

CLASS LDAEXD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op=0b1 sz=0b11 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXD xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2) || UInt(Rn) == 15',
};

CLASS LDAEXH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAEXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAEXH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAEXH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAEXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAEXH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDAH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDAH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=LDAH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDC_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
};

CLASS LDC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
};

CLASS LDC_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn=0b1111 CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=LDC_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
  unpredict  => 'W == 1 || (P == 0 && CurrentInstrSet() != InstrSet_A32)',
};

CLASS LDC_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b1 Rn=0b1111 CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=LDC_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
  unpredict  => 'W == 1 || (P == 0 && CurrentInstrSet() != InstrSet_A32)',
};

CLASS LDM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((register_list)) < 1) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS LDM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1100 L=0b1 Rn:u=0bxxx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldstm16 page=LDM xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'BitCount((0:register_list)) < 1',
};

CLASS LDM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=LDM xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2 || (P == 1 && M == 1)) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1 && InITBlock() && !LastInITBlock())',
};

CLASS LDM_e_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W:u=0bx L=0b1 Rn:u=0bxxxx ig1=0b1 register_list:u=0bxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM_e xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS LDM_u_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W=0bZ L=0b1 Rn:u=0bxxxx ig1=0b0 register_list:u=0bxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDM_u xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS LDMDA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b0 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMDA xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((register_list)) < 1) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS LDMDB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMDB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((register_list)) < 1) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS LDMDB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=LDMDB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2 || (P == 1 && M == 1)) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1 && InITBlock() && !LastInITBlock())',
};

CLASS LDMIB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b1 op=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=LDMIB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((register_list)) < 1) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS LDR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt)',
};

CLASS LDR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b0 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=LDR_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1001 L=0b1 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_sp page=LDR_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110001 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDR_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=LDR_i xid=t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(((W == 1)) && UInt(Rn) == UInt(Rt)) || (UInt(Rt) == 15 && InITBlock() && !LastInITBlock())',
};

CLASS LDR_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDR_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '((P == 0) || (W == 1))',
};

CLASS LDR_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b01001 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldlit16 page=LDR_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_l_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b10 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDR_l xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 && InITBlock() && !LastInITBlock()',
};

CLASS LDR_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDR_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS LDR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b0 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDR_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDR_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDR_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rm) == 15) || (UInt(Rt) == 15 && InITBlock() && !LastInITBlock())',
};

CLASS LDRB_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || (((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b1 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=LDRB_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRB_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDRB_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRB_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=LDRB_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(UInt(Rt) == 15 && W == 1) || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRB_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRB_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || ((P == 0) || (W == 1))',
};

CLASS LDRB_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b00 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDRB_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRB_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRB_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS LDRB_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b1 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRB_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRB_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDRB_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS LDRBT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRBT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS LDRBT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRBT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS LDRBT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRBT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS LDRD_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(Rt<0> == 1) || (P == 0 && W == 1) || (((P == 0) || (W == 1)) && (UInt(Rn) == UInt(Rt) || UInt(Rn) == (UInt(Rt)+1))) || ((UInt(Rt)+1) == 15)',
};

CLASS LDRD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd page=LDRD_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(((W == 1)) && (UInt(Rn) == UInt(Rt) || UInt(Rn) == UInt(Rt2))) || (UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2))',
};

CLASS LDRD_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0bS U:u=0bx ig1=0b1 W=0bZ o1=0b0 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRD_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(Rt<0> == 1) || ((UInt(Rt)+1) == 15)',
};

CLASS LDRD_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L=0b1 ig2=0b1111 Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=lddlit page=LDRD_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2)) || (W == 1)',
};

CLASS LDRD_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRD_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(Rt<0> == 1) || (P == 0 && W == 1) || ((UInt(Rt)+1) == 15 || UInt(Rm) == 15 || UInt(Rm) == UInt(Rt) || UInt(Rm) == (UInt(Rt)+1)) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rn) == (UInt(Rt)+1)))',
};

CLASS LDREX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREX xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDREX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000010 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rd=0bSSSS imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex page=LDREX xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDREXB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDREXB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDREXD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXD xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'Rt<0> == 1 || (UInt(Rt) + 1) == 15 || UInt(Rn) == 15',
};

CLASS LDREXD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz=0b11 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXD xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2) || UInt(Rn) == 15',
};

CLASS LDREXH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 xRt=0bSSSS',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDREXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=LDREXH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDREXH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=LDREXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=LDREXH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS LDRH_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || (((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRH_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1000 L=0b1 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldsth16_imm page=LDRH_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRH_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=LDRH_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRH_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=LDRH_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(UInt(Rt) == 15 && W == 1) || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRH_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRH_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || ((P == 0) || (W == 1))',
};

CLASS LDRH_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx size=0b01 L=0b1 ig1=0b1111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=LDRH_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRH_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRH_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS LDRH_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b0 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRH_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRH_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=LDRH_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS LDRHT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRHT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS LDRHT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRHT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS LDRHT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRHT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS LDRSB_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || (((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRSB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=LDRSB_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSB_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=LDRSB_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(UInt(Rt) == 15 && W == 1) || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRSB_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSB_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || ((P == 0) || (W == 1))',
};

CLASS LDRSB_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b00 ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=LDRSB_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSB_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSB_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS LDRSB_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b1 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRSB_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSB_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=LDRSB_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS LDRSBT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b10 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSBT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS LDRSBT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b10 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSBT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS LDRSBT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_unpriv page=LDRSBT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS LDRSH_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || (((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRSH_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=LDRSH_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSH_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=LDRSH_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(UInt(Rt) == 15 && W == 1) || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS LDRSH_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b1 Rn=0b1111 Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSH_l xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || ((P == 0) || (W == 1))',
};

CLASS LDRSH_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b01 ig1=0b11111 Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rt!=0b1111',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=LDRSH_l xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSH_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSH_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS LDRSH_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b1 B=0b1 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=LDRSH_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS LDRSH_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111 Rt!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=LDRSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=LDRSH_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS LDRSHT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=LDRSHT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS LDRSHT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=LDRSHT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS LDRSHT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b01 ig1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRSHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_unpriv page=LDRSHT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS LDRT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=LDRT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS LDRT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b1 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=LDRT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS LDRT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=LDRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=LDRT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS MCR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L=0b0 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig1=0b1 CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MCR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as iclass=movcpgp32 page=MCR xid=a1',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS MCR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b1110 opc1:u=0bxxx L=0b0 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MCR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf iclass=cp_mov32 page=MCR xid=t1',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS MCRR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 L=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MCRR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=movcpgp64 page=MCRR xid=a1',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15',
};

CLASS MCRR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b11000 D=0b1 ig2=0b0 L=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MCRR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_mov64 page=MCRR xid=t1',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15',
};

CLASS MLA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b001 S:u=0bx Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MLA xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS MLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MLA xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS MLS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MLS xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS MLS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MLS xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS MOV_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b01 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MOV_i xid=a1',
};

CLASS MOV_i_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 H=0b0 ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=MOVW',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movw page=MOV_i xid=a2',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MOV_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b001 op=0b00 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=MOV_i xid=t1',
};

CLASS MOV_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0bx Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MOV_i xid=t2',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MOV_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=MOVW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=movw page=MOV_i xid=t3',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MOV_r xid=a1',
};

CLASS MOV_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010001 op=0b10 D:u=0bx Rm:u=0bxxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=spcd iclass=addsub16_2h page=MOV_r xid=t1',
  unpredict  => 'UInt(D:Rd) == 15 && InITBlock() && !LastInITBlock()',
};

CLASS MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0bxx imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'op!=0b11',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0bx Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0xxx Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MOV_rr xid=t1',
};

CLASS MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0bxx S:u=0bx Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS MOVT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 H=0b1 ig1=0b00 imm4:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MOVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movw page=MOVT xid=a1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MOVT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b100 imm4:u=0bxxxx ig3=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=movw page=MOVT xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MRC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 opc1:u=0bxxx L=0b1 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig1=0b1 CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=cops_as iclass=movcpgp32 page=MRC xid=a1',
};

CLASS MRC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b1110 opc1:u=0bxxx L=0b1 CRn:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc2:u=0bxxx ig2=0b1 CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=cpaf iclass=cp_mov32 page=MRC xid=t1',
};

CLASS MRRC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 L=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRRC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=movcpgp64 page=MRRC xid=a1',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2)',
};

CLASS MRRC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b11000 D=0b1 ig2=0b0 L=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx coproc:u=0b111x opc1:u=0bxxxx CRm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRRC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_mov64 page=MRRC xid=t1',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rt) == UInt(Rt2)',
};

CLASS MRS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b0 ig2=0b0 mask=0bSSSS Rd:u=0bxxxx ig3=0bZ ig4=0bZ B=0b0 m=0bZ ig5=0b0000 Rn=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MRS xid=a1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MRS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011111 R:u=0bx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 Rd:u=0bxxxx ig8=0bZ ig9=0bZ ig10=0b0 ig11=0bZ ig12=0bZ ig13=0bZ ig14=0bZ ig15=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=mrs_spec page=MRS xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MRS_br_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b0 ig2=0b0 M1:u=0bxxxx Rd:u=0bxxxx ig3=0bZ ig4=0bZ B=0b1 M:u=0bx ig5=0b0000 Rn=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MRS_br xid=a1',
  exceptions => 'UNCATEGORIZED',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MRS_br_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011111 R:u=0bx M1:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 Rd:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 M:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=mrs_bank page=MRS_br xid=t1',
  exceptions => 'UNCATEGORIZED',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MSR_br_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b1 ig2=0b0 M1:u=0bxxxx Rd=0bSSSS ig3=0bZ ig4=0bZ B=0b1 M:u=0bx ig5=0b0000 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MSR_br xid=a1',
  exceptions => 'UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS MSR_br_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 M1:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b1 M:u=0bx ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=msr_bank page=MSR_br xid=t1',
  exceptions => 'UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS MSR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R:u=0bx ig1=0b10 mask:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=MSR_i xid=a1',
  unpredict  => 'mask == 0',
};

CLASS MSR_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 R:u=0bx ig1=0b1 ig2=0b0 mask:u=0bxxxx Rd=0bSSSS ig3=0bZ ig4=0bZ B=0b0 m=0bZ ig5=0b0000 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=MSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=movsr_reg page=MSR_r xid=a1',
  unpredict  => '(mask == 0) || (UInt(Rn) == 15)',
};

CLASS MSR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110011100 R:u=0bx Rn:u=0bxxxx ig1=0b10 ig2=0bZ ig3=0b0 mask:u=0bxxxx ig4=0bZ ig5=0bZ ig6=0b0 ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ ig11=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=msr_spec page=MSR_r xid=t1',
  unpredict  => '(mask == 0) || (UInt(Rn) == 15)',
};

CLASS MUL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b000 S:u=0bx Rd:u=0bxxxx RdLo=0bZZZZ Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=MUL xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS MUL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1101 Rn:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MUL',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MUL xid=t1',
};

CLASS MUL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110110 op1=0b000 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=MUL xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS MVN_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b11 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=MVN_i xid=a1',
};

CLASS MVN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0bx Rn=0b1111 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=MVN_i xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS MVN_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=MVN_r xid=a1',
};

CLASS MVN_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1111 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MVN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=MVN_r xid=t1',
};

CLASS MVN_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0bx Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=MVN_r xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS MVN_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b11 S:u=0bx Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=MVN_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS NOP_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000000',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=NOP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=NOP xid=a1',
};

CLASS NOP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0000 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=NOP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=NOP xid=t1',
};

CLASS NOP_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0000',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=NOP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=NOP xid=t2',
};

CLASS ORN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0011 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORN_i xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ORN_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110101 op1=0b0011 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORN_r xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS ORR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00111 opc=0b00 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=log2reg_imm page=ORR_i xid=a1',
};

CLASS ORR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0010 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=ORR_i xid=t1',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ORR_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ORR_r xid=a1',
};

CLASS ORR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1100 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ORR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ORR_r xid=t1',
};

CLASS ORR_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ORR_r xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS ORR_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b00 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ORR_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS PKH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx tb:u=0bx ig1=0b01 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=pack page=PKH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS PKH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110101 op1=0b0110 S:u=0b0 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx tb:u=0bx T:u=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=PKH xid=t1',
  undefined  => 'S == 1 || T == 1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS PLD_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111010 D=0b1 U:u=0bx R:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLD_i xid=a1',
};

CLASS PLD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110001 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=PLD_i xid=t1',
};

CLASS PLD_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_nimm page=PLD_i xid=t2',
};

CLASS PLD_l_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111010 D=0b1 U:u=0bx R=0bS ig1=0b01 Rn=0b1111 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLD preload-type=pld',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLD_l xid=a1',
};

CLASS PLD_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111000 U:u=0bx ig1=0b0 W=0bZ L=0b1 ig2=0b1111 Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=general isa=T32 mnemonic=PLD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_unsigned page=PLD_l xid=t1',
};

CLASS PLD_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111011 D=0b1 U:u=0bx R:u=0bx ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLD_r xid=a1',
  unpredict  => 'UInt(Rm) == 15 || (UInt(Rn) == 15 && ((R == 0)))',
};

CLASS PLD_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=PLD_r xid=t1',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS PLI_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111010 D=0b0 U:u=0bx R=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_imm page=PLI_i xid=a1',
};

CLASS PLI_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_pimm page=PLI_i xid=t1',
};

CLASS PLI_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b1100 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_nimm page=PLI_i xid=t2',
};

CLASS PLI_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11111001 U:u=0bx size=0b00 ig1=0b11111 Rt=0b1111 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T3 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldlit_signed page=PLI_i xid=t3',
};

CLASS PLI_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111011 D=0b0 U:u=0bx o2=0b1 ig1=0b01 Rn:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PLI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=preload_reg page=PLI_r xid=a1',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS PLI_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 size=0b00 ig1=0b1 Rn:u=0bxxxx Rt=0b1111 ig2=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=PLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_signed_reg page=PLI_r xid=t1',
  unpredict  => 'UInt(Rm) == 15',
};

CLASS POP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011 L=0b1 ig1=0b10 P:u=0bx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=POP',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=pushpop16 page=POP xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(BitCount((P:0:register_list)) < 1) || ((P:0:register_list)<15> == 1 && InITBlock() && !LastInITBlock())',
};

CLASS PSSBB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option=0b0100',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=PSSBB xid=a1',
};

CLASS PSSBB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option=0b0100',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PSSBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=PSSBB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS PUSH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011 L=0b0 ig1=0b10 M:u=0bx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=PUSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=pushpop16 page=PUSH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'BitCount((0:M:0:register_list)) < 1',
};

CLASS QADD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QADD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QADD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QDADD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QDADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QDADD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QDADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QDADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QDADD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QDSUB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QDSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QDSUB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QDSUB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QDSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QDSUB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b0101 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=intsat page=QSUB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=QSUB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b010 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=QSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=QSUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS QSUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=QSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=QSUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS RBIT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b1 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b0 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=RBIT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=RBIT xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS RBIT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RBIT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=RBIT xid=t1',
  unpredict  => 'UInt(Rm) != UInt(Rn) || UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REV_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b0 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b0 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REV xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REV_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b00 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REV xid=t1',
};

CLASS REV_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REV xid=t2',
  unpredict  => 'UInt(Rm) != UInt(Rn) || UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REV16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b0 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b1 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REV16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REV16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REV16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b01 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REV16',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REV16 xid=t1',
};

CLASS REV16_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REV16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REV16 xid=t2',
  unpredict  => 'UInt(Rm) != UInt(Rn) || UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REVSH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 o1=0b1 ig1=0b11 ig2=0bS ig3=0bS ig4=0bS ig5=0bS Rd:u=0bxxxx ig6=0bS ig7=0bS ig8=0bS ig9=0bS o2=0b1 ig10=0b011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=reverse page=REVSH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS REVSH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111010 op=0b11 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=rev16 page=REVSH xid=t1',
};

CLASS REVSH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=REVSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=REVSH xid=t2',
  unpredict  => 'UInt(Rm) != UInt(Rn) || UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS RFE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111100 P:u=0bx U:u=0bx S=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx op=0bZZZZSZSZZZZ mode=0bZZZZZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=RFE xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS RFE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b00 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P=0bS M=0bS register_list=0bZZZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 ldmstm-mode=dec-before mnemonic=RFEDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=RFE xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15) || (InITBlock() && !LastInITBlock())',
};

CLASS RFE_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b11 ig1=0b0 W:u=0bx L=0b1 Rn:u=0bxxxx P=0bS M=0bS register_list=0bZZZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 ldmstm-mode=inc-after mnemonic=RFE{IA}',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=RFE xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15) || (InITBlock() && !LastInITBlock())',
};

CLASS RSB_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b011 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSB_i xid=a1',
};

CLASS RSB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1001 Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=RSB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=RSB_i xid=t1',
};

CLASS RSB_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1110 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=RSB_i xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS RSB_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSB_r xid=a1',
};

CLASS RSB_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110101 op1=0b1110 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RSB_r xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS RSB_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b011 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSB_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS RSC_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b111 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=RSC_i xid=a1',
};

CLASS RSC_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=RSC_r xid=a1',
};

CLASS RSC_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=RSC_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS SADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0111 option=0bZZZZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=SB xid=a1',
};

CLASS SB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0111 option=0bZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=SB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS SBC_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b110 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SBC_i xid=a1',
};

CLASS SBC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1011 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SBC_i xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SBC_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SBC_r xid=a1',
};

CLASS SBC_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b0110 Rm:u=0bxxx Rdn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=SBC_r xid=t1',
};

CLASS SBC_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b1011 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SBC_r xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SBC_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SBC_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS SBFX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111 U=0b0 ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SBFX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfx page=SBFX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SBFX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b010 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SBFX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SBFX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SDIV_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b001 Rd:u=0bxxxx Ra:u=0bSSSS Rm:u=0bxxxx op2=0b000 ig1=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SDIV xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) != 15',
};

CLASS SDIV_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b001 Rn:u=0bxxxx Ra:u=0bSSSS Rd:u=0bxxxx op2=0b1111 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SDIV xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) != 15',
};

CLASS SEL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101000 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b1011 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=selbytes page=SEL xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SEL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b10 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=dpint_2r page=SEL xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SETEND_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100010000 imod=0bZZ M=0bZ op=0b1 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ E:u=0bx A=0bZ I=0b0 F=0b0 ig7=0b0 mode=0b0ZZZZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SETEND',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=cps page=SETEND xid=a1',
};

CLASS SETEND_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1011011001 op=0b0 ig1=0bS E:u=0bx ig2=0bZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SETEND',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=cps16 page=SETEND xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS SETPAN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100010001 ig1=0bZ ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ ig10=0bZ imm1:u=0bx ig11=0bZ ig12=0b0000 ig13=0bZ ig14=0bZ ig15=0bZ ig16=0bZ',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SETPAN',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=uncondmisc iclass=setpan page=SETPAN xid=a1',
  undefined  => '!HavePANExt()',
};

CLASS SETPAN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110110000 ig1=0bS imm1:u=0bx ig2=0bZ ig3=0bZ ig4=0bZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SETPAN',
  metadata   => 'alias=0 isa=THUMB isaform=T16 arch_variant=ARMv8v1 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=setpan16 page=SETPAN xid=t1',
  undefined  => '!HavePANExt()',
  unpredict  => 'InITBlock()',
};

CLASS SEV_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=SEV xid=a1',
};

CLASS SEV_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0100 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEV',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=SEV xid=t1',
};

CLASS SEV_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0100',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SEV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=SEV xid=t2',
};

CLASS SEVL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000101',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=SEVL xid=a1',
};

CLASS SEVL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0101 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=SEVL xid=t1',
};

CLASS SEVL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SEVL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=SEVL xid=t2',
};

CLASS SHADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b011 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SHSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SHSUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SHSUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SHSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SHSUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 imm12=0bZZZZZZZZZZZZ ig2=0b0111 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpmisc iclass=except page=SMC xid=a1',
  exceptions => 'MONITORCALL',
};

CLASS SMC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110111111 o1=0b1 imm4:u=0bxxxx ig1=0b10 o2=0b0 ig2=0b0 imm12=0bZZZZZZZZZZZZ',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=SMC xid=t1',
  exceptions => 'MONITORCALL',
  unpredict  => 'InITBlock() && !LastInITBlock()',
};

CLASS SMLABB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0bx N:u=0bx ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLABB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS SMLABB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 N:u=0bx M:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLABB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLAD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLAD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLAD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLAL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b111 S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMLAL xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLAL xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLALBB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b10 ig1=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0bx N:u=0bx ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLALBB xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLALBB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b10 N:u=0bx M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALBB xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLALD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b00 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLALD xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLALD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b100 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLALD xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLAWB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig2=0b1 M:u=0bx N=0b0 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMLAWB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS SMLAWB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLAWB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLSD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLSD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMLSD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMLSLD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b100 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b01 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMLSLD xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMLSLD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b101 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx ig1=0b110 M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMLSLD xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMMLA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b00 R:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLA xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLA xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMMLS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b11 R:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMLS xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS SMMLS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b110 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMLS xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) == 15',
};

CLASS SMMUL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b101 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 R:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMMUL xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMMUL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b101 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 R:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMMUL xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMUAD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b00 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUAD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMUAD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b010 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUAD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMULBB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b11 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0bx N:u=0bx ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULBB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMULBB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b001 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 N:u=0bx M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULBB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMULL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b110 S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=SMULL xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMULL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b000 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=SMULL xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS SMULWB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b0 Rd:u=0bxxxx Ra=0bZZZZ Rm:u=0bxxxx ig2=0b1 M:u=0bx N=0b1 ig3=0b0 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_half page=SMULWB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMULWB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b011 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMULWB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMUSD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b01 M:u=0bx ig2=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=SMUSD xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SMUSD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b100 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 ig2=0b0 M:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=SMUSD xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SRS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111100 P:u=0bx U:u=0bx S=0b1 W:u=0bx L=0b0 Rn=0bSSZS op=0bZZZZZSZSZZZ mode:u=0bxxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstexcept page=SRS xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
};

CLASS SRS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b00 ig1=0b0 W:u=0bx L=0b0 Rn=0bSSZS P=0bS M=0bS ig2=0bZZZZZZZZZ mode:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 ldmstm-mode=dec-before mnemonic=SRSDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=SRS xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
};

CLASS SRS_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b11 ig1=0b0 W:u=0bx L=0b0 Rn=0bSSZS P=0bS M=0bS ig2=0bZZZZZZZZZ mode:u=0bxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 ldmstm-mode=inc-after mnemonic=SRS{IA}',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=SRS xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT|UNCATEGORIZED',
};

CLASS SSAT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0bx ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=SSAT xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SSAT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b00 sh:u=0bx ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SSAT xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SSAT16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAT16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat16 page=SSAT16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SSAT16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b001 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3=0b000 Rd:u=0bxxxx imm2=0b00 ig5=0bZ ig6=0bZ sat_imm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAT16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=SSAT16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS SSAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SSAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SSBB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101010111 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0bS ig7=0bS ig8=0bS ig9=0bZ ig10=0bZ ig11=0bZ ig12=0bZ opcode=0b0100 option=0b0000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSBB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=uncondhints iclass=barriers page=SSBB xid=a1',
};

CLASS SSBB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111011 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bS ig9=0bS ig10=0bS ig11=0bS opc=0b0100 option=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSBB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=system page=SSBB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS SSUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SSUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SSUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b001 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=SSUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SSUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b0 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=SSUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS STC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx CRd=0b0101 ig1=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=ldstcp page=STC xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
  unpredict  => 'UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)',
};

CLASS STC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 o0=0b0 ig1=0b110 P:u=0bx U:u=0bx D=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx CRd=0b0101 ig2=0b111 cp15=0b0 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=sysldst_mov64 iclass=cp_ldst page=STC xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && U == 0 && W == 0',
  unpredict  => 'UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)',
};

CLASS STL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STL xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b10 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STL xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STLB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STLB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b00 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STLEX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEX xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLEX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b10 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEX xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLEXB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLEXB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b00 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLEXD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXD xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || Rt<0> == 1 || (UInt(Rt)+1) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt) || UInt(Rd) == (UInt(Rt)+1))',
};

CLASS STLEXD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b1 op=0b1 sz=0b11 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXD xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt) || UInt(Rd) == UInt(Rt2))',
};

CLASS STLEXH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLEXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLEXH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLEXH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b1 sz=0b01 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLEXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLEXH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STLH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx xRd=0bSSSS ig1=0bS ig2=0bS ex=0b0 ord=0b0 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STLH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STLH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STLH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b1 op=0b0 sz=0b01 Rd=0bSSSS',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STLH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldastl page=STLH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15',
};

CLASS STM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STM xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS STM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1100 L=0b0 Rn:u=0bxxx register_list:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldstm16 page=STM xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'BitCount((0:register_list)) < 1',
};

CLASS STM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=STM xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1)',
};

CLASS STM_u_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P:u=0bx U:u=0bx op=0b1 W=0bZ L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STM_u xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS STMDA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b0 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMDA xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS STMDB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMDB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS STMDB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=STMDB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1)',
};

CLASS STMIB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b1 op=0b0 W:u=0bx L=0b0 Rn:u=0bxxxx register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=STMIB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS STR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STR_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt))',
};

CLASS STR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b0 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=STR_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STR_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1001 L=0b0 Rt:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_sp page=STR_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STR_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110001 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STR_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS STR_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=STR_i xid=t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15 || (P == 0 && W == 0)',
  unpredict  => 'UInt(Rt) == 15 || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS STR_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0bx U:u=0bx o2=0b0 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STR_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS STR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b0 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STR_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STR_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STR_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rm) == 15',
};

CLASS STRB_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRB_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS STRB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b011 B=0b1 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_imm page=STRB_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STRB_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STRB_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS STRB_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=STRB_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15 || (P == 0 && W == 0)',
  unpredict  => 'UInt(Rt) == 15 || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS STRB_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P:u=0bx U:u=0bx o2=0b1 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRB_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS STRB_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b1 H=0b0 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STRB_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STRB_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STRB_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rm) == 15',
};

CLASS STRBT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRBT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS STRBT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRBT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS STRBT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b00 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRBT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRBT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS STRD_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b11 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRD_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(Rt<0> == 1) || (P == 0 && W == 1) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rn) == (UInt(Rt)+1))) || ((UInt(Rt)+1) == 15)',
};

CLASS STRD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STRD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd page=STRD_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(((W == 1)) && (UInt(Rn) == UInt(Rt) || UInt(Rn) == UInt(Rt2))) || (UInt(Rn) == 15 || UInt(Rt) == 15 || UInt(Rt2) == 15)',
};

CLASS STRD_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b11 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRD_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(Rt<0> == 1) || (P == 0 && W == 1) || ((UInt(Rt)+1) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rn) == (UInt(Rt)+1)))',
};

CLASS STREX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b00 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREX xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STREX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000010 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex page=STREX xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STREXB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b10 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STREXB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b00 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STREXD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b01 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXD xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || Rt<0> == 1 || (UInt(Rt)+1) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt) || UInt(Rd) == (UInt(Rt)+1))',
};

CLASS STREXD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2:u=0bxxxx ig1=0b01 sz=0b11 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXD xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt) || UInt(Rd) == UInt(Rt2))',
};

CLASS STREXH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 size=0b11 L=0b0 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ex=0b1 ord=0b1 ig3=0b1001 Rt:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STREXH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=sync iclass=ldst_excl page=STREXH xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STREXH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101000110 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx Rt2=0bSSSS ig1=0b01 sz=0b01 Rd:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STREXH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=ldstex_bhd page=STREXH xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rd) == 15 || UInt(Rt) == 15 || UInt(Rn) == 15) || (UInt(Rd) == UInt(Rn) || UInt(Rd) == UInt(Rt))',
};

CLASS STRH_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b1 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRH_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS STRH_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1000 L=0b0 imm5:u=0bxxxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldsth16_imm page=STRH_i xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STRH_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110001 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pimm page=STRH_i xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS STRH_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1 P:u=0bx U:u=0bx W:u=0bx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst page=STRH_i xid=t3',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15 || (P == 0 && W == 0)',
  unpredict  => 'UInt(Rt) == 15 || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS STRH_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P:u=0bx U:u=0bx ig1=0b0 W:u=0bx o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRH_r xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rm) == 15) || (((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)))',
};

CLASS STRH_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b0101 L=0b0 B=0b0 H=0b1 Rm:u=0bxxx Rn:u=0bxxx Rt:u=0bxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=ldst16_reg page=STRH_r xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
};

CLASS STRH_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b000000 imm2:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=register-offset armarmheading=T2 instr-class=general isa=T32 mnemonic=STRH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_reg page=STRH_r xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rm) == 15',
};

CLASS STRHT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b1 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm4H:u=0bxxxx ig2=0b1 op2=0b01 ig3=0b1 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstximm page=STRHT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS STRHT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b000 P=0b0 U:u=0bx ig1=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ ig6=0b1 op2=0b01 ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=xldst iclass=ldstxreg page=STRHT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rt) == 15 || UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS STRHT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b01 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRHT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRHT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS STRT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=STRT xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rn) == UInt(Rt)',
};

CLASS STRT_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b011 P=0b0 U:u=0bx o2=0b0 W=0b1 o1=0b0 Rn:u=0bxxxx Rt:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=general isa=A32 mnemonic=STRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstreg page=STRT xid=a2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rn) == UInt(Rt) || UInt(Rm) == 15',
};

CLASS STRT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0bxxxx Rt:u=0bxxxx ig1=0b1110 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=general isa=T32 mnemonic=STRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_unpriv page=STRT xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS SUB_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_i xid=a1',
};

CLASS SUB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000111 S=0b1 imm3:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_2l_imm page=SUB_i xid=t1',
};

CLASS SUB_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b001 op=0b11 Rdn:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_1l_imm page=SUB_i xid=t2',
};

CLASS SUB_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0bx Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1101',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_i xid=t3',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15',
};

CLASS SUB_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b11x1',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=SUBW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_i xid=t4',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS SUB_i_t5 => {
  name       => 'T5',
  diagram    => 'ig0=0b111100111101 Rn:u=0bSSSZ ig1=0b10 ig2=0bZ ig3=0b0 ig4=0bS ig5=0bS ig6=0bS ig7=0bS imm8:u=0bxxxxxxxx',
  cnsts      => 'imm8!=0b00000000',
  docvars    => 'armarmheading=T5 instr-class=general isa=T32 mnemonic=SUBS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=eret page=SUB_i xid=t5',
  unpredict  => '(UInt(Rn) != 14) || (InITBlock() && !LastInITBlock())',
};

CLASS SUB_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1101',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_r xid=a1',
};

CLASS SUB_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b000110 S=0b1 Rm:u=0bxxx Rn:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=addsub16_3l page=SUB_r xid=t1',
};

CLASS SUB_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0bx Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1101',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_r xid=t2',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS SUB_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0bx Rn:u=0bxxxx Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0bxx ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp3reg_regsh page=SUB_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS SUB_SP_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S:u=0bx Rn=0b1101 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_SP_i xid=a1',
};

CLASS SUB_SP_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110000 S=0b1 imm7:u=0bxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SUB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=adjsp16 page=SUB_SP_i xid=t1',
};

CLASS SUB_SP_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b1101 S:u=0bx Rn=0b1101 ig2=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=SUB_SP_i xid=t2',
  unpredict  => 'UInt(Rd) == 15 && !((S == 1))',
};

CLASS SUB_SP_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1101 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=SUBW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_SP_i xid=t3',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS SUB_SP_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S:u=0bx Rn=0b1101 Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0bxx ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp3reg_immsh page=SUB_SP_r xid=a1',
};

CLASS SUB_SP_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110101 op1=0b1101 S:u=0bx Rn=0b1101 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=SUB_SP_r xid=t1',
  unpredict  => '(UInt(Rd) == 15 && !((S == 1))) || UInt(Rm) == 15',
};

CLASS SVC_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1111 imm24:u=0bxxxxxxxxxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SVC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=svcall iclass=svc page=SVC xid=a1',
  exceptions => 'SUPERVISORCALL',
};

CLASS SVC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101111 S=0b1 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SVC',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=except16 page=SVC xid=t1',
  exceptions => 'SUPERVISORCALL',
};

CLASS SXTAB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b10 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTAB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTAB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b00 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTAB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTAH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b11 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTAH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTAH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b0 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTAH xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b10 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b0 B=0b1 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=SXTB xid=t1',
};

CLASS SXTB_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTB xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b00 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b0 op=0b11 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=SXTH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SXTH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b0 B=0b0 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=SXTH xid=t1',
};

CLASS SXTH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b0 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=SXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=SXTH xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS TBB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111010001101 Rn:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ ig9=0b000 H:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=dstd iclass=tblbr page=TBB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rm) == 15) || (InITBlock() && !LastInITBlock())',
};

CLASS TEQ_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=TEQ_i xid=a1',
};

CLASS TEQ_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0100 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=TEQ_i xid=t1',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS TEQ_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TEQ_r xid=a1',
};

CLASS TEQ_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110101 op1=0b0100 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TEQ_r xid=t1',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS TEQ_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b01 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=TEQ_rr xid=a1',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS TSB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00010010',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TSB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=TSB xid=a1',
  unpredict  => 'cond != 14',
};

CLASS TSB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0001 option=0b0010',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TSB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v4 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=TSB xid=t1',
  unpredict  => 'InITBlock()',
};

CLASS TST_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp1reg_imm page=TST_i xid=a1',
};

CLASS TST_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b0 op1=0b0000 S=0b1 Rn:u=0bxxxx ig2=0b0 imm3:u=0bxxx Rd=0b1111 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TST',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_immm page=TST_i xid=t1',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS TST_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ imm5:u=0bxxxxx stype:u=0bxx ig6=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=intdp2reg_immsh page=TST_r xid=a1',
};

CLASS TST_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op=0b1000 Rm:u=0bxxx Rn:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=TST',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=TST_r xid=t1',
};

CLASS TST_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110101 op1=0b0000 S=0b1 Rn:u=0bxxxx ig1=0bZ imm3:u=0bxxx Rd=0b1111 imm2:u=0bxx stype:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=TST',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=TST_r xid=t2',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS TST_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00010 opc=0b00 ig1=0b1 Rn:u=0bxxxx ig2=0bZ ig3=0bZ ig4=0bZ ig5=0bZ Rs:u=0bxxxx ig6=0b0 stype:u=0bxx ig7=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=TST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=intdp2reg_regsh page=TST_rr xid=a1',
  unpredict  => 'UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS UADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UBFX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111 U=0b1 ig1=0b1 widthm1:u=0bxxxxx Rd:u=0bxxxx lsb:u=0bxxxxx ig2=0b101 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UBFX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=bfx page=UBFX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS UBFX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b110 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig5=0bZ widthm1:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UBFX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=UBFX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS UDF_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0b1110 ig0=0b01111111 imm12:u=0bxxxxxxxxxxxx ig1=0b1111 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UDF',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=udf page=UDF xid=a1',
};

CLASS UDF_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1101111 S=0b0 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UDF',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=brc iclass=except16 page=UDF xid=t1',
};

CLASS UDF_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110111111 o1=0b1 imm4:u=0bxxxx ig1=0b10 o2=0b1 ig2=0b0 imm12:u=0bxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UDF',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=except page=UDF xid=t2',
};

CLASS UDIV_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01110 op1=0b011 Rd:u=0bxxxx Ra:u=0bSSSS Rm:u=0bxxxx op2=0b000 ig1=0b1 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=smul_div page=UDIV xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) != 15',
};

CLASS UDIV_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b011 Rn:u=0bxxxx Ra:u=0bSSSS Rd:u=0bxxxx op2=0b1111 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UDIV xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15 || UInt(Ra) != 15',
};

CLASS UHADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b111 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UHSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UHSUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UHSUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b1 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UHSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UHSUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UMAAL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b010 S=0b0 RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UMAAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMAAL xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UMAAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b110 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0110 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMAAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMAAL xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UMLAL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b101 S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMLAL xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UMLAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b110 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMLAL xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UMULL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0000 opc=0b100 S:u=0bx RdHi:u=0bxxxx RdLo:u=0bxxxx Rm:u=0bxxxx ig1=0b1001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dp iclass=mul_word page=UMULL xid=a1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UMULL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110111 op1=0b010 Rn:u=0bxxxx RdLo:u=0bxxxx RdHi:u=0bxxxx op2=0b0000 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=lmul_div page=UMULL xid=t1',
  unpredict  => '(UInt(RdLo) == 15 || UInt(RdHi) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15) || (UInt(RdHi) == UInt(RdLo))',
};

CLASS UQADD16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQADD16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQADD16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQADD16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b001 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQADD16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQADD16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQADD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b00 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQADD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQADD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQADD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b000 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQADD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQADD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQASX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b01 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQASX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQASX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQASX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b010 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQASX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQASX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b110 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UQSUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=UQSUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UQSUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b1 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UQSUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=UQSUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USAD8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111000 Rd:u=0bxxxx Ra=0b1111 Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAD8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=usad page=USAD8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USAD8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b111 Rn:u=0bxxxx Ra=0b1111 Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAD8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=USAD8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USADA8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01111000 Rd:u=0bxxxx Ra:u=0bxxxx Rm:u=0bxxxx ig1=0b0001 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Ra!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USADA8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=usad page=USADA8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USADA8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110110 op1=0b111 Rn:u=0bxxxx Ra:u=0bxxxx Rd:u=0bxxxx ig1=0b00 op2=0b00 Rm:u=0bxxxx',
  cnsts      => 'Ra!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USADA8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=mul iclass=mul_abd page=USADA8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USAT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 ig1=0b1 sat_imm:u=0bxxxxx Rd:u=0bxxxx imm5:u=0bxxxxx sh:u=0bx ig2=0b01 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat32 page=USAT xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS USAT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 ig3=0b10 sh:u=0bx ig4=0b0 Rn:u=0bxxxx ig5=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx ig6=0bZ sat_imm:u=0bxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=USAT xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS USAT16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 ig1=0b10 sat_imm:u=0bxxxx Rd:u=0bxxxx ig2=0bS ig3=0bS ig4=0bS ig5=0bS ig6=0b0011 Rn:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAT16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=sat16 page=USAT16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS USAT16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11110 ig1=0bZ ig2=0b11 op1=0b101 ig3=0b0 Rn:u=0bxxxx ig4=0b0 imm3=0b000 Rd:u=0bxxxx imm2=0b00 ig5=0bZ ig6=0bZ sat_imm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAT16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=sat_bit page=USAT16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15',
};

CLASS USAX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b10 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USAX xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USAX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b110 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USAX xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USUB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b0 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USUB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USUB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USUB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b101 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USUB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USUB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USUB8_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01100 op1=0b101 Rn:u=0bxxxx Rd:u=0bxxxx ig1=0bS ig2=0bS ig3=0bS ig4=0bS B=0b1 op2=0b11 ig5=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=USUB8',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=parallel page=USUB8 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS USUB8_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110101 op1=0b100 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0 U=0b1 H=0b0 S=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=USUB8',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=addsub_par page=USUB8 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rn) == 15 || UInt(Rm) == 15',
};

CLASS UXTAB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b10 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTAB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAB xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTAB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b00 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTAB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTAH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b11 Rn:u=0bxxxx Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTAH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTAH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b1 Rn:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTAH xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b10 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTB xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b1 B=0b1 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=UXTB xid=t1',
};

CLASS UXTB_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b10 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UXTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTB xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTB16_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b00 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTB16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTB16 xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTB16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110100 op1=0b01 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTB16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTB16 xid=t1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTH_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b01101 U=0b1 op=0b11 Rn=0b1111 Rd:u=0bxxxx rotate:u=0bxx ig1=0bZ ig2=0bZ ig3=0b0111 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=media iclass=extend page=UXTH xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS UXTH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10110010 U=0b1 B=0b0 Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=ext16 page=UXTH xid=t1',
};

CLASS UXTH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 op1=0b00 U=0b1 Rn=0b1111 ig1=0b1111 Rd:u=0bxxxx ig2=0b1 ig3=0bZ rotate:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=UXTH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=extendr page=UXTH xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS WFE_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000010',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=WFE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=WFE xid=a1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS WFE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0010 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=WFE',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=WFE xid=t1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS WFE_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0010',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=WFE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=WFE xid=t2',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS WFI_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000011',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=WFI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=WFI xid=a1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS WFI_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0011 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=WFI',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=WFI xid=t1',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS WFI_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0011',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=WFI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=WFI xid=t2',
  exceptions => 'UNCATEGORIZED|WFXTRAP',
};

CLASS YIELD_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00110 R=0b0 ig1=0b10 imm4=0b0000 ig2=0bS ig3=0bS ig4=0bS ig5=0bS imm12=0bZZZZ00000001',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=movsr_hint_imm page=YIELD xid=a1',
};

CLASS YIELD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b10111111 hint=0b0001 ig1=0b0000',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=misc16 iclass=hints16 page=YIELD xid=t1',
};

CLASS YIELD_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111100111010 ig1=0bS ig2=0bS ig3=0bS ig4=0bS ig5=0b10 ig6=0bZ ig7=0b0 ig8=0bZ ig9=0b000 hint=0b0000 option=0b0001',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=YIELD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=bcrtrl iclass=hints page=YIELD xid=t2',
};

CLASS ADD_ADR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b100 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=ADD_ADR xid=a1',
};

CLASS ADD_ADR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1010 SP=0b0 Rd:u=0bxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=addpcsp16 page=ADD_ADR xid=t1',
};

CLASS ADD_ADR_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b0 ig2=0b0 o2=0b0 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=ADD_ADR xid=t3',
  unpredict  => 'UInt(Rd) == 15',
};

CLASS ASR_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b10 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ASR_MOV_r xid=a1',
};

CLASS ASR_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b10 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=ASR_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS ASR_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ASR_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS ASR_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b10 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ASR_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ASR_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0100 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ASR_MOV_rr xid=t1',
};

CLASS ASR_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b10 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ASR_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ASRS_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b10 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ASRS_MOV_r xid=a1',
};

CLASS ASRS_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b10 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=ASRS_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS ASRS_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b10 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ASRS_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS ASRS_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b10 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ASRS_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ASRS_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0100 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ASRS_MOV_rr xid=t1',
};

CLASS ASRS_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b10 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ASRS_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSL_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b00 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSL_MOV_r xid=a1',
};

CLASS LSL_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b00 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'imm5!=0b00000',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSL_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS LSL_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSL_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS LSL_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b00 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSL_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSL_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0010 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSL_MOV_rr xid=t1',
};

CLASS LSL_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b00 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSL_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSLS_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b00 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSLS_MOV_r xid=a1',
};

CLASS LSLS_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b00 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  cnsts      => 'imm5!=0b00000',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSLS_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS LSLS_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b00 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSLS_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS LSLS_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b00 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSLS_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSLS_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0010 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSLS_MOV_rr xid=t1',
};

CLASS LSLS_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b00 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSLS_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSR_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b01 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSR_MOV_r xid=a1',
};

CLASS LSR_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b01 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSR_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS LSR_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSR_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS LSR_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b01 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSR_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSR_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0011 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSR_MOV_rr xid=t1',
};

CLASS LSR_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b01 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSR_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSRS_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b01 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=LSRS_MOV_r xid=a1',
};

CLASS LSRS_MOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b000 op:u=0b01 imm5:u=0bxxxxx Rm:u=0bxxx Rd:u=0bxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=sftdpi iclass=shift16_imm page=LSRS_MOV_r xid=t2',
  unpredict  => 'op == 0 && imm5 == 0 && InITBlock()',
};

CLASS LSRS_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b01 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=LSRS_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS LSRS_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b01 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=LSRS_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS LSRS_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0011 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=LSRS_MOV_rr xid=t1',
};

CLASS LSRS_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b01 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=LSRS_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS POP_LDM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b0 U=0b1 op=0b0 W:u=0b1 L=0b1 Rn:u=0b1101 register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDM',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=POP_LDM xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((register_list)) < 1) || (((W == 1)) && (register_list)<UInt(Rn)> == 1)',
};

CLASS POP_LDM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110100 opc=0b01 ig1=0b0 W:u=0b1 L=0b1 Rn:u=0b1101 P:u=0bx M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=LDM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=POP_LDM xid=t2',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2 || (P == 1 && M == 1)) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1 && InITBlock() && !LastInITBlock())',
};

CLASS POP_LDR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b0 U:u=0b1 o2=0b0 W:u=0b0 o1=0b1 Rn:u=0b1101 Rt:u=0bxxxx imm12:u=0b000000000100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=LDR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=POP_LDR_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '((P == 0) || (W == 1)) && UInt(Rn) == UInt(Rt)',
};

CLASS POP_LDR_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b1 Rn:u=0b1101 Rt:u=0bxxxx ig1=0b1 P:u=0b0 U:u=0b1 W:u=0b1 imm8:u=0b00000100',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=LDR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_post page=POP_LDR_i xid=t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'P == 0 && W == 0',
  unpredict  => '(((W == 1)) && UInt(Rn) == UInt(Rt)) || (UInt(Rt) == 15 && InITBlock() && !LastInITBlock())',
};

CLASS PUSH_STMDB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b100 P=0b1 U=0b0 op=0b0 W:u=0b1 L=0b0 Rn:u=0b1101 register_list:u=0bxxxxxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STM',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=brblk iclass=ldstm page=PUSH_STMDB xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => 'UInt(Rn) == 15 || BitCount((register_list)) < 1',
};

CLASS PUSH_STMDB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110100 opc=0b10 ig1=0b0 W:u=0b1 L=0b0 Rn:u=0b1101 P:u=0bZ M:u=0bx register_list:u=0bxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=STM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=ldstm page=PUSH_STMDB xid=t1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '(UInt(Rn) == 15 || BitCount((P:M:register_list)) < 2) || (((W == 1)) && (P:M:register_list)<UInt(Rn)> == 1) || ((P:M:register_list)<13> == 1) || ((P:M:register_list)<15> == 1)',
};

CLASS PUSH_STR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b010 P:u=0b1 U:u=0b0 o2=0b0 W:u=0b1 o1=0b0 Rn:u=0b1101 Rt:u=0bxxxx imm12:u=0b000000000100',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32 mnemonic=STR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'iclass=ldstimm page=PUSH_STR_i xid=a1',
  exceptions => 'ALIGNMENT|DATAABORT',
  unpredict  => '((P == 0) || (W == 1)) && (UInt(Rn) == 15 || UInt(Rn) == UInt(Rt))',
};

CLASS PUSH_STR_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110000 size=0b10 L=0b0 Rn:u=0b1101 Rt:u=0bxxxx ig1=0b1 P:u=0b1 U:u=0b0 W:u=0b1 imm8:u=0b00000100',
  docvars    => 'armarmheading=T4 instr-class=general isa=T32 mnemonic=STR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=ldst iclass=ldst_unsigned_pre page=PUSH_STR_i xid=t4',
  exceptions => 'ALIGNMENT|DATAABORT',
  undefined  => 'Rn == 15 || (P == 0 && W == 0)',
  unpredict  => 'UInt(Rt) == 15 || (((W == 1)) && UInt(Rn) == UInt(Rt))',
};

CLASS ROR_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=ROR_MOV_r xid=a1',
};

CLASS ROR_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=ROR_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS ROR_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b11 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=ROR_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS ROR_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0111 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=ROR_MOV_rr xid=t1',
};

CLASS ROR_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b11 S:u=0b0 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=ROR_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS RORS_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0bxxxxx stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111 imm5!=0b00000',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RORS_MOV_r xid=a1',
};

CLASS RORS_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0bxxx Rd:u=0bxxxx imm2:u=0bxx stype:u=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RORS_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS RORS_MOV_rr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx Rs:u=0bxxxx ig1=0b0 stype:u=0b11 ig2=0b1 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregrs iclass=logic3reg_regsh page=RORS_MOV_rr xid=a1',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS RORS_MOV_rr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b010000 op:u=0b0111 Rs:u=0bxxx Rdm:u=0bxxx',
  docvars    => 'armarmheading=T1 instr-class=general isa=T32 mnemonic=MOV',
  metadata   => 'alias=1 isa=THUMB isaform=T16 alphaindex=BASIC',
  tags       => 'group=n iclass=dpint16_2l page=RORS_MOV_rr xid=t1',
};

CLASS RORS_MOV_rr_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110100 stype:u=0b11 S:u=0b1 Rm:u=0bxxxx ig1=0b1111 Rd:u=0bxxxx ig2=0b0000 Rs:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=reg iclass=shiftr page=RORS_MOV_rr xid=t2',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15 || UInt(Rs) == 15',
};

CLASS RRX_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b0 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RRX_MOV_r xid=a1',
};

CLASS RRX_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b0 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RRX_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS RRXS_MOV_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b00011 opc=0b01 S:u=0b1 Rn=0bZZZZ Rd:u=0bxxxx imm5:u=0b00000 stype:u=0b11 ig1=0b0 Rm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=general isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpregis iclass=logic3reg_immsh page=RRXS_MOV_r xid=a1',
};

CLASS RRXS_MOV_r_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b1110101 op1=0b0010 S:u=0b1 Rn=0b1111 ig1=0bZ imm3:u=0b000 Rd:u=0bxxxx imm2:u=0b00 stype:u=0b11 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=general isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=w iclass=dpint_shiftr page=RRXS_MOV_r xid=t3',
  unpredict  => 'UInt(Rd) == 15 || UInt(Rm) == 15',
};

CLASS SUB_ADR_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b0010 opc=0b010 S=0b0 Rn=0b1111 Rd:u=0bxxxx imm12:u=0bxxxxxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=general isa=A32 mnemonic=ADR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=BASIC',
  tags       => 'group=dpimm iclass=intdp2reg_imm page=SUB_ADR xid=a2',
};

CLASS SUB_ADR_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11110 i:u=0bx ig1=0b10 o1=0b1 ig2=0b0 o2=0b1 ig3=0b0 Rn=0b1111 ig4=0b0 imm3:u=0bxxx Rd:u=0bxxxx imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 instr-class=general isa=T32 mnemonic=ADR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=BASIC',
  tags       => 'group=imm iclass=dpint_imms page=SUB_ADR xid=t2',
  unpredict  => 'UInt(Rd) == 15',
};


1;

