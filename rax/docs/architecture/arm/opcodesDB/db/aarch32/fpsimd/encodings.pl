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

ENCODING AESD_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESD cclass=AESD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESD_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESD cclass=AESD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESE_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESE cclass=AESE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESE_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESE cclass=AESE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESIMC_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESIMC cclass=AESIMC_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESIMC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESIMC cclass=AESIMC_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESMC_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=AESMC',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=AESMC cclass=AESMC_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING AESMC_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=AESMC',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=AESMC cclass=AESMC_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING FLDMDBX_A1 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=FLDMDBX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FLDMX cclass=FLDMX_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FLDMIAX_A1 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=FLDMIAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FLDMX cclass=FLDMX_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FLDMDBX_T1 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=FLDMDBX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FLDMX cclass=FLDMX_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FLDMIAX_T1 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=FLDMIAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FLDMX cclass=FLDMX_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FSTMDBX_A1 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=FSTMDBX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FSTMX cclass=FSTMX_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FSTMIAX_A1 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=FSTMIAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=FSTMX cclass=FSTMX_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FSTMDBX_T1 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=FSTMDBX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FSTMX cclass=FSTMX_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING FSTMIAX_T1 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx1',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=FSTMIAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=FSTMX cclass=FSTMX_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING SHA1C_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1C cclass=SHA1C_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1C_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1C cclass=SHA1C_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1H_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 opc2=0b0101 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA1H cclass=SHA1H_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1H_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 opc2=0b0101 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA1H cclass=SHA1H_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1M_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1M cclass=SHA1M_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1M_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1M cclass=SHA1M_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1P_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1P cclass=SHA1P_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1P_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1P cclass=SHA1P_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU0_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA1SU0 cclass=SHA1SU0_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU0_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA1SU0 cclass=SHA1SU0_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU1_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA1SU1 cclass=SHA1SU1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA1SU1_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA1SU1 cclass=SHA1SU1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256H cclass=SHA256H_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256H cclass=SHA256H_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H2_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256H2 cclass=SHA256H2_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256H2_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256H2 cclass=SHA256H2_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU0_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=SHA256SU0 cclass=SHA256SU0_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU0_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0111 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=SHA256SU0 cclass=SHA256SU0_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU1_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=SHA256SU1 cclass=SHA256SU1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING SHA256SU1_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=SHA256SU1 cclass=SHA256SU1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CRYPTO',
};

ENCODING VST4_1_A3_nowb => {
  name       => 'A3',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=ST4_1 cclass=ST4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VABA_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABA cclass=VABA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABA_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABA cclass=VABA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABA_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABA cclass=VABA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABA_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABA cclass=VABA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABAL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VABAL cclass=VABAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABAL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VABAL cclass=VABAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_f cclass=VABD_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_f cclass=VABD_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_f cclass=VABD_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_f cclass=VABD_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_i cclass=VABD_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABD simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VABD_i cclass=VABD_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_i cclass=VABD_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABD_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABD simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VABD_i cclass=VABD_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABDL_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VABDL_i cclass=VABDL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABDL_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0111 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VABDL_i cclass=VABDL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VABS cclass=VABS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VABS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VABS cclass=VABS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VABS cclass=VABS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VABS cclass=VABS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VABS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VABS cclass=VABS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VABS cclass=VABS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b110 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VABS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VABS cclass=VABS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VABS cclass=VABS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VABS cclass=VABS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VABS_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VABS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VABS cclass=VABS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VACGE_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGE cclass=VACGE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGE_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGE cclass=VACGE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGE_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGE cclass=VACGE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGE_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGE cclass=VACGE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGT cclass=VACGT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACGT cclass=VACGT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGT cclass=VACGT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VACGT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACGT cclass=VACGT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VADD_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_f cclass=VADD_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_f cclass=VADD_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_f_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VADD_f cclass=VADD_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_f_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VADD_f cclass=VADD_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_f_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VADD_f cclass=VADD_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_f cclass=VADD_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_f cclass=VADD_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_f_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VADD_f cclass=VADD_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_f_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VADD_f cclass=VADD_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_f_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VADD_f cclass=VADD_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VADD_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_i cclass=VADD_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VADD_i cclass=VADD_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_i cclass=VADD_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADD_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VADD_i cclass=VADD_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDHN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDHN cclass=VADDHN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDHN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDHN cclass=VADDHN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDL cclass=VADDL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDL cclass=VADDL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDW_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VADDW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VADDW cclass=VADDW_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VADDW_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b000 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VADDW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VADDW cclass=VADDW_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VAND_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VAND simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VAND_r cclass=VAND_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VAND_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VAND simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VAND_r cclass=VAND_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VAND_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VAND simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VAND_r cclass=VAND_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VAND_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VAND simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VAND_r cclass=VAND_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i cclass=VBIC_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i cclass=VBIC_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i cclass=VBIC_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VBIC_i cclass=VBIC_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i cclass=VBIC_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i cclass=VBIC_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i cclass=VBIC_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_i_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VBIC_i cclass=VBIC_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIC_r cclass=VBIC_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIC_r cclass=VBIC_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIC_r cclass=VBIC_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIC_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIC_r cclass=VBIC_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VBIF_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIF simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIF cclass=VBIF_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIF_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIF simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIF cclass=VBIF_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIF_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIF simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIF cclass=VBIF_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIF_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIF simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIF cclass=VBIF_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIT cclass=VBIT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBIT cclass=VBIT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIT cclass=VBIT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBIT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBIT cclass=VBIT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|INSERTION|BITWISE|CONDITIONALLY',
};

ENCODING VBSL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBSL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBSL cclass=VBSL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SELECTION',
};

ENCODING VBSL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBSL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VBSL cclass=VBSL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SELECTION',
};

ENCODING VBSL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBSL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBSL cclass=VBSL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SELECTION',
};

ENCODING VBSL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBSL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VBSL cclass=VBSL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SELECTION',
};

ENCODING VCADD_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCADD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCADD cclass=VCADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCADD_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCADD cclass=VCADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCADD_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCADD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCADD cclass=VCADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCADD_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bx ig1=0b1 D:u=0bx ig2=0b0 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig3=0b1 op3=0b0 ig4=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCADD simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCADD cclass=VCADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCEQ_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCEQ_i cclass=VCEQ_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCEQ_i cclass=VCEQ_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCEQ_i cclass=VCEQ_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b010 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCEQ_i cclass=VCEQ_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r cclass=VCEQ_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r cclass=VCEQ_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r cclass=VCEQ_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCEQ_r cclass=VCEQ_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r cclass=VCEQ_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r cclass=VCEQ_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r cclass=VCEQ_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCEQ_r_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCEQ simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCEQ_r cclass=VCEQ_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGE_i cclass=VCGE_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGE_i cclass=VCGE_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGE_i cclass=VCGE_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b001 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGE_i cclass=VCGE_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r cclass=VCGE_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r cclass=VCGE_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r cclass=VCGE_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGE_r cclass=VCGE_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r cclass=VCGE_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r cclass=VCGE_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r cclass=VCGE_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGE_r_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGE_r cclass=VCGE_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGT_i cclass=VCGT_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCGT_i cclass=VCGT_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGT_i cclass=VCGT_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b000 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCGT_i cclass=VCGT_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r cclass=VCGT_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r cclass=VCGT_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r cclass=VCGT_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCGT_r cclass=VCGT_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r cclass=VCGT_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r cclass=VCGT_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r cclass=VCGT_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCGT_r_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCGT_r cclass=VCGT_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLE_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLE_i cclass=VCLE_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLE_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLE_i cclass=VCLE_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLE_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLE_i cclass=VCLE_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLE_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b011 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLE_i cclass=VCLE_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLS cclass=VCLS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLS cclass=VCLS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLS cclass=VCLS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLS cclass=VCLS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLT_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLT_i cclass=VCLT_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLT_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLT_i cclass=VCLT_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLT_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLT_i cclass=VCLT_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLT_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b100 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLT_i cclass=VCLT_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCLZ_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLZ simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLZ cclass=VCLZ_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLZ_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCLZ simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCLZ cclass=VCLZ_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLZ_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLZ simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLZ cclass=VCLZ_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCLZ_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCLZ simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCLZ cclass=VCLZ_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCMLA_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCMLA cclass=VCMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VCMLA cclass=VCMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCMLA cclass=VCMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 rot:u=0bxx D:u=0bx ig1=0b1 S:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VCMLA cclass=VCMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_A1_DH => {
  name       => '64-bit SIMD vector of half-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b0 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && Q == 0b0',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMLA simd-fp-type=double-halfprec simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VCMLA_s cclass=VCMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_A1_DS => {
  name       => '64-bit SIMD vector of single-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b1 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && Q == 0b0',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMLA simd-fp-type=double-singleprec simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VCMLA_s cclass=VCMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_A1_QH => {
  name       => '128-bit SIMD vector of half-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b0 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && Q == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMLA simd-fp-type=quad-halfprec simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VCMLA_s cclass=VCMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_A1_QS => {
  name       => '128-bit SIMD vector of single-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b1 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && Q == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMLA simd-fp-type=quad-singleprec simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VCMLA_s cclass=VCMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_T1_DH => {
  name       => '64-bit SIMD vector of half-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b0 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && Q == 0b0',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMLA simd-fp-type=double-halfprec simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VCMLA_s cclass=VCMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_T1_DS => {
  name       => '64-bit SIMD vector of single-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b1 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && Q == 0b0',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMLA simd-fp-type=double-singleprec simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VCMLA_s cclass=VCMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_T1_QH => {
  name       => '128-bit SIMD vector of half-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b0 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b0 && Q == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMLA simd-fp-type=quad-halfprec simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VCMLA_s cclass=VCMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMLA_s_T1_QS => {
  name       => '128-bit SIMD vector of single-precision floating-point',
  diagram    => 'ig0=0b11111110 S:u=0b1 D:u=0bx rot:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'S == 0b1 && Q == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMLA simd-fp-type=quad-singleprec simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VCMLA_s cclass=VCMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPLEX',
};

ENCODING VCMP_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMP cclass=VCMP_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMP_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b0 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCMP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMP cclass=VCMP_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCMPE cclass=VCMPE_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCMPE_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 E:u=0b1 ig3=0b1 M=0bZ ig4=0b0 Vm=0bZZZZ',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCMPE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCMPE cclass=VCMPE_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VCNT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCNT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCNT cclass=VCNT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCNT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCNT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCNT cclass=VCNT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCNT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCNT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCNT cclass=VCNT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCNT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1010 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCNT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCNT cclass=VCNT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITFIELD',
};

ENCODING VCVT_bfs_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size=0b01 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1100 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_bfs cclass=VCVT_bfs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_bfs_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size=0b01 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1100 Q=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_bfs cclass=VCVT_bfs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_ds_A1 => {
  name       => 'Single-precision to double-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 convert-type=single-to-double instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_ds cclass=VCVT_ds_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVT_sd_A1 => {
  name       => 'Double-precision to single-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 convert-type=double-to-single instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_ds cclass=VCVT_ds_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVT_ds_T1 => {
  name       => 'Single-precision to double-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 convert-type=single-to-double instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_ds cclass=VCVT_ds_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVT_sd_T1 => {
  name       => 'Double-precision to single-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 convert-type=double-to-single instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_ds cclass=VCVT_ds_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVT_sh_A1 => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0b1 ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 convert-type=half-to-single instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_hs cclass=VCVT_hs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_hs_A1 => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0b0 ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 convert-type=single-to-half instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_hs cclass=VCVT_hs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_sh_T1 => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0b1 ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 convert-type=half-to-single instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_hs cclass=VCVT_hs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_hs_T1 => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0b0 ig4=0b0 Q=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 convert-type=single-to-half instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_hs cclass=VCVT_hs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_is_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_is cclass=VCVT_is_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_is_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVT_is cclass=VCVT_is_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_is_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_is cclass=VCVT_is_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_is_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b11 op:u=0bxx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVT_is cclass=VCVT_is_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_uiv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_uiv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_uiv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_iv cclass=VCVT_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_uiv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_uiv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_uiv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_siv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_iv cclass=VCVT_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_vi cclass=VCVT_vi_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_vi cclass=VCVT_vi_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_vi cclass=VCVT_vi_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_vi cclass=VCVT_vi_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_vi cclass=VCVT_vi_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_vi_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_vi cclass=VCVT_vi_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xs_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VCVT_xs cclass=VCVT_xs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_xs_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VCVT_xs cclass=VCVT_xs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_xs_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VCVT_xs cclass=VCVT_xs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_xs_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bxx L=0b0 Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VCVT_xs cclass=VCVT_xs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVT_toxv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b01 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sf == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b01 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sf == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_toxv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b10 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sf == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b10 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sf == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_toxv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b11 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sf == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b11 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sf == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVT_xv cclass=VCVT_xv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_toxv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b01 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sf == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b01 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sf == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_toxv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b10 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sf == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b10 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sf == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_toxv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b0 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b11 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sf == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVT_xv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 op:u=0b1 ig2=0b1 U:u=0bx Vd:u=0bxxxx ig3=0b10 sf:u=0b11 sx:u=0bx ig4=0b1 i:u=0bx ig5=0b0 imm4:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sf == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVT_xv cclass=VCVT_xv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTA_asimd cclass=VCVTA_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTA_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTA_asimd cclass=VCVTA_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTA_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTA_asimd cclass=VCVTA_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTA_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b00 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTA_asimd cclass=VCVTA_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTA_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTA_vfp cclass=VCVTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTA_vfp cclass=VCVTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTA_vfp cclass=VCVTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTA_vfp cclass=VCVTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTA_vfp cclass=VCVTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTA_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTA_vfp cclass=VCVTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTB_A1_SH => {
  name       => 'Half-precision to single-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sz == 0b0',
  docvars    => 'armarmheading=A1 convert-type=half-to-single instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB cclass=VCVTB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_A1_DH => {
  name       => 'Half-precision to double-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sz == 0b1',
  docvars    => 'armarmheading=A1 convert-type=half-to-double instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB cclass=VCVTB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_A1_HS => {
  name       => 'Single-precision to half-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sz == 0b0',
  docvars    => 'armarmheading=A1 convert-type=single-to-half instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB cclass=VCVTB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_A1_HD => {
  name       => 'Double-precision to half-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sz == 0b1',
  docvars    => 'armarmheading=A1 convert-type=double-to-half instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB cclass=VCVTB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_T1_SH => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sz == 0b0',
  docvars    => 'armarmheading=T1 convert-type=half-to-single instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB cclass=VCVTB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_T1_DH => {
  name       => 'Half-precision to double-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sz == 0b1',
  docvars    => 'armarmheading=T1 convert-type=half-to-double instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB cclass=VCVTB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_T1_HS => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sz == 0b0',
  docvars    => 'armarmheading=T1 convert-type=single-to-half instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB cclass=VCVTB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_T1_HD => {
  name       => 'Double-precision to half-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b0 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sz == 0b1',
  docvars    => 'armarmheading=T1 convert-type=double-to-half instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB cclass=VCVTB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_A1_bfs => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTB_bfs cclass=VCVTB_bfs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTB_T1_bfs => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTB_bfs cclass=VCVTB_bfs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTM_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTM simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTM_asimd cclass=VCVTM_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTM_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTM simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTM_asimd cclass=VCVTM_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTM_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTM simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTM_asimd cclass=VCVTM_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTM_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b11 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTM simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTM_asimd cclass=VCVTM_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTM_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTM_vfp cclass=VCVTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTM_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTM_vfp cclass=VCVTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTM_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTM_vfp cclass=VCVTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTM_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTM_vfp cclass=VCVTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTM_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTM_vfp cclass=VCVTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTM_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTM_vfp cclass=VCVTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTN_asimd cclass=VCVTN_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTN_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTN_asimd cclass=VCVTN_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTN_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTN_asimd cclass=VCVTN_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTN_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b01 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTN_asimd cclass=VCVTN_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTN_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTN_vfp cclass=VCVTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTN_vfp cclass=VCVTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTN_vfp cclass=VCVTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTN_vfp cclass=VCVTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTN_vfp cclass=VCVTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTN_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTN_vfp cclass=VCVTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTP simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTP_asimd cclass=VCVTP_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTP_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx OP=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTP simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VCVTP_asimd cclass=VCVTP_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTP_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTP simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTP_asimd cclass=VCVTP_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTP_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b0 RM:u=0b10 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTP simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VCVTP_asimd cclass=VCVTP_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CONVERSION',
};

ENCODING VCVTP_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTP_vfp cclass=VCVTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTP_vfp cclass=VCVTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VCVTP_vfp cclass=VCVTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTP_vfp cclass=VCVTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTP_vfp cclass=VCVTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTP_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b1 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VCVTP_vfp cclass=VCVTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b100 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'opc2 == 0b101 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTR_iv cclass=VCVTR_iv_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_uiv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b100 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b100 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTR_siv_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2:u=0b101 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'opc2 == 0b101 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VCVTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTR_iv cclass=VCVTR_iv_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|CONVERSION',
};

ENCODING VCVTT_A1_SH => {
  name       => 'Half-precision to single-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sz == 0b0',
  docvars    => 'armarmheading=A1 convert-type=half-to-single instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT cclass=VCVTT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_A1_DH => {
  name       => 'Half-precision to double-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0 && sz == 0b1',
  docvars    => 'armarmheading=A1 convert-type=half-to-double instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT cclass=VCVTT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_A1_HS => {
  name       => 'Single-precision to half-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sz == 0b0',
  docvars    => 'armarmheading=A1 convert-type=single-to-half instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT cclass=VCVTT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_A1_HD => {
  name       => 'Double-precision to half-precision',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1 && sz == 0b1',
  docvars    => 'armarmheading=A1 convert-type=double-to-half instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT cclass=VCVTT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_T1_SH => {
  name       => 'Half-precision to single-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sz == 0b0',
  docvars    => 'armarmheading=T1 convert-type=half-to-single instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT cclass=VCVTT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_T1_DH => {
  name       => 'Half-precision to double-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b0 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0 && sz == 0b1',
  docvars    => 'armarmheading=T1 convert-type=half-to-double instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT cclass=VCVTT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_T1_HS => {
  name       => 'Single-precision to half-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b0 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sz == 0b0',
  docvars    => 'armarmheading=T1 convert-type=single-to-half instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT cclass=VCVTT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_T1_HD => {
  name       => 'Double-precision to half-precision',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 ig2=0b01 op:u=0b1 Vd:u=0bxxxx ig3=0b10 ig4=0b1 sz:u=0b1 T:u=0b1 ig5=0b1 M:u=0bx ig6=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1 && sz == 0b1',
  docvars    => 'armarmheading=T1 convert-type=double-to-half instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT cclass=VCVTT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_A1_bfs => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VCVTT_bfs cclass=VCVTT_bfs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VCVTT_T1_bfs => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b011 Vd:u=0bxxxx ig2=0b10 size=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCVTT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VCVTT_bfs cclass=VCVTT_bfs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION',
};

ENCODING VDIV_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VDIV cclass=VDIV_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDIV_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VDIV cclass=VDIV_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDIV_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VDIV cclass=VDIV_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDIV_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VDIV cclass=VDIV_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDIV_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VDIV cclass=VDIV_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDIV_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VDIV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VDIV cclass=VDIV_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VDOT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VDOT cclass=VDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VDOT cclass=VDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VDOT cclass=VDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VDOT cclass=VDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VDOT_s cclass=VDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VDOT_s cclass=VDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VDOT_s cclass=VDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDOT_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VDOT_s cclass=VDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VDUP_r_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 ig1=0b1 B:u=0bx Q:u=0bx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx ig3=0b0 E:u=0bx ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VDUP_r cclass=VDUP_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VDUP_r_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 ig1=0b1 B:u=0bx Q:u=0bx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx ig3=0b0 E:u=0bx ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VDUP_r cclass=VDUP_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VDUP_s_A1_D => {
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_dup page=VDUP_s cclass=VDUP_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VDUP_s_A1_Q => {
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_dup page=VDUP_s cclass=VDUP_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VDUP_s_T1_D => {
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_dup_sc page=VDUP_s cclass=VDUP_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VDUP_s_T1_Q => {
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 imm4:u=0bxxxx Vd:u=0bxxxx ig2=0b11 opc=0b000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VDUP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_dup_sc page=VDUP_s cclass=VDUP_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BROADCASTING',
};

ENCODING VEOR_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEOR simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VEOR cclass=VEOR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VEOR_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEOR simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VEOR cclass=VEOR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VEOR_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEOR simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VEOR cclass=VEOR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VEOR_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size=0b00 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEOR simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VEOR cclass=VEOR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VEXT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT cclass=VEXT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING VEXT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT cclass=VEXT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING VEXT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT cclass=VEXT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING VEXT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT cclass=VEXT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
};

ENCODING VFMA_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMA cclass=VFMA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMA cclass=VFMA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMA cclass=VFMA_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMA cclass=VFMA_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMA cclass=VFMA_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMA cclass=VFMA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMA cclass=VFMA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMA cclass=VFMA_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMA cclass=VFMA_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VFMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMA cclass=VFMA_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_bf_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMA_bf cclass=VFMA_bf_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_bf_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMA_bf cclass=VFMA_bf_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_bfs_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMA_bfs cclass=VFMA_bfs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMA_bfs_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b0 ig2=0b0 op4=0b0 N:u=0bx Q:u=0bx M:u=0bx U=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMA_bfs cclass=VFMA_bfs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMAL cclass=VFMAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMAL cclass=VFMAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op2 == 0b10 && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMAL cclass=VFMAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b0 D:u=0bx op2:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op2 == 0b11 && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMAL cclass=VFMAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMAL_s cclass=VFMAL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMAL_s cclass=VFMAL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMAL_s cclass=VFMAL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMAL_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMAL_s cclass=VFMAL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMS cclass=VFMS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VFMS cclass=VFMS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMS cclass=VFMS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMS cclass=VFMS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFMS cclass=VFMS_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMS cclass=VFMS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VFMS cclass=VFMS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMS cclass=VFMS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMS cclass=VFMS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMS_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VFMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFMS cclass=VFMS_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMSL cclass=VFMSL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VFMSL cclass=VFMSL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMSL cclass=VFMSL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 ig1=0b0 S:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VFMSL cclass=VFMSL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMSL_s cclass=VFMSL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VFMSL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VFMSL_s cclass=VFMSL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b0 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMSL_s cclass=VFMSL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFMSL_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx ig1=0b0 S:u=0b1 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b0 ig3=0b0 op4=0b0 N:u=0bx Q:u=0b1 M:u=0bx U=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VFMSL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VFMSL_s cclass=VFMSL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMA cclass=VFNMA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMA cclass=VFNMA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMA cclass=VFNMA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMA cclass=VFNMA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMA cclass=VFNMA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMA_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VFNMA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMA cclass=VFNMA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMS cclass=VFNMS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMS cclass=VFNMS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VFNMS cclass=VFNMS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMS cclass=VFNMS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMS cclass=VFNMS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VFNMS_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b1 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VFNMS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VFNMS cclass=VFNMS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VHADD_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHADD simdvectorsize=double',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHADD cclass=VHADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHADD_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHADD simdvectorsize=quad',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHADD cclass=VHADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHADD_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHADD simdvectorsize=double',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHADD cclass=VHADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHADD_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b0 ig3=0b0 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHADD simdvectorsize=quad',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHADD cclass=VHADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHSUB_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHSUB simdvectorsize=double',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHSUB cclass=VHSUB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHSUB_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VHSUB simdvectorsize=quad',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VHSUB cclass=VHSUB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHSUB_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHSUB simdvectorsize=double',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHSUB cclass=VHSUB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VHSUB_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b00 op:u=0b1 ig3=0b0 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VHSUB simdvectorsize=quad',
  docvars2   => 'halving=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VHSUB cclass=VHSUB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING',
};

ENCODING VINS_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VINS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpextins page=VINS cclass=VINS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VINS_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VINS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_extins page=VINS cclass=VINS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VJCVT_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b1 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 convert-type=double-to-single instr-class=fpsimd isa=A32 mnemonic=VJCVT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VJCVT cclass=VJCVT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION|SIGNED',
};

ENCODING VJCVT_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b1 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 convert-type=double-to-single instr-class=fpsimd isa=T32 mnemonic=VJCVT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VJCVT cclass=VJCVT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|CONVERSION|SIGNED',
};

ENCODING VLD1_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD1_1 cclass=VLD1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD1_1 cclass=VLD1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD1_a cclass=VLD1_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD1_a cclass=VLD1_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD1_a cclass=VLD1_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD1_a cclass=VLD1_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD1_a cclass=VLD1_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_a_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b00 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD1_a cclass=VLD1_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A4_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A4_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_A4_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD1_m cclass=VLD1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T4_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T4_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD1_m_T4_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VLD1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD1_m cclass=VLD1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD2_1 cclass=VLD2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD2_1 cclass=VLD2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD2_a cclass=VLD2_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD2_a cclass=VLD2_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD2_a cclass=VLD2_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD2_a cclass=VLD2_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD2_a cclass=VLD2_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_a_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b01 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD2_a cclass=VLD2_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD2_m cclass=VLD2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD2_m_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD2_m cclass=VLD2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD3_1 cclass=VLD3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD3_1 cclass=VLD3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD3_a cclass=VLD3_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD3_a cclass=VLD3_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD3_a cclass=VLD3_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD3_a cclass=VLD3_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD3_a cclass=VLD3_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_a_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b10 size:u=0bxx T:u=0bx a:u=0b0 Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD3_a cclass=VLD3_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD3_m cclass=VLD3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD3_m cclass=VLD3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD3_m cclass=VLD3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD3_m cclass=VLD3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD3_m cclass=VLD3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD3_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD3_m cclass=VLD3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VLD4_1 cclass=VLD4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VLD4_1 cclass=VLD4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD4_a cclass=VLD4_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD4_a cclass=VLD4_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldv_ssall page=VLD4_a cclass=VLD4_a_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD4_a cclass=VLD4_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD4_a cclass=VLD4_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_a_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 N=0b11 size:u=0bxx T:u=0bx a:u=0bx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldall page=VLD4_a cclass=VLD4_a_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD4_m cclass=VLD4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD4_m cclass=VLD4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VLD4_m cclass=VLD4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD4_m cclass=VLD4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD4_m cclass=VLD4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLD4_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b1 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VLD4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VLD4_m cclass=VLD4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDMDB_A1 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VLDMDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM cclass=VLDM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDM_A1 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM cclass=VLDM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDMDB_A2 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VLDMDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM cclass=VLDM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDM_A2 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDM cclass=VLDM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDMDB_T1 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VLDMDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM cclass=VLDM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDM_T1 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM cclass=VLDM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDMDB_T2 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VLDMDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM cclass=VLDM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDM_T2 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDM cclass=VLDM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_i cclass=VLDR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_i cclass=VLDR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111 Rn!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_i cclass=VLDR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_i cclass=VLDR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_i cclass=VLDR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  cnsts      => 'Rn!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_i cclass=VLDR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=literal armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_l cclass=VLDR_l_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=literal armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_l cclass=VLDR_l_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=literal armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VLDR_l cclass=VLDR_l_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=literal armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_l cclass=VLDR_l_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=literal armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_l cclass=VLDR_l_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VLDR_l_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b1 Rn:u=0b1111 Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=literal armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VLDR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VLDR_l cclass=VLDR_l_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|LOAD',
};

ENCODING VMAX_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_f cclass=VMAX_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_f cclass=VMAX_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_f cclass=VMAX_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_f cclass=VMAX_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_i cclass=VMAX_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b1 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAX simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAX_i cclass=VMAX_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_i cclass=VMAX_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAX_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b1 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAX simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAX_i cclass=VMAX_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAXNM_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAXNM simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAXNM cclass=VMAXNM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAXNM_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMAXNM simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMAXNM cclass=VMAXNM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAXNM_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMAXNM cclass=VMAXNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMAXNM_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMAXNM cclass=VMAXNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMAXNM_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMAXNM cclass=VMAXNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMAXNM_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAXNM simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAXNM cclass=VMAXNM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAXNM_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMAXNM simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMAXNM cclass=VMAXNM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMAXNM_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMAXNM cclass=VMAXNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMAXNM_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMAXNM cclass=VMAXNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMAXNM_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMAXNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMAXNM cclass=VMAXNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMIN_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_f cclass=VMIN_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_f cclass=VMIN_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_f cclass=VMIN_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_f cclass=VMIN_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_i cclass=VMIN_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b1 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMIN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMIN_i cclass=VMIN_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_i cclass=VMIN_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMIN_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx Q:u=0b1 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMIN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMIN_i cclass=VMIN_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMINNM_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMINNM simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMINNM cclass=VMINNM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMINNM_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMINNM simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMINNM cclass=VMINNM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMINNM_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMINNM cclass=VMINNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMINNM_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMINNM cclass=VMINNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMINNM_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpminmaxnm page=VMINNM cclass=VMINNM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMINNM_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMINNM simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMINNM cclass=VMINNM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMINNM_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMINNM simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMINNM cclass=VMINNM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VMINNM_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMINNM cclass=VMINNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMINNM_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMINNM cclass=VMINNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMINNM_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMINNM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_minmax page=VMINNM cclass=VMINNM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH|MIN_MAX',
};

ENCODING VMLA_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_f cclass=VMLA_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_f cclass=VMLA_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLA_f cclass=VMLA_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLA_f cclass=VMLA_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLA_f cclass=VMLA_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_f cclass=VMLA_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_f cclass=VMLA_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLA_f cclass=VMLA_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLA_f cclass=VMLA_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_f_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLA_f cclass=VMLA_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_i cclass=VMLA_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLA_i cclass=VMLA_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_i cclass=VMLA_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLA_i cclass=VMLA_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLA_s cclass=VMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLA_s cclass=VMLA_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLA_s cclass=VMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLA_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLA_s cclass=VMLA_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLAL_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMLAL_i cclass=VMLAL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLAL_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMLAL_i cclass=VMLAL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLAL_s_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLAL_s cclass=VMLAL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLAL_s_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLAL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLAL_s cclass=VMLAL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_f cclass=VMLS_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_f cclass=VMLS_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLS_f cclass=VMLS_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLS_f cclass=VMLS_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMLS_f cclass=VMLS_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_f cclass=VMLS_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_f cclass=VMLS_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLS_f cclass=VMLS_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLS_f cclass=VMLS_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_f_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMLS_f cclass=VMLS_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_i cclass=VMLS_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMLS_i cclass=VMLS_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_i cclass=VMLS_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMLS_i cclass=VMLS_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLS_s cclass=VMLS_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLS_s cclass=VMLS_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLS_s cclass=VMLS_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLS_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b0 F:u=0bx N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLS_s cclass=VMLS_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLSL_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMLSL_i cclass=VMLSL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLSL_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMLSL_i cclass=VMLSL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLSL_s_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMLSL_s cclass=VMLSL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMLSL_s_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b10 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMLSL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMLSL_s cclass=VMLSL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMMLA_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op=0b0 sz=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VMMLA cclass=VMMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMMLA_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op=0b0 sz=0b0 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VMMLA cclass=VMMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VMOV_tod_A1 => {
  name       => 'From general-purpose registers',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=from-gps',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_d cclass=VMOV_d_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_d_A1 => {
  name       => 'To general-purpose registers',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=to-gps',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_d cclass=VMOV_d_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_tod_T1 => {
  name       => 'From general-purpose registers',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=from-gps',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_d cclass=VMOV_d_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_d_T1 => {
  name       => 'To general-purpose registers',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b11 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=to-gps',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_d cclass=VMOV_d_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_toh_A1 => {
  name       => 'From general-purpose register',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0b0 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=from-gp',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp16 page=VMOV_h cclass=VMOV_h_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_h_A1 => {
  name       => 'To general-purpose register',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=to-gp',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp16 page=VMOV_h cclass=VMOV_h_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_toh_T1 => {
  name       => 'From general-purpose register',
  diagram    => 'ig0=0b11101110000 op:u=0b0 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=from-gp',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=p_cpaf page=VMOV_h cclass=VMOV_h_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_h_T1 => {
  name       => 'To general-purpose register',
  diagram    => 'ig0=0b11101110000 op:u=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1001 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=to-gp',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=p_cpaf page=VMOV_h cclass=VMOV_h_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpimm page=VMOV_i cclass=VMOV_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpimm page=VMOV_i cclass=VMOV_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpimm page=VMOV_i cclass=VMOV_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_A3_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A3_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A4_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A4_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=VMOV_i_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A5_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0b0 op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A5 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=a5',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_A5_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0b1 op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A5 instr-class=fpsimd isa=A32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMOV_i cclass=a5',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b01 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_movi page=VMOV_i cclass=VMOV_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b10 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_movi page=VMOV_i cclass=VMOV_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 imm4H:u=0bxxxx Vd:u=0bxxxx ig2=0b10 size:u=0b11 ig3=0bZ ig4=0b0 ig5=0bZ ig6=0b0 imm4L:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_movi page=VMOV_i cclass=VMOV_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_i_T3_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T3_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T4_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0b0 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T4_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b11xx ig3=0b0 Q:u=0b1 op:u=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T5_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0b0 op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T5 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t5',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_i_T5_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b1110 ig3=0b0 Q:u=0b1 op:u=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T5 instr-class=fpsimd isa=T32 mnemonic=VMOV simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMOV_i cclass=VMOV_i_t5',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_r_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VMOV_r cclass=VMOV_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_r_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VMOV_r cclass=VMOV_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_r_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VMOV_r cclass=VMOV_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_r_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b000 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VMOV_r cclass=VMOV_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FLOAT|DATA_TRANSFER',
};

ENCODING VMOV_rs_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 ig1=0b0 opc1:u=0bxx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx opc2:u=0bxx ig3=0b1 ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VMOV_rs cclass=VMOV_rs_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_rs_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 ig1=0b0 opc1:u=0bxx L=0b0 Vd:u=0bxxxx Rt:u=0bxxxx ig2=0b1011 D:u=0bx opc2:u=0bxx ig3=0b1 ig4=0bZ ig5=0bZ ig6=0bZ ig7=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VMOV_rs cclass=VMOV_rs_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_tos_A1 => {
  name       => 'From general-purpose register',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0b0 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=from-gp',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp32 page=VMOV_s cclass=VMOV_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_s_A1 => {
  name       => 'To general-purpose register',
  diagram    => 'cond:u=0bxxxx ig0=0b1110000 op:u=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=to-gp',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpgp32 page=VMOV_s cclass=VMOV_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_tos_T1 => {
  name       => 'From general-purpose register',
  diagram    => 'ig0=0b11101110000 op:u=0b0 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=from-gp',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_mov32 page=VMOV_s cclass=VMOV_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_s_T1 => {
  name       => 'To general-purpose register',
  diagram    => 'ig0=0b11101110000 op:u=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 N:u=0bx ig2=0bZ ig3=0bZ ig4=0b1 ig5=0bZ ig6=0bZ ig7=0bZ ig8=0bZ',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=to-gp',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_mov32 page=VMOV_s cclass=VMOV_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_sr_A1 => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 U:u=0bx opc1:u=0bxx L=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movsimdgp page=VMOV_sr cclass=VMOV_sr_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_sr_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110 U:u=0bx opc1:u=0bxx L=0b1 Vn:u=0bxxxx Rt:u=0bxxxx ig1=0b1011 N:u=0bx opc2:u=0bxx ig2=0b1 ig3=0bZ ig4=0bZ ig5=0bZ ig6=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=simd_dup_el page=VMOV_sr cclass=VMOV_sr_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOV_toss_A1 => {
  name       => 'From general-purpose registers',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=from-gps',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_ss cclass=VMOV_ss_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_ss_A1 => {
  name       => 'To general-purpose registers',
  diagram    => 'cond:u=0bxxxx ig0=0b11000 D=0b1 ig1=0b0 op:u=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOV to-or-from-gp=to-gps',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=movsimdfpgp64 page=VMOV_ss cclass=VMOV_ss_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_toss_T1 => {
  name       => 'From general-purpose registers',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0b0 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=from-gps',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_ss cclass=VMOV_ss_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOV_ss_T1 => {
  name       => 'To general-purpose registers',
  diagram    => 'ig0=0b111011000 D=0b1 ig1=0b0 op:u=0b1 Rt2:u=0bxxxx Rt:u=0bxxxx ig2=0b10 size=0b10 opc2=0b00 M:u=0bx o3=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOV to-or-from-gp=to-gps',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_mov64 page=VMOV_ss cclass=VMOV_ss_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMOVL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm3H:u=0bxxx imm3L=0b000 Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm3H!=0b000',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VMOVL cclass=VMOVL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOVL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm3H:u=0bxxx imm3L=0b000 Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm3H!=0b000',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VMOVL cclass=VMOVL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOVN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VMOVN cclass=VMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOVN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VMOVN cclass=VMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOVX_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVX',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpextins page=VMOVX cclass=VMOVX_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMOVX_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b110000 Vd:u=0bxxxx ig2=0b10 size=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_extins page=VMOVX cclass=VMOVX_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VMRS_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110111 L=0b1 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMRS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpsr page=VMRS cclass=VMRS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|CP14DTTRAP|CP14RRTTRAP|CP14RTTRAP|CP15RRTTRAP|CP15RTTRAP|FPIDTRAP|UNCATEGORIZED',
  pstate     => 'C=W EL=CR N=W V=W Z=W',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMRS_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110111 L=0b1 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMRS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_msr page=VMRS cclass=VMRS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|CP14DTTRAP|CP14RRTTRAP|CP14RTTRAP|CP15RRTTRAP|CP15RTTRAP|FPIDTRAP|UNCATEGORIZED',
  pstate     => 'C=CW EL=CR N=CW V=CW Z=CW',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMSR_A1_AS => {
  name       => 'A1',
  diagram    => 'cond:u=0bxxxx ig0=0b1110111 L=0b0 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  cnsts      => 'cond!=0b1111',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMSR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=movfpsr page=VMSR cclass=VMSR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  pstate     => 'EL=CR',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMSR_T1_AS => {
  name       => 'T1',
  diagram    => 'ig0=0b11101110111 L=0b0 reg:u=0bxxxx Rt:u=0bxxxx ig1=0b1010 ig2=0bZ ig3=0bZ ig4=0bZ ig5=0b1 ig6=0bZ ig7=0bZ ig8=0bZ ig9=0bZ',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMSR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpsimd_mov32 iclass=fp_msr page=VMSR cclass=VMSR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  pstate     => 'EL=CR',
  categories => 'FPSIMD|DATA_TRANSFER',
};

ENCODING VMUL_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_f cclass=VMUL_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_f cclass=VMUL_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_f_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMUL_f cclass=VMUL_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_f_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMUL_f cclass=VMUL_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_f_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VMUL_f cclass=VMUL_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_f cclass=VMUL_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_f cclass=VMUL_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_f_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMUL_f cclass=VMUL_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_f_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMUL_f cclass=VMUL_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_f_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VMUL_f cclass=VMUL_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VMUL_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_i cclass=VMUL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMUL_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 op:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMUL_i cclass=VMUL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMUL_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_i cclass=VMUL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMUL_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 op:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMUL_i cclass=VMUL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMUL_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMUL_s cclass=VMUL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMUL_s cclass=VMUL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMUL_s cclass=VMUL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMUL_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b100 F:u=0bx N:u=0bx ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMUL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMUL_s cclass=VMUL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMULL_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bx ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VMULL_i cclass=VMULL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMULL_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b11 op:u=0bx ig3=0b0 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VMULL_i cclass=VMULL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|POLYNOMIAL',
};

ENCODING VMULL_s_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VMULL_s cclass=VMULL_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMULL_s_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMULL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VMULL_s cclass=VMULL_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VMVN_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_A3_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_A3_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VMVN_i cclass=VMVN_i_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx0 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x0 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T3_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_i_T3_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b110x ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VMVN_i cclass=VMVN_i_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VMVN_r cclass=VMVN_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VMVN_r cclass=VMVN_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VMVN_r cclass=VMVN_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VMVN_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMVN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VMVN_r cclass=VMVN_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNEG simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VNEG cclass=VNEG_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VNEG simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VNEG cclass=VNEG_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VNEG cclass=VNEG_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VNEG cclass=VNEG_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VNEG cclass=VNEG_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNEG simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VNEG cclass=VNEG_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b01 Vd:u=0bxxxx ig2=0b0 F:u=0bx ig3=0b111 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VNEG simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VNEG cclass=VNEG_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VNEG cclass=VNEG_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VNEG cclass=VNEG_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNEG_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VNEG',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VNEG cclass=VNEG_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VNMLA_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLA cclass=VNMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLA_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLA cclass=VNMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLA_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLA cclass=VNMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLA_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLA cclass=VNMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLA_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLA cclass=VNMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLA_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VNMLA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLA cclass=VNMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLS cclass=VNMLS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLS cclass=VNMLS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMLS cclass=VNMLS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLS cclass=VNMLS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLS cclass=VNMLS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMLS_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx op:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VNMLS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMLS cclass=VNMLS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMUL cclass=VNMUL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMUL cclass=VNMUL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VNMUL cclass=VNMUL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMUL cclass=VNMUL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMUL cclass=VNMUL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VNMUL_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VNMUL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VNMUL cclass=VNMUL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VORN_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORN_r cclass=VORN_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORN_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORN_r cclass=VORN_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORN_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORN_r cclass=VORN_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORN_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b11 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORN_r cclass=VORN_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i cclass=VORR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i cclass=VORR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i cclass=VORR_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORR_i cclass=VORR_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i cclass=VORR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i cclass=VORR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i cclass=VORR_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_i_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORR_i cclass=VORR_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORR_r cclass=VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VORR_r cclass=VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORR_r cclass=VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VORR_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VORR_r cclass=VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
};

ENCODING VPADAL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADAL cclass=VPADAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VPADAL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADAL cclass=VPADAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VPADAL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADAL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADAL cclass=VPADAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VPADAL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b110 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADAL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADAL cclass=VPADAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VPADD_f_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADD',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPADD_f cclass=VPADD_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|ARITHMETIC',
};

ENCODING VPADD_f_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0bx M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADD',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPADD_f cclass=VPADD_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|ARITHMETIC',
};

ENCODING VPADD_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADD simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPADD_i cclass=VPADD_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPADD_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0bx M:u=0bx o1=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADD simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPADD_i cclass=VPADD_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPADDL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADDL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADDL cclass=VPADDL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPADDL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPADDL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VPADDL cclass=VPADDL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPADDL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADDL simdvectorsize=double',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADDL cclass=VPADDL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPADDL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0bx Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPADDL simdvectorsize=quad',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VPADDL cclass=VPADDL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VPMAX_f_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMAX_f cclass=VPMAX_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|MATH|MIN_MAX',
};

ENCODING VPMAX_f_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMAX_f cclass=VPMAX_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|MATH|MIN_MAX',
};

ENCODING VPMAX_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMAX_i cclass=VPMAX_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VPMAX_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMAX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMAX_i cclass=VPMAX_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VPMIN_f_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMIN_f cclass=VPMIN_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|MATH|MIN_MAX',
};

ENCODING VPMIN_f_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMIN_f cclass=VPMIN_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FPSIMD|MATH|MIN_MAX',
};

ENCODING VPMIN_i_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VPMIN_i cclass=VPMIN_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VPMIN_i_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1010 N:u=0bx Q=0b0 M:u=0bx op:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VPMIN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VPMIN_i cclass=VPMIN_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|MIN_MAX',
};

ENCODING VQABS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQABS simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQABS cclass=VQABS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING',
};

ENCODING VQABS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQABS simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQABS cclass=VQABS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING',
};

ENCODING VQABS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQABS simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQABS cclass=VQABS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING',
};

ENCODING VQABS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1110 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQABS simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQABS cclass=VQABS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH|SATURATING',
};

ENCODING VQADD_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQADD simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQADD cclass=VQADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQADD_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQADD simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQADD cclass=VQADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQADD_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQADD simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQADD cclass=VQADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQADD_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQADD simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQADD cclass=VQADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMLAL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMLAL cclass=VQDMLAL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLAL_A2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMLAL cclass=VQDMLAL_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLAL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b0 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMLAL cclass=VQDMLAL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLAL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b0 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMLAL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMLAL cclass=VQDMLAL_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLSL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMLSL cclass=VQDMLSL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLSL_A2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMLSL cclass=VQDMLSL_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLSL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 op:u=0b1 ig3=0b1 N:u=0bx ig4=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMLSL cclass=VQDMLSL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMLSL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b0 op:u=0b1 ig3=0b11 N:u=0bx ig4=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMLSL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMLSL cclass=VQDMLSL_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SATURATING',
};

ENCODING VQDMULH_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQDMULH cclass=VQDMULH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQDMULH cclass=VQDMULH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMULH cclass=VQDMULH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMULH cclass=VQDMULH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQDMULH cclass=VQDMULH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQDMULH cclass=VQDMULH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMULH cclass=VQDMULH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULH_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMULH cclass=VQDMULH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VQDMULL cclass=VQDMULL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULL_A2 => {
  name       => 'A2',
  diagram    => 'ig0=0b1111001 Q=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQDMULL cclass=VQDMULL_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VQDMULL cclass=VQDMULL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQDMULL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111 Q=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQDMULL',
  docvars2   => 'doubling=1 saturating=1 shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQDMULL cclass=VQDMULL_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQMOVN_A1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1x',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQMOVN cclass=VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SATURATING',
};

ENCODING VQMOVUN_A1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b01',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQMOVN cclass=VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SATURATING',
};

ENCODING VQMOVN_T1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1x',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQMOVN cclass=VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SATURATING',
};

ENCODING VQMOVUN_T1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b01',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQMOVN cclass=VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|SATURATING',
};

ENCODING VQNEG_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQNEG simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQNEG cclass=VQNEG_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING',
};

ENCODING VQNEG_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQNEG simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQNEG cclass=VQNEG_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING',
};

ENCODING VQNEG_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQNEG simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQNEG cclass=VQNEG_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING',
};

ENCODING VQNEG_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 opc2=0b1111 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQNEG simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQNEG cclass=VQNEG_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE|SATURATING',
};

ENCODING VQRDMLAH_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLAH cclass=VQRDMLAH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLAH cclass=VQRDMLAH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLAH cclass=VQRDMLAH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLAH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLAH cclass=VQRDMLAH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLAH cclass=VQRDMLAH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLAH cclass=VQRDMLAH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLAH cclass=VQRDMLAH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLAH_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLAH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLAH cclass=VQRDMLAH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLSH cclass=VQRDMLSH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMLSH cclass=VQRDMLSH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLSH cclass=VQRDMLSH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMLSH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMLSH cclass=VQRDMLSH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLSH cclass=VQRDMLSH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMLSH cclass=VQRDMLSH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLSH cclass=VQRDMLSH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMLSH_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMLSH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMLSH cclass=VQRDMLSH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMULH cclass=VQRDMULH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRDMULH cclass=VQRDMULH_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMULH cclass=VQRDMULH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 Q:u=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VQRDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_scalar page=VQRDMULH cclass=VQRDMULH_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMULH cclass=VQRDMULH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRDMULH cclass=VQRDMULH_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH simdvectorsize=double',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMULH cclass=VQRDMULH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRDMULH_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 Q:u=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx ig2=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VQRDMULH simdvectorsize=quad',
  docvars2   => 'doubling=1 rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_sc page=VQRDMULH cclass=VQRDMULH_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING|SATURATING',
};

ENCODING VQRSHL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRSHL simdvectorsize=double',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRSHL cclass=VQRSHL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING',
};

ENCODING VQRSHL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRSHL simdvectorsize=quad',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQRSHL cclass=VQRSHL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING',
};

ENCODING VQRSHL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRSHL simdvectorsize=double',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRSHL cclass=VQRSHL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING',
};

ENCODING VQRSHL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRSHL simdvectorsize=quad',
  docvars2   => 'rounding=1 saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQRSHL cclass=VQRSHL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING|SATURATING',
};

ENCODING VQRSHRN_A1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b1 L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => '!(imm6 == 0b000xxx) && op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRSHRN result-type=signed-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQRSHRN cclass=VQRSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING VQRSHRUN_A1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b1111001 U:u=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b0 L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx) && op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQRSHRUN result-type=unsigned-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQRSHRN cclass=VQRSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING VQRSHRN_T1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b1 L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => '!(imm6 == 0b000xxx) && op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRSHRN result-type=signed-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQRSHRN cclass=VQRSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING VQRSHRUN_T1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111 U:u=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b0 L=0b0 Q=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx) && op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQRSHRUN result-type=unsigned-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQRSHRN cclass=VQRSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
};

ENCODING VQSHL_i_A1_D => {
  name       => 'VQSHL,double,signed-result',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b1 L:u=0bx Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && op == 0b1 && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHL result-type=signed-result simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHL_i cclass=VQSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_i_A1_Q => {
  name       => 'VQSHL,quad,signed-result',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b1 L:u=0bx Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && op == 0b1 && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHL result-type=signed-result simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHL_i cclass=VQSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHLU_i_A1_D => {
  name       => 'VQSHLU,double,unsigned-result',
  diagram    => 'ig0=0b1111001 U:u=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b0 L:u=0bx Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx && L == 0b0) && op == 0b0 && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHLU result-type=unsigned-result simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHL_i cclass=VQSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHLU_i_A1_Q => {
  name       => 'VQSHLU,quad,unsigned-result',
  diagram    => 'ig0=0b1111001 U:u=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b0 L:u=0bx Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx && L == 0b0) && op == 0b0 && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHLU result-type=unsigned-result simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHL_i cclass=VQSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_i_T1_D => {
  name       => 'VQSHL,double,signed-result',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b1 L:u=0bx Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && op == 0b1 && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHL result-type=signed-result simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHL_i cclass=VQSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_i_T1_Q => {
  name       => 'VQSHL,quad,signed-result',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b1 L:u=0bx Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && op == 0b1 && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHL result-type=signed-result simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHL_i cclass=VQSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHLU_i_T1_D => {
  name       => 'VQSHLU,double,unsigned-result',
  diagram    => 'ig0=0b111 U:u=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b0 L:u=0bx Q:u=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx && L == 0b0) && op == 0b0 && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHLU result-type=unsigned-result simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHL_i cclass=VQSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHLU_i_T1_Q => {
  name       => 'VQSHLU,quad,unsigned-result',
  diagram    => 'ig0=0b111 U:u=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b011 op:u=0b0 L:u=0bx Q:u=0b1 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx && L == 0b0) && op == 0b0 && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHLU result-type=unsigned-result simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHL_i cclass=VQSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHL simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSHL_r cclass=VQSHL_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHL simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSHL_r cclass=VQSHL_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHL simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSHL_r cclass=VQSHL_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHL_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHL simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSHL_r cclass=VQSHL_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|SATURATING',
};

ENCODING VQSHRN_A1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b1 L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => '!(imm6 == 0b000xxx) && op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHRN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHRN cclass=VQSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING VQSHRUN_A1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b1111001 U:u=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b0 L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx) && op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSHRUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VQSHRN cclass=VQSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING VQSHRN_T1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b1 L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => '!(imm6 == 0b000xxx) && op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHRN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHRN cclass=VQSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING VQSHRUN_T1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111 U:u=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx ig2=0b100 op:u=0b0 L=0b0 Q=0b0 M:u=0bx ig3=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'U == 0b1 && !(imm6 == 0b000xxx) && op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSHRUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VQSHRN cclass=VQSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
};

ENCODING VQSUB_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSUB simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSUB cclass=VQSUB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQSUB_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQSUB simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VQSUB cclass=VQSUB_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQSUB_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSUB simdvectorsize=double',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSUB cclass=VQSUB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VQSUB_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0010 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQSUB simdvectorsize=quad',
  docvars2   => 'saturating=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VQSUB cclass=VQSUB_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|SATURATING',
};

ENCODING VRADDHN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRADDHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VRADDHN cclass=VRADDHN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING VRADDHN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRADDHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VRADDHN cclass=VRADDHN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING VRECPE_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRECPE cclass=VRECPE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPE_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRECPE cclass=VRECPE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPE_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRECPE cclass=VRECPE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPE_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b0 Q:u=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRECPE cclass=VRECPE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRECPS cclass=VRECPS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRECPS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRECPS cclass=VRECPS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRECPS cclass=VRECPS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRECPS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRECPS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRECPS cclass=VRECPS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VREV16_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV16 simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV16 cclass=VREV16_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV16_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV16 simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV16 cclass=VREV16_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV16_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV16 simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV16 cclass=VREV16_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV16_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b10 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV16 simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV16 cclass=VREV16_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV32_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV32 simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV32 cclass=VREV32_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV32_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV32 simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV32 cclass=VREV32_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV32_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV32 simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV32 cclass=VREV32_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV32_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b01 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV32 simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV32 cclass=VREV32_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV64_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV64 simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV64 cclass=VREV64_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV64_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VREV64 simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VREV64 cclass=VREV64_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV64_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV64 simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV64 cclass=VREV64_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VREV64_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b00 Vd:u=0bxxxx ig2=0b0 ig3=0b00 op:u=0b00 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VREV64 simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VREV64 cclass=VREV64_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VRHADD_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRHADD simdvectorsize=double',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRHADD cclass=VRHADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING',
};

ENCODING VRHADD_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRHADD simdvectorsize=quad',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRHADD cclass=VRHADD_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING',
};

ENCODING VRHADD_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRHADD simdvectorsize=double',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRHADD cclass=VRHADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING',
};

ENCODING VRHADD_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRHADD simdvectorsize=quad',
  docvars2   => 'halving=1 rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRHADD cclass=VRHADD_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|HALVING|ROUNDING',
};

ENCODING VRINTA_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTA_asimd cclass=VRINTA_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTA_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTA_asimd cclass=VRINTA_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTA_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTA_asimd cclass=VRINTA_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTA_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b010 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTA_asimd cclass=VRINTA_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTA_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTA_vfp cclass=VRINTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTA_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTA_vfp cclass=VRINTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTA_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTA_vfp cclass=VRINTA_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTA_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTA_vfp cclass=VRINTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTA_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTA_vfp cclass=VRINTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTA_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b00 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTA',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTA_vfp cclass=VRINTA_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTM simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTM_asimd cclass=VRINTM_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTM_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTM simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTM_asimd cclass=VRINTM_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTM_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTM simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTM_asimd cclass=VRINTM_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTM_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b101 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTM simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTM_asimd cclass=VRINTM_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTM_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTM_vfp cclass=VRINTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTM_vfp cclass=VRINTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTM_vfp cclass=VRINTM_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTM_vfp cclass=VRINTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTM_vfp cclass=VRINTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTM_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b11 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTM_vfp cclass=VRINTM_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTN_asimd cclass=VRINTN_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTN_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTN_asimd cclass=VRINTN_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTN_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTN_asimd cclass=VRINTN_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTN_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b000 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTN_asimd cclass=VRINTN_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTN_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTN_vfp cclass=VRINTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTN_vfp cclass=VRINTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTN_vfp cclass=VRINTN_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTN_vfp cclass=VRINTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTN_vfp cclass=VRINTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTN_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b01 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTN',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTN_vfp cclass=VRINTN_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTP simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTP_asimd cclass=VRINTP_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTP_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTP simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTP_asimd cclass=VRINTP_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTP_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTP simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTP_asimd cclass=VRINTP_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTP_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b1 op:u=0b111 Q:u=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTP simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTP_asimd cclass=VRINTP_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTP_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTP_vfp cclass=VRINTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTP_vfp cclass=VRINTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcvtrnd page=VRINTP_vfp cclass=VRINTP_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTP_vfp cclass=VRINTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTP_vfp cclass=VRINTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTP_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111111101 D:u=0bx ig1=0b111 o1=0b0 RM:u=0b10 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTP',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_toint page=VRINTP_vfp cclass=VRINTP_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTR_vfp cclass=VRINTR_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTR_vfp cclass=VRINTR_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTR_vfp cclass=VRINTR_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTR_vfp cclass=VRINTR_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTR_vfp cclass=VRINTR_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTR_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTR_vfp cclass=VRINTR_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTX simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTX_asimd cclass=VRINTX_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTX_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTX simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTX_asimd cclass=VRINTX_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTX_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTX simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTX_asimd cclass=VRINTX_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTX_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTX simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTX_asimd cclass=VRINTX_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTX_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTX_vfp cclass=VRINTX_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTX_vfp cclass=VRINTX_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTX_vfp cclass=VRINTX_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTX_vfp cclass=VRINTX_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTX_vfp cclass=VRINTX_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTX_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b111 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b0 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTX_vfp cclass=VRINTX_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_asimd_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTZ simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTZ_asimd cclass=VRINTZ_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTZ_asimd_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRINTZ simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRINTZ_asimd cclass=VRINTZ_asimd_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTZ_asimd_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTZ simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTZ_asimd cclass=VRINTZ_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTZ_asimd_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b1011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRINTZ simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRINTZ_asimd cclass=VRINTZ_asimd_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ROUNDING',
};

ENCODING VRINTZ_vfp_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTZ_vfp cclass=VRINTZ_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_vfp_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTZ_vfp cclass=VRINTZ_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_vfp_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VRINTZ_vfp cclass=VRINTZ_vfp_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_vfp_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b01 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTZ_vfp cclass=VRINTZ_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_vfp_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b10 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTZ_vfp cclass=VRINTZ_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRINTZ_vfp_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b110 Vd:u=0bxxxx ig2=0b10 size:u=0b11 op:u=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VRINTZ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VRINTZ_vfp cclass=VRINTZ_vfp_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ROUNDING',
};

ENCODING VRSHL_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHL simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHL cclass=VRSHL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING',
};

ENCODING VRSHL_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHL simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHL cclass=VRSHL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING',
};

ENCODING VRSHL_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHL simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHL cclass=VRSHL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING',
};

ENCODING VRSHL_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHL simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHL cclass=VRSHL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|ROUNDING',
};

ENCODING VRSHR_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHR simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSHR cclass=VRSHR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSHR_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHR simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSHR cclass=VRSHR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSHR_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHR simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSHR cclass=VRSHR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSHR_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0010 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHR simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSHR cclass=VRSHR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSHRN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSHRN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSHRN cclass=VRSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSHRN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSHRN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSHRN cclass=VRSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
};

ENCODING VRSQRTE_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTE simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRSQRTE cclass=VRSQRTE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTE_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTE simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRSQRTE cclass=VRSQRTE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTE_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0b0 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTE simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRSQRTE cclass=VRSQRTE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTE_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b11 Vd:u=0bxxxx ig2=0b0 ig3=0b10 F:u=0bx ig4=0b1 Q:u=0b1 M:u=0bx ig5=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTE simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRSQRTE cclass=VRSQRTE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTS_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTS simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSQRTS cclass=VRSQRTS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTS_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSQRTS simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSQRTS cclass=VRSQRTS_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTS_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTS simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSQRTS cclass=VRSQRTS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSQRTS_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1111 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSQRTS simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSQRTS cclass=VRSQRTS_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|MATH',
};

ENCODING VRSRA_A1_Q => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSRA simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSRA cclass=VRSRA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING',
};

ENCODING VRSRA_A1_D => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSRA simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VRSRA cclass=VRSRA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING',
};

ENCODING VRSRA_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSRA simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSRA cclass=VRSRA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING',
};

ENCODING VRSRA_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0011 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSRA simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VRSRA cclass=VRSRA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC|ROUNDING',
};

ENCODING VRSUBHN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VRSUBHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VRSUBHN cclass=VRSUBHN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING VRSUBHN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VRSUBHN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VRSUBHN cclass=VRSUBHN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC|ROUNDING',
};

ENCODING VSDOT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VSDOT cclass=VSDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VSDOT cclass=VSDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VSDOT cclass=VSDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VSDOT cclass=VSDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSDOT_s cclass=VSDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSDOT_s cclass=VSDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSDOT_s cclass=VSDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSDOT_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSDOT_s cclass=VSDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|SIGNED',
};

ENCODING VSELEQ_A1_D => {
  name       => 'VSELEQ,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELEQ_A1_H => {
  name       => 'VSELEQ,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELEQ_A1_S => {
  name       => 'VSELEQ,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_A1_D => {
  name       => 'VSELGE,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_A1_H => {
  name       => 'VSELGE,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_A1_S => {
  name       => 'VSELGE,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_A1_D => {
  name       => 'VSELGT,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_A1_H => {
  name       => 'VSELGT,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_A1_S => {
  name       => 'VSELGT,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_A1_D => {
  name       => 'VSELVS,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_A1_H => {
  name       => 'VSELVS,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_A1_S => {
  name       => 'VSELVS,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpcsel page=VSEL cclass=VSEL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELEQ_T1_D => {
  name       => 'VSELEQ,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELEQ_T1_H => {
  name       => 'VSELEQ,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELEQ_T1_S => {
  name       => 'VSELEQ,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b00 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSELEQ',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_T1_D => {
  name       => 'VSELGE,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_T1_H => {
  name       => 'VSELGE,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGE_T1_S => {
  name       => 'VSELGE,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b10 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSELGE',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_T1_D => {
  name       => 'VSELGT,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_T1_H => {
  name       => 'VSELGT,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELGT_T1_S => {
  name       => 'VSELGT,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b11 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSELGT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_T1_D => {
  name       => 'VSELVS,doubleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_T1_H => {
  name       => 'VSELVS,halfprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSELVS_T1_S => {
  name       => 'VSELVS,singleprec',
  diagram    => 'ig0=0b111111100 D:u=0bx cc:u=0b01 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b00',
  bitdiffs   => 'cc == 0b01 && size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSELVS',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_csel page=VSEL cclass=VSEL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'FPSIMD|SELECTION|CONDITIONALLY',
};

ENCODING VSHL_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHL_i cclass=VSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHL_i cclass=VSHL_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHL_i cclass=VSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHL_i cclass=VSHL_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHL_r cclass=VSHL_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHL simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHL_r cclass=VSHL_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHL_r cclass=VSHL_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHL_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0100 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHL simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHL_r cclass=VSHL_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHLL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHLL cclass=VSHLL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHLL_A2 => {
  name       => 'A2',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSHLL cclass=VSHLL_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHLL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1010 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHLL cclass=VSHLL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHLL_T2 => {
  name       => 'T2',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0110 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VSHLL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSHLL cclass=VSHLL_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT',
};

ENCODING VSHR_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHR simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHR cclass=VSHR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSHR_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHR simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHR cclass=VSHR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSHR_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHR simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHR cclass=VSHR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSHR_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0000 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHR simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHR cclass=VSHR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSHRN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSHRN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSHRN cclass=VSHRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSHRN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b1000 L=0b0 Q=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6!=0b000xxx',
  bitdiffs   => 'imm6 != 0b000xxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSHRN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSHRN cclass=VSHRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
};

ENCODING VSLI_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSLI simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSLI cclass=VSLI_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING VSLI_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSLI simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSLI cclass=VSLI_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING VSLI_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSLI simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSLI cclass=VSLI_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING VSLI_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0101 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSLI simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSLI cclass=VSLI_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_LEFT|INSERTION',
};

ENCODING VSMMLA_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VSMMLA cclass=VSMMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING VSMMLA_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VSMMLA cclass=VSMMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|SIGNED',
};

ENCODING VSQRT_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VSQRT cclass=VSQRT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSQRT_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VSQRT cclass=VSQRT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSQRT_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b11101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp2reg page=VSQRT cclass=VSQRT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSQRT_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b01 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VSQRT cclass=VSQRT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSQRT_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b10 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VSQRT cclass=VSQRT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSQRT_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b111011101 D:u=0bx ig1=0b11 o1=0b0 opc2=0b001 Vd:u=0bxxxx ig2=0b10 size:u=0b11 o3=0b1 ig3=0b1 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSQRT',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_2r page=VSQRT cclass=VSQRT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|MATH',
};

ENCODING VSRA_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRA simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRA cclass=VSRA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC',
};

ENCODING VSRA_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRA cclass=VSRA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC',
};

ENCODING VSRA_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRA simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRA cclass=VSRA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC',
};

ENCODING VSRA_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0001 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRA cclass=VSRA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|CARITHMETIC',
};

ENCODING VSRI_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRI simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRI cclass=VSRI_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING VSRI_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b1 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSRI simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd2reg_shift page=VSRI cclass=VSRI_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING VSRI_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0b0 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRI simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRI cclass=VSRI_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING VSRI_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11111 D:u=0bx imm6:u=0bxxxxxx Vd:u=0bxxxx opc=0b0100 L:u=0bx Q:u=0b1 M:u=0bx ig2=0b1 Vm:u=0bxxxx',
  cnsts      => 'imm6:L!=0b000xxx0',
  bitdiffs   => '!(imm6 == 0b000xxx && L == 0b0) && Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSRI simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_2r_shift page=VSRI cclass=VSRI_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|INSERTION',
};

ENCODING VST1_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST1_1 cclass=VST1_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b00 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST1_1 cclass=VST1_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A4_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A4_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_A4_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A4 instr-class=fpsimd isa=A32 mnemonic=VST1',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST1_m cclass=VST1_m_a4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0111 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b1010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0110 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T4_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T4_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST1_m_T4_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0010 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T4 instr-class=fpsimd isa=T32 mnemonic=VST1',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST1_m cclass=VST1_m_t4',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST2_1 cclass=VST2_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b01 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST2_1 cclass=VST2_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST2',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST2_m cclass=VST2_m_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b100x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST2_m_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype=0b0011 size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST2',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST2_m cclass=VST2_m_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_A3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST3_1 cclass=VST3_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b10 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST3_1 cclass=VST3_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST3_m cclass=VST3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST3_m cclass=VST3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST3',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST3_m cclass=VST3_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST3_m cclass=VST3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST3_m cclass=VST3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST3_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b010x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST3',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST3_m cclass=VST3_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A3_posti => {
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_A3_postr => {
  name       => 'Reg-offset',
  diagram    => 'ig0=0b111101001 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A3 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ssone page=VST4_1 cclass=VST4_1_a3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b00 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T2_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T2_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T2_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b01 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T3_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T3_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_1_T3_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110011 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx size:u=0b10 N=0b11 index_align:u=0bxxxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T3 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstss page=VST4_1 cclass=VST4_1_t3',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_A1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST4_m cclass=VST4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_A1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST4_m cclass=VST4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_A1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111101000 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VST4',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimdls iclass=ldstv_ms page=VST4_m cclass=VST4_m_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_T1_nowb => {
  name       => 'Offset',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1111',
  bitdiffs   => 'Rm == 0b1111',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST4_m cclass=VST4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_T1_posti => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0b1101',
  bitdiffs   => 'Rm == 0b1101',
  docvars    => 'address-form=post-indexed armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST4_m cclass=VST4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VST4_m_T1_postr => {
  name       => 'Post-indexed',
  diagram    => 'ig0=0b111110010 D:u=0bx L=0b0 ig1=0b0 Rn:u=0bxxxx Vd:u=0bxxxx itype:u=0b000x size:u=0bxx align:u=0bxx Rm:u=0bxxxx',
  cnsts      => 'Rm!=0b11x1',
  bitdiffs   => 'Rm != 0b11x1',
  docvars    => 'address-form=post-indexed address-offset=reg-offset armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VST4',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=vldst iclass=asimldstms page=VST4_m cclass=VST4_m_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTMDB_A1 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM cclass=VSTM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTM_A1 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VSTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM cclass=VSTM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTMDB_A2 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM cclass=VSTM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTM_A2 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VSTM',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTM cclass=VSTM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTMDB_T1 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM cclass=VSTM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTM_T1 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VSTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM cclass=VSTM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTMDB_T2 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b1 && U == 0b0 && W == 0b1',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM cclass=VSTM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTM_T2 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0bx L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'P == 0b0 && U == 0b1',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VSTM',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTM cclass=VSTM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_A1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTR cclass=VSTR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_A1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTR cclass=VSTR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_A1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VSTR cclass=VSTR_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_T1_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTR cclass=VSTR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_T1_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTR cclass=VSTR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSTR_T1_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b1110110 P=0b1 U:u=0bx D:u=0bx W=0b0 L=0b0 Rn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 imm8:u=0bxxxxxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'address-form=base-plus-offset armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSTR',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VSTR cclass=VSTR_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER|STORE',
};

ENCODING VSUB_f_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_f cclass=VSUB_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_f_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_f cclass=VSUB_f_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_f_A2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=A2 fpdatasize=halfprec instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VSUB_f cclass=VSUB_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_f_A2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VSUB_f cclass=VSUB_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_f_A2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'cond:u=0bxxxx ig0=0b1110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  cnsts      => 'cond!=0b1111',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=A2 fpdatasize=doubleprec instr-class=fpsimd isa=A32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fpdp3reg page=VSUB_f cclass=VSUB_f_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_f_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_f cclass=VSUB_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_f_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1101 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_f cclass=VSUB_f_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_f_T2_H => {
  name       => 'Half-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b01 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b01',
  docvars    => 'armarmheading=T2 fpdatasize=halfprec instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VSUB_f cclass=VSUB_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_f_T2_S => {
  name       => 'Single-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b10 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b10',
  docvars    => 'armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VSUB_f cclass=VSUB_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_f_T2_D => {
  name       => 'Double-precision scalar',
  diagram    => 'ig0=0b11101110 o0=0b0 D:u=0bx o1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b10 size:u=0b11 N:u=0bx o2=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'size == 0b11',
  docvars    => 'armarmheading=T2 fpdatasize=doubleprec instr-class=fpsimd isa=T32 mnemonic=VSUB',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=fpdp iclass=fp_3r page=VSUB_f cclass=VSUB_f_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'FLOAT|ARITHMETIC',
};

ENCODING VSUB_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_i cclass=VSUB_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUB simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSUB_i cclass=VSUB_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_i cclass=VSUB_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUB_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUB simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSUB_i cclass=VSUB_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBHN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBHN cclass=VSUBHN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBHN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0110 N:u=0bx ig2=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBHN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBHN cclass=VSUBHN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBL_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBL cclass=VSUBL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBL_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b0 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBL',
  docvars2   => 'shape=long',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBL cclass=VSUBL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBW_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b1 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUBW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_diff page=VSUBW cclass=VSUBW_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUBW_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11111 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b001 op:u=0b1 N:u=0bx ig3=0b0 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  cnsts      => 'size!=0b11',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUBW',
  docvars2   => 'shape=wide',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_3diff page=VSUBW cclass=VSUBW_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|ARITHMETIC',
};

ENCODING VSUDOT_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSUDOT_s cclass=VSUDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VSUDOT_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VSUDOT_s cclass=VSUDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VSUDOT_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSUDOT_s cclass=VSUDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VSUDOT_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VSUDOT_s cclass=VSUDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VSWP_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSWP simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSWP cclass=VSWP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VSWP_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VSWP simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSWP cclass=VSWP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VSWP_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSWP simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSWP cclass=VSWP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VSWP_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0b00 opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0000 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VSWP simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSWP cclass=VSWP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
};

ENCODING VTBL_A1 => {
  name       => 'VTBL',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTBL',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_tbl page=VTBL cclass=VTBL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING VTBX_A1 => {
  name       => 'VTBX',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTBX',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_tbl page=VTBL cclass=VTBL_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING VTBL_T1 => {
  name       => 'VTBL',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTBL',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_tbl page=VTBL cclass=VTBL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING VTBX_T1 => {
  name       => 'VTBX',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b10 len:u=0bxx N:u=0bx op:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'op == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTBX',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_tbl page=VTBL cclass=VTBL_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|TABLE_LOOKUP',
};

ENCODING VTRN_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VTRN cclass=VTRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VTRN_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VTRN cclass=VTRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VTRN_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VTRN cclass=VTRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VTRN_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VTRN cclass=VTRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VTST_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTST simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VTST cclass=VTST_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VTST_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTST simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VTST cclass=VTST_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VTST_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTST simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VTST cclass=VTST_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VTST_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1000 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTST simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VTST cclass=VTST_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
};

ENCODING VUDOT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUDOT cclass=VUDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUDOT cclass=VUDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUDOT cclass=VUDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b00 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUDOT cclass=VUDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUDOT_s cclass=VUDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUDOT_s cclass=VUDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUDOT_s cclass=VUDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUDOT_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUDOT_s cclass=VUDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT|UNSIGNED',
};

ENCODING VUMMLA_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUMMLA cclass=VUMMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING VUMMLA_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b0 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b1 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUMMLA cclass=VUMMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC|UNSIGNED',
};

ENCODING VUSDOT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUSDOT cclass=VUSDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUSDOT cclass=VUSDOT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUSDOT cclass=VUSDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111110 op1=0b01 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUSDOT cclass=VUSDOT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_s_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUSDOT_s cclass=VUSDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_s_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd2reg_scalarext page=VUSDOT_s cclass=VUSDOT_s_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_s_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b0 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUSDOT_s cclass=VUSDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSDOT_s_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b11111110 op1=0b1 D:u=0bx op2=0b00 Vn:u=0bxxxx Vd:u=0bxxxx ig1=0b1 op3=0b1 ig2=0b0 op4=0b1 N:u=0bx Q:u=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSDOT simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_2r_scext page=VUSDOT_s cclass=VUSDOT_s_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DOT_PRODUCT',
};

ENCODING VUSMMLA_A1_Q => {
  name       => 'A1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cops_as iclass=simd3reg_sameext page=VUSMMLA cclass=VUSMMLA_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VUSMMLA_T1_Q => {
  name       => 'T1',
  diagram    => 'ig0=0b1111110 ig1=0b0 B:u=0b1 D:u=0bx op2=0b10 Vn:u=0bxxxx Vd:u=0bxxxx ig2=0b1 op3=0b1 ig3=0b0 op4=0b0 N:u=0bx Q=0b1 M:u=0bx U:u=0b0 Vm:u=0bxxxx',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUSMMLA simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=cpaf iclass=simd_3sameext page=VUSMMLA cclass=VUSMMLA_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|CARITHMETIC',
};

ENCODING VUZP_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUZP simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VUZP cclass=VUZP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VUZP_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VUZP simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VUZP cclass=VUZP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VUZP_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUZP simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VUZP cclass=VUZP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VUZP_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0010 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VUZP simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VUZP cclass=VUZP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VZIP_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VZIP simdvectorsize=double',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VZIP cclass=VZIP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VZIP_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VZIP simdvectorsize=quad',
  metadata   => 'alias=0 isa=ARM isaform=A32 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VZIP cclass=VZIP_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VZIP_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VZIP simdvectorsize=double',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VZIP cclass=VZIP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VZIP_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0011 Q:u=0b1 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  docvars    => 'armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VZIP simdvectorsize=quad',
  metadata   => 'alias=0 isa=THUMB isaform=T32 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VZIP cclass=VZIP_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
};

ENCODING VACLE_VACGE_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLE armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VACGE_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLE_VACGE cclass=VACLE_VACGE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VACLE_VACGE_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLE armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VACGE_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLE_VACGE cclass=VACLE_VACGE_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VACLE_VACGE_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLE armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VACGE_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLE_VACGE cclass=VACLE_VACGE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VACLE_VACGE_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLE armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VACGE_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLE_VACGE cclass=VACLE_VACGE_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VACLT_VACGT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VACGT_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLT_VACGT cclass=VACLT_VACGT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VACLT_VACGT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VACGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VACGT_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VACLT_VACGT cclass=VACLT_VACGT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VACLT_VACGT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VACGT_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLT_VACGT cclass=VACLT_VACGT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VACLT_VACGT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx op:u=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VACLT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VACGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VACGT_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VACLT_VACGT cclass=VACLT_VACGT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VACGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VAND_VBIC_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VBIC_i_A1_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I16 <Dd>, #~<imm>',
};

ENCODING VAND_VBIC_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VBIC_i_A1_Q alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I16 <Qd>, #~<imm>',
};

ENCODING VAND_VBIC_i_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VBIC_i_A2_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I32 <Dd>, #~<imm>',
};

ENCODING VAND_VBIC_i_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VBIC_i_A2_Q alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I32 <Qd>, #~<imm>',
};

ENCODING VAND_VBIC_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VBIC_i_T1_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I16 <Dd>, #~<imm>',
};

ENCODING VAND_VBIC_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VBIC_i_T1_Q alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I16 <Qd>, #~<imm>',
};

ENCODING VAND_VBIC_i_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VBIC_i_T2_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I32 <Dd>, #~<imm>',
};

ENCODING VAND_VBIC_i_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b1 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VAND armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VBIC simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VBIC_i_T2_Q alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VAND_VBIC_i cclass=VAND_VBIC_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VBIC{<c>}{<q>}.I32 <Qd>, #~<imm>',
};

ENCODING VCLE_VCGE_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGE_r_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLE_VCGE_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGE_r_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLE_VCGE_r_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGE_r_A2_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLE_VCGE_r_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGE_r_A2_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLE_VCGE_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGE_r_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLE_VCGE_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGE_r_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLE_VCGE_r_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGE_r_T2_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLE_VCGE_r_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b0 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLE armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGE simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGE_r_T2_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLE_VCGE_r cclass=VCLE_VCGE_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGE{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLT_VCGT_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGT_r_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLT_VCGT_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U:u=0bx ig1=0b0 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGT_r_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLT_VCGT_r_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGT_r_A2_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLT_VCGT_r_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b1 ig1=0b0 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VCGT_r_A2_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLT_VCGT_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGT_r_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLT_VCGT_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U:u=0bx ig1=0b11110 D:u=0bx size:u=0bxx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0011 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGT_r_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VCLT_VCGT_r_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b0 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGT_r_T2_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Dd>, <Dm>, <Dn>',
};

ENCODING VCLT_VCGT_r_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b1 ig1=0b11110 D:u=0bx ig2=0b1 sz:u=0bx Vn:u=0bxxxx Vd:u=0bxxxx opc=0b1110 N:u=0bx Q:u=0b1 M:u=0bx o1=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VCLT armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VCGT simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VCGT_r_T2_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VCLT_VCGT_r cclass=VCLT_VCGT_r_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  categories => 'ADVSIMD|COMPARISON',
  eq_to      => 'VCGT{<c>}{<q>}.<dt> <Qd>, <Qm>, <Qn>',
};

ENCODING VEXT_VEXT_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VEXT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VEXT_A1_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT_VEXT cclass=VEXT_VEXT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
  eq_to      => 'VEXT{<c>}{<q>}.8 {<Dd>,} <Dn>, <Dm>, #<imm*(size/8)>',
};

ENCODING VEXT_VEXT_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111100101 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VEXT armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VEXT simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VEXT_A1_Q alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd3reg_ext page=VEXT_VEXT cclass=VEXT_VEXT_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
  eq_to      => 'VEXT{<c>}{<q>}.8 {<Qd>,} <Qn>, <Qm>, #<imm*(size/8)>',
};

ENCODING VEXT_VEXT_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b0 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VEXT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VEXT_T1_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT_VEXT cclass=VEXT_VEXT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
  eq_to      => 'VEXT{<c>}{<q>}.8 {<Dd>,} <Dn>, <Dm>, #<imm*(size/8)>',
};

ENCODING VEXT_VEXT_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111011111 D:u=0bx ig1=0b11 Vn:u=0bxxxx Vd:u=0bxxxx imm4:u=0bxxxx N:u=0bx Q:u=0b1 M:u=0bx ig2=0b0 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VEXT armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VEXT simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VEXT_T1_Q alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_ext page=VEXT_VEXT cclass=VEXT_VEXT_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|EXTRACTION',
  eq_to      => 'VEXT{<c>}{<q>}.8 {<Qd>,} <Qn>, <Qm>, #<imm*(size/8)>',
};

ENCODING VMOV_VORR_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'N:Vn == M:Vm',
  docvars    => 'alias_mnemonic=VMOV armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMOV_VORR_r cclass=VMOV_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VMOV_VORR_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'N:Vn == M:Vm',
  docvars    => 'alias_mnemonic=VMOV armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VMOV_VORR_r cclass=VMOV_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VMOV_VORR_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'N:Vn == M:Vm',
  docvars    => 'alias_mnemonic=VMOV armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMOV_VORR_r cclass=VMOV_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VMOV_VORR_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'N:Vn == M:Vm',
  docvars    => 'alias_mnemonic=VMOV armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VMOV_VORR_r cclass=VMOV_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|DATA_TRANSFER',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VORN_VORR_i_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_i_A1_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i cclass=VORN_VORR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I16 <Dd>, #~<imm>',
};

ENCODING VORN_VORR_i_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_i_A1_Q alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i cclass=VORN_VORR_i_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I16 <Qd>, #~<imm>',
};

ENCODING VORN_VORR_i_A2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_i_A2_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i cclass=VORN_VORR_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I32 <Dd>, #~<imm>',
};

ENCODING VORN_VORR_i_A2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 i:u=0bx ig1=0b1 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=A2 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_i_A2_Q alphaindex=FPSIMD',
  tags       => 'group=a_simd_12reg iclass=simd1reg_imm page=VORN_VORR_i cclass=VORN_VORR_i_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I32 <Qd>, #~<imm>',
};

ENCODING VORN_VORR_i_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_i_T1_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i cclass=VORN_VORR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I16 <Dd>, #~<imm>',
};

ENCODING VORN_VORR_i_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b0xx1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_i_T1_Q alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i cclass=VORN_VORR_i_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I16 <Qd>, #~<imm>',
};

ENCODING VORN_VORR_i_T2_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b0 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_i_T2_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i cclass=VORN_VORR_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I32 <Dd>, #~<imm>',
};

ENCODING VORN_VORR_i_T2_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 i:u=0bx ig1=0b11111 D:u=0bx ig2=0b000 imm3:u=0bxxx Vd:u=0bxxxx cmode:u=0b10x1 ig3=0b0 Q:u=0b1 op=0b0 ig4=0b1 imm4:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VORN armarmheading=T2 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_i_T2_Q alphaindex=FPSIMD',
  tags       => 'group=t_simd_12reg iclass=simd_1r_imm page=VORN_VORR_i cclass=VORN_VORR_i_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|BITWISE',
  eq_to      => 'VORR{<c>}{<q>}.I32 <Qd>, #~<imm>',
};

ENCODING VPOP_VLDM_A1 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPOP armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VLDM_A1 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPOP_VLDM cclass=VPOP_VLDM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VLDM{<c>}{<q>}{.<size>} SP!, <dreglist>',
};

ENCODING VPOP_VLDM_A2 => {
  name       => 'Increment After',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPOP armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VLDM_A2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPOP_VLDM cclass=VPOP_VLDM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VLDM{<c>}{<q>}{.<size>} SP!, <sreglist>',
};

ENCODING VPOP_VLDM_T1 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPOP armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VLDM_T1 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPOP_VLDM cclass=VPOP_VLDM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VLDM{<c>}{<q>}{.<size>} SP!, <dreglist>',
};

ENCODING VPOP_VLDM_T2 => {
  name       => 'Increment After',
  diagram    => 'ig0=0b1110110 P:u=0b0 U:u=0b1 D:u=0bx W:u=0b1 L=0b1 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPOP armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=inc-after mnemonic=VLDM',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VLDM_T2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPOP_VLDM cclass=VPOP_VLDM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VLDM{<c>}{<q>}{.<size>} SP!, <sreglist>',
};

ENCODING VPUSH_VSTMDB_A1 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPUSH armarmheading=A1 fpdatasize=doubleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VSTMDB_A1 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPUSH_VSTM cclass=VPUSH_VSTM_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VSTMDB{<c>}{<q>}{.<size>} SP!, <dreglist>',
};

ENCODING VPUSH_VSTMDB_A2 => {
  name       => 'Decrement Before',
  diagram    => 'cond:u=0bxxxx ig0=0b110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  cnsts      => 'cond!=0b1111',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPUSH armarmheading=A2 fpdatasize=singleprec instr-class=fpsimd isa=A32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VSTMDB_A2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=ldstsimdfp page=VPUSH_VSTM cclass=VPUSH_VSTM_a2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VSTMDB{<c>}{<q>}{.<size>} SP!, <sreglist>',
};

ENCODING VPUSH_VSTMDB_T1 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b11 imm8:u=0bxxxxxxx0',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPUSH armarmheading=T1 fpdatasize=doubleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VSTMDB_T1 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPUSH_VSTM cclass=VPUSH_VSTM_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VSTMDB{<c>}{<q>}{.<size>} SP!, <dreglist>',
};

ENCODING VPUSH_VSTMDB_T2 => {
  name       => 'Decrement Before',
  diagram    => 'ig0=0b1110110 P:u=0b1 U:u=0b0 D:u=0bx W:u=0b1 L=0b0 Rn:u=0b1101 Vd:u=0bxxxx ig1=0b10 size=0b10 imm8:u=0bxxxxxxxx',
  aliascond  => 'Unconditionally',
  docvars    => 'alias_mnemonic=VPUSH armarmheading=T2 fpdatasize=singleprec instr-class=fpsimd isa=T32 ldmstm-mode=dec-before mnemonic=VSTMDB',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VSTMDB_T2 alphaindex=FPSIMD',
  tags       => 'group=simdldst_mov64 iclass=simdfp_ldst page=VPUSH_VSTM cclass=VPUSH_VSTM_t2',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
  categories => 'FPSIMD|STACK_TRANSFER',
  eq_to      => 'VSTMDB{<c>}{<q>}{.<size>} SP!, <sreglist>',
};

ENCODING VQRSHRN_VQMOVN_A1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQRSHRN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VQMOVN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQRSHRN_VQMOVN cclass=VQRSHRN_VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
  eq_to      => 'VQMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQRSHRN_VQMOVN_T1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQRSHRN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VQMOVN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQRSHRN_VQMOVN cclass=VQRSHRN_VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
  eq_to      => 'VQMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQRSHRUN_VQMOVUN_A1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQRSHRUN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VQMOVUN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQRSHRUN_VQMOVN cclass=VQRSHRUN_VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
  eq_to      => 'VQMOVUN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQRSHRUN_VQMOVUN_T1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQRSHRUN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'rounding=1 saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VQMOVUN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQRSHRUN_VQMOVN cclass=VQRSHRUN_VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING|SATURATING',
  eq_to      => 'VQMOVUN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQSHRN_VQMOVN_A1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQSHRN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VQMOVN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQSHRN_VQMOVN cclass=VQSHRN_VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
  eq_to      => 'VQMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQSHRN_VQMOVN_T1 => {
  name       => 'Signed result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b1x M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQSHRN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVN result-type=signed-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VQMOVN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQSHRN_VQMOVN cclass=VQSHRN_VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
  eq_to      => 'VQMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQSHRUN_VQMOVUN_A1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQSHRUN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VQMOVUN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VQSHRUN_VQMOVN cclass=VQSHRUN_VQMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
  eq_to      => 'VQMOVUN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VQSHRUN_VQMOVUN_T1 => {
  name       => 'Unsigned result',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 ig3=0b010 op:u=0b01 M:u=0bx ig4=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VQSHRUN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VQMOVUN result-type=unsigned-result',
  docvars2   => 'saturating=1 shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VQMOVUN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VQSHRUN_VQMOVN cclass=VQSHRUN_VQMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|SATURATING',
  eq_to      => 'VQMOVUN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VRSHR_VORR_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHR armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHR_VORR_r cclass=VRSHR_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VRSHR_VORR_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHR armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VRSHR_VORR_r cclass=VRSHR_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VRSHR_VORR_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHR armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  docvars2   => 'rounding=1',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHR_VORR_r cclass=VRSHR_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VRSHR_VORR_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHR armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  docvars2   => 'rounding=1',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VRSHR_VORR_r cclass=VRSHR_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VRSHRN_VMOVN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHRN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VMOVN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VRSHRN_VMOVN cclass=VRSHRN_VMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VRSHRN_VMOVN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VRSHRN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  docvars2   => 'rounding=1 shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VMOVN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VRSHRN_VMOVN cclass=VRSHRN_VMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT|ROUNDING',
  eq_to      => 'VMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VSHR_VORR_r_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHR armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_D alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHR_VORR_r cclass=VSHR_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VSHR_VORR_r_A1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b1111001 U=0b0 ig1=0b0 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHR armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VORR_r_A1_Q alphaindex=FPSIMD',
  tags       => 'group=advsimddp iclass=simd3reg_same page=VSHR_VORR_r cclass=VSHR_VORR_r_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VSHR_VORR_r_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b0 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b0',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHR armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_D alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHR_VORR_r cclass=VSHR_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Dd>, <Dm>, <Dm>',
};

ENCODING VSHR_VORR_r_T1_Q => {
  name       => '128-bit SIMD vector',
  diagram    => 'ig0=0b111 U=0b0 ig1=0b11110 D:u=0bx size=0b10 Vn:u=0bxxxx Vd:u=0bxxxx opc=0b0001 N:u=0bx Q:u=0b1 M:u=0bx o1=0b1 Vm:u=0bxxxx',
  bitdiffs   => 'Q == 0b1',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHR armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VORR simdvectorsize=quad',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VORR_r_T1_Q alphaindex=FPSIMD',
  tags       => 'group=simddp iclass=simd_3same page=VSHR_VORR_r cclass=VSHR_VORR_r_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VORR{<c>}{<q>}{.<dt>} <Qd>, <Qm>, <Qm>',
};

ENCODING VSHRN_VMOVN_A1 => {
  name       => 'A1',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHRN armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VMOVN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VMOVN_A1 alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VSHRN_VMOVN cclass=VSHRN_VMOVN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VSHRN_VMOVN_T1 => {
  name       => 'T1',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0100 Q=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VSHRN armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VMOVN',
  docvars2   => 'shape=narrow',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VMOVN_T1 alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VSHRN_VMOVN cclass=VSHRN_VMOVN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|SHIFT_RIGHT',
  eq_to      => 'VMOVN{<c>}{<q>}.<dt> <Dd>, <Qm>',
};

ENCODING VUZP_VTRN_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VUZP armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VTRN_A1_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VUZP_VTRN cclass=VUZP_VTRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
  eq_to      => 'VTRN{<c>}{<q>}.32 <Dd>, <Dm>',
};

ENCODING VUZP_VTRN_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VUZP armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VTRN_T1_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VUZP_VTRN cclass=VUZP_VTRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
  eq_to      => 'VTRN{<c>}{<q>}.32 <Dd>, <Dm>',
};

ENCODING VZIP_VTRN_A1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111100111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VZIP armarmheading=A1 instr-class=fpsimd isa=A32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 isa=ARM isaform=A32 aliasof=VTRN_A1_D alphaindex=FPSIMD',
  tags       => 'group=a_simd_mulreg iclass=simd2reg_misc page=VZIP_VTRN cclass=VZIP_VTRN_a1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
  eq_to      => 'VTRN{<c>}{<q>}.32 <Dd>, <Dm>',
};

ENCODING VZIP_VTRN_T1_D => {
  name       => '64-bit SIMD vector',
  diagram    => 'ig0=0b111111111 D:u=0bx ig1=0b11 size:u=0bxx opc1=0b10 Vd:u=0bxxxx ig2=0b0 opc2=0b0001 Q:u=0b0 M:u=0bx ig3=0b0 Vm:u=0bxxxx',
  aliascond  => 'Never',
  docvars    => 'alias_mnemonic=VZIP armarmheading=T1 instr-class=fpsimd isa=T32 mnemonic=VTRN simdvectorsize=double',
  metadata   => 'alias=1 isa=THUMB isaform=T32 aliasof=VTRN_T1_D alphaindex=FPSIMD',
  tags       => 'group=t_simd_mulreg iclass=simd_2r_misc page=VZIP_VTRN cclass=VZIP_VTRN_t1',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  categories => 'ADVSIMD|MISC',
  eq_to      => 'VTRN{<c>}{<q>}.32 <Dd>, <Dm>',
};


1;

