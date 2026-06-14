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

CLASS ABS_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=ABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=ABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS ABS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=ABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS ADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=ADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS ADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS ADDHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDHN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=ADDHN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS ADDP_asisdpair_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b11000 opcode=0b11011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=ADDP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS ADDP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ADDP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS ADDV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b11011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ADDV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=ADDV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS AESD_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0010 D:u=0b1 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveAESExt()',
};

CLASS AESE_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0010 D:u=0b0 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveAESExt()',
};

CLASS AESIMC_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0011 D:u=0b1 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESIMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESIMC_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveAESExt()',
};

CLASS AESMC_B_cryptoaes => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01001110 size=0b00 ig1=0b10100 ig2=0b0011 D:u=0b0 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=AESMC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptoaes page=AESMC_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveAESExt()',
};

CLASS AND_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b00 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=AND',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=AND_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS BCAX_VVV16_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b01 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=BCAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=BCAX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA3Ext()',
};

CLASS BFCVT_BS_floatdp1 => {
  name       => 'Single-precision to BFloat16',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ptype=0b01 ig2=0b1 opcode=0b000110 ig3=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'convert-type=single-to-bf16 instr-class=float isa=A64 mnemonic=BFCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=BFCVT_float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFCVTN_asimdmisc_4S => {
  name       => 'Vector single-precision and BFloat16',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size=0b10 ig2=0b10000 opcode=0b10110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-bf16 advsimd-type=simd datatype=single-and-bf16 instr-class=advsimd isa=A64 mnemonic=BFCVTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=BFCVTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFDOT_asimdelem_E => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b01 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=BFDOT_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b01 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFDOT_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFMLAL_asimdelem_F => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b11 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=BFMLAL reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=BFMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFMLAL_asimdsame2_F_ => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b11 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BFMMLA_asimdsame2_E => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U=0b1 ig1=0b01110 size=0b01 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b1101 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BFMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=BFMMLA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveBF16Ext()',
};

CLASS BIC_advsimd_imm_shifted_immediate => {
  name       => 'Shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxx1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'asimdimm-immtype=shifted-immediate instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=BIC_advsimd_imm xid=shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS BIC_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b01 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIC',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIC_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS BIF_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b11 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIF_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS BIT_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BIT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS BSL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b01 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=BSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=BSL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS CLS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CLS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS CLZ_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CLZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CLZ_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS CMEQ_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMEQ_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMEQ_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMEQ_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMGE_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMGE_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMGE_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMGE_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMGT_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMGT_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMGT_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMGT_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMHI_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMHI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMHI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMHI_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMHI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMHI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMHS_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMHS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMHS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMHS_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0011 eq:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMHS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMHS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMLE_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMLE_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0100 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMLT_asisdmisc_Z => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=CMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMLT_asimdmisc_Z => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CMTST_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=CMTST',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=CMTST_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS CMTST_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CMTST',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=CMTST_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS CNT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=CNT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=CNT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 0',
};

CLASS DUP_advsimd_elt_DUP_asisdone_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 op=0b0 ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=scalar-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdone page=DUP_advsimd_elt xid=DUP_asisdone_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS DUP_asimdins_DV_v => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=vector-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=DUP_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '((LowestSetBit(imm5)) > 3) || ((LowestSetBit(imm5)) == 3 && Q == 0)',
};

CLASS DUP_asimdins_DR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=DUP_advsimd_gen',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '((LowestSetBit(imm5)) > 3) || ((LowestSetBit(imm5)) == 3 && Q == 0)',
};

CLASS EOR3_VVV16_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b00 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EOR3',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=EOR3_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA3Ext()',
};

CLASS EOR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 opc2:u=0b00 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=EOR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=EOR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS EXT_asimdext_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b101110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 imm4:u=0bxxxx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=EXT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdext page=EXT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'Q == 0 && imm4<3> == 1',
};

CLASS FABD_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FABD_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FABD_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FABD reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FABD_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABD reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FABS_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FABS_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FABS_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b0000 opc:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FABS_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FACGE_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FACGE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FACGE_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FACGE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FACGE_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGE reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FACGE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FACGE_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGE reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FACGE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FACGT_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FACGT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FACGT_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FACGT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FACGT_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FACGT reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FACGT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FACGT_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FACGT reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FACGT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FADD_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FADD reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FADD_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADD reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FADD_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FADD_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FADDP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0b0 ig3=0b11000 opcode=0b01101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FADDP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FADDP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b11000 opcode=0b01101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADDP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FADDP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FADDP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FADDP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FADDP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FADDP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FADDP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FADDP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCADD_asimdsame2_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b11 rot:u=0bx ig5=0b0 ig6=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=FCADD_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (size == 0) || (Q == 0 && size == 3) || (!HaveFP16Ext() && (8 << UInt(size)) == 16)',
};

CLASS FCCMP_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b0 nzcv:u=0bxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMP_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCCMPE_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b01 Rn:u=0bxxxxx op:u=0b1 nzcv:u=0bxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatccmp page=FCCMPE_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMEQ_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMEQ_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMEQ_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMEQ_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMEQ_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMEQ_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMEQ_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMEQ_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMEQ_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMEQ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMEQ_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMGE_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGE_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMGE_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGE_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMGE_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMGE_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGE_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMGE_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGE_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMGE_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMGT_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FCMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGT_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FCMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMGT_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 ig4=0b10 ac:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FCMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGT_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 E:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx ig3=0b1110 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FCMGT_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMGT_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGT_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMGT_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMGT_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMGT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMGT_advsimd_zero',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMLA_advsimd_elt_2reg_element => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 rot:u=0bxx ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FCMLA_advsimd_elt xid=2reg_element',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (size == 0 || size == 3) || (!HaveFP16Ext() && (8 << UInt(size)) == 16) || (size == 2 && (L == 1 || Q == 0)) || (size == 1 && H == 1 && Q == 0)',
};

CLASS FCMLA_asimdsame2_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b10 rot:u=0bxx ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=FCMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFCADDExt()) || (size == 0) || (Q == 0 && size == 3) || (!HaveFP16Ext() && (8 << UInt(size)) == 16)',
};

CLASS FCMLE_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMLE_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMLE_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 ig3=0b0110 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMLE_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 ig4=0b0110 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMLE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMLT_asisdmiscfp16_FZ => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b01110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMLT_asisdmisc_FZ => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMLT_asimdmiscfp16_FZ => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01110 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCMLT_asimdmisc_FZ => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCMLT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCMLT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCMP_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b0x ig4=0b000',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMP_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCMPE_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op=0b00 ig3=0b1000 Rn:u=0bxxxxx opc:u=0b1x ig4=0b000',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCMPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatcmp page=FCMPE_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCSEL_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx cond:u=0bxxxx ig3=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCSEL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatsel page=FCSEL_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FCVT_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b0001 opc:u=0bxx ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FCVT_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'ftype == opc',
};

CLASS FCVTAS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTAS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTAS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTAS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTAS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTAS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTAS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTAS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTAS_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b100 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTAS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAS_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTAU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTAU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTAU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTAU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTAU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTAU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTAU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTAU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTAU_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b101 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTAU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTAU_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTL_asimdmisc_L => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTMS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTMS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTMS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTMS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTMS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTMS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTMS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTMS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTMS_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b10 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTMS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMS_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTMU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTMU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTMU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTMU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTMU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTMU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTMU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTMU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTMU_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b10 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTMU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTMU_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTN_asimdmisc_N => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTNS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTNS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTNS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTNS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTNS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTNS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTNS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTNS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTNS_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTNS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNS_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTNU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTNU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTNU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTNU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTNU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTNU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTNU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTNU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTNU_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTNU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTNU_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTPS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTPS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTPS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTPS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTPS_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b01 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPS_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTPU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTPU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTPU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTPU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTPU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTPU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTPU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTPU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTPU_float_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b01 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTPU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTPU_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTXN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTXN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTXN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 0',
};

CLASS FCVTXN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b10110 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTXN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTXN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz == 0',
};

CLASS FCVTZS_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=FCVTZS_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())',
};

CLASS FCVTZS_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=FCVTZS_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())) || (immh<3>:Q == 2)',
};

CLASS FCVTZS_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTZS_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTZS_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTZS_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTZS_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTZS_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTZS_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTZS_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTZS_float_fix_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b0 rmode:u=0b11 opcode:u=0b000 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZS_float_fix xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTZS_float_int_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b11 opcode:u=0b000 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZS_float_int xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTZU_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=FCVTZU_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())',
};

CLASS FCVTZU_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11111 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=FCVTZU_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())) || (immh<3>:Q == 2)',
};

CLASS FCVTZU_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FCVTZU_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTZU_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FCVTZU_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTZU_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FCVTZU_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FCVTZU_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1101 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FCVTZU_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FCVTZU_float_fix_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b0 rmode:u=0b11 opcode:u=0b001 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=FCVTZU_float_fix xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FCVTZU_float_int_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b11 opcode:u=0b001 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FCVTZU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FCVTZU_float_int xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FDIV_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FDIV reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FDIV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FDIV_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FDIV reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FDIV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FDIV_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b0001 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FDIV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FDIV_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FJCVTZS_32D_float2int => {
  name       => 'Double-precision to 32-bit',
  diagram    => 'sf:u=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b01 ig2=0b1 rmode:u=0b11 opcode:u=0b110 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'convert-type=double-to-32 instr-class=float isa=A64 mnemonic=FJCVTZS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FJCVTZS',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFJCVTZSExt()',
};

CLASS FMADD_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0bxx o1:u=0b0 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMADD_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMAX_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAX reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAX_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAX reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMAX_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b00 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAX_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMAXNM_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNM reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXNM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXNM_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNM reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXNM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMAXNM_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b10 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMAXNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMAXNM_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMAXNMP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b0 sz:u=0b0 ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXNMP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXNMP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXNMP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMAXNMP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXNMP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXNMP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXNMP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMAXNMV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b0 ig2=0b0 ig3=0b11000 opcode=0b01100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXNMV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXNMV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXNMV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q != 1',
};

CLASS FMAXP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b0 sz:u=0b0 ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMAXP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMAXP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMAXP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMAXP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMAXV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b0 ig2=0b0 ig3=0b11000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMAXV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b0 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMAXV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q != 1',
};

CLASS FMIN_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMIN reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMIN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMIN_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMIN reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMIN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMIN_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b01 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMIN_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMINNM_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNM reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINNM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINNM_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNM reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINNM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMINNM_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b11 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMINNM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMINNM_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMINNMP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b1 sz:u=0b0 ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINNMP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINNMP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINNMP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMINNMP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 Op3=0b000 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINNMP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINNMP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINNMP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMINNMV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b1 ig2=0b0 ig3=0b11000 opcode=0b01100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINNMV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINNMV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINNMV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINNMV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q != 1',
};

CLASS FMINP_asisdpair_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 o1:u=0b1 sz:u=0b0 ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINP_asisdpair_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdpair page=FMINP_advsimd_pair',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMINP_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b110 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMINP reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMINP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINP_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINP reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMINP_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMINV_asimdall_only_H => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 o1:u=0b1 ig2=0b0 ig3=0b11000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=half instr-class=advsimd isa=A64 mnemonic=FMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMINV_asimdall_only_SD => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 o1:u=0b1 sz:u=0bx ig2=0b11000 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=FMINV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q != 1',
};

CLASS FMLA_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLA_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLA_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b0 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLA_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMLA_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLA_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLA_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b0 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLA_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMLA_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a:u=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b001 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLA_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 op:u=0b0 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLA reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMLAL_asimdelem_LH => {
  name       => 'FMLAL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b0 S:u=0b0 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLAL2_asimdelem_LH => {
  name       => 'FMLAL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b1 S:u=0b0 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLAL2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLAL_asimdsame_F => {
  name       => 'FMLAL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 S:u=0b0 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLAL2_asimdsame_F => {
  name       => 'FMLAL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 S:u=0b0 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLAL2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLS_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLS_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLS_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b1 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMLS_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMLS_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b01 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLS_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLS_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx ig3=0b0 o2:u=0b1 ig4=0b01 H:u=0bx ig5=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLS_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMLS_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a:u=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b001 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMLS_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMLS_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 op:u=0b1 sz:u=0bx ig2=0b1 Rm:u=0bxxxxx opcode=0b11001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMLS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLS_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMLSL_asimdelem_LH => {
  name       => 'FMLSL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b0 S:u=0b1 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLSL2_asimdelem_LH => {
  name       => 'FMLSL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size[1]=0b1 sz:u=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode[3]=0b1 S:u=0b1 opcode[1:0]=0b00 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=FMLSL2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLSL_asimdsame_F => {
  name       => 'FMLSL',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 S:u=0b1 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMLSL2_asimdsame_F => {
  name       => 'FMLSL2',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 S:u=0b1 sz:u=0b0 ig2=0b1 Rm:u=0bxxxxx opcode[4]=0b1 opcode[3:0]=0b1001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=FMLSL2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveFP16MulNoRoundingToFP32Ext()) || (sz == 1)',
};

CLASS FMOV_asimdimm_H_h => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode=0b1111 o2=0b1 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'asimdimm-datatype=per-half instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=FMOV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS single_and_double => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0bx ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0b1111 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=FMOV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FMOV_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b0000 opc:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FMOV_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FMOV_float_gen_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b0x opcode:u=0b11x ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v3 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=FMOV_float_gen xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS imm8f => {
  name       => 'Floating point constant in 8 bits',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 imm8:u=0bxxxxxxxx ig3=0b100 imm5=0b00000 Rd:u=0bxxxxx',
  docvars    => 'immediate-type=imm8f instr-class=float isa=A64 mnemonic=FMOV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatimm page=FMOV_float_imm',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FMSUB_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0bxx o1:u=0b0 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FMSUB_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMUL_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMUL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMUL_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMUL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMUL_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMUL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMUL_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMUL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMUL_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMUL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMUL_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMUL reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMUL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FMUL_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op:u=0b0 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FMUL_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMULX_asisdelem_RH_H => {
  name       => 'Scalar, half-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-scalar-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMULX_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMULX_asisdelem_R_SD => {
  name       => 'Scalar, single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-reguse=2reg-scalar advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-scalar-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=FMULX_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMULX_asimdelem_RH_H => {
  name       => 'Vector, half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size=0b00 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-element-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMULX_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMULX_asimdelem_R_SD => {
  name       => 'Vector, single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 ig2=0b1 sz:u=0bx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1001 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=2reg-element-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=FMULX_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMULX_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FMULX_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMULX_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FMULX_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FMULX_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FMULX_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FMULX_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FMULX reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FMULX_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FNEG_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b01111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FNEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FNEG_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b01111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FNEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FNEG_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b0000 opc:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FNEG_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS FNMADD_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0bxx o1:u=0b1 Rm:u=0bxxxxx o0:u=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMADD_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FNMSUB_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11111 ftype:u=0bxx o1:u=0b1 Rm:u=0bxxxxx o0:u=0b1 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp3 page=FNMSUB_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FNMUL_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx op:u=0b1 ig3=0b000 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FNMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FNMUL_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRECPE_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRECPE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRECPE_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRECPE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRECPE_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRECPE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRECPE_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRECPE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRECPS_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FRECPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRECPS_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FRECPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRECPS_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b0 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FRECPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRECPS_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FRECPS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRECPX_asisdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b11111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRECPX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRECPX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRECPX_asisdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRECPX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRECPX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINT32X_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32X',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT32X_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFrintExt()) || (sz:Q == 2)',
};

CLASS FRINT32X_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b0x ig2=0b1 ig3=0b0100 op:u=0b01 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT32X',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32X_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFrintExt()',
};

CLASS FRINT32Z_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b0 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT32Z_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFrintExt()) || (sz:Q == 2)',
};

CLASS FRINT32Z_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b0x ig2=0b1 ig3=0b0100 op:u=0b00 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT32Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT32Z_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFrintExt()',
};

CLASS FRINT64X_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64X',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT64X_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFrintExt()) || (sz:Q == 2)',
};

CLASS FRINT64X_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b0x ig2=0b1 ig3=0b0100 op:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT64X',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64X_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFrintExt()',
};

CLASS FRINT64Z_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 ig4=0b1111 op:u=0b1 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINT64Z_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(!HaveFrintExt()) || (sz:Q == 2)',
};

CLASS FRINT64Z_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0b0x ig2=0b1 ig3=0b0100 op:u=0b10 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINT64Z',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v5 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINT64Z_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFrintExt()',
};

CLASS FRINTA_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTA_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTA_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b100 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTA_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTI_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTI',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTI_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTI_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b111 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTI_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTM_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTM_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTM_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTM_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b010 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTM',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTM_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTN_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTN_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTN_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b000 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTN_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTP_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTP_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTP_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b001 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTP_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTX_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTX',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTX_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 o2:u=0b0 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTX_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b110 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTX_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRINTZ_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 ig2=0b111100 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRINTZ_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRINTZ_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 o2:u=0b1 sz:u=0bx ig2=0b10000 ig3=0b1100 o1:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRINTZ_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRINTZ_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b001 rmode:u=0b011 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FRINTZ',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FRINTZ_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRSQRTE_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=FRSQRTE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRSQRTE_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=FRSQRTE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRSQRTE_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FRSQRTE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRSQRTE_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FRSQRTE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FRSQRTS_asisdsamefp16_only => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTS',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsamefp16 page=FRSQRTS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRSQRTS_asisdsame_only => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=FRSQRTS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FRSQRTS_asimdsamefp16_only => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FRSQRTS reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FRSQRTS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FRSQRTS_asimdsame_only => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11111 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FRSQRTS reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FRSQRTS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FSQRT_asimdmiscfp16_R => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 a=0b1 ig2=0b111100 opcode=0b11111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=FSQRT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FSQRT_asimdmisc_R => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11111 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=FSQRT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FSQRT_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 ig3=0b0000 opc:u=0b11 ig4=0b10000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FSQRT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp1 page=FSQRT_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS FSUB_asimdsamefp16_only => {
  name       => 'Half-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b1 ig2=0b10 Rm:u=0bxxxxx ig3=0b00 opcode=0b010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-reguse=3reg-same advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=FSUB reguse-datatype=3reg-same-half',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsamefp16 page=FSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS FSUB_asimdsame_only => {
  name       => 'Single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b1 Rm:u=0bxxxxx opcode=0b11010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-reguse=3reg-same advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=FSUB reguse-datatype=3reg-same-single-and-double',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=FSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS FSUB_float_float => {
  name       => 'Floating-point',
  diagram    => 'M=0b0 ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b001 op:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=FSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=floatdp2 page=FSUB_float xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS INS_advsimd_elt_INS_asimdins_IV_v => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b1 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=INS_advsimd_elt xid=INS_asimdins_IV_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS INS_advsimd_gen_INS_asimdins_IR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=INS_advsimd_gen xid=INS_asimdins_IR_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS LD1_advsimd_mult_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0bxx1x size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD1_advsimd_mult xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD1_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0bxx1x size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD1_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD1_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD1_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD1R_asisdlso_R1 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD1R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD1R_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD1R_advsimd_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD1R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD1R_advsimd xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2_asisdlse_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD2_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-2reg mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD2_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2R_asisdlso_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD2R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD2R_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD2R_advsimd_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b110 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD2R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD2R_advsimd xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3_asisdlse_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD3_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-3reg mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD3_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3R_asisdlso_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b0 ig2=0b00000 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD3R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD3R_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD3R_advsimd_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b0 Rm:u=0bxxxxx opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD3R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD3R_advsimd xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4_asisdlse_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b1 ig2=0b000000 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=LD4_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b1 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 ldstruct-regcount=to-4reg mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=LD4_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4R_asisdlso_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b1 R:u=0b1 ig2=0b00000 opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=LD4R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=LD4R_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LD4R_advsimd_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b1 R:u=0b1 Rm:u=0bxxxxx opcode:u=0b111 S:u=0b0 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-org=to-all-lanes as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=LD4R',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=LD4R_advsimd xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LDNP_fpsimd_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b000 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=LDNP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=LDNP_fpsimd xid=signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LDP_fpsimd_post_indexed => {
  name       => 'Post-index',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b001 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=LDP_fpsimd xid=post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LDP_fpsimd_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b011 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=LDP_fpsimd xid=pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LDP_fpsimd_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b010 L:u=0b1 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=LDP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=LDP_fpsimd xid=signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS LDR_imm_fpsimd_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx1 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=LDR_imm_fpsimd xid=post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS LDR_imm_fpsimd_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx1 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=LDR_imm_fpsimd xid=pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS LDR_imm_fpsimd_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b01 opc:u=0bx1 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=LDR_imm_fpsimd xid=unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS LDR_lit_fpsimd_literal => {
  name       => 'Literal',
  diagram    => 'opc:u=0bxx ig0=0b011 V=0b1 ig1=0b00 imm19:u=0bxxxxxxxxxxxxxxxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=literal instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off19s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=loadlit page=LDR_lit_fpsimd xid=literal',
  exceptions => 'ADVSIMDFPACCESSTRAP|ALIGNMENT|DATAABORT|UNCATEGORIZED',
};

CLASS LDR_reg_fpsimd_fpsimd => {
  name       => 'SIMD&FP registers',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx1 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=LDR offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=LDR_reg_fpsimd xid=fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => '(UInt(opc<1>:size) > 4) || (option<1> == 0)',
};

CLASS LDUR_fpsimd_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx1 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd isa=A64 mnemonic=LDUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=LDUR_fpsimd xid=base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS MLA_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b00 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MLA_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MLA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS MLS_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b00 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MLS_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MLS_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MLS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MLS_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS MOVI_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0bx ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxxx o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=MOVI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MOVI_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MUL_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1000 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=MUL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MUL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=MUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MUL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(U == 1 && size != 0) || (size == 3)',
};

CLASS MVNI_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b1 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxxx o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=MVNI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=MVNI_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS NEG_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=NEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=NEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS NEG_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b01011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=NEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS NOT_advsimd_NOT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b00 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=NOT_advsimd xid=NOT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS ORN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b11 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ORN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS ORR_advsimd_imm_shifted_immediate => {
  name       => 'Shifted immediate',
  diagram    => 'ig0=0b0 Q:u=0bx op:u=0b0 ig1=0b0111100000 a:u=0bx b:u=0bx c:u=0bx cmode:u=0bxxx1 o2=0b0 ig2=0b1 d:u=0bx e:u=0bx f:u=0bx g:u=0bx h:u=0bx Rd:u=0bxxxxx',
  docvars    => 'asimdimm-immtype=shifted-immediate instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdimm page=ORR_advsimd_imm xid=shifted_immediate',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS ORR_advsimd_reg_ORR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=ORR_advsimd_reg xid=ORR_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS PMUL_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMUL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=PMUL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(U == 1 && size != 0) || (size == 3)',
};

CLASS PMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1110 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=PMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=PMULL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size == 1 || size == 2) || (size == 3 && !HaveBit128PMULLExt())',
};

CLASS RADDHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RADDHN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=RADDHN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS RAX1_VVV2_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=RAX1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=RAX1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA3Ext()',
};

CLASS RBIT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b01 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RBIT',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=RBIT_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS REV16_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b1 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV16',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV16_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'UInt((o0:U)) + UInt(size) >= 3',
};

CLASS REV32_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV32',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV32_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'UInt((o0:U)) + UInt(size) >= 3',
};

CLASS REV64_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b0000 o0:u=0b0 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=REV64',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=REV64_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'UInt((o0:U)) + UInt(size) >= 3',
};

CLASS RSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=RSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS RSUBHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=RSUBHN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=RSUBHN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SABA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SABA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SABAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SABAL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SABD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SABDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SABDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SABDL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SADALP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADALP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SADALP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SADDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SADDL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SADDLP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDLP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SADDLP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SADDLV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SADDLV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SADDLV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS SADDW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SADDW',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SADDW_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SCVTF_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SCVTF_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())',
};

CLASS SCVTF_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SCVTF_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())) || (immh<3>:Q == 2)',
};

CLASS SCVTF_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=SCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS SCVTF_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS SCVTF_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=SCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS SCVTF_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS SCVTF_float_fix_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b0 rmode:u=0b00 opcode:u=0b010 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=SCVTF_float_fix xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS SCVTF_float_int_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b010 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=SCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=SCVTF_float_int xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS SDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1110 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SDOT_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (size != 2)',
};

CLASS SDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SDOT_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (size != 2)',
};

CLASS SHA1C_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b000 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1C',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1C_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA1H_SS_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00000 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1H',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA1H_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA1M_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b010 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1M',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1M_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA1P_QSV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b001 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1P',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1P_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA1SU0_VVV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b011 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA1SU0_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA1SU1_VV_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00001 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA1SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA1SU1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA1Ext()',
};

CLASS SHA256H2_QQV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 ig3=0b10 P:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256H2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA256Ext()',
};

CLASS SHA256H_QQV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 ig3=0b10 P:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256H',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256H_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA256Ext()',
};

CLASS SHA256SU0_VV_cryptosha2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b10100 opcode=0b00010 ig2=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha2 page=SHA256SU0_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA256Ext()',
};

CLASS SHA256SU1_VVV_cryptosha3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b01011110 size=0b00 ig1=0b0 Rm:u=0bxxxxx ig2=0b0 opcode=0b110 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA256SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha3 page=SHA256SU1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA256Ext()',
};

CLASS SHA512H2_QQV_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512H2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA512Ext()',
};

CLASS SHA512H_QQV_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512H',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512H_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA512Ext()',
};

CLASS SHA512SU0_VV2_cryptosha512_2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110110000001000 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU0',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_2 page=SHA512SU0_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA512Ext()',
};

CLASS SHA512SU1_VVV2_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b0 ig2=0b00 opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SHA512SU1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SHA512SU1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA512Ext()',
};

CLASS SHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SHADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SHLL_asimdmisc_S => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHLL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SHLL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SHSUB_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00100 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SHSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SHSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SLI_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SLI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SLI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SLI_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01010 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SLI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SLI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SM3PARTW1_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM3PARTW1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3PARTW2_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3PARTW2',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM3PARTW2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3SS1_VVV4_crypto4 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b110011100 Op0=0b10 Rm:u=0bxxxxx ig1=0b0 Ra:u=0bxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3SS1',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto4 page=SM3SS1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3TT1A_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1A',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT1A_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3TT1B_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT1B',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT1B_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3TT2A_VVV4_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2A',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT2A_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM3TT2B_VVV_crypto3_imm2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110010 Rm:u=0bxxxxx ig1=0b10 imm2:u=0bxx opcode=0b11 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM3TT2B',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm2 page=SM3TT2B_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM3Ext()',
};

CLASS SM4E_VV4_cryptosha512_2 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110110000001000 opcode=0b01 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4E',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_2 page=SM4E_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM4Ext()',
};

CLASS SM4EKEY_VVV4_cryptosha512_3 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110011 Rm:u=0bxxxxx ig1=0b1 O=0b1 ig2=0b00 opcode=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SM4EKEY',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=cryptosha512_3 page=SM4EKEY_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSM4Ext()',
};

CLASS SMAX_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMAX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMAXP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMAXP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMAXV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b0 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SMAXV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS SMIN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMIN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMINP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SMINP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMINV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b1 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=SMINV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS SMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SMLAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SMLSL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SMMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS SMOV_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=SMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=SMOV_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1010 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SMULL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1100 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SMULL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SQABS_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQABS_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQABS',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQABS_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQDMLAL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMLAL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQDMLAL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQDMLSL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b11 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMLSL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQDMLSL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQDMULH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b0 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMULH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMULH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b0 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMULH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMULH_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQDMULH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || size == 0',
};

CLASS SQDMULH_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQDMULH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || size == 0',
};

CLASS SQDMULL_asisdelem_L => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1011 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQDMULL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1011 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQDMULL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQDMULL_asisddiff_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1101 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisddiff page=SQDMULL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQDMULL_asimddiff_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1101 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQDMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SQDMULL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 0 || size == 3',
};

CLASS SQNEG_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQNEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQNEG_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00111 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQNEG',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQNEG_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQRDMLAH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b0 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMLAH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveQRDMLAHExt()',
};

CLASS SQRDMLAH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b0 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMLAH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveQRDMLAHExt()',
};

CLASS SQRDMLAH_asisdsame2_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame2 page=SQRDMLAH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 3 || size == 0)',
};

CLASS SQRDMLAH_asimdsame2_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMLAH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SQRDMLAH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 3 || size == 0)',
};

CLASS SQRDMLSH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b1 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMLSH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveQRDMLAHExt()',
};

CLASS SQRDMLSH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b11 S:u=0b1 ig3=0b1 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMLSH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveQRDMLAHExt()',
};

CLASS SQRDMLSH_asisdsame2_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame2 page=SQRDMLSH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 3 || size == 0)',
};

CLASS SQRDMLSH_asimdsame2_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b000 S:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMLSH',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v1 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=SQRDMLSH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveQRDMLAHExt()) || (size == 3 || size == 0)',
};

CLASS SQRDMULH_asisdelem_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b0 ig1=0b11111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b1 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-scalar advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdelem page=SQRDMULH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQRDMULH_asimdelem_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b110 op:u=0b1 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SQRDMULH_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQRDMULH_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQRDMULH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || size == 0',
};

CLASS SQRDMULH_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10110 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRDMULH',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQRDMULH_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3 || size == 0',
};

CLASS SQRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS SQRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQRSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQRSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS SQRSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQRSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SQRSHRUN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQRSHRUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQRSHRUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS SQRSHRUN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQRSHRUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQRSHRUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SQSHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHL_advsimd_imm',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh == 0',
};

CLASS SQSHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHL_advsimd_imm',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SQSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQSHL_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS SQSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQSHL_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQSHLU_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHLU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHLU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh == 0',
};

CLASS SQSHLU_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHLU',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHLU_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SQSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS SQSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SQSHRUN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSHRUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SQSHRUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS SQSHRUN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1000 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSHRUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SQSHRUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SQSUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SQSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SQSUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SQSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SQXTN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQXTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQXTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SQXTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQXTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQXTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SQXTUN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SQXTUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SQXTUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SQXTUN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SQXTUN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SQXTUN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SRHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRHADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SRHADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SRI_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01000 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SRI_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b01000 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRI',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRI_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS SRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SRSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SRSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SRSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SRSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SRSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SRSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SRSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SRSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS SSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SSHLL_advsimd_SSHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHLL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSHLL_advsimd xid=SSHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS SSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=SSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS SSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS SSUBL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SSUBL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SSUBW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSUBW',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SSUBW_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS ST1_advsimd_mult_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0bxx1x size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST1_advsimd_mult xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST1_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0bxx1x size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST1_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST1_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST1_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST1_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST1_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST2_asisdlse_R2 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST2_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST2_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b1000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST2 ststruct-regcount=from-2reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST2_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST2_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST2_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST2_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0bxx0 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST2_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST3_asisdlse_R3 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST3_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST3_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0100 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST3 ststruct-regcount=from-3reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST3_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST3_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b0 ig2=0b00000 opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST3_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST3_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b0 Rm:u=0bxxxxx opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST3',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST3_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST4_asisdlse_R4 => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011000 L:u=0b0 ig2=0b000000 opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlse page=ST4_advsimd_mult',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST4_advsimd_mult_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011001 L:u=0b0 ig2=0b0 Rm:u=0bxxxxx opcode:u=0b0000 size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST4 ststruct-regcount=from-4reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsep page=ST4_advsimd_mult xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST4_advsimd_sngl_as_no_post_index => {
  name       => 'No offset',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011010 L:u=0b0 R:u=0b1 ig2=0b00000 opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-no-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlso page=ST4_advsimd_sngl xid=as_no_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS ST4_advsimd_sngl_as_post_index => {
  name       => 'Post-index',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b0011011 L:u=0b0 R:u=0b1 Rm:u=0bxxxxx opcode:u=0bxx1 S:u=0bx size:u=0bxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'as-structure-post-index=as-post-index instr-class=advsimd isa=A64 mnemonic=ST4',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=asisdlsop page=ST4_advsimd_sngl xid=as_post_index',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS STNP_fpsimd_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b000 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=STNP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstnapair_offs page=STNP_fpsimd xid=signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS STP_fpsimd_post_indexed => {
  name       => 'Post-index',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b001 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_post page=STP_fpsimd xid=post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS STP_fpsimd_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b011 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_pre page=STP_fpsimd xid=pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS STP_fpsimd_signed_scaled_offset => {
  name       => 'Signed offset',
  diagram    => 'opc:u=0bxx ig0=0b101 V=0b1 ig1=0b010 L:u=0b0 imm7:u=0bxxxxxxx Rt2:u=0bxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=signed-scaled-offset instr-class=fpsimd isa=A64 mnemonic=STP offset-type=off7s_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldstpair_off page=STP_fpsimd xid=signed_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
};

CLASS STR_imm_fpsimd_post_indexed => {
  name       => 'Post-index',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx0 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b01 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=post-indexed instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpost page=STR_imm_fpsimd xid=post_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS STR_imm_fpsimd_pre_indexed => {
  name       => 'Pre-index',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx0 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b11 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=pre-indexed instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_immpre page=STR_imm_fpsimd xid=pre_indexed',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS STR_imm_fpsimd_unsigned_scaled_offset => {
  name       => 'Unsigned offset',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b01 opc:u=0bx0 imm12:u=0bxxxxxxxxxxxx Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=unsigned-scaled-offset instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off12u_s',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_pos page=STR_imm_fpsimd xid=unsigned_scaled_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS STR_reg_fpsimd_fpsimd => {
  name       => 'SIMD&FP registers',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx0 ig2=0b1 Rm:u=0bxxxxx option:u=0bxxx S:u=0bx ig3=0b10 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'instr-class=fpsimd isa=A64 mnemonic=STR offset-type=off-reg',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_regoff page=STR_reg_fpsimd xid=fpsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => '(UInt(opc<1>:size) > 4) || (option<1> == 0)',
};

CLASS STUR_fpsimd_base_plus_offset => {
  name       => 'Unscaled offset',
  diagram    => 'size:u=0bxx ig0=0b111 V=0b1 ig1=0b00 opc:u=0bx0 ig2=0b0 imm9:u=0bxxxxxxxxx ig3=0b00 Rn:u=0bxxxxx Rt:u=0bxxxxx',
  docvars    => 'address-form=base-plus-offset instr-class=fpsimd isa=A64 mnemonic=STUR offset-type=off9s_u',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=ldst iclass=ldst_unscaled page=STUR_fpsimd xid=base_plus_offset',
  exceptions => 'ADVSIMDFPACCESSTRAP|DATAABORT|SPALIGNMENT|UNCATEGORIZED',
  undefined  => 'UInt(opc<1>:size) > 4',
};

CLASS SUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=SUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size != 3',
};

CLASS SUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b10000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=SUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS SUBHN_asimddiff_N => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUBHN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=SUBHN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS SUDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 US:u=0b0 ig2=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=SUDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=SUDOT_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS SUQADD_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b0 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=SUQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=SUQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SUQADD_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SUQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=SUQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS TBL_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0bxx op:u=0b0 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBL_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS TBX_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 op2=0b00 ig2=0b0 Rm:u=0bxxxxx ig3=0b0 len:u=0bxx op:u=0b1 ig4=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TBX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdtbl page=TBX_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS TRN1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=TRN1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS TRN2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=TRN2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=TRN2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UABA_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UABA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UABAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UABAL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UABD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0111 ac:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UABD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UABDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b01 op:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UABDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UABDL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UADALP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b1 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADALP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UADALP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UADDL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UADDL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UADDLP_asimdmisc_P => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 ig3=0b00 op:u=0b0 ig4=0b10 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDLP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UADDLP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UADDLV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UADDLV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UADDLV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS UADDW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b0 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UADDW',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UADDW_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UCVTF_asisdshf_C => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UCVTF_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())',
};

CLASS UCVTF_asimdshf_C => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b11100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UCVTF_advsimd_fix',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '(immh == 0b000x || (immh == 0b001x && !HaveFP16Ext())) || (immh<3>:Q == 2)',
};

CLASS UCVTF_asisdmiscfp16_R => {
  name       => 'Scalar half precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-half advsimd-type=sisd datatype=half instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmiscfp16 page=UCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS UCVTF_asisdmisc_R => {
  name       => 'Scalar single-precision and double-precision',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=sisd-single-and-double advsimd-type=sisd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=UCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS UCVTF_asimdmiscfp16_R => {
  name       => 'Vector half precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 a=0b0 ig2=0b111100 opcode=0b11101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-half advsimd-type=simd datatype=half instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmiscfp16 page=UCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => '!HaveFP16Ext()',
};

CLASS UCVTF_asimdmisc_R => {
  name       => 'Vector single-precision and double-precision',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 ig2=0b0 sz:u=0bx ig3=0b10000 opcode=0b11101 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-datatype=simd-single-and-double advsimd-type=simd datatype=single-and-double instr-class=advsimd isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UCVTF_advsimd_int',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
  undefined  => 'sz:Q == 2',
};

CLASS UCVTF_float_fix_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b0 rmode:u=0b00 opcode:u=0b011 scale:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2fix page=UCVTF_float_fix xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS UCVTF_float_int_float => {
  name       => 'Floating-point',
  diagram    => 'sf:u=0bx ig0=0b0 S=0b0 ig1=0b11110 ftype:u=0bxx ig2=0b1 rmode:u=0b00 opcode:u=0b011 ig3=0b000000 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=float isa=A64 mnemonic=UCVTF',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=float2int page=UCVTF_float_int xid=float',
  exceptions => 'ADVSIMDFPACCESSTRAP|FP|UNCATEGORIZED',
};

CLASS UDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1110 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UDOT_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (size != 2)',
};

CLASS UDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0010 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=UDOT_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(!HaveDOTPExt()) || (size != 2)',
};

CLASS UHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UHADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UHSUB_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00100 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UHSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UHSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMAX_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAX',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMAX_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMAXP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMAXP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMAXP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMAXV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b0 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMAXV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UMAXV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS UMIN_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b0110 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMIN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMIN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMINP_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b1010 o1:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMINP',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UMINP_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMINV_asimdall_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b11000 op:u=0b1 ig3=0b1010 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMINV',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdall page=UMINV_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(size:Q == 4) || (size == 3)',
};

CLASS UMLAL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b0 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMLAL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UMLAL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b0 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLAL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMLAL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMLSL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx ig2=0b0 o2:u=0b1 ig3=0b10 H:u=0bx ig4=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMLSL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UMLSL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b10 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMLSL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMLSL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b1 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b0 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=UMMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS UMOV_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=UMOV_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UMULL_asimdelem_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01111 size:u=0bxx L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1010 H:u=0bx ig2=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=UMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=UMULL_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UMULL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b1100 ig3=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UMULL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=UMULL_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UQADD_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UQADD_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00001 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UQRSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS UQRSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQRSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQRSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UQRSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQRSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQRSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS UQRSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b1 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQRSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQRSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS UQSHL_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQSHL_advsimd_imm',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh == 0',
};

CLASS UQSHL_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b011 op:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQSHL_advsimd_imm',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS UQSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQSHL_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS UQSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b1 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQSHL_advsimd_reg',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UQSHRN_asisdshf_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=UQSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(immh == 0) || (immh<3> == 1)',
};

CLASS UQSHRN_asimdshf_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b1001 op:u=0b0 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSHRN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UQSHRN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS UQSUB_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=UQSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS UQSUB_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00101 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQSUB',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=UQSUB_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UQXTN_asisdmisc_N => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=UQXTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=UQXTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UQXTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10100 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=UQXTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=UQXTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS URECPE_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URECPE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=URECPE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'sz == 1',
};

CLASS URHADD_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx opcode=0b00010 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URHADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=URHADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS URSHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=URSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS URSHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b1 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=URSHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS URSHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=URSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS URSHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=URSHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS URSQRTE_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 ig2=0b1 sz:u=0bx ig3=0b10000 opcode=0b11100 ig4=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSQRTE',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=URSQRTE_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'sz == 1',
};

CLASS URSRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=URSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=URSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS URSRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b1 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=URSRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=URSRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS USDOT_asimdelem_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01111 US:u=0b1 ig2=0b0 L:u=0bx M:u=0bx Rm:u=0bxxxx opcode=0b1111 H:u=0bx ig3=0b0 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=2reg-element instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdelem page=USDOT_advsimd_elt',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS USDOT_asimdsame2_D => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 opcode=0b0011 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USDOT',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=USDOT_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS USHL_asisdsame_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdsame page=USHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'S == 0 && size != 3',
};

CLASS USHL_asimdsame_only => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b010 R:u=0b0 S:u=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=USHL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS USHLL_advsimd_USHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHLL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USHLL_advsimd xid=USHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS USHR_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=USHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS USHR_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b0 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHR',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USHR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS USMMLA_asimdsame2_G => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q=0b1 U:u=0b0 ig1=0b01110 size=0b10 ig2=0b0 Rm:u=0bxxxxx ig3=0b1 ig4=0b010 B:u=0b1 ig5=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USMMLA',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v6 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame2 page=USMMLA_advsimd_vec',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveInt8MatMulExt()',
};

CLASS USQADD_asisdmisc_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b11110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdmisc page=USQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS USQADD_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b00011 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USQADD',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=USQADD_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS USRA_asisdshf_R => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 U:u=0b1 ig1=0b111110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=sisd instr-class=advsimd isa=A64 mnemonic=USRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdshf page=USRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> != 1',
};

CLASS USRA_asimdshf_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0bxxx ig2=0b00 o1:u=0b0 o0:u=0b1 ig3=0b0 ig4=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USRA',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=USRA_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3>:Q == 2',
};

CLASS USUBL_asimddiff_L => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b0 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBL',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=USUBL_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS USUBW_asimddiff_W => {
  name       => 'Three registers, not all the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b01110 size:u=0bxx ig2=0b1 Rm:u=0bxxxxx ig3=0b00 o1:u=0b1 ig4=0b1 ig5=0b00 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-diff advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USUBW',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimddiff page=USUBW_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS UZP1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b01 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=UZP1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS UZP2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b01 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UZP2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=UZP2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS XAR_VVV2_crypto3_imm6 => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b11001110100 Rm:u=0bxxxxx imm6:u=0bxxxxxx Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=XAR',
  metadata   => 'alias=0 isa=A64 isaform=A64 arch_variant=ARMv8v2 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=crypto3_imm6 page=XAR_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '!HaveSHA3Ext()',
};

CLASS XTN_asimdmisc_N => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0bxx ig2=0b10000 opcode=0b10010 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=XTN',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=XTN_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size == 3',
};

CLASS ZIP1_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b0 ig4=0b11 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP1',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=ZIP1_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS ZIP2_asimdperm_only => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx ig1=0b001110 size:u=0bxx ig2=0b0 Rm:u=0bxxxxx ig3=0b0 op:u=0b1 ig4=0b11 ig5=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=ZIP2',
  metadata   => 'alias=0 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdperm page=ZIP2_advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'size:Q == 6',
};

CLASS MOV_DUP_advsimd_elt_DUP_asisdone_only => {
  name       => 'Scalar',
  diagram    => 'ig0=0b01 op=0b0 ig1=0b11110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0000 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=DUP vector-xfer-type=scalar-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asisdone page=MOV_DUP_advsimd_elt xid=DUP_asisdone_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS MOV_INS_advsimd_elt_INS_asimdins_IV_v => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b1 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4:u=0bxxxx ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_INS_advsimd_elt xid=INS_asimdins_IV_v',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS MOV_INS_advsimd_gen_INS_asimdins_IR_r => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q=0b1 op=0b0 ig1=0b01110000 imm5:u=0bxxxxx ig2=0b0 imm4=0b0011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=INS vector-xfer-type=vector-from-general',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_INS_advsimd_gen xid=INS_asimdins_IR_r',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => '(LowestSetBit(imm5)) > 3',
};

CLASS MOV_ORR_advsimd_reg_ORR_asimdsame_only => {
  name       => 'Three registers of the same type',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b0 ig1=0b01110 size:u=0b10 ig2=0b1 Rm:u=0bxxxxx opcode=0b00011 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-reguse=3reg-same advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=ORR',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdsame page=MOV_ORR_advsimd_reg xid=ORR_asimdsame_only',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MOV_UMOV_advsimd_advsimd => {
  name       => 'Advanced SIMD',
  diagram    => 'ig0=0b0 Q:u=0bx op=0b0 ig1=0b01110000 imm5:u=0bxxx00 ig2=0b0 imm4=0b0111 ig3=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'instr-class=advsimd isa=A64 mnemonic=UMOV vector-xfer-type=general-from-element',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdins page=MOV_UMOV_advsimd xid=advsimd',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS MVN_NOT_advsimd_NOT_asimdmisc_R => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U=0b1 ig1=0b01110 size=0b00 ig2=0b10000 opcode=0b00101 ig3=0b10 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=NOT',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdmisc page=MVN_NOT_advsimd xid=NOT_asimdmisc_R',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
};

CLASS SXTL_SSHLL_advsimd_SSHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b0 ig1=0b011110 immh:u=0bxxxx immb:u=0b000 opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=SSHLL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=SXTL_SSHLL_advsimd xid=SSHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};

CLASS UXTL_USHLL_advsimd_USHLL_asimdshf_L => {
  name       => 'Vector',
  diagram    => 'ig0=0b0 Q:u=0bx U:u=0b1 ig1=0b011110 immh:u=0bxxxx immb:u=0b000 opcode=0b10100 ig2=0b1 Rn:u=0bxxxxx Rd:u=0bxxxxx',
  cnsts      => 'immh!=0b0000',
  docvars    => 'advsimd-type=simd instr-class=advsimd isa=A64 mnemonic=USHLL',
  metadata   => 'alias=1 isa=A64 isaform=A64 alphaindex=FPSIMD',
  tags       => 'group=simd_dp iclass=asimdshf page=UXTL_USHLL_advsimd xid=USHLL_asimdshf_L',
  exceptions => 'ADVSIMDFPACCESSTRAP|UNCATEGORIZED',
  undefined  => 'immh<3> == 1',
};


1;

