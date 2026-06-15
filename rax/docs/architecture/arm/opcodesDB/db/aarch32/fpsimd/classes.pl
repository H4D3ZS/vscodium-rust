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

CLASS AESD_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESD xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS AESD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESD xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS AESE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS AESE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS AESIMC_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESIMC xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS AESIMC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESIMC xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS AESMC_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESMC xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS AESMC_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESMC xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAESExt()) || (size != 0) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS FLDMX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FLDMX xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS FLDMX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FLDMX xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS FSTMX_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FSTMX xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS FSTMX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FSTMX xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS SHA1C_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1C xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1C_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1C xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA1H_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 opc2=0b0101 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA1H xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1H_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 opc2=0b0101 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA1H xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA1M_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1M xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1M_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1M xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA1P_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1P xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1P_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1P xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA1SU0_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1SU0 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1SU0_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1SU0 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA1SU1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA1SU1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS SHA1SU1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA1SU1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA1Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA256H_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256H xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA256H_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256H xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA256H2_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256H2 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA256H2_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256H2 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA256SU0_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA256SU0 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS SHA256SU0_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA256SU0 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (size != 2) || (Vd<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS SHA256SU1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256SU1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS SHA256SU1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256SU1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveSHA256Ext()) || (Q != 1) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS ST4_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=ST4_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1:0> == 3)',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VABA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VABA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VABAL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VABAL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VABAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VABAL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VABD_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VABD_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VABD_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VABD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VABDL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABDL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VABDL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VABDL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABDL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VABDL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VABS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VABS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VABS_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VABS xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VABS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VABS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VABS_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VABS xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VACGE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VACGE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VACGT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VACGT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VADD_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VADD_f_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VADD_f xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VADD_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VADD_f_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VADD_f xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VADD_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VADD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VADDHN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDHN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDHN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VADDHN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDHN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDHN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VADDL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VADDL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VADDW_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDW',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDW xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VADDW_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDW xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VAND_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VAND',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VAND_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VAND_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VAND',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VAND_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIC_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VBIC_i_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VBIC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VBIC_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VBIC_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIC_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIC_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIC_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIF_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIF',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIF xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIF_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIF',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIF xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBIT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBSL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBSL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VBSL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBSL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VCADD_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCADD xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
};

CLASS VCADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCADD xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
  unpredict  => 'InITBlock()',
};

CLASS VCEQ_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCEQ_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCEQ_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCEQ_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VCEQ_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCEQ_r_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VCEQ_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCEQ_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VCGE_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGE_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCGE_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGE_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VCGE_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCGE_r_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VCGE_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCGE_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VCGT_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGT_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCGT_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGT_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VCGT_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCGT_r_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VCGT_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCGT_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VCLE_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLE_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCLE_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLE_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VCLS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCLS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCLT_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLT_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCLT_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLT_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VCLZ_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLZ xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCLZ_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLZ xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCMLA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
};

CLASS VCMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
  unpredict  => 'InITBlock()',
};

CLASS VCMLA_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 S:u=0bx D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VCMLA_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
};

CLASS VCMLA_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 S:u=0bx D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VCMLA_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1)) || (!HaveFP16Ext() && (16 << UInt(S)) == 16)',
  unpredict  => 'InITBlock()',
};

CLASS VCMP_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCMP_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCMP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCMP_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCMPE_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCMPE_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCMPE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCMPE_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0bxx E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCNT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCNT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCNT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCNT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCNT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCNT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCVT_bfs_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size=0b01 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1100 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_bfs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vm<0> == 1)',
};

CLASS VCVT_bfs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size=0b01 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1100 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_bfs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vm<0> == 1)',
};

CLASS VCVT_ds_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b1x o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_ds xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVT_ds_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b1x o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_ds xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVT_hs_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bx ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_hs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size != 1) || (((op == 1)) && Vd<0> == 1) || (!((op == 1)) && Vm<0> == 1)',
};

CLASS VCVT_hs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bx ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_hs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size != 1) || (((op == 1)) && Vd<0> == 1) || (!((op == 1)) && Vm<0> == 1)',
};

CLASS VCVT_is_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_is xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VCVT_is_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_is xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCVT_iv_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b10x Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCVT_iv_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b10x Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCVT_vi_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_vi xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCVT_vi_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_vi xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCVT_xs_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0bx M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VCVT_xs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(op<1> == 0 && !HaveFP16Ext()) || (op<1> == 0 && imm6 == 0b10xxxx) || (imm6 == 0b0xxxxx) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCVT_xs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0bx M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VCVT_xs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(op<1> == 0 && !HaveFP16Ext()) || (op<1> == 0 && imm6 == 0b10xxxx) || (imm6 == 0b0xxxxx) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VCVT_xv_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0bx ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0bxx sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sf == 0 || (sf == 1 && !HaveFP16Ext())',
  unpredict  => '(sf == 1 && cond != 14) || (((IIF(sx == 0, 16, 32)) - UInt(imm4:i)) < 0)',
};

CLASS VCVT_xv_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0bx ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0bxx sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sf == 0 || (sf == 1 && !HaveFP16Ext())',
  unpredict  => '(sf == 1 && InITBlock()) || (((IIF(sx == 0, 16, 32)) - UInt(imm4:i)) < 0)',
};

CLASS VCVTA_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTA_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VCVTA_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTA_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VCVTA_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTA_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VCVTA_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTA_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VCVTB_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0bx Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0bx T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVTB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0bx Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0bx T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVTB_bfs_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB_bfs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveAArch32BF16Ext()',
};

CLASS VCVTB_bfs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB_bfs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveAArch32BF16Ext()',
};

CLASS VCVTM_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTM_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VCVTM_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTM_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VCVTM_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTM_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VCVTM_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTM_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VCVTN_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTN_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VCVTN_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTN_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VCVTN_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTN_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VCVTN_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTN_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VCVTP_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTP_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VCVTP_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTP_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VCVTP_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTP_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VCVTP_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTP_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VCVTR_iv_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b10x Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VCVTR_iv_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b10x Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VCVTT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0bx Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0bx T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVTT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0bx Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0bx T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS VCVTT_bfs_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT_bfs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveAArch32BF16Ext()',
};

CLASS VCVTT_bfs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT_bfs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveAArch32BF16Ext()',
};

CLASS VDIV_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VDIV xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VDIV_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VDIV xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VDOT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VDOT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VDOT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VDOT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VDOT_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VDOT_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VDOT_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VDOT_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VDUP_r_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 ig1=0b1 B:u=0bx Q:u=0bx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx ig3=0b0 E:u=0bx ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VDUP_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VDUP_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 ig1=0b1 B:u=0bx Q:u=0bx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx ig3=0b0 E:u=0bx ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VDUP_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VDUP_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_dup page=VDUP_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(imm4 == 0bx000) || (Q == 1 && Vd<0> == 1)',
};

CLASS VDUP_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_dup_sc page=VDUP_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(imm4 == 0bx000) || (Q == 1 && Vd<0> == 1)',
};

CLASS VEOR_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEOR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VEOR xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VEOR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEOR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VEOR xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VEXT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (Q == 0 && imm4<3> == 1)',
};

CLASS VEXT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (Q == 0 && imm4<3> == 1)',
};

CLASS VFMA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VFMA_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMA xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VFMA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VFMA_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMA xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VFMA_bf_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMA_bf xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VFMA_bf_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMA_bf xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFMA_bfs_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMA_bfs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1)',
};

CLASS VFMA_bfs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMA_bfs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFMAL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMAL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
};

CLASS VFMAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2:u=0b1x Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMAL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFMAL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMAL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
};

CLASS VFMAL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMAL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFMS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VFMS_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMS xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VFMS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VFMS_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMS xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VFMSL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMSL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
};

CLASS VFMSL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMSL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFMSL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMSL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
};

CLASS VFMSL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMSL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (Q == 1 && Vd<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VFNMA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VFNMA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VFNMS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VFNMS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VHADD_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHADD xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VHADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHADD xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VHSUB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHSUB xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VHSUB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHSUB xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VINS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VINS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpextins page=VINS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16Ext()) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
};

CLASS VINS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VINS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_extins page=VINS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16Ext()) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'InITBlock()',
};

CLASS VJCVT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 convert-type=double-to-single instr-class=fpsimd isa=A32 mnemonic=VJCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VJCVT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFJCVTZSExt()',
  unpredict  => 'cond != 14',
};

CLASS VJCVT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 convert-type=double-to-single instr-class=fpsimd isa=T32 mnemonic=VJCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VJCVT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFJCVTZSExt()',
  unpredict  => 'InITBlock()',
};

CLASS VLD1_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1> != 0',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(index_align<2> != 0) || (index_align<1:0> != 0 && index_align<1:0> != 3)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1> != 0',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(index_align<2> != 0) || (index_align<1:0> != 0 && index_align<1:0> != 3)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VLD1_a_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD1_a xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || (size == 0 && a == 1)',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(IIF(T == 0, 1, 2)) > 32',
};

CLASS VLD1_a_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD1_a xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || (size == 0 && a == 1)',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(IIF(T == 0, 1, 2)) > 32',
};

CLASS VLD1_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(1) > 32',
};

CLASS VLD1_m_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(2) > 32',
};

CLASS VLD1_m_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(3) > 32',
};

CLASS VLD1_m_a4 => {
  name       => 'A4',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m xid=a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(4) > 32',
};

CLASS VLD1_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(1) > 32',
};

CLASS VLD1_m_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(2) > 32',
};

CLASS VLD1_m_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(3) > 32',
};

CLASS VLD1_m_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m xid=t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(4) > 32',
};

CLASS VLD2_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (1)) > 31',
};

CLASS VLD2_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD2_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1> != 0',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD2_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (1)) > 31',
};

CLASS VLD2_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD2_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1> != 0',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD2_a_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD2_a xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD2_a_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD2_a xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD2_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(itype == 9, 2, 1)))+(1) > 32',
};

CLASS VLD2_m_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (2))+(2) > 32',
};

CLASS VLD2_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(itype == 9, 2, 1)))+(1) > 32',
};

CLASS VLD2_m_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (2))+(2) > 32',
};

CLASS VLD3_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (1)) + (1)) > 31',
};

CLASS VLD3_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD3_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1:0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD3_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (1)) + (1)) > 31',
};

CLASS VLD3_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD3_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1:0> != 0',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD3_a_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD3_a xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || a == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD3_a_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD3_a xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || a == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD3_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD3_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || align<1> == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (CASE(itype, 4, 1, 5, 2))) + (CASE(itype, 4, 1, 5, 2))) > 31',
};

CLASS VLD3_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD3_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || align<1> == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (CASE(itype, 4, 1, 5, 2))) + (CASE(itype, 4, 1, 5, 2))) > 31',
};

CLASS VLD4_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (1)) + (1)) + (1)) > 31',
};

CLASS VLD4_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD4_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1:0> == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD4_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (1)) + (1)) + (1)) > 31',
};

CLASS VLD4_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VLD4_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'index_align<1:0> == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VLD4_a_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD4_a xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 && a == 0',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD4_a_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD4_a xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 && a == 0',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) + (IIF(T == 0, 1, 2))) > 31',
};

CLASS VLD4_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD4_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) > 31',
};

CLASS VLD4_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD4_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) > 31',
};

CLASS VLDM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VLDM_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VLDM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VLDM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VLDR_i_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VLDR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VLDR_l_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=literal armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_l xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VLDR_l_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=literal armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_l xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VMAX_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMAX_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VMAX_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0bx M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VMAX_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0bx M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VMAXNM_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAXNM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMAXNM_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMAXNM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VMAXNM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAXNM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VMAXNM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMAXNM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VMIN_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMIN_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VMIN_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0bx M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VMIN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0bx M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VMINNM_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMINNM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMINNM_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMINNM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VMINNM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMINNM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VMINNM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMINNM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VMLA_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMLA_f_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLA_f xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VMLA_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VMLA_f_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLA_f xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VMLA_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 op:u=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMLA_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 op:u=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMLA_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLA_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VMLA_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLA_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VMLAL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMLAL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMLAL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMLAL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMLAL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLAL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMLAL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLAL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMLS_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMLS_f_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLS_f xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VMLS_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VMLS_f_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLS_f xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VMLS_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 op:u=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMLS_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 op:u=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMLS_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLS_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VMLS_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLS_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VMLSL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMLSL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMLSL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMLSL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMLSL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLSL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMLSL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLSL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMMLA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op=0b0 sz=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VMMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VMMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op=0b0 sz=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VMMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32BF16Ext()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VMOV_d_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_d xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rt2) == 15) || (((op == 1)) && UInt(Rt) == UInt(Rt2))',
};

CLASS VMOV_d_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_d xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rt2) == 15) || (((op == 1)) && UInt(Rt) == UInt(Rt2))',
};

CLASS VMOV_h_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp16 page=VMOV_h xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
  unpredict  => '(cond != 14) || (UInt(Rt) == 15)',
};

CLASS VMOV_h_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=p_cpaf page=VMOV_h xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
  unpredict  => '(InITBlock()) || (UInt(Rt) == 15)',
};

CLASS VMOV_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpimm page=VMOV_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VMOV_i_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_a4 => {
  name       => 'A4',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i xid=a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS a5 => {
  name       => 'A5',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0bx op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A5 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0bxx ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_movi page=VMOV_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VMOV_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0bx op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i xid=t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_i_t5 => {
  name       => 'T5',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0bx op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T5 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i xid=t5',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMOV_r_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b1x o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VMOV_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'FPSCR.Len != 0 || FPSCR.Stride != 0',
};

CLASS VMOV_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b1x o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VMOV_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'FPSCR.Len != 0 || FPSCR.Stride != 0',
};

CLASS VMOV_rs_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 ig1=0b0 opc1:u=0bxx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx opc2:u=0bxx ig3=0b1 ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VMOV_rs xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS VMOV_rs_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 ig1=0b0 opc1:u=0bxx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx opc2:u=0bxx ig3=0b1 ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VMOV_rs xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS VMOV_s_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp32 page=VMOV_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS VMOV_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110000 op:u=0bx Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_mov32 page=VMOV_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => 'UInt(Rt) == 15',
};

CLASS VMOV_sr_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 U:u=0bx opc1:u=0bxx L=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VMOV_sr xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS VMOV_sr_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 U:u=0bx opc1:u=0bxx L=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VMOV_sr xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS VMOV_ss_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_ss xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Vm:M) == 31) || (((op == 1)) && UInt(Rt) == UInt(Rt2))',
};

CLASS VMOV_ss_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0bx Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_ss xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(UInt(Rt) == 15 || UInt(Rt2) == 15 || UInt(Vm:M) == 31) || (((op == 1)) && UInt(Rt) == UInt(Rt2))',
};

CLASS VMOVL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm3H:u=0bxxx imm3L=0b000 Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm3H!=0b000',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VMOVL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMOVL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm3H:u=0bxxx imm3L=0b000 Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm3H!=0b000',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VMOVL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VMOVX_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVX',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpextins page=VMOVX xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16Ext()) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
};

CLASS VMOVX_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_extins page=VMOVX xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16Ext()) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'InITBlock()',
};

CLASS VMRS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110111 L=0b1 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpsr page=VMRS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|CP14DTTRAP|CP14RRTTRAP|CP14RTTRAP|CP15RRTTRAP|CP15RTTRAP|FPIDTRAP|UNCATEGORIZED',
  unpredict  => '(!(reg IN {0b000x, 5, 0b011x, 8})) || (UInt(Rt) == 15 && reg != 1)',
};

CLASS VMRS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110111 L=0b1 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_msr page=VMRS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|CP14DTTRAP|CP14RRTTRAP|CP14RTTRAP|CP15RRTTRAP|CP15RTTRAP|FPIDTRAP|UNCATEGORIZED',
  unpredict  => '(!(reg IN {0b000x, 5, 0b011x, 8})) || (UInt(Rt) == 15 && reg != 1)',
};

CLASS VMSR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110111 L=0b0 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpsr page=VMSR xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(reg != 0b000x && reg != 8) || (UInt(Rt) == 15)',
};

CLASS VMSR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110111 L=0b0 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_msr page=VMSR xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => '(reg != 0b000x && reg != 8) || (UInt(Rt) == 15)',
};

CLASS VMUL_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VMUL_f_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMUL_f xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VMUL_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VMUL_f_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMUL_f xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VMUL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 op:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (op == 1 && size != 0)) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMUL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 op:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (op == 1 && size != 0)) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VMUL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMUL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VMUL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMUL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (F == 1 && size == 1 && !HaveFP16Ext())) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VMULL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bx ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMULL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMULL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMULL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bx ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMULL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VMULL_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMULL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMULL_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMULL_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMULL_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VMVN_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_i_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_i_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_i_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VMVN_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VMVN_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VMVN_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VMVN_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VNEG_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VNEG xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VNEG_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VNEG xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VNEG_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VNEG xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (F == 1 && ((size == 1 && !HaveFP16Ext()) || size == 0)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
  unpredict  => 'F == 1 && size == 1 && InITBlock()',
};

CLASS VNEG_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VNEG xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VNMLA_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VNMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VNMLS_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VNMLS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VNMUL_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMUL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VNMUL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMUL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VORN_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORN_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VORN_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORN_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VORR_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORR_i_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORR_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORR_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORR_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VORR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORR_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VPADAL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADAL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VPADAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADAL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VPADD_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPADD_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VPADD_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPADD_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VPADD_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPADD_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Q == 1',
};

CLASS VPADD_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPADD_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Q == 1',
};

CLASS VPADDL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADDL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADDL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VPADDL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADDL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADDL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VPMAX_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMAX_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 1 && !HaveFP16Ext()',
};

CLASS VPMAX_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMAX_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 1 && !HaveFP16Ext()',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VPMAX_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMAX_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS VPMAX_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMAX_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS VPMIN_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMIN_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 1 && !HaveFP16Ext()',
};

CLASS VPMIN_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMIN_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 1 && !HaveFP16Ext()',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VPMIN_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMIN_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS VPMIN_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMIN_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS VQABS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQABS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQABS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQABS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQADD_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQADD xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VQADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQADD xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VQDMLAL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMLAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMLAL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLAL_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMLAL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMLAL xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLAL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMLAL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLAL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMLAL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMLAL xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLSL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMLSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMLSL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLSL_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMLSL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMLSL xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLSL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMLSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMLSL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMLSL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMLSL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMLSL xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMULH_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMULH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQDMULH xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQDMULH_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMULH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMULH xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQDMULH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMULH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQDMULH xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQDMULH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMULH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMULH xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQDMULL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMULL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMULL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMULL_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMULL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMULL xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMULL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMULL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQDMULL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMULL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMULL xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || Vd<0> == 1',
};

CLASS VQMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bxx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bxx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQNEG_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQNEG xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQNEG_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQNEG xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQRDMLAH_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLAH xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQRDMLAH_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLAH xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQRDMLAH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLAH xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
  unpredict  => 'InITBlock()',
};

CLASS VQRDMLAH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLAH xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VQRDMLSH_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLSH xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQRDMLSH_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLSH xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQRDMLSH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLSH xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
  unpredict  => 'InITBlock()',
};

CLASS VQRDMLSH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLSH xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VQRDMULH_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMULH xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQRDMULH_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMULH xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQRDMULH_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMULH xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 0 || size == 3)',
};

CLASS VQRDMULH_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMULH xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 0) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VQRSHL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRSHL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRSHL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VQRSHL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRSHL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRSHL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VQRSHRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0bx L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQRSHRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VQRSHRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0bx L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQRSHRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VQSHL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0bx L:u=0bx Q:u=0bx M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(U == 0 && op == 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQSHL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0bx L:u=0bx Q:u=0bx M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(U == 0 && op == 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VQSHL_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSHL_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VQSHL_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSHL_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VQSHRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0bx L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VQSHRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0bx L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VQSUB_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSUB xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VQSUB_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSUB xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VRADDHN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRADDHN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VRADDHN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VRADDHN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRADDHN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VRADDHN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VRECPE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0bx M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRECPE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRECPE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0bx M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRECPE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VRECPS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRECPS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VRECPS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRECPS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VREV16_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV16',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV16 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VREV16_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV16',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV16 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VREV32_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV32 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VREV32_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV32 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VREV64_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV64',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV64 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VREV64_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV64',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV64 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(UInt(op)+UInt(size) >= 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VRHADD_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRHADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRHADD xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VRHADD_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRHADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRHADD xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VRINTA_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTA_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTA_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTA_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTA_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTA_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VRINTA_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTA_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VRINTM_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTM_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTM_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTM_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTM_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTM_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VRINTM_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTM_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VRINTN_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTN_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTN_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTN_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTN_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTN_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VRINTN_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTN_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VRINTP_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTP_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTP_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0bx M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTP_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTP_vfp_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTP_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VRINTP_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTP_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VRINTR_vfp_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTR_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VRINTR_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTR_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VRINTX_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTX_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTX_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTX_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTX_vfp_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTX_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VRINTX_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTX_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VRINTZ_asimd_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTZ_asimd xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRINTZ_asimd_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTZ_asimd xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'InITBlock()',
};

CLASS VRINTZ_vfp_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTZ_vfp xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VRINTZ_vfp_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0bxx op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTZ_vfp xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VRSHL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VRSHL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VRSHR_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSHR xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VRSHR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSHR xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VRSHRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHRN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSHRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VRSHRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHRN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSHRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VRSQRTE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0bx M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRSQRTE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
};

CLASS VRSQRTE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0bx M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRSQRTE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vm<0> == 1)) || ((size == 1 && !HaveFP16Ext()) || size IN {0, 3})',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VRSQRTS_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSQRTS xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VRSQRTS_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSQRTS xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VRSRA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSRA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSRA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VRSRA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSRA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSRA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VRSUBHN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSUBHN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VRSUBHN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VRSUBHN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSUBHN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VRSUBHN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VSDOT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VSDOT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VSDOT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VSDOT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VSDOT_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSDOT_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VSDOT_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSDOT_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VSEL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
};

CLASS VSEL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'InITBlock()',
};

CLASS VSHL_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHL_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSHL_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHL_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSHL_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHL_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VSHL_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHL_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1 || Vn<0> == 1)',
};

CLASS VSHLL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHLL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHLL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VSHLL_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VSHLL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSHLL xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vd<0> == 1',
};

CLASS VSHLL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHLL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHLL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1',
};

CLASS VSHLL_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VSHLL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSHLL xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vd<0> == 1',
};

CLASS VSHR_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHR xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSHR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHR xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSHRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHRN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VSHRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHRN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vm<0> == 1',
};

CLASS VSLI_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSLI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSLI xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSLI_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSLI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSLI xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSMMLA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VSMMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VSMMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VSMMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VSQRT_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VSQRT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VSQRT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0bxx o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VSQRT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size == 0 || (size == 1 && !HaveFP16Ext())) || (FPSCR.Len != 0 || FPSCR.Stride != 0)',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VSRA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSRA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSRI_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRI',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRI xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VSRI_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0bx M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRI',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRI xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vm<0> == 1)',
};

CLASS VST1_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1> != 0)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<2> != 0) || (index_align<1:0> != 0 && index_align<1:0> != 3)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1> != 0)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<2> != 0) || (index_align<1:0> != 0 && index_align<1:0> != 3)',
  unpredict  => 'UInt(Rn) == 15',
};

CLASS VST1_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(1) > 32',
};

CLASS VST1_m_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(2) > 32',
};

CLASS VST1_m_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(3) > 32',
};

CLASS VST1_m_a4 => {
  name       => 'A4',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m xid=a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(4) > 32',
};

CLASS VST1_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(1) > 32',
};

CLASS VST1_m_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(2) > 32',
};

CLASS VST1_m_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(3) > 32',
};

CLASS VST1_m_t4 => {
  name       => 'T4',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m xid=t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  unpredict  => 'UInt(Rn) == 15 || UInt(D:Vd)+(4) > 32',
};

CLASS VST2_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (1)) > 31',
};

CLASS VST2_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST2_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1> != 0)',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST2_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (1)) > 31',
};

CLASS VST2_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST2_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1> != 0)',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST2_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(itype == 9, 2, 1)))+(1) > 32',
};

CLASS VST2_m_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (2))+(2) > 32',
};

CLASS VST2_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (IIF(itype == 9, 2, 1)))+(1) > 32',
};

CLASS VST2_m_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (UInt(D:Vd) + (2))+(2) > 32',
};

CLASS VST3_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (1)) + (1)) > 31',
};

CLASS VST3_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST3_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1:0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST3_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (1)) + (1)) > 31',
};

CLASS VST3_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST3_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1:0> != 0)',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST3_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST3_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || align<1> == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (CASE(itype, 4, 1, 5, 2))) + (CASE(itype, 4, 1, 5, 2))) > 31',
};

CLASS VST3_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST3_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3 || align<1> == 1',
  unpredict  => 'UInt(Rn) == 15 || ((UInt(D:Vd) + (CASE(itype, 4, 1, 5, 2))) + (CASE(itype, 4, 1, 5, 2))) > 31',
};

CLASS VST4_1_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (1)) + (1)) + (1)) > 31',
};

CLASS VST4_1_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST4_1_a3 => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 xid=a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1:0> == 3)',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST4_1_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (1)) + (1)) + (1)) > 31',
};

CLASS VST4_1_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) + (IIF(index_align<1> == 0, 1, 2))) > 31',
};

CLASS VST4_1_t3 => {
  name       => 'T3',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 xid=t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => '(size == 3) || (index_align<1:0> == 3)',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) + (IIF(index_align<2> == 0, 1, 2))) > 31',
};

CLASS VST4_m_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST4_m xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) > 31',
};

CLASS VST4_m_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST4_m xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 3',
  unpredict  => 'UInt(Rn) == 15 || (((UInt(D:Vd) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) + (CASE(itype, 0, 1, 1, 2))) > 31',
};

CLASS VSTM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VSTM_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VSTM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VSTM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110110 P:u=0bx U:u=0bx D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VSTR_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTR xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VSTR_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx imm8:u=0bxxxxxxxx',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTR xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'size == 0 || (size == 1 && !HaveFP16Ext())',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VSUB_f_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_f xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VSUB_f_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VSUB_f xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && cond != 14',
};

CLASS VSUB_f_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_f xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VSUB_f_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0bxx N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VSUB_f xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(FPSCR.Len != 0 || FPSCR.Stride != 0) || (size == 0 || (size == 1 && !HaveFP16Ext()))',
  unpredict  => 'size == 1 && InITBlock()',
};

CLASS VSUB_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VSUB_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VSUBHN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBHN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBHN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VSUBHN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBHN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBHN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vn<0> == 1 || Vm<0> == 1',
};

CLASS VSUBL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VSUBL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VSUBW_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBW',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBW xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VSUBW_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBW',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBW xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Vd<0> == 1 || (op == 1 && Vn<0> == 1)',
};

CLASS VSUDOT_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSUDOT_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VSUDOT_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSUDOT_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VSWP_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSWP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSWP xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VSWP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSWP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSWP xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size != 0) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VTBL_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_tbl page=VTBL xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => 'UInt(N:Vn)+(UInt(len)+1) > 32',
};

CLASS VTBL_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_tbl page=VTBL xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  unpredict  => 'UInt(N:Vn)+(UInt(len)+1) > 32',
};

CLASS VTRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VTRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VTRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VTRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VTST_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTST',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VTST xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VTST_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTST',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VTST xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VUDOT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUDOT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VUDOT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUDOT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VUDOT_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUDOT_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VUDOT_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUDOT_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VUMMLA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUMMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VUMMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUMMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VUSDOT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUSDOT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
};

CLASS VUSDOT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUSDOT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VUSDOT_s_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUSDOT_s xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
};

CLASS VUSDOT_s_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0bx M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUSDOT_s xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Q == 1 && (Vd<0> == 1 || Vn<0> == 1))',
  unpredict  => 'InITBlock()',
};

CLASS VUSMMLA_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUSMMLA xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VUSMMLA_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUSMMLA xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveAArch32Int8MatMulExt()) || (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
  unpredict  => 'InITBlock()',
};

CLASS VUZP_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUZP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VUZP xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (Q == 0 && size == 2)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VUZP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUZP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VUZP xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (Q == 0 && size == 2)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VZIP_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VZIP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VZIP xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (Q == 0 && size == 2)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VZIP_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0bx M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VZIP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VZIP xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3 || (Q == 0 && size == 2)) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VACLE_VACGE_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLE_VACGE xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VACLE_VACGE_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLE_VACGE xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VACLT_VACGT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLT_VACGT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VACLT_VACGT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLT_VACGT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VAND_VBIC_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VAND_VBIC_i_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VAND_VBIC_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VAND_VBIC_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b1 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VCLE_VCGE_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCLE_VCGE_r_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VCLE_VCGE_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCLE_VCGE_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VCLT_VCGT_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCLT_VCGT_r_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
};

CLASS VCLT_VCGT_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (size == 3)',
};

CLASS VCLT_VCGT_r_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (sz == 1 && !HaveFP16Ext())',
  unpredict  => 'sz == 1 && InITBlock()',
};

CLASS VEXT_VEXT_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT_VEXT xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (Q == 0 && imm4<3> == 1)',
};

CLASS VEXT_VEXT_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0bx M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT_VEXT xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)) || (Q == 0 && imm4<3> == 1)',
};

CLASS VMOV_VORR_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMOV_VORR_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VMOV_VORR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMOV_VORR_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VORN_VORR_i_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORN_VORR_i_a2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORN_VORR_i_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VORN_VORR_i_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0bx op=0b0 ig4=0b1 imm4:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && Vd<0> == 1',
};

CLASS VPOP_VLDM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPOP_VLDM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VPOP_VLDM_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPOP_VLDM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VPOP_VLDM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPOP_VLDM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VPOP_VLDM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPOP_VLDM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VPUSH_VSTM_a1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPUSH_VSTM xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VPUSH_VSTM_a2 => {
  name       => 'A2',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPUSH_VSTM xid=a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VPUSH_VSTM_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPUSH_VSTM xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || ((UInt(imm8) DIV 2) == 0 || (UInt(imm8) DIV 2) > 16 || (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 32) || (imm8<0> == 1 && (UInt(D:Vd)+(UInt(imm8) DIV 2)) > 16)',
};

CLASS VPUSH_VSTM_t2 => {
  name       => 'T2',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPUSH_VSTM xid=t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  undefined  => 'P == U && W == 1',
  unpredict  => '(UInt(Rn) == 15 && (((W == 1)) || CurrentInstrSet() != InstrSet_A32)) || (UInt(imm8) == 0 || (UInt(Vd:D)+UInt(imm8)) > 32)',
};

CLASS VQRSHRN_VQMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQRSHRN_VQMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQRSHRN_VQMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQRSHRN_VQMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQRSHRUN_VQMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQRSHRUN_VQMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQRSHRUN_VQMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQRSHRUN_VQMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQSHRN_VQMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQSHRN_VQMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQSHRN_VQMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQSHRN_VQMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQSHRUN_VQMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQSHRUN_VQMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VQSHRUN_VQMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQSHRUN_VQMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || Vm<0> == 1',
};

CLASS VRSHR_VORR_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHR_VORR_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VRSHR_VORR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHR_VORR_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VRSHRN_VMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRSHRN_VMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VRSHRN_VMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRSHRN_VMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VSHR_VORR_r_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHR_VORR_r xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VSHR_VORR_r_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHR_VORR_r xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 1 && (Vd<0> == 1 || Vn<0> == 1 || Vm<0> == 1)',
};

CLASS VSHRN_VMOVN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSHRN_VMOVN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VSHRN_VMOVN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSHRN_VMOVN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Vm<0> == 1)',
};

CLASS VUZP_VTRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VUZP_VTRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VUZP_VTRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VUZP_VTRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VZIP_VTRN_a1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN',
  metadata   => 'alias=1 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VZIP_VTRN xid=a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};

CLASS VZIP_VTRN_t1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN',
  metadata   => 'alias=1 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VZIP_VTRN xid=t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 3) || (Q == 1 && (Vd<0> == 1 || Vm<0> == 1))',
};


1;

